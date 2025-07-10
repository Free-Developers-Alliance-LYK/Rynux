//! Early put in qemu

use crate::macros::section_init_text;

#[allow(dead_code)]
#[section_init_text]
fn uart_put_hex(byte: u8) {
    // 高4位
    let hi = (byte >> 4) & 0xf;
    // 低4位
    let lo = byte & 0xf;
    early_uart_putchar(nibble_to_ascii(hi));
    early_uart_putchar(nibble_to_ascii(lo));
}

#[section_init_text]
fn nibble_to_ascii(n: u8) -> u8 {
    match n {
        0..=9 => b'0' + n,
        10..=15 => b'A' + (n - 10),
        _ => b'?', // 仅理论
    }
}

#[allow(dead_code)]
#[section_init_text]
fn uart_put_u64_hex(val: u64) {
    for i in (0..8).rev() {
        let byte = ((val >> (i * 8)) & 0xff) as u8;
        uart_put_hex(byte);
    }
}

#[allow(dead_code)]
#[section_init_text]
fn uart_put_u64_hex_le(val: u64) {
    for i in 0..8 {
        let byte = ((val >> (i * 8)) & 0xff) as u8;
        uart_put_hex(byte);
    }
}

/// Early uart put
#[inline(always)]
#[section_init_text]
pub fn early_uart_putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(0x09000000 as *mut u32, c as u32);
    }
}

