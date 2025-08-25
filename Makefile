VERSION = 0
PATCHLEVEL = 1
SUBLEVEL = 0
EXTRAVERSION =
NAME = Rynux

$(if $(filter __%, $(MAKECMDGOALS)), \
      $(error targets prefixed with '__' are only for internal use))

PHONY := __all
__all:

# SET SUBARCH
this-makefile := $(lastword $(MAKEFILE_LIST))
abs_srctree := $(realpath $(dir $(this-makefile)))
abs_objtree := $(CURDIR)

ifneq ($(sub_make_done),1)
# Do not use make's built-in rules and variables
# (this increases performance and avoids hard-to-debug behaviour)
MAKEFLAGS += -rR
# Avoid funny character set dependencies
unexport LC_ALL
LC_COLLATE=C
LC_NUMERIC=C
export LC_COLLATE RC_NUMERIC

# Avoid interference with shell env settings
unexport GREP_OPTIONS

# Beautify output
# ---------------------------------------------------------------------------
#
# Most of build commands in Kbuild start with "cmd_". You can optionally define
# "quiet_cmd_*". If defined, the short log is printed. Otherwise, no log from
# that command is printed by default.
#
# e.g.)
#    quiet_cmd_depmod = DEPMOD  $(MODLIB)
#          cmd_depmod = $(srctree)/scripts/depmod.sh $(DEPMOD) $(KERNELRELEASE)
#
# A simple variant is to prefix commands with $(Q) - that's useful
# for commands that shall be hidden in non-verbose mode.
#
#    $(Q)$(MAKE) $(build)=scripts/basic
#
# If KBUILD_VERBOSE contains 1, the whole command is echoed.
# If KBUILD_VERBOSE contains 2, the reason for rebuilding is printed.
#
# To put more focus on warnings, be less verbose as default
# Use 'make V=1' to see the full commands

ifeq ("$(origin V)", "command line")
  KBUILD_VERBOSE = $(V)
endif

quiet = quiet_
Q = @

ifneq ($(findstring 1, $(KBUILD_VERBOSE)),)
  quiet =
  Q =
endif

# If the user is running make -s (silent mode), suppress echoing of
# commands
ifneq ($(findstring s,$(firstword -$(MAKEFLAGS))),)
quiet=silent_
override KBUILD_VERBOSE :=
endif

export quiet Q KBUILD_VERBOSE

# Enable "clippy" (a linter) as part of the Rust compilation.
#
# Use 'make CLIPPY=1' to enable it.
ifeq ("$(origin CLIPPY)", "command line")
  KBUILD_CLIPPY := $(CLIPPY)
endif

export KBUILD_CLIPPY

# Kbuild will save output files in the current working directory.
# This does not need to match to the root of the kernel source tree.
#
# For example, you can do this:
#
#  cd /dir/to/store/output/files; make -f /dir/to/kernel/source/Makefile
#
# If you want to save output files in a different location, there are
# two syntaxes to specify it.
#
# 1) O=
# Use "make O=dir/to/store/output/files/"
#
# 2) Set KBUILD_OUTPUT
# Set the environment variable KBUILD_OUTPUT to point to the output directory.
# export KBUILD_OUTPUT=dir/to/store/output/files/; make
#
# The O= assignment takes precedence over the KBUILD_OUTPUT environment

# variable.

# Do we want to change the working directory?
ifeq ("$(origin O)", "command line")
  KBUILD_OUTPUT := $(O)
endif

# Do we want to change the working directory?
ifeq ("$(origin O)", "command line")
  KBUILD_OUTPUT := $(O)
endif

ifneq ($(KBUILD_OUTPUT),)
# $(realpath ...) gets empty if the path does not exist. Run 'mkdir -p' first.
$(shell mkdir -p "$(KBUILD_OUTPUT)")
# $(realpath ...) resolves symlinks
abs_objtree := $(realpath $(KBUILD_OUTPUT))
$(if $(abs_objtree),,$(error failed to create output directory "$(KBUILD_OUTPUT)"))
endif # ifneq ($(KBUILD_OUTPUT),)

ifneq ($(words $(subst :, ,$(abs_srctree))), 1)
$(error source directory cannot contain spaces or colons)
endif

export sub_make_done := 1

endif # sub_make_done

ifeq ($(abs_objtree),$(CURDIR))
# Suppress "Entering directory ..." if we are at the final work directory.
no-print-directory := --no-print-directory
else
# Recursion to show "Entering directory ..."
need-sub-make := 1
endif

ifeq ($(filter --no-print-directory, $(MAKEFLAGS)),)
# If --no-print-directory is unset, recurse once again to set it.
# You may end up recursing into __sub-make twice. This is needed due to the
# behavior change in GNU Make 4.4.1.
need-sub-make := 1
endif

ifeq ($(need-sub-make),1)

PHONY += $(MAKECMDGOALS) __sub-make

$(filter-out $(this-makefile), $(MAKECMDGOALS)) __all: __sub-make
	@:

# Invoke a second make in the output directory, passing relevant variables
__sub-make:
	$(Q)$(MAKE) $(no-print-directory) -C $(abs_objtree) \
	-f $(abs_srctree)/Makefile $(MAKECMDGOALS)

else # need-sub-make

# We process the rest of the Makefile if this is the final invocation of make
ifeq ($(abs_srctree),$(abs_objtree))
        # building in the source tree
        srctree := .
	building_out_of_srctree :=
else
        ifeq ($(abs_srctree)/,$(dir $(abs_objtree)))
                # building in a subdirectory of the source tree
                srctree := ..
        else
                srctree := $(abs_srctree)
        endif
	building_out_of_srctree := 1
endif

ifneq ($(KBUILD_ABS_SRCTREE),)
srctree := $(abs_srctree)
endif

