//! Rynux arm64 boot head


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

#[allow(missing_docs)]
#[no_mangle]
#[naked]
#[link_section = ".init.text"]
pub unsafe extern "C" fn record_mmu_state() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mrs x19, CurrentEL",
            "cmp x19, #CurrentEL_EL2",
            "mrs x19, sctlr_el1",
            "b.ne 0f",
            "mrs x19, sctlr_el2",
            "0:",
            "tbnz x19, #SCTLR_ELx_EE_SHIFT, 1f",
            "tbz x19, #SCTLR_ELx_EE_SHIFT, 1f",
            "tst x19, #SCTLR_ELx_C",
            "and x19, x19, #SCTLR_ELx_M",
            "csel x19, xzr, x19, eq",
            "ret",
            "1:",
            "eor x19, x19, #SCTLR_ELx_EE",
            "bic x19, x19, #SCTLR_ELx_M",
            "b.ne 2f",
            "pre_disable_mmu_workaround",
            "msr sctlr_el2, x19",
            "b 3f",
            "2:",
            "pre_disable_mmu_workaround",
            "msr sctlr_el1, x19",
            "3:",
            "isb",
        );
    }
}

/*
use kernel::sym_code_start;
sym_code_start!(primary_entry,
    "mov x0, 0x09000000; mov w1, #65; str w1, [x0]");
*/
