#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    system::with_stdout(|s| s.clear().unwrap());
    info!("Hello World!");
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
