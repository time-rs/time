use alloc::string::String;

use super::{ast, Error};

pub(super) fn parse<'a>(
    ast_items: impl Iterator<Item = Result<ast::Item<'a>, Error>>,
) -> impl Iterator<Item = Result<Item<'a>, Error>> {
    ast_items.map(|ast_item| ast_item.and_then(Item::from_ast))
}

#[allow(variant_size_differences)]
pub(super) enum Item<'a> {
    Literal(&'a [u8]),
    Component(Component),
}

impl Item<'_> {
    pub(super) fn from_ast(ast_item: ast::Item<'_>) -> Result<Item<'_>, Error> {
        Ok(match ast_item {
            ast::Item::Component {
                _opening_bracket: _,
                _leading_whitespace: _,
                name,
                modifiers,
                _trailing_whitespace: _,
                _closing_bracket: _,
            } => Item::Component(component_from_ast(&name, &modifiers)?),
            ast::Item::Literal { value, _span: _ } => Item::Literal(value),
            ast::Item::EscapedBracket {
                _first: _,
                _second: _,
            } => Item::Literal(b"["),
        })
    }
}

impl<'a> From<Item<'a>> for crate::format_description::FormatItem<'a> {
    fn from(item: Item<'a>) -> Self {
        match item {
            Item::Literal(literal) => Self::Literal(literal),
            Item::Component(component) => Self::Component(component.into()),
        }
    }
}

