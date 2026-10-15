mod component;
pub(super) mod modifier;
mod optimizing;

use proc_macro::TokenStream;

pub(crate) use self::component::Component;
use crate::FormatDescriptionVersion;
use crate::to_tokens::ToTokenStream;

#[derive(Clone)]
pub(crate) struct OwnedFormatItem {
    pub(crate) version: FormatDescriptionVersion,
    pub(crate) inner: OwnedFormatItemInner,
}

#[derive(Clone)]
pub(crate) enum OwnedFormatItemInner {
    Literal(Vec<u8>),
    StringLiteral(String),
    Component(Component),
    Compound(Vec<Self>),
    Optional { format: bool, item: Box<Self> },
    First(Vec<Self>),
}

impl ToTokenStream for OwnedFormatItem {
    fn into_token_stream(self) -> TokenStream {
        match self.version {
            FormatDescriptionVersion::V1 | FormatDescriptionVersion::V2 => match self.inner {
                OwnedFormatItemInner::Literal(bytes) => quote! {
                    BorrowedFormatItem::Literal(#(Literal::byte_string(bytes.as_ref())))
                },
                OwnedFormatItemInner::StringLiteral(string) => quote! {
                    BorrowedFormatItem::StringLiteral(#(string.as_ref()))
                },
                OwnedFormatItemInner::Component(component) => quote! {
                    BorrowedFormatItem::Component { 0: #S(component) }
                },
                OwnedFormatItemInner::Compound(items) => {
                    let items = items
                        .into_iter()
                        .map(|item| {
                            quote! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote! {
                        BorrowedFormatItem::Compound { 0: &[#S(items)] }
                    }
                }
                OwnedFormatItemInner::Optional { format, item } => {
                    if !format {
                        bug!("v1 and v2 format descriptions must format optional items")
                    }
                    quote! {
                        BorrowedFormatItem::Optional {
                            0: &#S(Self { version: self.version, inner: *item })
                        }
                    }
                }
                OwnedFormatItemInner::First(items) => {
                    let items = items
                        .into_iter()
                        .map(|item| {
                            quote! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote! {
                        BorrowedFormatItem::First { 0: &[#S(items)] }
                    }
                }
            },
            FormatDescriptionVersion::V3 => match self.inner {
                OwnedFormatItemInner::Literal(_) => {
                    bug!("v3 format descriptions should never have non-UTF8 literals")
                }
                OwnedFormatItemInner::StringLiteral(string) => quote! {
                    FormatDescriptionV3Inner::BorrowedLiteral(#(string.as_ref()))
                },
                OwnedFormatItemInner::Component(component) => {
                    component.into_token_stream_with_enum_name("FormatDescriptionV3Inner")
                }
                OwnedFormatItemInner::Compound(items) => {
                    let items = items
                        .into_iter()
                        .map(|item| {
                            quote! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote! {
                        FormatDescriptionV3Inner::BorrowedCompound(const { &[#S(items)] })
                    }
                }
                OwnedFormatItemInner::Optional { format, item } => {
                    quote! {
                        FormatDescriptionV3Inner::BorrowedOptional {
                            format: #S(format),
                            item: &#S(Self { version: self.version, inner: *item })
                        }
                    }
                }
                OwnedFormatItemInner::First(items) => {
                    let items = items
                        .into_iter()
                        .map(|item| {
                            quote! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote! {
                        FormatDescriptionV3Inner::BorrowedFirst(const { &[#S(items)] })
                    }
                }
            },
        }
    }
}
