## Developer Guide

## How to use rustfmt

To ensure code style consistency, we use `rustfmt` to format the Rust code.

You can use the following command to format the code:


### rustfmtcheck
only check the format without modifying files:

```bash
make LLVM=1 rustfmtcheck
```

### rustfmt
format and modify files:

```bash
make LLVM=1 rustfmt
```

## How to use rustanalyzer

We use `rust-analyzer` as the language server for Rust.
You can use the following command to generate the configuration file for `rust-analyzer`:

It requires kernel `.config` file generated, so need to run `make menuconfig` first.

```bash
make LLVM=1  O=build_dir rust-analyzer
```

This will generate a `rust-project.json` file in the build directory of the kernel.

copy this file to the root directory of the kernel:

```bash
cp build_dir/rust-project.json .
```

### About cfg test

`rust-analyzer` will default enable the `test` cfg flag, which may cause some
problems when analyzing the kernel code.

To disable this flag, you can modify the `coc-settings.json`

```json
{
 "rust-analyzer.cfg.setTest": false,
}
```

## How to Import a thirdlib

I will use an example as a reference here

### Prepare

It is necessary to manually check whether the third-party library has other
dependencies. We need to start building from the lowest level dependent library.

We can confirm this from the target library's Cargo.toml file

For example
```toml
[dependencies]
bitflags = "1.3.2"
```

We also need to confirm the features of the third-party libraries we need.

### cargo-download Install

```bash
cargo install cargo-download
```

### Download

Enter `third_lib` directory, and use `cargo download` to download the 
library you want to import

```bash
cd third_lib
cargo download -x tock-registers=0.10.0
```
now, we have a `tock-registers-0.10.0` directory in `third_lib` directory.

### Add Makefile

```bash
vim third_lib/Kbuild
```

Here, we can skip or add some flags to the compilation process.

```makefile
# ----------------- tock-registers ------------------------
$(obj)/tock_registers.o: private skip_flags = -Wunreachable_pub -Wmissing_docs
$(obj)/tock_registers.o: private rustc_target_flags = -Aelided-lifetimes-in-paths
$(obj)/tock_registers.o: $(src)/tock-registers-0.10.0/src/lib.rs  $(base_libs) FORCE
	+$(call if_changed_rule,rustc_library)

obj-y 			+= tock_registers.o

```

### Add dependency to kernel

```bash
vim Kbuild
```

```makefile
third_lib= --extern const_format --extern static_assertions --extern bitflags \
 --extern tock_registers
```

### Use it in kernel

```bash
vim kernel/lib.rs
```

```rust
pub use tock_registers;
```

### Add to rust analysis

change `scripts/gen_rust_analyzer_config.py`

