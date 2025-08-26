//! Some symbol definitions that need to be used directly, such as segments
//! defined in link files, or function symbols that need to be called directly
//! by assembly

pub use crate::arch::symbols::*;

extern "C" {
    /// define in init kernel
    pub fn start_kernel();

    /// obs_kernel_param start address
    pub fn __setup_start();
    /// obs_kernel_param end address
    pub fn __setup_end();

    /// early con table
    pub fn __earlycon_table();
    /// early con table end
    pub fn __earlycon_table_end();

}
