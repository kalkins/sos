#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use log::info;
use uefi::{fs::FileSystem, prelude::*};

use crate::fileutils::*;

mod fileutils;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    system::with_stdout(|s| s.clear().unwrap());

    info!("Hello World!");

    let fs = boot::get_image_file_system(boot::image_handle()).unwrap();
    let mut fs = FileSystem::new(fs);

    enumerate_files(&mut fs, cstr16!(""));

    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
