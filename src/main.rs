#![no_std]
#![no_main]

mod align;
mod boot;
mod builtins;
mod mm;
mod sync;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
