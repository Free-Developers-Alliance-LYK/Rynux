

/// DO NOT MODIFY. Image header expected by Linux boot-loaders.
#[unsafe(naked)]
#[unsafe(link_section = ".head.text")]
unsafe extern "C" fn _head() -> ! {
    core::arch::naked_asm!("
	nop			            // special NOP to identity as PE/COFF executable
	b	primary_entry		// branch to kernel start, magic
	.quad	0				// Image load offset from start of RAM, little-endian
    ",)
}
