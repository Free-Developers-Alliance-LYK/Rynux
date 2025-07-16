//! ARM64 system registers

#[macro_use]
mod macros;

mod current_el;
mod sctlr_el1;
mod tcr_elx;
mod spsr_el1;
mod elr_el1;
mod lr;
mod cpacr_el1;
mod mdscr_el1;
mod pmuserenr_el0;
mod id_aa64dfr0_el1;
mod id_aa64pfr0_el1;
mod amuserenr_el0;
mod mair_el1;

pub use current_el::CurrentEL;
pub use sctlr_el1::SctlrEl1;
pub use tcr_elx::Tcr;
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
