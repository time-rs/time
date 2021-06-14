mod component;
pub(crate) mod error;
pub(crate) mod modifier;
pub(crate) mod parse;

use proc_macro::{Literal, TokenStream};

pub(crate) use self::component::Component;
pub(crate) use self::parse::parse;
use crate::to_tokens::ToTokens;

mod helper {
    pub(crate) fn consume_whitespace<'a>(s: &'a str, index: &mut usize) -> &'a str {
        *index += s.len();
        let s = s.trim_start();
        *index -= s.len();
        s
    }
}

#[allow(single_use_lifetimes)] // false positive
#[allow(variant_size_differences)]
pub(crate) enum FormatItem<'a> {
    Literal(&'a str),
    Component(Component),
}

impl ToTokens for FormatItem<'_> {
    fn into_token_stream(self) -> TokenStream {
        quote! {
            ::time::format_description::FormatItem::#(match self {
                FormatItem::Literal(s) => quote! { Literal(#(Literal::byte_string(s.as_bytes()))) },
                FormatItem::Component(component) => quote! { Component(#(component)) },
            })
        }
    }
}
