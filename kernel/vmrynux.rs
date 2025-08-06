//! Vmrynux for linker
//!
//! /*
//! Helper macros to support writing architecture specific
//! linker scripts.
//! 
//! A minimal linker scripts has following content:
//! [This is a sample, architectures may have special requirements]
//! 
//! OUTPUT_FORMAT(...)
//! OUTPUT_ARCH(...)
//! ENTRY(...)
//! SECTIONS
//! {
//!      . = START;
//!      __init_begin = .;
//!      HEAD_TEXT_SECTION
//!      INIT_TEXT_SECTION(PAGE_SIZE)
//!      INIT_DATA_SECTION(...)
//!      PERCPU_SECTION(CACHELINE_SIZE)
//!      __init_end = .;
//! 
//!      _stext = .;
//!      TEXT_SECTION = 0
//!      _etext = .;
//! 
//!      _sdata = .;
//!      RO_DATA(PAGE_SIZE)
//!      RW_DATA(...)
//!      _edata = .;
//! 
//!      EXCEPTION_TABLE(...)
//! 
//!      BSS_SECTION(0, 0, 0)
//!      _end = .;
//! 
//!      STABS_DEBUG
//!      DWARF_DEBUG
//!      ELF_DETAILS
//! 
//!      DISCARDS                // must be the last
//! }
//! 
//! [__init_begin, __init_end] is the init section that may be freed after init
//!      // __init_begin and __init_end should be page aligned, so that we can
//!      // free the whole .init memory
//! [_stext, _etext] is the text section
//! [_sdata, _edata] is the data section
//! 
//! Some of the included output section have their own set of constants.
//! Examples are: [__initramfs_start, __initramfs_end] for initramfs and
//!               [__nosave_begin, __nosave_end] for the nosave data
//! 

use crate::{
    const_str_to_u8_array_with_null,
    macros::need_export,
    mm::page::PageConfig,
    linkage::FUNCTION_ALIGNMENT,
    arch::mm::ArchThreadMemLayout,
};

use const_format::concatcp;
use crate::arch::vmrynux::*;


const EXIT_TEXT: &str = concat!(
    "*(.exit.text) ",
    "*(.text.exit) ",
);

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_EXIT_TEXT: [u8; EXIT_TEXT.len()+1] = const_str_to_u8_array_with_null!(EXIT_TEXT);


const EXIT_DATA: &str = concat!(
   "*(.exit.data .exit.data.*) ",
   "*(.fini_array .fini_array.*) ",
   "*(.dtors .dtors.*) ",
);

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_EXIT_DATA: [u8; EXIT_DATA.len()+1] = const_str_to_u8_array_with_null!(EXIT_DATA);

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

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_DISCARDS: [u8; DISCARDS.len()+1] = const_str_to_u8_array_with_null!(DISCARDS);

/// LINKD HEAD TEXT
const HEAD_TEXT: &str = "KEEP(*(.head.text))";

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_HEAD_TEXT: [u8; HEAD_TEXT.len()+1] = const_str_to_u8_array_with_null!(HEAD_TEXT);

const ALIGN_FUNCTION: &str = concatcp!{
    ". = ALIGN(",FUNCTION_ALIGNMENT,")",
};

const STRUCT_ALIGNMENT: usize = 32;
const ALIGN_STRUCT: &str = concatcp!{
    ". = ALIGN(",STRUCT_ALIGNMENT,")",
};

/// LINKD IRQENTRY
const IRQENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __irqentry_text_start = .; \n",
    "*(.irqentry.text) \n",
    "__irqentry_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_IRQENTRY_TEXT: [u8; IRQENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(IRQENTRY_TEXT);

/// LINKD IRQENTRY
const SOFTIRQENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __softirqentry_text_start = .; \n",
    "*(.softirqentry.text) \n",
    "__softirqentry_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_SOFTIRQENTRY_TEXT: [u8; SOFTIRQENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(SOFTIRQENTRY_TEXT);

