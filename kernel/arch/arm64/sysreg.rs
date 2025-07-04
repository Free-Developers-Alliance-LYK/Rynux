//! ARM64 system registers

/// Current Exception Level
pub struct CurrentEL {}

impl CurrentEL {
    /// EL1
    pub const EL1: u64 = 0b0100;
    /// EL2
    pub const EL2: u64 = 0b1000;
}

/// SCRCTLR_ELx
pub struct SctlrElx {}

impl SctlrElx {
    /// EE bit shift
    pub const EE_SHIFT: u64 = 25;

    /// EE bit
    pub const EE: u64 = 1 << Self::EE_SHIFT;

    /// SA bit
    pub const SA: u64 = 1 << 3;
    /// Data cache BIT
    pub const C: u64 = 1 << 2;
    /// Alignment check
    pub const A: u64 = 1 << 1;
    /// mmu enable BIT
    pub const M: u64 = 1 << 0;
}
