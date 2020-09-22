use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::fmt;

pub(crate) enum Error {
    MissingComponent { name: &'static str },
    InvalidComponent { name: &'static str, value: String },
    UnexpectedCharacter(char),
    ExpectedString,
    UnexpectedToken { tree: TokenTree },
    UnexpectedEndOfInput,
    Custom(String),
}

#[allow(clippy::use_self)]
impl fmt::Display for Error {
    #[allow(clippy::use_self)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingComponent { name } => write!(f, "missing component: {}", name),
            Error::InvalidComponent { name, value } => {
                write!(f, "invalid component: {} was {}", name, value)
            }
            Error::UnexpectedCharacter(char) => write!(f, "unexpected character: {}", char),
            Error::ExpectedString => f.write_str("expected string"),
            Error::UnexpectedToken { tree } => write!(f, "unexpected token: {}", tree),
            Error::UnexpectedEndOfInput => f.write_str("unexpected end of input"),
            Error::Custom(s) => f.write_str(s),
        }
    }
}

impl Error {
    pub(crate) fn to_compile_error(&self) -> TokenStream {
        [
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
            TokenStream::from(TokenTree::Ident(Ident::new("core", Span::call_site()))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
            TokenStream::from(TokenTree::Ident(Ident::new(
                "compile_error",
                Span::call_site(),
            ))),
            TokenStream::from(TokenTree::Punct(Punct::new('!', Spacing::Alone))),
            TokenStream::from(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from(TokenTree::Literal(Literal::string(&self.to_string()))),
            ))),
        ]
        .iter()
        .cloned()
        .collect()
    }
}
