//! Rynux arm64 boot head

use core::arch::naked_asm;

#[allow(missing_docs)]
#[naked]
#[no_mangle]
#[link_section = ".head.text"]
pub unsafe extern "C" fn _head() -> ! {
    unsafe {
    // DO NOT MODIFY. Image header expected by Linux boot-loaders.
        naked_asm!(
            "nop",                         // 特殊 NOP
            "b    primary_entry",          // 跳转到主入口
            ".quad 0",                     // 加载偏移
        );
    }
}

