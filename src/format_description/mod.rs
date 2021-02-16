//! Description of how types should be formatted and parsed.

mod component;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

#[cfg(all(feature = "alloc", feature = "formatting"))]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "formatting")]
use core::fmt;

pub use self::component::Component;
#[cfg(feature = "alloc")]
pub use self::parse::parse;
pub(crate) use self::sealed::FormatDescription;
use crate::error;
#[cfg(feature = "formatting")]
use crate::formatting::format_component;
#[cfg(feature = "parsing")]
use crate::parsing::{combinator, Parsed};
#[cfg(feature = "formatting")]
use crate::{Date, Time, UtcOffset};

/// Helper methods.
#[cfg(feature = "alloc")]
mod helper {
    /// Consume all leading whitespace, advancing `index` as appropriate.
    #[must_use = "This does not modify the original string."]
    pub(crate) fn consume_whitespace<'a>(s: &'a str, index: &mut usize) -> &'a str {
        *index += s.len();
        let s = s.trim_start();
        *index -= s.len();
        s
    }
}

/// A complete description of how to format and parse a type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatItem<'a> {
    /// A string that is formatted as-is.
    Literal(&'a str),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    Compound(&'a [Self]),
}

/// Seal the `FormatDescription` trait to prevent downstream users from implementing it, while still
/// allowing them to use it.
pub(crate) mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Format the item using a format description, the intended output, and the various components.
    #[allow(unreachable_pub)] // That's the point.
    pub trait FormatDescription<'a> {
        /// An error that may be returned when formatting.
        #[cfg(feature = "formatting")]
        #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
        type FormatError;
        /// An error that may be returned when parsing.
        #[cfg(feature = "parsing")]
        #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
        type ParseError: Into<error::Parse>;

        /// Format the item into the provided output.
        #[cfg(feature = "formatting")]
        fn format_into(
            &self,
            output: &mut impl fmt::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<(), Self::FormatError>;

        /// Format the item directly to a `String`.
        #[cfg(all(feature = "formatting", feature = "alloc"))]
        #[cfg_attr(
            __time_03_docs,
            doc(cfg(all(feature = "formatting", feature = "alloc")))
        )]
        fn format(
            &self,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<String, Self::FormatError> {
            let mut s = String::new();
            self.format_into(&mut s, date, time, offset)?;
            Ok(s)
        }

        /// Parse the item into the provided [`Parsed`] struct.
        #[cfg(feature = "parsing")]
        #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
        fn parse_into(
            &self,
            input: &'a str,
            parsed: &mut Parsed,
        ) -> Result<&'a str, Self::ParseError>;

        /// Parse the item into a new [`Parsed`] struct.
        #[cfg(feature = "parsing")]
        #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
        fn parse(&self, input: &'a str) -> Result<Parsed, Self::ParseError> {
            let mut parsed = Parsed::new();
            self.parse_into(input, &mut parsed)?;
            Ok(parsed)
        }
    }
}

impl<'a> FormatDescription<'a> for FormatItem<'a> {
    #[cfg(feature = "formatting")]
    type FormatError = error::Format;
    #[cfg(feature = "parsing")]
    type ParseError = error::ParseFromDescription;

    #[cfg(feature = "formatting")]
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::FormatError> {
        match *self {
            Self::Literal(literal) => output.write_str(literal)?,
            Self::Component(component) => format_component(output, component, date, time, offset)?,
            Self::Compound(items) => items.format_into(output, date, time, offset)?,
        }

        Ok(())
    }

    #[cfg(feature = "parsing")]
    fn parse_into(
        &self,
        mut input: &'a str,
        parsed: &mut Parsed,
    ) -> Result<&'a str, Self::ParseError> {
        match self {
            Self::Literal(literal) => {
                input = combinator::string(literal)(input)
                    .ok_or(error::ParseFromDescription::InvalidLiteral)?
                    .0;
            }
            Self::Component(component) => input = parsed.parse_component(input, *component)?,
            Self::Compound(compound) => input = compound.parse_into(input, parsed)?,
        }

        Ok(input)
    }
}

impl<'a> FormatDescription<'a> for &[FormatItem<'a>] {
    #[cfg(feature = "formatting")]
    type FormatError = error::Format;
    #[cfg(feature = "parsing")]
    type ParseError = error::ParseFromDescription;

    #[cfg(feature = "formatting")]
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::FormatError> {
        for item in self.iter() {
            item.format_into(output, date, time, offset)?;
        }
        Ok(())
    }

    #[cfg(feature = "parsing")]
    fn parse_into(
        &self,
        mut input: &'a str,
        parsed: &mut Parsed,
    ) -> Result<&'a str, Self::ParseError> {
        for item in self.iter() {
            input = item.parse_into(input, parsed)?;
        }
        Ok(input)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl<'a> FormatDescription<'a> for Vec<FormatItem<'a>> {
    #[cfg(feature = "formatting")]
    type FormatError = <&'a [FormatItem<'a>] as FormatDescription<'a>>::FormatError;
    #[cfg(feature = "parsing")]
    type ParseError = <&'a [FormatItem<'a>] as FormatDescription<'a>>::ParseError;

    #[cfg(feature = "formatting")]
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::FormatError> {
        self.as_slice().format_into(output, date, time, offset)
    }

    #[cfg(feature = "parsing")]
    fn parse_into(&self, input: &'a str, parsed: &mut Parsed) -> Result<&'a str, Self::ParseError> {
        self.as_slice().parse_into(input, parsed)
    }
}
