//! Arm64 mmu

/*
#[link_section = ".mmuoff.data.write"]
/// Save Cpu boot status
pub static EARLY_CPU_BOOT_STATUS: usize = 0;
*/

#[allow(dead_code)]
/// Cpu boot status
pub enum CpuStatus {
    /// Default is success
    CpuBootSuccess = 0,
    /// Killed by other
    CpuKillMe = 1,
    /// Stuck in kernel
    CpuStuckInKernel = 2,
    /// Panic in kernel
    CpuPanicKernel = 3,
}

use crate::{
    arch::arm64::{
        mm::fixmap::FixMap,
        mm::Arm64VaLayout,
        pgtable::{
            PgTableEntry, PgdirTable, PmdEntry, PmdTable, PteEntry, PtePgProt, PteTable, PudEntry,
            PudTable,
        },
    },
    bitflags::bitflags,
    macros::section_init_text,
    mm::{page::PageConfig, PhysAddr, VirtAddr},
};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Mmu mapping flags
    pub struct MmuMapFlags: u8 {
        /// No block mapping
        const NO_BLOCK = 1 << 0;
        /// No contiguous mapping
        const NO_CONT = 1 << 1;
        /// No execute mapping
        const NO_EXEC = 1 << 2;
    }
}

/// Mmu manager
pub struct Mmu;

impl Mmu {
    fn pgattr_change_is_safe(old: u64, new: u64) -> bool {
        let old = PteEntry::new(old);
        let new = PteEntry::new(new);
        // the following mapping attributes may be updated in live kernel
        // mappings without the need for break-before-make
        let mask = PtePgProt::PTE_PXN.bits()
            | PtePgProt::PTE_RDONLY.bits()
            | PtePgProt::PTE_WRITE.bits()
            | PtePgProt::PTE_NG.bits()
            | PtePgProt::PTE_SWBITS_MASK;

        // creating or taking down mappings is always safe
        if !old.is_valid() || new.is_valid() {
            return true;
        }

        // A live entry's pfn should not change
        if old.pfn() != new.pfn() {
            return false;
        }

        // live contiguous mappings may not be manipulated at all
        if old.is_contiguous() || new.is_contiguous() {
            return false;
        }

        // TODO: check taged and NG
        let changed_bits = old.value() ^ new.value();
        if (changed_bits & mask) != 0 {
            return false;
        }

        true
    }

    fn init_pte(
        pte_tbl: &mut PteTable,
        virt: VirtAddr,
        end: VirtAddr,
        mut phys: PhysAddr,
        prot: PtePgProt,
    ) {
        let mut cur_virt = virt;
        while cur_virt < end {
            let pte_entry = &mut pte_tbl[PteTable::addr_index(cur_virt)];
            let old_pte = pte_entry.read();
            pte_entry.write(PteEntry::from_phys(phys.align_down_page()).value() | prot.bits());

            debug_assert!(!Self::pgattr_change_is_safe(old_pte, pte_entry.read()));
            cur_virt += PageConfig::PAGE_SIZE;
            phys += PageConfig::PAGE_SIZE;
        }
    }

    fn alloc_init_cont_pte(
        pte_tbl: &mut PteTable,
        virt: VirtAddr,
        end: VirtAddr,
        mut phys: PhysAddr,
        prot: PtePgProt,
        flags: MmuMapFlags,
    ) {
        let mut cur_virt = virt;
        while cur_virt < end {
            let next = PteTable::cont_addr_end_next(cur_virt, end);
            let prot = if cur_virt.is_aligned(PteTable::CONT_ENTRY_SIZE)
                && next.is_aligned(PteTable::CONT_ENTRY_SIZE)
                && phys.is_aligned(PteTable::CONT_ENTRY_SIZE)
                && !flags.contains(MmuMapFlags::NO_CONT)
            {
                prot | PtePgProt::PTE_CONT
            } else {
                prot
            };

            Self::init_pte(pte_tbl, cur_virt, next, phys, prot);
            cur_virt = next;
            phys += next - cur_virt;
        }
    }

