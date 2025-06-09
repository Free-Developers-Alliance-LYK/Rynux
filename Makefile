# SET SUBARCH
this-makefile := $(lastword $(MAKEFILE_LIST))
abs_srctree := $(realpath $(dir $(this-makefile)))
srctree = $(abs_srctree)

include $(srctree)/scripts/subarch.include

#ARCH := $(SUBARCH)
# Currently we only support arm64
ARCH := arm64
SRCARCH := $(ARCH)

O ?= $(srctree)

# if O start with '/'
ifeq ($(shell echo $(O) | cut -c1),/)
  $(info "with /")
  objtree := $(realpath $(O))
else
  $(info "no /")
  objtree := $(srctree)/$(O)
endif

$(info srctree = $(srctree))
$(info O = $(O))
$(info objtree = $(objtree))

.PHONY: all config tools build clean mrproper

TOOLS_DIR := $(srctree)/tools
TOOLS_BUILD_DIR := $(objtree)/tools/build

KBUILD_DIR := $(TOOLS_DIR)/kbuild-standalone
KBUILD_BUILD_DIR := $(TOOLS_BUILD_DIR)/kbuild-standalone


KCONFIG_CONFIG  ?= $(objtree)/.config

all: build

menuconfig: tools
		cd $(O) && SRCARCH=$(SRCARCH) srctree=$(srctree) $(KBUILD_BUILD_DIR)/kconfig/mconf $(srctree)/Kconfig

tools: kbuild

kbuild:
	@if [ ! -d $(KBUILD_BUILD_DIR) ]; then mkdir -p $(KBUILD_BUILD_DIR); fi
	$(MAKE) -C $(KBUILD_DIR) -f Makefile.sample O=$(KBUILD_BUILD_DIR) -j
	@if [ ! -f $(KBUILD_BUILD_DIR)/kconfig/conf ] || [ ! -f $(KBUILD_BUILD_DIR)/kconfig/mconf ]; then \
		echo "Error: kbuild-standalone build failed!"; exit 1; \
	fi

build: $(KCONFIG_CONFIG)

$(KCONFIG_CONFIG):
	@echo >&2 '***'
	@echo >&2 '*** Configuration file "$@" not found!'
	@echo >&2 '***'
	@echo >&2 '*** Please run some configurator (e.g. "make menuconfig")'
	@echo >&2 '***'
	@/bin/false

clean:
	@echo "Cleaning build artifacts in $(O) (keep config)..."
	#Clean tools
	rm -rf $(KBUILD_BUILD_DIR)

mrproper: clean
	@echo "Removing configuration and extra files in $(O)..."
	rm -f $(objtree)/.config
	rm -f $(objtree)/.config.old
	rm -rf $(objtree)/include
