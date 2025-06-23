//! Arm64 linker variables

use const_format::concatcp;
use crate::discard::{EXIT_CALL, COMMON_DISCARDS};

const STR_DISCARDS: &str = concatcp!{
    "/DISCARD/ : { ",
    "  ", EXIT_CALL, " ",
    "  ", COMMON_DISCARDS, " ",
    "}"
};

const fn to_array<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let mut arr = [0u8; N];
    let mut i = 0;
    while i < N {
        arr[i] = bytes[i];
        i += 1;
    }
    arr
}

#[no_mangle]
/// LINKD DISCARDS FOR ARM64
/// Use [u8] array because need to be exported
pub static DISCARDS: [u8; STR_DISCARDS.len()] = to_array::<{ STR_DISCARDS.len() }>(STR_DISCARDS);

