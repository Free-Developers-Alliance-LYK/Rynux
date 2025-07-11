
# disable overflow check, it would create rela code
KBUILD_RUSTFLAGS := $(KBUILD_RUSTFLAGS) -Crelocation-model=static   \
             -Copt-level=s -g  \
             -Cforce-unwind-tables=n \
			 -Coverflow-checks=n

# this code may run with the MMU off so disable unaligned ac  cesses
RUSTFLAGS_map_range.o += -Ctarget-feature=+strict-align

hostprogs := relacheck

quiet_cmd_piobjcopy = $(quiet_cmd_objcopy)
      cmd_piobjcopy = $(cmd_objcopy) && $(obj)/relacheck $(@) $(<)

$(obj)/%.pi.o: OBJCOPYFLAGS := --prefix-symbols=__pi_ \
                   --remove-section=.note.gnu.property 
$(obj)/%.pi.o: $(obj)/%.o $(obj)/relacheck FORCE
	$(call if_changed,piobjcopy)

# ensure that all the lib- code ends up as __init code and data
$(obj)/lib-%.pi.o: OBJCOPYFLAGS += --prefix-alloc-sections=.  init

$(obj)/lib-%.o: $(srctree)/lib/%.c FORCE
	$(call if_changed_rule,cc_o_c)

obj-y := map_range.pi.o
#map_range.pi.o
extra-y := $(patsubst %.pi.o,%.o,$(obj-y))

