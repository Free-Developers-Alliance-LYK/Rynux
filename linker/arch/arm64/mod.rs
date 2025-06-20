//! Arm64 linker variables

use const_format::concatcp;
use crate::discard::{EXIT_CALL, COMMON_DISCARDS};

#[no_mangle]
/// LINKD DISCARDS FOR ARM64
pub static DISCARDS: &str = concatcp!{
    "/DISCARD/ : { ",
    "  ", EXIT_CALL, " ",
    "  ", COMMON_DISCARDS, " ",
    "}"
};
