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

use kernel::arch::setup::{ArchProcessorInit, ProcessorInit};
use kernel::arch::irq::{ArchIrq, IRQ};
//use kernel::schedule::task::CurrentTask;

/// Start kernel
#[no_mangle] 
extern "C" fn start_kernel() -> ! {
    INIT_TASK.set_stack_end_magic();
    ProcessorInit::smp_setup_processor_id();
    // early boot irq disable
    IRQ::local_disable();

    //early_uart_putchar('O' as u8);
    early_uart_put_u64_hex(0x1234);
    /*
    let current = CurrentTask::get();
    early_uart_put_u64_hex(current.magic);
    */
    loop {}
}