objtree		:= .

VPATH		:=

ifdef building_out_of_srctree
$(info "-------------------building out of source tree, srctree=$(srctree)")
VPATH       := $(srctree)
endif

export srctree objtree VPATH

# To make sure we do not include .config for any of the *config targets
# catch them early, and hand them over to scripts/kconfig/Makefile
# It is allowed to specify more targets when calling make, including
# mixing *config targets and build targets.
# For example 'make oldconfig all'.
# Detect when mixed targets is specified, and make a second invocation
# of make so .config is not included in this case either (for *config).

clean-targets := %clean mrproper cleandocs

no-dot-config-targets := $(clean-targets)  help% %docs kernelversion \
			 dt_binding_check outputmakefile rustavailable rustfmt \
			 rustfmtcheck

no-sync-config-targets := $(no-dot-config-targets) %install \
			  image_name kernelrelease

config-build	:=
mixed-build	:=
need-config	:= 1
may-sync-config	:= 1

ifneq ($(filter $(no-dot-config-targets), $(MAKECMDGOALS)),)
    ifeq ($(filter-out $(no-dot-config-targets), $(MAKECMDGOALS)),)
        need-config :=
    endif
endif

ifneq ($(filter $(no-sync-config-targets), $(MAKECMDGOALS)),)
    ifeq ($(filter-out $(no-sync-config-targets), $(MAKECMDGOALS)),)
        may-sync-config :=
    endif
endif

ifneq ($(filter %config,$(MAKECMDGOALS)),)
    config-build := 1
    ifneq ($(words $(MAKECMDGOALS)),1)
        mixed-build := 1
    endif
endif

need-compiler := $(may-sync-config)

# For "make -j clean all", "make -j mrproper defconfig all", etc.
ifneq ($(filter $(clean-targets),$(MAKECMDGOALS)),)
    ifneq ($(filter-out $(clean-targets),$(MAKECMDGOALS)),)
        mixed-build := 1
    endif
endif

# For "make -j clean all", "make -j mrproper defconfig all", etc.
ifneq ($(filter $(clean-targets),$(MAKECMDGOALS)),)
    ifneq ($(filter-out $(clean-targets),$(MAKECMDGOALS)),)
        mixed-build := 1
    endif
endif


ifdef mixed-build
# ===========================================================================
# We're called with mixed targets (*config and build targets).
# Handle them one by one.

PHONY += $(MAKECMDGOALS) __build_one_by_one

$(MAKECMDGOALS): __build_one_by_one
	@:

__build_one_by_one:
	$(Q)set -e; \
	for i in $(MAKECMDGOALS); do \
		$(MAKE) -f $(srctree)/Makefile $$i; \
	done

else # !mixed-build

include $(srctree)/scripts/Kbuild.include

# Read KERNELRELEASE from include/config/kernel.release (if it exists)
KERNELRELEASE = $(call read-file, include/config/kernel.release)
KERNELVERSION = $(VERSION)$(if $(PATCHLEVEL),.$(PATCHLEVEL)$(if $(SUBLEVEL),.$(SUBLEVEL)))$(EXTRAVERSION)
export VERSION PATCHLEVEL SUBLEVEL KERNELRELEASE KERNELVERSION

include $(srctree)/scripts/subarch.include

# Cross compiling and selecting different set of gcc/bin-utils
# ---------------------------------------------------------------------------
#
# When performing cross compilation for other architectures ARCH shall be set
# to the target architecture. (See arch/* for the possibilities).
# ARCH can be set during invocation of make:
# make ARCH=arm64
# Another way is to have ARCH set in the environment.
# The default ARCH is the host where make is executed.

# CROSS_COMPILE specify the prefix used for all executables used
# during compilation. Only gcc and related bin-utils executables
# are prefixed with $(CROSS_COMPILE).
# CROSS_COMPILE can be set on the command line
# make CROSS_COMPILE=aarch64-linux-gnu-
# Alternatively CROSS_COMPILE can be set in the environment.
# Default value for CROSS_COMPILE is not to prefix executables
# Note: Some architectures assign CROSS_COMPILE in their arch/*/Makefile
#ARCH		?= $(SUBARCH)
# Currently we only support arm64
ARCH := arm64

# Architecture as present in compile.h
UTS_MACHINE 	:= $(ARCH)
SRCARCH 	:= $(ARCH)

export cross_compiling :=
ifneq ($(SRCARCH),$(SUBARCH))
cross_compiling := 1
endif

KCONFIG_CONFIG	?= .config
export KCONFIG_CONFIG

# SHELL used by kbuild
CONFIG_SHELL := sh

HOST_LFS_CFLAGS := $(shell getconf LFS_CFLAGS 2>/dev/null)
HOST_LFS_LDFLAGS := $(shell getconf LFS_LDFLAGS 2>/dev/null)
HOST_LFS_LIBS := $(shell getconf LFS_LIBS 2>/dev/null)

ifeq ($(LLVM),)
$(error LLVM variable is empty! Please set LLVM to a non-empty value.)
endif

ifneq ($(filter %/,$(LLVM)),)
LLVM_PREFIX := $(LLVM)
endif

ifneq ($(filter -%,$(LLVM)),)
LLVM_SUFFIX := $(LLVM)
endif

HOSTCC	= $(LLVM_PREFIX)clang$(LLVM_SUFFIX)
HOSTCXX	= $(LLVM_PREFIX)clang++$(LLVM_SUFFIX)
HOSTRUSTC = rustc
HOSTPKG_CONFIG	= pkg-config

KBUILD_USERHOSTCFLAGS := -Wall -Wmissing-prototypes -Wstrict-prototypes \
			 -O2 -fomit-frame-pointer -std=gnu11
