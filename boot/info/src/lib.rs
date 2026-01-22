#![no_std]

use crate::memory::MemoryMapInfo;

pub mod memory;

#[repr(C)]
pub struct BootInfo {
    pub memory_map: MemoryMapInfo,
}
