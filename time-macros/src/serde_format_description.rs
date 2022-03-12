use proc_macro::{Ident, TokenStream};

use crate::to_tokens;

pub(crate) fn build(
    mod_name: Ident,
    items: impl to_tokens::ToTokens,
    ty: TokenStream,
    format_string: &str,
) -> TokenStream {
    let visitor = quote! {
        struct Visitor<T: ?Sized>(::core::marker::PhantomData<T>);

        impl<'a> ::serde::de::Visitor<'a> for Visitor<#(ty.clone())> {
            type Value = #(ty.clone());

            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "a(n) `",
                        #(ty.to_string()),
                        "` in the format \"",
                        #(format_string),
                        "\"",
                    )
                )
            }

            fn visit_str<E: ::serde::de::Error>(
                self,
                value: &str
            ) -> Result<Self::Value, E> {
                #(ty.clone())::parse(value, &DESCRIPTION).map_err(E::custom)
            }
        }

        impl<'a> ::serde::de::Visitor<'a> for Visitor<Option<#(ty.clone())>> {
            type Value = Option<#(ty.clone())>;

            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "an `Option<",
                        #(ty.to_string()),
                        ">` in the format \"",
                        #(format_string),
                        "\"",
                    )
                )
            }

            fn visit_some<D: ::serde::de::Deserializer<'a>>(
                self,
                deserializer: D
            ) -> Result<Self::Value, D::Error> {
                let visitor = Visitor::<#(ty.clone())>(::core::marker::PhantomData);
                deserializer
                    .deserialize_any(visitor)
                    .map(Some)
            }

            fn visit_none<E: ::serde::de::Error>(
                self
            ) -> Result<Option<#(ty.clone())>, E> {
                Ok(None)
            }
        }

    };

    let primary_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
            datetime: &#(ty.clone()),
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
        ) -> Result<#(ty.clone()), D::Error> {
            use ::serde::Deserialize;
            let visitor = Visitor::<#(ty.clone())>(::core::marker::PhantomData);
            deserializer.deserialize_any(visitor)
        }
    };

    let options_fns = quote! {
        pub fn serialize<S: ::serde::Serializer>(
            option: &Option<#(ty.clone())>,
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
        ) -> Result<Option<#(ty.clone())>, D::Error> {
            use ::serde::Deserialize;
            let visitor = Visitor::<Option<#(ty.clone())>>(::core::marker::PhantomData);
            deserializer.deserialize_option(visitor)
        }
    };

    quote! {
        mod #(mod_name) {
            use ::time::#(ty.clone());

            const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[#(items)];

            #(visitor)
            #(primary_fns)

            pub(super) mod option {
                use super::{DESCRIPTION, #(ty), Visitor};

                #(options_fns)
            }
        }
    }
}
