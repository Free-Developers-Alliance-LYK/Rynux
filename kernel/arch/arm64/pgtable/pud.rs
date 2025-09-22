//! Arm64 Page table PUD

use crate::arch::arm64::pgtable::config::Arm64PgtableConfig;
use crate::mm::page::PageConfig;
use crate::mm::{PhysAddr, VirtAddr};
use crate::size::SZ_4K;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::{marker::PhantomData, ptr::NonNull};

use super::{
    pmd::{PmdEntry, PmdTable},
    PgTableEntry, PteEntry,
};

/// Pud
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PudEntry(u64);

#[allow(dead_code)]
impl PudEntry {
    /// Type table for pud
    pub const PUD_TYPE_TABLE: u64 = 3 << 0;
    /// Section type
    pub const PUD_TYPE_SECT: u64 = 1 << 0;
    /// AP 3 bits
    pub const PUD_SECT_RDONLY: u64 = 1 << 7;
    /// Pud AF
    pub const PUD_TABLE_AF: u64 = 1 << 10;
    /// Pud PXN
    pub const PUD_TABLE_PXN: u64 = 1 << 59;
    /// Pud UXN
    pub const PUD_TABLE_UXN: u64 = 1 << 60;

    /// Create a new Pud
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl PgTableEntry for PudEntry {
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
        PteEntry::new(self.read()).to_phys()
    }

    /// Value
    #[inline(always)]
    fn value(&self) -> u64 {
        self.0
    }
}

/// Pud table
pub struct PudTable {
    base: NonNull<PudEntry>,
    _marker: PhantomData<PudEntry>,
    _downgrage_from_pgdir: bool,
}

#[allow(dead_code)]
impl PudTable {
    /// determines the size a pud page table entry can map
    pub const SHIFT: usize = Arm64PgtableConfig::hw_pgtable_levels_shift(1);
    /// Size of a PUD entry in bytes.
    pub const ENTRY_SIZE: usize = 1 << Self::SHIFT;
    /// Number of entries per PUD
    pub const PTRS: usize = 1 << Arm64PgtableConfig::PTDESC_TABLE_SHIFT;
    /// Mask for PUD entry
    const MASK: usize = !(Self::ENTRY_SIZE - 1);

    /// Create a new PudTable
    pub const fn from_raw(base: *mut PudEntry) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(base) },
            _marker: PhantomData,
            _downgrage_from_pgdir: false,
        }
    }

    pub(crate) fn from_pgdir(base: *mut PudEntry) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(base) },
            _marker: PhantomData,
            _downgrage_from_pgdir: true,
        }
    }

    /// len of the table
    #[inline]
    pub const fn len(&self) -> usize {
        Self::PTRS
    }

    /// Get the index of a PudTable
    #[inline]
    pub const fn addr_index(addr: VirtAddr) -> usize {
        (addr.as_usize() >> Self::SHIFT) & (Self::PTRS - 1)
    }

    /// To phys
    #[inline]
    pub fn to_phys(&self) -> PhysAddr {
        VirtAddr::from(self.base.as_ptr() as usize).to_phys()
    }

    /// When PGTABLE_LEVELS < 3, pud need downgrage to a pmd table
    /// In fact, it also implies that the pud table must also be downgraded
    /// from pgdir
    #[inline(always)]
    pub fn downgrade_to_pmd_table(&self) -> Option<PmdTable> {
        if Arm64PgtableConfig::PGTABLE_LEVELS == 2 {
            assert!(
                self._downgrage_from_pgdir,
                "pud must be downgraded from pgdir"
            );
            // whne PGTABLE_LEVELS IS 2, pud is not used, so pud down level to pmd
            Some(PmdTable::from_pud(self.base.as_ptr() as *mut PmdEntry))
        } else {
            None
        }
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

    /// support sect only when PAGE_SIZE == 4K
    pub fn support_sect() -> bool {
        PageConfig::PAGE_SIZE == SZ_4K
    }
}

impl Deref for PudTable {
    type Target = [PudEntry];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.base.as_ptr(), Self::PTRS) }
    }
}

impl DerefMut for PudTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.base.as_ptr(), Self::PTRS) }
    }
}

impl Index<usize> for PudTable {
    type Output = PudEntry;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < Self::PTRS, "index out of bounds");
        &self.deref()[index]
    }
}

impl IndexMut<usize> for PudTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < Self::PTRS, "index out of bounds");
        &mut self.deref_mut()[index]
    }
}
