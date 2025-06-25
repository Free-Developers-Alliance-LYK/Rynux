//! Vmrynux for linker

use const_format::concatcp;
use klib::{cfg_if, const_str_to_u8_array_with_null};

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
       const EXIT_DISCARDS: &str = "";

        static LOAD_OFFSET: usize = 0;

        const INIT_TEXT_ALIGN: usize = 8;

    }
}

const EXIT_TEXT: &str = concat!(
    "*(.exit.text) ",
    "*(.text.exit) ",
);


#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_EXIT_TEXT: [u8; EXIT_TEXT.len()+1] = const_str_to_u8_array_with_null!(EXIT_TEXT);

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
const HEAD_TEXT: &str = "KEEP(*(.head.text))";
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
    ". = ALIGN(",FUNCTION_ALIGNMENT,")",
};

/// LINKD IRQENTRY
const IRQENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __irqentry_text_start = .; \n",
    "*(.irqentry.text) \n",
    "__irqentry_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_IRQENTRY_TEXT: [u8; IRQENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(IRQENTRY_TEXT);

/// LINKD IRQENTRY
const SOFTIRQENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __softirqentry_text_start = .; \n",
    "*(.softirqentry.text) \n",
    "__softirqentry_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_SOFTIRQENTRY_TEXT: [u8; SOFTIRQENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(SOFTIRQENTRY_TEXT);

const ENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __entry_text_start = .; \n",
    "*(.entry.text) \n",
    "__entry_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_ENTRY_TEXT: [u8; ENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(ENTRY_TEXT);

// TODO: dummy
const TEXT_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    "*(.ref.text)  \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_TEXT_TEXT: [u8; TEXT_TEXT.len()+1] = const_str_to_u8_array_with_null!(TEXT_TEXT);

const SCHED_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, " ; \n",
    " __sched_text_start = .; \n",
    "*(.sched.text) \n",
    "__sched_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_SCHED_TEXT: [u8; SCHED_TEXT.len()+1] = const_str_to_u8_array_with_null!(SCHED_TEXT);

const LOCK_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __lock_text_start = .; \n",
    "*(.spinlock.text) \n",
    "__lock_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_LOCK_TEXT: [u8; LOCK_TEXT.len()+1] = const_str_to_u8_array_with_null!(LOCK_TEXT);

const KPROBES_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __kprobes_text_start = .; \n",
    "*(.kprobes.text) \n",
    "__kprobes_text_end = .; \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_KPROBES_TEXT: [u8; KPROBES_TEXT.len()+1] = const_str_to_u8_array_with_null!(KPROBES_TEXT);

const INIT_TEXT: &str = concatcp!{
    "*(.init.text .init.text.*) \n",
    "*(.text.startup)\n",
};

const INIT_TEXT_SECTION: &str = concatcp!{
    ". = ALIGN(",  INIT_TEXT_ALIGN, "); \n",
    ".init.text : AT(ADDR(.init.text) -", LOAD_OFFSET, ") { \n",
    " _sinittext = .; \n",
    INIT_TEXT,
    "_einittext = .; \n",
    "} \n",
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_INIT_TEXT_SECTION: [u8; INIT_TEXT_SECTION.len()+1] = const_str_to_u8_array_with_null!(INIT_TEXT_SECTION);