KBUILD_USERCFLAGS  := $(KBUILD_USERHOSTCFLAGS) $(USERCFLAGS)
KBUILD_USERLDFLAGS := $(USERLDFLAGS)

# These flags apply to all Rust code in the tree, including the kernel and
# host programs.

export rust_common_flags := --edition=2021 \
                  -Zbinary_dep_depinfo=y \
                  -Astable_features \
                  -Dnon_ascii_idents \
                  -Dunsafe_op_in_unsafe_fn \
                  -Wmissing_docs \
                  -Wrust_2018_idioms \
                  -Wunreachable_pub \
                  -Wclippy::all \
                  -Wclippy::ignored_unit_patterns \
                  -Wclippy::mut_mut \
                  -Wclippy::needless_bitwise_bool \
                  -Aclippy::needless_lifetimes \
                  -Wclippy::no_mangle_with_rust_abi \
                  -Wclippy::undocumented_unsafe_blocks \
                  -Wclippy::unnecessary_safety_comment \
                  -Wclippy::unnecessary_safety_doc \
                  -Wrustdoc::missing_crate_level_docs \
                  -Wrustdoc::unescaped_backticks

KBUILD_HOSTCFLAGS   := $(KBUILD_USERHOSTCFLAGS) $(HOST_LFS_CFLAGS) \
		       $(HOSTCFLAGS) -I $(srctree)/scripts/include
KBUILD_HOSTCXXFLAGS := -Wall -O2 $(HOST_LFS_CFLAGS) $(HOSTCXXFLAGS) \
		       -I $(srctree)/scripts/include
KBUILD_HOSTRUSTFLAGS := $(rust_common_flags) -O -Cstrip=debuginfo \
			-Zallow-features= $(HOSTRUSTFLAGS)
KBUILD_HOSTLDFLAGS  := $(HOST_LFS_LDFLAGS) $(HOSTLDFLAGS)
KBUILD_HOSTLDLIBS   := $(HOST_LFS_LIBS) $(HOSTLDLIBS)

KBUILD_PROCMACROLDFLAGS := $(or $(PROCMACROLDFLAGS),$(KBUILD_HOSTLDFLAGS))

# Make variables (CC, etc...)
CPP		= $(CC) -E
CC		= $(LLVM_PREFIX)clang$(LLVM_SUFFIX)
LD		= $(LLVM_PREFIX)ld.lld$(LLVM_SUFFIX)
AR		= $(LLVM_PREFIX)llvm-ar$(LLVM_SUFFIX)
NM		= $(LLVM_PREFIX)llvm-nm$(LLVM_SUFFIX)
OBJCOPY		= $(LLVM_PREFIX)llvm-objcopy$(LLVM_SUFFIX)
OBJDUMP		= $(LLVM_PREFIX)llvm-objdump$(LLVM_SUFFIX)
READELF		= $(LLVM_PREFIX)llvm-readelf$(LLVM_SUFFIX)
STRIP		= $(LLVM_PREFIX)llvm-strip$(LLVM_SUFFIX)

RUSTC		= rustc
RUSTDOC		= rustdoc
RUSTFMT		= rustfmt
CLIPPY_DRIVER	= clippy-driver
PAHOLE		= pahole
LEX		= flex
YACC		= bison
AWK		= awk
INSTALLKERNEL  := installkernel
PERL		= perl
PYTHON3		= python3
BASH		= bash
KGZIP		= gzip
KBZIP2		= bzip2
KLZOP		= lzop
LZMA		= lzma
LZ4		= lz4c
XZ		= xz
ZSTD		= zstd

NOSTDINC_FLAGS :=
CFLAGS_KERNEL	=
RUSTFLAGS_KERNEL =
AFLAGS_KERNEL	=
LDFLAGS_vmrynux =

KBUILD_RUSTFLAGS := $(rust_common_flags) \
		    -Cpanic=abort -Cembed-bitcode=n -Clto=n \
		    -Cforce-unwind-tables=n -Ccodegen-units=1 \
		    -Csymbol-mangling-version=v0 \
		    -Crelocation-model=static \
		    -Zfunction-sections=n \
		    -Wclippy::float_arithmetic

KBUILD_AFLAGS_KERNEL :=
KBUILD_CFLAGS_KERNEL :=
KBUILD_RUSTFLAGS_KERNEL :=
KBUILD_LDFLAGS :=
CLANG_FLAGS :=
KBUILD_CPPFLAGS := -I$(objtree) -D__KERNEL__

# Use USERINCLUDE when you must reference the UAPI directories only.
USERINCLUDE    := \
          -I$(srctree)/arch/$(SRCARCH)/include/uapi \
          -I$(objtree)/arch/$(SRCARCH)/include/generated/uapi \
          -I$(srctree)/include/uapi \
          -I$(objtree)/include/generated/uapi \
		  	-include $(srctree)/include/kconfig.h

# Use RYNUXINCLUDE when you must reference the include/ directory.
# Needed to be compatible with the O= option
RYNUXINCLUDE    := \
		-I$(srctree)/arch/$(SRCARCH)/include \
		-I$(objtree)/arch/$(SRCARCH)/include/generated \
		-I$(srctree)/include \
		-I$(objtree)/include \
		$(USERINCLUDE)

KBUILD_AFLAGS   := -D__ASSEMBLY__ -fno-PIE


ifeq ($(KBUILD_CLIPPY),1)
	RUSTC_OR_CLIPPY_QUIET := CLIPPY
	RUSTC_OR_CLIPPY = $(CLIPPY_DRIVER)
else
	RUSTC_OR_CLIPPY_QUIET := RUSTC
	RUSTC_OR_CLIPPY = $(RUSTC)
endif

# Allows the usage of unstable features in stable compilers.
export RUSTC_BOOTSTRAP := 1

