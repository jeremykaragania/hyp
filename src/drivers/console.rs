use crate::device::{DEVICES, InitializedDevice};
use crate::mm::pool::PoolIndex;
use crate::sync::Spinlock;
use core::fmt;

pub static CONSOLE: Spinlock<Console> = Spinlock::new(Console::new());

pub struct Console {
    pub device: Option<PoolIndex>,
}

impl Console {
    pub const fn new() -> Self {
        Self { device: None }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let device_id = self.device.ok_or(fmt::Error)?;
        let mut devices = DEVICES.lock();
        let device = devices.get_mut(device_id).ok_or(fmt::Error)?;

        match device {
            InitializedDevice::Pl011(pl011) => fmt::Write::write_str(pl011, s),
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use crate::drivers::console::CONSOLE;

        let mut console = CONSOLE.lock();

        let _ = write!(console, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };

    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
