//! Memblock mem managemnt
mod memblock_region;
pub use memblock_region::{MemBlockTypeFlags,MemBlockRegionArray, MemBlockRegion};

use core::ptr::NonNull;
use crate::mm::{
    PhysAddr,VirtAddr,
    page::PageConfig,
};

use crate::macros::{section_init_data, section_init_text};
use crate::sync::lock::RawSpinNoPreemptLockIrq;
use crate::alloc::{AllocError, AllocFlags};

/// Memblock
#[allow(dead_code)] // TODO: Remove it after finishing
pub struct MemBlock {
    bottom_up: bool,
    current_limit: PhysAddr,
    // Present memory
    memory: MemBlockRegionArray,
    // Reserved memory
    reserved: MemBlockRegionArray,
}

/// Iterator over free memory regions
pub struct FreeMemIter<'a> {
    mem: &'a [MemBlockRegion],
    reserved: &'a [MemBlockRegion],
    flags: MemBlockTypeFlags,
    idx_mem: usize,
    idx_res: usize,

    idx_mem_back: usize, // initial is mem.len()
    idx_res_back: usize, // initial is reserved.len()
                   
}

impl<'a> FreeMemIter<'a> {
    fn new(mem: &'a [MemBlockRegion], reserved: &'a [MemBlockRegion],
        flags: MemBlockTypeFlags) -> Self {
        FreeMemIter {
            mem,
            reserved,
            flags,
            idx_mem: 0,
            idx_res: 0,
            idx_mem_back: mem.len(),
            idx_res_back: reserved.len(),
        }
    }

    // nomap and device managed shoud not be used, unless explicitly specified, 
    // when specified mirror,can only use mirror region
    #[inline(always)]
    fn should_skip_region(region: &MemBlockRegion, flags: MemBlockTypeFlags) -> bool {
        // if we want mirror memory skip non-mirror memory regions
        if flags.contains(MemBlockTypeFlags::MIRROR) && !region.flags
        .contains(MemBlockTypeFlags::MIRROR) {
            return true;
        }

        // skip nomap mem unless we were asked for it explicitly
        if !flags.contains(MemBlockTypeFlags::NOMAP) && region.flags.contains(MemBlockTypeFlags::NOMAP) {
            return true;
        }

        // skip driver-managed mem unless we were asked for it explicitly
        if !flags.contains(MemBlockTypeFlags::DRIVER_MANAGED) && region.flags.contains(MemBlockTypeFlags::DRIVER_MANAGED) {
            return true;
        }

        false
    }
}

impl<'a> Iterator for FreeMemIter<'a> {
    type Item = (PhysAddr, PhysAddr);