export ARCH SRCARCH CONFIG_SHELL BASH HOSTCC KBUILD_HOSTCFLAGS CROSS_COMPILE LD CC HOSTPKG_CONFIG
export RUSTC RUSTDOC RUSTFMT RUSTC_OR_CLIPPY_QUIET RUSTC_OR_CLIPPY
export HOSTRUSTC KBUILD_HOSTRUSTFLAGS
export CPP AR NM STRIP OBJCOPY OBJDUMP READELF PAHOLE RESOLVE_BTFIDS LEX YACC AWK INSTALLKERNEL
export PERL PYTHON3 MAKE UTS_MACHINE HOSTCXX
export KGZIP KBZIP2 KLZOP LZMA LZ4 XZ ZSTD
export KBUILD_HOSTCXXFLAGS KBUILD_HOSTLDFLAGS KBUILD_HOSTLDLIBS KBUILD_PROCMACROLDFLAGS
export KBUILD_USERCFLAGS KBUILD_USERLDFLAGS

export KBUILD_CPPFLAGS NOSTDINC_FLAGS RYNUXINCLUDE OBJCOPYFLAGS KBUILD_LDFLAGS
export KBUILD_CFLAGS CFLAGS_KERNEL
export KBUILD_RUSTFLAGS RUSTFLAGS_KERNEL
export KBUILD_AFLAGS AFLAGS_KERNEL
export KBUILD_AFLAGS_KERNEL KBUILD_CFLAGS_KERNEL KBUILD_RUSTFLAGS_KERNEL

# Files to ignore in find ... statements

export RCS_FIND_IGNORE := \( -name SCCS -o -name BitKeeper -o -name .svn -o    \
			  -name CVS -o -name .pc -o -name .hg -o -name .git \) \
			  -prune -o

# ===========================================================================
# Rules shared between *config targets and build targets

# Basic helpers built in scripts/basic/
PHONY += scripts_basic
scripts_basic:
	$(Q)$(MAKE) $(build)=scripts/basic

PHONY += outputmakefile

ifdef building_out_of_srctree

$(info "-------------------outputmakefile: building out of source tree, srctree=$(srctree)")
# Before starting out-of-tree build, make sure the source tree is clean.
# outputmakefile generates a Makefile in the output directory, if using a
# separate output directory. This allows convenient use of make in the
# output directory.
# At the same time when output Makefile generated, generate .gitignore to
# ignore whole output directory

quiet_cmd_makefile = GEN     Makefile
      cmd_makefile = { \
	echo "\# Automatically generated by $(srctree)/Makefile: don't edit"; \
	echo "include $(srctree)/Makefile"; \
	} > Makefile

outputmakefile:
	@if [ -f $(srctree)/.config -o \
		 -d $(srctree)/include/config -o \
		 -d $(srctree)/arch/$(SRCARCH)/include/generated ]; then \
		echo >&2 "***"; \
		echo >&2 "*** The source tree is not clean, please run 'make$(if $(findstring command line, $(origin ARCH)), ARCH=$(ARCH)) mrproper'"; \
		echo >&2 "*** in $(abs_srctree)";\
		echo >&2 "***"; \
		false; \
	fi
	$(Q)ln -fsn $(srctree) source
	$(call cmd,makefile)
	$(Q)test -e .gitignore || \
	{ echo "# this is build directory, ignore it"; echo "*"; } > .gitignore
endif

# The expansion should be delayed until arch/$(SRCARCH)/Makefile is included.
# Some architectures define CROSS_COMPILE in arch/$(SRCARCH)/Makefile.
# CC_VERSION_TEXT and RUSTC_VERSION_TEXT are referenced from Kconfig (so they
# need export), and from include/config/auto.conf.cmd to detect the compiler
# upgrade.
CC_VERSION_TEXT = $(subst $(pound),,$(shell LC_ALL=C $(CC) --version 2>/dev/null | head -n 1))
RUSTC_VERSION_TEXT = $(subst $(pound),,$(shell $(RUSTC) --version 2>/dev/null))

include $(srctree)/scripts/Makefile.clang

# Include this also for config targets because some architectures need
# cc-cross-prefix to determine CROSS_COMPILE.
ifdef need-compiler
include $(srctree)/scripts/Makefile.compiler
endif

ifdef config-build

include $(srctree)/arch/$(SRCARCH)/Makefile
export KBUILD_DEFCONFIG KBUILD_KCONFIG CC_VERSION_TEXT RUSTC_VERSION_TEXT

config: outputmakefile scripts_basic FORCE
	$(Q)$(MAKE) $(build)=scripts/kconfig $@

%config: outputmakefile scripts_basic FORCE
	$(Q)$(MAKE) $(build)=scripts/kconfig $@

else #!config-build
# ===========================================================================
# Build targets only - this includes vmrynux, arch-specific targets, clean
# targets and others. In general all targets except *config targets.

# If building an external module we do not care about the all: rule
# but instead __all depend on modules
PHONY += all
__all: all

targets :=

KBUILD_BUILTIN := 1

export KBUILD_BUILTIN

ifdef need-config
include include/config/auto.conf
endif

# The all: target is the default when no target is given on the
# command line.
# This allow a user to issue only 'make' to build a kernel including modules
# Defaults to vmrynux, but the arch makefile usually adds further targets
all: vmrynux

include $(srctree)/arch/$(SRCARCH)/Makefile

ifdef need-config
ifdef may-sync-config
# Read in dependencies to all Kconfig* files, make sure to run syncconfig if
# changes are detected. This should be included after arch/$(SRCARCH)/Makefile
# because some architectures define CROSS_COMPILE there.
include include/config/auto.conf.cmd

