//! Command line parsing

use crate::types::OnceCell;
use crate::drivers::fdt::GLOBAL_FDT;
use crate::arch::arm64::early_debug::early_uart_put_str;

const COMMAND_LINE_SIZE: usize = 2048;

/// Command line Parser
pub struct CommandLine {
    arch_boot_cmdline: [u8; COMMAND_LINE_SIZE],
    used: usize,
}

impl CommandLine {
    /// Get the command line as a string
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.arch_boot_cmdline[..self.used]).unwrap()
    }
}


/// A static instance of the command line.
pub static GLOBAL_COMMAND_LINE: OnceCell<CommandLine> = OnceCell::new();

pub(crate) fn setup_from_fdt()  {
    let chosen = GLOBAL_FDT.chosen();
    let bootargs = chosen.bootargs().unwrap().as_bytes();
    if bootargs.len() >= COMMAND_LINE_SIZE {
        panic!("bootargs too long");
    }

    let mut buf = [0; COMMAND_LINE_SIZE];
    buf[..bootargs.len()].copy_from_slice(bootargs);
   
    early_uart_put_str("bootargs: ");
    early_uart_put_str(core::str::from_utf8(bootargs).unwrap());
    early_uart_put_str("\n");
    GLOBAL_COMMAND_LINE.set(CommandLine {
        arch_boot_cmdline: buf,
        used: bootargs.len(),
    });
}
