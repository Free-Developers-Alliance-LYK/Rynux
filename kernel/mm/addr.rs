//! Rynux mem addr

use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::mm::page::PageConfig;
use crate::arch::valayout::{ArchVaLayout, VaLayout};

/// Align address upwards.
///
/// Returns the smallest `x` with alignment `align` so that `x >= addr`.                
/// The alignment must be a power of two.
#[inline]                                                                  
const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Align address downwards.
///
/// Returns the greatest `x` with alignment `align` so that `x <= addr`.
///
/// The alignment must be a power of two.
#[inline]
const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

/// Align address offset
/// 
#[inline]
const fn align_offset(addr: usize, align: usize) -> usize {
    addr & (align - 1)
}


/// Is Aligned
#[inline]
const fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}

/// A physical memory address.
///
/// It's a wrapper type around an `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(usize);

impl PhysAddr {
    /// Converts an `usize` to a physical address.
    #[inline]                                                              
    pub const fn from(addr: usize) -> Self {
        Self(addr)
    }

    /// Converts the address to an `usize`.                                
    #[inline]                                                              
    pub const fn as_usize(self) -> usize {                                 
        self.0                                                             
    }

    /// Aligns the address upwards to the given alignment.
    ///
    /// See the [`align_up`] function for more information.
    #[inline]
    pub fn align_up<U>(self, align: U) -> Self
    where U: Into<usize>,
    {
        Self(align_up(self.0, align.into()))
    }

    /// Aligns the address downwards to the given alignment.
    /// 
    /// See the [`align_down`] function for more information.
    #[inline]
    pub fn align_down<U>(self, align: U) -> Self
    where U: Into<usize>,
    {
        Self(align_down(self.0, align.into()))
    }

    /// is Aligned 
    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where U: Into<usize>,
    {
        is_aligned(self.0, align.into())
    }

    /// align down to Page 
    #[inline]
    pub fn align_down_page(self) -> Self {
        Self(align_down(self.0, PageConfig::PAGE_SIZE))
    }

    /// Align offset page
    #[inline]
    pub fn align_offset_page(self) -> usize {
        align_offset(self.0, PageConfig::PAGE_SIZE)
    }

    /// To virtual address.
    /// TODO: implement start addr offset
    #[inline]
    pub fn to_virt(self) -> VirtAddr {
        VirtAddr::from(self.0 | VaLayout::kernel_va_start())
    }

    /// checked sub
    #[inline]
    pub fn checked_sub(self, rhs: PhysAddr) -> Option<PhysAddr> {
        self.0.checked_sub(rhs.0).map(PhysAddr)
    }

    /// PFN
    #[inline]
    pub fn pfn(self) -> usize {
        self.0 >> PageConfig::PAGE_SHIFT
    }

}

impl From<usize> for PhysAddr {
    #[inline]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl PhysAddr {
    #[inline]
    /// Saturating add
    pub const fn saturating_add(self, off: usize) -> Self {
        Self(self.0.saturating_add(off))
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl AddAssign<usize> for PhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for PhysAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = usize;
    #[inline]
    fn sub(self, rhs: PhysAddr) -> Self::Output {
        self.0.checked_sub(rhs.0).expect("address underflow")
    }
}

/// A virtual memory address.
///
/// It's a wrapper type around an `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(usize);

impl VirtAddr {
      /// Converts an `usize` to a virtual address.
      #[inline]                                                              
      pub const fn from(addr: usize) -> Self {
          Self(addr)
      }

      /// Converts the address to an `usize`.
      #[inline]
      pub const fn as_usize(self) -> usize {
          self.0
      }

      /// Converts to a NonNull<[u8]> pointer.
      /// SAFETY: The caller must ensure that the address is valid and aligned.
      #[inline]
      pub const unsafe fn as_mut_ptr(self) -> *mut u8 {
          self.0 as *mut u8
      }

      /// A kernel symbol to phys
      #[inline]
      fn symbol_to_phys(self) -> PhysAddr {
          PhysAddr::from(self.0 - VaLayout::kimg_va_offset())
      }
      #[inline]
      fn is_lm_address(self) -> bool {
          (VaLayout::kernel_va_start()..VaLayout::linear_map_end()).contains(&self.0)
      }

      /// Convert to a physical address.
      #[inline]
      pub fn to_phys(self) -> PhysAddr {
         if self.is_lm_address() {
             PhysAddr::from(self.0 - VaLayout::kernel_va_start())
         } else {
             self.symbol_to_phys()
         }
      }


      /// Is Aligend 
      #[inline]
      pub fn is_aligned<U>(self, align: U) -> bool
      where U: Into<usize>,
      {
          is_aligned(self.0, align.into())
      }

      /// Align down to page
      #[inline]
      pub fn align_down_page(self) -> Self {
          Self(align_down(self.0, PageConfig::PAGE_SIZE))
      }

      /// Align up to page
      #[inline]
      pub fn align_up_page(self) -> Self {
          Self(align_up(self.0, PageConfig::PAGE_SIZE))
      }

      #[inline]
      /// Align offset page
      pub fn align_offset_page(self) -> usize {
          align_offset(self.0, PageConfig::PAGE_SIZE)
      }


}

impl Add<usize> for VirtAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for VirtAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = usize;
    #[inline]
    fn sub(self, rhs: VirtAddr) -> Self::Output {
        self.0.checked_sub(rhs.0).expect("address underflow")
    }
}


impl AddAssign<usize> for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for VirtAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl fmt::LowerHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::UpperHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#X}", self.0))
    }
}

impl fmt::LowerHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl fmt::UpperHex for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#X}", self.0))
    }
}
