//! Formatting for various types.

mod component_provider;
pub(crate) mod formattable;
mod iso8601;
mod metadata;

use core::mem::MaybeUninit;
use core::num::NonZero;
use std::io;

use deranged::{OptionRangedI32, OptionRangedU8, RangedI32, RangedU8, RangedU16, RangedU32};
use num_conv::prelude::*;

use self::component_provider::ComponentProvider;
pub use self::formattable::Formattable;
use crate::format_description::{Component, Period, modifier};
use crate::internal_macros::try_likely_ok;
use crate::time::{Hours, Minutes, Nanoseconds, Seconds};
use crate::utc_offset::{Hours as OffsetHours, Minutes as OffsetMinutes, Seconds as OffsetSeconds};
use crate::{Month, Weekday, error, num_fmt};

type Day = RangedU8<1, 31>;
type OptionDay = OptionRangedU8<1, 31>;
type Ordinal = RangedU16<1, 366>;
type IsoWeekNumber = RangedU8<1, 53>;
type OptionIsoWeekNumber = OptionRangedU8<1, 53>;
type MondayBasedWeek = RangedU8<0, 53>;
type SundayBasedWeek = RangedU8<0, 53>;
type Year = RangedI32<-999_999, 999_999>;
type OptionYear = OptionRangedI32<-999_999, 999_999>;
type AnyWeekNumber = RangedU8<0, 53>;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_NAMES: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Write all bytes to the output, returning the number of bytes written.
#[inline]
pub(crate) fn write_bytes(
    output: &mut (impl io::Write + ?Sized),
    bytes: &[u8],
) -> io::Result<usize> {
    try_likely_ok!(output.write_all(bytes));
    Ok(bytes.len())
}

/// Write the string to the output, returning the number of bytes written.
#[inline]
pub(crate) fn write(output: &mut (impl io::Write + ?Sized), s: &str) -> io::Result<usize> {
    try_likely_ok!(output.write_all(s.as_bytes()));
    Ok(s.len())
}

/// Write all strings to the output (in order), returning the total number of bytes written.
#[inline]
pub(crate) fn write_many<const N: usize>(
    output: &mut (impl io::Write + ?Sized),
    arr: [&str; N],
) -> io::Result<usize> {
    let mut bytes = 0;
    for s in arr {
        try_likely_ok!(output.write_all(s.as_bytes()));
        bytes += s.len();
    }
    Ok(bytes)
}

/// If `pred` is true, write the string to the output, returning the number of bytes written.
#[inline]
pub(crate) fn write_if(
    output: &mut (impl io::Write + ?Sized),
    pred: bool,
    s: &str,
) -> io::Result<usize> {
    if pred { write(output, s) } else { Ok(0) }
}

/// If `pred` is true, write `true_str` to the output. Otherwise, write `false_str`.
#[inline]
pub(crate) fn write_if_else(
    output: &mut (impl io::Write + ?Sized),
    pred: bool,
    true_str: &str,
    false_str: &str,
) -> io::Result<usize> {
    write(output, if pred { true_str } else { false_str })
}

/// Helper function to obtain 10^x, guaranteeing determinism for x ≤ 9. For these cases, the
/// function optimizes to a lookup table. For x ≥ 10, it falls back to `10_f64.powi(x)`. The only
/// situation where this would occur is if the user explicitly requests such precision when
/// configuring the ISO 8601 well known format. All other possibilities max out at nine digits.
#[inline]
fn f64_10_pow_x(x: NonZero<u8>) -> f64 {
    match x.get() {
        1 => 10.,
        2 => 100.,
        3 => 1_000.,
        4 => 10_000.,
        5 => 100_000.,
        6 => 1_000_000.,
        7 => 10_000_000.,
        8 => 100_000_000.,
        9 => 1_000_000_000.,
        x => 10_f64.powi(x.cast_signed().extend()),
    }
}

