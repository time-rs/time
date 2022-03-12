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
#![recursion_limit = "256"]

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
    (|| {
        let (span, string) = helpers::get_string_literal(input)?;
        let items = format_description::parse(&string, span)?;

        Ok(quote! {{
            const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[#(
                items
                    .into_iter()
                    .map(|item| quote! { #(item), })
                    .collect::<TokenStream>()
            )];
            DESCRIPTION
        }})
    })()
    .unwrap_or_else(|err: Error| err.to_compile_error())
}

fn make_serde_serializer_module(
    mod_name: proc_macro::Ident,
    items: impl to_tokens::ToTokens,
    formattable: TokenStream,
    format_string: &str,
) -> TokenStream {
    let visitor_struct = quote! {
        struct Visitor<T: ?Sized>(::core::marker::PhantomData<T>);

        impl<'a> ::serde::de::Visitor<'a> for Visitor<#(formattable.clone())> {
            type Value = #(formattable.clone());

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                // `write!` macro confuses `quote!` so format our message manually
                formatter.write_str("a(n) `")?;
                formatter.write_str(#(formattable.to_string()))?;
                formatter.write_str("` in the format \"")?;
                formatter.write_str(&FORMAT_STRING)?;
                formatter.write_str("\"")
            }

            fn visit_str<E: ::serde::de::Error>(
                self,
                value: &str
            ) -> Result<Self::Value, E> {
                #(formattable.clone())::parse(value, &DESCRIPTION).map_err(E::custom)
            }
        }

        impl<'a> ::serde::de::Visitor<'a> for Visitor<Option<#(formattable.clone())>> {
            type Value = Option<#(formattable.clone())>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                // `write!` macro confuses `quote!` so format our message manually
                formatter.write_str("an `Option<")?;
                formatter.write_str(#(formattable.to_string()))?;
                formatter.write_str(">` in the format \"")?;
                formatter.write_str(&FORMAT_STRING)?;
                formatter.write_str("\"")
            }

            fn visit_some<D: ::serde::de::Deserializer<'a>>(
                self,
                deserializer: D
            ) -> Result<Self::Value, D::Error> {
                let visitor = Visitor::<#(formattable.clone())>(::core::marker::PhantomData);
                deserializer
                    .deserialize_any(visitor)
                    .map(Some)
            }

            fn visit_none<E: ::serde::de::Error>(
                self
            ) -> Result<Option<#(formattable.clone())>, E> {
                Ok(None)
            }
        }

    };
    let serialize_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
            datetime: &#(formattable.clone()),
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            use ::serde::Serialize;
            datetime
                .format(&DESCRIPTION)
                .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
                .serialize(serializer)
        }

        pub fn deserialize<'a, D: ::serde::Deserializer<'a>>(
            deserializer: D
        ) -> Result<#(formattable.clone()), D::Error> {
            use ::serde::Deserialize;
            let visitor = Visitor::<#(formattable.clone())>(::core::marker::PhantomData);
            deserializer.deserialize_any(visitor)
        }
    };
    let option_serialize_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
            option: &Option<#(formattable.clone())>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            use ::serde::Serialize;
            option.map(|datetime| datetime.format(&DESCRIPTION))
                    .transpose()
                    .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
                    .serialize(serializer)
        }

        pub fn deserialize<'a, D: ::serde::Deserializer<'a>>(
            deserializer: D
        ) -> Result<Option<#(formattable.clone())>, D::Error> {
            use ::serde::Deserialize;
            let visitor = Visitor::<Option<#(formattable.clone())>>(::core::marker::PhantomData);
            deserializer.deserialize_option(visitor)
        }
    };

    quote! {mod #(mod_name) {
            use ::time::#(formattable.clone());

            const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[#(items)];
            const FORMAT_STRING: &str = #(format_string);

            #(visitor_struct)

            #(serialize_fns)

            pub mod option {
                use super::{DESCRIPTION, #(formattable), Visitor};

                #(option_serialize_fns)
            }
        }
    }
}

#[proc_macro]
pub fn declare_format_string(input: TokenStream) -> TokenStream {
    (|| {
        let mut tokens = input.into_iter().peekable();
        // First, an identifier (the desired module name)
        let mod_name = match tokens.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(tree) => Err(Error::UnexpectedToken { tree }),
            None => Err(Error::UnexpectedEndOfInput),
        }?;

        // Followed by a comma
        helpers::consume_punct(',', &mut tokens)?;

        // Then, the type to create serde serializers for (e.g., `OffsetDateTime`).
        let formattable = match tokens.next() {
            Some(tree @ TokenTree::Ident(_)) => Ok(tree),
            Some(tree) => Err(Error::UnexpectedToken { tree }),
            None => Err(Error::UnexpectedEndOfInput),
        }?;

        // Another comma
        helpers::consume_punct(',', &mut tokens)?;

        // Then, a string literal.
        let input = TokenStream::from_iter(tokens);
        let (span, format_string) = helpers::get_string_literal(input)?;

        let items = format_description::parse(&format_string, span)?;
        let items: TokenStream = items.into_iter().map(|item| quote! { #(item), }).collect();

        Ok(make_serde_serializer_module(
            mod_name,
            items,
            formattable.into(),
            std::str::from_utf8(&format_string).unwrap(),
        ))
    })()
    .unwrap_or_else(|err: Error| err.to_compile_error_standalone())
}
