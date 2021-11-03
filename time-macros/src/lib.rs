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
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unwrap_used,
    clippy::use_debug,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences
)]
#![allow(clippy::missing_const_for_fn, clippy::redundant_pub_crate)]

#[macro_use]
mod quote;

mod date;
mod datetime;
mod error;
mod format_description;
mod helpers;
mod offset;
mod time;
mod to_tokens;

use std::iter::FromIterator;

use proc_macro::{TokenStream, TokenTree};

use self::error::Error;

macro_rules! impl_macros {
    ($($name:ident)*) => {$(
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            use crate::to_tokens::ToTokens;

            let mut iter = input.into_iter().peekable();
            match $name::parse(&mut iter) {
                Ok(value) => match iter.peek() {
                    Some(tree) => Error::UnexpectedToken { tree: tree.clone() }.to_compile_error(),
                    None => value.into_token_stream(),
                },
                Err(err) => err.to_compile_error(),
            }
        }
    )*};
}

impl_macros![date datetime offset time];

// TODO Gate this behind the the `formatting` or `parsing` feature flag when weak dependency
// features land.
#[proc_macro]
pub fn format_description(input: TokenStream) -> TokenStream {
    let (span, string) = match helpers::get_string_literal(input) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error(),
    };

    let items = match format_description::parse(&string, span) {
        Ok(items) => items,
        Err(err) => return err.to_compile_error(),
    };

    quote! {{
        const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[#(
            items
                .into_iter()
                .map(|item| quote! { #(item), })
                .collect::<TokenStream>()
        )];
        DESCRIPTION
    }}
}

fn make_serde_serializer_module(
    mod_name: proc_macro::Ident,
    items: impl to_tokens::ToTokens,
) -> TokenStream {
    let serialize_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
                datetime: &::time::OffsetDateTime,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                use ::serde::Serialize;
                datetime.format(&DESCRIPTION)
                    .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
                    .serialize(serializer)
        }

        pub fn deserialize<'a, D: ::serde::Deserializer<'a> >(
                deserializer: D
        ) -> Result<::time::OffsetDateTime, D::Error> {
            use ::serde::Deserialize;
                ::time::OffsetDateTime::parse(<&str>::deserialize(deserializer)?, &DESCRIPTION)
                    .map_err(time::error::Parse::to_invalid_serde_value::<D>)
        }
    };
    let option_serialize_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
            option: &Option<::time::OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            use ::serde::Serialize;
            option.map(|datetime| datetime.format(&DESCRIPTION))
                    .transpose()
                    .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
                    .serialize(serializer)
        }

        pub fn deserialize<'a, D: ::serde::Deserializer<'a> >(
            deserializer: D
        ) -> Result<Option<::time::OffsetDateTime>, D::Error> {
            use ::serde::Deserialize;
            Option::<&str>::deserialize(deserializer)?
                .map(|string| ::time::OffsetDateTime::parse(string, &DESCRIPTION))
                .transpose()
                .map_err(time::error::Parse::to_invalid_serde_value::<D>)
        }
    };

    quote! {
        mod #(mod_name) {
            const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[#(items)];

            #(serialize_fns)

            pub mod option {
                use super::DESCRIPTION;

                #(option_serialize_fns)
            }
        }
    }
}

#[proc_macro]
pub fn declare_format_string(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    // First, an identifier (the desired module name)
    let mod_name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tree) => {
            return Error::UnexpectedToken { tree }.to_compile_error_standalone();
        }
        None => return Error::UnexpectedEndOfInput.to_compile_error_standalone(),
    };
    // Followed by a comma
    match tokens.next() {
        Some(tree) => {
            if let TokenTree::Punct(ref punct) = tree {
                if punct.as_char() != ',' {
                    return Error::UnexpectedToken { tree }.to_compile_error_standalone();
                }
            }
        }
        None => {
            return Error::UnexpectedEndOfInput.to_compile_error_standalone();
        }
    };
    // Then, a string literal.
    let input = TokenStream::from_iter(tokens);
    let (span, string) = match helpers::get_string_literal(input) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error_standalone(),
    };

    let items = match format_description::parse(&string, span) {
        Ok(items) => items,
        Err(err) => return err.to_compile_error_standalone(),
    };
    let items: TokenStream = items.into_iter().map(|item| quote! { #(item), }).collect();

    make_serde_serializer_module(mod_name, items)
}