/// Write the floating point number to the output, returning the number of bytes written.
///
/// This method accepts the number of digits before and after the decimal. The value will be padded
/// with zeroes to the left if necessary.
#[inline]
pub(crate) fn format_float(
    output: &mut (impl io::Write + ?Sized),
    mut value: f64,
    digits_before_decimal: u8,
    digits_after_decimal: Option<NonZero<u8>>,
) -> io::Result<usize> {
    match digits_after_decimal {
        Some(digits_after_decimal) => {
            // If the precision is less than nine digits after the decimal point, truncate the
            // value. This avoids rounding up and causing the value to exceed the maximum permitted
            // value (as in #678). If the precision is at least nine, then we don't truncate so as
            // to avoid having an off-by-one error (as in #724). The latter is necessary
            // because floating point values are inherently imprecise with decimal
            // values, so a minuscule error can be amplified easily.
            //
            // Note that this is largely an issue for second values, as for minute and hour decimals
            // the value is divided by 60 or 3,600, neither of which divide evenly into 10^x.
            //
            // While not a perfect approach, this addresses the bugs that have been reported so far
            // without being overly complex.
            if digits_after_decimal.get() < 9 {
                let trunc_num = f64_10_pow_x(digits_after_decimal);
                value = f64::trunc(value * trunc_num) / trunc_num;
            }

            let digits_after_decimal = digits_after_decimal.get().extend();
            let width = digits_before_decimal.extend::<usize>() + 1 + digits_after_decimal;
            try_likely_ok!(write!(output, "{value:0>width$.digits_after_decimal$}"));
            Ok(width)
        }
        None => {
            let value = value.trunc() as u64;
            let width = digits_before_decimal.extend();
            try_likely_ok!(write!(output, "{value:0>width$}"));
            Ok(width)
        }
    }
}

/// Format a single digit.
#[inline]
pub(crate) fn format_single_digit(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU8<0, 9>,
) -> io::Result<usize> {
    write(output, num_fmt::single_digit(value))
}

/// Format a two digit number with the specified padding.
#[inline]
pub(crate) fn format_two_digits(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU8<0, 99>,
    padding: modifier::Padding,
) -> io::Result<usize> {
    let s = match padding {
        modifier::Padding::Space => num_fmt::two_digits_space_padded(value),
        modifier::Padding::Zero => num_fmt::two_digits_zero_padded(value),
        modifier::Padding::None => num_fmt::one_to_two_digits_no_padding(value),
    };
    write(output, s)
}

/// Format a three digit number with the specified padding.
#[inline]
pub(crate) fn format_three_digits(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU16<0, 999>,
    padding: modifier::Padding,
) -> io::Result<usize> {
    let [first, second_and_third] = match padding {
        modifier::Padding::Space => num_fmt::three_digits_space_padded(value),
        modifier::Padding::Zero => num_fmt::three_digits_zero_padded(value),
        modifier::Padding::None => num_fmt::one_to_three_digits_no_padding(value),
    };
    write_many(output, [first, second_and_third])
}

/// Format a four digit number with the specified padding.
#[inline]
pub(crate) fn format_four_digits(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU16<0, 9_999>,
    padding: modifier::Padding,
) -> io::Result<usize> {
    let [first_and_second, third_and_fourth] = match padding {
        modifier::Padding::Space => num_fmt::four_digits_space_padded(value),
        modifier::Padding::Zero => num_fmt::four_digits_zero_padded(value),
        modifier::Padding::None => num_fmt::one_to_four_digits_no_padding(value),
    };
    write_many(output, [first_and_second, third_and_fourth])
}

/// Format a four digit number that is padded with zeroes.
#[inline]
pub(crate) fn format_four_digits_pad_zero(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU16<0, 9_999>,
) -> io::Result<usize> {
    write_many(output, num_fmt::four_digits_zero_padded(value))
}

/// Format a five digit number that is padded with zeroes.
#[inline]
pub(crate) fn format_five_digits_pad_zero(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU32<0, 99_999>,
) -> io::Result<usize> {
    write_many(output, num_fmt::five_digits_zero_padded(value))
}

/// Format a six digit number that is padded with zeroes.
#[inline]
pub(crate) fn format_six_digits_pad_zero(
    output: &mut (impl io::Write + ?Sized),
    value: RangedU32<0, 999_999>,
) -> io::Result<usize> {
    write_many(output, num_fmt::six_digits_zero_padded(value))
}

/// Format a number with no padding.
///
/// If the sign is mandatory, the sign must be written by the caller.
#[inline]
pub(crate) fn format_number_pad_none(
    output: &mut (impl io::Write + ?Sized),
    value: impl itoa::Integer + Copy,
) -> Result<usize, io::Error> {
    write(output, itoa::Buffer::new().format(value))
}

