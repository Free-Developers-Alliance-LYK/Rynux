// SPDX-License-Identifier: GPL-2.0

//! The `klib` crate.
//!
//! This crate contains the basic tools APIS
//! usage by Rust code in the kernel and is shared by all of them.
//!
#![no_std]

/// Converts a string literal to a byte array.
#[macro_export]
macro_rules! const_str_to_u8_array_with_null {
    ($s:expr) => {{
        const S: &str = $s;
        const fn to_array<const N: usize>(s: &str) -> [u8; N] {
            let bytes = s.as_bytes();
            let mut arr = [0u8; N];
            let mut i = 0;
            while i < bytes.len() {
                arr[i] = bytes[i];
                i += 1;
            }
            arr[bytes.len()] = 0; //NULL
            arr
        }
        to_array::<{ S.len()+1 }>(S)
    }};
}

pub(crate) mod cfg_if;

pub mod math;
pub mod string;
