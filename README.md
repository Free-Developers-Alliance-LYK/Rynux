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
