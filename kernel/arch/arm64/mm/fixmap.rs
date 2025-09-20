//! Arm64 fixmap mem
//!
//! TODO: access fixmap virt region need lock protect, it is share by all cpu.
use crate::{
    arch::arm64::{
        asm::tlb::TlbFlushOps,
        mm::Arm64VaLayout,
        pgtable::{
            idmap::InitIdmap,
            pmd::{PmdEntry, PmdTable},
            pud::{PudEntry, PudTable},
            Arm64PgtableConfig, PgTableEntry, PgdirEntry, PgdirTable, PteEntry, PtePgProt,
            PteTable,
        },
    },
    klib::math::div_round_up,
    macros::{page_aligned, section_bss_page_aligned, section_init_text},
    mm::{page::PageConfig, PhysAddr, VirtAddr},
    static_assertions::const_assert_eq,
};

/// Here we define all the compile-time 'special' virtual addresses.
/// The poinnt is to have a constant address at compile time.
/// but to set the physical address only at runtime.
///
/// Each ennum increment in these  'compile-time allocated'
/// memory regions is a page, so we can use the page size
/// to calculate the physical address at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixMapType {
    /// Preserved for the kernel's linear map.
    Hole = 0,
    /// FDT_END
    FdtEnd = 1,
    /// Fdt
    Fdt = Self::FdtEnd as isize
        + div_round_up(InitIdmap::MAX_FDT_SIZE, PageConfig::PAGE_SIZE) as isize
        + 1,
    /// Early con mem base.
    EarlyConMemBase,

    /// End permanent mapping
    EndPermanentFixMap,

    /// PTE map used for kernel page table creation
    PteMap,
    /// PMD map used for kernel page table creation
    PmdMap,
    /// PUD map used for kernel page table creation
    PudMap,
    /// P4D map used for kernel page table creation
    P4dMap,
    /// PGD map used for kernel page table creation
    PgdMap,
    /// Fixmap end
    EndFixMap, // end of fixmap
}

impl From<usize> for FixMapType {
    fn from(idx: usize) -> Self {
        match idx {
            0 => FixMapType::Hole,
            1 => FixMapType::FdtEnd,
            x if x == FixMapType::Fdt as usize => FixMapType::Fdt,
            x if x == FixMapType::EarlyConMemBase as usize => FixMapType::EarlyConMemBase,
            x if x == FixMapType::EndPermanentFixMap as usize => FixMapType::EndPermanentFixMap,
            x if x == FixMapType::PteMap as usize => FixMapType::PteMap,
            x if x == FixMapType::PmdMap as usize => FixMapType::PmdMap,
            x if x == FixMapType::PudMap as usize => FixMapType::PudMap,
            x if x == FixMapType::P4dMap as usize => FixMapType::P4dMap,
            x if x == FixMapType::PgdMap as usize => FixMapType::PgdMap,
            x if x == FixMapType::EndFixMap as usize => FixMapType::EndFixMap,
            _ => panic!("Invalid index for FixMapType: {}", idx),
        }
    }
}

impl FixMapType {
    /// From type to virtual address.
    pub const fn to_virt(&self) -> VirtAddr {
        VirtAddr::from(Arm64VaLayout::FIXMAP_TOP - ((*self as usize) << PageConfig::PAGE_SHIFT))
    }
}

impl From<VirtAddr> for FixMapType {
    fn from(addr: VirtAddr) -> Self {
        let idx = (Arm64VaLayout::FIXMAP_TOP - (addr.as_usize() & PageConfig::PAGE_MASK))
            >> PageConfig::PAGE_SHIFT;
        FixMapType::from(idx)
    }
}

#[page_aligned]
struct FixMapPmdTable {
    _pmds: [PmdEntry; PmdTable::PTRS],
}

#[section_bss_page_aligned]
static mut FIXMAP_PMD_TABLE: FixMapPmdTable = FixMapPmdTable {
    _pmds: [PmdEntry::new(0); PmdTable::PTRS],
};

impl FixMapPmdTable {
    #[inline(always)]
    // SAFETY: Caller should use lock protect
    unsafe fn mut_ptr() -> *mut PmdEntry {
        unsafe {
            let p = core::ptr::addr_of_mut!(FIXMAP_PMD_TABLE);
            core::ptr::addr_of_mut!((*p)._pmds[0])
        }
    }
}

#[cfg(CONFIG_PAGE_SIZE_4KB)]
const_assert_eq!(FixMapType::Fdt as isize, 514);

#[page_aligned]
struct FixMapPudTable {
    _puds: [PudEntry; PudTable::PTRS],
}

