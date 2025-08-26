//! A reference-counted pointer.

use crate::cfg_if;

cfg_if! {
    if #[cfg(not(test))] {
        mod std_vendor;
        mod arc;
        pub use arc::Arc;
        pub(crate) use arc::ArcInner;
    } else {
        pub use std::sync::Arc;
    }
}
