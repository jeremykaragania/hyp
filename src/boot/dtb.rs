use crate::align::align_up;
use core::ffi::CStr;
use core::mem::{size_of, size_of_val};
use core::slice::from_raw_parts;

unsafe extern "C" {
    static ram_begin: Header;
}

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_BEGIN_NODE: u32 = 0x1;
const FDT_END_NODE: u32 = 0x2;
const FDT_PROP: u32 = 0x3;
const FDT_NOP: u32 = 0x4;
const FDT_END: u32 = 0x9;

#[derive(PartialEq, Eq)]
enum FDTToken<'a> {
    BeginNode(&'a str),
    Prop { nameoff: u32, value: &'a [u8] },
    EndNode,
    Nop,
    End,
}

#[repr(C)]
struct Header {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

struct FDTStream<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> FDTStream<'a> {
    fn new(data: &'a [u8], offset: usize) -> Self {
        Self {
            data: data,
            offset: offset,
        }
    }

    fn parse_u32(&mut self) -> Result<u32, ()> {
        let next_offset = self.offset + size_of::<u32>();

        if next_offset > self.data.len() {
            return Err(());
        }

        let ret = u32::from_be_bytes(self.data[self.offset..next_offset].try_into().unwrap());
        self.offset = next_offset;
        Ok(ret)
    }

    fn parse_u64(&mut self) -> Result<u64, ()> {
        let next_offset = self.offset + size_of::<u64>();

        if next_offset > self.data.len() {
            return Err(());
        }

        let ret = u64::from_be_bytes(self.data[self.offset..next_offset].try_into().unwrap());
        self.offset = next_offset;
        Ok(ret)
    }

    fn next_token(&mut self) -> Result<FDTToken<'a>, ()> {
        let token_type = self.parse_u32()?;

        match token_type {
            FDT_BEGIN_NODE => {
                let name = CStr::from_bytes_until_nul(&self.data[self.offset..])
                    .unwrap()
                    .to_str()
                    .unwrap();

                self.offset = align_up((self.offset + name.len() + 1) as u64, 4) as usize;
                Ok(FDTToken::BeginNode(name))
            }
            FDT_END_NODE => Ok(FDTToken::EndNode),
            FDT_PROP => {
                let len = self.parse_u32()? as usize;
                let nameoff = self.parse_u32()?;

                let value = &self.data[self.offset..self.offset + len];

                self.offset = align_up((self.offset + len) as u64, 4) as usize;
                Ok(FDTToken::Prop { nameoff, value })
            }
            FDT_NOP => {
                self.offset = align_up((self.offset + size_of::<u32>()) as u64, 4) as usize;
                Ok(FDTToken::Nop)
            }
            FDT_END => Ok(FDTToken::End),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
struct FDTParseContext {
    depth: usize,
}

impl Default for FDTParseContext {
    fn default() -> Self {
        Self { depth: 0 }
    }
}

struct Devicetree<'a> {
    data: &'a [u8],
    struct_offset: usize,
    strings_offset: usize,
    reserve_offset: usize,
}

impl<'a> Devicetree<'a> {
    pub fn new(header: &Header) -> Result<Self, ()> {
        if u32::from_be(header.magic) != FDT_MAGIC {
            return Err(());
        }

        let totalsize = u32::from_be(header.totalsize) as usize;
        let data = unsafe { from_raw_parts(header as *const Header as *const u8, totalsize) };
        let struct_offset = u32::from_be(header.off_dt_struct) as usize;
        let strings_offset = u32::from_be(header.off_dt_strings) as usize;
        let reserve_offset = u32::from_be(header.off_mem_rsvmap) as usize;

        if struct_offset >= totalsize || strings_offset >= totalsize {
            return Err(());
        }

        Ok(Self {
            data: data,
            struct_offset: struct_offset,
            strings_offset: strings_offset,
            reserve_offset: reserve_offset,
        })
    }

    fn get_string(&self, offset: u32) -> &'a str {
        CStr::from_bytes_until_nul(&self.data[self.strings_offset + offset as usize..])
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn parse(&self) -> Result<(), ()> {
        self.parse_reservation_block()?;
        self.parse_structure_block()
    }

    fn parse_structure_block(&self) -> Result<(), ()> {
        let mut stream = FDTStream::new(self.data, self.struct_offset);
        let context = FDTParseContext::default();

        self.parse_node(&mut stream, context)
    }

    fn parse_reservation_block(&self) -> Result<(), ()> {
        let mut stream = FDTStream::new(self.data, self.reserve_offset);

        loop {
            let address = stream.parse_u64()?;
            let size = stream.parse_u64()?;

            if address == 0 || size == 0 {
                break;
            }
        }

        Ok(())
    }

    fn parse_node(&self, stream: &mut FDTStream<'a>, context: FDTParseContext) -> Result<(), ()> {
        let mut next_context = FDTParseContext {
            depth: context.depth + 1,
        };

        while let Ok(token) = stream.next_token() {
            match token {
                FDTToken::BeginNode(name) => {
                    self.parse_node(stream, next_context)?;
                }
                FDTToken::Prop { nameoff, value } => {
                    let name = self.get_string(nameoff);
                }
                FDTToken::EndNode => {
                    return Ok(());
                }
                FDTToken::Nop => {
                    return Ok(());
                }
                FDTToken::End => {
                    return Ok(());
                }
            }
        }

        Err(())
    }
}

pub fn parse_fdt() -> Result<(), ()> {
    let header: &Header = unsafe { &*(&ram_begin as *const Header) };
    let dt = Devicetree::new(header)?;

    dt.parse()
}
