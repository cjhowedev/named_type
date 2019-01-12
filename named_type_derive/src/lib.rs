//! This crate provides support for deriving the `NamedType` trait from the
//! `named_type` crate. See that crate for further documentation.

#![crate_type = "proc-macro"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate named_type;

use proc_macro::TokenStream;
use syn::{Lit, MetaItem, NestedMetaItem};

#[doc(hidden)]
#[proc_macro_derive(NamedType, attributes(named_type))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");

    let result = named_type_derive(ast).to_string();

    result
        .parse()
        .expect(&format!("Couldn't parse `{}` to tokens", result))
}

fn find_prefix_suffix(props: &Vec<NestedMetaItem>) -> Option<(&str, &str)> {
    let prefix = props
        .iter()
        .find(|item| match item {
            &&NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, _)) => {
                ident == "short_prefix"
            }
            _ => false,
        })
        .and_then(|item| match item {
            &NestedMetaItem::MetaItem(MetaItem::NameValue(_, ref value)) => match value {
                &Lit::Str(ref string, _) => Some(string.as_ref()),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or("");

    let suffix = props
        .iter()
        .find(|item| match item {
            &&NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, _)) => {
                ident.to_string() == "short_suffix"
            }
            _ => false,
        })
        .and_then(|item| match item {
            &NestedMetaItem::MetaItem(MetaItem::NameValue(_, ref value)) => match value {
                &Lit::Str(ref string, _) => Some(string.as_ref()),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or("");

    Some((prefix, suffix))
}

fn named_type_derive(ast: syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (prefix, suffix) = {
        ast.attrs
            .iter()
            .find(|attr| match &attr.value {
                &MetaItem::List(ref ident, _) => ident == "named_type",
                _ => false,
            })
            .and_then(|attr| match &attr.value {
                &MetaItem::List(_, ref props) => find_prefix_suffix(props),
                _ => None,
            })
            .unwrap_or(("", ""))
    };

    let short_type_name: String = format!("{}{}{}", prefix, name, suffix);

    quote! {
        impl #impl_generics NamedType for #name #ty_generics #where_clause {
            fn type_name() -> &'static str {
                concat!(module_path!(), "::", stringify!(#name))
            }

            fn short_type_name() -> &'static str {
                #short_type_name
            }
        }
    }
}
