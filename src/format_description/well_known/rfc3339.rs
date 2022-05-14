//! The format described in RFC 3339.

/// The format described in [RFC 3339](https://tools.ietf.org/html/rfc3339#section-5.6).
///
/// Format example: 1985-04-12T23:20:50.52Z
///
/// # Examples
///
/// ```rust
/// # use time::{format_description::well_known::Rfc3339, macros::datetime, OffsetDateTime};
/// assert_eq!(
///     OffsetDateTime::parse("1985-04-12T23:20:50.52Z", &Rfc3339)?,
///     datetime!(1985-04-12 23:20:50.52 +00:00)
/// );
/// # Ok::<_, time::Error>(())
/// ```
///
/// ```rust
/// # use time::{format_description::well_known::Rfc3339, macros::datetime};
/// assert_eq!(
///     datetime!(1985-04-12 23:20:50.52 +00:00).format(&Rfc3339)?,
///     "1985-04-12T23:20:50.52Z"
/// );
/// # Ok::<_, time::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rfc3339;
