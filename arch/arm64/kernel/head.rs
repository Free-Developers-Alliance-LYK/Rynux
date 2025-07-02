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
            "mov x0, 0x09000000 ",
            "mov w1, #65",
            "str w1, [x0]",
            //"b    primary_entry",          // 跳转到主入口
            ".quad 0",                     // 加载偏移
        );
    }
}

