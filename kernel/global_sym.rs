//! Some symbol definitions that need to be used directly, such as segments
//! defined in link files, or function symbols that need to be called directly
//! by assembly

pub use crate::arch::symbols::*;

extern "C" {
    /// define in init kernel
    pub fn start_kernel();

}
