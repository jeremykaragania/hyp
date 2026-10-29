use crate::bitmap_words;
use crate::drivers::pl011::Pl011;
use crate::mm::pool::Pool;
use crate::platform::{PLATFORM, Platform};

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

pub enum InitializedDevice {
    Pl011(Pl011),
}

pub fn init_devices() -> Result<(), ()> {
    let mut platform = PLATFORM.lock();

    for (_, desc) in platform.devices.iter() {
        init_device(desc)?;
    }

    Ok(())
}

fn init_device(desc: &DeviceDesc) -> Result<InitializedDevice, ()> {
    match desc.kind {
        DeviceKind::Pl011 => {
            let (_, region) = desc.mmio_regions.iter().next().ok_or(())?;
            return Ok(InitializedDevice::Pl011(Pl011::new(region.begin)));
        }
    }

    Err(())
}