    fn init_pmd(
        pmd_tbl: &mut PmdTable,
        virt: VirtAddr,
        end: VirtAddr,
        mut phys: PhysAddr,
        prot: PtePgProt,
        no_alloc: bool,
        flags: MmuMapFlags,
    ) {
        let mut cur_virt = virt;
        while cur_virt < end {
            let next = PmdTable::addr_end_next(cur_virt, end);
            let pmd_entry = &mut pmd_tbl[PmdTable::addr_index(cur_virt)];
            // try sect map
            if cur_virt.is_aligned(PmdTable::ENTRY_SIZE)
                && next.is_aligned(PmdTable::ENTRY_SIZE)
                && phys.is_aligned(PmdTable::ENTRY_SIZE)
                && !flags.contains(MmuMapFlags::NO_BLOCK)
            {
                let old_pmd = pmd_entry.read();
                pmd_entry.write(
                    PmdEntry::from_phys(phys.align_down_page()).value()
                        | PmdEntry::mk_pmd_sect_prot(prot),
                );
                debug_assert!(Self::pgattr_change_is_safe(old_pmd, pmd_entry.read()));
            } else {
                if pmd_entry.is_none() {
                    if no_alloc {
                        panic!("create_pmd_mapping: no_alloc is true but pmd_entry is none");
                    }
                    // todo finish alloc pte table and init
                    todo!();
                }

                let pte_tbl_phys = pmd_entry.to_phys();
                let pte_tbl_virt = FixMap::set_pte_map(pte_tbl_phys);
                let mut pte_tbl = PteTable::from_raw(pte_tbl_virt.as_usize() as *mut PteEntry);
                Self::alloc_init_cont_pte(&mut pte_tbl, cur_virt, next, phys, prot, flags);
            }
            cur_virt = next;
            phys += next - cur_virt;
        }
    }

    fn alloc_init_cont_pmd(
        pmd_tbl: &mut PmdTable,
        virt: VirtAddr,
        end: VirtAddr,
        mut phys: PhysAddr,
        prot: PtePgProt,
        no_alloc: bool,
        flags: MmuMapFlags,
    ) {
        let mut cur_virt = virt;
        while cur_virt < end {
            let next = PmdTable::cont_addr_end_next(cur_virt, end);

            let prot = if cur_virt.is_aligned(PmdTable::CONT_ENTRY_SIZE)
                && next.is_aligned(PmdTable::CONT_ENTRY_SIZE)
                && phys.is_aligned(PmdTable::CONT_ENTRY_SIZE)
                && !flags.contains(MmuMapFlags::NO_CONT)
            {
                prot | PtePgProt::PTE_CONT
            } else {
                prot
            };

            Self::init_pmd(pmd_tbl, cur_virt, next, phys, prot, no_alloc, flags);

            cur_virt = next;
            phys += next - cur_virt;
        }
    }

    fn alloc_init_pud(
        pud_tbl: &mut PudTable,
        virt: VirtAddr,
        end: VirtAddr,
        mut phys: PhysAddr,
        prot: PtePgProt,
        no_alloc: bool,
        flags: MmuMapFlags,
    ) {
        match &mut pud_tbl.downgrade_to_pmd_table() {
            Some(pmd_tbl) => {
                Self::alloc_init_cont_pmd(pmd_tbl, virt, end, phys, prot, no_alloc, flags);
            }
            None => {
                let mut cur_virt = virt;
                while cur_virt < end {
                    let next = PudTable::addr_end_next(cur_virt, end);

                    // For 4K granule only, attempt to put down a 1GB block
                    if PudTable::support_sect()
                        && cur_virt.is_aligned(PudTable::ENTRY_SIZE)
                        && next.is_aligned(PudTable::ENTRY_SIZE)
                        && phys.is_aligned(PudTable::ENTRY_SIZE)
                        && !flags.contains(MmuMapFlags::NO_BLOCK)
                    {
                        // no need to alloc pmd table
                        todo!();
                    } else {
                        let pud_entry = &mut pud_tbl[PudTable::addr_index(cur_virt)];
                        if pud_entry.is_none() {
                            if no_alloc {
                                panic!(
                                    "create_pud_mapping: no_alloc is true but pud_entry is none"
                                );
                            }
                            // todo finish alloc pmd table and init
                            todo!();
                        }

                        // we need to access pmd table in pud_entry
                        // first we need to map pmdtable to fixmap
                        let pmd_tbl_raw_phys = pud_entry.to_phys();
                        let pmd_tbl_raw_virt = FixMap::set_pmd_map(pmd_tbl_raw_phys);
                        let mut pmd_tbl =
                            PmdTable::from_raw(pmd_tbl_raw_virt.as_usize() as *mut PmdEntry);
                        Self::alloc_init_cont_pmd(
                            &mut pmd_tbl,
                            cur_virt,
                            next,
                            phys,
                            prot,
                            no_alloc,
                            flags,
                        );
                        FixMap::clear_pmd_map();
                    }
                    cur_virt = next;
                    phys += next - cur_virt;
                }
            }
        }
    }

