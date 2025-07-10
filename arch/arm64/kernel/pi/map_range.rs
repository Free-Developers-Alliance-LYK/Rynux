//! Map range

use core::cmp::min;
use kernel::{
    arch::arm64::{
        pgtable::{
            hard::PTDESC_TABLE_SHIFT,
            PtePgProt,
            Pte,
            Pgdir,
            idmap::Idmap,
        },
        kernel::image::symbols::*,
    },

    mm::page::{
        PAGE_MASK,
        PAGE_SIZE,
        PAGE_SHIFT,
        page_align,
    },

    macros::section_init_text,
};

/// map-range - Map a contiguous range of physical pages into virtual memory
///
/// # Arguments
///
/// * `pte` - Address of physical pointer to array of pages to allocate page tables from
/// * `pgd` - Address of physical pointer to array of pages to allocate page tables from
/// * `start` - Virtual address of the start of the range
/// * `end` - Virtual address of the end of the range (exclusive)
/// * `pa` - Physical address of the start of the range
/// * `prot` - Access permissions of the range
/// * `level` - Translation level for the mapping
/// * `tbl` - The level `level` page table to create the mappings in
/// * `may_use_cont` - Whether the use of the contiguous attribute is allowed
/// * `va_offset` - Offset between a physical page and its current mapping in the VA space
pub unsafe fn map_range(
    pte: &mut usize,
    start: usize,
    end: usize,
    pa: usize,
    prot: PtePgProt,
    level: usize,
    tbl: *mut Pte,
    may_use_cont: bool,
    va_offset: usize,
) {
    // continue map mask
    //let mut cmask = usize::MAX;
    let mut cmask = 0;
    if level == 3 {
        cmask = Pte::CONT_SIZE - 1;
    }

    // remove type
    let mut protval = PtePgProt::from_bits_truncate(prot.bits() & !PtePgProt::PTE_TYPE_MASK);

    let lshift: usize = (3 - level) * PTDESC_TABLE_SHIFT;
    let lmask: usize = (PAGE_SIZE << lshift) - 1;

    let mut start = start & PAGE_MASK;
    let mut  pa = pa &PAGE_MASK;

    // Advance tbl to the entry that covers start
    let mut tbl: *mut Pte = unsafe {tbl.add((start >> (lshift + PAGE_SHIFT)) % Pte::PTRS)};

    // Set the right block/page bits for this level unless we are clearing the mapping
    if !protval.is_empty() {
        if level == 2 {
            protval |= PtePgProt::PMD_TYPE_SECT;
        } else {
            protval |= PtePgProt::PTE_TYPE_PAGE;
        }
    }

     while start < end {
        let next = min((start | lmask).wrapping_add(1), page_align(end));

        if level < 2 || (level == 2 && ((start | next | pa) & lmask) != 0) {
            // finer grained mapping
            unsafe {
                if (*tbl).is_none() {
                    // set tbl entry
                    let tbl_entry = Pte::new(
                        Pte::from_phys(*pte).bits()
                        | PtePgProt::PMD_TYPE_TABLE.bits()
                        | PtePgProt::PMD_TABLE_UXN.bits()
                    );
                    *tbl = tbl_entry;
                    // move pte to next page
                    *pte = ((*pte) as *mut Pte).add(Pte::PTRS) as usize;
                }
                // map next level
                map_range(
                    pte,
                    start,
                    next,
                    pa,
                    prot,
                    level + 1,
                    ((*tbl).to_phys() + va_offset) as *mut Pte,
                    may_use_cont,
                    va_offset,
                );
            }
        } else {
            // start a contiguous range if start and pa are suitably aligned
            if ((start | pa) & cmask) == 0 && may_use_cont {
                protval |= PtePgProt::PTE_CONT;
            }

            // clear the contiguous attribute if the remaining range does not cover a contiguous block
            if (end & !cmask) <= start {
                protval &= !PtePgProt::PTE_CONT;
            }

            // Put down a block or page mapping
            let tbl_content: Pte = Pte::new(
                Pte::from_phys(pa).bits()
                | protval.bits()
            );

            // set tbl entry
            unsafe {
                *tbl = tbl_content;
            }
        }
        pa = pa.wrapping_add(next - start);
        start = next;

        // move tbl to next entry
        unsafe {
           tbl = tbl.add(1);
        }
    }
}


/// Create initial ID map
#[no_mangle]
#[section_init_text]
pub unsafe extern "C" fn create_init_idmap(pg_dir: *mut Pgdir, clrmask: u64) -> usize {
    let mut pte = (pg_dir as usize) + PAGE_SIZE;

    let mut text_prot = PtePgProt::PAGE_KERNEL_ROX;
    let mut data_prot = PtePgProt::PAGE_KERNEL;
    let clrmask = PtePgProt::from_bits_truncate(clrmask);
    text_prot &= !clrmask;
    data_prot &= !clrmask;

    unsafe {
        map_range(
            &mut pte,
            _stext,
            __initdata_begin,
            _stext,
            text_prot,
            Idmap::INIT_PGTABLE_LEVELS,
            pg_dir as *mut Pte,
            false,
            0,
        );
        map_range(
            &mut pte,
            __initdata_begin,
            _end,
            __initdata_begin,
            data_prot,
            Idmap::INIT_PGTABLE_LEVELS,
            pg_dir as *mut Pte,
            false,
            0,
        );
    }

    pte
}
