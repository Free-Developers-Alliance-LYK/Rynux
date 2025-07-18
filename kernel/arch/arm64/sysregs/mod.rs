//! ARM64 system registers

#[macro_use]
mod macros;

pub(crate) mod current_el;
pub(crate) mod sctlr_el1;
pub(crate) mod tcr_el1;
pub(crate) mod spsr_el1;
pub(crate) mod elr_el1;
pub(crate) mod lr;
pub(crate) mod cpacr_el1;
pub(crate) mod mdscr_el1;
pub(crate) mod pmuserenr_el0;
pub(crate) mod id_aa64dfr0_el1;
pub(crate) mod id_aa64mmfr0_el1;
pub(crate) mod id_aa64pfr0_el1;
pub(crate) mod amuserenr_el0;
pub(crate) mod mair_el1;
pub(crate) mod midr_el1;
pub(crate) mod id_aa64mmfr3_el1;
pub(crate) mod tcr2_el1;

pub use current_el::CurrentEL;
pub use sctlr_el1::SctlrEl1;
pub use tcr_el1::Tcr;
pub use spsr_el1::SpsrEl1;
pub use elr_el1::ElrEl1;
pub use lr::Lr;
pub use cpacr_el1::CpacrEl1;
pub use mdscr_el1::MdscrEl1;
pub use pmuserenr_el0::PmuserenrEl0;
pub use id_aa64dfr0_el1::IdAa64dfr0El1;
pub use id_aa64pfr0_el1::IdAa64pfr0El1;
pub use amuserenr_el0::AmuserenrEl0;
pub use mair_el1::{MairEl1, MairAttrIdx, MairAttr};
pub use midr_el1::MidrEl1;
pub use id_aa64mmfr0_el1::IdAa64mmfr0El1;
pub use id_aa64mmfr3_el1::IdAa64mmfr3El1;
pub use tcr2_el1::Tcr2El1;
