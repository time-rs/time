mod component;
pub(super) mod modifier;

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
    Literal(Box<[u8]>),
    StringLiteral(Box<str>),
    Component(Component),
    Compound(Box<[Self]>),
    Optional { format: bool, item: Box<Self> },
    First(Box<[Self]>),
}

impl ToTokenStream for OwnedFormatItem {
    fn append_to(self, ts: &mut TokenStream) {
        match self.version {
            FormatDescriptionVersion::V1 | FormatDescriptionVersion::V2 => match self.inner {
                OwnedFormatItemInner::Literal(bytes) => quote_append! { ts
                    BorrowedFormatItem::Literal(#(Literal::byte_string(bytes.as_ref())))
                },
                OwnedFormatItemInner::StringLiteral(string) => quote_append! { ts
                    BorrowedFormatItem::StringLiteral(#(string.as_ref()))
                },
                OwnedFormatItemInner::Component(component) => quote_append! { ts
                    BorrowedFormatItem::Component { 0: #S(component) }
                },
                OwnedFormatItemInner::Compound(items) => {
                    let items = items
                        .into_vec()
                        .into_iter()
                        .map(|item| {
                            quote_! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote_append! { ts
                        BorrowedFormatItem::Compound { 0: &[#S(items)] }
                    }
                }
                OwnedFormatItemInner::Optional { format, item } => {
                    if !format {
                        bug!("v1 and v2 format descriptions must format optional items")
                    }
                    quote_append! { ts
                        BorrowedFormatItem::Optional {
                            0: &#S(Self { version: self.version, inner: *item })
                        }
                    }
                }
                OwnedFormatItemInner::First(items) => {
                    let items = items
                        .into_vec()
                        .into_iter()
                        .map(|item| {
                            quote_! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote_append! { ts
                        BorrowedFormatItem::First { 0: &[#S(items)] }
                    }
                }
            },
            FormatDescriptionVersion::V3 => match self.inner {
                OwnedFormatItemInner::Literal(_) => {
                    bug!("v3 format descriptions should never have non-UTF8 literals")
                }
                OwnedFormatItemInner::StringLiteral(string) => quote_append! { ts
                    FormatDescriptionV3Inner::BorrowedLiteral(#(string.as_ref()))
                },
                OwnedFormatItemInner::Component(component) => {
                    quote_append! { ts
                        FormatDescriptionV3Inner::Component(#S(component))
                    }
                }
                OwnedFormatItemInner::Compound(items) => {
                    let items = items
                        .into_iter()
                        .map(|item| {
                            quote_! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote_append! { ts
                        FormatDescriptionV3Inner::BorrowedCompound(const { &[#S(items)] })
                    }
                }
                OwnedFormatItemInner::Optional { format, item } => {
                    quote_append! { ts
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
                            quote_! { #S(Self { version: self.version, inner: item }), }
                        })
                        .collect::<TokenStream>();
                    quote_append! { ts
                        FormatDescriptionV3Inner::BorrowedFirst(const { &[#S(items)] })
                    }
                }
            },
        }
    }
}