$(KCONFIG_CONFIG):
	@echo >&2 '***'
	@echo >&2 '*** Configuration file "$@" not found!'
	@echo >&2 '***'
	@echo >&2 '*** Please run some configurator (e.g. "make oldconfig" or'
	@echo >&2 '*** "make menuconfig" or "make xconfig").'
	@echo >&2 '***'
	@/bin/false

# The actual configuration files used during the build are stored in
# include/generated/ and include/config/. Update them if .config is newer than
# include/config/auto.conf (which mirrors .config).
#
# This exploits the 'multi-target pattern rule' trick.
# The syncconfig should be executed only once to make all the targets.
# (Note: use the grouped target '&:' when we bump to GNU Make 4.3)
#
# Do not use $(call cmd,...) here. That would suppress prompts from syncconfig,
# so you cannot notice that Kconfig is waiting for the user input.
%/config/auto.conf %/config/auto.conf.cmd %/generated/autoconf.h %/generated/rustc_cfg: $(KCONFIG_CONFIG)
	$(Q)$(kecho) "  SYNC    $@"
	$(Q)$(MAKE) -f $(srctree)/Makefile syncconfig

else # !may-sync-config
# External modules and some install targets need include/generated/autoconf.h
# and include/config/auto.conf but do not care if they are up-to-date.
# Use auto.conf to trigger the test
PHONY += include/config/auto.conf

include/config/auto.conf:
	@test -e include/generated/autoconf.h -a -e $@ || (		\
	echo >&2;							\
	echo >&2 "  ERROR: Kernel configuration is invalid.";		\
	echo >&2 "         include/generated/autoconf.h or $@ are missing.";\
	echo >&2 "         Run 'make oldconfig && make prepare' on kernel src to fix it.";	\
	echo >&2 ;							\
	/bin/false)

endif # may-sync-config
endif # need-config


ifdef CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE
KBUILD_RUSTFLAGS += -Copt-level=2
else ifdef CONFIG_CC_OPTIMIZE_FOR_SIZE
KBUILD_RUSTFLAGS += -Copt-level=s
endif

ifdef building_out_of_srctree
KBUILD_CPPFLAGS += $(call cc-option,-fmacro-prefix-map=$(srctree)/=)
$(info "KBUILD_CPPFLAGS = $(KBUILD_CPPFLAGS)")
endif

# Always set `debug-assertions` and `overflow-checks` because their default
# depends on `opt-level` and `debug-assertions`, respectively.
KBUILD_RUSTFLAGS += -Cdebug-assertions=$(if $(CONFIG_RUST_DEBUG_ASSERTIONS),y,n)
KBUILD_RUSTFLAGS += -Coverflow-checks=$(if $(CONFIG_RUST_OVERFLOW_CHECKS),y,n)

KBUILD_RUSTFLAGS-$(CONFIG_WERROR) += -Dwarnings
KBUILD_RUSTFLAGS += $(KBUILD_RUSTFLAGS-y)

ifdef CONFIG_FRAME_POINTER
KBUILD_RUSTFLAGS += -Cforce-frame-pointers=y
endif

# arch Makefile may override CC so keep this after arch Makefile is included
NOSTDINC_FLAGS += -nostdinc

LDFLAGS_vmrynux += --build-id=sha1

ifeq ($(CONFIG_RELR),y)
  # ld.lld before 15 did not support -z pack-relative-relocs.
  LDFLAGS_vmrynux += $(call ld-option,--pack-dyn-relocs=relr,-z pack-relative-relocs)
endif

# We never want expected sections to be placed heuristically by the
# linker. All sections should be explicitly named in the linker script.
LDFLAGS_vmrynux += --orphan-handling=warn

KBUILD_LDFLAGS	+= -z noexecstack

# Align the bit size of userspace programs with the kernel
KBUILD_USERCFLAGS  += $(filter -m32 -m64 --target=%, $(KBUILD_CFLAGS))
KBUILD_USERLDFLAGS += $(filter -m32 -m64 --target=%, $(KBUILD_CFLAGS))

# Default kernel image to build when no specific target is given.
# KBUILD_IMAGE may be overruled on the command line or
# set in the environment
# Also any assignments in arch/$(ARCH)/Makefile take precedence over
# this default value
export KBUILD_IMAGE ?= vmrynux

# INSTALL_PATH specifies where to place the updated kernel and system map
# images. Default is /boot, but you can set it to other values
export	INSTALL_PATH ?= $(objtree)/boot

#
# INSTALL_DTBS_PATH specifies a prefix for relocations required by build roots.
# Like INSTALL_MOD_PATH, it isn't defined in the Makefile, but can be passed as
# an argument if needed. Otherwise it defaults to the kernel install path
#
export INSTALL_DTBS_PATH ?= $(INSTALL_PATH)/dtbs/$(KERNELRELEASE)

PHONY += prepare0

export extmod_prefix = $(if $(KBUILD_EXTMOD),$(KBUILD_EXTMOD)/)

build-dir	:= .
clean-dirs	:= .

# Externally visible symbols (used by link-vmrynux.sh)

KBUILD_VMRYNUX_OBJS := ./built-in.a

export KBUILD_LDS          := arch/$(SRCARCH)/kernel/vmrynux.lds

# '$(AR) mPi' needs 'T' to workaround the bug of llvm-ar <= 14
quiet_cmd_ar_vmrynux.a = AR      $@
      cmd_ar_vmrynux.a = \
	rm -f $@; \
	$(AR) cDPrST $@ $(KBUILD_VMRYNUX_OBJS)

targets += vmrynux.a
vmrynux.a: $(KBUILD_VMRYNUX_OBJS) FORCE
	$(call if_changed,ar_vmrynux.a)

PHONY += vmrynux_o
vmrynux_o: vmrynux.a
	$(Q)$(MAKE) -f $(srctree)/scripts/Makefile.vmrynux_o

vmrynux.o : vmrynux_o
	@:

