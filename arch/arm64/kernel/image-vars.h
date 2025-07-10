/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Linker script variables to be set after section resolution, as
 * ld.lld does not like variables assigned before SECTIONS is processed.
 */
#ifndef __ARM64_KERNEL_IMAGE_VARS_H
#define __ARM64_KERNEL_IMAGE_VARS_H

#ifndef LINKER_SCRIPT
#error This file should only be included in vmlinux.lds.S
#endif


PROVIDE(__pi_init_idmap_pg_dir		= init_idmap_pg_dir);
PROVIDE(__pi_init_idmap_pg_end		= init_idmap_pg_end);
PROVIDE(__pi_init_pg_dir		= init_pg_dir);
PROVIDE(__pi_init_pg_end		= init_pg_end);
PROVIDE(__pi_swapper_pg_dir		= swapper_pg_dir);

PROVIDE(__pi__text			= _text);
PROVIDE(__pi__stext               	= _stext);
PROVIDE(__pi__etext               	= _etext);
PROVIDE(__pi___start_rodata       	= __start_rodata);
PROVIDE(__pi___inittext_begin     	= __inittext_begin);
PROVIDE(__pi___inittext_end       	= __inittext_end);
PROVIDE(__pi___initdata_begin     	= __initdata_begin);
PROVIDE(__pi___initdata_end       	= __initdata_end);
PROVIDE(__pi__data                	= _data);
PROVIDE(__pi___bss_start		= __bss_start);
PROVIDE(__pi__end			= _end);

#endif /* __ARM64_KERNEL_IMAGE_VARS_H */