    fn create_pgd_mapping(
        pgd_tbl: &mut PgdirTable,
        phys: PhysAddr,
        virt: VirtAddr,
        size: usize,
        prot: PtePgProt,
        no_alloc: bool,
        flags: MmuMapFlags,
    ) {
        // TODO: USE MUTEX LOCK TO PROTECT FIXMAP VIRT REGION
        // make sure virt and phys has the same offset
        assert_eq!(phys.align_offset_page(), virt.align_offset_page());

        let phys_base = phys.align_down_page();
        let virt_base = virt.align_down_page();
        let end = (virt + size).align_up_page();

        match &mut pgd_tbl.downgrade_to_pud_table() {
            Some(pud_tbl) => {
                Self::alloc_init_pud(pud_tbl, virt_base, end, phys_base, prot, no_alloc, flags);
            }
            None => {
                let mut cur_virt = virt_base;
                let mut cur_phys = phys_base;
                while cur_virt < end {
                    let next = PgdirTable::addr_end_next(cur_virt, end);
                    let pgd_entry = &mut pgd_tbl[PgdirTable::addr_index(cur_virt)];
                    if pgd_entry.is_none() {
                        if no_alloc {
                            panic!("create_pgd_mapping: no_alloc is true but pgd_entry is none");
                        }
                        // todo  fini alloc pud table and init
                        todo!();
                    }
                    // we need to access pud table in pgd_entry
                    // need to map pudtable to fixmap
                    let pud_tbl_raw_phys = pgd_entry.to_phys();
                    let pud_tbl_raw_virt = FixMap::set_pud_map(pud_tbl_raw_phys);
                    let mut pud_tbl =
                        PudTable::from_raw(pud_tbl_raw_virt.as_usize() as *mut PudEntry);

                    Self::alloc_init_pud(
                        &mut pud_tbl,
                        cur_virt,
                        next,
                        cur_phys,
                        prot,
                        no_alloc,
                        flags,
                    );
                    FixMap::clear_pud_map();
                    cur_virt = next;
                    cur_phys += next - cur_virt;
                }
            }
        }
    }

    /// Create a mapping without allocating new page table  caller must make
    /// sure the page table is already init
    #[unsafe(section_init_text)]
    pub fn create_map_noalloc(phys: PhysAddr, virt: VirtAddr, size: usize, prot: PtePgProt) {
        // virt must be in kernel space
        if virt.as_usize() < Arm64VaLayout::KERNNEL_VA_START {
            panic!(
                "create_map_noalloc: virt 0x{:x} is not in kernel space",
                virt.as_usize()
            );
        }
        let mut k_pgdir_tbl = PgdirTable::kernel_pgdir();
        Self::create_pgd_mapping(
            &mut k_pgdir_tbl,
            phys,
            virt,
            size,
            prot,
            true,
            MmuMapFlags::NO_CONT,
        );
    }
}

pub(crate) fn paging_init() {
    // nothing to do now
}
