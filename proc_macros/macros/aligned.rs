//! Aligned impl

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ExprLit, Lit, Meta, Token};

pub(crate) fn aligned_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);
    let mut repr_args = Vec::new();
    for meta in args.iter() {
        match meta {
            Meta::NameValue(nv) if nv.path.is_ident("align") => {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(lit_int),
                    ..
                }) = &nv.value
                {
                    repr_args.push(quote!(align(#lit_int)));
                }
            }
            Meta::Path(path) if path.is_ident("C") => {
                repr_args.push(quote!(C));
            }
            Meta::Path(path) if path.is_ident("transparent") => {
                repr_args.push(quote!(transparent));
            }
            // support more
            _ => {}
        }
    }

    let repr_attr = quote! {
        #[repr( #(#repr_args),* )]
    };

    let mut input = parse_macro_input!(item as syn::Item);
    match &mut input {
        syn::Item::Struct(s) => {
            s.attrs.insert(0, syn::parse_quote!(#repr_attr));
        }
        syn::Item::Enum(e) => {
            e.attrs.insert(0, syn::parse_quote!(#repr_attr));
        }
        _ => {
            return quote! {
                compile_error!("aligned attribute only supports struct/enum!");
            }
            .into();
        }
    }

    quote!(#input).into()
}
