#!/usr/bin/env python3
from elftools.elf.elffile import ELFFile
import sys

# 需要提取的变量及类型："name": "type"
# type: "int" 或 "str"
INTERESTED_VARS = {
    "SZ_4K": "int",
    "PAGE_SIZE": "int",
    "KIMAGE_VADDR": "int",
    "SEGMENT_ALIGN": "int",
    "INIT_IDMAP_DIR_SIZE":"int",
    "INIT_IDMAP_DIR_PAGES": "int",
    "EXPORT_DISCARDS": "str",
    "EXPORT_HEAD_TEXT": "str",
    "EXPORT_IRQENTRY_TEXT": "str",
    "EXPORT_SOFTIRQENTRY_TEXT": "str",
    "EXPORT_ENTRY_TEXT": "str",
    "EXPORT_TEXT_TEXT": "str",
    "EXPORT_SCHED_TEXT": "str",
    "EXPORT_LOCK_TEXT": "str",
    "EXPORT_KPROBES_TEXT": "str",
    "EXPORT_INIT_TEXT_SECTION": "str",
    "EXPORT_EXIT_TEXT": "str",
}

def extract_int(rodata_bytes, sym, rodata, elfclass):
    addr = sym['st_value']
    offset = addr - rodata['sh_addr']
    size = elfclass // 8
    raw = rodata_bytes[offset:offset+size]
    return int.from_bytes(raw, 'little')

def extract_str(f, sym, rodata):
    addr = sym['st_value']
    file_offset = rodata['sh_offset'] + (addr - rodata['sh_addr'])
    f.seek(file_offset)
    raw = f.read(512)   # 512 字节足够一般字符串用
    s = raw.split(b'\0', 1)[0].decode('utf-8', errors='replace')
    return s

def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input.o> <output.h>")
        sys.exit(1)
    OBJ, OUT = sys.argv[1], sys.argv[2]

    found = {}

    with open(OBJ, 'rb') as f:
        elf = ELFFile(f)
        rodata = elf.get_section_by_name('.rodata')
        if not rodata:
            print("No .rodata section found.")
            sys.exit(2)
        rodata_bytes = rodata.data()
        elfclass = elf.elfclass

        for sec in elf.iter_sections():
            if sec['sh_type'] != 'SHT_SYMTAB':
                continue
            for sym in sec.iter_symbols():
                name = sym.name
                if name in INTERESTED_VARS:
                    typ = INTERESTED_VARS[name]
                    if typ == "int":
                        val = extract_int(rodata_bytes, sym, rodata, elfclass)
                    elif typ == "str":
                        val = extract_str(f, sym, rodata)
                    else:
                        continue  # 未知类型，跳过
                    out_name = name
                    if name.startswith("EXPORT_"):
                        out_name = name[len("EXPORT_"):]
                    found[out_name] = (typ, val)

    # 写头文件
    with open(OUT, 'w') as h:
        h.write("/* Auto-generated; DO NOT EDIT */\n")
        guard = "__GENERATED_VARS_H__"
        h.write(f"#ifndef {guard}\n#define {guard}\n\n")
        for name in sorted(found):
            typ, val = found[name]
            if typ == "int":
                h.write(f"#define {name:<16} 0x{val:X}\n")
            elif typ == "str":
                val_escaped = val.replace('\n', '\\\n')
                h.write(f"#define {name:<16} {val_escaped}\n")
        h.write(f"\n#endif /* {guard} */\n")

    print(f"Wrote {OUT} with {len(found)} constants.")

if __name__ == "__main__":
    main()
