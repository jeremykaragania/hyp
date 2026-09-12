unsafe extern "C" {
    static ram_begin: u8;
}

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

struct ReserveEntry {
    address: u64,
    size: u64,
}

struct PropertyData {
    len: u32,
    nameoff: u32,
}
