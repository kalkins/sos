pub mod frame;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct PhysAddr(u64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtAddr(usize);

impl PhysAddr {
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}