/// Format the provided component into the designated output. An `Err` will be returned if the
/// component requires information that it does not provide or if the value cannot be output to the
/// stream.
#[inline]
pub(crate) fn format_component<V>(
    output: &mut (impl io::Write + ?Sized),
    component: Component,
    value: &V,
    state: &mut V::State,
) -> Result<usize, error::Format>
where
    V: ComponentProvider,
{
    use Component::*;
    match component {
        Day(modifier) if V::SUPPLIES_DATE => fmt_day(output, value.day(state), modifier),
        Month(modifier) if V::SUPPLIES_DATE => fmt_month(output, value.month(state), modifier),
        Ordinal(modifier) if V::SUPPLIES_DATE => {
            fmt_ordinal(output, value.ordinal(state), modifier)
        }
        Weekday(modifier) if V::SUPPLIES_DATE => {
            fmt_weekday(output, value.weekday(state), modifier)
        }
        WeekNumber(modifier) if V::SUPPLIES_DATE => fmt_week_number(
            output,
            match modifier.repr {
                modifier::WeekNumberRepr::Iso => value.iso_week_number(state).expand(),
                modifier::WeekNumberRepr::Sunday => value.sunday_based_week(state).expand(),
                modifier::WeekNumberRepr::Monday => value.monday_based_week(state).expand(),
            },
            modifier,
        ),
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::Full,
                iso_week_based: false,
                ..
            },
        ) if V::SUPPLIES_DATE => {
            return fmt_full_year(output, value.calendar_year(state), modifier);
        }
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::Century,
                iso_week_based: false,
                ..
            },
        ) if V::SUPPLIES_DATE => return fmt_century(output, value.calendar_year(state), modifier),
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: false,
                ..
            },
        ) if V::SUPPLIES_DATE => fmt_year_last_two(output, value.calendar_year(state), modifier),
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::Full,
                iso_week_based: true,
                ..
            },
        ) if V::SUPPLIES_DATE => {
            return fmt_full_year(output, value.iso_year(state), modifier);
        }
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::Century,
                iso_week_based: true,
                ..
            },
        ) if V::SUPPLIES_DATE => return fmt_century(output, value.iso_year(state), modifier),
        Year(
            modifier @ modifier::Year {
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: true,
                ..
            },
        ) if V::SUPPLIES_DATE => fmt_year_last_two(output, value.iso_year(state), modifier),
        Hour(modifier) if V::SUPPLIES_TIME => fmt_hour(output, value.hour(state), modifier),
        Minute(modifier) if V::SUPPLIES_TIME => fmt_minute(output, value.minute(state), modifier),
        Period(modifier) if V::SUPPLIES_TIME => fmt_period(output, value.period(state), modifier),
        Second(modifier) if V::SUPPLIES_TIME => fmt_second(output, value.second(state), modifier),
        Subsecond(modifier) if V::SUPPLIES_TIME => {
            fmt_subsecond(output, value.nanosecond(state), modifier)
        }
        OffsetHour(modifier) if V::SUPPLIES_OFFSET => fmt_offset_hour(
            output,
            value.offset_is_negative(state),
            value.offset_hour(state),
            modifier,
        ),
        OffsetMinute(modifier) if V::SUPPLIES_OFFSET => {
            fmt_offset_minute(output, value.offset_minute(state), modifier)
        }
        OffsetSecond(modifier) if V::SUPPLIES_OFFSET => {
            fmt_offset_second(output, value.offset_second(state), modifier)
        }
        Ignore(_) => return Ok(0),
        UnixTimestamp(modifier) if V::SUPPLIES_TIMESTAMP => match modifier.precision {
            modifier::UnixTimestampPrecision::Second => {
                fmt_unix_timestamp_seconds(output, value.unix_timestamp_seconds(state), modifier)
            }
            modifier::UnixTimestampPrecision::Millisecond => fmt_unix_timestamp_milliseconds(
                output,
                value.unix_timestamp_milliseconds(state),
                modifier,
            ),
            modifier::UnixTimestampPrecision::Microsecond => fmt_unix_timestamp_microseconds(
                output,
                value.unix_timestamp_microseconds(state),
                modifier,
            ),
            modifier::UnixTimestampPrecision::Nanosecond => fmt_unix_timestamp_nanoseconds(
                output,
                value.unix_timestamp_nanoseconds(state),
                modifier,
            ),
        },
        End(modifier::End { trailing_input: _ }) => return Ok(0),

        // This is functionally the same as a wildcard arm, but it will cause an error if a new
        // component is added. This is to avoid a bug where a new component, the code compiles, and
        // formatting fails.
        // Allow unreachable patterns because some branches may be fully matched above.
        #[allow(unreachable_patterns)]
        Day(_) | Month(_) | Ordinal(_) | Weekday(_) | WeekNumber(_) | Year(_) | Hour(_)
        | Minute(_) | Period(_) | Second(_) | Subsecond(_) | OffsetHour(_) | OffsetMinute(_)
        | OffsetSecond(_) | Ignore(_) | UnixTimestamp(_) | End(_) => {
            return Err(error::Format::InsufficientTypeInformation);
        }
    }
    .map_err(Into::into)
}