    fn next(&mut self) -> Option<Self::Item> {
        for (idx_m,mem_region) in self.mem.iter().enumerate().skip(self.idx_mem as usize) {
            if Self::should_skip_region(mem_region, self.flags) {
                continue;
            }

            let mem_start = mem_region.base;
            let mem_end = mem_start + mem_region.size;

            // we are looking for avaiable reserved reegion than include mem_region
            // There are these cases: 
            //                       m_start                   m_end
            //          r_end
            //                                        r_end    
            //                                                                r_end 
            // r_start                       rstart                   rstart
            //
            // only one case is ok,
            //      m_start              m_end 
            //  r_start                     r_end
            //
            for idx_r in self.idx_res..=self.reserved.len() {
                let res_start = if idx_r == 0 {
                    PhysAddr::from(0)
                } else {
                    self.reserved[idx_r - 1].base + self.reserved[idx_r - 1].size
                };

                let res_end = if idx_r < self.reserved.len() {
                    self.reserved[idx_r].base
                } else {
                    PhysAddr::from(usize::MAX)
                };
                
                // if idx_r advaced past idx_m,mem step continue
                if res_start >= mem_end {
                    break;
                }

                // if the two regions intersect, done 
                if mem_start < res_end {
                    let found_start = mem_start.max(res_start);
                    let found_end = mem_end.min(res_end);
                    // The region which ends first is advanced for the next
                    if mem_end <= res_end {
                        self.idx_mem = idx_m + 1;
                    } else {
                        self.idx_res = idx_r + 1;
                    }
                    return Some((found_start, found_end));
                }
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // We don't know the exact size, so we return None
        (0, None)
    }
}

impl DoubleEndedIterator for FreeMemIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        for idx_m  in (0..self.idx_mem_back).rev() {
            let mem_region = &self.mem[idx_m];
            if Self::should_skip_region(mem_region, self.flags) {
                continue;
            }

            let mem_start = mem_region.base;
            let mem_end = mem_start + mem_region.size;


            // we are looking for avaiable reserved reegion than include mem_region
            // There are these cases: 
            //                       m_start                   m_end
            //          r_end
            //                                        r_end    
            //                                                                r_end 
            // r_start                       rstart                   rstart
            //
            // only one case is ok,
            //      m_start              m_end 
            //  r_start                     r_end
            //
            for idx_r in (0..=self.idx_res_back).rev() {
                let res_start = if idx_r == 0 {
                    PhysAddr::from(0)
                } else {
                    self.reserved[idx_r - 1].base + self.reserved[idx_r - 1].size
                };

                let res_end = if idx_r < self.reserved.len() {
                    self.reserved[idx_r].base
                } else {
                    PhysAddr::from(usize::MAX)
                };
                
                // if idx_r advaced past idx_m break
                if res_end <= mem_start {
                    break;
                }

                // if the two regions intersect, done 
                if mem_end > res_start {
                    let found_start = mem_start.max(res_start);
                    let found_end = mem_end.min(res_end);
                    // The region which ends first is advanced for the next
                    if mem_start >= res_start {
                        self.idx_mem_back = idx_m.saturating_sub(1);
                    } else {
                        self.idx_res_back = idx_r.saturating_sub(1);
                    }
                    return Some((found_start, found_end));
                }
            }
        }
        None

    }
}



impl MemBlock {
    // When set MEMBLOCK_ALLOC_ACCESSIBLE, it will limit with memblock 
    // current limit
    const MEMBLOCK_ALLOC_ACCESSIBLE: PhysAddr = PhysAddr::from(0);

    // Our default alloc policy is top-down, so it is safe to use 0
    // but should never use the first page
    const MEMBLOCK_ALLOC_LOW_LIMIT: PhysAddr = PhysAddr::from(PageConfig::PAGE_SIZE);

    // Alloc anywhere in the memblock
    const MEMBLOCK_ALLOC_ANYWHERE: usize = usize::MAX;

    /// is the allocator bottom-up?
    #[inline(always)]
    pub fn bottom_up(&self) -> bool {
        self.bottom_up
    }

    /// Add new memory region
    /// 
    /// # Arguments
    /// 
    /// * `base` - Base address of the new region
    /// * `size` - Size of the new region
    ///
    #[inline(always)]
    pub fn add_memory(&mut self, base: PhysAddr, size: usize) {
        self.memory.add_range(base, size, MemBlockTypeFlags::NORMAL);
    }

    /// Remove a memory region
    /// 
    /// # Arguments
    /// 
    /// * `base` - Base address of the region
    /// * `size` - Size of the region
    ///
    #[inline(always)]
    pub fn remove_memory(&mut self, base: PhysAddr, size: usize) {
        self.memory.remove_range(base, size);
    }

    /// Add new reserved region
    /// 
    /// # Arguments
    /// 
    /// * `base` - Base address of the new region
    /// * `size` - Size of the new region
    ///
    #[inline(always)]
    pub fn add_reserved(&mut self, base: PhysAddr, size: usize) {
        self.reserved.add_range(base, size, MemBlockTypeFlags::NORMAL);
    }

    /// Remove a reserved region
    /// 
    /// # Arguments
    /// 
    /// * `base` - Base address of the region
    /// * `size` - Size of the region
    ///
    #[allow(dead_code)] // TODO: Remove it after finishing
    #[inline(always)]
    fn remove_reserved(&mut self, base: PhysAddr, size: usize) {
        self.reserved.remove_range(base, size);
    }

