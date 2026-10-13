pub const MAX_CPUS: usize = 8;

#[derive(Copy, Clone)]
pub struct CpuId(pub usize);

// A physical CPU on the platform.
#[derive(Clone)]
pub struct Cpu {
    pub id: CpuId,
}
