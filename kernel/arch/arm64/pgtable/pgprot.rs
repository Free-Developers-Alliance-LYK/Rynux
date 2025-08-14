//! Arm64 Page table properties

use crate::{
    bitflags::bitflags,
    klib::bits::genmask64,
    arch::arm64::sysregs::MairAttrIdx,
};

bitflags! {
    /// No address, only include page table attributes
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PtePgProt: u64 {
        //PTE descriptor
        /// Valid flag
        const PTE_VALID     = 1 << 0;
        /// Next is table or block, 1 is table
        const PTE_NON_BLOCK = 1 << 1;
        /// Memory attributes index field.
        const ATTR_INDX =   0b111 << 2;
        /// User flag
        const PTE_USER       = 1 << 6;
        /// Read only flag
        const PTE_RDONLY     = 1 << 7;
        /// Shared flag
        const PTE_SHARED     = 3 << 8;
        /// Access flag
        const PTE_AF         = 1 << 10;
        /// Not global flag
        const PTE_NG         = 1 << 11;
        /// Guarded page
        const PTE_GP         = 1 << 50;
        /// Dirty bit management
        const PTE_DBM        = 1 << 51;
        /// Contiguous range
        const PTE_CONT       = 1 << 52;
        /// Privileged execute never
        const PTE_PXN        = 1 << 53;
        /// User execute never
        const PTE_UXN        = 1 << 54;
    }
}


impl PtePgProt {
    /// Alias same as DBM
    pub const PTE_WRITE: Self =  Self::PTE_DBM;

    /// Software bits mask
    pub const PTE_SWBITS_MASK: u64 = (1 << 63) | genmask64(58, 55);
    /// Type mask
    pub const PTE_TYPE_MASK: u64  = 3 << 0;
    /// Type page PTE_VALID | PTE_NON_BLOCK
    pub const PTE_TYPE_PAGE: Self = Self::from_bits_truncate(Self::PTE_VALID.bits() | Self::PTE_NON_BLOCK.bits());

    /// Attribute index mask
    pub const PTE_ATTRINDX_MASK: u64 = 7 << 2;
    const PTE_ATTRINDX_SHIFT: u64 = 2;
    const fn pte_mair_attridx(mairidx: MairAttrIdx) -> Self {
        Self::from_bits_truncate((mairidx as u64) << Self::PTE_ATTRINDX_SHIFT)
    }

    /// Always false now
    #[inline(always)]
    pub fn lpa2_is_enabled() -> bool {
        false
    }

    /// Maybe shared bits is fixed
    #[inline(always)]
    pub const fn pte_maybe_shared() -> Self {
        Self::PTE_SHARED
    }
  
    /// Maybe not global
    #[inline(always)]
    pub const fn pte_maybe_ng() -> Self {
        use crate::arch::arm64::kernel::cpufeature::ARM64_USE_NG_MAPPINGS;
        if ARM64_USE_NG_MAPPINGS {
           Self::PTE_NG
        } else {
            Self::from_bits_truncate(0)
        }
    }

    /// Default page table attributes
    pub const PROT_DEFAULT: Self = Self::from_bits_truncate(Self::PTE_TYPE_PAGE.bits() | Self::pte_maybe_shared().bits() | Self::pte_maybe_ng().bits() | Self::PTE_AF.bits());

    /// Page Normal
    pub const PROT_NORMAL: Self = Self::from_bits_truncate(
        Self::PROT_DEFAULT.bits() | Self::PTE_PXN.bits()
        | Self::PTE_UXN.bits()
        | Self::PTE_WRITE.bits()
        | Self::pte_mair_attridx(MairAttrIdx::Normal).bits()
    );

    /// PROT_DEVICE_nGnRnE
    #[allow(non_upper_case_globals)]
    pub const PROT_DEVICE_nGnRnE: Self = Self::from_bits_truncate(
        Self::PROT_DEFAULT.bits() 
        | Self::PTE_PXN.bits()
        | Self::PTE_UXN.bits()
        | Self::PTE_WRITE.bits()
        | Self::pte_mair_attridx(MairAttrIdx::DeviceNgnRnE).bits()
    );

    /// PROT_DEVICE_nGnRE
    #[allow(non_upper_case_globals)]
    pub const PROT_DEVICE_nGnRE: Self = Self::from_bits_truncate(
        Self::PROT_DEFAULT.bits() | Self::PTE_PXN.bits()
        | Self::PTE_UXN.bits()
        | Self::PTE_WRITE.bits()
        | Self::pte_mair_attridx(MairAttrIdx::DeviceNgnRE).bits()
    );
    
    /// PROT_NORMAL_NC
    pub const PROT_NORMAL_NC: Self = Self::from_bits_truncate(
        Self::PROT_DEFAULT.bits() | Self::PTE_PXN.bits()
        | Self::PTE_UXN.bits()
        | Self::PTE_WRITE.bits()
        | Self::pte_mair_attridx(MairAttrIdx::NormalNc).bits()
    );

    /// PROT_NORMAL_TAGGED
    pub const PROT_NORMAL_TAGGED: Self = Self::from_bits_truncate(
        Self::PROT_DEFAULT.bits() | Self::PTE_PXN.bits()
        | Self::PTE_UXN.bits()
        | Self::PTE_WRITE.bits()
        | Self::pte_mair_attridx(MairAttrIdx::NormalTagged).bits()
    );

    /// Page kernel
    pub const PAGE_KERNEL: Self = Self::PROT_NORMAL;

    /// Page kernel read only
    pub const PAGE_KERNEL_RO: Self = Self::from_bits_truncate(Self::PROT_NORMAL.bits() & !(Self::PTE_WRITE.bits()) | Self::PTE_RDONLY.bits());

    /// Page kernel read only execute
    pub const PAGE_KERNEL_ROX: Self = Self::from_bits_truncate(Self::PROT_NORMAL.bits() & !(Self::PTE_WRITE.bits() | Self::PTE_PXN.bits()) | Self::PTE_RDONLY.bits());

}
