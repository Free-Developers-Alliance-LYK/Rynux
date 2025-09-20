//! Command line parsing

use crate::arch::arm64::early_debug::early_uart_put_str;
use crate::drivers::fdt::GLOBAL_FDT;
use crate::macros::section_init_text;
use crate::param::obs_param::for_each_setup_param;
use crate::param::ParamParser;
use crate::sync::lock::RawSpinLockNoIrq;
use crate::types::OnceCell;

const COMMAND_LINE_SIZE: usize = 2048;

/// Command line Parser
pub struct CommandLine {
    arch_boot_cmdline: [u8; COMMAND_LINE_SIZE],
    parsed_early_options: bool,
    used: usize,
}

impl CommandLine {
    /// Get the command line as a string
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.arch_boot_cmdline[..self.used]).unwrap()
    }

    #[section_init_text]
    /// Parse early options
    pub fn parse_early_options(&mut self) {
        if self.parsed_early_options {
            return;
        }

        let tmp_cmdline = self.as_str();
        let parser = ParamParser::new(tmp_cmdline);

        for (param, val) in parser {
            for_each_setup_param(|p| {
                if p.name == param {
                    let _ = (p.func)(val);
                }
            });
        }

        self.parsed_early_options = true;
    }
}

/// A static instance of the command line.
pub static GLOBAL_COMMAND_LINE: RawSpinLockNoIrq<OnceCell<CommandLine>> =
    RawSpinLockNoIrq::new(OnceCell::new(), Some("GLOBAL_COMMAND_LINE"));

#[section_init_text]
pub(crate) fn setup_from_fdt() {
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
    GLOBAL_COMMAND_LINE.lock().set(CommandLine {
        arch_boot_cmdline: buf,
        used: bootargs.len(),
        parsed_early_options: false,
    });
}
