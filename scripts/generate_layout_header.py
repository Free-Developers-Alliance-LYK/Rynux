#!/usr/bin/env python3
from elftools.elf.elffile import ELFFile
import sys, struct, pathlib

OBJ  = sys.argv[1]          # linker.o
OUT  = sys.argv[2]          # xxx.h

with open(OBJ, 'rb') as f:
    elf = ELFFile(f)
    rodata = elf.get_section_by_name('.rodata')
    rodata_bytes = rodata.data() if rodata else b''

    syms = {}
    for sec in elf.iter_sections():
        if not sec['sh_type'] == 'SHT_SYMTAB':
            continue
        for sym in sec.iter_symbols():
            name = sym.name
            if name.startswith('SZ_'):
                addr = sym['st_value']
                # symbol值存放在 rodata，相对文件开头偏移= rodata['sh_offset']+addr-rodata['sh_addr']
                off = (addr - rodata['sh_addr']) + rodata['sh_offset']
                raw = rodata_bytes[addr - rodata['sh_addr']: addr - rodata['sh_addr'] + elf.elfclass // 8]
                # 默认小端
                val = int.from_bytes(raw, 'little')
                syms[name] = val

# 写头文件
with open(OUT, 'w') as h:
    h.write("/* Auto-generated; DO NOT EDIT */\n")
    guard = "__SIZES_GENERATED_H__"
    h.write(f"#ifndef {guard}\n#define {guard}\n\n")
    for k in sorted(syms):
        h.write(f"#define {k:<8}  0x{syms[k]:X}\n")
    h.write(f"\n#endif /* {guard} */\n")

print(f"Wrote {OUT} with {len(syms)} constants.")

