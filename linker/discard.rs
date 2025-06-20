//! Discard in linker

#[allow(dead_code)]
pub(crate) const EXIT_TEXT: &str = concat!(
    "*(.exit.text) ",
    "*(.text.exit) ",
);

#[allow(dead_code)]
pub(crate) const EXIT_DATA: &str = concat!(
   "*(.exit.data .exit.data.*) ",
   "*(.fini_array .fini_array.*) ",
   "*(.dtors .dtors.*) ",
);

#[allow(dead_code)]
pub(crate) const EXIT_CALL: &str = "*(.exitcall.exit) ";

pub(crate) const COMMON_DISCARDS: &str = concat!(
    "*(.discard) ",
    "*(.discard.*) ",
    "*(.export_symbol) ",
    "*(.no_trim_symbol) ",
    "*(.modinfo) ",
    "*(.gnu.version*) ",
);

pub use crate::arch::DISCARDS;
