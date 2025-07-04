//! Link impl

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, Meta, Token, Expr, ExprLit, Lit};
use syn::punctuated::Punctuated;

pub(crate) fn aligned_section_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Punctuated<Meta, Token![,]>);

    let mut section: Option<String> = None;
    let mut align: Option<usize> = None;

    for meta in args.iter() {
        if let Meta::NameValue(nv) = meta {
            let ident = nv.path.get_ident().map(|i| i.to_string());
            if let Some(name) = ident {
                if name == "section" {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &nv.value {
                        section = Some(lit_str.value());
                    }
                }
                if name == "align" {
                    if let Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) = &nv.value {
                        align = Some(lit_int.base10_parse::<usize>().unwrap());
                    }
                }
            }
        }
    }

    if section.is_none() || align.is_none() {
        return quote! {
            compile_error!("aligned_section requires both section and align, e.g. #[aligned_section(section = \".foo\", align = 64)]");
        }
        .into();
    }

    let section = section.unwrap();
    let align = align.unwrap();

    let input = parse_macro_input!(item as Item);

    let output = match input {
        Item::Static(mut s) => {
            s.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            s.attrs.insert(1, syn::parse_quote!(#[repr(align(#align))]));
            quote!(#s)
        }
        Item::Const(mut c) => {
            c.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            c.attrs.insert(1, syn::parse_quote!(#[repr(align(#align))]));
            quote!(#c)
        }
        Item::Fn(mut f) => {
            f.attrs.insert(0, syn::parse_quote!(#[link_section = #section]));
            quote!(#f)
        }
        other => {
            quote! {
                #[link_section = #section]
                #[repr(align(#align))]
                #other
            }
        }
    };
    output.into()
}

pub(crate) fn section_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Punctuated<Meta, Token![,]>);

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
        other => {
            quote! {
                #[link_section = #section]
                #other
            }
        }
    };
    output.into()
}