const ENTRY_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __entry_text_start = .; \n",
    "*(.entry.text) \n",
    "__entry_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_ENTRY_TEXT: [u8; ENTRY_TEXT.len()+1] = const_str_to_u8_array_with_null!(ENTRY_TEXT);

const TEXT_SPLIT: &str = concatcp!{
    "__split_text_start = .; \n",
    "*(.text.split .text.split.[0-9a-zA-Z_]*)  \n",
    "__split_text_end = .; \n",
};

const TEXT_UNLIKELY: &str = concatcp!{
    "__unlikely_text_start = .; \n",
    "*(.text.unlikely .text.unlikely.*) \n", 
    "__unlikely_text_end = .; \n",
};

const TEXT_HOT: &str = concatcp!{
    "__hot_text_start = .; \n",
    "*(.text.hot .text.hot.*) \n",
    "__hot_text_end = .; \n",
};

const NOINSTR_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, " ; \n",
    "__noinstr_text_start = .; \n",
    "*(.noinstr.text)  \n",
    "__cpuidle_text_start = .; \n",
    "*(.cpuidle.text) \n",
    "__cpuidle_text_end = .; \n",
    "__noinstr_text_end = .; \n",
};

// TODO: dummy
const TEXT_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, " ; \n",
    "*(.text.unknown .text.unknown.*) \n",
    TEXT_SPLIT,
    TEXT_UNLIKELY,
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    TEXT_HOT,
    "*(.text .text.fixup) \n",
    NOINSTR_TEXT,
    "*(.ref.text)  \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_TEXT_TEXT: [u8; TEXT_TEXT.len()+1] = const_str_to_u8_array_with_null!(TEXT_TEXT);

const SCHED_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, " ; \n",
    " __sched_text_start = .; \n",
    "*(.sched.text) \n",
    "__sched_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_SCHED_TEXT: [u8; SCHED_TEXT.len()+1] = const_str_to_u8_array_with_null!(SCHED_TEXT);

const LOCK_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __lock_text_start = .; \n",
    "*(.spinlock.text) \n",
    "__lock_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_LOCK_TEXT: [u8; LOCK_TEXT.len()+1] = const_str_to_u8_array_with_null!(LOCK_TEXT);

