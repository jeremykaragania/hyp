use crate::bitmap_words;
use crate::mm::pool::Pool;

pub const MAX_MMIO_REGIONS: usize = 8;

#[derive(Clone, Copy)]
pub struct MmioRegion {
    pub begin: u64,
    pub size: usize,
}

#[derive(Clone, Copy)]
pub enum DeviceKind {
    Pl011,
}

// A device on the platform that the hypervisor supports.
#[derive(Clone)]
pub struct DeviceDesc {
    pub kind: DeviceKind,
    pub mmio_regions: Pool<MmioRegion, MAX_MMIO_REGIONS, { bitmap_words!(MAX_MMIO_REGIONS) }>,
}

impl DeviceDesc {
    pub fn new(kind: DeviceKind) -> Self {
        Self {
            kind,
            mmio_regions: Pool::new(),
        }
    }
}
