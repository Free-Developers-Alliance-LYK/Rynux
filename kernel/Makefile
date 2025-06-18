# SPDX-License-Identifier: GPL-2.0

always-y += uapi/uapi_generated.rs
obj-y += uapi.o

bindgen_c_flags = $(c_flags)

bindgen_c_flags_lto = $(bindgen_c_flags)

bindgen_c_flags_final = $(bindgen_c_flags_lto) -D__BINDGEN__

quiet_cmd_bindgen = BINDGEN $@
      cmd_bindgen = \
	$(BINDGEN) $< $(bindgen_target_flags) \
		--use-core --with-derive-default --ctypes-prefix core::ffi --no-layout-tests \
		--no-debug '.*' --enable-function-attribute-detection \
		-o $@ -- $(bindgen_c_flags_final) -DMODULE \
		$(bindgen_target_cflags) $(bindgen_target_extra)

$(obj)/uapi/uapi_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^#|^$$' $(src)/bindgen_parameters)
$(obj)/uapi/uapi_generated.rs: $(src)/uapi/uapi_helper.h \
    $(src)/bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)

rust-analyzer:
	$(Q)$(srctree)/scripts/generate_rust_analyzer.py \
		--cfgs='core=$(core-cfgs)' --cfgs='alloc=$(alloc-cfgs)' \
		$(realpath $(srctree)) $(realpath $(objtree)) \
		$(rustc_sysroot) $(RUST_LIB_SRC) $(KBUILD_EXTMOD) > \
		$(if $(KBUILD_EXTMOD),$(extmod_prefix),$(objtree))/rust-project.json

$(obj)/uapi.o: $(src)/uapi/lib.rs \
    $(obj)/compiler_builtins.o \
    $(obj)/uapi/uapi_generated.rs FORCE
	+$(call if_changed_rule,rustc_library)

