#![no_std]
#![no_main]

mod align;
mod boot;
mod builtins;
mod cpu;
mod device;
mod drivers;
mod memory;
mod mm;
mod platform;
mod sync;

use boot::dtb::parse_fdt;
use core::panic::PanicInfo;
use platform::PLATFORM;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    match parse_fdt() {
        Ok(p) => {
            let mut platform = PLATFORM.lock();
            *platform = p;
        }
        Err(()) => {
            panic!(); // TODO: Maybe store an error code in a register or something.
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
