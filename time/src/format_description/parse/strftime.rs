use alloc::string::String;
use alloc::vec::Vec;
use core::iter;

use crate::error::InvalidFormatDescription;
use crate::format_description::parse::{
    Error, ErrorInner, Location, Spanned, SpannedValue, Unused, attach_location, unused,
};
use crate::format_description::{self, BorrowedFormatItem, Component, modifier};

/// Parse a sequence of items from the [`strftime` format description][strftime docs].
///
/// The only heap allocation required is for the `Vec` itself. All components are bound to the
/// lifetime of the input.
///
/// [strftime docs]: https://man7.org/linux/man-pages/man3/strftime.3.html
#[doc(alias = "parse_strptime_borrowed")]
#[inline]
pub fn parse_strftime_borrowed(
    s: &str,
) -> Result<Vec<BorrowedFormatItem<'_>>, InvalidFormatDescription> {
    let tokens = lex(s.as_bytes());
    let items = into_items(tokens).collect::<Result<_, _>>()?;
    Ok(items)
}

/// Parse a sequence of items from the [`strftime` format description][strftime docs].
///
/// This requires heap allocation for some owned items.
///
/// [strftime docs]: https://man7.org/linux/man-pages/man3/strftime.3.html
#[doc(alias = "parse_strptime_owned")]
#[inline]
pub fn parse_strftime_owned(
    s: &str,
) -> Result<format_description::OwnedFormatItem, InvalidFormatDescription> {
    parse_strftime_borrowed(s).map(Into::into)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Padding {
    /// The default padding for a numeric component. Indicated by no character.
    Default,
    /// Pad a numeric component with spaces. Indicated by an underscore.
    Spaces,
    /// Do not pad a numeric component. Indicated by a hyphen.
    None,
    /// Pad a numeric component with zeroes. Indicated by a zero.
    Zeroes,
}

enum Token<'a> {
    Literal(Spanned<&'a [u8]>),
    Component {
        _percent: Unused<Location>,
        padding: Spanned<Padding>,
        component: Spanned<u8>,
    },
}

#[inline]
fn lex(mut input: &[u8]) -> iter::Peekable<impl Iterator<Item = Result<Token<'_>, Error>>> {
    let mut iter = attach_location(input.iter()).peekable();

    iter::from_fn(move || {
        Some(Ok(match iter.next()? {
            (b'%', percent_loc) => match iter.next() {
                Some((padding @ (b'_' | b'-' | b'0'), padding_loc)) => {
                    let padding = match padding {
                        b'_' => Padding::Spaces,
                        b'-' => Padding::None,
                        b'0' => Padding::Zeroes,
                        _ => unreachable!(),
                    };
                    let (&component, component_loc) = iter.next()?;
                    input = &input[3..];
                    Token::Component {
                        _percent: unused(percent_loc),
                        padding: padding.spanned(padding_loc.to_self()),
                        component: component.spanned(component_loc.to_self()),
                    }
                }
                Some((&component, component_loc)) => {
                    input = &input[2..];
                    let span = component_loc.to_self();
                    Token::Component {
                        _percent: unused(percent_loc),
                        padding: Padding::Default.spanned(span),
                        component: component.spanned(span),
                    }
                }
                None => {
                    return Some(Err(Error {
                        _inner: unused(percent_loc.error("unexpected end of input")),
                        public: InvalidFormatDescription::Expected {
                            what: "valid escape sequence",
                            index: percent_loc.byte as usize,
                        },
                    }));
                }
            },
            (_, start_location) => {
                let mut bytes = 1;
                let mut end_location = start_location;

                while let Some((_, location)) = iter.next_if(|&(&byte, _)| byte != b'%') {
                    end_location = location;
                    bytes += 1;
                }

                let value = &input[..bytes];
                input = &input[bytes..];

                Token::Literal(value.spanned(start_location.to(end_location)))
            }
        }))
    })
    .peekable()
}

