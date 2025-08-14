//! Rynux init
//!
//! TODO: 
//!   start_kernel:  
//!     - debug_objects_early_init
//!     - init_vmlinux_build_id
//!     - cgroup_init_early
//!

use kernel::init::init_task::INIT_TASK;
use kernel::arch::arm64::early_debug::early_uart_put_u64_hex;
use kernel::arch::irq::{ArchIrq, IRQ};
use kernel::cpu::processor::processor_boot_init;
use kernel::arch::setup::{ArchBootSetupTrait, ArchBootSetup};

/// Start kernel
#[no_mangle] 
extern "C" fn start_kernel() -> ! {
    INIT_TASK.set_stack_end_magic();

    // early boot irq disable
    IRQ::local_disable();
    processor_boot_init();

    //early_uart_putchar('O' as u8);
    early_uart_put_u64_hex(0x1234);
    ArchBootSetup::setup_arch();
    early_uart_put_u64_hex(0x1234);
    loop {}
}

