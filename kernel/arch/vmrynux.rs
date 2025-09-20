//!  Arch Vmlinux

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        use crate::mm::page::PageConfig;
        use crate::arch::arm64::mm::cache::L1_CACHE_BYTES;
        /// Default discarded section
        ///
        /// Some archs want to discard exit text/data at runtime rather than
        /// link time due to cross-section references such as alt instructions,
        /// bug table, eh_frame, etc.  DISCARDS must be the last of output
        /// section definitions so that such archs put those in earlier section
        /// definitions.
        pub const EXIT_DISCARDS: &str = "";

        /// Offset
        pub static LOAD_OFFSET: usize = 0;
        /// INIT TEXT ALIGN
        pub const INIT_TEXT_ALIGN: usize = 8;
        /// RODATA
        pub const RO_DATA_ALIGN: usize = PageConfig::PAGE_SIZE;
        /// init setup align
        pub const INIT_SETUP_ALIGN: usize = 16;
        /// Percpu cache align
        pub const PERCPU_CACHE_ALIGN: usize = L1_CACHE_BYTES;
        /// SBSS ALIGN
        pub const SBSS_ALIGN: usize = 0;
        /// BSS ALIGN
        pub const BSS_ALIGN: usize = 0;
        /// BSS STOP ALIGN
        pub const BSS_STOP_ALIGN: usize = 0;
        /// cache align
        pub const CACHE_ALIGN: usize = L1_CACHE_BYTES;
    }
}
