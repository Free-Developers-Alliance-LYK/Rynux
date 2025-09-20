//! A reference-counted pointer.

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use std::sync::Arc;
    } else {
        mod std_vendor;
        mod arc;
        pub use arc::Arc;
        pub(crate) use arc::ArcInner;
    }
}