/// Format the day into the designated output.
#[inline]
fn fmt_day(
    output: &mut (impl io::Write + ?Sized),
    day: Day,
    modifier::Day { padding }: modifier::Day,
) -> Result<usize, io::Error> {
    format_two_digits(output, day.expand(), padding)
}

/// Format the month into the designated output.
#[inline]
fn fmt_month(
    output: &mut (impl io::Write + ?Sized),
    month: Month,
    modifier::Month {
        padding,
        repr,
        case_sensitive: _, // no effect on formatting
    }: modifier::Month,
) -> Result<usize, io::Error> {
    match repr {
        modifier::MonthRepr::Numerical => format_two_digits(
            output,
            // Safety: The month is guaranteed to be in the range `1..=12`.
            unsafe { RangedU8::new_unchecked(u8::from(month)) },
            padding,
        ),
        modifier::MonthRepr::Long => {
            write(output, MONTH_NAMES[u8::from(month).extend::<usize>() - 1])
        }
        // Safety: All month names are at least three bytes long.
        modifier::MonthRepr::Short => write(output, unsafe {
            MONTH_NAMES[u8::from(month).extend::<usize>() - 1].get_unchecked(..3)
        }),
    }
}

/// Format the ordinal into the designated output.
#[inline]
fn fmt_ordinal(
    output: &mut (impl io::Write + ?Sized),
    ordinal: Ordinal,
    modifier::Ordinal { padding }: modifier::Ordinal,
) -> Result<usize, io::Error> {
    format_three_digits(output, ordinal.expand(), padding)
}

/// Format the weekday into the designated output.
#[inline]
fn fmt_weekday(
    output: &mut (impl io::Write + ?Sized),
    weekday: Weekday,
    modifier::Weekday {
        repr,
        one_indexed,
        case_sensitive: _, // no effect on formatting
    }: modifier::Weekday,
) -> Result<usize, io::Error> {
    match repr {
        // Safety: All weekday names are at least three bytes long.
        modifier::WeekdayRepr::Short => write(output, unsafe {
            WEEKDAY_NAMES[weekday.number_days_from_monday().extend::<usize>()].get_unchecked(..3)
        }),
        modifier::WeekdayRepr::Long => write(
            output,
            WEEKDAY_NAMES[weekday.number_days_from_monday().extend::<usize>()],
        ),
        // Safety: The value is guaranteed to be in the range `1..=7`.
        modifier::WeekdayRepr::Sunday => format_single_digit(output, unsafe {
            RangedU8::new_unchecked(weekday.number_days_from_sunday() + u8::from(one_indexed))
        }),
        // Safety: The value is guaranteed to be in the range `1..=7`.
        modifier::WeekdayRepr::Monday => format_single_digit(output, unsafe {
            RangedU8::new_unchecked(weekday.number_days_from_monday() + u8::from(one_indexed))
        }),
    }
}

/// Format the week number into the designated output.
#[inline]
fn fmt_week_number(
    output: &mut (impl io::Write + ?Sized),
    week_number: AnyWeekNumber,
    modifier::WeekNumber { padding, repr: _ }: modifier::WeekNumber,
) -> Result<usize, io::Error> {
    format_two_digits(output, week_number.expand(), padding)
}

