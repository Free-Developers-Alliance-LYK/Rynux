//! Constants for common sizes in bytes.
use crate::macros::need_export;

macro_rules! sz_export {
    ($name:ident, $val:expr) => {
        /// Constant for the size of `$name` in bytes.
        #[need_export]
        pub static $name: usize = $val;
    };
}

macro_rules! sz {
    ($name:ident, $val:expr) => {
        /// Constant for the size of `$name` in bytes.
        pub const $name: usize = $val;
    };
}

sz!(SZ_1,    0x0000_0001);
sz!(SZ_2,    0x0000_0002);
sz!(SZ_4,    0x0000_0004);
sz!(SZ_8,    0x0000_0008);
sz!(SZ_16,   0x0000_0010);
sz!(SZ_32,   0x0000_0020);
sz!(SZ_64,   0x0000_0040);
sz!(SZ_128,  0x0000_0080);
sz!(SZ_256,  0x0000_0100);
sz!(SZ_512,  0x0000_0200);

sz!(SZ_1K,  0x0000_0400);
sz!(SZ_2K,  0x0000_0800);
sz_export!(EXPORT_SZ_2K,  0x0000_0800);
sz!(SZ_4K,  0x0000_1000);
sz_export!(EXPORT_SZ_4K,  0x0000_1000);
sz!(SZ_8K,  0x0000_2000);
sz!(SZ_16K, 0x0000_4000);
sz!(SZ_32K, 0x0000_8000);
sz!(SZ_64K, 0x0001_0000);
sz!(SZ_128K, 0x0002_0000);
sz!(SZ_256K, 0x0004_0000);
sz!(SZ_512K, 0x0008_0000);

sz!(SZ_1M,  0x0010_0000);
sz!(SZ_2M,  0x0020_0000);
sz!(SZ_4M,  0x0040_0000);
sz!(SZ_8M,  0x0080_0000);
sz!(SZ_16M, 0x0100_0000);
sz!(SZ_32M, 0x0200_0000);
sz!(SZ_64M, 0x0400_0000);
sz!(SZ_128M, 0x0800_0000);
sz!(SZ_256M, 0x1000_0000);
sz!(SZ_512M, 0x2000_0000);


sz!(SZ_1G,  0x4000_0000);
sz!(SZ_2G,  0x8000_0000);
sz!(SZ_4G,  0x0000_0001_0000_0000);
sz!(SZ_8G,  0x0000_0002_0000_0000);
sz!(SZ_16G, 0x0000_0004_0000_0000);
sz!(SZ_32G, 0x0000_0008_0000_0000);
sz!(SZ_64G, 0x0000_0010_0000_0000);
sz!(SZ_128G, 0x0000_0020_0000_0000);
sz!(SZ_256G, 0x0000_0040_0000_0000);
sz!(SZ_512G, 0x0000_0080_0000_0000);

sz!(SZ_1T,  0x0000_0100_0000_0000);
sz!(SZ_2T,  0x0000_0200_0000_0000);
sz!(SZ_4T,  0x0000_0400_0000_0000);
sz!(SZ_8T,  0x0000_0800_0000_0000);
sz!(SZ_16T, 0x0000_1000_0000_0000);
sz!(SZ_32T, 0x0000_2000_0000_0000);
sz!(SZ_64T, 0x0000_4000_0000_0000);
