// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate.
//!
//! This crate contains the kernel APIs that have been ported or wrapped for
//! usage by Rust code in the kernel and is shared by all of them.
//!
//! In other words, all the rest of the Rust code in the kernel (e.g. kernel
//! modules written in Rust) depends on [`core`], [`alloc`] and this crate.

#![cfg_attr(not(test), no_std)]
#![feature(naked_functions)]
#![feature(derive_coerce_pointee)]

#[cfg(target_endian = "big")]
compile_error!("This crate only supports little endian platforms!");

// Allow proc-macros to refer to `::kernel` inside the `kernel` crate (this crate).
//extern crate self as kernel;

pub use macros;
pub use static_assertions;
pub use const_format;
pub use bitflags;
pub use tock_registers;
pub use fdt;

pub mod arch;

cfg_if! {
    if #[cfg(not(test))] {
        // vmrynux and global_sym only used in real image
        pub mod vmrynux;
        pub mod global_sym;
        // init only avaliable in real image
        pub mod init;
    }
}

pub mod klib;
pub mod cpu;
pub mod alloc;
pub mod sync;
pub mod size;
pub mod mm;
pub mod linkage;
pub mod prelude;
pub mod schedule;
pub mod types;
pub mod compiler;

#[cfg(not(any(testlib, test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
        loop {}
}
