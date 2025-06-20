//! Rynux arm64 boot head

use core::arch::naked_asm;

#[unsafe(naked)]
#[no_mangle]
/// DO NOT MODIFY. Image header expected by Linux boot-loaders.
pub unsafe extern "C" fn _head() -> ! {
        naked_asm!(
            "nop",                         // 特殊 NOP
            "b    primary_entry",          // 跳转到主入口
            ".quad 0",                     // 加载偏移
        );
}

