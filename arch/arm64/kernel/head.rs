//! Rynux arm64 boot head

use kernel::arch::arm64::{
    sysreg,
};

use kernel::{cpu_le, cpu_be};

core::arch::global_asm!(r#"
    .section .head.text, "ax"
    .global _head
_head:
    b primary_entry
    .quad 0
    .long _kernel_size_le_lo32
    .long _kernel_size_le_hi32
    .long _kernel_flags_le_lo32
    .long _kernel_flags_le_hi32
    .quad 0
    .quad 0
    .quad 0
    .ascii "ARM\x64"
    .long 0
    .section .idmap.text, "ax"
"#);


#[allow(missing_docs)]
#[no_mangle]
#[naked]
#[link_section = ".text"] // prevent to link as .text.primary_entry
pub unsafe extern "C" fn primary_entry() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "bl record_mmu_state",
            "mov x0, 0x09000000",
            "mov w1, #65",
            "str w1, [x0]"
        );
    }
}

/*
cfg_if! {
    if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
        macro_rules! ee_test_instr {
            () => {
                "tbz x19, {shift}, 1f"
            };
        }
    } else {
        macro_rules! ee_test_instr {
            () => {
                "tbnz x19, {sctlr_elx_ee_shift}, 1f"
            }
        }
    }
}
*/
#[allow(missing_docs)]
#[no_mangle]
#[naked]
#[link_section = ".init.text"]
pub unsafe extern "C" fn record_mmu_state() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mrs x19, CurrentEL",
            "cmp x19, {CurrentEL_EL2}",
            "mrs x19, sctlr_el1",
            "b.ne 0f",
            "mrs x19, sctlr_el2",
            "0:",
            cpu_le!("tbnz x19, {sctlr_elx_ee_shift}, 1f"),
            cpu_be!("tbz x19, {sctlr_elx_ee_shift}, 1f"),
            "tst x19, {sctlr_elx_c}",
            "and x19, x19, {sctlr_elx_m}",
            "csel x19, xzr, x19, eq",
            "ret",
            "1:",
            sctlr_elx_ee_shift =  const sysreg::SctlrElx::EE_SHIFT,
            CurrentEL_EL2 = const sysreg::CurrentEL::EL2,
        );
    }
}

/*
use kernel::sym_code_start;
sym_code_start!(primary_entry,
    "mov x0, 0x09000000; mov w1, #65; str w1, [x0]");
*/
