//! ARM64 midr_el1

use crate::bitflags::bitflags;

bitflags! {
    /// MIDR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct MidrEl1: u64 {
        /// Implementer
        const IMPLEMENTER = 0xff << 24;
        /// Variant
        const VARIANT = 0xf << 20;
        /// Architecture
        const ARCHITECTURE = 0xf << 16;
        /// Primary part number
        const PRIMARY_PART_NUMBER = 0xfff << 4;
        /// Revision
        const REVISION = 0xf << 0;
    }
}

// CPU Implementer
#[allow(dead_code)]
enum ArmCpuImplementer {
    /// ARM
    ARM = 0x41,
    /// APM
    APM = 0x50,
    /// Cavium
    Cavium = 0x43,
    /// Broadcom
    Broadcom = 0x42,
    /// Qualcomm
    Qualcomm = 0x51,
    /// NVIDIA
    NVIDIA = 0x4E,
    /// Fujitsu
    Fujitsu = 0x46,
    /// HiSilicon
    HiSilicon = 0x48,
    /// Apple
    Apple = 0x61,
    /// Ampere
    Ampere = 0xC0,
    /// Microsoft
    Microsoft = 0x6D,
}

impl ArmCpuImplementer {
    const IMPLEMENTER_OFFSET: u64 = 24;

    #[inline(always)]
    const fn implementer(x: ArmCpuImplementer) -> u64 {
        (x as u64) << Self::IMPLEMENTER_OFFSET
    }
}

// CPU Part Number
#[allow(dead_code)]
enum ArmCpuPartNumber {
    /// AEMv8
    AEMv8 = 0xD0F,
    /// Foundation
    Foundation = 0xD00,
    /// Cortex-A57
    CortexA57 = 0xD07,
    /// Cortex-A72
    CortexA72 = 0xD08,
    /// Cortex-A53
    CortexA53 = 0xD03,
    /// Cortex-A73
    CortexA73 = 0xD09,
    /// Cortex-A75
    CortexA75 = 0xD0A,
    /// Cortex-A35
    CortexA35 = 0xD04,
    /// Cortex-A55
    CortexA55 = 0xD05,
    /// Cortex-A76
    CortexA76 = 0xD0B,
    /// Neoverse-N1
    NeoverseN1 = 0xD0C,
    /// Cortex-A77
    CortexA77 = 0xD0D,
    /// Cortex-A76AE
    CortexA76AE = 0xD0E,
    /// Neoverse-V1
    NeoverseV1 = 0xD40,
    /// Cortex-A78
    CortexA78 = 0xD41,
    /// Cortex-A78AE
    CortexA78AE = 0xD42,
    /// Cortex-X1
    CortexX1 = 0xD44,
    /// Cortex-A510
    CortexA510 = 0xD46,
    /// Cortex-X1C
    CortexX1C = 0xD4C,
    /// Cortex-X2
    CortexX2 = 0xD48,
    /// Neoverse-N2
    NeoverseN2 = 0xD49,
    /// Cortex-A78C
    CortexA78C = 0xD4B,
    /// Cortex-X3
    CortexX3 = 0xD4E,
    /// Neoverse-V2
    NeoverseV2 = 0xD4F,
    /// Cortex-A720
    CortexA720 = 0xD81,
    /// Cortex-X4
    CortexX4 = 0xD82,
    /// Neoverse-V3
    NeoverseV3 = 0xD84,
    /// Cortex-X925
    CortexX925 = 0xD85,
    /// Cortex-A725
    CortexA725 = 0xD87,
    /// Neoverse-N3
    NeoverseN3 = 0xD8E,

    /// APM X-Gene
    ApmXgene = 0x000,

    /// Cavium ThunderX
    CaviumThunderX = 0x0A1,
    /// Cavium ThunderX 81
    CaviumThunderX81 = 0x0A2,
    /// Cavium ThunderX 83
    CaviumThunderX83 = 0x0A3,
    /// Cavium ThunderX 2
    CaviumThunderX2 = 0x0AF,

    /// Fujitsu A64FX
    FujitsuA64FX = 0x001,
}

impl ArmCpuPartNumber {
    const PART_NUMBER_OFFSET: u64 = 4;

    #[inline(always)]
    const fn part_number(x: ArmCpuPartNumber) -> u64 {
        (x as u64) << Self::PART_NUMBER_OFFSET
    }
}

impl MidrEl1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let midr: u64;
        sys_coproc_read_raw!(u64, "MIDR_EL1", "x", midr);
        Self::from_bits_truncate(midr)
    }

    /// Read register raw.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let midr: u64;
        sys_coproc_read_raw!(u64, "MIDR_EL1", "x", midr);
        midr
    }

    const ARCHITECTURE_OFFSET: u64 = 16;
    #[inline(always)]
    const fn cpu_model(implementer: ArmCpuImplementer, part_number: ArmCpuPartNumber) -> u64 {
        ArmCpuImplementer::implementer(implementer) | ArmCpuPartNumber::part_number(part_number)
            | 0xf << Self::ARCHITECTURE_OFFSET
    }

    const VARIANT_OFFSET: u64 = 20;
    /// Variant and revision
    #[inline(always)]
    pub const fn cpu_var_rev(var: u64, rev: u64) -> u64 {
        (var << Self::VARIANT_OFFSET) | (rev << 0)
    }

}

/// Fujitsu A64FX
pub(crate) const FUJITSU_A64FX: MidrEl1 = MidrEl1::from_bits_truncate(
    MidrEl1::cpu_model(ArmCpuImplementer::Fujitsu, ArmCpuPartNumber::FujitsuA64FX)
);
