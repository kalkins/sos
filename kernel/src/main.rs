#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

pub mod serial;

pub fn kernel_main() -> ! {
    arch::init();

    println!("Welcome to SOS!");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    print!("{info}");
    loop {}
}
