// SPDX-License-Identifier: GPL-2.0

//! The `Linker` crate.
//!
//! This crate contains some constants needed for image link layout, and will
//! also be used to generate layout.h for compilation of vmrynux.lds

#![no_std]

pub mod size;

pub mod discard;

pub mod arch;

