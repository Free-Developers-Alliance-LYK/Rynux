//! Rynux arm64 image header

use klib::{cfg_if, const_str_to_u8_array_with_null} ;
use const_format::concatcp;

cfg_if! {
    if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
        const fn data_le32(data: u64) -> u32 {
            ((data & 0x000000ff) << 24) |
            ((data & 0x0000ff00) << 8)  |
            ((data & 0x00ff0000) >> 8)  |
            ((data & 0xff000000) >> 24)
        }
    } else {
        #[allow(dead_code)]
        const fn data_le32(data: u64) -> u32 {
            (data & 0xffffffff) as u32
        }
    }
}

const KERNEL_SIZE_LE: &str = concatcp!{
    "_kernel_size_le_hi32 = ((_end - _text) >> 32) & 0xffffffff; \n",
    "_kernel_size_le_lo32 = ((_end - _text) & 0xffffffff) & 0xffffffff; \n",
};

const HEAD_SYMBOLS: &str = concatcp!{
    KERNEL_SIZE_LE,
};

#[no_mangle]
#[allow(missing_docs)]
pub static EXPORT_HEAD_SYMBOLS: [u8; HEAD_SYMBOLS.len()+1] = const_str_to_u8_array_with_null!(HEAD_SYMBOLS);
