# SPDX-License-Identifier: GPL-2.0
#
# Kbuild for top-level directory of the kernel

third_lib= --extern const_format --extern static_assertions --extern bitflags \
 --extern tock_registers

#allow_features= naked_functions
$(obj)/kernel.o: private rustc_target_flags = --extern macros $(third_lib) 
	
#-Zallow-features=$(allow_features)
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

rusttest: rusttest-kernel

rusttest-kernel: private rustc_target_flags = --extern macros $(third_lib)
rusttest-kernel: $(src)/kernel/lib.rs  rusttest-third_lib FORCE
	+$(call if_changed,rustc_test)

PHONY += rusttest-third_lib
rusttest-third_lib:
	$(Q)$(MAKE) $(build)=third_lib rusttest

# Ordinary directory descending
# ---------------------------------------------------------------------------
obj-y			+= third_lib/
obj-y 			+= kernel.o
obj-y			+= init/
obj-y			+= arch/$(SRCARCH)/
