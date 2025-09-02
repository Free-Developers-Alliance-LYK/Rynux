//! Arm64 Virtual Address memory layout
//! |---------------|
//! |  User Space   |
//! |---------------|
//! |  Kernel Space |
//! |---------------|
//!
//! The Kernel Space Layout
//!
//! |---------------|
//! | Line Map      |
//! |---------------|
//! |  Modules      |
//! |---------------|
//! |  vmalloc      |
//! |---------------|
//! |  vmemmap      |
//! |---------------|
//! |   Guard       |
//! |---------------|
//! |  PCI I/O      |
//! |---------------|
//! |  Fixmap       |
//! |---------------|
//!

use crate::types::OnceCell;
use crate::macros::section_rodata_after_init;

use crate::{
    arch::valayout::ArchVaLayout,
    size::*,
    cfg_if,
    mm::page::{PageConfig, Page},
    macros::need_export,
    arch::arm64::mm::fixmap::FixMap,
};

/// Virtual Address Layout
pub struct Arm64VaLayout();

impl ArchVaLayout for Arm64VaLayout {
    #[inline(always)]
    fn kernel_va_start() -> usize {
        Arm64VaLayout::KERNNEL_VA_START
    }

    fn linear_map_end() -> usize { 
        Arm64VaLayout::KERNEL_VA_LINE_END
    }

    #[inline(always)]
    fn kimg_va_offset() -> usize {
        *KIMAGE_VOFFSET.get().unwrap()
    }
}

impl Arm64VaLayout {
    cfg_if! {
        if #[cfg(CONFIG_ARM64_VA_BITS_36)] {
            #[allow(missing_docs)]
            pub const VA_BITS: usize = 36;
        } else if #[cfg(CONFIG_ARM64_VA_BITS_39)] {
            #[allow(missing_docs)]
            pub const VA_BITS: usize = 39;
        } else if #[cfg(CONFIG_ARM64_VA_BITS_42)] {
            #[allow(missing_docs)]
            pub const VA_BITS: usize = 42;
        } else if #[cfg(CONFIG_ARM64_VA_BITS_47)] {
            #[allow(missing_docs)]
            pub const VA_BITS: usize = 47;
        } else if #[cfg(CONFIG_ARM64_VA_BITS_48)] {
            #[allow(missing_docs)]
            pub const VA_BITS: usize = 48;
        } else {
            compile_error!("Unknown VA_BITS");
        }
    }

    // the virtual address of the start of the linear map, at the start of
    // the TTBR1 address space.   
    #[inline(always)]
    const fn liner_map_start(vabits: usize) -> usize {
        (-(1_isize << vabits)) as usize
    }

    // The end of the linear map, where all other kernel mappings begin.
    #[inline(always)]
    const fn linear_map_end(vabits: usize) -> usize {
        (-(1_isize << (vabits - 1))) as usize
    }
    
    /// The virtual address of the start of the linear map, at the start of the
    /// TTBR1 address space.
    pub const KERNNEL_VA_START: usize = Self::liner_map_start(Self::VA_BITS);

    /// The end of the linear map, where all other kernel mappings begin.
    pub const KERNEL_VA_LINE_END: usize = Self::linear_map_end(Self::VA_BITS);

    /// The size of the linear map
    pub const KERNEL_VA_LINE_SIZE: usize = Self::KERNEL_VA_LINE_END - Self::KERNNEL_VA_START;

    /// MODULES_VADDR - the virtual address of the start of the module space.
    pub const MODULES_VADDR: usize = Self::KERNEL_VA_LINE_END;

    /// MODULES_VSIZE - the size of the module space.
    pub const MODULES_VSIZE: usize = SZ_2G;
    
    /// MODULES_END - the end of the module space.
    pub const MODULES_END: usize = Self::MODULES_VADDR + Self::MODULES_VSIZE;

    /// KIMAGE_VADDR - the virtual address of the start of the kernel image
    pub const  KIMAGE_VADDR: usize = Self::MODULES_END;

    // VMEMMAP_RANGE - the range of the vmemmap
    // If we are configured with a 52-bit kernel VA then our VMEMMAP_SIZE
    // needs to cover the memory region from the beginning of the 52-bit
    // PAGE_OFFSET all the way to PAGE_END for 48-bit. This allows us to
    // keep a constant PAGE_OFFSET and "fallback" to using the higher end
    // of the VMEMMAP where 52-bit support is not available in hardware.
    const VMEMMAP_RANGE: usize = Self::KERNEL_VA_LINE_END - Self::KERNNEL_VA_START;
    
    // VMEMMAP_SIZE - allows the whole linear region to be covered by
    //                a struct page array
    const VMEMMAP_SIZE: usize = (Self::VMEMMAP_RANGE >> PageConfig::PAGE_SHIFT) * core::mem::size_of::<Page>();

    /// VMEMMAP_START - the start of the vmemmap.
    pub const VMEMMAP_START: usize = Self::VMEMMAP_END - Self::VMEMMAP_SIZE;

    /// From 1G mem is vmemmap
    pub const VMEMMAP_END: usize = -(SZ_1G as isize) as usize;

    // Size of the PCI I/O space. This must remain a power of two so that
    // IO_SPACE_LIMIT acts as a mask for the low bits of I/O addresses.
    const PCI_IO_SIZE : usize = SZ_16M;

    /// PCI I/O Start
    pub const PCI_IO_START: usize = Self::VMEMMAP_END + SZ_8M;
    /// PCI I/O End
    pub const PCI_IO_END: usize = Self::PCI_IO_START + Self::PCI_IO_SIZE;


    /// FIXMAP VA LAYOUT 
    /// 
    /// -------  FIX START
    ///
    /// Temp Fixmap
    ///
    /// ------- FIX permanent start
    /// 
    /// Permanent Fixmap
    ///
    /// -------  FIX_TOP
    ///   8MB
    /// --------

    /// Fixmap TOP 
    pub const FIXMAP_TOP: usize = -(SZ_8M as isize) as usize;
    /// FIXMAP START
    pub const FIXMAP_START: usize = Self::FIXMAP_TOP - FixMap::FIXMAP_SIZE;
}

/// KIMAGE_VADDR - the virtual address of the start of the kernel image.
#[need_export]
pub static EXPORT_KIMAGE_VADDR: usize = Arm64VaLayout::KIMAGE_VADDR;

#[section_rodata_after_init]
static KIMAGE_VOFFSET : OnceCell<usize> = OnceCell::new();

/// Set kimage voffset
#[inline(always)]
pub fn set_kimage_va_offset(voffset: usize) {
    KIMAGE_VOFFSET.set(voffset);
}
