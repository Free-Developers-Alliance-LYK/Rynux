//! Rynux mem addr

use core::ops::{Add, AddAssign, Sub, SubAssign};

/// A physical memory address.
///
/// It's a wrapper type around an `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
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