impl FixMapPudTable {
    #[inline(always)]
    unsafe fn mut_ptr() -> *mut PudEntry {
        unsafe {
            let p = core::ptr::addr_of_mut!(FIXMAP_PUD_TABLE);
            core::ptr::addr_of_mut!((*p)._puds[0])
        }
    }
}

#[section_bss_page_aligned]
static mut FIXMAP_PUD_TABLE: FixMapPudTable = FixMapPudTable {
    _puds: [PudEntry::new(0); PudTable::PTRS],
};

#[page_aligned]
#[derive(Copy, Clone)]
struct FixMapPteTable {
    _ptes: [PteEntry; PteTable::PTRS],
}

const PTE_TABLES_CNT: usize = Arm64PgtableConfig::spann_nr_entries(
    Arm64VaLayout::FIXMAP_START,
    Arm64VaLayout::FIXMAP_TOP,
    PmdTable::SHIFT,
);

#[section_bss_page_aligned]
static mut FIXMAP_PTE_TABLES: [FixMapPteTable; PTE_TABLES_CNT] = [FixMapPteTable {
    _ptes: [PteEntry::new(0); PteTable::PTRS],
}; PTE_TABLES_CNT];

impl FixMapPteTable {
    fn pte_tbl_index(addr: VirtAddr) -> usize {
        (addr.as_usize() >> PmdTable::SHIFT) - (Arm64VaLayout::FIXMAP_START >> PmdTable::SHIFT)
    }

    #[inline(always)]
    unsafe fn mut_ptr(addr: VirtAddr) -> *mut PteEntry {
        unsafe {
            let idx = Self::pte_tbl_index(addr);
            debug_assert!(idx < PTE_TABLES_CNT);
            core::ptr::addr_of_mut!((FIXMAP_PTE_TABLES[idx])._ptes[0]) as *mut PteEntry
        }
    }
}

/// FixMap configuration
pub struct FixMap;

impl FixMap {
    /// Fixmap virtual address space size
    pub const FIXMAP_PERMANENT_SIZE: usize =
        (FixMapType::EndPermanentFixMap as usize) << PageConfig::PAGE_SHIFT;
    /// Fixmap virtual address space size
    pub const FIXMAP_SIZE: usize = (FixMapType::EndFixMap as usize) << PageConfig::PAGE_SHIFT;

    #[section_init_text]
    fn early_fixmap_init_pmd(pmd_tbl: &mut PmdTable, addr: VirtAddr, end: VirtAddr) {
        let mut cur = addr;
        while cur < end {
            let next = PmdTable::addr_end_next(cur, end);
            let pmd_entry = &mut pmd_tbl[PmdTable::addr_index(cur)];
            if pmd_entry.is_none() {
                let pte_tbl = PteTable::from_raw(unsafe { FixMapPteTable::mut_ptr(cur) });
                pmd_entry.write(
                    pte_tbl.to_phys().as_usize() as u64
                        | PmdEntry::PMD_TYPE_TABLE
                        | PmdEntry::PMD_TABLE_AF,
                );
            }
            cur = next;
        }
    }

    #[section_init_text]
    fn early_fixmap_init_pud(pud_tbl: &mut PudTable, addr: VirtAddr, end: VirtAddr) {
        match &mut pud_tbl.downgrade_to_pmd_table() {
            Some(pmd_tbl) => {
                Self::early_fixmap_init_pmd(pmd_tbl, addr, end);
            }
            None => {
                let pud_entry = &mut pud_tbl[PudTable::addr_index(addr)];
                let mut pmd_tbl = PmdTable::from_raw(unsafe { FixMapPmdTable::mut_ptr() });
                if pud_entry.is_none() {
                    pud_entry.write(
                        pmd_tbl.to_phys().as_usize() as u64
                            | PudEntry::PUD_TYPE_TABLE
                            | PudEntry::PUD_TABLE_AF,
                    );
                }
                Self::early_fixmap_init_pmd(&mut pmd_tbl, addr, end);
            }
        }
    }

    #[section_init_text]
    fn early_fixmap_init_pgd(pgdir_tbl: &mut PgdirTable, addr: VirtAddr, end: VirtAddr) {
        match &mut pgdir_tbl.downgrade_to_pud_table() {
            Some(pud_tbl) => {
                Self::early_fixmap_init_pud(pud_tbl, addr, end);
            }
            None => {
                let pgdir_entry = &mut pgdir_tbl[PgdirTable::addr_index(addr)];
                let mut pud_tbl = PudTable::from_raw(unsafe { FixMapPudTable::mut_ptr() });
                if pgdir_entry.is_none() {
                    pgdir_entry.write(
                        pud_tbl.to_phys().as_usize() as u64
                            | PgdirEntry::PGD_TYPE_TABLE
                            | PgdirEntry::PGD_TABLE_AF,
                    );
                }
                Self::early_fixmap_init_pud(&mut pud_tbl, addr, end);
            }
        }
    }

