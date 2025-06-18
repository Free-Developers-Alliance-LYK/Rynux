# SPDX-License-Identifier: GPL-2.0
#
# Kbuild for top-level directory of the kernel

# Prepare global headers and check sanity before descending into sub-directories
# ---------------------------------------------------------------------------


# `$(rust_flags)` is passed in case the user added `--sysroot`.
rustc_sysroot := $(shell MAKEFLAGS= $(RUSTC) $(rust_flags) --print sysroot)
rustc_host_target := $(shell $(RUSTC) --version --verbose | grep -F 'host: ' | cut -d' ' -f2)
RUST_LIB_SRC ?= $(rustc_sysroot)/lib/rustlib/src/rust/library

core-cfgs = \
    --cfg no_fp_fmt_parse

alloc-cfgs = \
    --cfg no_global_oom_handling \
    --cfg no_rc


redirect-intrinsics = \
	__addsf3 __eqsf2 __extendsfdf2 __gesf2 __lesf2 __ltsf2 __mulsf3 __nesf2 __truncdfsf2 __unordsf2 \
	__adddf3 __eqdf2 __ledf2 __ltdf2 __muldf3 __unorddf2 \
	__muloti4 __multi3 \
	__udivmodti4 __udivti3 __umodti3

ifneq ($(or $(CONFIG_ARM64),$(and $(CONFIG_RISCV),$(CONFIG_64BIT))),)
	# These intrinsics are defined for ARM64 and RISCV64
	redirect-intrinsics += \
		__ashrti3 \
		__ashlti3 __lshrti3
endif

no-clean-files += libmacros.so

# Procedural macros can only be used with the `rustc` that compiled it.
$(obj)/libmacros.so: $(src)/macros/lib.rs FORCE
	+$(call if_changed_dep,rustc_procmacro)

$(obj)/core.o: private skip_clippy = 1
$(obj)/core.o: private skip_flags = -Wunreachable_pub
$(obj)/core.o: private rustc_objcopy = $(foreach sym,$(redirect-intrinsics),--redefine-sym $(sym)=__rust$(sym))
$(obj)/core.o: private rustc_target_flags = $(core-cfgs)
$(obj)/core.o: $(RUST_LIB_SRC)/core/src/lib.rs \
    $(wildcard $(objtree)/include/config/RUSTC_VERSION_TEXT) FORCE
	+$(call if_changed_rule,rustc_library)

$(obj)/compiler_builtins.o: private rustc_objcopy = -w -W '__*'
$(obj)/compiler_builtins.o: $(src)/compiler_builtins.rs $(obj)/core.o FORCE
	+$(call if_changed_rule,rustc_library)

$(obj)/alloc.o: private skip_clippy = 1
$(obj)/alloc.o: private skip_flags = -Wunreachable_pub
$(obj)/alloc.o: private rustc_target_flags = $(alloc-cfgs)
$(obj)/alloc.o: $(RUST_LIB_SRC)/alloc/src/lib.rs $(obj)/compiler_builtins.o FORCE
	+$(call if_changed_rule,rustc_library)

$(obj)/kernel.o: private rustc_target_flags = --extern alloc \
    --extern macros
$(obj)/kernel.o: $(src)/kernel/lib.rs $(obj)/alloc.o \
    $(obj)/libmacros.so FORCE
	+$(call if_changed_rule,rustc_library)

PHONY += prepare
prepare: $(obj)/core.o $(obj)/compiler_builtins.o $(obj)/alloc.o $(obj)/kernel.o $(obj)/libmacros.so
	@:

# Ordinary directory descending
# ---------------------------------------------------------------------------
always-y += libmacros.so
obj-y			+= core.o
obj-y			+= compiler_builtins.o
obj-y 			+= alloc.o
obj-y 			+= kernel.o
obj-y			+= arch/$(SRCARCH)/
