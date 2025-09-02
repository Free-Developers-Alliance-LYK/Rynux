//! Arm64 Page table PGDIR

use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::{marker::PhantomData, ptr::NonNull};

use super::{Arm64PgtableConfig, PteEntry};
use crate::arch::arm64::{
    mm::Arm64VaLayout,
};
use crate::mm::{PhysAddr, VirtAddr};
use super::pud::{PudTable, PudEntry};
use super::PgTableEntry;

/// Pgdir
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PgdirEntry(u64);

impl PgdirEntry {
    /// Type table
    pub const PGD_TYPE_TABLE: u64 = 3 << 0;
    /// Access flag
    pub const PGD_TABLE_AF: u64 = 1 << 10;
    /// Privileged execute never
    pub const PGD_TABLE_PXN: u64 = 1 << 59;
    /// User execute never
    pub const PGD_TABLE_UXN: u64 = 1 << 60;
    /// Create a new Pgd
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl PgTableEntry for PgdirEntry {
    #[inline(always)]
    /// Create a new Pgd from physical address
    fn from_phys(pa: PhysAddr) -> Self {
        Self(PteEntry::from_phys(pa).value())
    }

    /// Get the physical address of the Pgd
    #[inline(always)]
    fn to_phys(&self) -> PhysAddr {
        PteEntry::new(self.read()).to_phys()
    }

    /// Get the value of the Pgd
    #[inline(always)]
    fn read(&self) -> u64 {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    /// Write the value of the Pgd
    #[inline(always)]
    fn write(&mut self, val: u64) {
        unsafe { core::ptr::write_volatile(&mut self.0, val) }
    }

    /// Value
    #[inline(always)]
    fn value(&self) -> u64 {
        self.0
    }
}

/// Pgtable
pub struct PgdirTable {
    base: NonNull<PgdirEntry>,
    len: usize,
    _marker: PhantomData<PgdirEntry>,
}

#[allow(dead_code)]
impl PgdirTable {
    /// determines the size a top-level page table entry can map
    const SHIFT: usize =
        Arm64PgtableConfig::hw_pgtable_levels_shift(4 - Arm64PgtableConfig::PGTABLE_LEVELS);

    /// Size of a PGDIR entry map in bytes.
    const ENTRY_SIZE: usize = 1 << Self::SHIFT;

    /// Mask for PGDIR entry
    const MASK: usize = !(Self::ENTRY_SIZE - 1);

    /// Number of entries per Pgd
    const PTRS: usize = 1 << (Arm64VaLayout::VA_BITS - Self::SHIFT);

    /// get kernel pgdir
    pub fn kernel_pgdir() -> PgdirTable {
        use crate::arch::arm64::kernel::image::symbols::swapper_pg_dir;
        PgdirTable::from_raw(swapper_pg_dir as *mut PgdirEntry)
    }
}

impl PgdirTable {
    /// From a raw pointer to a PgdirEntry
    #[inline(always)]
    pub const fn from_raw(base: *mut PgdirEntry) -> Self {
        Self {
            // SAFETY: caller make sure it is valid
            base: unsafe {NonNull::new_unchecked(base)},
            len: Self::PTRS,
            _marker: PhantomData,
        }
    }

    /// Get the length of a PgdirTable
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get the index of a PgdirTable
    #[inline(always)]
    pub const fn addr_index(addr: VirtAddr) -> usize {
        (addr.as_usize() >> Self::SHIFT) & (Self::PTRS - 1)
    }

    /// When PGTABLE_LEVELS < 4, pgd downgrage to a pud table
    #[inline(always)]
    pub fn downgrade_to_pud_table(&self) -> Option<PudTable> {
        if Arm64PgtableConfig::PGTABLE_LEVELS > 3 {
            None
        } else {
            // when PGTABLE_LEVELS is 2 3, pgd downgrage to a pud table
            Some(PudTable::from_pgdir(self.base.as_ptr() as *mut PudEntry))
        }
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
}

impl Deref for PgdirTable {
    type Target = [PgdirEntry];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self.base.as_ptr(), self.len)
        }
    }
}

impl DerefMut for PgdirTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self.base.as_ptr(), self.len)
        }
    }
}

impl Index<usize> for PgdirTable {
    type Output = PgdirEntry;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len);
        &self.deref()[index]
    }
}

impl IndexMut<usize> for PgdirTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.len);
        &mut self.deref_mut()[index]
    }
}
