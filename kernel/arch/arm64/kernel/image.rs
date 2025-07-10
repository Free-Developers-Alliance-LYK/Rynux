//! Rynux arm64 image header

use crate::{
    mm::page::PAGE_SHIFT,
    cfg_if,
    const_str_to_u8_array_with_null,
    static_assertions::const_assert_eq,
    macros::need_export,
    const_format::concatcp,
};

/// Image symbols
pub mod symbols {

    // Define in vmrynux.lds.S
    extern "C" {
        /// early init stack
        pub static early_init_stack: usize;
        /// init idmap page directory
        pub static init_idmap_pg_dir: usize;
        /// start of text
        pub static _stext: usize;
        /// start of init data
        pub static __initdata_begin: usize;
        /// end of text
        pub static _etext: usize;
        /// end of kernel
        pub static _end: usize;

        /// init idmap page directory end
        pub static __pi_create_init_idmap: usize;
    }

}


cfg_if! {
    if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
        const fn data_le32(data: u64) -> u64 {
            ((data & 0x000000ff) << 24) |
            ((data & 0x0000ff00) << 8)  |
            ((data & 0x00ff0000) >> 8)  |
            ((data & 0xff000000) >> 24)
        }
        macro_rules! data_le32_macro {
            ($data:expr) => {
                concat!(
                    "(((", $data, ") & 0x000000ff) << 24) | ",
                    "((", $data, ") & 0x0000ff00) << 8)  | ",
                    "((", $data, ") & 0x00ff0000) >> 8)  | ",
                    "((", $data, ") & 0xff000000) >> 24)\n"
                )
            };
        }
    } else {
        #[allow(dead_code)]
        const fn data_le32(data: u64) -> u64 {
            data & 0xffffffff
        }

        macro_rules! data_le32_macro {
            ($data:expr) => {
                concat!("((", $data, ") & 0xffffffff)\n")
            };
        }
    }
}

macro_rules! define_image_le64_macro {
    ($sym:expr, $data:expr) => {
        concat!(
            $sym, "_hi32 = ", data_le32_macro!($data), " >> 32;\n",
            $sym, "_lo32 = ", data_le32_macro!($data), "& 0xffffffff;\n"
        )
    };
}
#[allow(dead_code)]
struct HeadFlags();

#[allow(dead_code)]
impl HeadFlags {
    const ARM64_IMAGE_FLAG_LE: u64 = 0;
    const ARM64_IMAGE_FLAG_BE: u64 = 1;

    const ARM64_IMAGE_FLAG_PAGE_SIZE_4K: u64 = 1;
    const ARM64_IMAGE_FLAG_PAGE_SIZE_16K: u64 = 2;
    const ARM64_IMAGE_FLAG_PAGE_SIZE_64K: u64 = 3;

    const ARM64_IMAGE_FLAG_PHYS_BASE: u64 = 1;

    // 1 bit
    const ARM64_IMAGE_FLAG_BE_SHIFT: u64 = 0;
    // 2 bit
    const ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT: u64 = 1;
    // 1 bit
    const ARM64_IMAGE_FLAG_PHYS_BASE_SHIFT: u64 = 3;

    const _HEAD_FLAG_PAGE_SIZE: u64 = (PAGE_SHIFT as u64 - 10) / 2;
    const _HEAD_FLAG_PHYS_BASE: u64 = 1;

    cfg_if! {
        if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
            const _HEAD_FLAG_BE: u64 = Self::ARM64_IMAGE_FLAG_BE;
        } else {
            const _HEAD_FLAG_BE: u64 = Self::ARM64_IMAGE_FLAG_LE;
        }
    }

    const fn head_flag(field: u64, shift: u64) -> u64 {
        field << shift
    }

    const fn head_flags() -> u64 {
        Self::head_flag(Self::_HEAD_FLAG_BE, Self::ARM64_IMAGE_FLAG_BE_SHIFT) |
        Self::head_flag(Self::_HEAD_FLAG_PAGE_SIZE, Self::ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT) |
        Self::head_flag(Self::_HEAD_FLAG_PHYS_BASE, Self::ARM64_IMAGE_FLAG_PHYS_BASE_SHIFT)
    }
}

cfg_if! {
    if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
        cfg_if! {
        if #[cfg(CONFIG_PAGE_SIZE_4KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1011),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1011);
        } else if #[cfg(CONFIG_PAGE_SIZE_16KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1101),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1101);
        } else if #[cfg(CONFIG_PAGE_SIZE_64KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1111),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1111);
        } else {
            compile_error!("Unsupported page size");
        }
        }
    } else {
        cfg_if! {
        if #[cfg(CONFIG_PAGE_SIZE_4KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1010),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1010);
        } else if #[cfg(CONFIG_PAGE_SIZE_16KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1100),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1100);
        } else if #[cfg(CONFIG_PAGE_SIZE_64KB)] {
                const HEAD_SYMBOLS: &str = concatcp!{
                    define_image_le64_macro!("_kernel_size_le", "_end - _text"),
                    define_image_le64_macro!("_kernel_flags_le", 0b1110),
                };
                const_assert_eq!(HeadFlags::head_flags(), 0b1110);
        } else {
            compile_error!("Unsupported page size");
        }
        }
    }
}

#[need_export]
#[allow(missing_docs)]
pub static EXPORT_HEAD_SYMBOLS: [u8; HEAD_SYMBOLS.len()+1] = const_str_to_u8_array_with_null!(HEAD_SYMBOLS);


