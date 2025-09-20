//! amba_pl011.rs

use kernel::arch::arm64::early_debug::early_uart_put_str;
use kernel::drivers::tty::serial::earlycon::{earlycon_declare, EarlyConDevice};

fn pl011_earlycon_setup(_dev: &mut EarlyConDevice, _options: Option<&'static str>) {
    early_uart_put_str("enter pl011_earlycon_setup\n");
}

earlycon_declare!(PL011_EARLYCON, "amba_pl011", pl011_earlycon_setup);
earlycon_declare!(ARM_PL011_EARLYCON, "arm,pl011", pl011_earlycon_setup);
