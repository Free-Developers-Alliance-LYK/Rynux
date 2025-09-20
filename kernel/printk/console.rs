//! console

use crate::bitflags::bitflags;
use crate::list::def_node;
use crate::sync::arc::Arc;
use crate::sync::lock::Mutex;

bitflags! {
    /// Console flags
    #[repr(transparent)]
    #[allow(dead_code)]
    pub struct ConsoleFlags: u32 {
        /// Used by newly registered consoles to avoid duplicate output of messages that were already shown by boot consoles or read by userspace via syslog() syscall.
        const CON_PRINTBUFFER = 1 << 0;
        /// Indicates that the console driver is backing /dev/console.
        const CON_CONSDEV = 1 << 1;
        /// Indicates if a console is allowed to print records. If false, the console also will not advance to later records.
        const CON_ENABLED = 1 << 2;
        /// Marks the console driver as early console driver which is used during boot before the real driver becomes available. It will be automatically unregistered when the real console driver is registered unless "keep_bootcon" parameter is used.
        const CON_BOOT = 1 << 3;
        /// A misnomed historical flag which tells the core code that the legacy @console::write callback can be invoked on a CPU which is marked OFFLINE. That is misleading as it suggests that there is no contextual limit for invoking the callback. The original motivation was readiness of the per-CPU areas.
        const CON_ANYTIME = 1 << 4;
        /// Indicates a braille device which is exempt from receiving the printk spam for obvious reasons.
        const CON_BRL = 1 << 5;
        /// The console supports the extended output format of /dev/kmesg which requires a larger output buffer.
        const CON_EXTENDED = 1 << 6;
        /// Indicates if a console is suspended. If true, the printing callbacks must not be called.
        const CON_SUSPENDED = 1 << 7;
        /// Console can operate outside of the legacy style console_lock
        const CON_NBCON = 1 << 8;
    }
}

/// console
#[allow(dead_code)]
pub struct Console {
    name: [u8; 16],
    name_len: usize,
    write: Option<fn(&str)>,
    read: Option<fn(&mut [u8]) -> usize>,
    flags: ConsoleFlags,
    index: u32,
}

#[allow(dead_code)]
impl Console {
    const fn name_as_array(name: &str) -> [u8; 16] {
        let bytes = name.as_bytes();
        let mut arr = [0u8; 16];
        let mut i = 0;
        while i < bytes.len() && i < 16 {
            arr[i] = bytes[i];
            i += 1;
        }
        arr
    }

    /// Default console constructor
    pub const fn empty(name: &str, flags: ConsoleFlags, index: u32) -> Self {
        Self {
            name: Self::name_as_array(name),
            name_len: if name.len() > 16 { 16 } else { name.len() },
            write: None,
            read: None,
            flags,
            index,
        }
    }

    /// name str
    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len]).unwrap()
    }
}

def_node! {
    /// console
    pub struct ConsoleNode(Console);
}

/// console list
type ConsoleList = crate::list::List<Arc<ConsoleNode>>;

/// A global console mange list
pub static GLOBAL_CONSOLE: Mutex<ConsoleList> =
    Mutex::new(ConsoleList::new(), Some("GlobalConsoleList"));

impl ConsoleList {
    /// register a console
    pub fn register(&mut self, console: Arc<ConsoleNode>) {
        self.push_back(console);
    }

    /// is register
    pub fn is_register(&self, console: &Arc<ConsoleNode>) -> bool {
        self.iter().any(|c| core::ptr::eq(c, &**console))
    }
}
