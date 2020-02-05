//! This crate provides support for deriving the `NamedType` trait from the
//! `named_type` crate. See that crate for further documentation.

extern crate proc_macro; // must use extern crate for proc macros
                         // see here: https://users.rust-lang.org/t/how-to-use-proc-macro-on-rust-2018/20833/2

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, Ident, Lit, Meta, NestedMeta};

#[doc(hidden)]
#[proc_macro_derive(NamedType, attributes(named_type))]
pub fn derive(input: TokenStream) -> TokenStream {
    named_type_derive(syn::parse_macro_input!(input))
}

fn find_prefix_suffix<P>(props: &Punctuated<NestedMeta, P>) -> Option<(String, String)> {
    let prefix = props
        .iter()
        .find_map(|item| {
            if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
                if name_value.path.get_ident() == Some(&Ident::new("short_prefix", Span::call_site())) {
                    if let Lit::Str(s) = &name_value.lit {
                        return Some(s.value())
                    }
                }
            }
            None
        })
        .unwrap_or_else(String::new);

    let suffix = props
        .iter()
        .find_map(|item| {
            if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
                if name_value.path.get_ident() == Some(&Ident::new("short_suffix", Span::call_site())) {
                    if let Lit::Str(s) = &name_value.lit {
                        return Some(s.value())
                    }
                }
            }
            None
        })
        .unwrap_or_else(String::new);

    Some((prefix, suffix))
}

fn named_type_derive(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (prefix, suffix) = {
        ast.attrs
            .iter()
            .find_map(|attr| {
                if let Meta::List(meta_list) = attr.parse_meta().unwrap() {
                    if meta_list.path.get_ident() == Some(&Ident::new("named_type", Span::call_site())) {
                        return find_prefix_suffix(&meta_list.nested);
                    }
                }
                None
            })
            .unwrap_or_else(|| (String::new(), String::new()))
    };

    let short_type_name: String = format!("{}{}{}", prefix, name, suffix);

    quote!(
        impl #impl_generics NamedType for #name #ty_generics #where_clause {
            fn type_name() -> &'static str {
                concat!(module_path!(), "::", stringify!(#name))
            }

            fn short_type_name() -> &'static str {
                #short_type_name
            }
        }
    ).into()
}
