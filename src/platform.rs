use crate::bitmap_words;
use crate::cpu::{Cpu, MAX_CPUS};
use crate::device::{Device, MAX_MMIO_REGIONS};
use crate::memory::MemoryRegion;
use crate::sync::Spinlock;
use crate::mm::pool::Pool;

// TODO: This should be a `OnceLock` but we don't have that in the `core` crate.
pub static mut PLATFORM: Spinlock<Platform> = Spinlock::new(Platform::new());

pub const MAX_PLATFORM_DEVICES: usize = 64;

// A representation of the platform the hypervisor is running on.
#[derive(Clone)]
pub struct Platform {
    pub devices: Pool<Device, MAX_PLATFORM_DEVICES, { bitmap_words!(MAX_PLATFORM_DEVICES) }>,
    pub memory: Pool<MemoryRegion, MAX_MMIO_REGIONS, { bitmap_words!(MAX_MMIO_REGIONS) }>,
    pub cpus: Pool<Cpu, MAX_CPUS, { bitmap_words!(MAX_CPUS) }>,
}

impl Platform {
    pub const fn new() -> Self {
        Self {
            devices: Pool::new(),
            memory: Pool::new(),
            cpus: Pool::new(),
        }
    }
}
