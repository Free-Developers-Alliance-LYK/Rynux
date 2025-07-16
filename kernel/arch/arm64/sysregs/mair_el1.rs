//! ARM64 mair_el1

use crate::bitflags::bitflags;

bitflags! {
    /// MAIR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct MairEl1: u64 {
        /// Attr0
        const Attr0 = 0xff << 0;
        /// Attr1
        const Attr1 = 0xff << 8;
        /// Attr2
        const Attr2 = 0xff << 16;
        /// Attr3
        const Attr3 = 0xff << 24;
        /// Attr4
        const Attr4 = 0xff << 32;
        /// Attr5
        const Attr5 = 0xff << 40;
        /// Attr6
        const Attr6 = 0xff << 48;
        /// Attr7
        const Attr7 = 0xff << 56;
    }
}

/// Specify the purpose of attridx
pub enum MairAttrIdx {
    /// Normal memory
    Normal = 0,
    /// Normal memory with tags
    NormalTagged = 1,
    /// Non-cacheable normal memory
    NormalNc = 2,
    /// Device memory
    DeviceNgnRnE = 3,
    /// Device memory
    DeviceNgnRE = 4,
}

impl From<u64> for MairAttrIdx {
    #[inline(always)]
    fn from(value: u64) -> Self {
        match value {
            0 => MairAttrIdx::Normal,
            1 => MairAttrIdx::NormalTagged,
            2 => MairAttrIdx::NormalNc,
            3 => MairAttrIdx::DeviceNgnRnE,
            4 => MairAttrIdx::DeviceNgnRE,
            _ => panic!("Invalid MairAttrIdx value"),
        }
    }
}

/// Each attr type is 8 bits wide
pub enum MairAttr {
    /// Device memory
    DeviceNgnRnE = 0x00,
    /// Device memory
    DeviceNgnRE = 0x04,
    /// Non-cacheable normal memory
    NormalNc = 0x44,
    /// Normal memory with tags
    NormalTagged = 0xf0,
    /// Normal memory
    Normal = 0xff,
}

impl From<u64> for MairAttr {
    #[inline(always)]
    fn from(value: u64) -> Self {
        match value {
            0x00 => MairAttr::DeviceNgnRnE,
            0x04 => MairAttr::DeviceNgnRE,
            0x44 => MairAttr::NormalNc,
            0xf0 => MairAttr::NormalTagged,
            0xff => MairAttr::Normal,
            _ => panic!("Invalid MairAttr value"),
        }
    }
}

impl MairEl1 {
    const ATTR_WIDTH: u64 = 8;
    const fn mair_attridx(attr: MairAttr, attridx: MairAttrIdx) -> u64 {
        (attr as u64) << ((attridx as u64) * Self::ATTR_WIDTH)
    }

    /// Default MAIR_EL1
    pub const MAIR_EL1_SET: Self = Self::from_bits_truncate(
        Self::mair_attridx(MairAttr::DeviceNgnRnE, MairAttrIdx::DeviceNgnRnE) |
        Self::mair_attridx(MairAttr::DeviceNgnRE, MairAttrIdx::DeviceNgnRE)   |
        Self::mair_attridx(MairAttr::Normal, MairAttrIdx::Normal) |
        Self::mair_attridx(MairAttr::NormalTagged, MairAttrIdx::NormalTagged) |
        Self::mair_attridx(MairAttr::NormalNc, MairAttrIdx::NormalNc)
    );

    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let mair: u64;
        sys_coproc_read_raw!(u64, "MAIR_EL1", "x", mair);
        Self::from_bits_truncate(mair)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let mair = self.bits();
        sys_coproc_write_raw!(u64, "MAIR_EL1", "x", mair);
    }
}
