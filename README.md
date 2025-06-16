# Rynux
Rustix is a modern, secure, and high-performance Linux kernel reimagined and rewritten in Rust. This project aims to leverage Rust’s safety guarantees and concurrency features to build a reliable and efficient operating system kernel from the ground up.

## Features

- Now only supports arm64 architecture.


## Build
it simlar to the Linux kernel build process, you can use `make` to compile the kernel. The build system is designed to be modular and extensible, allowing for easy customization and configuration.

### Config

To configure the kernel, you can use the `menuconfig` interface, which provides a user-friendly way to select options and features. The configuration is stored in a `.config` file, which can be modified directly or through the menu interface.

```bash
make ARCH=arm64 menuconfig
```




## Related Projects
[kbuild-standalone](https://github.com/WangNan0/kbuild-standalone): A standalone Linux kernel config and build tools
