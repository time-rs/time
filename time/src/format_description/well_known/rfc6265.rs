//! The RFC6265 `cookie-date` and `sane-cookie-date` formats.

/// The RFC6265 `cookie-date` and `sane-cookie-date` formats.
///
/// When parsing, this accepts the permissive `cookie-date` algorithm used by user agents in
/// [RFC6265 section 5.1.1]. When formatting, this emits the narrower `sane-cookie-date` form used
/// by `Expires` in [RFC6265 section 4.1.1]: `Sun, 06 Nov 1994 08:49:37 GMT`.
///
/// This format parses an `Expires` attribute value, not a complete `Set-Cookie` header. Parsing
/// extracts only the time, day, month, and year components required by the RFC6265 algorithm.
/// Weekday names and time zone tokens are ignored, and the resulting value uses the UTC offset.
/// [RFC6265 section 5.2.1] is the step that passes the `Expires` attribute value to the
/// `cookie-date` parser.
///
/// Parsing follows the RFC6265 token algorithm: the first matching time, day, month, and year
/// tokens are used, and later matching tokens are ignored. Two-digit years are normalized as
/// specified by RFC6265: `70..=99` map to `1970..=1999`, and `00..=69` map to `2000..=2069`.
///
/// Formatting always converts the value to UTC and emits the literal `GMT`.
///
/// [RFC6265 section 5.1.1]: https://datatracker.ietf.org/doc/html/rfc6265#section-5.1.1
/// [RFC6265 section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6265#section-4.1.1
/// [RFC6265 section 5.2.1]: https://datatracker.ietf.org/doc/html/rfc6265#section-5.2.1
///
#[cfg_attr(feature = "parsing", doc = "```rust")]
#[cfg_attr(not(feature = "parsing"), doc = "```rust,ignore")]
/// # use time::{format_description::well_known::Rfc6265, OffsetDateTime};
/// # fn main() -> time::Result<()> {
/// assert_eq!(
///     OffsetDateTime::parse("Sun, 06 Nov 1994 08:49:37 GMT", &Rfc6265)?,
///     time::macros::datetime!(1994-11-06 08:49:37 UTC),
/// );
/// # Ok(())
/// # }
/// ```
///
#[cfg_attr(feature = "formatting", doc = "```rust")]
#[cfg_attr(not(feature = "formatting"), doc = "```rust,ignore")]
/// # use time::format_description::well_known::Rfc6265;
/// # fn main() -> time::Result<()> {
/// assert_eq!(
///     time::macros::datetime!(1994-11-06 03:49:37 -5).format(&Rfc6265)?,
///     "Sun, 06 Nov 1994 08:49:37 GMT",
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rfc6265;
