//! Page management code.

use crate:: {
    cfg_if,
    macros::need_export,
};

cfg_if! {
    if #[cfg(CONFIG_PAGE_SIZE_4KB)] {
        /// Page shift for 4KB pages.
        pub const PAGE_SHIFT: usize = 12;
    } else if #[cfg(CONFIG_PAGE_SIZE_8KB)] {
        /// Page shift for 8KB pages.
        pub const PAGE_SHIFT: usize = 13;
    } else if #[cfg(CONFIG_PAGE_SIZE_16KB)] {
        /// Page shift for 16KB pages.
        pub const PAGE_SHIFT: usize = 14;
    } else if #[cfg(CONFIG_PAGE_SIZE_32KB)] {
        /// Page shift for 32KB pages.
        pub const PAGE_SHIFT: usize = 15;
    } else if #[cfg(CONFIG_PAGE_SIZE_64KB)] {
        /// Page shift for 64KB pages.
        pub const PAGE_SHIFT: usize = 16;
    } else if #[cfg(CONFIG_PAGE_SIZE_256KB)] {
        /// Page shift for 256KB pages.
        pub const PAGE_SHIFT: usize = 18;
    }else {
        compile_error!("Unsupported page size");
    }
}

/// Size of a page PA SIZE in bytes.
#[need_export]
pub static PAGE_SIZE: usize = 1 << PAGE_SHIFT;

/// Page mask
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);


/// Page structure.
pub struct Page {
    _dummy: u64,
}
