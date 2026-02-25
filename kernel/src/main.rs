#![no_std]
#![no_main]

use core::panic::PanicInfo;

use boot_info::{memory::MemoryType, BootInfo};
use log::{debug, info, LevelFilter};
use logger::KernelLogger;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

pub mod logger;
pub mod serial;

pub fn kernel_main(boot_info: BootInfo) -> ! {
    arch::init();

    KernelLogger::init();
    KernelLogger::set_max_level(LevelFilter::Trace);

    info!("Welcome to SOS!");

    let mut free: usize = 0;

    for index in 0..boot_info.memory_map.count {
        unsafe {
            let region = *boot_info.memory_map.regions.add(index);

            debug!("");
            debug!("Memory region {index} ({:?})", region.kind);
            debug!("0x{:x}-0x{:x}", region.start, region.start + region.len);

            if region.kind == MemoryType::Free {
                free += region.len as usize;
            }
        }
    }

    info!("Total free memory: {free} bytes");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    print!("{info}");
    loop {}
}
