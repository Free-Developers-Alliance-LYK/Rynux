//! Rynux arm64 symbols

// Sections define in vmrynux.lds.S
extern "C" {
    /// head of image
    pub fn _text();

    /// start of text
    pub fn _stext();
    /// end of text
    pub fn _etext();

    /// start RODATA
    pub fn __start_rodata();
    /// start of idmap text,
    /// These code sections that are never executed via the kernel mapping
    pub fn __idmap_text_start();
    /// end of idmap text
    pub fn __idmap_text_end();
    /// idmap page directory
    pub fn idmap_pg_dir();
    /// init page directory
    pub fn reserved_pg_dir();
    /// swapper_pg_dir
    pub fn swapper_pg_dir();

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
}

// Extern c function define in pi
extern "C" {
    /// init idmap page directory end
    pub fn __pi_create_init_idmap();
    /// early map kernel
    pub fn __pi_early_map_kernel();
}
