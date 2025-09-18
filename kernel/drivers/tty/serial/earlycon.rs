//! earlycon
//!
//! TODO: support ACPI

use crate::macros::section_init_text;
use crate::param::ParamHandleErr;
use crate::printk::console::{Console, ConsoleFlags};
use crate::param::obs_param::early_setup_param;
use crate::{
    printk::console::{GLOBAL_CONSOLE, ConsoleNode},
    sync::{
        lock::RawSpinLockNoIrq,
        arc::{Arc, ArcInner},
    },
    fdtree_rs::chosen::Stdout,
};
use super::uart_port::{UartPort, UartPortIoType};

/// Earlycon device
#[allow(dead_code)]
pub struct EarlyConDevice {
    console: Arc<ConsoleNode>,
    port: RawSpinLockNoIrq<UartPort>,
}


/// SAFETY: we know what we are doing here. 
/// we use a satic mem to init Arc, if this init Arc refcont to 0, it will panic. 
static EARLYCON_CONSOLE_NODE: ArcInner<ConsoleNode> = ArcInner::new_static(ConsoleNode::new(
        Console::empty("uart", ConsoleFlags::from_bits_truncate(
                        ConsoleFlags::CON_PRINTBUFFER.bits() | ConsoleFlags::CON_BOOT.bits()), 0)
));

static EARLYCON_DEV: EarlyConDevice = EarlyConDevice {
        // SAFETY: we know what we are doing here. 
        // we use a satic mem to init Arc, if this init Arc refcont to 0, it will panic. 
        console: unsafe {Arc::from_static(&EARLYCON_CONSOLE_NODE)},
        port: RawSpinLockNoIrq::new(UartPort::new_empty(), None),
};


impl EarlyConDevice {
    fn init_from_fdt_node(&self, _con_id: &EarlyConId, _node: Stdout<'_, '_>, _options: Option<&str>) 
        -> Result<(), ParamHandleErr> {
        if GLOBAL_CONSOLE.lock().is_register(&self.console) {
            return Ok(());
        }
    
        let mut port = self.port.lock();
        port.set_iotype(UartPortIoType::Mem);
        Ok(())
    }
}

/// Earlycon id, all of them are linked in section __earlycon_table
///
/// Example:
///
/// use crate::drivers::tty::serial::earlycon::earlycon_declare;
///
/// earlycon_declare!(pl011, "arm,pl011", setup_pl011_earlycon);
///
#[repr(C)]
pub struct EarlyConId {
    name: &'static str,
    compatible: &'static str,
    setup: fn(&mut EarlyConDevice, Option<&'static str>),
}

impl EarlyConId {
    /// Create a new EarlyConId
    pub const fn new(name: &'static str, compatible: &'static str, setup: fn(&mut EarlyConDevice, Option<&'static str>)) -> Self {
        Self {
            name,
            compatible,
            setup,
        }
    }

    fn find(compatible: &str) -> Option<&'static EarlyConId> {
        use crate::global_sym::{__earlycon_table, __earlycon_table_end};
        // SAFETY: __earlycon_table and __earlycon_table_end are defined in link script
        unsafe {
            let start = __earlycon_table as *const EarlyConId;
            let end   = __earlycon_table_end   as *const EarlyConId;
            let n = (end as usize - start as usize) / core::mem::size_of::<EarlyConId>();
            let slice = core::slice::from_raw_parts(start, n);
            for id in slice {
                if id.compatible == compatible {
                    return Some(id);
                }
            }
            None
        }
    }
}

//use crate::arch::arm64::early_debug::early_uart_put_str;

/// earlycon_declare!
///
/// Example:
///
/// earlycon_declare!(pl011, setup_pl011_earlycon);
///
#[macro_export]
macro_rules! earlycon_declare {
    ($name:ident, $compatible:expr, $fn:ident) => {
        #[unsafe(link_section = "__earlycon_table")]
        #[used]
        static $name: $crate::drivers::tty::serial::earlycon::EarlyConId = $crate::drivers::tty::serial::earlycon::EarlyConId::new(stringify!($name), $compatible, $fn);
    };
}

pub use earlycon_declare;

fn init_earlycon_from_fdt() -> Result<(), ParamHandleErr> { 
    let stdout = crate::drivers::fdt::GLOBAL_FDT.chosen().stdout();
    match stdout {
        Some(stdout) => {
            let compatible = stdout.node.compatible().unwrap().first();
            let options = stdout.options;
            let earlycon_id = EarlyConId::find(compatible);
            match earlycon_id {
                Some(earlycon_id) => {
                    EARLYCON_DEV.init_from_fdt_node(earlycon_id, stdout, options)?;
                }
                None => {
                    return Err(ParamHandleErr::Unknown)
                }
            }
        }
        None => {
            todo!();
        }
    }

    Ok(())
}

#[section_init_text]
fn setup_earlycon_param(val: Option<&str>) -> Result<(), ParamHandleErr> {
    match val {
        Some(_val) => todo!(),
        None =>  return init_earlycon_from_fdt(),
    }
}

// register earlycon param setup func
early_setup_param!(EARLYCON_PARAM, "earlycon", setup_earlycon_param);
