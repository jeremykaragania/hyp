const SZ_512GB: usize = 0x8000000000;
const SZ_1GB: usize = 0x40000000;
const SZ_2MB: usize = 0x200000;
const SZ_4KB: usize = 0x1000;

pub enum DescriptorKind {
    Invalid,
    Table,
    Block,
}

#[derive(PartialEq, Eq)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[derive(PartialEq, Eq)]
pub enum Execution {
    Executable,
    NonExecutable,
}

pub struct MemoryAttributes {
    pub permissions: AccessPermissions,
    pub execution: Execution,
}

#[derive(Clone, Copy)]
pub struct Descriptor(u64);

impl Descriptor {
    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn kind(self) -> DescriptorKind {
        let kind = self.bits() & 0b11;

        match kind {
            0 => DescriptorKind::Invalid,
            1 => DescriptorKind::Block,
            3 => DescriptorKind::Table,
            _ => unreachable!(),
        }
    }

    pub fn addr(self, level: u8) -> u64 {
        let bits = self.bits();

        match self.kind() {
            DescriptorKind::Invalid => 0,
            DescriptorKind::Block => bits & (u64::MAX << level_shift(level)),
            DescriptorKind::Table => bits & (u64::MAX << 12),
        }
    }

    pub fn invalid() -> Self {
        Self(0)
    }

    pub fn block(addr: u64, level: u8, attrs: &MemoryAttributes) -> Self {
        let kind_bits = { if level == 3 { 0b11 } else { 0b01 } };
        // AP[1] honestly seems kind of useless. We have to translation tables for a reason right?
        let permission_bits = ((attrs.permissions == AccessPermissions::ReadOnly) as u64) << 2 | 1;
        let xn_bit = (attrs.execution == Execution::NonExecutable) as u64;
        let execution_bits = xn_bit << 54 | xn_bit << 53;

        Self(
            addr & (u64::MAX << level_shift(level))
                | execution_bits
                | permission_bits
                | 1 << 10
                | kind_bits,
        )
    }

    pub fn table(addr: u64) -> Self {
        Self(addr & (u64::MAX << 12) | 0b11)
    }
}

impl From<Descriptor> for u64 {
    fn from(desc: Descriptor) -> u64 {
        desc.0
    }
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
