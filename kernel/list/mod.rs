//! Lists support

pub mod linked_list;
pub(crate) mod raw_list;

pub use linked_list::List;
pub use raw_list::{GetLinks, Links, RawList};

#[doc(hidden)]
macro_rules! __def_node_internal {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($type:ty);) => {
        $(#[$meta])*
        $vis struct $name {
            inner: $type,
            links: $crate::list::Links<Self>,
        }

        impl $crate::GetLinks for $name {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::list::Links<Self> {
                &t.links
            }
        }

        impl $name {
            #[doc = "Create a node"]
            pub const fn new(inner: $type) -> Self {
                Self {
                    inner,
                    links: $crate::list::Links::new(),
                }
            }

            #[inline]
            #[doc = "Return the referece of wrapped inner"]
            pub const fn inner(&self) -> &$type {
                &self.inner
            }

            #[inline]
            #[doc = "Consumes the `node`, returning the wrapped inner"]
            pub fn into_inner(self) -> $type {
                self.inner
            }
        }

        impl core::ops::Deref for $name {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };

    ($(#[$meta:meta])* $vis:vis struct $name:ident<$gen:ident>($type:ty);) => {
        $(#[$meta])*
        $vis struct $name<$gen> {
            inner: $type,
            links: $crate::list::Links<Self>,
        }

        impl<$gen> $crate::list::GetLinks for $name<$gen> {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::list::Links<Self> {
                &t.links
            }
        }

        impl<$gen> $name<$gen> {
            #[doc = "Create a node"]
            pub const fn new(inner: $type) -> Self {
                Self {
                    inner,
                    links: $crate::list::Links::new(),
                }
            }

            #[inline]
            #[doc = "Return the referece of wrapped inner"]
            pub const fn inner(&self) -> &$type {
                &self.inner
            }

            #[inline]
            #[doc = "Consumes the `node`, returning the wrapped inner"]
            pub fn into_inner(self) -> $type {
                self.inner
            }
        }

        impl<$gen> core::ops::Deref for $name<$gen> {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

/// A macro for create a node type that can be used in List.
///
/// # Syntax
///
/// ```ignore
/// def_node! {
/// /// A node with usize value.
/// [pub] struct UsizedNode(usize);
/// /// A node with generic inner type.
/// [pub] struct WrapperNode<T>(T);
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use crate::list::{def_node, List};
/// use crate::sync::arc::Arc;
///
/// def_node!{
///     /// An example Node with usize
///     struct ExampleNode(usize);
///     /// An example Node with generic Inner type and pub(crate)
///     pub(crate) struct NativeGenericNode(usize);
///     /// An example Node with generic Inner type and pub vis
///     pub struct GenericNode<T>(T);
/// }
///
/// let node1 = Arc::new(ExampleNode::new(0));
/// let node2 = Arc::new(ExampleNode::new(1));
/// let mut list =  List::<Arc<ExampleNode>>::new();
///
/// list.push_back(node1);
/// list.push_back(node2);
///
/// for (i,e) in list.iter().enumerate() {
///     assert!(*e.inner() == i);
/// }
///
/// let node1 = list.pop_front().unwrap();
/// let node2 = list.pop_front().unwrap();
///
/// assert!(node1.into_inner() == 0);
/// assert!(node2.into_inner() == 1);
/// assert!(list.pop_front().is_none());
///
/// let node1 = Arc::new(GenericNode::new(0));
/// let node2 = Arc::new(GenericNode::new(1));
///
/// let mut list =  List::<Arc<GenericNode<usize>>>::new();
///
/// list.push_back(node1);
/// list.push_back(node2);
///
/// for (i,e) in list.iter().enumerate() {
///     assert!(*e.inner() == i);
/// }
/// ```
///
#[macro_export]
macro_rules! def_node {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($type:ty); $($t:tt)*) => {
        $crate::list::__def_node_internal!($(#[$meta])* $vis struct $name($type););
        def_node!($($t)*);
    };
    ($(#[$meta:meta])* $vis:vis struct $name:ident<$gen:ident>($type:ty); $($t:tt)*) => {
        $crate::list::__def_node_internal!($(#[$meta])* $vis struct $name<$gen>($type););
        def_node!($($t)*);
    };
    () => ()
}

pub use def_node;
