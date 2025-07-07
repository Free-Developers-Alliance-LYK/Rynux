//! Linkage code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use crate::arch::arm64::linkage::__ALIGN_STR;
    }
}

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

