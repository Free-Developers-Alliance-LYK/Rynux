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
}