PHONY += vmrynux
# LDFLAGS_vmrynux in the top Makefile defines linker flags for the top vmrynux,
# not for decompressors. LDFLAGS_vmrynux in arch/*/boot/compressed/Makefile is
# unrelated; the decompressors just happen to have the same base name,
# arch/*/boot/compressed/vmrynux.
# Export LDFLAGS_vmrynux only to scripts/Makefile.vmrynux.
#
# _LDFLAGS_vmrynux is a workaround for the 'private export' bug:
#   https://savannah.gnu.org/bugs/?61463
# For Make > 4.4, the following simple code will work:
#  vmrynux: private export LDFLAGS_vmrynux := $(LDFLAGS_vmrynux)
vmrynux: private _LDFLAGS_vmrynux := $(LDFLAGS_vmrynux)
vmrynux: export LDFLAGS_vmrynux = $(_LDFLAGS_vmrynux)
vmrynux: vmrynux.o $(KBUILD_LDS)
	$(Q)$(MAKE) -f $(srctree)/scripts/Makefile.vmrynux

# The actual objects are generated when descending,
# make sure no implicit rule kicks in
$(sort $(KBUILD_LDS) $(KBUILD_VMRYNUX_OBJS)): . ;

ifeq ($(origin KERNELRELEASE),file)
filechk_kernel.release = $(srctree)/scripts/setlocalversion $(srctree)
else
	filechk_kernel.release = echo $(KERNELRELEASE)
endif

# Store (new) KERNELRELEASE string in include/config/kernel.release
include/config/kernel.release: FORCE
	$(call filechk,kernel.release)

# Additional helpers built in scripts/
# Carefully list dependencies so we do not try to build scripts twice
# in parallel
PHONY += scripts
scripts: scripts_basic scripts_dtc
	$(Q)$(MAKE) $(build)=$(@)

# Things we need to do before we recursively start building the kernel
# or the modules are listed in "prepare".
# A multi level approach is used. prepareN is processed before prepareN-1.
# archprepare is used in arch Makefiles and when processed asm symlink,
# version.h and scripts_basic is processed / created.

PHONY += prepare archprepare

archprepare: outputmakefile scripts include/config/kernel.release \
	include/generated/autoconf.h

prepare0: archprepare
	$(Q)$(MAKE) $(build)=. prepare

# All the preparing..
prepare: prepare0
	+$(Q)$(CONFIG_SHELL) $(srctree)/scripts/rust_is_available.sh

# ---------------------------------------------------------------------------
# Install

# Many distributions have the custom install script, /sbin/installkernel.
# If DKMS is installed, 'make install' will eventually recurse back
# to this Makefile to build and install external modules.
# Cancel sub_make_done so that options such as M=, V=, etc. are parsed.

quiet_cmd_install = INSTALL $(INSTALL_PATH)
      cmd_install = unset sub_make_done; $(srctree)/scripts/install.sh

# ---------------------------------------------------------------------------
# vDSO install

PHONY += vdso_install
vdso_install: export INSTALL_FILES = $(vdso-install-y)
vdso_install:
	$(Q)$(MAKE) -f $(srctree)/scripts/Makefile.vdsoinst

# ---------------------------------------------------------------------------
# Tools

# Clear a bunch of variables before executing the submake
ifeq ($(quiet),silent_)
tools_silent=s
endif

tools/: FORCE
	$(Q)mkdir -p $(objtree)/tools
	$(Q)$(MAKE) LDFLAGS= MAKEFLAGS="$(tools_silent) $(filter --j% -j,$(MAKEFLAGS))" O=$(abspath $(objtree)) subdir=tools -C $(srctree)/tools/

tools/%: FORCE
	$(Q)mkdir -p $(objtree)/tools
	$(Q)$(MAKE) LDFLAGS= MAKEFLAGS="$(tools_silent) $(filter --j% -j,$(MAKEFLAGS))" O=$(abspath $(objtree)) subdir=tools -C $(srctree)/tools/ $*

# ---------------------------------------------------------------------------
# Devicetree files

ifneq ($(wildcard $(srctree)/arch/$(SRCARCH)/boot/dts/),)
dtstree := arch/$(SRCARCH)/boot/dts
endif

ifneq ($(dtstree),)

%.dtb: dtbs_prepare
	$(Q)$(MAKE) $(build)=$(dtstree) $(dtstree)/$@

%.dtbo: dtbs_prepare
	$(Q)$(MAKE) $(build)=$(dtstree) $(dtstree)/$@

PHONY += dtbs dtbs_prepare dtbs_install dtbs_check
dtbs: dtbs_prepare
	$(Q)$(MAKE) $(build)=$(dtstree) need-dtbslist=1


# include/config/kernel.release is actually needed when installing DTBs because
# INSTALL_DTBS_PATH contains $(KERNELRELEASE). However, we do not want to make
# dtbs_install depend on it as dtbs_install may run as root.
dtbs_prepare: include/config/kernel.release scripts_dtc
dtbs_check: dtbs

dtbs_install:
	$(Q)$(MAKE) -f $(srctree)/scripts/Makefile.dtbinst obj=$(dtstree)

all: dtbs

endif

PHONY += scripts_dtc
scripts_dtc: scripts_basic
	$(Q)$(MAKE) $(build)=scripts/dtc

###
# Cleaning is done on three levels.
# make clean     Delete most generated files
#                Leave enough to build external modules
# make mrproper  Delete the current configuration, and all generated files
# make distclean Remove editor backup files, patch leftover files and the like

# Directories & files removed with 'make clean'
CLEAN_FILES += vmrynux.symvers vmrynux.o.map \
	       compile_commands.json rust-project.json \
		   .vmrynux.objs .vmrynux.export.c

