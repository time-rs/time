#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate,
    unstable_name_collisions
)]

mod date;
mod datetime;
mod error;
mod format_description;
mod helpers;
mod offset;
mod peeking_take_while;
mod time;

use std::iter;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use self::date::Date;
use self::datetime::DateTime;
use self::error::Error;
use self::offset::Offset;
use self::time::Time;

trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
}

#[allow(clippy::use_self)] // false positive
impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(TokenTree::Ident(Ident::new(
            if *self { "true" } else { "false" },
            Span::mixed_site(),
        ))))
    }
}

macro_rules! impl_macros {
    ($($name:ident : $type:ty)*) => {$(
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            let string = match helpers::get_string_literal(input) {
                Ok(string) => string,
                Err(err) => return err.to_compile_error(),
            };
            let chars = &mut string.chars().peekable();

            let value = match <$type>::parse(chars) {
                Ok(value) => value,
                Err(err) => return err.to_compile_error(),
            };

            match chars.peek() {
                Some(&char) => Error::UnexpectedCharacter(char).to_compile_error(),
                None => value.to_token_stream(),
            }
        }
    )*};
}

impl_macros! {
    date: Date
    datetime: DateTime
    offset: Offset
    time: Time
}

// TODO Gate this behind the the `formatting` or `parsing` feature flag when weak dependency
// features land.
#[proc_macro]
pub fn format_description(input: TokenStream) -> TokenStream {
    let string = match helpers::get_string_literal(input) {
        Ok(string) => string,
        Err(err) => return err.to_compile_error(),
    };

    let items = match format_description::parse(&string) {
        Ok(items) => items,
        Err(err) => return err.to_compile_error(),
    };

    let mut tokens = TokenStream::new();
    for item in items {
        tokens.extend(
            [
                item.to_token_stream(),
                TokenStream::from(TokenTree::Punct(Punct::new(',', Spacing::Alone))),
            ]
            .iter()
            .cloned()
            .collect::<TokenStream>(),
        );
    }

    helpers::const_block(
        [
            TokenTree::Punct(Punct::new('&', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Bracket, tokens)),
        ]
        .iter()
        .cloned()
        .collect(),
        [
            TokenTree::Punct(Punct::new('&', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
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
                    TokenTree::Punct(Punct::new('<', Spacing::Alone)),
                    TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("_", Span::mixed_site())),
                    TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                ]
                .iter()
                .cloned()
                .collect(),
            )),
        ]
        .iter()
        .cloned()
        .collect(),
    )
}
