//! Rynux init
//!
//! TODO: 
//!   start_kernel:  
//!     - debug_objects_early_init
//!     - init_vmlinux_build_id
//!     - cgroup_init_early
//!

use kernel::arch::arm64::early_debug::early_uart_put_u64_hex;
//use kernel::arch::arm64::early_debug::early_uart_putchar;
use kernel::arch::irq::{ArchIrq, IRQ};
use kernel::cpu::processor::processor_boot_init;
use kernel::arch::setup::{ArchBootSetupTrait, ArchBootSetup};

/// Start kernel
#[unsafe(no_mangle)] 
extern "C" fn start_kernel() -> ! {
    // Test current is set OK ?
    let current = kernel::schedule::current();
    if current.magic != kernel::schedule::task::Task::BOOT_TASK_MAGIC {
        panic!("current task magic is not correct");
    }
    current.set_stack_end_magic();

    // early boot irq disable
    IRQ::local_disable();
    processor_boot_init();

    //early_uart_putchar('O' as u8);
    early_uart_put_u64_hex(0x1234);
    // After this, we can use memblock allocator
    ArchBootSetup::setup_arch();
    early_uart_put_u64_hex(0x1234);
    loop {}
}