/// Format the full year into the designated output.
#[inline]
fn fmt_full_year(
    output: &mut (impl io::Write + ?Sized),
    full_year: Year,
    modifier::Year {
        padding,
        repr: _,
        range,
        iso_week_based: _,
        sign_is_mandatory,
    }: modifier::Year,
) -> Result<usize, error::Format> {
    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(
        output,
        full_year.is_negative(),
        sign_is_mandatory || (cfg!(feature = "large-dates") && full_year.get() >= 10_000),
    ));
    bytes += if cfg!(feature = "large-dates") && range == modifier::YearRange::Extended {
        // Safety: We just called `.abs()`, so zero is the minimum. The maximum is
        // unchanged.
        let value: RangedU32<0, 999_999> =
            unsafe { full_year.abs().narrow_unchecked::<0, 999_999>().into() };

        if let Some(value) = value.narrow::<0, 9_999>() {
            try_likely_ok!(format_four_digits(output, value.into(), padding))
        } else if let Some(value) = value.narrow::<0, 99_999>() {
            try_likely_ok!(format_five_digits_pad_zero(output, value))
        } else {
            try_likely_ok!(format_six_digits_pad_zero(output, value))
        }
    } else if let Some(value) = full_year.abs().narrow::<0, 9_999>() {
        try_likely_ok!(format_four_digits(output, value.into(), padding))
    } else {
        return Err(error::ComponentRange::conditional("year").into());
    };
    Ok(bytes)
}

/// Format the century into the designated output. Requires the full year be provided as an
/// argument.
#[inline]
fn fmt_century(
    output: &mut (impl io::Write + ?Sized),
    full_year: Year,
    modifier::Year {
        padding,
        repr: _,
        range,
        iso_week_based: _,
        sign_is_mandatory,
    }: modifier::Year,
) -> Result<usize, error::Format> {
    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(
        output,
        full_year.is_negative(),
        sign_is_mandatory || (cfg!(feature = "large-dates") && full_year.get() >= 10_000),
    ));
    bytes += if cfg!(feature = "large-dates") && range == modifier::YearRange::Extended {
        // Safety: The maximum divided by 100 is 9,9999, and the minimum is zero due to the
        // `.unsigned_abs()` call.
        let value: RangedU16<0, 9_999> =
            unsafe { RangedU16::new_unchecked((full_year.get().unsigned_abs() / 100).truncate()) };

        if let Some(value) = value.narrow::<0, 99>() {
            try_likely_ok!(format_two_digits(output, value.into(), padding))
        } else if let Some(value) = value.narrow::<0, 999>() {
            try_likely_ok!(format_three_digits(output, value, padding))
        } else {
            try_likely_ok!(format_four_digits(output, value, padding))
        }
    } else {
        // Safety: The maximum year in any configuration is 999,999, so dividing by 100
        // results in 9,999. The minimum is zero due to the `.unsigned_abs()` call.
        let value =
            unsafe { RangedU32::<0, 9_999>::new_unchecked(full_year.get().unsigned_abs() / 100) };

        if let Some(value) = value.narrow::<0, 99>() {
            try_likely_ok!(format_two_digits(output, value.into(), padding))
        } else {
            return Err(error::ComponentRange::conditional("year").into());
        }
    };
    Ok(bytes)
}

#[inline]
fn fmt_year_last_two(
    output: &mut (impl io::Write + ?Sized),
    full_year: Year,
    modifier::Year {
        padding,
        repr: _,
        range: _,
        iso_week_based: _,
        sign_is_mandatory: _,
    }: modifier::Year,
) -> Result<usize, io::Error> {
    let mut bytes = 0;
    // Safety: Modulus of 100 followed by `.unsigned_abs()` guarantees that the value
    // is in the range `0..=99`.
    let value =
        unsafe { RangedU8::new_unchecked((full_year.get() % 100).unsigned_abs().truncate()) };
    bytes += try_likely_ok!(format_two_digits(output, value, padding));
    Ok(bytes)
}

/// Format the hour into the designated output.
#[inline]
fn fmt_hour(
    output: &mut (impl io::Write + ?Sized),
    hour: Hours,
    modifier::Hour {
        padding,
        is_12_hour_clock,
    }: modifier::Hour,
) -> Result<usize, io::Error> {
    let value = match (hour.get(), is_12_hour_clock) {
        (_, false) => hour,
        (0 | 12, true) => Hours::new_static::<12>(),
        (_, true) if hour.get() < 12 => hour,
        // Safety: The resulting value is guaranteed to be in the range `1..=11`.
        (_, true) => unsafe { Hours::new_unchecked(hour.get() - 12) },
    };
    format_two_digits(output, value.expand(), padding)
}

