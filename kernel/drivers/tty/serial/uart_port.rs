//! uart port define


/// uart port
#[allow(dead_code)]
pub struct UartPort {
    iobase: u64,
    membase: *mut u8,
    irq: u32,
    uartclk: u32,
    fifosize: u32,
    flags: u32,
    x_char: u8,
    read_status_mask: u32,
    ignore_status_mask: u32,
    timeout: u32,
}
