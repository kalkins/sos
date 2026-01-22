#![no_std]
#![no_main]

extern crate alloc;

use boot_info::{
    memory::{MemoryMapInfo, MemoryRegion, MemoryType},
    BootInfo,
};
use log::{debug, info};
use uefi::{
    boot::{AllocateType, MemoryType as UefiMemoryType},
    fs::PathBuf,
    mem::memory_map::MemoryMap,
    prelude::*,
};

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
        let mmap = exit_boot();

        let kernel_entry: extern "sysv64" fn(BootInfo) -> ! = core::mem::transmute(kernel_addr);

        kernel_entry(BootInfo { memory_map: mmap });
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

fn exit_boot() -> MemoryMapInfo {
    let num_regions = boot::memory_map(UefiMemoryType::LOADER_DATA).unwrap().len();
    let num_pages = 2 + num_regions * size_of::<MemoryRegion>() / boot::PAGE_SIZE;

    let regions = boot::allocate_pages(
        AllocateType::AnyPages,
        UefiMemoryType::LOADER_DATA,
        num_pages,
    )
    .unwrap()
    .cast::<MemoryRegion>();

    let mut region_index: usize = 0;

    debug!("Exiting boot services");
    unsafe {
        let uefi_mmap = boot::exit_boot_services(None);

        for uefi_entry in uefi_mmap.entries() {
            *regions.as_ptr().add(region_index) = MemoryRegion {
                start: uefi_entry.phys_start,
                len: uefi_entry.page_count * boot::PAGE_SIZE as u64,
                kind: match uefi_entry.ty {
                    UefiMemoryType::CONVENTIONAL => MemoryType::Free,
                    UefiMemoryType::LOADER_CODE => MemoryType::Bootloader,
                    UefiMemoryType::LOADER_DATA => MemoryType::Bootloader,
                    UefiMemoryType::ACPI_RECLAIM => MemoryType::Acpi,
                    UefiMemoryType::ACPI_NON_VOLATILE => MemoryType::Nvs,
                    _ => MemoryType::Reserved,
                },
            };

            region_index += 1;
        }
    }

    MemoryMapInfo {
        regions: regions.as_ptr(),
        count: region_index,
    }
}
