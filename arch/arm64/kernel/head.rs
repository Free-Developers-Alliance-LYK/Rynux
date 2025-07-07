//! Rynux arm64 boot head

use kernel::arch::arm64::{
    sysreg,
};

use kernel::{cpu_le, cpu_be, adr_l, str_l};

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


/*
 * The following callee saved general purpose registers are used on the
 *primary lowlevel boot path:
 *
 * Register   Scope                      Purpose
 * x19        primary_entry() .. start_kernel()        whether we entered with the MMU on
 * x20        primary_entry() .. __primary_switch()    CPU boot mode
 * x21        primary_entry() .. start_kernel()        FDT pointer passed at boot in x0
*/
#[no_mangle]
#[naked]
#[link_section = ".text"] // prevent to link as .text.primary_entry
unsafe extern "C" fn primary_entry() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "bl record_mmu_state",
            "bl preserve_boot_args",

            // init stack
            "adrp x1, {early_init_stack}",
            "mov sp, x1",
            "mov x29, xzr",
            "adrp x0, {init_idmap_pg_dir}",
            "mov x1, xzr",
            "bl {__pi_create_init_idmap}",

            "mov x0, 0x09000000",
            "mov w1, #65",
            "str w1, [x0]",
            early_init_stack = sym early_init_stack,
            init_idmap_pg_dir = sym init_idmap_pg_dir,
            __pi_create_init_idmap = sym __pi_create_init_idmap,
        );
    }
}

#[no_mangle]
#[naked]
#[link_section = ".init.text"]
unsafe extern "C" fn record_mmu_state() -> ! {
    unsafe {
    // Record the mmu state in x19
        core::arch::naked_asm!(
            "mrs x19, CurrentEL",
            "cmp x19, {CurrentEL_EL2}",
            "mrs x19, sctlr_el1",
            "b.ne 0f",
            "mrs x19, sctlr_el2",
            "0:",
            cpu_le!("tbnz x19, {sctlr_elx_ee_shift}, 1f"),
            cpu_be!("tbz x19, {sctlr_elx_ee_shift}, 1f"),
            "tst x19, {sctlr_elx_c}", // Z := (C == 0)
            "and x19, x19, {sctlr_elx_m}", // isolate M bit
            "csel x19, xzr, x19, eq", // clear x19 if Z
            "ret",
            "1:", //TODO: now we do nothing if EE is not match
            sctlr_elx_ee_shift =  const sysreg::SctlrElx::EE_SHIFT,
            CurrentEL_EL2 = const sysreg::CurrentEL::EL2,
            sctlr_elx_c = const sysreg::SctlrElx::C,
            sctlr_elx_m = const sysreg::SctlrElx::M,
        );
    }
}

#[no_mangle]
#[naked]
#[link_section = ".init.text"]
unsafe extern "C" fn preserve_boot_args() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "mov x21, x0", // x21=FDT
            adr_l!("x0", "{boot_args}"),
            "stp x21, x1, [x0]", // x0 .. x3 at kernel entry
            "stp x2, x3, [x0, #16]",
            "cbnz x19, 0f", // skip cache invalidation if MMU is on
            "dmb sy", // needed before dc ivac with MMU off
            "add x1, x0, #0x20", // 4 x 8 bytes
            "b {dcache_inval_poc}",
            "0:",
            str_l!("x19", "{mmu_enabled_at_boot}", "x0"),
            "ret",
            boot_args = sym kernel::arch::arm64::kernel::setup::BOOT_ARGS,
            mmu_enabled_at_boot = sym kernel::arch::arm64::kernel::setup::MMU_ENABLED_AT_BOOT,
            dcache_inval_poc = sym kernel::arch::arm64::mm::cache::dcache_inval_poc,
        );
    }
}

// Define in vmrynux.lds.S
extern "C" {
    static early_init_stack: u8;
    static init_idmap_pg_dir: u8;
    static __pi_create_init_idmap: u8;
}
