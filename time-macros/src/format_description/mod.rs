mod component;
pub(crate) mod error;
pub(crate) mod modifier;
pub(crate) mod parse;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub(crate) use self::component::Component;
pub(crate) use self::parse::parse;
use crate::ToTokens;

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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FormatItem::Literal(s) => tokens.extend(
                [
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("time", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("format_description", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("FormatItem", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("Literal", Span::mixed_site())),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        TokenStream::from(TokenTree::Literal(Literal::byte_string(s.as_bytes()))),
                    )),
                ]
                .iter()
                .cloned()
                .collect::<TokenStream>(),
            ),
            FormatItem::Component(component) => tokens.extend(
                [
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("time", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("format_description", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("FormatItem", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("Component", Span::mixed_site())),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        component.to_token_stream(),
                    )),
                ]
                .iter()
                .cloned()
                .collect::<TokenStream>(),
            ),
        }
    }
}
