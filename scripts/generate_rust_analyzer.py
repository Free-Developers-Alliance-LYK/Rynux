#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""generate_rust_analyzer - Generates the `rust-project.json` file for `rust-analyzer`.
"""

import argparse
import json
import logging
import os
import pathlib
import subprocess
import sys

def args_crates_cfgs(cfgs):
    crates_cfgs = {}
    for cfg in cfgs:
        crate, vals = cfg.split("=", 1)
        crates_cfgs[crate] = vals.replace("--cfg", "").split()

    return crates_cfgs

def generate_crates(srctree, objtree, sysroot_src, external_src, cfgs, core_edition):
    # Generate the configuration list.
    cfg = []
    with open(objtree / "include" / "generated" / "rustc_cfg") as fd:
        for line in fd:
            line = line.replace("--cfg=", "")
            line = line.replace("\n", "")
            cfg.append(line)

    # Now fill the crates list -- dependencies need to come first.
    #
    # Avoid O(n^2) iterations by keeping a map of indexes.
    crates = []
    crates_indexes = {}
    crates_cfgs = args_crates_cfgs(cfgs)

    def append_crate(display_name, root_module, deps, cfg=[], is_workspace_member=True, is_proc_macro=False, edition="2024"):
        crate = {
            "display_name": display_name,
            "root_module": str(root_module),
            "is_workspace_member": is_workspace_member,
            "is_proc_macro": is_proc_macro,
            "deps": [{"crate": crates_indexes[dep], "name": dep} for dep in deps],
            "cfg": cfg,
            "edition": edition,
            "env": {
                "RUST_MODFILE": "This is only for rust-analyzer"
            }
        }
        if is_proc_macro:
            proc_macro_dylib_name = subprocess.check_output(
                [os.environ["RUSTC"], "--print", "file-names", "--crate-name", display_name, "--crate-type", "proc-macro", "-"],
                stdin=subprocess.DEVNULL,
            ).decode('utf-8').strip()
            crate["proc_macro_dylib_path"] = f"{objtree}/proc_macros/{proc_macro_dylib_name}"
        crates_indexes[display_name] = len(crates)
        crates.append(crate)

    def append_sysroot_crate(
        display_name,
        deps,
        cfg=[],
        edition="2024",
    ):
        append_crate(
            display_name,
            sysroot_src / display_name / "src" / "lib.rs",
            deps,
            cfg,
            is_workspace_member=False,
            edition=edition,
        )

    # NB: sysroot crates reexport items from one another so setting up our transitive dependencies
    # here is important for ensuring that rust-analyzer can resolve symbols. The sources of truth
    # for this dependency graph are `(sysroot_src / crate / "Cargo.toml" for crate in crates)`.
    append_sysroot_crate("core", [], cfg=crates_cfgs.get("core", []), edition=core_edition)
    append_sysroot_crate("alloc", ["core"])
    append_sysroot_crate("std", ["alloc", "core"])
    append_sysroot_crate("proc_macro", ["core", "std"])
    append_crate(
            "compiler_builtins",
            srctree / "third_lib" / "compiler_builtins.rs",
            [],
    )

    # Prepare proccmacro third lib crates
    def append_procmacros_crate(
        display_name,
        subdir,
        deps,
        cfg,
        is_proc_macro=False,
    ):
        append_crate(
            display_name,
            srctree / "proc_macros" / subdir / "src" / "lib.rs",
            deps,
            cfg,
            is_workspace_member=True,
            is_proc_macro=is_proc_macro,
        )

    append_procmacros_crate(
        "unicode_ident",
        "unicode-ident",
        deps=[],
        cfg=[],
    )

    append_procmacros_crate(
        "unicode_xid",
        "unicode-xid",
        deps=[],
        cfg=[],
    )

    append_procmacros_crate (
            "proc_macro2",
            "proc-macro2",
            deps=["std", "proc_macro", "unicode_ident"],
            cfg = [
                'feature="default"',
                'feature="proc-macro"',
                'wrap_proc_macro'
                ],
            is_proc_macro=True,
    )

    append_procmacros_crate (
            "quote",
            "quote",
            deps=["proc_macro2", "std"],
            cfg = [
                'feature="proc-macro"',
            ],
    )

    append_procmacros_crate (
            "syn",
            "syn",
            deps=["proc_macro2", "quote", "unicode_ident", "std"],
            cfg = [
                'feature="derive"',
                'feature="parsing"',
                'feature="printing"',
                'feature="proc_macro"',
                'feature="full"',
                'feature="clone_impls"',
                ],
    )

    append_procmacros_crate(
        "const_format_proc_macros",
        "const_format_proc_macros",
        deps=["std", "proc_macro2", "unicode_xid", "quote"],
        cfg=[],
        is_proc_macro=True,
    )

    append_procmacros_crate(
        "static_assertions_proc_macros",
        "static_assertions_proc_macros",
        deps=["std", "proc_macro"],
        cfg=[],
        is_proc_macro=True,
    )

    ## add macros
    append_crate(
        "macros",
        srctree / "proc_macros" / "macros" / "lib.rs",
        deps=["std", "proc_macro", "syn", "quote"],
        cfg=cfg,
        is_workspace_member=True,
        is_proc_macro=True,
    )

    def append_third_lib_crate(
        display_name,
        subdir,
        deps,
        cfg,
    ):
        deps.append("core");
        deps.append("compiler_builtins");
        append_crate(
            display_name,
            srctree / "third_lib" / subdir / "src" / "lib.rs",
            deps,
            cfg,
        )

    append_third_lib_crate(
        "const_format",
        "const_format",
        deps=["const_format_proc_macros"],
        cfg=[],
    )

    append_third_lib_crate(
        "static_assertions",
        "static_assertions-1.1.0",
        deps=["static_assertions_proc_macros"],
        cfg=[],
    )

    append_third_lib_crate(
        "bitflags",
        "bitflags",
        deps=[],
        cfg=[],
    )

    append_third_lib_crate(
        "tock_registers",
        "tock-registers-0.10.0",
        deps=[],
        cfg=['feature="register_types"'],
    )

    append_third_lib_crate(
        "fdtree_rs",
        "fdtree-rs",
        deps=[],
        cfg=[],
    )

    append_third_lib_crate(
        "cfg_if",
        "cfg-if-1.0.3",
        deps=[],
        cfg=[],
    )

    ## add kernel
    append_crate(
            "kernel",
            srctree / "kernel" / "lib.rs",
            deps=["core", "macros", "bitflags", "tock_registers",
                  "fdtree_rs", "const_format", "static_assertions", "cfg_if"],
            cfg=cfg,
    )
    crates[-1]["env"]["OBJTREE"] = str(objtree.resolve(True))
    crates[-1]["source"] = {
       "include_dirs": [
           str(srctree / "kernel"),
           str(objtree / "kernel")
       ],
       "exclude_dirs": [],
    }

    def is_root_crate(build_file, target):
        try:
            return f"{target}.o" in open(build_file).read()
        except FileNotFoundError:
            return False


    # Then, the rest outside of `kernel/`.
    #
    # We explicitly mention the top-level folders we want to cover.
    extra_dirs = map(lambda dir: srctree / dir, ("arch", "drivers", "init"))
    if external_src is not None:
        extra_dirs = [external_src]
    for folder in extra_dirs:
        for path in folder.rglob("*.rs"):
            logging.info("Checking %s", path)
            name = path.name.replace(".rs", "")

            # Skip those that are not crate roots.
            if not is_root_crate(path.parent / "Makefile", name) and \
               not is_root_crate(path.parent / "Kbuild", name):
                continue

            logging.info("Adding %s", name)
            append_crate(
                name,
                path,
                ["core", "kernel"],
                cfg=cfg,
            )

    return crates

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose', '-v', action='store_true')
    parser.add_argument('--cfgs', action='append', default=[])
    parser.add_argument("core_edition")
    parser.add_argument("srctree", type=pathlib.Path)
    parser.add_argument("objtree", type=pathlib.Path)
    parser.add_argument("sysroot", type=pathlib.Path)
    parser.add_argument("sysroot_src", type=pathlib.Path)
    parser.add_argument("exttree", type=pathlib.Path, nargs="?")
    args = parser.parse_args()

    logging.basicConfig(
        format="[%(asctime)s] [%(levelname)s] %(message)s",
        level=logging.INFO if args.verbose else logging.WARNING
    )

    # Making sure that the `sysroot` and `sysroot_src` belong to the same toolchain.
    assert args.sysroot in args.sysroot_src.parents

    rust_project = {
        "crates": generate_crates(args.srctree, args.objtree, args.sysroot_src, args.exttree, args.cfgs, args.core_edition),
        "sysroot": str(args.sysroot),
    }

    json.dump(rust_project, sys.stdout, sort_keys=True, indent=4)

if __name__ == "__main__":
    main()
