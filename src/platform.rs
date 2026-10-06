use crate::bitmap_words;
use crate::device::Device;
use crate::mm::pool::Pool;

pub const MAX_PLATFORM_DEVICES: usize = 64;

// A representation of the platform the hypervisor is running on.
#[derive(Clone)]
pub struct Platform {
    pub devices: Pool<Device, MAX_PLATFORM_DEVICES, { bitmap_words!(MAX_PLATFORM_DEVICES) }>,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            devices: Pool::new(),
        }
    }
}
