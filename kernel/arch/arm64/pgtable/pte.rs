//! Arm64 Page table PTE

use crate::{arch::arm64::pgtable::config::Arm64PgtableConfig, cfg_if, mm::page::PageConfig};
use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr::NonNull,
};

use super::{PgTableEntry, PtePgProt};
use crate::mm::{PhysAddr, VirtAddr};

/// Pte
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PteEntry(u64);

#[allow(dead_code)]
impl PteEntry {
    // Address mask
    const PTE_ADDR_LOW_MASK: u64 = ((1 << (50 - PteTable::SHIFT)) - 1) << PteTable::SHIFT;

    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
            if #[cfg(CONFIG_ARM64_64K_PAGES)] {
                use klib::bits::genmask_ull;
                const PTE_ADDR_HIGH_MASK: u64 = 0xf << 12;
                const PTE_ADDR_HIGH_SHIFT: u64 = 36;
                const PHYS_TO_PTE_ADDR_MASK: u64 = Self::PTE_ADDR_LOW_MASK | Self::PTE_ADDR_HIGH_MASK;
            } else {
                /*
                 * PA is 52bit, lower 12 bit is not used(Page Aligned),
                 * still need to store 40bit(12-52)
                 * PTE only (12-50 bit) can store PA, still need to store 51-52bit
                 * in PTE(9,10)
                 */
                const PTE_ADDR_HIGH_MASK: u64 = 0x3 << 8;
                const PTE_ADDR_HIGH_SHIFT: u64 = 42;
                const PHYS_TO_PTE_ADDR_MASK: u64 = genmask_ull(49, 8);
            }
        }
    }

    /// Create a new Pte
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Self(val)
    }

    /// Pte is valid
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        (self.0 & PtePgProt::PTE_VALID.bits()) != 0
    }

    /// Get phys pfn
    #[inline(always)]
    pub fn pfn(&self) -> usize {
        self.to_phys().as_usize() >> PageConfig::PAGE_SHIFT
    }

    /// Pte is contiguous
    #[inline(always)]
    pub fn is_contiguous(&self) -> bool {
        (self.0 & PtePgProt::PTE_CONT.bits()) != 0
    }
}

impl PgTableEntry for PteEntry {
    /// Value
    #[inline(always)]
    fn value(&self) -> u64 {
        self.0
    }

    /// Check if the pte is none
    #[inline(always)]
    fn is_none(&self) -> bool {
        self.0 == 0
    }
    /// Get the value of the Pte
    #[inline(always)]
    fn read(&self) -> u64 {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    #[inline(always)]
    /// write the pte
    fn write(&mut self, val: u64) {
        unsafe { core::ptr::write_volatile(&mut self.0, val) }
    }

    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
            /// Consume the pte and convert it to a physical address
            #[inline(always)]
            fn to_phys(&self) -> u64 {
                // remove the maybe shared bit
                let val = self.0 & !PgProt::pte_maybe_shared().bits();
                let low_addr = val & Self::PTE_ADDR_LOW_MASK;
                let high_addr = (val & Self::PTE_ADDR_HIGH_MASK) << Self::PTE_ADDR_HIGH_SHIFT;
                low_addr | high_addr
            }

            /// Convert a physical address to a pte
            #[inline(always)]
            fn from_phys(pa: usize) -> Self {
                let pa = pa as u64;
                Self((pa | (pa >> Self::PTE_ADDR_HIGH_SHIFT)) & Self::PHYS_TO_PTE_ADDR_MASK)
            }

        } else {
            /// Consume the pte and convert it to a physical address
            #[inline(always)]
            fn to_phys(&self) -> PhysAddr {
                PhysAddr::from((self.value() & Self::PTE_ADDR_LOW_MASK) as usize)
            }

            /// Convert a physical address to a pte
            #[inline(always)]
            fn from_phys(pa: PhysAddr) -> Self {
                let pa = pa.as_usize() as u64;
                Self(pa)
            }
        }
    }
}

/// PteTable
pub struct PteTable {
    base: NonNull<PteEntry>,
    _marker: PhantomData<PteEntry>,
    len: usize,
}

