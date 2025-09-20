//! Rynux arm64 symbols

// Kernel page table dir reserved in link image
unsafe extern "C" {
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
}

// Extern c function define in pi
unsafe extern "C" {
    /// init idmap page directory end
    pub fn __pi_create_init_idmap();
    /// early map kernel
    pub fn __pi_early_map_kernel();
}
