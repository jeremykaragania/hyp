use crate::bitmap_words;
use crate::boot::map::{DATA_MEM_ATTRIBUTES, map_region};
use crate::drivers::pl011::Pl011;
use crate::mm::pool::Pool;
use crate::mm::table::Descriptor;
use crate::mm::table_alloc::base_table_ptr;
use crate::platform::MAX_PLATFORM_DEVICES;
use crate::platform::PLATFORM;
use crate::sync::Spinlock;

pub const MAX_MMIO_REGIONS: usize = 8;

pub static DEVICES: Spinlock<
    Pool<InitializedDevice, MAX_PLATFORM_DEVICES, { bitmap_words!(MAX_PLATFORM_DEVICES) }>,
> = Spinlock::new(Pool::new());

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

pub enum InitializedDevice {
    Pl011(Pl011),
}

pub fn init_devices() -> Result<(), ()> {
    let platform = PLATFORM.lock();
    let mut devices = DEVICES.lock();

    for (_, desc) in platform.devices.iter() {
        devices.insert(init_device(desc)?).ok_or(())?;
    }

    Ok(())
}

fn init_device(desc: &DeviceDesc) -> Result<InitializedDevice, ()> {
    match desc.kind {
        DeviceKind::Pl011 => {
            let (_, region) = desc.mmio_regions.iter().next().ok_or(())?;
            let begin = region.begin;
            let end = begin + region.size as u64;
            let table = base_table_ptr();

            unsafe {
                map_region(
                    table as *mut Descriptor,
                    begin,
                    end,
                    begin,
                    &DATA_MEM_ATTRIBUTES,
                );
            }

            let pl011 = Pl011::new(begin);

            return Ok(InitializedDevice::Pl011(pl011));
        }
    }
}
