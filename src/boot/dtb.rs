use crate::align::align_up;
use crate::bitmap_words;
use crate::cpu::CpuId;
use crate::device::{Device, DeviceKind, MAX_MMIO_REGIONS};
use crate::memory::MemoryRegion;
use crate::mm::pool::Pool;
use crate::platform::Platform;
use core::ffi::CStr;
use core::mem::size_of;
use core::slice::from_raw_parts;
use core::str::{FromStr, Utf8Error, from_utf8};

unsafe extern "C" {
    static ram_begin: Header;
}

const FDT_MAX_DEPTH: usize = 64;

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_BEGIN_NODE: u32 = 0x1;
const FDT_END_NODE: u32 = 0x2;
const FDT_PROP: u32 = 0x3;
const FDT_NOP: u32 = 0x4;
const FDT_END: u32 = 0x9;

#[derive(PartialEq, Eq)]
enum FdtToken<'a> {
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

struct FdtStream<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> FdtStream<'a> {
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

    fn next_token(&mut self) -> Result<FdtToken<'a>, ()> {
        let token_type = self.parse_u32()?;

        match token_type {
            FDT_BEGIN_NODE => {
                let name = CStr::from_bytes_until_nul(&self.data[self.offset..])
                    .unwrap()
                    .to_str()
                    .unwrap();

                self.offset = align_up((self.offset + name.len() + 1) as u64, 4) as usize;
                Ok(FdtToken::BeginNode(name))
            }
            FDT_END_NODE => Ok(FdtToken::EndNode),
            FDT_PROP => {
                let len = self.parse_u32()? as usize;
                let nameoff = self.parse_u32()?;

                let value = &self.data[self.offset..self.offset + len];

                self.offset = align_up((self.offset + len) as u64, 4) as usize;
                Ok(FdtToken::Prop { nameoff, value })
            }
            FDT_NOP => {
                self.offset = align_up((self.offset + size_of::<u32>()) as u64, 4) as usize;
                Ok(FdtToken::Nop)
            }
            FDT_END => Ok(FdtToken::End),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct FdtParseContext<'a> {
    depth: usize,
    address_cells: u32,
    size_cells: u32,
    node: Node<'a>,
    stdout_path: &'a str,
}

impl<'a> Default for FdtParseContext<'a> {
    fn default() -> Self {
        Self {
            depth: 0,
            address_cells: 0,
            size_cells: 0,
            node: Node::new(),
            stdout_path: "",
        }
    }
}

#[derive(Clone)]
struct Node<'a> {
    reg: Option<Reg>,
    compatible: Option<Compatible<'a>>,
    device_type: Option<DeviceType>,
}

#[derive(Clone)]
struct NodePath<'a> {
    names: [&'a str; FDT_MAX_DEPTH],
    count: usize,
}

impl<'a> NodePath<'a> {
    fn new() -> Self {
        Self {
            names: [""; FDT_MAX_DEPTH],
            count: 0,
        }
    }

    fn push_name(&mut self, name: &'a str) -> Result<(), ()> {
        if self.count > FDT_MAX_DEPTH {
            return Err(());
        }

        self.names[self.count] = name;
        self.count += 1;

        Ok(())
    }

    fn pop_name(&mut self) -> Option<&'a str> {
        if self.count == 0 {
            return None;
        }

        let name = self.names[self.count];

        self.names[self.count] = "";
        self.count -= 1;

        Some(name)
    }

    fn matches(&self, expected: &'a str) -> bool {
        if !expected.starts_with('/') {
            return false;
        }

        let mut expected_names = expected[1..].split('/');

        for name in self.names.iter().filter(|n| !n.is_empty()) {
            if expected_names.next() != Some(*name) {
                return false;
            }
        }

        return expected_names.next().is_none();
    }
}

impl<'a> Node<'a> {
    pub fn new() -> Self {
        Self {
            reg: None,
            compatible: None,
            device_type: None,
        }
    }

    fn to_device(&self) -> Option<Device> {
        // If the node doesn't have a `compatible` property, then we can't turn
        // this node into a `Device`.
        let compatible = self.compatible?;

        let device_kind = compatible.to_device_kind()?;
        let mut device = Device::new(device_kind);

        if let Some(reg) = self.reg.as_ref() {
            for (_, reg_entry) in reg.iter() {
                let id = device.mmio_regions.alloc()?;
                let mmio = device.mmio_regions.get_mut(id)?;

                mmio.begin = reg_entry.address;
                mmio.size = reg_entry.length as usize;
            }
        }

        Some(device)
    }
}

type Reg = Pool<RegEntry, MAX_MMIO_REGIONS, { bitmap_words!(MAX_MMIO_REGIONS) }>;

#[derive(Clone, Copy)]
struct RegEntry {
    address: u64,
    length: u64,
}

#[derive(Clone, Copy)]
struct Compatible<'a> {
    value: &'a [u8],
}

