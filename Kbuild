# SPDX-License-Identifier: GPL-2.0
#
# Kbuild for top-level directory of the kernel

proc_macros_prepare:
	$(Q)$(MAKE) $(build)=proc_macros prepare

# proc macros must built first, it is host lib
$(obj)/third_lib/built-in.a: proc_macros_prepare

$(obj)/klib.o: private rustc_target_flags = --extern alloc --extern macros
$(obj)/klib.o: $(src)/klib/lib.rs $(obj)/third_lib/built-in.a FORCE
	+$(call if_changed_rule,rustc_library)

$(obj)/kernel.o: private rustc_target_flags = --extern macros --extern klib --extern const_format --extern static_assertions
$(obj)/kernel.o: $(src)/kernel/lib.rs $(obj)/klib.o FORCE
	+$(call if_changed_rule,rustc_library)

quiet_cmd_exports = GEN $@
      cmd_exports = $(srctree)/scripts/generate_layout_header.py $< $@

targets += $(objtree)/layout.h

$(objtree)/layout.h: $(obj)/kernel.o $(srctree)/scripts/generate_layout_header.py  FORCE
	$(call if_changed,exports)

PHONY += prepare
prepare: $(obj)/kernel.o $(obj)/klib.o $(objtree)/layout.h
	@:

# Ordinary directory descending
# ---------------------------------------------------------------------------
obj-y			+= third_lib/
obj-y			+= klib.o
obj-y 			+= kernel.o
obj-y			+= arch/$(SRCARCH)/
