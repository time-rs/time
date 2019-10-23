use crate::format::{parse, ParseError, ParseResult, ParsedItems};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{DeferredFormat, Duration, Language};

/// An offset from UTC.
///
/// Guaranteed to store values up to ±23:59:59. Any values outside this range
/// may have incidental support that can change at any time without notice. If
/// you need support outside this range, please file an issue with your use
/// case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    /// The number of seconds offset from UTC. Positive is east, negative is
    /// west.
    pub(crate) seconds: i32,
}

impl UtcOffset {
    /// A `UtcOffset` that is UTC.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC, UtcOffset::seconds(0));
    /// ```
    pub const UTC: Self = Self::seconds(0);

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_hours(1).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_hours(2).as_minutes(), 120);
    /// ```
    pub const fn east_hours(hours: u8) -> Self {
        Self::hours(hours as i8)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_hours(1).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_hours(2).as_minutes(), -120);
    /// ```
    pub const fn west_hours(hours: u8) -> Self {
        Self::hours(-(hours as i8))
    }

    /// Create a `UtcOffset` representing an offset by the number of hours
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(1).as_hours(), 1);
    /// assert_eq!(UtcOffset::hours(-1).as_hours(), -1);
    /// assert_eq!(UtcOffset::hours(2).as_minutes(), 120);
    /// assert_eq!(UtcOffset::hours(-2).as_minutes(), -120);
    /// ```
    pub const fn hours(hours: i8) -> Self {
        Self::seconds(hours as i32 * 3_600)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_minutes(60).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_minutes(30).as_minutes(), 30);
    /// ```
    pub const fn east_minutes(minutes: u16) -> Self {
        Self::minutes(minutes as i16)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_minutes(60).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_minutes(30).as_minutes(), -30);
    /// ```
    pub const fn west_minutes(minutes: u16) -> Self {
        Self::minutes(-(minutes as i16))
    }

    /// Create a `UtcOffset` representing a offset by the number of minutes
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::minutes(60).as_hours(), 1);
    /// assert_eq!(UtcOffset::minutes(-60).as_hours(), -1);
    /// assert_eq!(UtcOffset::minutes(30).as_minutes(), 30);
    /// assert_eq!(UtcOffset::minutes(-30).as_minutes(), -30);
    /// ```
    pub const fn minutes(minutes: i16) -> Self {
        Self::seconds(minutes as i32 * 60)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_seconds(1_800).as_minutes(), 30);
    /// ```
    pub const fn east_seconds(seconds: u32) -> Self {
        Self::seconds(seconds as i32)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_seconds(3_600).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_seconds(1_800).as_minutes(), -30);
    /// ```
    pub const fn west_seconds(seconds: u32) -> Self {
        Self::seconds(-(seconds as i32))
    }

    /// Create a `UtcOffset` representing an offset by the number of seconds
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::seconds(-3_600).as_hours(), -1);
    /// assert_eq!(UtcOffset::seconds(1_800).as_minutes(), 30);
    /// assert_eq!(UtcOffset::seconds(-1_800).as_minutes(), -30);
    /// ```
    pub const fn seconds(seconds: i32) -> Self {
        Self { seconds }
    }

    /// Get the number of seconds from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_seconds(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_seconds(), 43_200);
    /// assert_eq!(UtcOffset::hours(-12).as_seconds(), -43_200);
    /// assert_eq!(UtcOffset::hours(24).as_seconds(), 86_400);
    /// assert_eq!(UtcOffset::hours(-24).as_seconds(), -86_400);
    /// ```
    pub const fn as_seconds(self) -> i32 {
        self.seconds
    }

    /// Get the number of minutes from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_minutes(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_minutes(), 720);
    /// assert_eq!(UtcOffset::hours(-12).as_minutes(), -720);
    /// assert_eq!(UtcOffset::hours(24).as_minutes(), 1_440);
    /// assert_eq!(UtcOffset::hours(-24).as_minutes(), -1_440);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_minutes(self) -> i16 {
        (self.as_seconds() / 60) as i16
    }

    /// Get the number of hours from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_hours(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_hours(), 12);
    /// assert_eq!(UtcOffset::hours(-12).as_hours(), -12);
    /// assert_eq!(UtcOffset::hours(24).as_hours(), 24);
    /// assert_eq!(UtcOffset::hours(-24).as_hours(), -24);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_hours(self) -> i8 {
        (self.as_seconds() / 3_600) as i8
    }

    /// Convert a `UtcOffset` to ` Duration`. Useful for implementing operators.
    pub(crate) fn as_duration(self) -> Duration {
        Duration::seconds(self.seconds as i64)
    }
}

/// Methods that allow parsing and formatting the `UtcOffset`.
impl UtcOffset {
    /// Format the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2).format("%z"), "+0200");
    /// assert_eq!(UtcOffset::hours(-2).format("%z"), "-0200");
    /// assert_eq!(UtcOffset::hours(12).format("%z"), "+1200");
    /// assert_eq!(UtcOffset::hours(-12).format("%z"), "-1200");
    /// ```
    pub fn format(self, format: &str) -> String {
        DeferredFormat {
            date: None,
            time: None,
            offset: Some(self),
            format: crate::format::parse_with_language(format, Language::en),
        }
        .to_string()
    }

    /// Attempt to parse the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::parse("+0200", "%z"), Ok(UtcOffset::hours(2)));
    /// assert_eq!(UtcOffset::parse("-0200", "%z"), Ok(UtcOffset::hours(-2)));
    /// assert_eq!(UtcOffset::parse("+1200", "%z"), Ok(UtcOffset::hours(12)));
    /// assert_eq!(UtcOffset::parse("-1200", "%z"), Ok(UtcOffset::hours(-12)));
    /// ```
    pub fn parse(s: &str, format: &str) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s, format, Language::en)?)
    }

    /// Given the items already parsed, attempt to create a `UtcOffset`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        items.offset.ok_or(ParseError::InsufficientInformation)
    }
}