const KPROBES_TEXT: &str = concatcp!{
    ALIGN_FUNCTION, "; \n",
    " __kprobes_text_start = .; \n",
    "*(.kprobes.text) \n",
    "__kprobes_text_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_KPROBES_TEXT: [u8; KPROBES_TEXT.len()+1] = const_str_to_u8_array_with_null!(KPROBES_TEXT);


const SCHED_DATA: &str = concatcp!{
    ALIGN_STRUCT, " ; \n",
    "__sched_class_highest = .; \n",
    "*(__stop_sched_class) \n",
    "*(__dl_sched_class) \n",
    "*(__rt_sched_class) \n",
    "*(__fair_sched_class) \n",
    "*(__idle_sched_class) \n",
    "__sched_class_lowest = .; \n",
};

const RO_AFTER_INIT_DATA: &str = concatcp!{
    ". = ALIGN(8) ; \n",
    "__start_ro_after_init = .; \n",
    "*(.data..ro_after_init) \n",
    "__end_ro_after_init = .; \n",
};
const NOTES: &str = concatcp!{
    "/DISCARD/ : { \n",
        "*(.note.GNU-stack) \n",
        "*(.note.gnu.property) \n",
    "} \n",
    ".notes : AT(ADDR(.notes) -", LOAD_OFFSET, ") { \n",
        "__start_notes = .; \n",
        "KEEP(*(.note.*)) \n",
        "__stop_notes = .; \n",
    "} \n",
};

const RO_DATA: &str = concatcp!{
    ". = ALIGN(", RO_DATA_ALIGN, "); \n",
    ".rodata : AT(ADDR(.rodata) -", LOAD_OFFSET, ") { \n",
        " __start_rodata = .; \n",
        "*(.rodata) *(.rodata.*) *(.data.rel.ro*) \n",
        SCHED_DATA,
        RO_AFTER_INIT_DATA,
    "} \n",

    ".rodata1 : AT(ADDR(.rodata1) -", LOAD_OFFSET, ") { \n",
        "*(.rodata1) \n",
    "} \n",
    NOTES,
    ". = ALIGN(", RO_DATA_ALIGN, "); \n",
    "__end_rodata = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_RO_DATA: [u8; RO_DATA.len()+1] = const_str_to_u8_array_with_null!(RO_DATA);

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

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_INIT_TEXT_SECTION: [u8; INIT_TEXT_SECTION.len()+1] = const_str_to_u8_array_with_null!(INIT_TEXT_SECTION);

const INIT_DATA: &str = concatcp!{
    "KEEP(*(SORT(___kentry+*))) \n",
    "*(.init.data .init.data.*) \n",
    "*(.init.rodata .init.rodata.*) \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_INIT_DATA: [u8; INIT_DATA.len()+1] = const_str_to_u8_array_with_null!(INIT_DATA);

const INIT_SETUP: &str = concatcp!{
    ". = ALIGN(", INIT_SETUP_ALIGN, "); \n",
    "__setup_start = .; \n",
    "*(.init.setup) \n",
    "__setup_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_INIT_SETUP: [u8; INIT_SETUP.len()+1] = const_str_to_u8_array_with_null!(INIT_SETUP);


const INIT_CALLS: &str = concatcp!{
    "__initcall_start = .; \n",
    "KEEP(*(.initcallearly.init)) \n",
    "__initcall0_start = .; KEEP(*(.initcall0.init)) KEEP(*(.initcall0s.init)) \n",
    "__initcall1_start = .; KEEP(*(.initcall1.init)) KEEP(*(.initcall1s.init)) \n",
    "__initcall2_start = .; KEEP(*(.initcall2.init)) KEEP(*(.initcall2s.init)) \n",
    "__initcall3_start = .; KEEP(*(.initcall3.init)) KEEP(*(.initcall3s.init)) \n",
    "__initcall4_start = .; KEEP(*(.initcall4.init)) KEEP(*(.initcall4s.init)) \n",
    "__initcall5_start = .; KEEP(*(.initcall5.init)) KEEP(*(.initcall5s.init)) \n",
    "__initcall6_start = .; KEEP(*(.initcall6.init)) KEEP(*(.initcall6s.init)) \n",
    "__initcall7_start = .; KEEP(*(.initcall7.init)) KEEP(*(.initcall7s.init)) \n",
    "__initcall_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_INIT_CALLS: [u8; INIT_CALLS.len()+1] = const_str_to_u8_array_with_null!(INIT_CALLS);

const CON_INITCALL: &str = concatcp!{
    "__con_initcall_start = .; \n",
    "KEEP(*(.con_initcall.init)) \n",
    "__con_initcall_end = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_CON_INITCALL: [u8; CON_INITCALL.len()+1] = const_str_to_u8_array_with_null!(CON_INITCALL);

const INIT_RAM_FS: &str = concatcp!{
    ". = ALIGN(4); \n",
    "__initramfs_start = .; \n",
    "KEEP(*(.init.ramfs)) \n",
    ". = ALIGN(8); \n",
    "KEEP(*(.init.ramfs.info)) \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_INIT_RAM_FS: [u8; INIT_RAM_FS.len()+1] = const_str_to_u8_array_with_null!(INIT_RAM_FS);


const PERCPU_INPUT: &str = concatcp!{
    "__per_cpu_start = .; \n",
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    "*(.data..percpu..page_aligned) \n",
    ". = ALIGN(", PERCPU_CACHE_ALIGN, "); \n",
    "__per_cpu_hot_start = .; \n",
    "*(SORT_BY_ALIGNMENT(.data..percpu..hot.*)) \n",
    "__per_cpu_hot_end = .; \n",
    ". = ALIGN(", PERCPU_CACHE_ALIGN, "); \n",
    "*(.data..percpu..read_mostly) \n",
    ". = ALIGN(", PERCPU_CACHE_ALIGN, "); \n",
    "*(.data..percpu) \n",
    "*(.data..percpu..shared_aligned) \n",
    "__per_cpu_end = .; \n",
};

const PERCPU_SECTION: &str = concatcp!{
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    ".data..percpu   : AT(ADDR(.data..percpu) -", LOAD_OFFSET, ") { \n",
    PERCPU_INPUT,
    "} \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_PERCPU_SECTION: [u8; PERCPU_SECTION.len()+1] = const_str_to_u8_array_with_null!(PERCPU_SECTION);

const INIT_TASK_DATA: &str = concatcp!{
    ". = ALIGN(", ArchThreadMemLayout::THREAD_ALIGN, "); \n",
    "__start_init_stack = .; \n",
    "init_thread_union = .; \n",
    "init_stack = .; \n",
    "KEEP(*(.data..init_thread_info)) \n",
    ". = __start_init_stack + " , ArchThreadMemLayout::THREAD_SIZE,"; \n",
    "__end_init_task = .; \n",
};

const READ_MOSTLY_DATA: &str = concatcp!{
    ". = ALIGN(", CACHE_ALIGN, "); \n",
    "*(.data..read_mostly) \n",
    ". = ALIGN(", CACHE_ALIGN, "); \n",
};

const CACHELINE_ALIGNED_DATA: &str = concatcp!{
    ". = ALIGN(", CACHE_ALIGN, "); \n",
    "*(.data..cacheline_aligned) \n",
};

// .data section
const DATA_DATA: &str = concatcp!{
    "*(.xiptext) \n",
    "*(.data .data.rel .data.rel.local) \n"
};

const RW_DATA: &str = concatcp!{
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    ".data : AT(ADDR(.data) -", LOAD_OFFSET, ") { \n",
    INIT_TASK_DATA,
    CACHELINE_ALIGNED_DATA,
    READ_MOSTLY_DATA,
    DATA_DATA,
    "} \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_RW_DATA: [u8; RW_DATA.len()+1] = const_str_to_u8_array_with_null!(RW_DATA);

const SBSS: &str = concatcp!{
    ". = ALIGN(", SBSS_ALIGN, "); \n",
    ".sbss : AT(ADDR(.sbss) -", LOAD_OFFSET, ") { \n",
    "*(.dynsbss) \n",
    "*(.sbss) \n",
    "*(.scommon) \n",
    "} \n",
};

const BSS: &str = concatcp!{
    ". = ALIGN(", BSS_ALIGN, "); \n",
    ".bss : AT(ADDR(.bss) -", LOAD_OFFSET, ") { \n",
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    "*(.bss..page_aligned) \n",
    ". = ALIGN(", PageConfig::PAGE_SIZE, "); \n",
    "*(.dynbss) \n",
    "*(.bss) \n",
    "*(COMMON) \n",
    "} \n",
};

const BSS_SECTION: &str = concatcp!{
    ". = ALIGN(", SBSS_ALIGN, "); \n",
    " __bss_start = .; \n",
    SBSS,
    BSS,
    ". = ALIGN(", BSS_STOP_ALIGN, "); \n",
    "__bss_stop = .; \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_BSS_SECTION: [u8; BSS_SECTION.len()+1] = const_str_to_u8_array_with_null!(BSS_SECTION);

const STABS_DEBUG: &str = concatcp!{
    ".stab 0 : { *(.stab) } \n",
    ".stabstr 0 : { *(.stabstr) } \n",
    ".stab.excl 0 : { *(.stab.excl) } \n",
    ".stab.exclstr 0 : { *(.stab.exclstr) } \n",
    ".stab.index 0 : { *(.stab.index) } \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_STABS_DEBUG: [u8; STABS_DEBUG.len()+1] = const_str_to_u8_array_with_null!(STABS_DEBUG);

const DWARF_1: &str = concatcp!{
    ".debug 0 : { *(.debug) } \n",
    ".line 0 : { *(.line) } \n",
};

const GNU_DWARF_EXTENSION: &str = concatcp!{
    ".debug_srcinfo 0 : { *(.debug_srcinfo) } \n",
    ".debug_sfnames 0 : { *(.debug_sfnames) } \n",
};

const DWARF_1_1: &str = concatcp!{
    ".debug_aranges 0 : { *(.debug_aranges) } \n",
    ".debug_pubnames 0 : { *(.debug_pubnames) } \n",
};

const DWARF_2: &str = concatcp!{
    ".debug_info 0 : { *(.debug_info .gnu.linkonce.wi.*) } \n",
    ".debug_abbrev 0 : { *(.debug_abbrev) } \n",
    ".debug_line 0 : { *(.debug_line) } \n",
    ".debug_frame 0 : { *(.debug_frame) } \n",
    ".debug_str 0 : { *(.debug_str) } \n",
    ".debug_loc 0 : { *(.debug_loc) } \n",
    ".debug_macinfo 0 : { *(.debug_macinfo) } \n",
    ".debug_pubtypes 0 : { *(.debug_pubtypes) } \n",
};

const DWARF_3: &str = concatcp!{
    ".debug_ranges 0 : { *(.debug_ranges) } \n",
};

const DWARF_2_EXTENSION: &str = concatcp!{
    ".debug_weaknames 0 : { *(.debug_weaknames) } \n",
    ".debug_funcnames 0 : { *(.debug_funcnames) } \n",
    ".debug_typenames 0 : { *(.debug_typenames) } \n",
    ".debug_varnames 0 : { *(.debug_varnames) } \n",
};

const GNU_DWARF_2_EXTENSION: &str = concatcp!{
    ".debug_gnu_pubnames 0 : { *(.debug_gnu_pubnames) } \n",
    ".debug_gnu_pubtypes 0 : { *(.debug_gnu_pubtypes) } \n",
};

const DWARF_4: &str = concatcp!{
    ".debug_types 0 : { *(.debug_types) } \n",
};

const DWARF_5: &str = concatcp!{
    ".debug_addr 0 : { *(.debug_addr) } \n",
    ".debug_line_str 0 : { *(.debug_line_str) } \n",
    ".debug_loclists 0 : { *(.debug_loclists) } \n",
    ".debug_macro 0 : { *(.debug_macro) } \n",
    ".debug_names 0 : { *(.debug_names) } \n",
    ".debug_rnglists 0 : { *(.debug_rnglists) } \n",
    ".debug_str_offsets 0 : { *(.debug_str_offsets) } \n",
};

const DWARF_DEBUG: &str = concatcp!{
    DWARF_1,
    GNU_DWARF_EXTENSION,
    DWARF_1_1,
    DWARF_2,
    DWARF_3,
    DWARF_2_EXTENSION,
    GNU_DWARF_2_EXTENSION,
    DWARF_4,
    DWARF_5,
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_DWARF_DEBUG: [u8; DWARF_DEBUG.len()+1] = const_str_to_u8_array_with_null!(DWARF_DEBUG);

const ELF_DETAILS: &str = concatcp!{
    ".comment 0 : { *(.comment) } \n",
    ".symtab 0 : { *(.symtab) } \n",
    ".strtab 0 : { *(.strtab) } \n",
    ".shstrtab 0 : { *(.shstrtab) } \n",
};

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_ELF_DETAILS: [u8; ELF_DETAILS.len()+1] = const_str_to_u8_array_with_null!(ELF_DETAILS);

