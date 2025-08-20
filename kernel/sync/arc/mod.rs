//! A reference-counted pointer.

mod std_vendor;
mod arc;

pub use arc::Arc;
pub(crate) use arc::ArcInner;
