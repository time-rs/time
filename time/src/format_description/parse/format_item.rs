//! Typed, validated representation of a parsed format description.

use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use core::num::NonZero;
use core::str::{self, FromStr};

use super::{
    Error, FormatDescriptionVersion, Location, OptionExt as _, Span, Spanned, SpannedValue as _,
    ast, unused,
};
use crate::error::InvalidFormatDescription;
use crate::internal_macros::bug;

macro_rules! parse_modifiers {
    ($modifiers:expr, struct {}) => {{
        struct Parsed {}
        drop($modifiers);
        Ok(Parsed {})
    }};
    ($modifiers:expr, struct { $($field:ident : $modifier:ident),* $(,)? }) => {
        'block: {
            struct Parsed {
                $($field: Option<Spanned<<$modifier as ModifierValue>::Type>>),*
            }

            let mut parsed = Parsed {
                $($field: None),*
            };

            for modifier in $modifiers {
                $(if modifier.key.eq_ignore_ascii_case(stringify!($field)) {
                    if parsed.$field.is_some() {
                        break 'block Err(Error {
                            _inner: unused(modifier.key.span.error("duplicate modifier key")),
                            public: InvalidFormatDescription::DuplicateModifier {
                                name: stringify!($field),
                                index: modifier.key.span.start.byte as usize,
                            }
                        });
                    }
                    match <$modifier>::from_modifier_value(&modifier.value) {
                        Ok(value) => {
                            parsed.$field = Some(
                                <<$modifier as ModifierValue>::Type>::from(value)
                                    .spanned(modifier.value.span)
                            )
                        },
                        Err(err) => break 'block Err(err),
                    }
                    continue;
                })*
                break 'block Err(Error {
                    _inner: unused(modifier.key.span.error("invalid modifier key")),
                    public: InvalidFormatDescription::InvalidModifier {
                        value: (**modifier.key).to_owned(),
                        index: modifier.key.span.start.byte as usize,
                    }
                });
            }

            Ok(parsed)
        }
    };
}

/// Parse an AST iterator into a sequence of format items.
#[inline]
pub(super) fn parse<'a>(
    ast_items: impl Iterator<Item = Result<ast::Item<'a>, Error>>,
) -> impl Iterator<Item = Result<Item<'a>, Error>> {
    ast_items.map(|ast_item| ast_item.and_then(Item::from_ast))
}

/// A description of how to format and parse one part of a type.
pub(super) enum Item<'a> {
    /// A literal string.
    Literal(&'a str),
    /// Part of a type, along with its modifiers.
    Component(AstComponent),
    /// A sequence of optional items.
    Optional {
        /// The items themselves.
        value: Box<[Self]>,
        /// Whether the value should be formatted.
        format: Spanned<bool>,
        /// The span of the full sequence.
        span: Span,
    },
    /// The first matching parse of a sequence of format descriptions.
    First {
        /// The sequence of format descriptions.
        value: Box<[Box<[Self]>]>,
        /// The span of the full sequence.
        span: Span,
    },
}

