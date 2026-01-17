use alloc::{format, string::String, vec::Vec};
use goblin::elf;
use log::debug;
use uefi::boot;

pub unsafe fn load_kernel(kernel_elf: Vec<u8>) -> Result<u64, String> {
    let elf = elf::Elf::parse(&kernel_elf).map_err(|e| format!("Could not parse elf: {e}"))?;

    for ph in elf.program_headers {
        if ph.p_type == elf::program_header::PT_LOAD {
            let start_addr = ph.p_paddr;
            let mem_size = ph.p_memsz as usize;
            let file_size = ph.p_filesz as usize;

            let num_pages = if mem_size % boot::PAGE_SIZE == 0 {
                mem_size / boot::PAGE_SIZE
            } else {
                mem_size / boot::PAGE_SIZE + 1
            };

            debug!(
                "Trying to allocate {num_pages} pages (0x{:x} bytes), 0x{start_addr:x}-0x{:x}",
                num_pages * boot::PAGE_SIZE,
                start_addr as usize + mem_size
            );

            boot::allocate_pages(
                boot::AllocateType::Address(start_addr),
                boot::MemoryType::LOADER_DATA,
                num_pages,
            )
            .map_err(|e| format!("Could not allocate memory pages: {e}"))?;

            let dest_ptr = start_addr as *mut u8;

            unsafe {
                core::ptr::write_bytes(dest_ptr, 0, mem_size);
            }

            if file_size > 0 {
                let elf_offset = ph.p_offset as usize;

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        kernel_elf[elf_offset..].as_ptr(),
                        dest_ptr,
                        file_size,
                    );
                }
            }
        }
    }

    Ok(elf.entry)
}
