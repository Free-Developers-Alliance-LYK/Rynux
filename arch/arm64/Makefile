#
# arch/arm64/Makefile
#
# This file is included by the global makefile so that you can add your own
# architecture-specific flags and dependencies.
#
# This file is subject to the terms and conditions of the GNU General Public
# License.  See the file "COPYING" in the main directory of this archive
# for more details.
#
# Copyright (C) 1995-2001 by Russell King

LDFLAGS_vmrynux	:=--no-undefined -X --pic-veneer

ifeq ($(CONFIG_RELOCATABLE), y)
  # Pass --no-apply-dynamic-relocs to restore pre-binutils-2.27 behaviour
  # for relative relocs, since this leads to better Image compression
  # with the relocation offsets always being zero.
  LDFLAGS_vmrynux     += -shared -Bsymbolic -z notext \
              $(call ld-option, --no-apply-dynamic-relocs)
endif

ifeq ($(CONFIG_ARM64_ERRATUM_843419),y)
  ifeq ($(CONFIG_ARM64_LD_HAS_FIX_ERRATUM_843419),y)
LDFLAGS_vmrynux	+= --fix-cortex-a53-843419
  endif
endif
cc_has_k_constraint := $(call try-run,echo				\
	'int main(void) {						\
		asm volatile("and w0, w0, %w0" :: "K" (4294967295));	\
		return 0;						\
	}' | $(CC) -S -x c -o "$$TMP" -,,-DCONFIG_CC_HAS_K_CONSTRAINT=1)

ifeq ($(CONFIG_BROKEN_GAS_INST),y)
$(warning Detected assembler with broken .inst; disassembly will be unreliable)
endif

KBUILD_RUSTFLAGS += --target=aarch64-unknown-none -Ctarget-feature="-neon"

$(info "KBUILD_RUSTFLAGS = $(KBUILD_RUSTFLAGS)")

# Same as above, prefer ELF but fall back to linux target if needed.
KBUILD_LDFLAGS	+= -EL $(call ld-option, -maarch64elf, -maarch64linux -z norelro)
KBUILD_LDFLAGS	+= -z norelro

ifeq ($(CONFIG_CPU_BIG_ENDIAN), y)
  KBUILD_CPPFLAGS += -mbig-endian
  # Prefer the baremetal ELF build target, but not all toolchains include
  # it so fall back to the standard linux version if needed.
  KBUILD_LDFLAGS  += -EB $(call ld-option, -maarch64elfb, -maarch64linuxb -z norelro)
  UTS_MACHINE := aarch64_be
else
  KBUILD_CPPFLAGS += -mlittle-endian
  # Same as above, prefer ELF but fall back to linux target if needed.
  KBUILD_LDFLAGS  += -EL $(call ld-option, -maarch64elf, -maarch64linux -z norelro)
  UTS_MACHINE := aarch64
endif

boot		:= arch/arm64/boot

BOOT_TARGETS	:= Image vmlinuz.efi image.fit

PHONY += $(BOOT_TARGETS)

KBUILD_IMAGE	:= $(boot)/Image.gz

all:	$(notdir $(KBUILD_IMAGE))

image.fit: dtbs

vmlinuz.efi image.fit: Image
$(BOOT_TARGETS): vmrynux
	$(Q)$(MAKE) $(build)=$(boot) $(boot)/$@

Image.%: Image
	$(Q)$(MAKE) $(build)=$(boot) $(boot)/$@

DEFAULT_KBUILD_IMAGE = $(boot)/Image

install: KBUILD_IMAGE := $(DEFAULT_KBUILD_IMAGE)
install zinstall:
	$(call cmd,install)

#archprepare:
#	$(Q)$(MAKE) $(build)=arch/arm64/tools kapi

# We need to generate vdso-offsets.h before compiling certain files in kernel/.
# In order to do that, we should use the archprepare target, but we can't since
# asm-offsets.h is included in some files used to generate vdso-offsets.h, and
# asm-offsets.h is built in prepare0, for which archprepare is a dependency.
# Therefore we need to generate the header after prepare0 has been made, hence
# this hack.
#prepare: vdso_prepare
#vdso_prepare: prepare0
#	$(Q)$(MAKE) $(build)=arch/arm64/kernel/vdso \
#	include/generated/vdso-offsets.h arch/arm64/kernel/vdso/vdso.so

#vdso-install-y				+= arch/arm64/kernel/vdso/vdso.so.dbg

define archhelp
  echo  '* Image.gz      - Compressed kernel image (arch/$(ARCH)/boot/Image.gz)'
  echo  '  Image         - Uncompressed kernel image (arch/$(ARCH)/boot/Image)'
  echo  '  image.fit     - Flat Image Tree (arch/$(ARCH)/boot/image.fit)'
  echo  '  install       - Install kernel (compressed if COMPRESSED_INSTALL set)'
  echo  '  zinstall      - Install compressed kernel'
  echo  '                  Install using (your) ~/bin/installkernel or'
  echo  '                  (distribution) /sbin/installkernel or'
  echo  '                  install to $$(INSTALL_PATH) and run lilo'
endef