/// Format the minute into the designated output.
#[inline]
fn fmt_minute(
    output: &mut (impl io::Write + ?Sized),
    minute: Minutes,
    modifier::Minute { padding }: modifier::Minute,
) -> Result<usize, io::Error> {
    format_two_digits(output, minute.expand(), padding)
}

/// Format the period into the designated output.
#[inline]
fn fmt_period(
    output: &mut (impl io::Write + ?Sized),
    period: Period,
    modifier::Period {
        is_uppercase,
        case_sensitive: _, // no effect on formatting
    }: modifier::Period,
) -> Result<usize, io::Error> {
    write(
        output,
        match (period, is_uppercase) {
            (Period::Am, false) => "am",
            (Period::Am, true) => "AM",
            (Period::Pm, false) => "pm",
            (Period::Pm, true) => "PM",
        },
    )
}

/// Format the second into the designated output.
#[inline]
fn fmt_second(
    output: &mut (impl io::Write + ?Sized),
    second: Seconds,
    modifier::Second { padding }: modifier::Second,
) -> Result<usize, io::Error> {
    format_two_digits(output, second.expand(), padding)
}

/// Format the subsecond into the designated output.
#[inline]
fn fmt_subsecond(
    output: &mut (impl io::Write + ?Sized),
    nanos: Nanoseconds,
    modifier::Subsecond { digits }: modifier::Subsecond,
) -> Result<usize, io::Error> {
    use modifier::SubsecondDigits::*;

    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Digits {
        _padding: MaybeUninit<[u8; 7]>,
        digit_1: u8,
        digits_2_thru_9: [u8; 8],
    }

    let [
        digit_1,
        digits_2_and_3,
        digits_4_and_5,
        digits_6_and_7,
        digits_8_and_9,
    ] = num_fmt::subsecond_from_nanos(nanos);

    // Ensure that digits 2 thru 9 are stored as a single array that is 8-aligned. This allows the
    // conversion to a `u64` to be zero cost, resulting in a nontrivial performance improvement.
    let buf = Digits {
        _padding: MaybeUninit::uninit(),
        digit_1: digit_1.as_bytes()[0],
        digits_2_thru_9: [
            digits_2_and_3.as_bytes()[0],
            digits_2_and_3.as_bytes()[1],
            digits_4_and_5.as_bytes()[0],
            digits_4_and_5.as_bytes()[1],
            digits_6_and_7.as_bytes()[0],
            digits_6_and_7.as_bytes()[1],
            digits_8_and_9.as_bytes()[0],
            digits_8_and_9.as_bytes()[1],
        ],
    };

    let len = match digits {
        One => 1,
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        OneOrMore => {
            // By converting the bytes into a single integer, we can effectively perform an equality
            // check against b'0' for all bytes at once. This is actually faster than
            // using portable SIMD (even with `-Ctarget-cpu=native`).
            let bitmask = u64::from_le_bytes(buf.digits_2_thru_9) ^ u64::from_le_bytes([b'0'; 8]);
            let digits_to_truncate = bitmask.leading_zeros() / 8;
            9 - digits_to_truncate as usize
        }
    };

    // Safety: All bytes are initialized and valid UTF-8, and `len` represents the number of bytes
    // we wish to display (that is between 1 and 9 inclusive). `Digits` is `#[repr(C)]`, so the
    // layout is guaranteed.
    let s = unsafe {
        num_fmt::StackStr::new(
            *(&raw const buf)
                .byte_add(core::mem::offset_of!(Digits, digit_1))
                .cast::<[MaybeUninit<u8>; 9]>(),
            len,
        )
    };
    write(output, &s)
}

#[inline]
fn fmt_sign(
    output: &mut (impl io::Write + ?Sized),
    is_negative: bool,
    sign_is_mandatory: bool,
) -> Result<usize, io::Error> {
    if is_negative {
        write(output, "-")
    } else if sign_is_mandatory {
        write(output, "+")
    } else {
        Ok(0)
    }
}

/// Format the offset hour into the designated output.
#[inline]
fn fmt_offset_hour(
    output: &mut (impl io::Write + ?Sized),
    is_negative: bool,
    hour: OffsetHours,
    modifier::OffsetHour {
        padding,
        sign_is_mandatory,
    }: modifier::OffsetHour,
) -> Result<usize, io::Error> {
    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(output, is_negative, sign_is_mandatory));
    // Safety: The value is guaranteed to be under 100 because of `OffsetHours`.
    bytes += try_likely_ok!(format_two_digits(
        output,
        unsafe { RangedU8::new_unchecked(hour.get().unsigned_abs()) },
        padding,
    ));
    Ok(bytes)
}

