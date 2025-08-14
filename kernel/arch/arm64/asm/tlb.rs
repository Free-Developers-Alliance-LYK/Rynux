//! TLB 

use crate::klib::bits::genmask64;
use crate::mm::VirtAddr;

#[allow(dead_code)]
enum FlushOps {
    Vaale1is
}

#[allow(dead_code)]
enum TlbLevel {
    Pte,
    Pmd,
    Pud,
    P4d,
    Pgdir,
    Unknown,
}

macro_rules! __TLBI_0 {
    ($op:literal) => {
        unsafe {
            core::arch::asm!(concat!("tlbi ", $op), options(nostack, nomem));
        }
    };
}

macro_rules! __TLBI_1 {
    ($op:literal, $arg:expr) => {
        unsafe {
            core::arch::asm!(concat!("tlbi ", $op, ", {arg}"),
                arg = in(reg) $arg,
                options(nostack, nomem)
            );
        }
    };
}

macro_rules! __tlbi {
    ($op:literal) => {
        __TLBI_0!($op)
    };
    ($op:literal, $arg:expr) => {
        __TLBI_1!($op, $arg)
    };
}

/// TLB flush operations
pub struct TlbFlushOps;

#[allow(dead_code)]
impl TlbFlushOps {
    #[inline(always)]
    const fn tlbi_range_pages(num: usize, scale: usize) -> usize {
        (num + 1) << (5 * scale + 1)
    }

    const MAX_TLBI_RANGE_PAGES: usize = Self::tlbi_range_pages(31, 3);

    #[inline(always)]
    fn flush_tlb_range_limit_excess(pages: usize) -> bool {
        if pages > Self::MAX_TLBI_RANGE_PAGES {
            return true;
        }
        false
    }

    #[inline(always)]
    fn tlb_range_num(pages: usize, scale: usize) -> usize {
        let num = pages.min(Self::tlbi_range_pages(31, scale));
        (num >> (5 * scale + 1)) - 1
    }

    #[inline(always)]
    fn tlbi_vaddr(addr: usize, asid: usize) -> usize {
        let mut ta: usize = 0;
        ta |= addr >> 12;
        ta &=  genmask64(43, 0) as usize;
        ta |= asid << 48;
        ta
    }

    #[inline(always)]
    fn tlbi_ops(ops: FlushOps, addr: usize) {
        match ops {
            FlushOps::Vaale1is => __tlbi!("vaale1is", addr),
        }
    }
    
    #[inline(always)]
    fn flush_tbl_range(_ops: FlushOps, _start: VirtAddr, _pages: usize, _stride: usize, 
        _asid: usize, _tlb_level: TlbLevel, _tlbi_user: bool) {
        todo!();
    }

    /// Flush local cpu TLB
    #[inline(always)]
    pub fn local_flush_tlb_all() {
        unsafe {
            core::arch::asm!("dsb nshst; tlbi vmalle1; dsb nsh; isb", options(nostack, nomem));
        }
    }

    /// Flush all tlb
    #[inline(always)]
    pub fn flush_tlb_all() {
        unsafe {
            core::arch::asm!("dsb ishst; tlbi vmalle1is; dsb ish; isb", options(nostack, nomem));
        }
    }

    /// flush tlb kernel range
    #[inline(always)]
    pub fn flush_tlb_kernel_range(_start: VirtAddr, _end: VirtAddr) {
        Self::flush_tlb_all();
        /*
        let stride = PageConfig::PAGE_SIZE;
        let start = start.align_down_page();
        let end = end.align_up_page();
        let pages = (end - start) >> PageConfig::PAGE_SHIFT;
        if Self::flush_tlb_range_limit_excess(start, end, pages, PageConfig::PAGE_SIZE) {
            return;
        }
        dsb(ISHST);
        flush_tlb_range(FlushOps::Vaale1is, pages, start, stride, 0, TlbLevel::Unknown, false);
        dsb(ISH);
        isb();
        */
    }

}



