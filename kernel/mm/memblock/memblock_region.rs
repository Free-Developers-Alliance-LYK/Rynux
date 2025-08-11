//! Memblock region implement

use core::ops::{Deref, DerefMut, Index, IndexMut};

use crate::mm::PhysAddr;
use crate::bitflags::bitflags;
use crate::macros::section_init_text;
use crate::types::ForStepResult;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// memblock region type flags
    pub struct MemBlockTypeFlags: u32 {
        /// Hotpluggable memory
        const NORMAL = 1 << 0;
        /// Mirror memory
        const MIRROR = 1 << 1;
        /// No map to virtual address
        const NOMAP = 1 << 2;
        /// reserved for kernel
        const RESERVED = 1 << 3;
        /// Driver managed
        const DRIVER_MANAGED = 1 << 4;
        /// Reserved for kernel, but not initialized
        const RESERVED_NO_INIT = 1 << 5;
    }
}

/// Memblock region
#[derive(Clone, Copy, Debug)]
pub struct MemBlockRegion {
    /// Base address of the memory block
    pub base: PhysAddr,
    /// Size of the memory block in bytes
    pub size: usize,
    /// Flags for the memory block type
    pub flags: MemBlockTypeFlags,
}

impl MemBlockRegion {
    const fn new(base: PhysAddr, size: usize, flags: MemBlockTypeFlags) -> Self {
        Self {
            base,
            size,
            flags,
        }
    }
}

const INIT_MEMBLOCK_MEMORY_REGIONS: usize = 128;
// Region storage
#[allow(dead_code)]
enum RegionStorage {
    Static {
        regions: [MemBlockRegion; INIT_MEMBLOCK_MEMORY_REGIONS],
        count: usize,
    },
    
    //* TODO: Support dynamic array
    //regions: Vec<MemBlockRegion>,
    Dynamic,
}

impl RegionStorage {
    #[inline(always)]
    fn len(&self) -> usize {
        match self {
            RegionStorage::Static { count, .. } => *count,
            RegionStorage::Dynamic => todo!(),
        }
    }

    fn insert_region(&mut self, idx: usize, region: MemBlockRegion) {
        match self {
            RegionStorage::Static { regions, count } => {
                assert!(*count < regions.len(), "no capacity to insert");
                assert!(idx <= *count, "idx out of range");

                // special case for empty array
                if *count == 0 {
                    regions[0] = region;
                    *count = 1;
                    return;
                }

                if idx < *count {
                    let slice = &mut regions[..=*count];
                    slice.copy_within(idx..*count, idx + 1);
                }

                regions[idx] = region;
                *count += 1;
            }
            RegionStorage::Dynamic => todo!(),
        }
    }

    fn remove_region(&mut self, idx: usize) {
        match self {
            RegionStorage::Static { regions, count } => {
                assert!(idx < *count, "idx out of range");

                let slice = &mut regions[..=*count];
                slice.copy_within(idx + 1..*count, idx);
                *count -= 1;

                // special case for empty array
                if *count == 0 {
                    regions[0] = MemBlockRegion::new(PhysAddr::from(0), 0, MemBlockTypeFlags::NORMAL);
                }
            }
            RegionStorage::Dynamic => todo!(),
        }
    }


    #[inline]
    fn split_mut_pair(&mut self, idx: usize) -> (&mut MemBlockRegion, &mut MemBlockRegion) {
        match self {
            RegionStorage::Static { regions, .. } => {
                let (a, b) = regions.split_at_mut(idx + 1);
                (&mut a[idx], &mut b[0])
            }
            RegionStorage::Dynamic => todo!(),
        }
    }


    // merge neighboring compatible regions
    //
    // # Arguments
    //
    // * `start_rgn` - Start scanning form `start_rgn - 1`
    // * `end_rgn` - End scanning at `end_rgn - 1`
    //
    #[section_init_text]
    fn merge_regions(&mut self, start_rgn: usize, mut end_rgn: usize) {
        if self.len() < 2 { return; }
        let mut i = if start_rgn != 0 { start_rgn - 1 } else { 0 };
        end_rgn = end_rgn.min(self.len() - 1);

        while i < end_rgn {
            let (this, next) = self.split_mut_pair(i);
            let this_end = this.base + this.size;
        
            let merge_able = this_end == next.base && this.flags == next.flags;
        
            if !merge_able {
                assert!(this_end <= next.base, "regions overlap or unordered");
                i += 1;
                continue;
            }

            this.size += next.size;
            {
                match self {
                    RegionStorage::Static { regions, count } => {
                        // move forward from next + 1, index of which is i + 2
                        let slice = &mut regions[.. *count];
                        slice.copy_within(i + 2 .. *count, i + 1);
                        *count -= 1;
                    }
                    RegionStorage::Dynamic => todo!(),
                }
            }
            end_rgn -= 1;
        }
    }
}

