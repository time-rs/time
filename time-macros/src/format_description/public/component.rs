use proc_macro::{Ident, Span, TokenStream};

use super::modifier;
use crate::to_tokens::ToTokenStream;

macro_rules! declare_component {
    ($($(#[cfg($cfg_inner:meta)])* $name:ident)*) => {
        #[derive(Clone, Copy)]
        pub(crate) enum Component {$(
            $(#[cfg($cfg_inner)])*
            $name(modifier::$name),
        )*}

        impl ToTokenStream for Component {
            fn into_token_stream(self) -> TokenStream {
                self.into_token_stream_with_enum_name("Component")
            }
        }

        impl Component {
            pub(crate) fn into_token_stream_with_enum_name(self, enum_name: &str) -> TokenStream {
                let (component, modifier) = match self {$(
                    $(#[cfg($cfg_inner)])*
                    Self::$name(modifier) => {
                        (stringify!($name), modifier.into_token_stream())
                    }
                )*};
                let enum_name = Ident::new(enum_name, Span::mixed_site());
                let component = Ident::new(component, Span::mixed_site());

                quote! {
                    #(enum_name)::#(component)(#S(modifier))
                }
            }
        }
    };
}

declare_component! {
    Day
    MonthShort
    MonthLong
    MonthNumerical
    Ordinal
    WeekdayShort
    WeekdayLong
    WeekdaySunday
    WeekdayMonday
    WeekNumberIso
    WeekNumberSunday
    WeekNumberMonday
    #[cfg(feature = "large-dates")]
    CalendarYearFullExtendedRange
    CalendarYearFullStandardRange
    #[cfg(feature = "large-dates")]
    IsoYearFullExtendedRange
    IsoYearFullStandardRange
    #[cfg(feature = "large-dates")]
    CalendarYearCenturyExtendedRange
    CalendarYearCenturyStandardRange
    #[cfg(feature = "large-dates")]
    IsoYearCenturyExtendedRange
    IsoYearCenturyStandardRange
    CalendarYearLastTwo
    IsoYearLastTwo
    Hour12
    Hour24
    Minute
    Period
    Second
    Subsecond
    OffsetHour
    OffsetMinute
    OffsetSecond
    Ignore
    UnixTimestampSecond
    UnixTimestampMillisecond
    UnixTimestampMicrosecond
    UnixTimestampNanosecond
    End
}
