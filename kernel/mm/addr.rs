//! Rynux mem addr

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
