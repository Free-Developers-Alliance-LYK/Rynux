//! Observation parameter parsing
//!
//! struct obs_kernel_param  would be static linked in .init.setup

use super::ParamHandleErr;

/// Rynux early param
#[repr(C)]
pub struct ObsKernelParam {
    /// Parameter name
    pub name: &'static str,
    /// Parameter handle function
    pub func: fn(Option<&str>) -> Result<(), ParamHandleErr>,
    /// Whether this parameter is early
    pub early: bool,
}

/// Only for really core code.
///
/// Force the alignment so the compiler doesn't space elements of the
/// obs_kernel_param "array" too far apart in .init.setup.
///
/// Return 0 if ok, -1 on error.
///
#[macro_export]
macro_rules! early_setup_param {
    ($id:ident, $name:expr, $func:ident) => {
        #[link_section = ".init.setup"]
        #[used]
        static $id: $crate::param::obs_param::ObsKernelParam = $crate::param::obs_param::ObsKernelParam {
            name: $name,
            func: $func,
            early: true,
        };
    };
}

pub use early_setup_param;

/// for each setup param
#[cfg(not(test))]
pub fn for_each_setup_param(mut f: impl FnMut(&ObsKernelParam)) {
    use crate::global_sym::{__setup_start, __setup_end};
    // SAFETY: __setup_start and __setup_end are defined in link files
    unsafe {
        let start = __setup_start as *const ObsKernelParam;
        let end   = __setup_end   as *const ObsKernelParam;
        let n = (end as usize - start as usize) / core::mem::size_of::<ObsKernelParam>();
        let slice = core::slice::from_raw_parts(start, n);
        for p in slice.iter().filter(|p| p.early) {
            f(p);
        }
    }
}
