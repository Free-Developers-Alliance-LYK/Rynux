//! Rynux arm64 setup

/// The recorded values of x0 .. x3 upon kernel entry.
#[cacheline_aligned]
pub static BOOT_ARGS: [usize; 4] = [0; 4];

#[init_data]
pub static MMU_ENABLED_AT_BOOT: usize = 0;
