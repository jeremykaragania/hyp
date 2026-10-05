use crate::bitmap_words;
use crate::mm::pool::Pool;

pub const MAX_MMIO_REGIONS: usize = 8;

#[derive(Clone, Copy)]
pub struct MMIORegion {
    pub begin: u64,
    pub size: usize,
}

#[derive(Clone, Copy)]
pub enum DeviceKind {
    PL011,
}

// A device on the platform that the hypervisor supports.
#[derive(Clone)]
pub struct Device {
    pub kind: DeviceKind,
    pub mmio_regions: Pool<MMIORegion, MAX_MMIO_REGIONS, { bitmap_words!(MAX_MMIO_REGIONS) }>,
}

impl Device {
    pub fn new(kind: DeviceKind) -> Self {
        Self {
            kind,
            mmio_regions: Pool::new(),
        }
    }
}