impl<'a> Compatible<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        Self { value }
    }

    fn to_device_kind(&self) -> Option<DeviceKind> {
        for result in self.iter() {
            let compatible = result.ok()?;

            match compatible {
                "arm,pl011" => {
                    return Some(DeviceKind::Pl011);
                }
                _ => {}
            }
        }

        None
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<&'a str, Utf8Error>> {
        self.value
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(from_utf8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DeviceType {
    Cpu,
    Memory,
}

impl FromStr for DeviceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpu" => Ok(DeviceType::Cpu),
            "memory" => Ok(DeviceType::Memory),
            _ => Err(()),
        }
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

    fn update_platform(node: &Node, platform: &mut Platform) -> Result<(), ()> {
        if let Some(device_type) = node.device_type {
            match device_type {
                DeviceType::Memory => {
                    if let Some(reg) = node.reg.as_ref() {
                        for (_, reg_entry) in reg.iter() {
                            let id = platform.memory.alloc().ok_or(())?;
                            let memory_region = platform.memory.get_mut(id).ok_or(())?;

                            memory_region.begin = reg_entry.address;
                            memory_region.size = reg_entry.length as usize;
                        }
                    }
                }
                DeviceType::Cpu => {
                    if let Some(reg) = node.reg.as_ref() {
                        for (_, reg_entry) in reg.iter() {
                            let id = platform.cpus.alloc().ok_or(())?;
                            let cpu = platform.cpus.get_mut(id).ok_or(())?;

                            cpu.id = CpuId(reg_entry.address as usize);
                        }
                    }
                }
            }
        } else if let Some(mut node_device) = node.to_device() {
            let id = platform.devices.alloc().ok_or(())?;
            let mut device = platform.devices.get_mut(id).ok_or(())?;
            device = &mut node_device;
        }

        Ok(())
    }

    pub fn parse(&self) -> Result<Platform, ()> {
        let mut platform = Platform::new();

        self.parse_reservation_block(&mut platform)?;
        self.parse_structure_block(&mut platform)?;

        Ok(platform)
    }

    fn parse_structure_block(&self, platform: &mut Platform) -> Result<(), ()> {
        let mut stream = FdtStream::new(self.data, self.struct_offset);
        let mut context = FdtParseContext::default();
        let mut path = NodePath::new();

        self.parse_node(&mut stream, &mut context, platform, &mut path)
    }

    fn parse_reservation_block(&self, platform: &mut Platform) -> Result<(), ()> {
        let mut stream = FdtStream::new(self.data, self.reserve_offset);

        loop {
            let address = stream.parse_u64()?;
            let size = stream.parse_u64()?;

            if address == 0 || size == 0 {
                break;
            }
        }

        Ok(())
    }

    fn parse_prop(
        &self,
        context: &mut FdtParseContext<'a>,
        name: &'a str,
        value: &'a [u8],
    ) -> Result<(), ()> {
        let mut stream = FdtStream::new(value, 0);

        match name {
            "compatible" => {
                context.node.compatible = Some(Compatible::new(value));
            }
            "#address-cells" => {
                context.address_cells = stream.parse_u32()?;
            }
            "#size-cells" => {
                context.size_cells = stream.parse_u32()?;
            }
            "reg" => {
                let pair_count = value.len()
                    / ((context.address_cells + context.size_cells) as usize * size_of::<u32>());

                if pair_count > MAX_MMIO_REGIONS {
                    return Err(());
                }

                context.node.reg = Some(Pool::new());

                let reg = context.node.reg.as_mut().ok_or(())?;

                for _ in 0..pair_count {
                    let id = reg.alloc().ok_or(())?;
                    let reg_entry = reg.get_mut(id).ok_or(())?;

                    match context.address_cells {
                        1 => {
                            reg_entry.address = stream.parse_u32()? as u64;
                        }
                        2 => {
                            reg_entry.address = stream.parse_u64()?;
                        }
                        _ => {
                            return Err(());
                        }
                    }
                    match context.size_cells {
                        0 => {}
                        1 => {
                            reg_entry.length = stream.parse_u32()? as u64;
                        }
                        2 => {
                            reg_entry.length = stream.parse_u64()?;
                        }
                        _ => {
                            return Err(());
                        }
                    }
                }
            }
            "device_type" => {
                let device_type = CStr::from_bytes_until_nul(value).unwrap().to_str().unwrap();

                context.node.device_type = device_type.parse().ok();
            }
            "stdout-path" => {
                let stdout_path = CStr::from_bytes_until_nul(value).unwrap().to_str().unwrap();

                context.stdout_path = stdout_path;
            }
            &_ => {}
        }

        Ok(())
    }

    fn parse_node(
        &self,
        stream: &mut FdtStream<'a>,
        context: &mut FdtParseContext<'a>,
        platform: &mut Platform,
        path: &mut NodePath<'a>,
    ) -> Result<(), ()> {
        if context.depth > FDT_MAX_DEPTH {
            return Err(());
        }

        while let Ok(token) = stream.next_token() {
            match token {
                FdtToken::BeginNode(name) => {
                    let mut next_context = context.clone();

                    let _ = path.push_name(name);
                    next_context.depth += 1;

                    self.parse_node(stream, &mut next_context, platform, path)?;
                }
                FdtToken::Prop { nameoff, value } => {
                    let name = self.get_string(nameoff);

                    self.parse_prop(context, name, value)?;
                }
                FdtToken::EndNode => {
                    Self::update_platform(&context.node, platform)?;

                    path.pop_name();
                    return Ok(());
                }
                FdtToken::Nop => {
                    return Ok(());
                }
                FdtToken::End => {
                    return Ok(());
                }
            }
        }

        Err(())
    }
}

pub fn parse_fdt() -> Result<Platform, ()> {
    let header: &Header = unsafe { &*(&ram_begin as *const Header) };
    let dt = Devicetree::new(header)?;

    dt.parse()
}
