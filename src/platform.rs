use crate::bitmap_words;
use crate::cpu::{Cpu, MAX_CPUS};
use crate::device::{DeviceDesc, MAX_MMIO_REGIONS};
use crate::memory::MemoryRegion;
use crate::mm::pool::{Pool, PoolIndex};
use crate::sync::Spinlock;

// TODO: This should be a `OnceLock` but we don't have that in the `core` crate.
pub static PLATFORM: Spinlock<Platform> = Spinlock::new(Platform::new());

pub const MAX_PLATFORM_DEVICES: usize = 64;

// A representation of the platform the hypervisor is running on.
#[derive(Clone)]
pub struct Platform {
    pub console: Option<PoolIndex>,
    pub devices: Pool<DeviceDesc, MAX_PLATFORM_DEVICES, { bitmap_words!(MAX_PLATFORM_DEVICES) }>,
    pub memory: Pool<MemoryRegion, MAX_MMIO_REGIONS, { bitmap_words!(MAX_MMIO_REGIONS) }>,
    pub cpus: Pool<Cpu, MAX_CPUS, { bitmap_words!(MAX_CPUS) }>,
}

impl Platform {
    pub const fn new() -> Self {
        Self {
            console: None,
            devices: Pool::new(),
            memory: Pool::new(),
            cpus: Pool::new(),
        }
    }
}
