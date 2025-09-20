//! Some symbol definitions that need to be used directly, such as segments
//! defined in link files, or function symbols that need to be called directly
//! by assembly

pub use crate::arch::symbols::*;

// SAFETY: These symbols are defined sections
unsafe extern "C" {
    /// head of image
    pub fn _text();
    /// start of text
    pub fn _stext();
    /// end of text
    pub fn _etext();
    /// start RODATA
    pub fn __start_rodata();

    /// init begin
    pub fn __init_begin();
    /// init text begin
    pub fn __inittext_begin();
    /// init text end
    pub fn __inittext_end();
    /// start of init data
    pub fn __initdata_begin();

    /// init idmap page directory
    pub fn init_idmap_pg_dir();
    /// init idmap page directory end
    pub fn init_idmap_pg_end();
    /// end of init data
    pub fn __initdata_end();
    /// end of init
    pub fn __init_end();

    /// Data start
    pub fn _data();
    /// start of bss
    pub fn __bss_start();
    /// init page directory
    pub fn init_pg_dir();
    /// init page directory end
    pub fn init_pg_end();

    /// early init stack 4K
    pub fn early_init_stack();

    /// end of kernel
    pub fn _end();

    /// obs_kernel_param start address
    pub fn __setup_start();
    /// obs_kernel_param end address
    pub fn __setup_end();
    /// early con table
    pub fn __earlycon_table();
    /// early con table end
    pub fn __earlycon_table_end();
    /// init_stack define in vmrynux.rs
    pub fn init_stack();
}

// SAFETY: no_mangle funcs
unsafe extern "C" {
    /// start kernel function
    pub fn start_kernel();
}
