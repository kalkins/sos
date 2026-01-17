#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use log::{debug, info};
use uefi::{boot::MemoryType, fs::PathBuf, prelude::*};

use crate::{fileutils::*, kernel::load_kernel};

mod fileutils;
mod kernel;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    system::with_stdout(|s| s.clear().unwrap());

    let kernel_addr = select_kernel();
    debug!("Kernel loaded");

    unsafe {
        debug!("Exiting boot services");
        let _ = boot::exit_boot_services(None);

        let kernel_entry: extern "sysv64" fn() -> ! = core::mem::transmute(kernel_addr);

        kernel_entry();
    }
}

fn select_kernel() -> u64 {
    let mut kernel_path = PathBuf::new();
    kernel_path.push(cstr16!("boot/kernel"));

    let bootable_file_systems = get_file_systems_with_file(&kernel_path).unwrap();

    if bootable_file_systems.len() > 1 {
        panic!("Found multiple bootable file systems. This is not supported yet.");
    }

    let mut root_file_system = bootable_file_systems
        .into_iter()
        .next()
        .expect("Found no bootable file systems.");

    let kernel_file = root_file_system.read(kernel_path).unwrap();

    info!("Found kernel file with size {} bytes", kernel_file.len());

    unsafe { load_kernel(kernel_file).unwrap() }
}
