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
use device::init_devices;
use platform::PLATFORM;

unsafe extern "C" {
    fn current_el() -> u64;
}

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

    init_devices().unwrap();

    // We should boot in the highest EL. So if we're not in EL2, then we
    // know that virtualization isn't supported. We wait until here to panic as
    // to give the hypervisor the chance to initialize a console.
    // NOTE: We don't support EL3.
    if unsafe { current_el() } != 2 {
        panic!("Virtualization is not supported on this platform");
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
