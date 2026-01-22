#[repr(C)]
pub struct MemoryMapInfo {
    pub regions: *mut MemoryRegion,
    pub count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub len: u64,
    pub kind: MemoryType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Free,       // Usable RAM
    Reserved,   // Hardware reserved / holes
    Kernel,     // Kernel code/data
    Acpi,       // ACPI Tables
    Nvs,        // Non-Volatile Storage (sleep data)
    Bootloader, // Reclaimable bootloader memory
}
