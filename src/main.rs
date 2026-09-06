#![no_std]
#![no_main]

mod boot;
mod builtins;
mod mmu;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
