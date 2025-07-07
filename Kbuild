# SPDX-License-Identifier: GPL-2.0
#
# Kbuild for top-level directory of the kernel

third_lib= --extern const_format --extern static_assertions --extern bitflags

$(obj)/kernel.o: private rustc_target_flags = --extern macros $(third_lib) \
	-Zallow-features=naked_functions
$(obj)/kernel.o: $(src)/kernel/lib.rs  $(obj)/third_lib/built-in.a FORCE
	+$(call if_changed_rule,rustc_library)

quiet_cmd_exports = GEN $@
      cmd_exports = $(srctree)/scripts/generate_layout_header.py $< $@

targets += $(objtree)/layout.h

$(objtree)/layout.h: $(obj)/kernel.o $(srctree)/scripts/generate_layout_header.py  FORCE
	$(call if_changed,exports)

PHONY += prepare
prepare: $(obj)/kernel.o $(objtree)/layout.h
	@:

# Ordinary directory descending
# ---------------------------------------------------------------------------
obj-y			+= third_lib/
obj-y 			+= kernel.o
obj-y			+= arch/$(SRCARCH)/
