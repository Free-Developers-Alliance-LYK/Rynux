//! Map range
//!
//! TODO: 
//! early_map_kernel:
//!    - Parse the command line for CPU feature overrides
//!    - CONFIG_RANDOMIZE_BASE support
//!    - CONFIG_ARM64_LPA2 support 
//! map_kernel:
//!  - External debuggers may need to write directly to the text mapping support
//!  - CONFIG_UNWIND_PATCH_PAC_INTO_SCS support
//!  - CONFIG_RELOCATABLE support
//!  - CONFIG_ARM64_LPA2 support

use core::{
    cmp::min,
};

use kernel::{
    arch::arm64::{
        pgtable::{
            Arm64PgtableConfig,
            PtePgProt,
            PteEntry,
            PteTable,
            PgdirEntry,
            PmdEntry,
            idmap::InitIdmap,
            PgTableEntry,
        },
        kernel::image::symbols::*,
        early_debug::{early_uart_putchar, early_uart_put_u64_hex},
        asm::{
            barrier::{isb, dsb, ISHST},
            tlb::TlbFlushOps,
        },
        sysregs::Ttbr1El1,
        va_layout::Arm64VaLayout,
    },

    mm::page::PageConfig,
    page_align,
    macros::{section_init_text, page_aligned, section_init_data},
    klib::string::{memset, memcpy},
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
#[no_mangle]
#[section_init_text]
pub unsafe extern "C" fn map_range(
    pte: &mut usize,
    start: usize,
    end: usize,
    pa: usize,
    prot: PtePgProt,
    level: usize,
    tbl: *mut PteEntry,
    may_use_cont: bool,
    va_offset: usize,
) {
    // continue map mask
    let mut cmask = usize::MAX;
    if level == 3 {
        cmask = PteTable::CONT_ENTRY_SIZE - 1;
    }

    // remove type
    let mut protval = PtePgProt::from_bits_truncate(prot.bits() & !PtePgProt::PTE_TYPE_MASK);

    let lshift: usize = (3-level) * Arm64PgtableConfig::PTDESC_TABLE_SHIFT;
    let lmask: usize = (PageConfig::PAGE_SIZE << lshift) - 1;

    let mut start = start & PageConfig::PAGE_MASK;
    let mut pa = pa &PageConfig::PAGE_MASK;

    // Advance tbl to the entry that covers start
    let mut tbl: *mut PteEntry = unsafe {tbl.add((start >> (lshift + PageConfig::PAGE_SHIFT)) % PteTable::PTRS)};

    // Set the right block/page bits for this level unless we are clearing the mapping
    if !protval.is_empty() {
        if level == 2 {
            protval |= PtePgProt::from_bits_truncate(PmdEntry::PMD_TYPE_SECT);
        } else {
            protval |= PtePgProt::PTE_TYPE_PAGE;
        }
    }

     while start < end {
        let next = min((start | lmask) + 1, page_align!(end));

        if level < 2 || (level == 2 && ((start | next | pa) & lmask) != 0) {
            // finer grained mapping
            unsafe {
                if (*tbl).is_none() {
                    // set tbl entry
                    let tbl_entry = PteEntry::new(
                        PteEntry::from_phys((*pte).into()).value()
                        | PmdEntry::PMD_TYPE_TABLE
                        | PmdEntry::PMD_TABLE_UXN
                    );
                    *tbl = tbl_entry;
                    // move pte to next page
                    *pte = ((*pte) as *mut PteEntry).add(PteTable::PTRS) as usize;
                }
                // map next level
                map_range(
                    pte,
                    start,
                    next,
                    pa,
                    prot,
                    level + 1,
                    ((*tbl).to_phys().as_usize() + va_offset) as *mut PteEntry,
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
            let tbl_content: PteEntry = PteEntry::new(
                PteEntry::from_phys(pa.into()).value()
                | protval.bits()
            );

            // set tbl entry
            unsafe {
                *tbl = tbl_content;
            }
        }

        pa += next - start;
        start = next;

        // move tbl to next entry
        unsafe {
           tbl = tbl.add(1);
        }
    }
}

#[page_aligned]
struct DevicePtes([u8; 8 * PageConfig::PAGE_SIZE]);

#[section_init_data]
static mut DEVICE_PTES: DevicePtes = DevicePtes([0; 8 * PageConfig::PAGE_SIZE]);

/// Create initial ID map
#[no_mangle]
#[section_init_text]
pub unsafe extern "C" fn create_init_idmap(pg_dir: *mut PgdirEntry, clrmask: u64) -> usize {
    let mut pte = (pg_dir as usize) + PageConfig::PAGE_SIZE;

    let mut text_prot = PtePgProt::PAGE_KERNEL_ROX;
    let mut data_prot = PtePgProt::PAGE_KERNEL;
    let clrmask = PtePgProt::from_bits_truncate(clrmask);
    text_prot &= !clrmask;
    data_prot &= !clrmask;

    unsafe {
        map_range(
            &mut pte,
            _stext as usize,
            __initdata_begin as usize,
            _stext as usize,
            text_prot,
            InitIdmap::ROOT_LEVEL,
            pg_dir as *mut PteEntry,
            false,
            0,
        );
        map_range(
            &mut pte,
            __initdata_begin as usize,
            _end as usize,
            __initdata_begin as usize,
            data_prot,
            InitIdmap::ROOT_LEVEL,
            pg_dir as *mut PteEntry,
            false,
            0,
        );

        let device_prot = PtePgProt::PROT_DEVICE_nGnRnE;
        let mut pte2 = &raw const DEVICE_PTES.0 as *mut DevicePtes as usize;
        let uart_device = kernel::arch::arm64::early_debug::EARLY_UART_BASE;
        map_range(
            &mut pte2,
            uart_device - 0x1000,
            uart_device + 0x1000,
            uart_device - 0x1000,
            device_prot,
            InitIdmap::ROOT_LEVEL,
            pg_dir as *mut PteEntry,
            false,
            0,
        );
    }
    pte
}

#[page_aligned]
struct FdtPtes([u8; InitIdmap::EARLY_FDT_PAGE_SIZE]);

#[section_init_data]
static mut FDT_PTES: FdtPtes = FdtPtes([0; InitIdmap::EARLY_FDT_PAGE_SIZE]);

// Create fdt map
#[section_init_text]
fn map_fdt(fdt: usize) {
    let efdt = fdt + InitIdmap::MAX_FDT_SIZE; 
    unsafe {
        let mut pte = &raw const FDT_PTES.0 as *mut FdtPtes as usize;
        map_range(
            &mut pte,
            fdt,
            min(_text as usize, efdt),
            fdt,
            PtePgProt::PAGE_KERNEL,
            InitIdmap::ROOT_LEVEL,
            init_idmap_pg_dir as *mut PteEntry,
            false,
            0,
        );
    }
    dsb(ISHST);
}

// Create kernel map
#[section_init_text]
fn map_segment(
    pg_dir: *mut PgdirEntry,
    pte: &mut usize,
    va_offset: usize,
    start: usize,
    end: usize,
    prot: PtePgProt,
    may_use_cont: bool,
    root_level: usize,
) {
    unsafe {
        map_range(
            pte,
            ((start + va_offset) & !Arm64VaLayout::KERNNEL_VA_START) as usize,
            ((end + va_offset) & !Arm64VaLayout::KERNNEL_VA_START) as usize,
            start,
            prot,
            root_level,
            pg_dir as *mut PteEntry,
            may_use_cont,
            0,
        );
    }
}


// Create kernel map
#[section_init_text]
fn map_kernel(va_offset: usize) {
    let rootlevel = 4 - Arm64PgtableConfig::PGTABLE_LEVELS;
    let mut pte = (init_pg_dir as usize) + PageConfig::PAGE_SIZE;
    let text_prot = PtePgProt::PAGE_KERNEL_ROX;
    let data_prot = PtePgProt::PAGE_KERNEL;
    // text segment
    map_segment(
        init_pg_dir as *mut PgdirEntry,
        &mut pte,
        va_offset,
        _stext as usize,
        _etext as usize,
        text_prot,
        true,
        rootlevel,
    );

    // rodata segment: include swapper_pg_dir reserved_pg_dir
    map_segment(
        init_pg_dir as *mut PgdirEntry,
        &mut pte,
        va_offset,
        __start_rodata as usize,
        __inittext_begin as usize,
        data_prot,
        false,
        rootlevel,
    );

    // init text segment
    map_segment(
        init_pg_dir as *mut PgdirEntry,
        &mut pte,
        va_offset,
        __inittext_begin as usize,
        __inittext_end as usize,
        text_prot,
        false,
        rootlevel,
    );

    // init data segment
    map_segment(
        init_pg_dir as *mut PgdirEntry,
        &mut pte,
        va_offset,
        __initdata_begin as usize,
        __initdata_end as usize,
        data_prot,
        false,
        rootlevel,
    );

    // data segment
    map_segment(
        init_pg_dir as *mut PgdirEntry,
        &mut pte,
        va_offset,
        _data as usize,
        _end as usize,
        data_prot,
        true,
        rootlevel,
    );

    // map uart device
    unsafe {
        let device_prot = PtePgProt::PROT_DEVICE_nGnRnE;
        let mut pte2 = &raw const DEVICE_PTES.0 as *mut DevicePtes as usize;
        let uart_device = kernel::arch::arm64::early_debug::EARLY_UART_BASE;
        map_range(
            &mut pte2,
            uart_device - 0x1000,
            uart_device + 0x1000,
            uart_device - 0x1000,
            device_prot,
            rootlevel,
            init_pg_dir as *mut PteEntry,
            false,
            0,
        );
    }

    dsb(ISHST);
    idmap_cpu_replace_ttbr1(init_pg_dir as usize);
    // Copy the root page table to its final location
    // Here swapper_pg_dir must use VA,since on stage1 idmap swapper_pg_dir
    // is mapped to text segment,it does not have write permission.
    // init_pg_dir can use PA because on stage1 init_pg_dir is mapped to
    // data segment, it has write/read permission.
    memcpy((swapper_pg_dir as usize + va_offset) as *mut u8, init_pg_dir as *mut u8, PageConfig::PAGE_SIZE);
    idmap_cpu_replace_ttbr1(swapper_pg_dir as usize);
}

/// Create initial ID map
#[no_mangle]
#[section_init_text]
pub unsafe extern "C" fn early_map_kernel(_boot_status: usize, fdt: usize) {
    map_fdt(fdt);
    // clear ZERO section
    memset(__bss_start as *mut u8, 0, init_pg_end as usize - __bss_start as usize);
    let va_base = Arm64VaLayout::KIMAGE_VADDR;
    let pa_base = _text as usize;
    map_kernel(va_base - pa_base);
}

#[inline(always)]
fn __idmap_cpu_set_reserved_ttbr1() {
    Ttbr1El1::write_pg_dir(reserved_pg_dir as u64);
    isb();
    TlbFlushOps::local_flush_tlb_all();
}

// Should not be called by anything else. It can only be executed from a TTBR0 mapping.
#[inline(always)]
fn idmap_cpu_replace_ttbr1(ttbr1: usize) {
    __idmap_cpu_set_reserved_ttbr1();
    Ttbr1El1::write_pg_dir(ttbr1 as u64);
    isb();
}

#[allow(dead_code)]
fn dump_page_table(
    start_addr: usize,
    end_addr: usize,
) {
    let mut addr = start_addr;
    let items_per_line = 4;
    early_uart_putchar(b'\n');
    while addr < end_addr {
        early_uart_put_u64_hex(addr as u64);
        early_uart_putchar(b':');
        early_uart_putchar(b' ');

        for _i in 0..items_per_line {
            if addr >= end_addr {
                break;
            }
            let value = unsafe { core::ptr::read_volatile(addr as *const u64) };
            early_uart_put_u64_hex(value);
            early_uart_putchar(b' ');
            addr += core::mem::size_of::<u64>();
        }
        early_uart_putchar(b'\n');
    }
}
