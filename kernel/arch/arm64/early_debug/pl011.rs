//! Types and definitions for PL011 UART.
//!
//! The official documentation: <https://developer.arm.com/documentation/ddi0183/latest>

use core::ptr::NonNull;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_structs! {
    /// Pl011 registers.
    Pl011UartRegs {
        /// Data Register.
        (0x0000 => dr: ReadWrite<u32>),
        (0x0004 => _reserved0),
        /// Flag Register.
        (0x0018 => fr: ReadOnly<u32>),
        (0x001c => _reserved1),
        (0x0024 => ibrd: ReadWrite<u32>),
        (0x0028 => fbrd: ReadWrite<u32>),
        (0x002c => lcrh: ReadWrite<u32>),
        /// Control register.
        (0x0030 => cr: ReadWrite<u32>),
        /// Interrupt FIFO Level Select Register.
        (0x0034 => ifls: ReadWrite<u32>),
        /// Interrupt Mask Set Clear Register.
        (0x0038 => imsc: ReadWrite<u32>),
        /// Raw Interrupt Status Register.
        (0x003c => ris: ReadOnly<u32>),
        /// Masked Interrupt Status Register.
        (0x0040 => mis: ReadOnly<u32>),
        /// Interrupt Clear Register.
        (0x0044 => icr: WriteOnly<u32>),
        (0x0048 => dmacr: ReadWrite<u32>),
        (0x004c => _reserved_0),
        (0x0080 => _reserved_test),
        (0x0090 => _reserved_1),
        (0x0fd0 => _reserved_future),
        (0x0fe0 => periphid0: ReadOnly<u32>),
        (0x0fe4 => periphid1: ReadOnly<u32>),
        (0x0fe8 => periphid2: ReadOnly<u32>),
        (0x0fec => periphid3: ReadOnly<u32>),
        (0x0ff0 => pcellid0: ReadOnly<u32>),
        (0x0ff4 => pcellid1: ReadOnly<u32>),
        (0x0ff8 => pcellid2: ReadOnly<u32>),
        (0x0ffc => pcellid3: ReadOnly<u32>),
        (0x1000 => @END),
    }
}

/// The Pl011 Uart
///
/// The Pl011 Uart provides a programing interface for:
/// 1. Construct a new Pl011 UART instance
/// 2. Initialize the Pl011 UART
/// 3. Read a char from the UART
/// 4. Write a char to the UART
/// 5. Handle a UART IRQ
pub struct Pl011Uart {
    base: NonNull<Pl011UartRegs>,
}

unsafe impl Send for Pl011Uart {}
unsafe impl Sync for Pl011Uart {}

#[allow(dead_code)]
impl Pl011Uart {
    /// Constrcut a new Pl011 UART instance from the base address.
    pub const unsafe fn new(base: *mut u8) -> Self {
        unsafe {
            Self {
                // SAFETY: base is a valid pointer
                base: NonNull::new_unchecked(base as *mut Pl011UartRegs),
            }
        }
    }

    const fn regs(&self) -> &Pl011UartRegs {
        unsafe { self.base.as_ref() }
    }

    /// Initializes the Pl011 UART.
    ///
    /// It clears all irqs, sets fifo trigger level, enables rx interrupt, enables receives
    pub fn init(&mut self) {
        // clear all irqs
        self.regs().icr.set(0x7ff);

        // set fifo trigger level
        self.regs().ifls.set(0); // 1/8 rxfifo, 1/8 txfifo.

        // enable rx interrupt
        self.regs().imsc.set(1 << 4); // rxim

        // enable receive
        self.regs().cr.set((1 << 0) | (1 << 8) | (1 << 9)); // tx enable, rx enable, uart enable
    }

    /// Output a char c to data register
    pub fn putchar(&self, c: u8) {
        while self.regs().fr.get() & (1 << 5) != 0 {}
        self.regs().dr.set(c as u32);
    }

    /// Return a Option if pl011 has received a new char
    /// Or it will return None
    pub fn getchar(&mut self) -> Option<u8> {
        if self.regs().fr.get() & (1 << 4) == 0 {
            Some(self.regs().dr.get() as u8)
        } else {
            None
        }
    }

    /// Return true if pl011 has received an interrupt
    pub fn is_receive_interrupt(&self) -> bool {
        let pending = self.regs().mis.get();
        pending & (1 << 4) != 0
    }

    /// Clear all interrupts
    pub fn ack_interrupts(&mut self) {
        self.regs().icr.set(0x7ff);
    }

    fn get_ris(&self) -> u32 {
        self.regs().ris.get()
    }

    fn get_periphid0(&self) -> u32 {
        self.regs().periphid0.get()
    }

    fn get_periphid1(&self) -> u32 {
        self.regs().periphid1.get()
    }

    fn get_periphid2(&self) -> u32 {
        self.regs().periphid2.get()
    }

    fn get_periphid3(&self) -> u32 {
        self.regs().periphid3.get()
    }

    fn get_pcellid0(&self) -> u32 {
        self.regs().pcellid0.get()
    }

    fn get_pcellid1(&self) -> u32 {
        self.regs().pcellid1.get()
    }

    fn get_pcellid2(&self) -> u32 {
        self.regs().pcellid2.get()
    }

    fn get_pcellid3(&self) -> u32 {
        self.regs().pcellid3.get()
    }
}