impl<'a> Item<'a> {
    /// Parse an AST item into a format item.
    pub(super) fn from_ast(ast_item: ast::Item<'a>) -> Result<Self, Error> {
        Ok(match ast_item {
            ast::Item::Literal(Spanned { value, span: _ }) => Item::Literal(value),
            ast::Item::Component {
                version,
                opening_bracket,
                _leading_whitespace: _,
                name,
                modifiers,
                nested_format_descriptions,
                _trailing_whitespace: _,
                closing_bracket,
            } => {
                // Perform additional syntactic checks that are required, even though not
                // semantically relevant.

                if let Some(first_nested_fd) = nested_format_descriptions.first()
                    && first_nested_fd.leading_whitespace.is_none()
                {
                    return Err(Error {
                        _inner: unused(
                            opening_bracket.to(closing_bracket).error(
                                "missing leading whitespace before nested format description",
                            ),
                        ),
                        public: InvalidFormatDescription::Expected {
                            what: "whitespace before nested format description",
                            index: first_nested_fd.opening_bracket.byte as usize,
                        },
                    });
                }

                // Parse the actual component, starting with those that require nested format
                // descriptions.

                if name.eq_ignore_ascii_case("optional") {
                    Self::optional_from_parts(
                        opening_bracket,
                        &modifiers,
                        nested_format_descriptions,
                        closing_bracket,
                    )?
                } else if name.eq_ignore_ascii_case("first") {
                    let _modifiers = parse_modifiers!(modifiers, struct {})?;

                    if version.is_at_least_v3() && nested_format_descriptions.is_empty() {
                        return Err(Error {
                            _inner: unused(opening_bracket.to(closing_bracket).error(
                                "the `first` component requires at least one nested format \
                                 description",
                            )),
                            public: InvalidFormatDescription::Expected {
                                what: "at least one nested format description",
                                index: closing_bracket.byte as usize,
                            },
                        });
                    }

                    let items = nested_format_descriptions
                        .into_iter()
                        .map(|nested_format_description| {
                            nested_format_description
                                .items
                                .into_iter()
                                .map(Item::from_ast)
                                .collect()
                        })
                        .collect::<Result<_, _>>()?;

                    Item::First {
                        value: items,
                        span: opening_bracket.to(closing_bracket),
                    }
                } else {
                    // Ensure no nested format descriptions are present.
                    if !nested_format_descriptions.is_empty() {
                        return Err(Error {
                            _inner: unused(opening_bracket.to(closing_bracket).error(
                                "this component does not support nested format descriptions",
                            )),
                            public: InvalidFormatDescription::NotSupported {
                                what: "nested format descriptions",
                                context: "on this component",
                                index: opening_bracket.byte as usize,
                            },
                        });
                    }

                    let mut component = component_from_ast(version, &name, &modifiers)?;
                    // v3 format descriptions default to `range:standard` rather than
                    // `range:extended` for v1 and v2.
                    if version.is_at_least_v3()
                        && let AstComponent::Year(y) = &mut component
                        && y.range.value.is_none()
                    {
                        y.range = Some(YearRange::Standard).spanned(Span::DUMMY);
                    }
                    Item::Component(component)
                }
            }
        })
    }

    fn optional_from_parts(
        opening_bracket: Location,
        modifiers: &[ast::Modifier<'_>],
        nested_format_descriptions: Box<[ast::NestedFormatDescription<'a>]>,
        closing_bracket: Location,
    ) -> Result<Self, Error> {
        let modifiers = parse_modifiers!(modifiers, struct {
            format: OptionalFormat,
        })?;

        let [nested_format_description] = if let Some(second_fd) = nested_format_descriptions.get(1)
        {
            return Err(Error {
                _inner: unused(
                    second_fd
                        .opening_bracket
                        .to(second_fd.closing_bracket)
                        .error(
                            "the `optional` component only allows a single nested format \
                             description",
                        ),
                ),
                public: InvalidFormatDescription::NotSupported {
                    what: "more than one nested format description",
                    context: "`optional` components",
                    index: second_fd.opening_bracket.byte as usize,
                },
            });
        } else if let Ok(nested_format_description) =
            <Box<[_; 1]>>::try_from(nested_format_descriptions)
        {
            *nested_format_description
        } else {
            return Err(Error {
                _inner: unused(
                    opening_bracket
                        .to(closing_bracket)
                        .error("missing nested format description for `optional` component"),
                ),
                public: InvalidFormatDescription::Expected {
                    what: "nested format description",
                    index: closing_bracket.byte as usize,
                },
            });
        };

        let format = modifiers.format.transpose().map(|val| val.unwrap_or(true));
        let items = nested_format_description
            .items
            .into_iter()
            .map(Item::from_ast)
            .collect::<Result<_, _>>()?;

        Ok(Item::Optional {
            value: items,
            format,
            span: opening_bracket.to(closing_bracket),
        })
    }
}

impl<'a> TryFrom<Item<'a>> for crate::format_description::BorrowedFormatItem<'a> {
    type Error = Error;

    #[inline]
    fn try_from(item: Item<'a>) -> Result<Self, Self::Error> {
        match item {
            Item::Literal(literal) => Ok(Self::StringLiteral(literal)),
            Item::Component(component) => Ok(Self::Component(component.try_into()?)),
            Item::Optional {
                value: _,
                format: _,
                span,
            } => Err(Error {
                _inner: unused(span.error(
                    "optional items are not supported in runtime-parsed format descriptions",
                )),
                public: InvalidFormatDescription::NotSupported {
                    what: "optional item",
                    context: "runtime-parsed format descriptions",
                    index: span.start.byte as usize,
                },
            }),
            Item::First { value: _, span } => Err(Error {
                _inner: unused(span.error(
                    "'first' items are not supported in runtime-parsed format descriptions",
                )),
                public: InvalidFormatDescription::NotSupported {
                    what: "'first' item",
                    context: "runtime-parsed format descriptions",
                    index: span.start.byte as usize,
                },
            }),
        }
    }
}

impl TryFrom<Item<'_>> for crate::format_description::OwnedFormatItem {
    type Error = Error;

