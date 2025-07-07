//! Link impl

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, Meta, Token, Expr, ExprLit, Lit};
use syn::punctuated::Punctuated;

pub(crate) fn section_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut section: Option<String> = None;

    for meta in args.iter() {
        if let Meta::NameValue(nv) = meta {
            let ident = nv.path.get_ident().map(|i| i.to_string());
            if let Some(name) = ident {
                if name == "section" {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &nv.value {
                        section = Some(lit_str.value());
                    }
                }
            }
        }
    }

    if section.is_none() {
        return quote! {
            compile_error!("section requires section  e.g. #[section(section = \".foo\")]");
        }
        .into();
    }

    let section = section.unwrap();

    let input = parse_macro_input!(item as Item);

    let output = match input {
        Item::Static(mut s) => {
            s.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            quote!(#s)
        }
        Item::Const(mut c) => {
            c.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            quote!(#c)
        }
        Item::Fn(mut f) => {
            f.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            quote!(#f)
        }
        other => {
            quote! {
                #[link_section = #section]
                #other
            }
        }
    };
    output.into()
}
