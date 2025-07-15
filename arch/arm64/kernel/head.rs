//! Rynux arm64 boot head

use kernel::arch::arm64::{
    kernel::image::symbols::*,
    sysregs::*,
};

use kernel::{cpu_le, cpu_be, adr_l, str_l};
use kernel::macros::{section_idmap_text, section_init_text};

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
#[unsafe(naked)]
#[section_idmap_text]
unsafe extern "C" fn primary_entry() -> ! {
        core::arch::naked_asm!(
            "bl record_mmu_state",
            "bl preserve_boot_args",

            // init stack
            "adrp x1, {early_init_stack}",
            "mov sp, x1",
            "mov x29, xzr",

            // create init idmap
            "adrp x0, {init_idmap_pg_dir}",
            "mov x1, xzr",
            "bl {__pi_create_init_idmap}",
            
            /*
             * If the page tables have been populated with non-cacheable
             * accesses (MMU disabled), invalidate those tables again to
             * remove any speculatively loaded cache lineso.
             */
            "cbnz x19, 0f",
            "dmb     sy",
            "mov x1, x0", // end of used region

            "adrp    x0, {init_idmap_pg_dir}",
            adr_l!("x2", "{dcache_inval_poc}"),
            "blr x2",
            "b 1f",

            /*
             * If we entered with the MMU and caches on, clean the ID mapped part
             * of the primary boot code to the PoC so we can safely execute it with
             * the MMU off.
             */
            "0:",
            "adrp x0, {__idmap_text_start}",
            adr_l!("x1", "{__idmap_text_end}"),
            adr_l!("x2", "{dcache_clean_poc}"),
            "blr x2",

            "1:",
            "mov x0, x19",
            "bl {init_kernel_el}",
            "mov x20, x0", // X20 = boot mode

            // The following calls CPU setup code, see arch/arm64/mm/proc.S for
            // details.
            // On return, the CPU will be ready for the MMU to be turned on and
            // the TCR will have been set.
            "bl {__cpu_setup}", // initialise processor
            "b {__primary_switch}",
            early_init_stack = sym early_init_stack,
            init_idmap_pg_dir = sym init_idmap_pg_dir,
            __pi_create_init_idmap = sym __pi_create_init_idmap,
            dcache_inval_poc = sym kernel::arch::arm64::mm::cache::dcache_inval_poc,
            __idmap_text_start = sym __idmap_text_start,
            __idmap_text_end = sym __idmap_text_end,
            dcache_clean_poc = sym kernel::arch::arm64::mm::cache::dcache_clean_poc,
            init_kernel_el = sym init_kernel_el,
            __cpu_setup = sym __cpu_setup,
            __primary_switch = sym __primary_switch,
        );
}

#[no_mangle]
#[unsafe(naked)]
#[section_init_text]
unsafe extern "C" fn record_mmu_state() -> ! {
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
            CurrentEL_EL2 = const CurrentEL::EL2.bits(),
            sctlr_elx_ee_shift =  const SctlrEl1::ELX_EE_SHIFT,
            sctlr_elx_c = const SctlrEl1::ELX_C.bits(),
            sctlr_elx_m = const SctlrEl1::ELX_M.bits(),
        );
}

#[no_mangle]
#[unsafe(naked)]
#[section_init_text]
unsafe extern "C" fn preserve_boot_args() -> ! {
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

/*
 * Starting from EL2 or EL1, configure the CPU to execute at the highest
 * reachable EL supported by the kernel in a chosen default state. If dropping
 * from EL2 to EL1, configure EL2 before configuring EL1.
 *
 * Since we cannot always rely on ERET synchronizing writes to sysregs (e.g. if
 * SCTLR_ELx.EOS is clear), we place an ISB prior to ERET.
 *
 * Returns either BOOT_CPU_MODE_EL1 or BOOT_CPU_MODE_EL2 in x0 if
 * booted in EL1 or EL2 respectively, with the top 32 bits containing
 * potential context flags. These flags are *not* stored in __boot_cpu_mode.
 *
 * x0: whether we are being called from the primary boot path with the MMU on
 */
#[section_idmap_text]
unsafe extern "C" fn init_kernel_el() {
    use kernel::arch::arm64::early_debug::early_uart_putchar;
    use kernel::arch::arm64::asm::barrier::isb;
    use kernel::arch::arm64::asm::eret;
    use kernel::arch::arm64::kernel::setup::BOOT_CPU_MODE_EL1;
    use kernel::write_gpr;
    let current_el = CurrentEL::read();
    if current_el.contains(CurrentEL::EL2) {
        early_uart_putchar('2' as u8);
        // TODO: init el2
    } else if current_el.contains(CurrentEL::EL1) {
        // init el1
        early_uart_putchar('M' as u8);
        isb();
        SctlrEl1::INIT_SCTLR_EL1_MMU_OFF.write();
        isb();
        SpsrEl1::INIT_PSTATE_EL1.write();
        ElrEl1::write_raw(Lr::read_raw());
        write_gpr!(x0, BOOT_CPU_MODE_EL1);
        eret();
    }
}


/*
 * __cpu_setup
 *
 * Initialise the processor for turning the MMU on.
 *
 * Output:
 *  Return in x0 the value of the SCTLR_EL1 register.
 */
#[section_idmap_text]
unsafe extern "C" fn __cpu_setup() {
    use kernel::arch::arm64::asm::tlb::local_flush_tlb_all;
    local_flush_tlb_all();
    // Reset cpacr_el1
    CpacrEl1::write_raw(0);
    // Reset mdscr_el1 and disable access to the DCC from EL0
    MdscrEl1::TDCC.write(); 
    PmuserenrEl0::reset();
    AmuserenrEl0::reset();
}

#[section_init_text]
unsafe extern "C" fn __primary_switch() {
}
