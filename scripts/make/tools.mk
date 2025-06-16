TOOLS_DIR := $(srctree)/tools
TOOLS_BUILD_DIR := $(objtree)/tools/build

KBUILD_DIR := $(TOOLS_DIR)/kbuild-standalone
KBUILD_BUILD_DIR := $(TOOLS_BUILD_DIR)/kbuild-standalone

tools: kbuild

kbuild:
	@if [ ! -d $(KBUILD_BUILD_DIR) ]; then mkdir -p $(KBUILD_BUILD_DIR); fi
	$(MAKE) -C $(KBUILD_DIR) -f Makefile.sample O=$(KBUILD_BUILD_DIR) -j
	@if [ ! -f $(KBUILD_BUILD_DIR)/kconfig/conf ] || [ ! -f $(KBUILD_BUILD_DIR)/kconfig/mconf ]; then \
		echo "Error: kbuild-standalone build failed!"; exit 1; \
	fi
