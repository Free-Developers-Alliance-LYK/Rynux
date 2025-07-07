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

pub mod klib;
pub mod arch;
pub mod size;
pub mod vmrynux;
pub mod mm;
pub mod linkage;
pub mod prelude;

pub use static_assertions as static_assertions;
pub use const_format as const_format;

#[cfg(not(any(testlib, test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
        loop {}
}
