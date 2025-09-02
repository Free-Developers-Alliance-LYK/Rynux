//! uart port define

/// uart port io type
pub enum UartPortIoType {
    /// unknown
    Unknown,
    /// 8b I/O port access
    Port,
    /// Hub6 ISA card
    Hub6,
    /// driver-specific
    Mem,
    /// 32b little endian
    Mem32,
    /// Au1x00 and RT288x type IO
    Au,
    /// Tsi108/109 type IO
    Tsi,
    /// 32b big endian
    Mem32Be,
    /// 16b little endian
    Mem16,
}

/// uart port
#[allow(dead_code)]
pub struct UartPort {
    iobase: u64,
    irq: u32,
    uartclk: u32,
    fifosize: u32,
    flags: u32,
    x_char: u8,
    read_status_mask: u32,
    ignore_status_mask: u32,
    timeout: u32,
    iotype: UartPortIoType,
}

impl UartPort {
    /// new empty uart port
    pub const fn new_empty() -> Self {
        Self {
            iobase: 0,
            irq: 0,
            uartclk: 0,
            fifosize: 0,
            flags: 0,
            x_char: 0,
            read_status_mask: 0,
            ignore_status_mask: 0,
            timeout: 0,
            iotype: UartPortIoType::Unknown,
        }
    }

    /// set iotype
    pub fn set_iotype(&mut self, iotype: UartPortIoType) {
        self.iotype = iotype;
    }
}
