use core::slice::{from_raw_parts, from_raw_parts_mut};

use boot_info::memory::{MemoryMapInfo, MemoryRegion, MemoryType};
use log::{debug, trace, warn};

use super::PhysAddr;

const FRAME_SIZE: u64 = 4096;

#[derive(PartialEq, Eq, Clone, Copy)]
enum FrameState {
    Allocated,
    Free,
}

pub struct FrameAllocator<'a> {
    /** A bitmap where each bit represents a frame, 0=free, 1=allocated */
    bitmap: &'a mut [u64],
    num_frames: u64,
    next_free_index: Option<u64>,
}

impl<'a> FrameAllocator<'a> {
    pub unsafe fn init(memory_map: MemoryMapInfo) -> Self {
        let regions: &[MemoryRegion] =
            unsafe { from_raw_parts(memory_map.regions, memory_map.count) };

        let region_iter = regions.iter().filter(|r| r.kind == MemoryType::Free);

        // Find the usable range
        let max_addr = region_iter
            .clone()
            .map(|&r| r.start + r.len)
            .max()
            .unwrap_or(0);

        let num_frames = max_addr / FRAME_SIZE;

        // Create a bitmap of the frames in the usable range
        let num_segments: usize = ((num_frames + 63) / 64).try_into().expect("The number of 64-bit segments in the frame allocator bitmap is too large to index");
        let bitmap_size = num_segments * size_of::<u64>();

        // Find the first frame big enough for the bitmap
        let bitmap_region_start = region_iter
            .clone()
            .filter(|&&r| r.kind == MemoryType::Free)
            .filter_map(|&r| {
                // If the region starts at 0x0, which is an illegal address in Rust,
                // ignore the first frame of the region.
                if r.start == 0 && r.len - FRAME_SIZE >= bitmap_size as u64 {
                    Some(r.start + FRAME_SIZE)
                } else if r.len >= bitmap_size as u64 {
                    Some(r.start)
                } else {
                    None
                }
            })
            .next()
            .expect("Not enough RAM to store the allocator metadata");

        trace!(
            "Allocating frame bitmap at 0x{:x}-0x{:x}",
            bitmap_region_start,
            bitmap_region_start + bitmap_size as u64
        );

        let bitmap: &'a mut [u64] =
            unsafe { from_raw_parts_mut(bitmap_region_start as *mut u64, num_segments) };

        // Set every frame as used, to be on the safe side, then mark the free ones
        for segment in bitmap.iter_mut() {
            *segment = u64::MAX;
        }

        let mut allocator = Self {
            bitmap: bitmap,
            num_frames: num_frames,
            next_free_index: None,
        };

        // Mark all free regions
        for region in region_iter.clone() {
            if region.kind == MemoryType::Free {
                allocator.set_region(region.start, region.len, FrameState::Free);
            }
        }

        // Explicitly ban address 0x0
        allocator.set_state(0, FrameState::Allocated);

        // Explicitly mark bitmap as allocated
        allocator.set_region(
            bitmap_region_start,
            bitmap_size as u64,
            FrameState::Allocated,
        );

        debug!(
            "Frame allocator has {} free bytes",
            (0..allocator.num_frames)
                .filter(|i| allocator.get_state(*i) == FrameState::Free)
                .count() as u64
                * FRAME_SIZE
        );

        allocator
    }

    pub fn allocate_frame(&mut self) -> Option<PhysAddr> {
        if let Some(frame_index) = self.next_free_index {
            if self.get_state(frame_index) == FrameState::Free {
                self.set_state(frame_index, FrameState::Allocated);

                Some(PhysAddr(frame_index as u64 * FRAME_SIZE))
            } else {
                panic!(
                    "Inconsistent state: Recorded next free index {} is not free",
                    frame_index
                )
            }
        } else {
            None
        }
    }

    pub fn free_frame(&mut self, addr: PhysAddr) {
        let frame_index = addr.to_u64() / FRAME_SIZE;
        self.set_state(frame_index, FrameState::Free);
    }

    fn set_region(&mut self, addr: u64, len: u64, state: FrameState) {
        let start_index = addr / FRAME_SIZE;
        let num_frames = (len + FRAME_SIZE - 1) / FRAME_SIZE;

        for i in 0..num_frames {
            self.set_state(start_index + i, state);
        }
    }

    fn get_state(&self, frame_index: u64) -> FrameState {
        if frame_index < self.num_frames && let Ok(segment) = usize::try_from(frame_index / 64) {
            let mask = 1 << (frame_index % 64);

            let is_allocated = self.bitmap[segment] & mask != 0;

            if is_allocated {
                FrameState::Allocated
            } else {
                FrameState::Free
            }
        } else {
            FrameState::Allocated
        }
    }

    fn set_state(&mut self, frame_index: u64, state: FrameState) {
        if frame_index < self.num_frames && let Ok(segment) = usize::try_from(frame_index / 64) {
            let mask = 1 << (frame_index % 64);

            if state == FrameState::Allocated {
                self.bitmap[segment] |= mask;

                if let Some(free_index) = self.next_free_index && free_index == frame_index {
                    self.next_free_index = self.find_next_free();

                    if self.next_free_index.is_none() {
                        warn!("There are no more free physical memory");
                    }
                }
            } else {
                self.bitmap[segment] &= !mask;

                if self.next_free_index.is_none() {
                    self.next_free_index = Some(frame_index);
                }
            }
        } else {
            warn!("Could not set state for frame {frame_index}. The index is outside the usable memory range.");
        }
    }

    fn find_next_free(&self) -> Option<u64> {
        let search_start = self.next_free_index.unwrap_or(0);

        // First loop through the rest of the bitmap, as that is more likely to be free,
        // then check from the start.
        for frame_index in (search_start..self.num_frames).chain(0..search_start) {
            if self.get_state(frame_index) == FrameState::Free {
                return Some(frame_index);
            }
        }

        None
    }
}
