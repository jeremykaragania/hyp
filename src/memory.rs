// A physical memory region on the platform.
#[derive(Clone, Debug)]
pub struct MemoryRegion {
    pub begin: u64,
    pub size: usize,
}
