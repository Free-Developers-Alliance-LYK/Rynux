// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate.
//!
//! This crate contains the kernel APIs that have been ported or wrapped for
//! usage by Rust code in the kernel and is shared by all of them.
//!
//! In other words, all the rest of the Rust code in the kernel (e.g. kernel
//! modules written in Rust) depends on [`core`], [`alloc`] and this crate.

#![no_std]
#![feature(naked_functions)]


// Allow proc-macros to refer to `::kernel` inside the `kernel` crate (this crate).
//extern crate self as kernel;

pub use macros;
pub use static_assertions;
pub use const_format;
pub use bitflags;
pub use tock_registers;


pub mod klib;
pub mod arch;
pub mod size;
pub mod vmrynux;
pub mod mm;
pub mod sync;
pub mod linkage;
pub mod prelude;
pub mod schedule;
pub mod init;
pub mod types;
pub mod global_sym;

#[cfg(not(any(testlib, test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
        loop {}
}