    #[inline]
    fn try_from(item: Item<'_>) -> Result<Self, Self::Error> {
        match item {
            Item::Literal(literal) => Ok(Self::StringLiteral(literal.to_owned().into_boxed_str())),
            Item::Component(component) => Ok(Self::Component(component.try_into()?)),
            Item::Optional {
                value,
                format,
                span: _,
            } => {
                if !*format {
                    return Err(Error {
                        _inner: unused(format.span.error(
                            "v1 and v2 format descriptions do not support optional items that are \
                             not formatted",
                        )),
                        public: InvalidFormatDescription::NotSupported {
                            what: "optional item with `format:false`",
                            context: "v1 and v2 format descriptions",
                            index: format.span.start.byte as usize,
                        },
                    });
                }
                Ok(Self::Optional(Box::new(value.try_into()?)))
            }
            Item::First { value, span: _ } => Ok(Self::First(
                value
                    .into_vec()
                    .into_iter()
                    .map(Self::try_from)
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

impl<'a> TryFrom<Box<[Item<'a>]>> for crate::format_description::OwnedFormatItem {
    type Error = Error;

    #[inline]
    fn try_from(items: Box<[Item<'a>]>) -> Result<Self, Self::Error> {
        let items = items.into_vec();
        match <[_; 1]>::try_from(items) {
            Ok([item]) => item.try_into(),
            Err(vec) => Ok(Self::Compound(
                vec.into_iter()
                    .map(Self::try_from)
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

impl<'a> TryFrom<Item<'a>> for crate::format_description::__private::FormatDescriptionV3Inner<'a> {
    type Error = Error;

    #[inline]
    fn try_from(item: Item<'a>) -> Result<Self, Self::Error> {
        match item {
            Item::Literal(literal) => Ok(Self::BorrowedLiteral(literal)),
            Item::Component(component) => Ok(component.try_into()?),
            Item::Optional {
                value,
                format,
                span: _,
            } => Ok(Self::OwnedOptional {
                format: *format,
                item: Box::new(value.try_into()?),
            }),
            Item::First { value, span: _ } => Ok(Self::OwnedFirst(
                value
                    .into_vec()
                    .into_iter()
                    .map(Self::try_from)
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

impl<'a> TryFrom<Box<[Item<'a>]>>
    for crate::format_description::__private::FormatDescriptionV3Inner<'a>
{
    type Error = Error;

    #[inline]
    fn try_from(items: Box<[Item<'a>]>) -> Result<Self, Self::Error> {
        let items = items.into_vec();
        match <[_; 1]>::try_from(items) {
            Ok([item]) => item.try_into(),
            Err(vec) => Ok(Self::OwnedCompound(
                vec.into_iter()
                    .map(Self::try_from)
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

/// Declare the `Component` struct.
macro_rules! component_definition {
    (@if_required required then { $($then:tt)* } $(else { $($else:tt)* })?) => { $($then)* };
    (@if_required then { $($then:tt)* } $(else { $($else:tt)* })?) => { $($($else)*)? };
    (@if_from_str from_str then { $($then:tt)* } $(else { $($else:tt)* })?) => { $($then)* };
    (@if_from_str then { $($then:tt)* } $(else { $($else:tt)* })?) => { $($($else)*)? };

    ($vis:vis enum $name:ident {$(
        $variant:ident = $parse_variant:literal {$(
            $(#[$required:tt])?
            $field:ident = $parse_field:literal:
            Option<$(#[$from_str:tt])? $field_type:ty>
        ),* $(,)?}
    ),* $(,)?}) => {
        $vis enum $name {
            $($variant($variant),)*
        }

        $($vis struct $variant {
            $($field: Spanned<Option<$field_type>>),*
        })*

        $(impl $variant {
            /// Parse the component from the AST, given its modifiers.
            #[inline]
            fn with_modifiers(
                version: FormatDescriptionVersion,
                modifiers: &[ast::Modifier<'_>],
                _component_span: Span,
            ) -> Result<Self, Error>
            {
                // rustc will complain if the modifier is empty.
                #[allow(unused_mut)]
                let mut this = Self {
                    $($field: None.spanned(Span::DUMMY)),*
                };

                for modifier in modifiers {
                    $(if modifier.key.eq_ignore_ascii_case($parse_field) {
                        if version.is_at_least_v3() && this.$field.is_some() {
                            return Err(Error {
                                _inner: unused(modifier.key.span.error("duplicate modifier key")),
                                public: InvalidFormatDescription::DuplicateModifier {
                                    name: stringify!($field),
                                    index: modifier.key.span.start.byte as usize,
                                }
                            });
                        }
                        this.$field = Some(
                            component_definition!(@if_from_str $($from_str)?
                                then {
                                    parse_from_modifier_value::<$field_type>(&modifier.value)?
                                } else {
                                    <$field_type>::from_modifier_value(&modifier.value)?
                                }
                            )
                        ).spanned(modifier.key_value_span());
                        continue;
                    })*
                    return Err(Error {
                        _inner: unused(modifier.key.span.error("invalid modifier key")),
                        public: InvalidFormatDescription::InvalidModifier {
                            value: (**modifier.key).to_owned(),
                            index: modifier.key.span.start.byte as usize,
                        }
                    });
                }

                $(component_definition! { @if_required $($required)? then {
                    if this.$field.is_none() {
                        return Err(Error {
                            _inner: unused(_component_span.error("missing required modifier")),
                            public:
                                InvalidFormatDescription::MissingRequiredModifier {
                                    name: $parse_field,
                                    index: _component_span.start.byte as usize,
                                }
                        });
                    }
                }})*

                Ok(this)
            }
        })*

        /// Parse a component from the AST, given its name and modifiers.
        #[inline]
        fn component_from_ast(
            version: FormatDescriptionVersion,
            name: &Spanned<&str>,
            modifiers: &[ast::Modifier<'_>],
        ) -> Result<AstComponent, Error> {
            $(if name.eq_ignore_ascii_case($parse_variant) {
                return Ok(AstComponent::$variant(
                    $variant::with_modifiers(version, &modifiers, name.span)?
                ));
            })*
            Err(Error {
                _inner: unused(name.span.error("invalid component")),
                public: InvalidFormatDescription::InvalidComponentName {
                    name: (**name).to_owned(),
                    index: name.span.start.byte as usize,
                },
            })
        }
    }
}

// Keep in alphabetical order.
component_definition! {
    pub(super) enum AstComponent {
        Day = "day" {
            padding = "padding": Option<Padding>,
        },
        End = "end" {
            trailing_input = "trailing_input": Option<TrailingInput>,
        },
        Hour = "hour" {
            padding = "padding": Option<Padding>,
            base = "repr": Option<HourBase>,
        },
        Ignore = "ignore" {
            #[required]
            count = "count": Option<#[from_str] NonZero<u16>>,
        },
        Minute = "minute" {
            padding = "padding": Option<Padding>,
        },
        Month = "month" {
            padding = "padding": Option<Padding>,
            repr = "repr": Option<MonthRepr>,
            case_sensitive = "case_sensitive": Option<MonthCaseSensitive>,
        },
        OffsetHour = "offset_hour" {
            sign_behavior = "sign": Option<SignBehavior>,
            padding = "padding": Option<Padding>,
        },
        OffsetMinute = "offset_minute" {
            padding = "padding": Option<Padding>,
        },
        OffsetSecond = "offset_second" {
            padding = "padding": Option<Padding>,
        },
        Ordinal = "ordinal" {
            padding = "padding": Option<Padding>,
        },
        Period = "period" {
            case = "case": Option<PeriodCase>,
            case_sensitive = "case_sensitive": Option<PeriodCaseSensitive>,
        },
        Second = "second" {
            padding = "padding": Option<Padding>,
        },
        Subsecond = "subsecond" {
            digits = "digits": Option<SubsecondDigits>,
        },
        UnixTimestamp = "unix_timestamp" {
            precision = "precision": Option<UnixTimestampPrecision>,
            sign_behavior = "sign": Option<SignBehavior>,
        },
        Weekday = "weekday" {
            repr = "repr": Option<WeekdayRepr>,
            one_indexed = "one_indexed": Option<WeekdayOneIndexed>,
            case_sensitive = "case_sensitive": Option<WeekdayCaseSensitive>,
        },
        WeekNumber = "week_number" {
            padding = "padding": Option<Padding>,
            repr = "repr": Option<WeekNumberRepr>,
        },
        Year = "year" {
            padding = "padding": Option<Padding>,
            repr = "repr": Option<YearRepr>,
            range = "range": Option<YearRange>,
            base = "base": Option<YearBase>,
            sign_behavior = "sign": Option<SignBehavior>,
        },
    }
}

macro_rules! impl_from_ast_component_for {
    ($([$reject_nonsensical:literal] $ty:ty),+ $(,)?) => {$(
        impl TryFrom<AstComponent> for $ty {
            type Error = Error;

            #[inline]
            fn try_from(component: AstComponent) -> Result<Self, Self::Error> {
                macro_rules! reject_modifier {
                    ($modifier:ident, $modifier_str:literal, $context:literal) => {
                        if $reject_nonsensical && $modifier.value.is_some() {
                            return Err(Error {
                                _inner: unused($modifier.span.error(concat!(
                                    "the '",
                                    $modifier_str,
                                    "' modifier is not valid ",
                                    $context
                                ))),
                                public: InvalidFormatDescription::InvalidModifierCombination {
                                    modifier: $modifier_str,
                                    context: $context,
                                    index: $modifier.span.start.byte as usize,
                                },
                            });
                        }
                    };
                }

                use crate::format_description::modifier;
                Ok(match component {
                    AstComponent::Day(Day { padding }) => Self::Day(modifier::Day {
                        padding: padding.unwrap_or_default().into(),
                    }),
                    AstComponent::End(End { trailing_input }) => Self::End(modifier::End {
                        trailing_input: trailing_input.unwrap_or_default().into(),
                    }),
                    AstComponent::Hour(Hour { padding, base }) => match base.unwrap_or_default() {
                        HourBase::Twelve => Self::Hour12(modifier::Hour12 {
                            padding: padding.unwrap_or_default().into(),
                        }),
                        HourBase::TwentyFour => Self::Hour24(modifier::Hour24 {
                            padding: padding.unwrap_or_default().into(),
                        }),
                    },
                    AstComponent::Ignore(Ignore { count }) => Self::Ignore(modifier::Ignore {
                        count: match *count {
                            Some(value) => value,
                            None => bug!("required modifier was not set"),
                        },
                    }),
                    AstComponent::Minute(Minute { padding }) => Self::Minute(modifier::Minute {
                        padding: padding.unwrap_or_default().into(),
                    }),
                    AstComponent::Month(Month {
                        padding,
                        repr,
                        case_sensitive,
                    }) => match repr.unwrap_or_default() {
                        MonthRepr::Numerical => {
                            reject_modifier!(
                                case_sensitive,
                                "case_sensitive",
                                "for numerical month"
                            );
                            Self::MonthNumerical(modifier::MonthNumerical {
                                padding: padding.unwrap_or_default().into(),
                            })
                        },
                        MonthRepr::Long => {
                            reject_modifier!(padding, "padding", "for long month");
                            Self::MonthLong(modifier::MonthLong {
                                case_sensitive: case_sensitive.unwrap_or_default().into(),
                            })
                        },
                        MonthRepr::Short => {
                            reject_modifier!(padding, "padding", "for short month");
                            Self::MonthShort(modifier::MonthShort {
                                case_sensitive: case_sensitive.unwrap_or_default().into(),
                            })
                        },
                    },
                    AstComponent::OffsetHour(OffsetHour {
                        sign_behavior,
                        padding,
                    }) => Self::OffsetHour(modifier::OffsetHour {
                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                        padding: padding.unwrap_or_default().into(),
                    }),
                    AstComponent::OffsetMinute(OffsetMinute { padding }) => {
                        Self::OffsetMinute(modifier::OffsetMinute {
                            padding: padding.unwrap_or_default().into(),
                        })
                    }
                    AstComponent::OffsetSecond(OffsetSecond { padding }) => {
                        Self::OffsetSecond(modifier::OffsetSecond {
                            padding: padding.unwrap_or_default().into(),
                        })
                    }
                    AstComponent::Ordinal(Ordinal { padding }) => Self::Ordinal(modifier::Ordinal {
                        padding: padding.unwrap_or_default().into(),
                    }),
                    AstComponent::Period(Period {
                        case,
                        case_sensitive,
                    }) => Self::Period(modifier::Period {
                        is_uppercase: case.unwrap_or_default().into(),
                        case_sensitive: case_sensitive.unwrap_or_default().into(),
                    }),
                    AstComponent::Second(Second { padding }) => Self::Second(modifier::Second {
                        padding: padding.unwrap_or_default().into(),
                    }),
                    AstComponent::Subsecond(Subsecond { digits }) => {
                        Self::Subsecond(modifier::Subsecond {
                            digits: digits.unwrap_or_default().into(),
                        })
                    },
                    AstComponent::UnixTimestamp(UnixTimestamp {
                        precision,
                        sign_behavior,
                    }) => match precision.unwrap_or_default() {
                        UnixTimestampPrecision::Second => {
                            Self::UnixTimestampSecond(modifier::UnixTimestampSecond {
                                sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                            })
                        }
                        UnixTimestampPrecision::Millisecond => {
                            Self::UnixTimestampMillisecond(modifier::UnixTimestampMillisecond {
                                sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                            })
                        }
                        UnixTimestampPrecision::Microsecond => {
                            Self::UnixTimestampMicrosecond(modifier::UnixTimestampMicrosecond {
                                sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                            })
                        }
                        UnixTimestampPrecision::Nanosecond => {
                            Self::UnixTimestampNanosecond(modifier::UnixTimestampNanosecond {
                                sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                            })
                        }
                    },
                    AstComponent::Weekday(Weekday {
                        repr,
                        one_indexed,
                        case_sensitive,
                    }) => match repr.unwrap_or_default() {
                        WeekdayRepr::Short => {
                            reject_modifier!(one_indexed, "one_indexed", "for short weekday");
                            Self::WeekdayShort(modifier::WeekdayShort {
                                case_sensitive: case_sensitive.unwrap_or_default().into(),
                            })
                        },
                        WeekdayRepr::Long => {
                            reject_modifier!(one_indexed, "one_indexed", "for long weekday");
                            Self::WeekdayLong(modifier::WeekdayLong {
                                case_sensitive: case_sensitive.unwrap_or_default().into(),
                            })
                        },
                        WeekdayRepr::Sunday => {
                            reject_modifier!(
                                case_sensitive,
                                "case_sensitive",
                                "for numerical weekday"
                            );
                            Self::WeekdaySunday(modifier::WeekdaySunday {
                                one_indexed: one_indexed.unwrap_or_default().into(),
                            })
                        },
                        WeekdayRepr::Monday => {
                            reject_modifier!(
                                case_sensitive,
                                "case_sensitive",
                                "for numerical weekday"
                            );
                            Self::WeekdayMonday(modifier::WeekdayMonday {
                                one_indexed: one_indexed.unwrap_or_default().into(),
                            })
                        },
                    },
                    AstComponent::WeekNumber(WeekNumber { padding, repr }) => {
                        match repr.unwrap_or_default() {
                            WeekNumberRepr::Iso => {
                                Self::WeekNumberIso(modifier::WeekNumberIso {
                                    padding: padding.unwrap_or_default().into(),
                                })
                            },
                            WeekNumberRepr::Sunday => {
                                Self::WeekNumberSunday(modifier::WeekNumberSunday {
                                    padding: padding.unwrap_or_default().into(),
                                })
                            },
                            WeekNumberRepr::Monday => {
                                Self::WeekNumberMonday(modifier::WeekNumberMonday {
                                    padding: padding.unwrap_or_default().into(),
                                })
                            },
                        }
                    }
                    AstComponent::Year(Year {
                        padding,
                        repr,
                        range,
                        base,
                        sign_behavior,
                    }) => {
                        #[cfg(not(feature = "large-dates"))]
                        reject_modifier!(
                            range,
                            "range",
                            "when the `large-dates` feature is not enabled"
                        );

                        match (
                            base.unwrap_or_default(),
                            repr.unwrap_or_default(),
                            range.unwrap_or_default(),
                        ) {
                            #[cfg(feature = "large-dates")]
                            (YearBase::Calendar, YearRepr::Full, YearRange::Extended) => {
                                Self::CalendarYearFullExtendedRange(
                                    modifier::CalendarYearFullExtendedRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            (YearBase::Calendar, YearRepr::Full, _) => {
                                Self::CalendarYearFullStandardRange(
                                    modifier::CalendarYearFullStandardRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            #[cfg(feature = "large-dates")]
                            (YearBase::Calendar, YearRepr::Century, YearRange::Extended) => {
                                Self::CalendarYearCenturyExtendedRange(
                                    modifier::CalendarYearCenturyExtendedRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            (YearBase::Calendar, YearRepr::Century, _) => {
                                Self::CalendarYearCenturyStandardRange(
                                    modifier::CalendarYearCenturyStandardRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            #[cfg(feature = "large-dates")]
                            (YearBase::IsoWeek, YearRepr::Full, YearRange::Extended) => {
                                Self::IsoYearFullExtendedRange(modifier::IsoYearFullExtendedRange {
                                    padding: padding.unwrap_or_default().into(),
                                    sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                })
                            }
                            (YearBase::IsoWeek, YearRepr::Full, _) => {
                                Self::IsoYearFullStandardRange(modifier::IsoYearFullStandardRange {
                                    padding: padding.unwrap_or_default().into(),
                                    sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                })
                            }
                            #[cfg(feature = "large-dates")]
                            (YearBase::IsoWeek, YearRepr::Century, YearRange::Extended) => {
                                Self::IsoYearCenturyExtendedRange(
                                    modifier::IsoYearCenturyExtendedRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            (YearBase::IsoWeek, YearRepr::Century, _) => {
                                Self::IsoYearCenturyStandardRange(
                                    modifier::IsoYearCenturyStandardRange {
                                        padding: padding.unwrap_or_default().into(),
                                        sign_is_mandatory: sign_behavior.unwrap_or_default().into(),
                                    },
                                )
                            }
                            (YearBase::Calendar, YearRepr::LastTwo, _) => {
                                #[cfg(feature = "large-dates")]
                                reject_modifier!(range, "range", "when `repr:last_two` is used");
                                reject_modifier!(
                                    sign_behavior,
                                    "sign",
                                    "when `repr:last_two` is used"
                                );
                                Self::CalendarYearLastTwo(modifier::CalendarYearLastTwo {
                                    padding: padding.unwrap_or_default().into(),
                                })
                            }
                            (YearBase::IsoWeek, YearRepr::LastTwo, _) => {
                                #[cfg(feature = "large-dates")]
                                reject_modifier!(range, "range", "when `repr:last_two` is used");
                                reject_modifier!(
                                    sign_behavior,
                                    "sign",
                                    "when `repr:last_two` is used"
                                );
                                Self::IsoYearLastTwo(modifier::IsoYearLastTwo {
                                    padding: padding.unwrap_or_default().into(),
                                })
                            }
                        }
                    }
                })
            }
        })+
    }
}

impl_from_ast_component_for!(
    [false] crate::format_description::Component,
    [true] crate::format_description::__private::FormatDescriptionV3Inner<'_>,
);

/// Get the target type for a given enum.
macro_rules! target_ty {
    ($name:ident $type:ty) => {
        $type
    };
    ($name:ident) => {
        $crate::format_description::modifier::$name
    };
}

/// Get the target value for a given enum.
macro_rules! target_value {
    ($name:ident $variant:ident $value:expr) => {
        $value
    };
    ($name:ident $variant:ident) => {
        $crate::format_description::modifier::$name::$variant
    };
}

trait ModifierValue {
    type Type;
}

/// Declare the various modifiers.
///
/// For the general case, ordinary syntax can be used. Note that you _must_ declare a default
/// variant. The only significant change is that the string representation of the variant must be
/// provided after the variant name. For example, `Numerical = b"numerical"` declares a variant
/// named `Numerical` with the string representation `b"numerical"`. This is the value that will be
/// used when parsing the modifier. The value is not case sensitive.
///
/// If the type in the public API does not have the same name as the type in the internal
/// representation, then the former must be specified in parenthesis after the internal name. For
/// example, `HourBase(bool)` has an internal name "HourBase", but is represented as a boolean in
/// the public API.
///
/// By default, the internal variant name is assumed to be the same as the public variant name. If
/// this is not the case, the qualified path to the variant must be specified in parenthesis after
/// the internal variant name. For example, `Twelve(true)` has an internal variant name "Twelve",
/// but is represented as `true` in the public API.
macro_rules! modifier {
    ($(
        $(#[expect($expect_inner:meta)])?
        enum $name:ident $(($target_ty:ty))? {
            $(
                $(#[$attr:meta])?
                $variant:ident $(($target_value:expr))? = $parse_variant:literal
            ),* $(,)?
        }
    )+) => {$(
        #[derive(Default, Clone, Copy)]
        enum $name {
            $($(#[$attr])? $variant),*
        }

        impl $name {
            /// Parse the modifier from its string representation.
            #[inline]
            fn from_modifier_value(value: &Spanned<&str>) -> Result<Self, Error> {
                $(if value.eq_ignore_ascii_case($parse_variant) {
                    return Ok(Self::$variant);
                })*
                Err(Error {
                    _inner: unused(value.span.error("invalid modifier value")),
                    public: InvalidFormatDescription::InvalidModifier {
                        value: (**value).to_owned(),
                        index: value.span.start.byte as usize,
                    },
                })
            }
        }

        $(#[expect($expect_inner)])?
        impl ModifierValue for $name {
            type Type = target_ty!($name $($target_ty)?);
        }

        $(#[expect($expect_inner)])?
        impl From<$name> for <$name as ModifierValue>::Type {
            #[inline]
            fn from(modifier: $name) -> Self {
                match modifier {
                    $($name::$variant => target_value!($name $variant $($target_value)?)),*
                }
            }
        }
    )+};
}

// Keep in alphabetical order.
modifier! {
    enum HourBase(bool) {
        Twelve(true) = "12",
        #[default]
        TwentyFour(false) = "24",
    }

    enum MonthCaseSensitive(bool) {
        False(false) = "false",
        #[default]
        True(true) = "true",
    }

    #[expect(deprecated)]
    enum MonthRepr {
        #[default]
        Numerical = "numerical",
        Long = "long",
        Short = "short",
    }

    enum OptionalFormat(bool) {
        False(false) = "false",
        #[default]
        True(true) = "true",
    }

    enum Padding {
        Space = "space",
        #[default]
        Zero = "zero",
        None = "none",
    }

    enum PeriodCase(bool) {
        Lower(false) = "lower",
        #[default]
        Upper(true) = "upper",
    }

    enum PeriodCaseSensitive(bool) {
        False(false) = "false",
        #[default]
        True(true) = "true",
    }

    enum SignBehavior(bool) {
        #[default]
        Automatic(false) = "automatic",
        Mandatory(true) = "mandatory",
    }

    enum SubsecondDigits {
        One = "1",
        Two = "2",
        Three = "3",
        Four = "4",
        Five = "5",
        Six = "6",
        Seven = "7",
        Eight = "8",
        Nine = "9",
        #[default]
        OneOrMore = "1+",
    }

    enum TrailingInput {
        #[default]
        Prohibit = "prohibit",
        Discard = "discard",
    }

    #[expect(deprecated)]
    enum UnixTimestampPrecision {
        #[default]
        Second = "second",
        Millisecond = "millisecond",
        Microsecond = "microsecond",
        Nanosecond = "nanosecond",
    }

    #[expect(deprecated)]
    enum WeekNumberRepr {
        #[default]
        Iso = "iso",
        Sunday = "sunday",
        Monday = "monday",
    }

    enum WeekdayCaseSensitive(bool) {
        False(false) = "false",
        #[default]
        True(true) = "true",
    }

    enum WeekdayOneIndexed(bool) {
        False(false) = "false",
        #[default]
        True(true) = "true",
    }

    #[expect(deprecated)]
    enum WeekdayRepr {
        Short = "short",
        #[default]
        Long = "long",
        Sunday = "sunday",
        Monday = "monday",
    }

    enum YearBase(bool) {
        #[default]
        Calendar(false) = "calendar",
        IsoWeek(true) = "iso_week",
    }

    #[expect(deprecated)]
    enum YearRepr {
        #[default]
        Full = "full",
        Century = "century",
        LastTwo = "last_two",
    }

    // For v1 and v2 format descriptions, the default is `extended`. For v3 format descriptions,
    // the default is `standard`. For backwards compatibility, the default here needs to stay
    // `extended`.
    #[expect(deprecated)]
    enum YearRange {
        Standard = "standard",
        #[default]
        Extended = "extended",
    }
}

/// Parse a modifier value using `FromStr`. Requires the modifier value to be valid UTF-8.
#[inline]
fn parse_from_modifier_value<T>(value: &Spanned<&str>) -> Result<T, Error>
where
    T: FromStr,
{
    value.parse::<T>().map_err(|_| Error {
        _inner: unused(value.span.error("invalid modifier value")),
        public: InvalidFormatDescription::InvalidModifier {
            value: (**value).to_owned(),
            index: value.span.start.byte as usize,
        },
    })
}
