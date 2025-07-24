// SPDX-License-Identifier: GPL-2.0

//! Crate for all kernel procedural macros.

#![allow(dead_code)]

// When fixdep scans this, it will find this string `CONFIG_RUSTC_VERSION_TEXT`
// and thus add a dependency on `include/config/RUSTC_VERSION_TEXT`, which is
// touched by Kconfig when the version string from the compiler changes.

#[macro_use]
mod concat_idents;
mod helpers;
mod paste;
mod link;
mod aligned;

use proc_macro::TokenStream;

/// Concatenate two identifiers.
///
/// This is useful in macros that need to declare or reference items with names
/// starting with a fixed prefix and ending in a user specified name. The resulting
/// identifier has the span of the second argument.
///
/// # Examples
///
/// ```ignore
/// use kernel::macro::concat_idents;
///
/// macro_rules! pub_no_prefix {
///     ($prefix:ident, $($newname:ident),+) => {
///         $(pub(crate) const $newname: u32 = kernel::macros::concat_idents!($prefix, $newname);)+
///     };
/// }
///
/// pub_no_prefix!(
///     binder_driver_return_protocol_,
///     BR_OK,
///     BR_ERROR,
///     BR_TRANSACTION,
///     BR_REPLY,
///     BR_DEAD_REPLY,
///     BR_TRANSACTION_COMPLETE,
///     BR_INCREFS,
///     BR_ACQUIRE,
///     BR_RELEASE,
///     BR_DECREFS,
///     BR_NOOP,
///     BR_SPAWN_LOOPER,
///     BR_DEAD_BINDER,
///     BR_CLEAR_DEATH_NOTIFICATION_DONE,
///     BR_FAILED_REPLY
/// );
///
/// assert_eq!(BR_OK, binder_driver_return_protocol_BR_OK);
/// ```
#[proc_macro]
pub fn concat_idents(ts: TokenStream) -> TokenStream {
    concat_idents::concat_idents(ts)
}

/// Paste identifiers together.
///
/// Within the `paste!` macro, identifiers inside `[<` and `>]` are concatenated together to form a
/// single identifier.
///
/// This is similar to the [`paste`] crate, but with pasting feature limited to identifiers and
/// literals (lifetimes and documentation strings are not supported). There is a difference in
/// supported modifiers as well.
///
/// # Example
///
/// ```ignore
/// use kernel::macro::paste;
///
/// macro_rules! pub_no_prefix {
///     ($prefix:ident, $($newname:ident),+) => {
///         paste! {
///             $(pub(crate) const $newname: u32 = [<$prefix $newname>];)+
///         }
///     };
/// }
///
/// pub_no_prefix!(
///     binder_driver_return_protocol_,
///     BR_OK,
///     BR_ERROR,
///     BR_TRANSACTION,
///     BR_REPLY,
///     BR_DEAD_REPLY,
///     BR_TRANSACTION_COMPLETE,
///     BR_INCREFS,
///     BR_ACQUIRE,
///     BR_RELEASE,
///     BR_DECREFS,
///     BR_NOOP,
///     BR_SPAWN_LOOPER,
///     BR_DEAD_BINDER,
///     BR_CLEAR_DEATH_NOTIFICATION_DONE,
///     BR_FAILED_REPLY
/// );
///
/// assert_eq!(BR_OK, binder_driver_return_protocol_BR_OK);
/// ```
///
/// # Modifiers
///
/// For each identifier, it is possible to attach one or multiple modifiers to
/// it.
///
/// Currently supported modifiers are:
/// * `span`: change the span of concatenated identifier to the span of the specified token. By
///   default the span of the `[< >]` group is used.
/// * `lower`: change the identifier to lower case.
/// * `upper`: change the identifier to upper case.
///
/// ```ignore
/// use kernel::macro::paste;
///
/// macro_rules! pub_no_prefix {
///     ($prefix:ident, $($newname:ident),+) => {
///         kernel::macros::paste! {
///             $(pub(crate) const fn [<$newname:lower:span>]: u32 = [<$prefix $newname:span>];)+
///         }
///     };
/// }
///
/// pub_no_prefix!(
///     binder_driver_return_protocol_,
///     BR_OK,
///     BR_ERROR,
///     BR_TRANSACTION,
///     BR_REPLY,
///     BR_DEAD_REPLY,
///     BR_TRANSACTION_COMPLETE,
///     BR_INCREFS,
///     BR_ACQUIRE,
///     BR_RELEASE,
///     BR_DECREFS,
///     BR_NOOP,
///     BR_SPAWN_LOOPER,
///     BR_DEAD_BINDER,
///     BR_CLEAR_DEATH_NOTIFICATION_DONE,
///     BR_FAILED_REPLY
/// );
///
/// assert_eq!(br_ok(), binder_driver_return_protocol_BR_OK);
/// ```
///
/// # Literals
///
/// Literals can also be concatenated with other identifiers:
///
/// ```ignore
/// macro_rules! create_numbered_fn {
///     ($name:literal, $val:literal) => {
///         kernel::macros::paste! {
///             fn [<some_ $name _fn $val>]() -> u32 { $val }
///         }
///     };
/// }
///
/// create_numbered_fn!("foo", 100);
///
/// assert_eq!(some_foo_fn100(), 100)
/// ```
///
/// [`paste`]: https://docs.rs/paste/
#[proc_macro]
pub fn paste(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().collect();
    paste::expand(&mut tokens);
    tokens.into_iter().collect()
}

use quote::quote;

#[proc_macro_attribute]
pub fn section_cache_aligned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    link::section_impl(
        quote!(section = ".data..cacheline_aligned").into(),
        item,
    )
}

#[proc_macro_attribute]
pub fn section_init_data(_attr: TokenStream, item: TokenStream) -> TokenStream {
    link::section_impl(
        quote!(section = ".init.data").into(),
        item,
    )
}

#[proc_macro_attribute]
pub fn section_init_text(_attr: TokenStream, item: TokenStream) -> TokenStream {
    link::section_impl(
        quote!(section = ".init.text").into(),
        item,
    )
}

#[proc_macro_attribute]
pub fn section_read_mostly(_attr: TokenStream, item: TokenStream) -> TokenStream {
    link::section_impl(
        quote!(section = ".data..read_mostly").into(),
        item,
    )
}

#[proc_macro_attribute]
pub fn section_idmap_text(_attr: TokenStream, item: TokenStream) -> TokenStream {
    link::section_impl(
        quote!(section = ".idmap.text").into(),
        item,
    )
}

use syn::parse_macro_input;
use syn::Item;

#[proc_macro_attribute]
pub fn need_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let output = match input {
        Item::Static(mut s) => {
            s.attrs.insert(0, syn::parse_quote!(#[no_mangle]));
            quote!(#s)
        }
        other => {
            let err = "The #[need_export] attribute may only be applied to static variables.";
            quote! {
                compile_error!(#err);
                #other
            }
            .into()
        }
    };
    output.into()
}


#[cfg(CONFIG_ARM64)]
#[proc_macro_attribute]
pub fn cache_aligned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // L1 cache aligend
    aligned::aligned_impl(
        quote!(align = 64).into(),
        item,
    )
}

#[cfg(CONFIG_PAGE_SIZE_4KB)]
#[proc_macro_attribute]
pub fn page_aligned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aligned::aligned_impl(
        quote!(align = 4096).into(),
        item,
    )
}

#[cfg(CONFIG_PAGE_SIZE_16KB)]
#[proc_macro_attribute]
pub fn page_aligned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aligned::aligned_impl(
        quote!(align = 16384).into(),
        item,
    )
}