/// Present memory regions
#[allow(dead_code)] // TODO: Remove it after finishing
pub struct MemBlockRegionArray {
    // use array instead of Vec because we don't have allocator yet
    regions: RegionStorage,
    total_size: usize,
    name: &'static str,
}

impl Deref for MemBlockRegionArray {
    type Target = [MemBlockRegion];

    fn deref(&self) -> &Self::Target {
        match &self.regions {
            RegionStorage::Static { regions, count } => {
                &regions[..*count]
            }
            RegionStorage::Dynamic => {
                panic!("Dynamic not supported yet");
            }
        }
    }
}

impl DerefMut for MemBlockRegionArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.regions {
            RegionStorage::Static { regions, count } => {
                &mut regions[..*count]
            }
            RegionStorage::Dynamic => {
                panic!("Dynamic region storage is not supported yet");
            }
        }
    }
}

impl Index<usize> for MemBlockRegionArray {
    type Output = MemBlockRegion;

    fn index(&self, index: usize) -> &Self::Output {
        let slice: &[MemBlockRegion] = &**self; 
        assert!(index < slice.len(), "index out of bounds");
        &slice[index]
    }
}

impl IndexMut<usize> for MemBlockRegionArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice: &mut [MemBlockRegion] = &mut **self; 
        assert!(index < slice.len(), "index out of bounds");
        &mut slice[index]
    }
}

#[allow(dead_code)]
impl MemBlockRegionArray {
    /// Create a new MemBlockRegionArray with static storage
    #[inline(always)]
    pub const fn new_static(name: &'static str) -> Self {
        Self {
            regions: RegionStorage::Static {
                regions: [MemBlockRegion::new(PhysAddr::from(0), 0, MemBlockTypeFlags::NORMAL); INIT_MEMBLOCK_MEMORY_REGIONS],
                count: 0,
            },
            total_size: 0,
            name,
        }
    }

    fn upgrade_to_dynamic(&mut self) {
        todo!();
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.regions.len()
    }

    #[section_init_text]
    fn insert_region(&mut self, idx: usize, region: MemBlockRegion) {
        self.regions.insert_region(idx, region);
        self.total_size += region.size;
    }

    #[section_init_text]
    fn remove_region(&mut self, idx: usize) {
        let r_size = self[idx].size;
        self.total_size -= r_size;
        self.regions.remove_region(idx);
        if self.len() == 0 {
            assert!(self.total_size == 0);
        }
    }

    #[section_init_text]
    // support change index
    fn for_each(&mut self, idx: &mut usize, mut f: impl FnMut(&mut Self, usize) -> ForStepResult) {
        *idx = 0;
        while *idx < self.len() {
            match f(self, *idx) {
                ForStepResult::Next  => *idx += 1,
                ForStepResult::JumpTo(j) => *idx = j,
                ForStepResult::Break => break,
            }
        }
    }

    /// Add a new region 
    ///
    /// # Arguments
    ///
    /// * `base` - Base address of the new region
    /// * `size` - Size of the new region
    /// * `flags` - Flags of the new region
    ///
    /// Add new memblock region [@base, @base + @size) into @type.  The new region
    /// is allowed to overlap with existing ones - overlaps don't affect already
    /// existing regions.  @type is guaranteed to be minimal (all neighbouring
    /// compatible regions are merged) after the addition.
    ///
    #[section_init_text]
    pub fn add_range(&mut self, mut base: PhysAddr, mut size: usize, flags: MemBlockTypeFlags) {
        let end = base.saturating_add(size);
        // reset to real size for overflow
        size = end - base;

        if size == 0 {
            return ;
        }

        // special case for empty array
        if self.len() == 0 {
            assert!(self.total_size == 0);
            self.insert_region(0, MemBlockRegion::new(base, size, flags));
            return;
        }

        let mut nr_new = 0usize;
        let mut start_rgn: Option<usize> = None;
        let mut end_rgn = 0usize;
        let mut idx = 0usize;

        // All possible cases:
        //  low   ------------------------------------  high
        //           [reg.base            reg.end] 
        //      ]                   ]                          ] 
        //  [                    [                   [
        //
        // The left and right intervals can be arranged and combined
        //
        self.for_each(&mut idx, |this, id| {
            let r = this[id];
            let r_base = r.base;
            let r_end = r_base + r.size;

            // There is only one case where the interval is on the left
            // directly insert
            if r_base >= end {
                return ForStepResult::Break;
            }

            // The interval is on the right, continue to next
            if r_end <= base {
                return ForStepResult::Next;
            }

            // Left three case: 
            //  - intersect from below
            //  - fully contained
            //  - intersect from above
            //
            //  If it separates the lower part of new area, insert that portion.
            if r_base > base {
                // TODO:  warn if flags not match
                if flags != r.flags {
                    panic!("Cannot add region with different flags: {:?} != {:?}", flags, r.flags);
                }

                nr_new += 1;
                if start_rgn.is_none() {
                    start_rgn = Some(id);
                }

                end_rgn = id + 1;
                this.insert_region(id, MemBlockRegion::new(base, r_base - base, flags));
                //change base to r_end
                base = r_end.min(end); 
                return ForStepResult::JumpTo(id + 2);
            }

            //change base to r_end
            base = r_end.min(end); 
            ForStepResult::Next
        });

        if base < end {
            nr_new += 1;
            if start_rgn.is_none() {
                start_rgn = Some(idx);
            }

            end_rgn = idx + 1;
            self.insert_region(idx, MemBlockRegion::new(base, end - base, flags));
        }

        if nr_new == 0 {
            return;
        }

        self.regions.merge_regions(start_rgn.unwrap(), end_rgn);
    }