#[allow(dead_code)]
impl PteTable {
    // Returns the ARM64 CONT_PTE_SHIFT for a given page shift (log2 of page size).
    //
    // # Arguments
    // * `page_shift` - The log2 of the page size in bytes. (e.g., 12 for 4KB, 14 for 16KB, 16 for 64KB)
    //
    // # Returns
    // * The CONT_PTE_SHIFT value, i.e., log2 of the number of pages that can be merged into a contiguous PTE.
    //
    // # ARM64 logic:
    // - For 4KB pages (page_shift = 12):  CONT_PTE_SHIFT = 4  (16 pages, 64KB block)
    // - For 16KB pages (page_shift = 14): CONT_PTE_SHIFT = 7  (128 pages, 2MB block)
    // - For 64KB pages (page_shift = 16): CONT_PTE_SHIFT = 5  (32 pages, 2MB block)
    // - Otherwise, defaults to 4 (for safety; can be adjusted for other configurations).
    const fn pte_cont_shift(page_shift: usize) -> usize {
        match page_shift {
            12 => 4, // 4KB pages
            14 => 7, // 16KB pages
            16 => 5, // 64KB pages
            _ => 4,  // Default/fallback value
        }
    }

    // determines the size a pte page table entry can map
    const SHIFT: usize = PageConfig::PAGE_SHIFT;
    // Size of a PTE entry in bytes.
    const ENTRY_SIZE: usize = 1 << Self::SHIFT;
    // Mask for aligning to a PTE entry
    const MASK: usize = !(Self::ENTRY_SIZE - 1);
    /// Number of entries per PTE
    pub const PTRS: usize = 1 << Arm64PgtableConfig::PTDESC_TABLE_SHIFT;
    // determines the continue PTE size map
    const CONT_SHIFT: usize = Self::pte_cont_shift(PageConfig::PAGE_SHIFT) + PageConfig::PAGE_SHIFT;
    /// Size of a contiguous PTE entry in bytes.
    pub const CONT_ENTRY_SIZE: usize = 1 << Self::CONT_SHIFT;
    // Number of entries per contiguous PTE
    const CONT_PTRS: usize = 1 << Self::CONT_SHIFT - PageConfig::PAGE_SHIFT;
    // Mask for aligning to a contiguous PTE entry
    const CONT_MASK: usize = !(Self::CONT_ENTRY_SIZE - 1);

    /// Create a new PteTable
    pub const fn from_raw(base: *mut PteEntry) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(base) },
            _marker: PhantomData,
            len: Self::PTRS,
        }
    }

    /// Get the index of a PteTable
    pub const fn addr_index(addr: VirtAddr) -> usize {
        (addr.as_usize() >> Self::SHIFT) & (Self::PTRS - 1)
    }

    /// len of the table
    pub const fn len(&self) -> usize {
        self.len
    }

    /// to phys
    pub fn to_phys(&self) -> PhysAddr {
        VirtAddr::from(self.base.as_ptr() as usize).to_phys()
    }

    /// addr end next
    #[inline(always)]
    pub fn addr_end_next(addr: VirtAddr, end: VirtAddr) -> VirtAddr {
        let boundary = (addr.as_usize().wrapping_add(Self::ENTRY_SIZE)) & Self::MASK;
        if boundary.wrapping_sub(1) < end.as_usize().wrapping_sub(1) {
            VirtAddr::from(boundary)
        } else {
            end
        }
    }

    /// cont addr end next
    #[inline(always)]
    pub fn cont_addr_end_next(addr: VirtAddr, end: VirtAddr) -> VirtAddr {
        let boundary = (addr.as_usize().wrapping_add(Self::CONT_ENTRY_SIZE)) & Self::CONT_MASK;
        if boundary.wrapping_sub(1) < end.as_usize().wrapping_sub(1) {
            VirtAddr::from(boundary)
        } else {
            end
        }
    }
}

impl Deref for PteTable {
    type Target = [PteEntry];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.base.as_ptr(), self.len) }
    }
}

impl DerefMut for PteTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.base.as_ptr(), self.len) }
    }
}

impl Index<usize> for PteTable {
    type Output = PteEntry;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index out of bounds");
        &self.deref()[index]
    }
}

impl IndexMut<usize> for PteTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "index out of bounds");
        &mut self.deref_mut()[index]
    }
}
