//! Arm64  Sparse memory config

cfg_if::cfg_if! {
    if #[cfg(CONFIG_PAGE_SIZE_64KB)] {
        /// Section size bits
        ///
        /// Section size must be at least 512MB for 64K base
        /// page size config. Otherwise it will be less than
        /// MAX_PAGE_ORDER and the build process will fail.
        pub const SECTION_SIZE_BITS: usize = 29;
    } else {

        /// Section size bits
        ///
        /// Section size must be at least 128MB for 4K base
        /// page size config. Otherwise PMD based huge page
        /// entries could not be created for vmemmap mappings.
        /// 16K follows 4K for simplicity.
        pub const SECTION_SIZE_BITS: usize = 27;
    }
}
