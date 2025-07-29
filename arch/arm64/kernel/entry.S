/* SPDX-License-Identifier: GPL-2.0-only */
// TODO: 
// - not support ARM64_MTE
// - not supoort CONFIG_KASAN_HW_TAGS
// - not support CONFIG_ARM64_PTR_AUTH
// - not support Spectre V4 SSBD
// - not support CONFIG_ARM64_SW_TTBR0_PAN
// - not support CONFIG_UNMAP_KERNEL_AT_EL0
// - not support CONFIG_ARM_SDE_INTERFACE
// - not support CONFIG_SHADOW_CALL_STACK
// - not supoort CONFIG_ARM64_PSEUDO_NMI
// - not supoort CONFIG_PID_IN_CONTEXTIDR
// - not supoort OVERFLOW_STACK CHECK
// - not support CONFIG_QCOM_FALKOR_ERRATUM_1003

#include <layout.h>

	.macro	clear_gp_regs
	.irp	n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29
	mov	x\n, xzr
	.endr
	.endm

	.macro kernel_ventry, el:req, ht:req, regsize:req, label:req
	.align 7
.Lventry_start\@:
	.if	\el == 0
	/*
	 * This must be the first instruction of the EL0 vector entries. It is
	 * skipped by the trampoline vectors, to trigger the cleanup.
	 */
	b	.Lskip_tramp_vectors_cleanup\@
	.if	\regsize == 64
	mrs	x30, tpidrro_el0
	msr	tpidrro_el0, xzr
	.else
	mov	x30, xzr
	.endif
.Lskip_tramp_vectors_cleanup\@:
	.endif
	sub	sp, sp, #PT_REGS_SIZE

	b	el\el\ht\()_\regsize\()_\label

