use crate::align::align_up;
use core::ffi::CStr;
use core::mem::size_of_val;

unsafe extern "C" {
    static ram_begin: Header;
}

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_BEGIN_NODE: u32 = 0x1;
const FDT_END_NODE: u32 = 0x2;
const FDT_PROP: u32 = 0x3;
const FDT_NOP: u32 = 0x4;
const FDT_END: u32 = 0x9;

#[repr(C)]
pub struct Header {
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

#[repr(C)]
struct ReserveEntry {
    address: u64,
    size: u64,
}

#[repr(C)]
struct PropertyData {
    len: u32,
    nameoff: u32,
}

pub fn parse_fdt() -> Result<(), ()> {
    let header: &Header = unsafe { &*(&ram_begin as *const Header) };

    if u32::from_be(header.magic) != FDT_MAGIC {
        return Err(());
    }

    let res = parse_fdt_reservation_block(header);

    if res.is_err() {
        return res;
    }

    let res = parse_fdt_structure_block(header);

    if res.is_err() {
        return res;
    }

    Ok(())
}

pub fn parse_fdt_reservation_block(header: &Header) -> Result<(), ()> {
    let header_ptr = header as *const Header;
    unsafe {
        let mut re: *const ReserveEntry = (header_ptr as *const u8)
            .add(u32::from_be(header.off_mem_rsvmap) as usize)
            as *const ReserveEntry;

        while (*re).address != 0 && (*re).size != 0 {
            re = re.add(1);
        }
    }

    Ok(())
}

pub fn parse_fdt_structure_block(header: &Header) -> Result<(), ()> {
    let header_ptr = header as *const Header;
    let base = unsafe { (header_ptr as *mut u8).add(u32::from_be(header.off_dt_struct) as usize) };
    let mut i = 0;
    let mut token: u32;
    let mut node_name_bytes: [u8; 32] = [0; 32];
    let mut node_name: &str = "";

    loop {
        token = unsafe { u32::from_be(*(base.add(i) as *const u32)) };
        i += size_of_val(&token);

        match token {
            FDT_BEGIN_NODE => {
                let ptr = unsafe { base.add(i) };
                let c_str = unsafe { CStr::from_ptr(ptr) };
                let name = c_str.to_bytes_with_nul();

                i += name.len();
                node_name_bytes[..name.len()].copy_from_slice(name);

                if let Some(unit_address) = name.iter().position(|&c| c == b'@') {
                    node_name_bytes[unit_address] = 0;
                }

                node_name = CStr::from_bytes_until_nul(&node_name_bytes)
                    .unwrap()
                    .to_str()
                    .unwrap();

                i = align_up(i as u64, 4) as usize;
            }
            FDT_END_NODE => {}
            FDT_PROP => {
                let ptr = unsafe { base.add(i) };
                let data: &PropertyData = unsafe { &*(ptr as *const PropertyData) };
                let data_len = u32::from_be(data.len);
                let data_nameoff = u32::from_be(data.nameoff);

                i += size_of_val(data);

                let ptr = unsafe {
                    (header_ptr as *const u8)
                        .add((u32::from_be(header.off_dt_strings) + data_nameoff) as usize)
                };

                let c_str = unsafe { CStr::from_ptr(ptr) };
                let property_name = c_str.to_str().unwrap();

                i += data_len as usize;
                i = align_up(i as u64, 4) as usize;
            }
            FDT_NOP => {}
            FDT_END => return Ok(()),
            _ => {
                return Err(());
            }
        }
    }
}