    #[inline(always)]
    fn iter_free(&self, flags: MemBlockTypeFlags) -> FreeMemIter<'_> {
        FreeMemIter::new(
            &self.memory,
            &self.reserved,
            flags,
        )
    }

    #[section_init_text]
    fn find_free_mem_range_bottom_up(
        &mut self,
        start: PhysAddr,
        end: PhysAddr,
        size: usize,
        align: usize,
        flags: MemBlockTypeFlags,
    ) -> Option<PhysAddr> {
        // Iterate over memory regions and find a free range
        let iter = self.iter_free(flags);
        for (free_start, free_end) in iter {
            // clamp the region inside [start, end)
            let clamped_start = free_start.max(start);
            let clamped_end = free_end.min(end);
            // align the starting address up
            let aligned = clamped_start.align_up(align);

            #[cfg(test)]
            {
                println!(
                    "find_free_mem_range_bottom_up: free_start: {:?}, free_end: {:?}, clamped_start: {:?}, clamped_end: {:?}, aligned: {:?}",
                    free_start, free_end, clamped_start, clamped_end, aligned
                );
            }
            // if enough space, return the aligned address
            if aligned < clamped_end && (clamped_end - aligned) >= size {
                return Some(aligned);
            }
        }
        None
    }

    #[section_init_text]
    fn find_free_mem_range_top_down(
        &mut self,
        start: PhysAddr,
        end: PhysAddr,
        size: usize,
        align: usize,
        flags: MemBlockTypeFlags,
    ) -> Option<PhysAddr> {
        // Iterate over memory regions and find a free range
        let iter = self.iter_free(flags).rev();
        for (free_start, free_end) in iter {
            let this_start = free_start.max(start);
            let this_end   = free_end.min(end);

            // is enough space?
            let end_minus_size = match this_end.checked_sub(PhysAddr::from(size)) {
                Some(v) => v,
                None => continue,
            };

            // align the end address down
            let cand = end_minus_size.align_down(align);

            if cand >= this_start {
                return Some(cand);
            }

        }
        None
    }

    /// Find a range of memory that is available for allocation
    #[section_init_text]
    fn find_free_mem_range(
        &mut self,
        start: PhysAddr,
        end: PhysAddr,
        align: usize,
        size: usize,
        flags: MemBlockTypeFlags,
    ) -> Option<PhysAddr> {
        if self.bottom_up {
            self.find_free_mem_range_bottom_up(start, end, size, align, flags)
        } else {
            self.find_free_mem_range_top_down(start, end, size, align, flags)
        }
    }

    #[section_init_text]
    fn alloc_phys_with_limit(
        &mut self,
        size: usize,
        align: usize,
        low_limit: PhysAddr,
        high_limit: PhysAddr,
    ) -> Result<PhysAddr, AllocError> {
        // Check size
        if size == 0 {
            return Err(AllocError::InvalidSize);
        }

        // check align
        if align == 0 || !align.is_power_of_two() {
            return Err(AllocError::InvalidAlign);
        }

        let mut start = low_limit;
        let mut end = high_limit;

        // if high_limit is MEMBLOCK_ALLOC_ACCESSIBLE, we should limit it to current limit
        if end == Self::MEMBLOCK_ALLOC_ACCESSIBLE {
            end = self.current_limit;
        } else {
            end = high_limit.min(self.current_limit);
        }

        // The first page should never be allocated
        start = start.max(PhysAddr::from(PageConfig::PAGE_SIZE));

        let found = self.find_free_mem_range(
            start,
            end,
            align,
            size,
            MemBlockTypeFlags::NORMAL,
        );

        if let Some(phys) = found {
            // add found range to reserved memory
            self.add_reserved(phys, size);
            Ok(phys)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    /// Alloc phys memory from the memblock allocator
    #[inline]
    pub fn alloc_phys(&mut self, size: usize, align: usize) -> Result<PhysAddr, AllocError> {
        self.alloc_phys_with_limit(size, align, Self::MEMBLOCK_ALLOC_LOW_LIMIT, Self::MEMBLOCK_ALLOC_ACCESSIBLE)
    }

    /// Free phys memory allocated by the memblock allocator
    #[inline]
    pub fn free_phys(&mut self, phys: PhysAddr, size: usize) {
        // Remove the reserved memory region
        self.remove_reserved(phys, size);
    }

    /// Alloc memory from the memblock allocator
    #[inline]
    pub fn alloc(&mut self, size: usize, align: usize, _flags: AllocFlags) -> Result<NonNull<u8>, AllocError> {

        let phys = self.alloc_phys_with_limit(size, align, Self::MEMBLOCK_ALLOC_LOW_LIMIT, Self::MEMBLOCK_ALLOC_ACCESSIBLE)?;

        let virt_addr = phys.to_virt();

        // Zero the memory
        // SAFETY: virt_addr is a valid virtual address and size is valid
        unsafe {
            let raw_ptr = virt_addr.as_mut_ptr();
            raw_ptr.write_bytes(0, size);
            Ok(NonNull::new(raw_ptr).ok_or(AllocError::NoMemory)?)
        }
    }

    /// Free memory allocated by the memblock allocator
    #[inline]
    pub fn free(&mut self, ptr: NonNull<u8>, size: usize) {
        // Remove the reserved memory region
        // SAFETY: We assume that the memory was allocated by this allocator
        // and the size is correct
        let virt_addr = VirtAddr::from(ptr.as_ptr() as usize);
        let phys = virt_addr.to_phys();
        self.free_phys(phys, size);
    }
}

#[cfg(not(test))]
#[section_init_data]
#[allow(dead_code)] // TODO: Remove it after finishing
/// Global static instance of the MemBlock allocator.
/// Actually MEMBLOCK doesn't need to be protect, we use `RawSpinNoPreemptLockIrq`
/// to ensure that the allocator will only be used in: 
///  - the initialization phase before scheduling is enabled
///  - avoid use unsafe to access it, because it is only used in the initialization phase
static MEMBLOCK: RawSpinNoPreemptLockIrq<MemBlock> = RawSpinNoPreemptLockIrq::new(MemBlock {
    bottom_up: false,
    current_limit: PhysAddr::from(MemBlock::MEMBLOCK_ALLOC_ANYWHERE),
    memory: MemBlockRegionArray::new_static("memory"),
    reserved: MemBlockRegionArray::new_static("reserved"),
}, Some("MEMBLOCK"));

#[cfg(test)]
mod tests {
    use super::*;

    fn new_memblock() -> MemBlock {
        MemBlock {
            bottom_up: true,
            current_limit: PhysAddr::from(MemBlock::MEMBLOCK_ALLOC_ANYWHERE),
            memory: MemBlockRegionArray::new_static("memory"),
            reserved: MemBlockRegionArray::new_static("reserved"),
        }
    }

    fn assert_reserved(idx: usize, memblock: &MemBlock, base: PhysAddr, size: usize) {
        let reserved = memblock.reserved.get(idx);
        assert!(reserved.is_some(), "Reserved memory at index {} should exist", idx);
        let reserved = reserved.unwrap();
        assert_eq!(reserved.base, base, "Reserved memory base at index {} should be {:?}", idx, base);
        assert_eq!(reserved.size, size, "Reserved memory size at index {} should be {}", idx, size);
    }

    #[test]
    fn test_alloc_basic() {
        let mut memblock = new_memblock();
        // Add some memory regions 
        memblock.add_memory(PhysAddr::from(0x1000), 0x1000);
        // now we have two memory regions: [0x1000, 0x2000)
        let alloc_size = 0x1000;
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_ok(), "Allocation should succeed");
        let phys = phys.unwrap();
        assert_eq!(phys, PhysAddr::from(0x1000), "Allocated address should be 0x1000");
        // Now we should have reserved memory at 0x1000
        assert_reserved(0, &memblock, PhysAddr::from(0x1000), alloc_size);
        // free
        memblock.free_phys(phys, alloc_size);

        // continue allocating memory
        let phys2 = memblock.alloc_phys(alloc_size, align);
        assert!(phys2.is_ok(), "Second allocation should succeed");
        let phys2 = phys2.unwrap();
        assert_eq!(phys2, PhysAddr::from(0x1000), "Second allocated address should be 0x1000");
        // Now we should have two reserved memory regions: [0x1000, 0x2000) and [0x3000, 0x4000)
        assert_reserved(0, &memblock, PhysAddr::from(0x1000), alloc_size);

        // no memory left for allocation
        let phys3 = memblock.alloc_phys(alloc_size, align);
        assert!(phys3.is_err(), "Third allocation should fail, no memory left");
        let err = phys3.unwrap_err();
        assert_eq!(err, AllocError::NoMemory, "Expected NoMemory error for third allocation");
    }

    #[test]
    fn test_alloc_basic_top_down() {
        let mut memblock = new_memblock(); 
        memblock.bottom_up = false; // Set to top-down allocation
        // Add some memory regions
        memblock.add_memory(PhysAddr::from(0x1000), 0x1000);
        memblock.add_memory(PhysAddr::from(0x3000), 0x1000);
        // now we have two memory regions: [0x1000, 0x2000) and [0x3000, 0x4000)
        // Allocate memory
        let alloc_size = 0x1000;
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_ok(), "Allocation should succeed");
        let phys = phys.unwrap();
        assert_eq!(phys, PhysAddr::from(0x3000), "Allocated address should be 0x3000");
        // Now we should have reserved memory at 0x3000
        assert_reserved(0, &memblock, PhysAddr::from(0x3000), alloc_size);
 
        // continue allocating memory
        let phys2 = memblock.alloc_phys(alloc_size, align);
        assert!(phys2.is_ok(), "Second allocation should succeed");
        let phys2 = phys2.unwrap();
        assert_eq!(phys2, PhysAddr::from(0x1000), "Second allocated address should be 0x1000");
        // Now we should have two reserved memory regions: [0x3000, 0x4000) and [0x1000, 0x2000)
        assert_reserved(1, &memblock, PhysAddr::from(0x3000), alloc_size);
        assert_reserved(0, &memblock, PhysAddr::from(0x1000), alloc_size);
    }

    #[test]
    fn test_alloc_size_larger() {
        let mut memblock = new_memblock();
        // Add some memory regions now we have 0x1000-0x3000
        memblock.add_memory(PhysAddr::from(0x1000), 0x2000);
        
        // Try to allocate larger than available size
        let alloc_size = 0x3000; // Larger than the available memory
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_err(), "Allocation with size larger than available should fail");
        let err = phys.unwrap_err();
        assert_eq!(err, AllocError::NoMemory, "Expected NoMemory error for allocation larger than available");

        memblock.add_memory(PhysAddr::from(0x5000), 0x3000);
        // Now we have [0x1000, 0x3000)  [0x5000, 0x8000) is available
        // try to allocate again
        let alloc_size = 0x3000;
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_ok(), "Allocation with size 0x3000 should succeed");
        let phys = phys.unwrap();
        assert_eq!(phys, PhysAddr::from(0x5000), "Allocated address should be 0x5000");
        // Now we should have reserved memory at 0x5000
        assert_reserved(0, &memblock, PhysAddr::from(0x5000), alloc_size);

        // Try to allocate again, should fail
        let phys2 = memblock.alloc_phys(alloc_size, align);
        assert!(phys2.is_err(), "Second allocation with size 0x3000 should fail, no memory left");
        let err = phys2.unwrap_err();
        assert_eq!(err, AllocError::NoMemory, "Expected NoMemory error for second allocation with size 0x3000");

        // allocate smaller size
        let alloc_size = 0x1000; // Smaller than the available memory
        let phys3 = memblock.alloc_phys(alloc_size, align);
        assert!(phys3.is_ok(), "Allocation with size 0x1000 should succeed");
        let phys3 = phys3.unwrap();
        assert_eq!(phys3, PhysAddr::from(0x1000), "Allocated address should be 0x8000");
        // Now we should have two reserved memory
        assert_reserved(0, &memblock, PhysAddr::from(0x1000), 0x1000);
        assert_reserved(1, &memblock, PhysAddr::from(0x5000), 0x3000);
    }

    #[test]
    fn test_alloc_size_larger_top_down() {
        let mut memblock = new_memblock();
        memblock.bottom_up = false; // Set to top-down allocation
        // Add some memory regions now we have 0x1000-0x3000
        memblock.add_memory(PhysAddr::from(0x1000), 0x2000);
        
        // Try to allocate larger than available size
        let alloc_size = 0x3000; // Larger than the available memory
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_err(), "Allocation with size larger than available should fail");
        let err = phys.unwrap_err();
        assert_eq!(err, AllocError::NoMemory, "Expected NoMemory error for allocation larger than available");

        memblock.add_memory(PhysAddr::from(0x5000), 0x3000);
        memblock.add_memory(PhysAddr::from(0x9000), 0x3000);
        // Now we have [0x1000, 0x3000)  [0x5000, 0x8000) [0x9000,0xc0000) is available
        // try to allocate again
        let alloc_size = 0x3000;
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_ok(), "Allocation with size 0x3000 should succeed");
        let phys = phys.unwrap();
        assert_eq!(phys, PhysAddr::from(0x9000), "Allocated address should be 0x9000");
        // Now we should have reserved memory at 0x9000
        assert_reserved(0, &memblock, PhysAddr::from(0x9000), alloc_size);

        // allocate smaller size
        let alloc_size = 0x1000; // Smaller than the available memory
        let phys3 = memblock.alloc_phys(alloc_size, align);
        assert!(phys3.is_ok(), "Allocation with size 0x1000 should succeed");
        let phys3 = phys3.unwrap();
        assert_eq!(phys3, PhysAddr::from(0x7000), "Allocated address should be 0x7000");
        // Now we should have two reserved memory
        assert_reserved(0, &memblock, PhysAddr::from(0x7000), 0x1000);
        assert_reserved(1, &memblock, PhysAddr::from(0x9000), 0x3000);
    }

    #[test]
    fn test_alloc_different_fragmented_memory() {
        let mut memblock = new_memblock();
        // Add some memory regions
        memblock.add_memory(PhysAddr::from(0x1000), 0x1000);
        
        // Now we have three memory regions: [0x1000, 0x2000)
        // test alloc size: 2 4 6 8 10
        let alloc_sizes = [2, 4, 6, 8, 10];
        let align = 4; // Align to 4 bytes
        for &size in &alloc_sizes {
            let phys = memblock.alloc_phys(size, align);
            assert!(phys.is_ok(), "Allocation with size {} should succeed", size);
            let phys = phys.unwrap();
            // Check if the allocated address is aligned
            assert!(phys.as_usize() % align == 0, "Allocated address for size {} should be aligned to {}", size, align);
        }
    }

    #[test]
    fn test_alloc_with_invalid_size() {
        let mut memblock = new_memblock();
        // Add some memory regions 
        memblock.add_memory(PhysAddr::from(0x1000), 0x1000);
        
        // Try to allocate with size 0
        let alloc_size = 0;
        let align = 0x1000;
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_err(), "Allocation with size 0 should fail");
        let err = phys.unwrap_err();
        assert_eq!(err, AllocError::InvalidSize, "Expected InvalidSize error for allocation with size 0");
    }

    #[test]
    fn test_alloc_with_invalid_align() {
        let mut memblock = new_memblock();
        // Add some memory regions 
        memblock.add_memory(PhysAddr::from(0x1000), 0x1000);
        
        // Try to allocate with invalid alignment
        let alloc_size = 0x1000;
        let align = 0; // Invalid alignment
        let phys = memblock.alloc_phys(alloc_size, align);
        assert!(phys.is_err(), "Allocation with invalid alignment should fail");
        let err = phys.unwrap_err();
        assert_eq!(err, AllocError::InvalidAlign, "Expected InvalidAlign error for allocation with invalid alignment");
    }
}

