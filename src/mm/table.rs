const SZ_512GB: usize = 0x8000000000;
const SZ_1GB: usize = 0x40000000;
const SZ_2MB: usize = 0x200000;
const SZ_4KB: usize = 0x1000;

pub enum DescriptorKind {
    Invalid,
    Table,
    Block,
}

pub fn level_shift(level: u8) -> u8 {
    match level {
        0 => 39,
        1 => 30,
        2 => 21,
        3 => 12,
        _ => unreachable!(),
    }
}

pub fn level_entry_size(level: u8) -> usize {
    match level {
        0 => SZ_512GB,
        1 => SZ_1GB,
        2 => SZ_2MB,
        3 => SZ_4KB,
        _ => unreachable!(),
    }
}

pub fn create_invalid_entry() -> u64 {
    0
}

pub fn create_table_entry(addr: u64) -> u64 {
    addr & (u64::MAX << 12) | 0b11
}

pub fn create_block_entry(addr: u64, level: u8) -> u64 {
    let kind_bits = { if level == 3 { 0b11 } else { 0b01 } };

    addr & (u64::MAX << level_shift(level)) | 1 << 10 | kind_bits
}

pub fn desc_kind(desc: u64) -> DescriptorKind {
    let kind = desc & 0b11;

    match kind {
        0 => DescriptorKind::Invalid,
        1 => DescriptorKind::Block,
        3 => DescriptorKind::Table,
        _ => unreachable!(),
    }
}

pub fn entry_addr(desc: u64, level: u8) -> u64 {
    let kind = desc_kind(desc);

    match kind {
        DescriptorKind::Invalid => 0,
        DescriptorKind::Block => desc & (u64::MAX << level_shift(level)),
        DescriptorKind::Table => desc & (u64::MAX << 12),
    }
}