.org .Lventry_start\@ + 128	// Did we overflow the ventry slot?
	.endm

	.macro	tramp_alias, dst, sym
	.set	.Lalias\@, TRAMP_VALIAS + \sym - .entry.tramp.text
	movz	\dst, :abs_g2_s:.Lalias\@
	movk	\dst, :abs_g1_nc:.Lalias\@
	movk	\dst, :abs_g0_nc:.Lalias\@
	.endm

	.macro	kernel_entry, el, regsize = 64
	.if	\el == 0
	alternative_insn nop, SET_PSTATE_DIT(1), ARM64_HAS_DIT
	.endif
	.if	\regsize == 32
	mov	w0, w0				// zero upper 32 bits of x0
	.endif
	stp	x0, x1, [sp, #16 * 0]
	stp	x2, x3, [sp, #16 * 1]
	stp	x4, x5, [sp, #16 * 2]
	stp	x6, x7, [sp, #16 * 3]
	stp	x8, x9, [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]

	.if	\el == 0
	clear_gp_regs
	mrs	x21, sp_el0
	ldr_this_cpu	tsk, __entry_task, x20
	msr	sp_el0, tsk

	/*
	 * Ensure MDSCR_EL1.SS is clear, since we can unmask debug exceptions
	 * when scheduling.
	 */
	ldr	x19, [tsk, #TSK_TI_FLAGS]
	disable_step_tsk x19, x20

	/* Check for asynchronous tag check faults in user space */
	ldr	x0, [tsk, THREAD_SCTLR_USER]

	/*
	 * Any non-self-synchronizing system register updates required for
	 * kernel entry should be placed before this point.
	 */
1:
	scs_load_current
	.else
	add	x21, sp, #PT_REGS_SIZE
	get_current_task tsk
	.endif /* \el == 0 */
	mrs	x22, elr_el1
	mrs	x23, spsr_el1
	stp	lr, x21, [sp, #S_LR]

	/*
	 * Create a metadata frame record. The unwinder will use this to
	 * identify and unwind exception boundaries.
	 */
	stp	xzr, xzr, [sp, #S_STACKFRAME]
	.if \el == 0
	mov	x0, #FRAME_META_TYPE_FINAL
	.else
	mov	x0, #FRAME_META_TYPE_PT_REGS
	.endif
	str	x0, [sp, #S_STACKFRAME_TYPE]
	add	x29, sp, #S_STACKFRAME

	stp	x22, x23, [sp, #S_PC]

	/* Not in a syscall by default (el0_svc overwrites for real syscall) */
	.if	\el == 0
	mov	w21, #NO_SYSCALL
	str	w21, [sp, #S_SYSCALLNO]
	.endif

	/*
	 * Registers that may be useful after this macro is invoked:
	 *
	 * x20 - ICC_PMR_EL1
	 * x21 - aborted SP
	 * x22 - aborted PC
	 * x23 - aborted PSTATE
	*/
	.endm

	.macro	kernel_exit, el
	.if	\el != 0
	disable_daif
	.endif

	ldp	x21, x22, [sp, #S_PC]		// load ELR, SPSR

	.if	\el == 0
	ldr	x23, [sp, #S_SP]		// load return stack pointer
	msr	sp_el0, x23
	tst	x22, #PSR_MODE32_BIT		// native task?
	b.eq	3f

#ifdef CONFIG_ARM64_ERRATUM_845719
alternative_if ARM64_WORKAROUND_845719
	msr contextidr_el1, xzr
alternative_else_nop_endif
#endif
3:
	scs_save tsk

	/* Ignore asynchronous tag check faults in the uaccess routines */
	ldr	x0, [tsk, THREAD_SCTLR_USER]

	.endif

	msr	elr_el1, x21			// set up the return data
	msr	spsr_el1, x22
	ldp	x0, x1, [sp, #16 * 0]
	ldp	x2, x3, [sp, #16 * 1]
	ldp	x4, x5, [sp, #16 * 2]
	ldp	x6, x7, [sp, #16 * 3]
	ldp	x8, x9, [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]

	ldr	lr, [sp, #S_LR]
	add	sp, sp, #PT_REGS_SIZE		// restore sp

	.if \el == 0
	/* This must be after the last explicit memory access */
alternative_if ARM64_WORKAROUND_SPECULATIVE_UNPRIV_LOAD
	tlbi	vale1, xzr
	dsb	nsh
alternative_else_nop_endif
	.else
	/* Ensure any device/NC reads complete */
	alternative_insn nop, "dmb sy", ARM64_WORKAROUND_1508412
	.endif

	eret
	sb
	.endm

/* GPRs used by entry code */
tsk	.req	x28		// current thread_info

	.text

/*
 * Exception vectors.
 */
	.pushsection ".entry.text", "ax"

	.align	11
SYM_CODE_START(vectors)
	kernel_ventry	1, t, 64, sync		// Synchronous EL1t
	kernel_ventry	1, t, 64, irq		// IRQ EL1t
	kernel_ventry	1, t, 64, fiq		// FIQ EL1t
	kernel_ventry	1, t, 64, error		// Error EL1t

	kernel_ventry	1, h, 64, sync		// Synchronous EL1h
	kernel_ventry	1, h, 64, irq		// IRQ EL1h
	kernel_ventry	1, h, 64, fiq		// FIQ EL1h
	kernel_ventry	1, h, 64, error		// Error EL1h

	kernel_ventry	0, t, 64, sync		// Synchronous 64-bit EL0
	kernel_ventry	0, t, 64, irq		// IRQ 64-bit EL0
	kernel_ventry	0, t, 64, fiq		// FIQ 64-bit EL0
	kernel_ventry	0, t, 64, error		// Error 64-bit EL0

	kernel_ventry	0, t, 32, sync		// Synchronous 32-bit EL0
	kernel_ventry	0, t, 32, irq		// IRQ 32-bit EL0
	kernel_ventry	0, t, 32, fiq		// FIQ 32-bit EL0
	kernel_ventry	0, t, 32, error		// Error 32-bit EL0
SYM_CODE_END(vectors)

	.macro entry_handler el:req, ht:req, regsize:req, label:req
SYM_CODE_START_LOCAL(el\el\ht\()_\regsize\()_\label)
	kernel_entry \el, \regsize
	mov	x0, sp
	bl	el\el\ht\()_\regsize\()_\label\()_handler
	.if \el == 0
	b	ret_to_user
	.else
	b	ret_to_kernel
	.endif
SYM_CODE_END(el\el\ht\()_\regsize\()_\label)
	.endm

/*
 * Early exception handlers
 */
	entry_handler	1, t, 64, sync
	entry_handler	1, t, 64, irq
	entry_handler	1, t, 64, fiq
	entry_handler	1, t, 64, error

	entry_handler	1, h, 64, sync
	entry_handler	1, h, 64, irq
	entry_handler	1, h, 64, fiq
	entry_handler	1, h, 64, error

	entry_handler	0, t, 64, sync
	entry_handler	0, t, 64, irq
	entry_handler	0, t, 64, fiq
	entry_handler	0, t, 64, error

	entry_handler	0, t, 32, sync
	entry_handler	0, t, 32, irq
	entry_handler	0, t, 32, fiq
	entry_handler	0, t, 32, error

SYM_CODE_START_LOCAL(ret_to_kernel)
	kernel_exit 1
SYM_CODE_END(ret_to_kernel)

SYM_CODE_START_LOCAL(ret_to_user)
	ldr	x19, [tsk, #TSK_TI_FLAGS]	// re-check for single-step
	enable_step_tsk x19, x2
	kernel_exit 0
SYM_CODE_END(ret_to_user)

	.popsection				// .entry.text

	// Move from tramp_pg_dir to swapper_pg_dir
	.macro tramp_map_kernel, tmp
	mrs	\tmp, ttbr1_el1
	add	\tmp, \tmp, #TRAMP_SWAPPER_OFFSET
	bic	\tmp, \tmp, #USER_ASID_FLAG
	msr	ttbr1_el1, \tmp
	.endm

	// Move from swapper_pg_dir to tramp_pg_dir
	.macro tramp_unmap_kernel, tmp
	mrs	\tmp, ttbr1_el1
	sub	\tmp, \tmp, #TRAMP_SWAPPER_OFFSET
	orr	\tmp, \tmp, #USER_ASID_FLAG
	msr	ttbr1_el1, \tmp
	/*
	 * We avoid running the post_ttbr_update_workaround here because
	 * it's only needed by Cavium ThunderX, which requires KPTI to be
	 * disabled.
	 */
	.endm

	.macro		tramp_data_read_var	dst, var
#ifdef CONFIG_RELOCATABLE
	ldr		\dst, .L__tramp_data_\var
	.ifndef		.L__tramp_data_\var
	.pushsection	".entry.tramp.rodata", "a", %progbits
	.align		3
.L__tramp_data_\var:
	.quad		\var
	.popsection
	.endif
#else
	/*
	 * As !RELOCATABLE implies !RANDOMIZE_BASE the address is always a
	 * compile time constant (and hence not secret and not worth hiding).
	 *
	 * As statically allocated kernel code and data always live in the top
	 * 47 bits of the address space we can sign-extend bit 47 and avoid an
	 * instruction to load the upper 16 bits (which must be 0xFFFF).
	 */
	movz		\dst, :abs_g2_s:\var
	movk		\dst, :abs_g1_nc:\var
	movk		\dst, :abs_g0_nc:\var
#endif
	.endm

#define BHB_MITIGATION_NONE	0
#define BHB_MITIGATION_LOOP	1
#define BHB_MITIGATION_FW	2
#define BHB_MITIGATION_INSN	3

	.macro tramp_ventry, vector_start, regsize, kpti, bhb
	.align	7
1:
	.if	\regsize == 64
	msr	tpidrro_el0, x30	// Restored in kernel_ventry
	.endif

	.if	\bhb == BHB_MITIGATION_LOOP
	/*
	 * This sequence must appear before the first indirect branch. i.e. the
	 * ret out of tramp_ventry. It appears here because x30 is free.
	 */
	__mitigate_spectre_bhb_loop	x30
	.endif // \bhb == BHB_MITIGATION_LOOP

	.if	\bhb == BHB_MITIGATION_INSN
	clearbhb
	isb
	.endif // \bhb == BHB_MITIGATION_INSN

	.if	\kpti == 1
	/*
	 * Defend against branch aliasing attacks by pushing a dummy
	 * entry onto the return stack and using a RET instruction to
	 * enter the full-fat kernel vectors.
	 */
	bl	2f
	b	.
2:
	tramp_map_kernel	x30
alternative_insn isb, nop, ARM64_WORKAROUND_QCOM_FALKOR_E1003
	tramp_data_read_var	x30, vectors
alternative_if_not ARM64_WORKAROUND_CAVIUM_TX2_219_PRFM
	prfm	plil1strm, [x30, #(1b - \vector_start)]
alternative_else_nop_endif

	msr	vbar_el1, x30
	isb
	.else
	adr_l	x30, vectors
	.endif // \kpti == 1

	.if	\bhb == BHB_MITIGATION_FW
	/*
	 * The firmware sequence must appear before the first indirect branch.
	 * i.e. the ret out of tramp_ventry. But it also needs the stack to be
	 * mapped to save/restore the registers the SMC clobbers.
	 */
	__mitigate_spectre_bhb_fw
	.endif // \bhb == BHB_MITIGATION_FW

	add	x30, x30, #(1b - \vector_start + 4)
	ret
.org 1b + 128	// Did we overflow the ventry slot?
	.endm

	.macro	generate_tramp_vector,	kpti, bhb
.Lvector_start\@:
	.space	0x400

	.rept	4
	tramp_ventry	.Lvector_start\@, 64, \kpti, \bhb
	.endr
	.rept	4
	tramp_ventry	.Lvector_start\@, 32, \kpti, \bhb
	.endr
	.endm

/*
 * Exception vectors for spectre mitigations on entry from EL1 when
 * kpti is not in use.
 */
	.macro generate_el1_vector, bhb
.Lvector_start\@:
	kernel_ventry	1, t, 64, sync		// Synchronous EL1t
	kernel_ventry	1, t, 64, irq		// IRQ EL1t
	kernel_ventry	1, t, 64, fiq		// FIQ EL1h
	kernel_ventry	1, t, 64, error		// Error EL1t

	kernel_ventry	1, h, 64, sync		// Synchronous EL1h
	kernel_ventry	1, h, 64, irq		// IRQ EL1h
	kernel_ventry	1, h, 64, fiq		// FIQ EL1h
	kernel_ventry	1, h, 64, error		// Error EL1h

	.rept	4
	tramp_ventry	.Lvector_start\@, 64, 0, \bhb
	.endr
	.rept 4
	tramp_ventry	.Lvector_start\@, 32, 0, \bhb
	.endr
	.endm

/* The order must match tramp_vecs and the arm64_bp_harden_el1_vectors enum. */
	.pushsection ".entry.text", "ax"
	.align	11
	.popsection


.section .text
.p2align 11
.global exception_vector_base
exception_vector_base:
    // current EL, with SP_EL0
    INVALID_EXCP 0 0
    INVALID_EXCP 1 0
    INVALID_EXCP 2 0
    INVALID_EXCP 3 0

    // current EL, with SP_ELx
    HANDLE_SYNC
    HANDLE_IRQ
    INVALID_EXCP 2 1
    INVALID_EXCP 3 1

    // lower EL, aarch64
    HANDLE_SYNC
    HANDLE_IRQ
    INVALID_EXCP 2 2
    INVALID_EXCP 3 2

    // lower EL, aarch32
    INVALID_EXCP 0 3
    INVALID_EXCP 1 3
    INVALID_EXCP 2 3




