# Set default
BUILD_RUSTFLAGS += -Cpanic=abort -Cembed-bitcode=no -Clto=no -Cforce-unwind-tables=no -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=no -Dclippy::float_arithmetic

ifdef CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE
	BUILD_RUSTFLAGS += -Copt-level=2
else ifdef CONFIG_CC_OPTIMIZE_FOR_SIZE
	BUILD_RUSTFLAGS += -Copt-level=s
endif

# SET toolchain
ifeq ($(CONFIG_ARM64),y)
	RUSTFLAGS += -Ctarget-feature="-neon"
	ifneq ($(CONFIG_UNWIND_TABLES),y)
		BUILD_RUSTFLAGS += -Cforce-unwind-tables=n
	else
		BUILD_RUSTFLAGS += -Cforce-unwind-tables=y -Zuse-sync-unwind=n
	endif
endif

$(if $(V), $(info BUILD_RUSTFLAGS: "$(BUILD_RUSTFLAGS)"))


LD := rust-lld -flavor gnu
OBJDUMP ?= rust-objdump -d --print-imm-hex --x86-asm-syntax=intel

ifeq ($(CONFIG_ARM64),y)
	OBJCOPY ?= rust-objcopy --binary-architecture=$(SRCARCH)
endif
