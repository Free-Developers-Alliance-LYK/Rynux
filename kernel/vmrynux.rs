//! Vmrynux for linker

use crate::size::*;
use const_format::concatcp;
use klib::{cfg_if, const_str_to_u8_array_with_null};

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

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
       const EXIT_DISCARDS: &str = "";
        /// Alignment of kernel segments (e.g. .text, .data).
        /// 4 KB granule:  16 level 3 entries, with contiguous bit
        /// 16 KB granule:   4 level 3 entries, without contiguous bit
        /// 64 KB granule:   1 level 3 entry
        #[no_mangle]
        pub static SEGMENT_ALIGN: usize = SZ_64K;
    }
}

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


/// LINKD DISCARDS FOR ARM64
pub const DISCARDS: &str = concatcp!{
    "/DISCARD/ : { ",
    "  ", EXIT_DISCARDS, " ",
    "  ", EXIT_CALL, " ",
     "  ", COMMON_DISCARDS, " ",
     "}"
};

#[allow(missing_docs)]
#[no_mangle]
pub static EXPORT_DISCARDS: [u8; DISCARDS.len()+1] = const_str_to_u8_array_with_null!(DISCARDS);

/// LINKD HEAD TEXT
pub const HEAD_TEXT: &str = "KEEP(*(.head.text))";
#[allow(missing_docs)]
#[no_mangle]
pub static EXPORT_HEAD_TEXT: [u8; HEAD_TEXT.len()+1] = const_str_to_u8_array_with_null!(HEAD_TEXT);

cfg_if! {
    if #[cfg(CONFIG_FUNCTION_ALIGNMENT_4B)] {
        /// Function alignment
        pub const FUNCTION_ALIGNMENT: usize = 4;
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_8B)] {
        /// Function alignment
        pub const FUNCTION_ALIGNMENT: usize = 8;
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_16B)] {
        /// Function alignment
        pub const FUNCTION_ALIGNMENT: usize = 16;
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_32B)] {
        /// Function alignment
        pub const FUNCTION_ALIGNMENT: usize = 32;
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_64B)] {
        /// Function alignment
        pub const FUNCTION_ALIGNMENT: usize = 64;
    } else {
        compile_error!("Unsupported function alignment");
    }
}

const ALIGN_FUNCTION: &str = concatcp!{
    ". = ALIGN(",
    "  ", FUNCTION_ALIGNMENT, " ",
    ") ",
};

/// LINKD IRQENTRY
pub const IRQENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, " ; \n",
    " __irqentry_text_start = .; \n",
    "*(.irqentry.text) \n",
    "__irqentry_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_IRQENTRY_TEXT: [u8; IRQENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(IRQENTRY_TEXT);
