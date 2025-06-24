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

use crate::size::*;
use crate::mm::page::{PAGE_SHIFT, Page};
use klib::cfg_if;

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
    } else if #[cfg(CONFIG_ARM64_VA_BITS_52)] {
        #[allow(missing_docs)]
        pub const VA_BITS: usize = 52;
    } else {
        compile_error!("Unknown VA_BITS");
    }
}

/// Minimum virtual address space size.
#[cfg(CONFIG_ARM64_16K_PAGES)]
pub const VA_BITS_MIN: usize = if VA_BITS > 48 { 47 } else { VA_BITS };

#[cfg(not(CONFIG_ARM64_16K_PAGES))]
/// Minimum virtual address space size.
pub const VA_BITS_MIN: usize = if VA_BITS > 48 { 48 } else { VA_BITS };

#[inline(always)]
const fn page_end(va: usize) -> usize {
    assert!(va > 0);
    (-(1_isize << (va - 1))) as usize
}

#[inline(always)]
const fn page_offset(va: usize) -> usize {
    (-(1_isize << va)) as usize
}

/// The virtual address of the start of the linear map, at the start of the
/// TTBR1 address space.
pub const KERNNEL_VA_START: usize = page_offset(VA_BITS);

/// The end of the linear map, where all other kernel mappings begin.
pub const KERNEL_VA_LINE_END: usize = page_end(VA_BITS_MIN);

/// MODULES_VADDR - the virtual address of the start of the module space.
pub const MODULES_VADDR: usize = page_end(VA_BITS_MIN);

/// MODULES_VSIZE - the size of the module space.
pub const MODULES_VSIZE: usize = SZ_2G;

/// MODULES_END - the end of the module space.
pub const MODULES_END: usize = MODULES_VADDR + MODULES_VSIZE;

/// KIMAGE_VADDR - the virtual address of the start of the kernel image.
#[no_mangle]
pub static KIMAGE_VADDR: usize = MODULES_END;

/// VMEMMAP_RANGE - the range of the vmemmap
/// If we are configured with a 52-bit kernel VA then our VMEMMAP_SIZE
/// needs to cover the memory region from the beginning of the 52-bit
/// PAGE_OFFSET all the way to PAGE_END for 48-bit. This allows us to
/// keep a constant PAGE_OFFSET and "fallback" to using the higher end
/// of the VMEMMAP where 52-bit support is not available in hardware.
pub const VMEMMAP_RANGE: usize = KERNEL_VA_LINE_END - KERNNEL_VA_START;

/// VMEMMAP_SIZE - allows the whole linear region to be covered by
///                a struct page array
pub const VMEMMAP_SIZE: usize = (VMEMMAP_RANGE >> PAGE_SHIFT)* core::mem::size_of::<Page>();

/// VMEMMAP_START - the start of the vmemmap.
pub const VMEMMAP_START: usize = VMEMMAP_END - VMEMMAP_SIZE;

/// From 1G mem is vmemmap
pub const VMEMMAP_END: usize = -(SZ_1G as isize) as usize;

/// Size of the PCI I/O space. This must remain a power of two so that
/// IO_SPACE_LIMIT acts as a mask for the low bits of I/O addresses.
const PCI_IO_SIZE : usize = SZ_16M;

/// PCI I/O Start
pub const PCI_IO_START: usize = VMEMMAP_END + SZ_8M;
/// PCI I/O End
pub const PCI_IO_END: usize = PCI_IO_START + PCI_IO_SIZE;

/// Fixmap Top
pub const FIXMAP_TOP: usize = -(SZ_8M as isize) as usize;
