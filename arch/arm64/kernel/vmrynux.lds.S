#define SBSS_ALIGN			0
#include <layout.h>

OUTPUT_ARCH(aarch64)
ENTRY(_text)

#define IDMAP_TEXT					\
	. = ALIGN(SZ_4K);				\
	__idmap_text_start = .;				\
	*(.idmap.text)					\
	__idmap_text_end = .;

/*
 * The size of the PE/COFF section that covers the kernel image, which
 * runs from _stext to _edata, must be a round multiple of the PE/COFF
 * FileAlignment, which we set to its minimum value of 0x200. '_stext'
 * itself is 4 KB aligned, so padding out _edata to a 0x200 aligned
 * boundary should be sufficient.
 */
PECOFF_FILE_ALIGNMENT = 0x200;
#define PECOFF_EDATA_PADDING    \
      .pecoff_edata_padding : { BYTE(0); . = ALIGN(PECOFF_FILE_ALIGNMENT); }


SECTIONS
{
	/*
	 * XXX: The linker does not define how output sections are
	 * assigned to input sections when there are multiple statements
	 * matching the same input section name.  There is no documented
	 * order of matching.
	 */
	DISCARDS
	/DISCARD/ : {
		*(.interp .dynamic)
		*(.dynsym .dynstr .hash .gnu.hash)
		 *(.ARM.attributes)
	}

	. = KIMAGE_VADDR;

	.head.text : {
		_text = .;
		HEAD_TEXT
	}
	.text : ALIGN(SEGMENT_ALIGN) {	/* Real text segment		*/
		_stext = .;		/* Text and read-only data	*/
			IRQENTRY_TEXT
			SOFTIRQENTRY_TEXT
			ENTRY_TEXT
			TEXT_TEXT
			SCHED_TEXT
			LOCK_TEXT
			KPROBES_TEXT
			*(.gnu.warning)
	}

	. = ALIGN(SEGMENT_ALIGN);
	_etext = .;			/* End of text section */

	/* everything from this point to __init_begin will be marked RO NX */
	RO_DATA

	.got : { *(.got) }
	/*
	 * Make sure that the .got.plt is either completely empty or it
	 * contains only the lazy dispatch entries.
	 */
	.got.plt : { *(.got.plt) }
	ASSERT(SIZEOF(.got.plt) == 0 || SIZEOF(.got.plt) == 0x18,
	       "Unexpected GOT/PLT entries detected!")

	/* code sections that are never executed via the kernel mapping */
	.rodata.text : {
		IDMAP_TEXT
		. = ALIGN(PAGE_SIZE);
	}

	idmap_pg_dir = .;
	. += PAGE_SIZE;

	reserved_pg_dir = .;
	. += PAGE_SIZE;

	swapper_pg_dir = .;
	. += PAGE_SIZE;

	. = ALIGN(SEGMENT_ALIGN);
	__init_begin = .;
	__inittext_begin = .;

	INIT_TEXT_SECTION

	__exittext_begin = .;
	.exit.text : {
		EXIT_TEXT
	}
	__exittext_end = .;

	. = ALIGN(4);
	.altinstructions : {
		__alt_instructions = .;
		*(.altinstructions)
		__alt_instructions_end = .;
	}

	. = ALIGN(SEGMENT_ALIGN);
	__inittext_end = .;
	__initdata_begin = .;

	init_idmap_pg_dir = .;
	. += INIT_IDMAP_DIR_SIZE;
	init_idmap_pg_end = .;

	.init.data : {
		INIT_DATA
		INIT_SETUP
		INIT_CALLS
		CON_INITCALL
		INIT_RAM_FS
		*(.init.altinstructions .init.bss)	/* from the EFI stub */
	}
	.exit.data : {
		EXIT_DATA
	}

	PERCPU_SECTION

	.rela.dyn : ALIGN(8) {
		__pi_rela_start = .;
		*(.rela .rela*)
		__pi_rela_end = .;
	}

	.relr.dyn : ALIGN(8) {
		__pi_relr_start = .;
		*(.relr.dyn)
		__pi_relr_end = .;
	}

	. = ALIGN(SEGMENT_ALIGN);
	__initdata_end = .;
	__init_end = .;

	.data.rel.ro : { *(.data.rel.ro) }
    ASSERT(SIZEOF(.data.rel.ro) == 0, "Unexpected RELRO detected!")

	_data = .;
	_sdata = .;
	RW_DATA

	/*
	 * Data written with the MMU off but read with the MMU on requires
	 * cache lines to be invalidated, discarding up to a Cache Writeback
	 * Granule (CWG) of data from the cache. Keep the section that
	 * requires this type of maintenance to be in its own Cache Writeback
	 * Granule (CWG) area so the cache maintenance operations don't
	 * interfere with adjacent data.
	 */
	.mmuoff.data.write : ALIGN(SZ_2K) {
		__mmuoff_data_start = .;
		*(.mmuoff.data.write)
	}
	. = ALIGN(SZ_2K);
	.mmuoff.data.read : {
		*(.mmuoff.data.read)
		__mmuoff_data_end = .;
	}

	PECOFF_EDATA_PADDING
	__pecoff_data_rawsize = ABSOLUTE(. - __initdata_begin);
	_edata = .;

	BSS_SECTION

	. = ALIGN(PAGE_SIZE);
	init_pg_dir = .;
	. += INIT_DIR_SIZE;
	init_pg_end = .;

	. = ALIGN(SEGMENT_ALIGN);
	__pecoff_data_size = ABSOLUTE(. - __initdata_begin);
	_end = .;

	STABS_DEBUG
	DWARF_DEBUG
	ELF_DETAILS

	HEAD_SYMBOLS

	/*
	 * Sections that should stay zero sized, which is safer to
	 * explicitly check instead of blindly discarding.
	 */
	.plt : {
		*(.plt) *(.plt.*) *(.iplt) *(.igot .igot.plt)
	}
	ASSERT(SIZEOF(.plt) == 0, "Unexpected run-time procedure linkages detected!")
}

ASSERT(__idmap_text_end - (__idmap_text_start & ~(SZ_4K - 1)) <= SZ_4K,
	"ID map text too big or misaligned")
/*
 * If padding is applied before .head.text, virt<->phys conversions will fail.
 */
ASSERT(_text == KIMAGE_VADDR, "HEAD is misaligned")
