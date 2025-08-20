//! FdtHeader

use crate::parsing::{BigEndianU32, FdtData};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct FdtHeader {
    /// FDT header magic
    magic: BigEndianU32,
    /// Total size in bytes of the FDT structure
    pub(crate) totalsize: BigEndianU32,
    /// Offset in bytes from the start of the header to the structure block
    off_dt_struct: BigEndianU32,
    /// Offset in bytes from the start of the header to the strings block
    off_dt_strings: BigEndianU32,
    /// Offset in bytes from the start of the header to the memory reservation
    /// block
    off_mem_rsvmap: BigEndianU32,
    /// FDT version
    version: BigEndianU32,
    /// Last compatible FDT version
    last_comp_version: BigEndianU32,
    /// System boot CPU ID
    boot_cpuid_phys: BigEndianU32,
    /// Length in bytes of the strings block
    size_dt_strings: BigEndianU32,
    /// Length in bytes of the struct block
    size_dt_struct: BigEndianU32,
}

impl FdtHeader {
    pub(crate) fn valid_magic(&self) -> bool {
        self.magic.get() == 0xd00dfeed
    }

    pub(crate) fn struct_range(&self) -> core::ops::Range<usize> {
        let start = self.off_dt_struct.get() as usize;
        let end = start + self.size_dt_struct.get() as usize;

        start..end
    }

    pub(crate) fn strings_range(&self) -> core::ops::Range<usize> {
        let start = self.off_dt_strings.get() as usize;
        let end = start + self.size_dt_strings.get() as usize;

        start..end
    }

    pub(crate) fn from_bytes(bytes: &mut FdtData<'_>) -> Option<Self> {
        Some(Self {
            magic: bytes.u32()?,
            totalsize: bytes.u32()?,
            off_dt_struct: bytes.u32()?,
            off_dt_strings: bytes.u32()?,
            off_mem_rsvmap: bytes.u32()?,
            version: bytes.u32()?,
            last_comp_version: bytes.u32()?,
            boot_cpuid_phys: bytes.u32()?,
            size_dt_strings: bytes.u32()?,
            size_dt_struct: bytes.u32()?,
        })
    }
}
