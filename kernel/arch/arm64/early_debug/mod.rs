//! This is very early uart print,debugging for the boot phase
//! we assume the uart is already inited by uboot,and device address is
//! identity map in arm64 boot.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_QEMU)] {
        pub mod pl011;
        use pl011::Pl011Uart;
        /// QEMU UART base address
        pub const EARLY_UART_BASE: usize = 0x0900_0000;
        static UART: Pl011Uart = unsafe {
            Pl011Uart::new(EARLY_UART_BASE as *mut u8)};
    } else if #[cfg(CONFIG_RASPI4B)] { 
        pub mod pl011;
        use pl011::Pl011Uart;
        /// Raspberry Pi 4B UART base address
        pub const EARLY_UART_BASE: usize = 0xFE20_1000;
        static UART: Pl011Uart = unsafe {
            Pl011Uart::new(EARLY_UART_BASE as *mut u8)};
    } else {
        compile_error!("Unsupported platform");
    }
}

#[allow(dead_code)]
#[inline(always)]
fn uart_put_hex(byte: u8) {
    let hi = (byte >> 4) & 0xf;
    let lo = byte & 0xf;
    early_uart_putchar(nibble_to_ascii(hi));
    early_uart_putchar(nibble_to_ascii(lo));
}

#[inline(always)]
fn nibble_to_ascii(n: u8) -> u8 {
    match n {
        0..=9 => b'0' + n,
        10..=15 => b'A' + (n - 10),
        _ => b'?',
    }
}

#[inline(always)]
#[allow(dead_code)]
fn uart_put_u64_hex_le(val: u64) {
    for i in 0..8 {
        let byte = ((val >> (i * 8)) & 0xff) as u8;
        uart_put_hex(byte);
    }
}

/// Early uart put
#[no_mangle]
pub fn early_uart_putchar(c: u8) {
      match c {
          b'\n' => {
              UART.putchar(b'\r');
              UART.putchar(b'\n');
          }
          c => UART.putchar(c),
      }
}

#[no_mangle]
/// Print u64 in hex
pub fn early_uart_put_u64_hex(val: u64) {
    for i in (0..8).rev() {
        let byte = ((val >> (i * 8)) & 0xff) as u8;
        uart_put_hex(byte);
    }
}
