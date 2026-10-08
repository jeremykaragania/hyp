#![no_std]
#![no_main]

mod align;
mod boot;
mod builtins;
mod device;
mod drivers;
mod mm;
mod platform;
mod sync;

use boot::dtb::parse_fdt;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let res = parse_fdt();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
