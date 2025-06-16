KCONFIG_AUTO_CONF := $(objtree)/include/config/auto.conf

-include $(objtree)/include/config/auto.conf

RUST_KCONFIG ?= $(objtree)/include/generated/rustc_cfg

BUILD_RUSTFLAGS := $(shell cat $(RUST_KCONFIG))