    /// he p*d_populate functions call virt_to_phys implicitly so they can't be used
    /// directly on kernel symbols (bm_p*d). This function is called too early to use
    /// lm_alias so __p*d_populate functions must be used to populate with the
    /// physical address from __pa_symbol.
    #[section_init_text]
    pub fn early_fixmap_init() {
        let mut k_pgdir_tbl = PgdirTable::kernel_pgdir();
        let addr = VirtAddr::from(Arm64VaLayout::FIXMAP_START);
        let end = VirtAddr::from(Arm64VaLayout::FIXMAP_TOP);
        Self::early_fixmap_init_pgd(&mut k_pgdir_tbl, addr, end);
    }

    #[inline(always)]
    fn set_fixmap(idx: FixMapType, phys: PhysAddr, prot: PtePgProt, clear: bool) -> VirtAddr {
        let virt_addr = idx.to_virt();
        let phys_base = phys.align_down_page();
        let mut pte_tbl = PteTable::from_raw(unsafe { FixMapPteTable::mut_ptr(virt_addr) });
        let pte_entry = &mut pte_tbl[PteTable::addr_index(virt_addr)];
        if clear {
            pte_entry.write(0);
            TlbFlushOps::flush_tlb_kernel_range(virt_addr, virt_addr + PageConfig::PAGE_SIZE);
        } else {
            pte_entry.write(PteEntry::from_phys(phys_base).value() | prot.bits());
        }
        // return virt addr
        virt_addr + phys.align_offset_page()
    }

    #[inline]
    /// Set fixmap pud map
    pub fn set_pud_map(phys: PhysAddr) -> VirtAddr {
        Self::set_fixmap(FixMapType::PudMap, phys, PtePgProt::PAGE_KERNEL, false)
    }

    #[inline]
    /// clear fixmap pud map
    pub fn clear_pud_map() {
        Self::set_fixmap(
            FixMapType::PudMap,
            PhysAddr::from(0),
            PtePgProt::empty(),
            true,
        );
    }

    #[inline]
    /// Set fixmap pmd map
    pub fn set_pmd_map(phys: PhysAddr) -> VirtAddr {
        Self::set_fixmap(FixMapType::PmdMap, phys, PtePgProt::PAGE_KERNEL, false)
    }

    #[inline]
    /// clear fixmap pmd map
    pub fn clear_pmd_map() {
        Self::set_fixmap(
            FixMapType::PmdMap,
            PhysAddr::from(0),
            PtePgProt::empty(),
            true,
        );
    }

    /// Set fixmap pte map
    #[inline]
    pub fn set_pte_map(phys: PhysAddr) -> VirtAddr {
        Self::set_fixmap(FixMapType::PteMap, phys, PtePgProt::PAGE_KERNEL, false)
    }

    /// clear fixmap pte map
    #[inline]
    pub fn clear_pte_map() {
        Self::set_fixmap(
            FixMapType::PteMap,
            PhysAddr::from(0),
            PtePgProt::empty(),
            true,
        );
    }

    /// remap fdt
    #[section_init_text]
    pub(crate) fn remap_fdt(dt_phys: PhysAddr, prot: PtePgProt) -> (VirtAddr, usize) {
        use crate::arch::arm64::mm::mmu::Mmu;
        // dt_phys must align to 8
        assert_eq!(dt_phys.as_usize() & 0x7, 0);
        let dt_virt_base = FixMapType::Fdt.to_virt();
        let dt_phys_base = dt_phys.align_down_page();
        let offset = dt_phys.align_offset_page();
        let dt_virt = dt_virt_base + offset;

        // Map the first page, so we can read the size from the header
        Mmu::create_map_noalloc(dt_phys_base, dt_virt_base, PageConfig::PAGE_SIZE, prot);
        let fdt = unsafe { fdtree_rs::LinuxFdt::from_ptr(dt_virt.as_usize() as *const u8) };

        match fdt {
            Ok(fdt) => {
                let size = fdt.total_size();
                if size > InitIdmap::MAX_FDT_SIZE {
                    panic!("Fdt size too large: {}", size);
                }
                if offset + size > PageConfig::PAGE_SIZE {
                    Mmu::create_map_noalloc(dt_phys_base, dt_virt_base, offset + size, prot);
                }
                (dt_virt, size)
            }
            Err(e) => {
                panic!("Invalid fdt: {}", e);
            }
        }
    }
}