# Directories & files removed with 'make mrproper'
MRPROPER_FILES += include/config include/generated          \
		  arch/$(SRCARCH)/include/generated .objdiff \
		  debian snap tar-install PKGBUILD pacman \
		  .config .config.old .version \
		  certs/signing_key.pem \
		  certs/x509.genkey \
		  vmrynux-gdb.py \
		  proc_macros/libmacros.so proc_macros/libmacros.dylib

# clean - Delete most, but leave enough to build external modules
#
clean: private rm-files := $(CLEAN_FILES)

PHONY += vmrynuxclean

vmrynuxclean:
	$(Q)$(CONFIG_SHELL) $(srctree)/scripts/link-vmrynux.sh clean

clean: vmrynuxclean

# mrproper - Delete all generated files, including .config
#
mrproper: private rm-files := $(MRPROPER_FILES)
mrproper-dirs      := $(addprefix _mrproper_,scripts)

PHONY += $(mrproper-dirs) mrproper
$(mrproper-dirs):
	$(Q)$(MAKE) $(clean)=$(patsubst _mrproper_%,%,$@)

mrproper: clean $(mrproper-dirs)
	$(call cmd,rmfiles)
	@find . $(RCS_FIND_IGNORE) \
		\( -name '*.rmeta' \) \
		-type f -print | xargs rm -f

# distclean
#
PHONY += distclean

distclean: mrproper
	@find . $(RCS_FIND_IGNORE) \
		\( -name '*.orig' -o -name '*.rej' -o -name '*~' \
		-o -name '*.bak' -o -name '#*#' -o -name '*%' \
		-o -name 'core' -o -name tags -o -name TAGS -o -name 'cscope*' \
		-o -name GPATH -o -name GRTAGS -o -name GSYMS -o -name GTAGS \) \
		-type f -print | xargs rm -f


PHONY += help
help:
	@echo  'Cleaning targets:'
	@echo  '  clean		  - Remove most generated files but keep the config and'
	@echo  '                    enough build support to build external modules'
	@echo  '  mrproper	  - Remove all generated files + config + various backup files'
	@echo  '  distclean	  - mrproper + remove editor backup and patch files'
	@echo  ''
	@$(MAKE) -f $(srctree)/scripts/kconfig/Makefile help
	@echo  ''
	@echo  'Other generic targets:'
	@echo  '  all		  - Build all targets marked with [*]'
	@echo  '* vmrynux	  - Build the bare kernel'
	@echo  '  kernelrelease	  - Output the release version string (use with make -s)'
	@echo  '  kernelversion	  - Output the version stored in Makefile (use with make -s)'
	@echo  '  image_name	  - Output the image name (use with make -s)'
	 echo  ''
	@echo  'Static analysers:'
	@echo  '  clang-analyzer  - Check with clang static analyzer'
	@echo  '  clang-tidy      - Check with clang-tidy'
	@echo  ''
	@echo  'Rust targets:'
	@echo  '  rustavailable   - Checks whether the Rust toolchain is'
	@echo  '		    available and, if not, explains why.'
	@echo  '  rustfmt	  - Reformat all the Rust code in the kernel'
	@echo  '  rustfmtcheck	  - Checks if all the Rust code in the kernel'
	@echo  '		    is formatted, printing a diff otherwise.'
	@echo  '  rustdoc	  - Generate Rust documentation'
	@echo  '		    (requires kernel .config)'
	@echo  '  rusttest        - Runs the Rust tests'
	@echo  '                    (requires kernel .config; downloads external repos)'
	@echo  '  rust-analyzer	  - Generate rust-project.json rust-analyzer support file'
	@echo  '		    (requires kernel .config)'
	@echo  ''
	@$(if $(dtstree), \
		echo 'Devicetree:'; \
		echo '* dtbs               - Build device tree blobs for enabled boards'; \
		echo '  dtbs_install       - Install dtbs to $(INSTALL_DTBS_PATH)'; \
		echo '  dt_binding_check   - Validate device tree binding documents and examples'; \
		echo '  dt_binding_schemas - Build processed device tree binding schemas'; \
		echo '  dtbs_check         - Validate device tree source files';\
		echo '')

	@echo 'Userspace tools targets:'
	@echo '  use "make tools/help"'
	@echo '  or  "cd tools; make help"'
	@echo  ''
	@echo  'Kernel packaging:'
	@$(MAKE) -f $(srctree)/scripts/Makefile.package help
	@echo  ''
	@echo  'Architecture-specific targets ($(SRCARCH)):'
	@$(or $(archhelp),\
		echo '  No architecture-specific help defined for $(SRCARCH)')
	@echo  ''
	@$(if $(boards), \
		$(foreach b, $(boards), \
		printf "  %-27s - Build for %s\\n" $(b) $(subst _defconfig,,$(b));) \
		echo '')
	@$(if $(board-dirs), \
		$(foreach b, $(board-dirs), \
		printf "  %-16s - Show %s-specific targets\\n" help-$(b) $(b);) \
		printf "  %-16s - Show all of the above\\n" help-boards; \
		echo '')

	@echo  '  make V=n   [targets] 1: verbose build'
	@echo  '                       2: give reason for rebuild of target'
	@echo  '                       V=1 and V=2 can be combined with V=12'
	@echo  '  make O=dir [targets] Locate all output files in "dir", including .config'
	@echo  '  make W=n   [targets] Enable extra build checks, n=1,2,3,c,e where'
	@echo  '		1: warnings which may be relevant and do not occur too often'
	@echo  '		2: warnings which occur quite often but may still be relevant'
	@echo  '		3: more obscure warnings, can most likely be ignored'
	@echo  '		c: extra checks in the configuration stage (Kconfig)'
	@echo  '		e: warnings are being treated as errors'
	@echo  '		Multiple levels can be combined with W=12 or W=123'
	@$(if $(dtstree), \
		echo '  make CHECK_DTBS=1 [targets] Check all generated dtb files against schema'; \
		echo '         This can be applied both to "dtbs" and to individual "foo.dtb" targets' ; \
		)
	@echo  ''
	@echo  'Execute "make" or "make all" to build all targets marked with [*] '
	@echo  'For further info see the ./README file'

