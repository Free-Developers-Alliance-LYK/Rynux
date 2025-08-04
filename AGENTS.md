# Agent Instructions

This repository provides a Rust-based ARM64 kernel. Follow these steps to prepare the environment and run basic checks before submitting changes.

## Environment Setup
- Ubuntu or similar Linux system
- Install required tools: `clang-17`, `lld-17`, `flex`, `bison`, `qemu-system-aarch64`, and a recent Rust toolchain (rustc is 1.88)

## Build and Test
1. Verify toolchain availability:
   ```bash
   make LLVM=-17 O=build_dir rustavailable
   ```
2. Generate default configuration:
   ```bash
   make LLVM=-17 O=build_dir defconfig
   ```
3. Build the kernel image:
   ```bash
   make LLVM=-17 O=build_dir
   ```
4. Run the image with QEMU:
   ```bash
   qemu-system-aarch64 -M virt -cpu cortex-a57 -smp 1 -m 4G \
      -kernel build_dir/arch/arm64/boot/Image -nographic \
      -append "earlycon root=/dev/ram rdinit=/bin/sh"
   ```

Always run the build step (#3) to ensure the repository compiles before committing.
