#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use log::info;
use uefi::{fs::PathBuf, prelude::*};

use crate::fileutils::*;

mod fileutils;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    system::with_stdout(|s| s.clear().unwrap());

    let mut kernel_path = PathBuf::new();
    kernel_path.push(cstr16!("boot/sos_kernel.elf"));

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

    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
