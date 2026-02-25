#![no_std]
#![no_main]

use core::panic::PanicInfo;

use boot_info::BootInfo;
use log::{info, LevelFilter};
use logger::KernelLogger;

use crate::memory::frame::FrameAllocator;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

pub mod logger;
pub mod memory;
pub mod serial;

pub fn kernel_main(boot_info: BootInfo) -> ! {
    arch::init();

    KernelLogger::init();
    KernelLogger::set_max_level(LevelFilter::Trace);

    info!("Welcome to SOS!");

    let mut frame_allocator = unsafe { FrameAllocator::init(boot_info.memory_map) };

    let frame = frame_allocator.allocate_frame().unwrap();
    frame_allocator.free_frame(frame);

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    print!("{info}");
    loop {}
}
