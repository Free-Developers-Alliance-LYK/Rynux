include scripts/make/rustflags.mk
include scripts/make/build_doc.mk
include scripts/make/uimage.mk

_cargo_build:
	@printf " $(GREEN_C)Building$(END_C) kernel Arch: $(ARCH)\n"
	$(call cargo_build)
	@cp $(rust_elf) $(OUT_ELF)

$(OUT_BIN): _cargo_build $(OUT_ELF)
	@printf " create bin\n"
	$(call run_cmd,$(OBJCOPY),$(OUT_ELF) --strip-all -O binary $@)

$(OUT_UIMG): $(OUT_BIN)
	@printf " create image\n"
	 $(call mk_uimage, $< ,$@)

.PHONY: _cargo_build