#[inline]
fn into_items<'iter, 'token, I>(
    mut tokens: iter::Peekable<I>,
) -> impl Iterator<Item = Result<BorrowedFormatItem<'token>, Error>> + use<'token, I>
where
    'token: 'iter,
    I: Iterator<Item = Result<Token<'token>, Error>> + 'iter,
{
    iter::from_fn(move || {
        let next = match tokens.next()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err)),
        };

        Some(match next {
            // Safety: `parse_strftime` functions only accept strings, so UTF-8 validation is
            // unnecessary
            Token::Literal(spanned) => Ok(BorrowedFormatItem::StringLiteral(unsafe {
                str::from_utf8_unchecked(*spanned)
            })),
            Token::Component {
                _percent,
                padding,
                component,
            } => parse_component(padding, component),
        })
    })
}

fn parse_component(
    padding: Spanned<Padding>,
    component: Spanned<u8>,
) -> Result<BorrowedFormatItem<'static>, Error> {
    let padding_or_default = |padding: Padding, default| match padding {
        Padding::Default => default,
        Padding::Spaces => modifier::Padding::Space,
        Padding::None => modifier::Padding::None,
        Padding::Zeroes => modifier::Padding::Zero,
    };

    /// Helper macro to create a component.
    macro_rules! component {
        ($name:ident { $($inner:tt)* }) => {
            BorrowedFormatItem::Component(Component::$name(modifier::$name {
                $($inner)*
            }))
        }
    }

    Ok(match *component {
        b'%' => BorrowedFormatItem::StringLiteral("%"),
        b'a' => component!(WeekdayShort {
            case_sensitive: true
        }),
        b'A' => component!(WeekdayLong {
            case_sensitive: true,
        }),
        b'b' | b'h' => component!(MonthShort {
            case_sensitive: true,
        }),
        b'B' => component!(MonthLong {
            case_sensitive: true,
        }),
        b'c' => BorrowedFormatItem::Compound(&[
            component!(WeekdayShort {
                case_sensitive: true,
            }),
            BorrowedFormatItem::StringLiteral(" "),
            component!(MonthShort {
                case_sensitive: true,
            }),
            BorrowedFormatItem::StringLiteral(" "),
            component!(Day {
                padding: modifier::Padding::Space
            }),
            BorrowedFormatItem::StringLiteral(" "),
            component!(Hour24 {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Minute {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Second {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(" "),
            #[cfg(feature = "large-dates")]
            component!(CalendarYearFullExtendedRange {
                padding: modifier::Padding::Zero,
                sign_is_mandatory: false,
            }),
            #[cfg(not(feature = "large-dates"))]
            component!(CalendarYearFullStandardRange {
                padding: modifier::Padding::Zero,
                sign_is_mandatory: false,
            }),
        ]),
        #[cfg(feature = "large-dates")]
        b'C' => component!(CalendarYearCenturyExtendedRange {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
            sign_is_mandatory: false,
        }),
        #[cfg(not(feature = "large-dates"))]
        b'C' => component!(CalendarYearCenturyStandardRange {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
            sign_is_mandatory: false,
        }),
        b'd' => component!(Day {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'D' => BorrowedFormatItem::Compound(&[
            component!(MonthNumerical {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral("/"),
            component!(Day {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral("/"),
            component!(CalendarYearLastTwo {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'e' => component!(Day {
            padding: padding_or_default(*padding, modifier::Padding::Space),
        }),
        b'F' => BorrowedFormatItem::Compound(&[
            #[cfg(feature = "large-dates")]
            component!(CalendarYearFullExtendedRange {
                padding: modifier::Padding::Zero,
                sign_is_mandatory: false,
            }),
            #[cfg(not(feature = "large-dates"))]
            component!(CalendarYearFullStandardRange {
                padding: modifier::Padding::Zero,
                sign_is_mandatory: false,
            }),
            BorrowedFormatItem::StringLiteral("-"),
            component!(MonthNumerical {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral("-"),
            component!(Day {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'g' => component!(IsoYearLastTwo {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        #[cfg(feature = "large-dates")]
        b'G' => component!(IsoYearFullExtendedRange {
            padding: modifier::Padding::Zero,
            sign_is_mandatory: false,
        }),
        #[cfg(not(feature = "large-dates"))]
        b'G' => component!(IsoYearFullStandardRange {
            padding: modifier::Padding::Zero,
            sign_is_mandatory: false,
        }),
        b'H' => component!(Hour24 {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'I' => component!(Hour12 {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'j' => component!(Ordinal {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'k' => component!(Hour24 {
            padding: padding_or_default(*padding, modifier::Padding::Space),
        }),
        b'l' => component!(Hour12 {
            padding: padding_or_default(*padding, modifier::Padding::Space),
        }),
        b'm' => component!(MonthNumerical {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'M' => component!(Minute {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'n' => BorrowedFormatItem::StringLiteral("\n"),
        b'O' => {
            return Err(Error {
                _inner: unused(ErrorInner {
                    _message: "unsupported modifier",
                    _span: component.span,
                }),
                public: InvalidFormatDescription::NotSupported {
                    what: "modifier",
                    context: "",
                    index: component.span.start.byte as usize,
                },
            });
        }
        b'p' => component!(Period {
            is_uppercase: true,
            case_sensitive: true
        }),
        b'P' => component!(Period {
            is_uppercase: false,
            case_sensitive: true
        }),
        b'r' => BorrowedFormatItem::Compound(&[
            component!(Hour12 {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Minute {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Second {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(" "),
            component!(Period {
                is_uppercase: true,
                case_sensitive: true,
            }),
        ]),
        b'R' => BorrowedFormatItem::Compound(&[
            component!(Hour24 {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Minute {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b's' => component!(UnixTimestampSecond {
            sign_is_mandatory: false,
        }),
        b'S' => component!(Second {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b't' => BorrowedFormatItem::StringLiteral("\t"),
        b'T' => BorrowedFormatItem::Compound(&[
            component!(Hour24 {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Minute {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Second {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'u' => component!(WeekdayMonday { one_indexed: true }),
        b'U' => component!(WeekNumberSunday {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'V' => component!(WeekNumberIso {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'w' => component!(WeekdaySunday { one_indexed: true }),
        b'W' => component!(WeekNumberMonday {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        b'x' => BorrowedFormatItem::Compound(&[
            component!(MonthNumerical {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral("/"),
            component!(Day {
                padding: modifier::Padding::Zero
            }),
            BorrowedFormatItem::StringLiteral("/"),
            component!(CalendarYearLastTwo {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'X' => BorrowedFormatItem::Compound(&[
            component!(Hour24 {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Minute {
                padding: modifier::Padding::Zero,
            }),
            BorrowedFormatItem::StringLiteral(":"),
            component!(Second {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'y' => component!(CalendarYearLastTwo {
            padding: padding_or_default(*padding, modifier::Padding::Zero),
        }),
        #[cfg(feature = "large-dates")]
        b'Y' => component!(CalendarYearFullExtendedRange {
            padding: modifier::Padding::Zero,
            sign_is_mandatory: false,
        }),
        #[cfg(not(feature = "large-dates"))]
        b'Y' => component!(CalendarYearFullStandardRange {
            padding: modifier::Padding::Zero,
            sign_is_mandatory: false,
        }),
        b'z' => BorrowedFormatItem::Compound(&[
            component!(OffsetHour {
                sign_is_mandatory: true,
                padding: modifier::Padding::Zero,
            }),
            component!(OffsetMinute {
                padding: modifier::Padding::Zero,
            }),
        ]),
        b'Z' => {
            return Err(Error {
                _inner: unused(ErrorInner {
                    _message: "unsupported component",
                    _span: component.span,
                }),
                public: InvalidFormatDescription::NotSupported {
                    what: "component",
                    context: "",
                    index: component.span.start.byte as usize,
                },
            });
        }
        _ => {
            return Err(Error {
                _inner: unused(ErrorInner {
                    _message: "invalid component",
                    _span: component.span,
                }),
                public: InvalidFormatDescription::InvalidComponentName {
                    name: String::from_utf8_lossy(&[*component]).into_owned(),
                    index: component.span.start.byte as usize,
                },
            });
        }
    })
}
