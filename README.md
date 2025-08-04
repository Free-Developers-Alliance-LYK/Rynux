# Rynux Quick Start Guide
Rustix is a modern, secure, and high-performance Linux kernel reimagined and rewritten in Rust. This project aims to leverage Rust’s safety guarantees and concurrency features to build a reliable and efficient operating system kernel from the ground up.


## Features

- Now only supports arm64 architecture.

## Build Check
it simlar to the Linux kernel build process, you can use `make` to compile the kernel. The build system is designed to be modular and extensible, allowing for easy customization and configuration.

To easily check whether the requirements are met, the following target can be used:
```shell
make LLVM=-17 O=build_dir rustavailable
```
This triggers the same logic used by Kconfig to determine whether `RUST_IS_AVAILABLE` should be enabled; it also explains why not if that is the case.

### Config

To configure the kernel, you can use the `menuconfig` interface, which provides a user-friendly way to select options and features. The configuration is stored in a `.config` file, which can be modified directly or through the menu interface.

```bash
make LLVM=-17  O=build_dir menuconfig
```

### Build Image

```bash
make LLVM=-17 O=build_dir
```

### Run Image o qemu

```bash
qemu-system-aarch64 -M virt -cpu cortex-a57 -smp 1 -m 4G   -kernel build_dir/arch/arm64/boot/Image  -nographic    -append " earlycon root=/dev/ram rdinit=/bin/sh "
```


## Develop Guide

### How to Import a thirdlib

I will use an example as a reference here

#### Prepare

It is necessary to manually check whether the third-party library has other
dependencies. We need to start building from the lowest level dependent library.

We can confirm this from the target library's Cargo.toml file

For example
```toml
[dependencies]
bitflags = "1.3.2"
```

We also need to confirm the features of the third-party libraries we need.

#### Download

Enter `third_lib` directory, and use `cargo download` to download the 
library you want to import

```bash
cd third_lib
cargo download -x tock-registers=0.10.0
```
now, we have a `tock-registers-0.10.0` directory in `third_lib` directory.

#### Add Makefile

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

#### Add dependency to kernel

```bash
vim Kbuild
```

```makefile
third_lib= --extern const_format --extern static_assertions --extern bitflags \
 --extern tock_registers
```

#### Use it in kernel

```bash
vim kernel/lib.rs
```

```rust
pub use tock_registers;
```
