//! Possible errors when attempting to create an `LinuxFdt`

/// Possible errors when attempting to create an `Fdt`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FdtError {
    /// The FDT had an invalid magic value
    BadMagic,
    /// The given pointer was null
    BadPtr,
    /// The slice passed in was too small to fit the given total size of the FDT
    /// structure
    BufferTooSmall,
}

impl core::fmt::Display for FdtError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FdtError::BadMagic => write!(f, "bad FDT magic value"),
            FdtError::BadPtr => write!(f, "an invalid pointer was passed"),
            FdtError::BufferTooSmall => {
                write!(f, "the given buffer was too small to contain a FDT header")
            }
        }
    }
}
