UIMAGE_ARCH ?= $(SRCARCH)
UIMAGE_TYPE ?= kernel
UIMAGE_LOADADDR ?= arch_must_set_this
UIMAGE_ENTRYADDR ?= $(UIMAGE_LOADADDR)

define mk_uimage
	$(call run_cmd,mkimage,\
		-A $(UIMAGE_ARCH) -O linux -T $(UIMAGE_TYPE) -C none \
		-a $(UIMAGE_LOADADDR) -e $(UIMAGE_ENTRYADDR) \
		-d $(1) $(2))
endef