help-board-dirs := $(addprefix help-,$(board-dirs))

help-boards: $(help-board-dirs)

boards-per-dir = $(sort $(notdir $(wildcard $(srctree)/arch/$(SRCARCH)/configs/$*/*_defconfig)))

$(help-board-dirs): help-%:
	@echo  'Architecture-specific targets ($(SRCARCH) $*):'
	@$(if $(boards-per-dir), \
		$(foreach b, $(boards-per-dir), \
		printf "  %-24s - Build for %s\\n" $*/$(b) $(subst _defconfig,,$(b));) \
		echo '')

# Rust targets
# ---------------------------------------------------------------------------

# "Is Rust available?" target
PHONY += rustavailable
rustavailable:
	+$(Q)$(CONFIG_SHELL) $(srctree)/scripts/rust_is_available.sh && echo "Rust is available!"
# Documentation target
#
# Using the singular to avoid running afoul of `no-dot-config-targets`.
PHONY += rustdoc
rustdoc: prepare
	$(Q)$(MAKE) $(build)=. $@

# Testing target
PHONY += rusttest
rusttest: prepare
	$(Q)$(MAKE) $(build)=. $@

# Formatting targets
PHONY += rustfmt rustfmtcheck

# We skip `rust/alloc` since we want to minimize the diff w.r.t. upstream.
#
# We match using absolute paths since `find` does not resolve them
# when matching, which is a problem when e.g. `srctree` is `..`.
# We `grep` afterwards in order to remove the directory entry itself.
rustfmt:
	$(Q)find $(abs_srctree) -type f -name '*.rs' \
		-o -path $(abs_srctree)/rust/alloc -prune \
		-o -path $(abs_objtree)/rust/test -prune \
		| grep -Fv $(abs_srctree)/rust/alloc \
		| grep -Fv $(abs_objtree)/rust/test \
		| grep -Fv generated \
		| xargs $(RUSTFMT) $(rustfmt_flags)

rustfmtcheck: rustfmt_flags = --check
rustfmtcheck: rustfmt


# Preset locale variables to speed up the build process. Limit locale
# tweaks to this spot to avoid wrong language settings when running
# make menuconfig etc.
# Error messages still appears in the original language
PHONY += $(build-dir)
$(build-dir): prepare
	$(Q)$(MAKE) $(build)=$@ need-builtin=1 $(single-goals)

clean-dirs := $(addprefix _clean_, $(clean-dirs))
PHONY += $(clean-dirs) clean
$(clean-dirs):
	$(Q)$(MAKE) $(clean)=$(patsubst _clean_%,%,$@)

clean: $(clean-dirs)
	$(call cmd,rmfiles)
	@find $(or $(KBUILD_EXTMOD), .) $(RCS_FIND_IGNORE) \
		\( -name '*.[aios]' -o -name '*.rsi' -o -name '.*.cmd' \
		-o -name '*.dtb' -o -name '*.dtbo' \
		-o -name '*.dtb.S' -o -name '*.dtbo.S' \
		-o -name '*.dt.yaml' -o -name 'dtbs-list' \
		-o -name '*.dwo' -o -name '*.lst' \
		-o -name '*.su' \
		-o -name '.*.d' -o -name '.*.tmp' \
		-o -name '*.lex.c' -o -name '*.tab.[ch]' \
		-o -name '*.asn1.[ch]' \
		-o -name '*.symtypes' \
		-o -name '*.c.[012]*.*' \
		-o -name '*.ll' \
		-o -name '*.gcno' \
		\) -type f -print \
		-o -name '.tmp_*' -print \
		| xargs rm -rf


# Clang Tooling
# ---------------------------------------------------------------------------

quiet_cmd_gen_compile_commands = GEN     $@
      cmd_gen_compile_commands = $(PYTHON3) $< -a $(AR) -o $@ $(filter-out $<, $(real-prereqs))

$(extmod_prefix)compile_commands.json: $(srctree)/scripts/clang-tools/gen_compile_commands.py \
	$(if $(KBUILD_EXTMOD),, vmrynux.a ) FORCE
	$(call if_changed,gen_compile_commands)

targets += $(extmod_prefix)compile_commands.json

PHONY += clang-tidy clang-analyzer

quiet_cmd_clang_tools = CHECK   $<
      cmd_clang_tools = $(PYTHON3) $(srctree)/scripts/clang-tools/run-clang-tools.py $@ $<

clang-tidy clang-analyzer: $(extmod_prefix)compile_commands.json
	$(call cmd,clang_tools)

kernelrelease:
	@$(filechk_kernel.release)

kernelversion:
	@echo $(KERNELVERSION)

image_name:
	@echo $(KBUILD_IMAGE)

PHONY += run-command
run-command:
	$(Q)$(KBUILD_RUN_COMMAND)

quiet_cmd_rmfiles = $(if $(wildcard $(rm-files)),CLEAN   $(wildcard $(rm-files)))
      cmd_rmfiles = rm -rf $(rm-files)

# read saved command lines for existing targets
existing-targets := $(wildcard $(sort $(targets)))

-include $(foreach f,$(existing-targets),$(dir $(f)).$(notdir $(f)).cmd)

endif # config-build
endif # mixed-build
endif # need-sub-make

PHONY += FORCE
FORCE:

# Declare the contents of the PHONY variable as phony.  We keep that
# information in a variable so we can use it in if_changed and friends.
.PHONY: $(PHONY)
