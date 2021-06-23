use core::mem;

use proc_macro::TokenStream;

use crate::format_description::error::InvalidFormatDescription;
use crate::format_description::helper;
use crate::to_tokens::ToTokens;

macro_rules! to_tokens {
    (
        $(#[$struct_attr:meta])*
        $struct_vis:vis struct $struct_name:ident {$(
            $(#[$field_attr:meta])*
            $field_vis:vis $field_name:ident : $field_ty:ty
        ),+ $(,)?}
    ) => {
        $(#[$struct_attr])*
        $struct_vis struct $struct_name {$(
            $(#[$field_attr])*
            $field_vis $field_name: $field_ty
        ),+}

        impl ToTokens for $struct_name {
            fn into_token_stream(self) -> TokenStream {
                quote! {
                    ::time::format_description::modifier::$struct_name {$(
                        $field_name: #(self.$field_name),
                    )+}
                }
            }
        }
    };

    (
        $(#[$enum_attr:meta])*
        $enum_vis:vis enum $enum_name:ident {$(
            $(#[$variant_attr:meta])*
            $variant_name:ident
        ),+ $(,)?}
    ) => {
        $(#[$enum_attr])*
        $enum_vis enum $enum_name {$(
            $(#[$variant_attr])*
            $variant_name
        ),+}

        impl ToTokens for $enum_name {
            fn into_token_stream(self) -> TokenStream {
                quote! {
                    ::time::format_description::modifier::$enum_name::#(match self {
                        $(Self::$variant_name => quote!($variant_name)),+
                    })
                }
            }
        }
    }
}

to_tokens! {
    pub(crate) struct Day {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) enum MonthRepr {
        Numerical,
        Long,
        Short,
    }
}

to_tokens! {
    pub(crate) struct Month {
        pub(crate) padding: Padding,
        pub(crate) repr: MonthRepr,
    }
}

to_tokens! {
    pub(crate) struct Ordinal {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) enum WeekdayRepr {
        Short,
        Long,
        Sunday,
        Monday,
    }
}

to_tokens! {
    pub(crate) struct Weekday {
        pub(crate) repr: WeekdayRepr,
        pub(crate) one_indexed: bool,
    }
}

to_tokens! {
    pub(crate) enum WeekNumberRepr {
        Iso,
        Sunday,
        Monday,
    }
}

to_tokens! {
    pub(crate) struct WeekNumber {
        pub(crate) padding: Padding,
        pub(crate) repr: WeekNumberRepr,
    }
}

to_tokens! {
    pub(crate) enum YearRepr {
        Full,
        LastTwo,
    }
}

to_tokens! {
    pub(crate) struct Year {
        pub(crate) padding: Padding,
        pub(crate) repr: YearRepr,
        pub(crate) iso_week_based: bool,
        pub(crate) sign_is_mandatory: bool,
    }
}

to_tokens! {
    pub(crate) struct Hour {
        pub(crate) padding: Padding,
        pub(crate) is_12_hour_clock: bool,
    }
}

to_tokens! {
    pub(crate) struct Minute {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) struct Period {
        pub(crate) is_uppercase: bool,
    }
}

to_tokens! {
    pub(crate) struct Second {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) enum SubsecondDigits {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        OneOrMore,
    }
}

to_tokens! {
    pub(crate) struct Subsecond {
        pub(crate) digits: SubsecondDigits,
    }
}

to_tokens! {
    pub(crate) struct OffsetHour {
        pub(crate) sign_is_mandatory: bool,
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) struct OffsetMinute {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) struct OffsetSecond {
        pub(crate) padding: Padding,
    }
}

to_tokens! {
    pub(crate) enum Padding {
        Space,
        Zero,
        None,
    }
}

macro_rules! impl_default {
    ($($type:ty => $default:expr;)*) => {$(
        impl Default for $type {
            fn default() -> Self {
                $default
            }
        }
    )*};
}

impl_default! {
    Padding => Self::Zero;
    MonthRepr => Self::Numerical;
    SubsecondDigits => Self::OneOrMore;
    WeekdayRepr => Self::Long;
    WeekNumberRepr => Self::Iso;
    YearRepr => Self::Full;
}

#[derive(Default)]
pub(crate) struct Modifiers {
    pub(crate) padding: Option<Padding>,
    pub(crate) hour_is_12_hour_clock: Option<bool>,
    pub(crate) period_is_uppercase: Option<bool>,
    pub(crate) month_repr: Option<MonthRepr>,
    pub(crate) subsecond_digits: Option<SubsecondDigits>,
    pub(crate) weekday_repr: Option<WeekdayRepr>,
    pub(crate) weekday_is_one_indexed: Option<bool>,
    pub(crate) week_number_repr: Option<WeekNumberRepr>,
    pub(crate) year_repr: Option<YearRepr>,
    pub(crate) year_is_iso_week_based: Option<bool>,
    pub(crate) sign_is_mandatory: Option<bool>,
}

impl Modifiers {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse(
        component_name: &str,
        mut s: &str,
        index: &mut usize,
    ) -> Result<Self, InvalidFormatDescription> {
        let mut modifiers = Self::default();

        while !s.is_empty() {
            s = helper::consume_whitespace(s, index);

            let modifier;
            if let Some(whitespace_loc) = s.find(char::is_whitespace) {
                *index += whitespace_loc;
                modifier = &s[..whitespace_loc];
                s = &s[whitespace_loc..];
            } else {
                modifier = mem::take(&mut s);
            }

            if modifier.is_empty() {
                break;
            }

            #[allow(clippy::unnested_or_patterns)]
            match (component_name, modifier) {
                ("day", "padding:space")
                | ("hour", "padding:space")
                | ("minute", "padding:space")
                | ("month", "padding:space")
                | ("offset_hour", "padding:space")
                | ("offset_minute", "padding:space")
                | ("offset_second", "padding:space")
                | ("ordinal", "padding:space")
                | ("second", "padding:space")
                | ("week_number", "padding:space")
                | ("year", "padding:space") => modifiers.padding = Some(Padding::Space),
                ("day", "padding:zero")
                | ("hour", "padding:zero")
                | ("minute", "padding:zero")
                | ("month", "padding:zero")
                | ("offset_hour", "padding:zero")
                | ("offset_minute", "padding:zero")
                | ("offset_second", "padding:zero")
                | ("ordinal", "padding:zero")
                | ("second", "padding:zero")
                | ("week_number", "padding:zero")
                | ("year", "padding:zero") => modifiers.padding = Some(Padding::Zero),
                ("day", "padding:none")
                | ("hour", "padding:none")
                | ("minute", "padding:none")
                | ("month", "padding:none")
                | ("offset_hour", "padding:none")
                | ("offset_minute", "padding:none")
                | ("offset_second", "padding:none")
                | ("ordinal", "padding:none")
                | ("second", "padding:none")
                | ("week_number", "padding:none")
                | ("year", "padding:none") => modifiers.padding = Some(Padding::None),
                ("hour", "repr:24") => modifiers.hour_is_12_hour_clock = Some(false),
                ("hour", "repr:12") => modifiers.hour_is_12_hour_clock = Some(true),
                ("month", "repr:numerical") => modifiers.month_repr = Some(MonthRepr::Numerical),
                ("month", "repr:long") => modifiers.month_repr = Some(MonthRepr::Long),
                ("month", "repr:short") => modifiers.month_repr = Some(MonthRepr::Short),
                ("offset_hour", "sign:automatic") | ("year", "sign:automatic") => {
                    modifiers.sign_is_mandatory = Some(false)
                }
                ("offset_hour", "sign:mandatory") | ("year", "sign:mandatory") => {
                    modifiers.sign_is_mandatory = Some(true)
                }
                ("period", "case:upper") => modifiers.period_is_uppercase = Some(true),
                ("period", "case:lower") => modifiers.period_is_uppercase = Some(false),
                ("subsecond", "digits:1") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::One)
                }
                ("subsecond", "digits:2") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Two)
                }
                ("subsecond", "digits:3") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Three)
                }
                ("subsecond", "digits:4") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Four)
                }
                ("subsecond", "digits:5") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Five)
                }
                ("subsecond", "digits:6") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Six)
                }
                ("subsecond", "digits:7") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Seven)
                }
                ("subsecond", "digits:8") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Eight)
                }
                ("subsecond", "digits:9") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Nine)
                }
                ("subsecond", "digits:1+") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::OneOrMore)
                }
                ("weekday", "repr:short") => modifiers.weekday_repr = Some(WeekdayRepr::Short),
                ("weekday", "repr:long") => modifiers.weekday_repr = Some(WeekdayRepr::Long),
                ("weekday", "repr:sunday") => modifiers.weekday_repr = Some(WeekdayRepr::Sunday),
                ("weekday", "repr:monday") => modifiers.weekday_repr = Some(WeekdayRepr::Monday),
                ("weekday", "one_indexed:true") => modifiers.weekday_is_one_indexed = Some(true),
                ("weekday", "one_indexed:false") => modifiers.weekday_is_one_indexed = Some(false),
                ("week_number", "repr:iso") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Iso)
                }
                ("week_number", "repr:sunday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Sunday)
                }
                ("week_number", "repr:monday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Monday)
                }
                ("year", "repr:full") => modifiers.year_repr = Some(YearRepr::Full),
                ("year", "repr:last_two") => modifiers.year_repr = Some(YearRepr::LastTwo),
                ("year", "base:calendar") => modifiers.year_is_iso_week_based = Some(false),
                ("year", "base:iso_week") => modifiers.year_is_iso_week_based = Some(true),
                _ => {
                    return Err(InvalidFormatDescription::InvalidModifier {
                        value: modifier.to_owned(),
                        index: *index,
                    });
                }
            }
        }

        Ok(modifiers)
    }
}
