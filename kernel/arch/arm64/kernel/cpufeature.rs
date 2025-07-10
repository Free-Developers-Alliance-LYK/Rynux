//! CPU features
use crate::macros::section_read_mostly;

/// Are we using non-global mappings?
#[section_read_mostly]
pub static ARM64_USE_NG_MAPPINGS: bool = false;