macro_rules! component_definition {
    ($vis:vis enum $name:ident {
        $($variant:ident = $parse_variant:literal {
            $($field:ident = $parse_field:literal:
                Option<$field_type:ty> => $target_field:ident),* $(,)?
        }),* $(,)?
    }) => {
        $vis enum $name {
            $($variant($variant),)*
        }

        $($vis struct $variant {
            $($field: Option<$field_type>),*
        })*

        $(impl $variant {
            fn with_modifiers(modifiers: &[ast::Modifier<'_>]) -> Result<Self, Error> {
                let mut this = Self {
                    $($field: None),*
                };

                for modifier in modifiers {
                    $(if modifier.key.value.eq_ignore_ascii_case($parse_field) {
                        this.$field = <$field_type>::from_modifier_value(&modifier.value)?;
                        continue;
                    })*
                    return Err(Error {
                        _inner: modifier.key.span.error("invalid modifier key"),
                        public: crate::error::InvalidFormatDescription::InvalidModifier {
                            value: String::from_utf8_lossy(modifier.key.value).into_owned(),
                            index: modifier.key.span.start_byte(),
                        }
                    });
                }

                Ok(this)
            }
        })*

        impl From<$name> for crate::format_description::Component {
            fn from(component: $name) -> Self {
                match component {$(
                    $name::$variant($variant { $($field),* }) => {
                        $crate::format_description::component::Component::$variant(
                            $crate::format_description::modifier::$variant {$(
                                $target_field: $field.unwrap_or_default().into()
                            ),*}
                        )
                    }
                )*}
            }
        }

        fn component_from_ast(
            name: &ast::Name<'_>,
            modifiers: &[ast::Modifier<'_>],
        ) -> Result<Component, Error> {
            $(if name.value.eq_ignore_ascii_case($parse_variant) {
                return Ok(Component::$variant($variant::with_modifiers(&modifiers)?));
            })*
            Err(Error {
                _inner: name.span.error("invalid component"),
                public: crate::error::InvalidFormatDescription::InvalidComponentName {
                    name: String::from_utf8_lossy(name.value).into_owned(),
                    index: name.span.start_byte(),
                },
            })
        }
    }
}

// Keep in alphabetical order.
component_definition! {
    pub(super) enum Component {
        Day = b"day" {
            padding = b"padding": Option<Padding> => padding,
        },
        Hour = b"hour" {
            padding = b"padding": Option<Padding> => padding,
            base = b"repr": Option<HourBase> => is_12_hour_clock,
        },
        Minute = b"minute" {
            padding = b"padding": Option<Padding> => padding,
        },
        Month = b"month" {
            padding = b"padding": Option<Padding> => padding,
            repr = b"repr": Option<MonthRepr> => repr,
            case_sensitive = b"case_sensitive": Option<MonthCaseSensitive> => case_sensitive,
        },
        OffsetHour = b"offset_hour" {
            sign_behavior = b"sign": Option<SignBehavior> => sign_is_mandatory,
            padding = b"padding": Option<Padding> => padding,
        },
        OffsetMinute = b"offset_minute" {
            padding = b"padding": Option<Padding> => padding,
        },
        OffsetSecond = b"offset_second" {
            padding = b"padding": Option<Padding> => padding,
        },
        Ordinal = b"ordinal" {
            padding = b"padding": Option<Padding> => padding,
        },
        Period = b"period" {
            case = b"case": Option<PeriodCase> => is_uppercase,
            case_sensitive = b"case_sensitive": Option<PeriodCaseSensitive> => case_sensitive,
        },
        Second = b"second" {
            padding = b"padding": Option<Padding> => padding,
        },
        Subsecond = b"subsecond" {
            digits = b"digits": Option<SubsecondDigits> => digits,
        },
        Weekday = b"weekday" {
            repr = b"repr": Option<WeekdayRepr> => repr,
            one_indexed = b"one_indexed": Option<WeekdayOneIndexed> => one_indexed,
            case_sensitive = b"case_sensitive": Option<WeekdayCaseSensitive> => case_sensitive,
        },
        WeekNumber = b"week_number" {
            padding = b"padding": Option<Padding> => padding,
            repr = b"repr": Option<WeekNumberRepr> => repr,
        },
        Year = b"year" {
            padding = b"padding": Option<Padding> => padding,
            repr = b"repr": Option<YearRepr> => repr,
            base = b"base": Option<YearBase> => iso_week_based,
            sign_behavior = b"sign": Option<SignBehavior> => sign_is_mandatory,
        },
    }
}

macro_rules! target_ty {
    ($name:ident $type:ty) => {
        $type
    };
    ($name:ident) => {
        $crate::format_description::modifier::$name
    };
}

macro_rules! target_value {
    ($name:ident $variant:ident $value:expr) => {
        $value
    };
    ($name:ident $variant:ident) => {
        $crate::format_description::modifier::$name::$variant
    };
}

// TODO use `#[derive(Default)]` on enums once MSRV is 1.62 (NET 2022-12-30)
macro_rules! derived_default_on_enum {
    ($type:ty; $default:expr) => {};
    ($attr:meta $type:ty; $default:expr) => {
        impl Default for $type {
            fn default() -> Self {
                $default
            }
        }
    };
}

macro_rules! modifier {
    ($(
        enum $name:ident $(($target_ty:ty))? {
            $(
                $(#[$attr:meta])?
                $variant:ident $(($target_value:expr))? = $parse_variant:literal
            ),* $(,)?
        }
    )+) => {$(
        enum $name {
            $($variant),*
        }

        $(derived_default_on_enum! {
            $($attr)? $name; $name::$variant
        })*

        impl $name {
            fn from_modifier_value(value: &ast::Value<'_>) -> Result<Option<Self>, Error> {
                $(if value.value.eq_ignore_ascii_case($parse_variant) {
                    return Ok(Some(Self::$variant));
                })*
                Err(Error {
                    _inner: value.span.error("invalid modifier value"),
                    public: crate::error::InvalidFormatDescription::InvalidModifier {
                        value: String::from_utf8_lossy(value.value).into_owned(),
                        index: value.span.start_byte(),
                    },
                })
            }
        }

        impl From<$name> for target_ty!($name $($target_ty)?) {
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
        Twelve(true) = b"12",
        #[default]
        TwentyFour(false) = b"24",
    }

    enum MonthCaseSensitive(bool) {
        False(false) = b"false",
        #[default]
        True(true) = b"true",
    }

    enum MonthRepr {
        #[default]
        Numerical = b"numerical",
        Long = b"long",
        Short = b"short",
    }

    enum Padding {
        Space = b"space",
        #[default]
        Zero = b"zero",
        None = b"none",
    }

    enum PeriodCase(bool) {
        Lower(false) = b"lower",
        #[default]
        Upper(true) = b"upper",
    }

    enum PeriodCaseSensitive(bool) {
        False(false) = b"false",
        #[default]
        True(true) = b"true",
    }

    enum SignBehavior(bool) {
        #[default]
        Automatic(false) = b"automatic",
        Mandatory(true) = b"mandatory",
    }

    enum SubsecondDigits {
        One = b"1",
        Two = b"2",
        Three = b"3",
        Four = b"4",
        Five = b"5",
        Six = b"6",
        Seven = b"7",
        Eight = b"8",
        Nine = b"9",
        #[default]
        OneOrMore = b"1+",
    }

    enum WeekNumberRepr {
        #[default]
        Iso = b"iso",
        Sunday = b"sunday",
        Monday = b"monday",
    }

    enum WeekdayCaseSensitive(bool) {
        False(false) = b"false",
        #[default]
        True(true) = b"true",
    }

    enum WeekdayOneIndexed(bool) {
        False(false) = b"false",
        #[default]
        True(true) = b"true",
    }

    enum WeekdayRepr {
        Short = b"short",
        #[default]
        Long = b"long",
        Sunday = b"sunday",
        Monday = b"monday",
    }

    enum YearBase(bool) {
        #[default]
        Calendar(false) = b"calendar",
        IsoWeek(true) = b"iso_week",
    }

    enum YearRepr {
        #[default]
        Full = b"full",
        LastTwo = b"last_two",
    }
}
