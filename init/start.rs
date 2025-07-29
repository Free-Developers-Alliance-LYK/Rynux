//! Rynux init

use kernel::init::init_task::INIT_TASK;
use kernel::arch::arm64::early_debug::early_uart_putchar;

/// Start kernel
#[no_mangle] 
extern "C" fn start_kernel() -> ! {
    INIT_TASK.set_stack_end_magic();
    early_uart_putchar('1' as u8);

    loop {}
}