    // isolate given range into disjoint memblocks
    //
    // # Arguments
    //
    // * `base` - Base address of the range
    // * `size` - Size of the range
    //
    // Return:
    //  Some((start_rgn, end_rgn)) if success, None if fail
    //
    // Walk self and ensure that regions don't cross the boundaries defined by
    // [@base, @base + @size).  Crossing regions are split at the boundaries,
    // which may create at most two more regions.  The index of the first
    // region inside the range is returned in *@start_rgn and the index of the
    // first region after the range is returned in *@end_rgn.
    //
    #[section_init_text]
    fn isolate_range(&mut self, base: PhysAddr, mut size: usize) -> (usize, usize) {
        let end = base.saturating_add(size);
        // reset to real size for overflow
        size = end - base;

        if size == 0 {
            return (0, 0);
        }

        let mut idx = 0usize;
        let mut start_rgn: Option<usize> = None;
        let mut end_rgn = 0usize;

        // All possible cases:
        //  low   ------------------------------------  high
        //            [rbase            rend] 
        //       end              end                end
        //  base            base                base
        //
        // The left and right intervals can be arranged and combined
        //
        self.for_each(&mut idx, |this, id| {
            let r_base = this[id].base;
            let r_end =  this[id].base + this[id].size;
            let r_flags = this[id].flags;
            
            // The interval is on the left do nothing
            if r_base >= end {
                return ForStepResult::Break;
            }

            // The interval is on the right continue
            if r_end <= base {
                return ForStepResult::Next;
            }

            // Left three case: 
            //  - intersect from left
            //  - fully contained
            //  - intersect from right
            if r_base < base {
                // split and continue to process the next region - the new top half.
                this[id].base = base;
                this[id].size -= base - r_base;
                this.total_size -= base - r_base;
                this.insert_region(id, MemBlockRegion::new(r_base, base - r_base, r_flags));
            } else if r_end > end {
                // split and redo the current region - the new bottom half.
                this[id].base = end;
                this[id].size -= end - r_base;
                this.total_size -= end - r_base;
                this.insert_region(id, MemBlockRegion::new(r_base, end - r_base, r_flags));
                return ForStepResult::JumpTo(id);
            } else {
                // fully contained, record it
                if start_rgn.is_none() {
                    start_rgn = Some(id);
                }
                end_rgn = id + 1;
            }

            ForStepResult::Next
        });

        (start_rgn.unwrap_or(0), end_rgn)
    }