/// Format the offset minute into the designated output.
#[inline]
fn fmt_offset_minute(
    output: &mut (impl io::Write + ?Sized),
    offset_minute: OffsetMinutes,
    modifier::OffsetMinute { padding }: modifier::OffsetMinute,
) -> Result<usize, io::Error> {
    format_two_digits(
        output,
        // Safety: `OffsetMinutes` is guaranteed to be in the range `-59..=59`, so the absolute
        // value is guaranteed to be in the range `0..=59`.
        unsafe { RangedU8::new_unchecked(offset_minute.get().unsigned_abs()) },
        padding,
    )
}

/// Format the offset second into the designated output.
#[inline]
fn fmt_offset_second(
    output: &mut (impl io::Write + ?Sized),
    offset_second: OffsetSeconds,
    modifier::OffsetSecond { padding }: modifier::OffsetSecond,
) -> Result<usize, io::Error> {
    format_two_digits(
        output,
        // Safety: `OffsetSeconds` is guaranteed to be in the range `-59..=59`, so the absolute
        // value is guaranteed to be in the range `0..=59`.
        unsafe { RangedU8::new_unchecked(offset_second.get().unsigned_abs()) },
        padding,
    )
}

/// Format the Unix timestamp (in seconds) into the designated output.
#[inline]
fn fmt_unix_timestamp_seconds(
    output: &mut (impl io::Write + ?Sized),
    timestamp: i64,
    modifier::UnixTimestamp {
        precision,
        sign_is_mandatory,
    }: modifier::UnixTimestamp,
) -> Result<usize, io::Error> {
    debug_assert_eq!(precision, modifier::UnixTimestampPrecision::Second);

    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(output, timestamp < 0, sign_is_mandatory));
    bytes += try_likely_ok!(format_number_pad_none(output, timestamp.unsigned_abs()));
    Ok(bytes)
}

/// Format the Unix timestamp (in milliseconds) into the designated output.
#[inline]
fn fmt_unix_timestamp_milliseconds(
    output: &mut (impl io::Write + ?Sized),
    timestamp_millis: i64,
    modifier::UnixTimestamp {
        precision,
        sign_is_mandatory,
    }: modifier::UnixTimestamp,
) -> Result<usize, io::Error> {
    debug_assert_eq!(precision, modifier::UnixTimestampPrecision::Millisecond);

    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(output, timestamp_millis < 0, sign_is_mandatory));
    bytes += try_likely_ok!(format_number_pad_none(
        output,
        timestamp_millis.unsigned_abs()
    ));
    Ok(bytes)
}

/// Format the Unix timestamp into the designated output.
#[inline]
fn fmt_unix_timestamp_microseconds(
    output: &mut (impl io::Write + ?Sized),
    timestamp_micros: i128,
    modifier::UnixTimestamp {
        precision,
        sign_is_mandatory,
    }: modifier::UnixTimestamp,
) -> Result<usize, io::Error> {
    debug_assert_eq!(precision, modifier::UnixTimestampPrecision::Microsecond);

    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(output, timestamp_micros < 0, sign_is_mandatory));
    bytes += try_likely_ok!(format_number_pad_none(
        output,
        timestamp_micros.unsigned_abs()
    ));
    Ok(bytes)
}

/// Format the Unix timestamp into the designated output.
#[inline]
fn fmt_unix_timestamp_nanoseconds(
    output: &mut (impl io::Write + ?Sized),
    timestamp_nanos: i128,
    modifier::UnixTimestamp {
        precision,
        sign_is_mandatory,
    }: modifier::UnixTimestamp,
) -> Result<usize, io::Error> {
    debug_assert_eq!(precision, modifier::UnixTimestampPrecision::Nanosecond);

    let mut bytes = 0;
    bytes += try_likely_ok!(fmt_sign(output, timestamp_nanos < 0, sign_is_mandatory));
    bytes += try_likely_ok!(format_number_pad_none(
        output,
        timestamp_nanos.unsigned_abs()
    ));
    Ok(bytes)
}
