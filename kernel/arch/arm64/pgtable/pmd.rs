//! Arm64 Page table PMD

use core::{marker::PhantomData, ptr::NonNull};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use crate::arch::arm64::pgtable::Arm64PgtableConfig;
use crate::mm::{PhysAddr, VirtAddr, page::PageConfig};
use super::{PgTableEntry, PteEntry, PtePgProt};

/// Pmd
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PmdEntry(u64);

#[allow(dead_code)]
impl PmdEntry {
    /// Type table for pmd
    pub const PMD_TYPE_TABLE: u64 =  3 << 0;
    /// Section type
    pub const PMD_TYPE_SECT: u64 = 1 << 0;
    /// Section type mask
    pub const PMD_TYPE_MASK: u64 = 3 << 0;

    /// PMD AF
    pub const PMD_TABLE_AF: u64 = 1 << 10;

    /// PMD PXN
    pub const PMD_TABLE_PXN: u64 = 1 << 59;
    /// PMD UXN
    pub const PMD_TABLE_UXN: u64 = 1 << 60;

    /// Create a new Pmd
    pub const fn new(val: u64) -> Self {
        Self(val)
    }

    /// mk sec pmd prot
    pub fn mk_pmd_sect_prot(prot: PtePgProt) -> u64 {
        (prot.bits() & !PmdEntry::PMD_TYPE_MASK) | PmdEntry::PMD_TYPE_SECT
    }
}

impl PgTableEntry for PmdEntry {
    /// Get the value of the Pud
    #[inline(always)]
    fn read(&self) -> u64 {
        unsafe { core::ptr::read_volatile(&self.0) }
    }
    
    /// Write the value of the Pud
    #[inline(always)]
    fn write(&mut self, val: u64) {
        unsafe { core::ptr::write_volatile(&mut self.0, val) }
    }

    /// From physical address
    #[inline(always)]
    fn from_phys(pa: PhysAddr) -> Self {
        Self(PteEntry::from_phys(pa).value())
    }

    /// To physical address
    #[inline(always)]
    fn to_phys(&self) -> PhysAddr {
        PteEntry::new(self.value()).to_phys()
    }

    /// Value
    #[inline(always)]
    fn value(&self) -> u64 {
        self.0
    }
}

/// Pmd table
pub struct PmdTable {
    base: NonNull<PmdEntry>,
    _downgrage_from_pud: bool,
    _marker: PhantomData<PmdEntry>,
}

#[allow(dead_code)]
impl PmdTable {
    /// determines the size a pmd page table entry can map
    pub const SHIFT: usize = Arm64PgtableConfig::hw_pgtable_levels_shift(2);

    /// Size of a PMD entry in bytes.
    pub const ENTRY_SIZE: usize = 1 << Self::SHIFT;

    /// Number of entries per PMD
    pub const PTRS: usize = 1 <<  Arm64PgtableConfig::PTDESC_TABLE_SHIFT;

    /// Mask for PMD entry
    const MASK: usize = !(Self::ENTRY_SIZE - 1);

    /// determines the continue Pmd size map
    const CONT_SHIFT: usize = Self::pmd_cont_shift(PageConfig::PAGE_SHIFT) + Self::SHIFT;
    /// Size of a continue PMD entry in bytes.
    pub const CONT_ENTRY_SIZE: usize = 1 << Self::CONT_SHIFT;
    /// Number of entries per continue PMD
    const CONT_PTRS: usize = 1 << (Self::CONT_SHIFT - Self::SHIFT);
    /// Mask for continue PMD entry
    const CONT_MASK: usize = !(Self::CONT_ENTRY_SIZE - 1);

    // Returns the ARM64 continue pmd shift for a given page shift (log2 of page size).
    //
    // # Arguments
    // * `page_shift` - The log2 of the page size in bytes. (e.g., 12 for 4KB, 14 for 16KB, 16 for 64KB)
    //
    // # Returns
    // * The CONT_PTE_SHIFT value, i.e., log2 of the number of pages that can be merged into a contiguous PTE.
    //
    // # ARM64 logic:
    // - For 4KB pages (page_shift = 12):  CONT_PMD_SHIFT = 4  (16 pages, 16*2MB = 32MB block)
    // - For 16KB pages (page_shift = 14): CONT_PMD_SHIFT = 5  (32 pages, 32*32MB = 1GB block)
    // - For 64KB pages (page_shift = 16): CONT_PMD_SHIFT = 5  (32 pages, 32*512MB= 16GB block)
    // - Otherwise, defaults to 4 (for safety; can be adjusted for other configurations).
    const fn pmd_cont_shift(page_shift: usize) -> usize {
        match page_shift {
            12 => 4,  // 4KB pages
            14 => 5,  // 16KB pages
            16 => 5,  // 64KB pages
            _  => 4,  // Default/fallback value
        }
    }

    /// Create a new PmdTable
    pub const fn from_raw(base: *mut PmdEntry) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(base) },
            _marker: PhantomData,
            _downgrage_from_pud: false,
        }
    }

    pub(crate) fn from_pud(base: *mut PmdEntry) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(base) },
            _marker: PhantomData,
            _downgrage_from_pud: true,
        }
    }

    /// get len
    pub const fn len(&self) -> usize {
        Self::PTRS
    }

    /// Get the index of a PmdTable
    pub const fn addr_index(addr: VirtAddr) -> usize {
        (addr.as_usize() >> Self::SHIFT) & (Self::PTRS - 1)
    }

    /// table to phys
    pub fn to_phys(&self) -> PhysAddr {
        VirtAddr::from(self.base.as_ptr() as usize).to_phys()
    }

    #[inline(always)]
    /// addr end next
    pub fn addr_end_next(addr: VirtAddr, end: VirtAddr) -> VirtAddr {
        let boundary = (addr.as_usize().wrapping_add(Self::ENTRY_SIZE)) & Self::MASK;
        if boundary.wrapping_sub(1) < end.as_usize().wrapping_sub(1) {
            VirtAddr::from(boundary)
        } else {
            end
        }
    }

    #[inline(always)]
    /// cont addr end next
    pub fn cont_addr_end_next(addr: VirtAddr, end: VirtAddr) -> VirtAddr {
        let boundary = (addr.as_usize().wrapping_add(Self::CONT_ENTRY_SIZE)) & Self::CONT_MASK;
        if boundary.wrapping_sub(1) < end.as_usize().wrapping_sub(1) {
            VirtAddr::from(boundary)
        } else {
            end
        }
    }
}

impl Deref for PmdTable {
    type Target = [PmdEntry];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self.base.as_ptr(), Self::PTRS)
        }
    }
}

impl DerefMut for PmdTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self.base.as_ptr(), Self::PTRS)
        }
    }
}

impl Index<usize> for PmdTable {
    type Output = PmdEntry;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < Self::PTRS, "index out of bounds");
        &self.deref()[index]
    }
}

impl IndexMut<usize> for PmdTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < Self::PTRS, "index out of bounds");
        &mut self.deref_mut()[index]
    }
}