    /// Remove a range of memory regions
    #[section_init_text]
    pub fn remove_range(&mut self, base: PhysAddr, size: usize) {
        let (start_rgn, end_rgn) = self.isolate_range(base, size);
        for i in (start_rgn..end_rgn).rev() {
            self.remove_region(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mm::PhysAddr;
    use std::panic::AssertUnwindSafe;

    #[test]
    fn test_add_range_basic() {
        let mut memblock = MemBlockRegionArray::new_static("test");
        memblock.add_range(PhysAddr::from(0x1000), 0x1000, MemBlockTypeFlags::NORMAL);
        memblock.add_range(PhysAddr::from(0x3000), 0x1000, MemBlockTypeFlags::NORMAL);
        memblock.add_range(PhysAddr::from(0x5000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 3);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock[1].base, PhysAddr::from(0x3000));
        assert_eq!(memblock[1].size, 0x1000);
        assert_eq!(memblock[2].base, PhysAddr::from(0x5000));
        assert_eq!(memblock[2].size, 0x1000);
        assert_eq!(memblock.total_size, 0x3000);
        // now region is [0x1000, 0x2000), [0x3000, 0x4000), [0x5000, 0x6000)
        memblock.remove_range(PhysAddr::from(0x3000), 0x1000);
        assert_eq!(memblock.len(), 2);
        memblock.remove_range(PhysAddr::from(0x5000), 0x1000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock.total_size, 0x1000);
        // now region is [0x1000, 0x2000)
        // test neighboring regions would be merged
        memblock.add_range(PhysAddr::from(0x2000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x2000);
        assert_eq!(memblock.total_size, 0x2000);
        // now region is [0x1000, 0x3000)

        // test fully include region are not affected
        memblock.add_range(PhysAddr::from(0x1500), 0x500, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x2000);
        assert_eq!(memblock.total_size, 0x2000);
    
        // test not neighboring regions are not merged
        memblock.add_range(PhysAddr::from(0x4000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 2);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x2000);
        assert_eq!(memblock[1].base, PhysAddr::from(0x4000));
        assert_eq!(memblock[1].size, 0x1000);
        assert_eq!(memblock.total_size, 0x3000);

        // test bridge regions are merged
        memblock.add_range(PhysAddr::from(0x3000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x4000);
        assert_eq!(memblock.total_size, 0x4000);

        // test left and right regions are merged
        memblock.add_range(PhysAddr::from(0x0000), 0x2000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x0000));
        assert_eq!(memblock[0].size, 0x5000);
        assert_eq!(memblock.total_size, 0x5000);
        
        memblock.add_range(PhysAddr::from(0x4000), 0x2000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x0000));
        assert_eq!(memblock[0].size, 0x6000);
        assert_eq!(memblock.total_size, 0x6000);
    }

    #[test]
    fn test_add_range_with_flags() {
        let mut memblock = MemBlockRegionArray::new_static("test");
        memblock.add_range(PhysAddr::from(0x1000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock[0].flags, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.total_size, 0x1000);

        // test different flags not be merged
        memblock.add_range(PhysAddr::from(0x2000), 0x1000, MemBlockTypeFlags::MIRROR);
        assert_eq!(memblock.len(), 2);
        assert_eq!(memblock[1].base, PhysAddr::from(0x2000));
        assert_eq!(memblock[1].size, 0x1000);
        assert_eq!(memblock[1].flags, MemBlockTypeFlags::MIRROR);

        // test differen flags intersection would panic
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            memblock.add_range(PhysAddr::from(0x500), 0x1000, MemBlockTypeFlags::MIRROR);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_range_basic() {
        let mut memblock = MemBlockRegionArray::new_static("test");
        // now region is empty
        memblock.add_range(PhysAddr::from(0x1000), 0x3000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock.total_size, 0x3000);

        // now region is [0x1000, 0x4000)
        // remove a tail range
        memblock.remove_range(PhysAddr::from(0x3000), 0x1000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x2000);
        assert_eq!(memblock.total_size, 0x2000);

        // now region is [0x1000, 0x3000)
        // remove a head range
        memblock.remove_range(PhysAddr::from(0x1000), 0x1000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x2000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock.total_size, 0x1000);
        // now region is [0x2000, 0x3000)
    
        // recover the region
        memblock.add_range(PhysAddr::from(0x1000), 0x3000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x3000);
        assert_eq!(memblock.total_size, 0x3000);
        
        // now region is [0x1000, 0x4000)
        // remove a middle range
        memblock.remove_range(PhysAddr::from(0x2000), 0x1000);
        assert_eq!(memblock.len(), 2);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock[1].base, PhysAddr::from(0x3000));
        assert_eq!(memblock[1].size, 0x1000);
        assert_eq!(memblock.total_size, 0x2000);
        // now region is [0x1000, 0x2000), [0x3000, 0x4000)
        // recover 
        memblock.add_range(PhysAddr::from(0x2000), 0x1000, MemBlockTypeFlags::NORMAL);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x3000);
        assert_eq!(memblock.total_size, 0x3000);

        // now region is [0x1000, 0x4000)
        // remove a range that is not in the region
        memblock.remove_range(PhysAddr::from(0x5000), 0x1000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x1000));
        assert_eq!(memblock[0].size, 0x3000);
        assert_eq!(memblock.total_size, 0x3000);

        // now region is [0x1000, 0x4000)
        // remove left range intersecting the region
        memblock.remove_range(PhysAddr::from(0x0000), 0x2000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x2000));
        assert_eq!(memblock[0].size, 0x2000);
        assert_eq!(memblock.total_size, 0x2000);

        // now region is [0x2000, 0x4000)
        // remove right range intersecting the region
        memblock.remove_range(PhysAddr::from(0x3000), 0x2000);
        assert_eq!(memblock.len(), 1);
        assert_eq!(memblock[0].base, PhysAddr::from(0x2000));
        assert_eq!(memblock[0].size, 0x1000);
        assert_eq!(memblock.total_size, 0x1000);
        // now region is [0x2000, 0x3000)
    }
}


