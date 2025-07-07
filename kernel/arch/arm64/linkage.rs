//! ARM64-specific linkage code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_FUNCTION_ALIGNMENT_4B)] {
        /// Function alignment str
        pub const __ALIGN_STR: &str = ".balign 4";

        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! sym_code_start {
            ($name:ident, $body:expr) => {
                core::arch::global_asm!(
                    concat!(
                        ".globl ", stringify!($name), "; \n",
                        ".balign 4; \n",
                        stringify!($name), ":\n",
                        $body, "\n"
                    )
                );
            };
        }

    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_8B)] {
        /// Function alignment str
        pub const __ALIGN_STR: &str = ".balign 8";

        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! sym_code_start {
            ($name:ident, $body:expr) => {
                core::arch::global_asm!(
                    concat!(
                        ".globl ", stringify!($name), "; \n",
                        ".balign 8; \n",
                        stringify!($name), ":\n",
                        $body, "\n"
                    )
                );
            };
        }
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_16B)] {
        /// Function alignment str
        pub const __ALIGN_STR: &str = ".balign 16";
        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! sym_code_start {
            ($name:ident, $body:expr) => {
                core::arch::global_asm!(
                    concat!(
                        ".globl ", stringify!($name), "; \n",
                        ".balign 16; \n",
                        stringify!($name), ":\n",
                        $body, "\n"
                    )
                );
            };
        }
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_32B)] {
        /// Function alignment str
        pub const __ALIGN_STR: &str = ".balign 32";
        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! sym_code_start {
            ($name:ident, $body:expr) => {
                core::arch::global_asm!(
                    concat!(
                        ".globl ", stringify!($name), "; \n",
                        ".balign 32; \n",
                        stringify!($name), ":\n",
                        $body, "\n"
                    )
                );
            };
        }
    } else if #[cfg(CONFIG_FUNCTION_ALIGNMENT_64B)] {
        /// Function alignment str
        pub const __ALIGN_STR: &str = ".balign 64";
        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! sym_code_start {
            ($name:ident, $body:expr) => {
                core::arch::global_asm!(
                    concat!(
                        ".globl ", stringify!($name), "; \n",
                        ".balign 64; \n",
                        stringify!($name), ":\n",
                        $body, "\n"
                    )
                );
            };
        }
    } else {
        compile_error!("Unsupported function alignment");
    }
}

