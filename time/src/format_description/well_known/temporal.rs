//! The date/time format used by the ECMAScript [`Temporal`] proposal, based on [RFC 9557] (an
//! extension of [RFC 3339]).
//!
//! [`Temporal`]: https://tc39.es/proposal-temporal/#sec-temporal-iso8601grammar
//! [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557.html
//! [RFC 3339]: https://tools.ietf.org/html/rfc3339#section-5.6

/// The date/time format used by the ECMAScript [`Temporal`] proposal.
///
/// This is the [RFC 9557] "Internet Extended Date/Time Format" (IXDTF), a superset of
/// [RFC 3339] that additionally permits a trailing time-zone annotation and any number of
/// key–value annotations, for example
/// `1996-12-19T16:39:57-08:00[America/Los_Angeles][u-ca=iso8601]`.
///
/// It differs from [`Rfc3339`](super::Rfc3339) in that the date/time separator is restricted to
/// `T`, `t`, or a space, the seconds component is optional, leap seconds are not accepted, and the
/// IXDTF annotation suffix is understood.
///
/// As `time` models neither named time zones nor alternative calendars, annotations are validated
/// for syntactic correctness and then discarded; only the numeric UTC offset is retained. In
/// accordance with RFC 9557, an annotation flagged as *critical* (prefixed with `!`) whose meaning
/// cannot be honoured causes parsing to fail. As no annotations can be produced, formatting yields
/// the RFC 9557 date-time, which is itself a valid Temporal string.
///
/// Format example: 1996-12-19T16:39:57-08:00
///
/// # Examples
#[cfg_attr(feature = "parsing", doc = "```rust")]
#[cfg_attr(not(feature = "parsing"), doc = "```rust,ignore")]
/// # use time::{format_description::well_known::Temporal, OffsetDateTime};
/// # use time_macros::datetime;
/// // The time-zone and calendar annotations are validated and discarded.
/// assert_eq!(
///     OffsetDateTime::parse(
///         "1996-12-19T16:39:57-08:00[America/Los_Angeles][u-ca=iso8601]",
///         &Temporal,
///     )?,
///     datetime!(1996-12-19 16:39:57 -08:00)
/// );
/// # Ok::<_, time::Error>(())
/// ```
///
#[cfg_attr(feature = "formatting", doc = "```rust")]
#[cfg_attr(not(feature = "formatting"), doc = "```rust,ignore")]
/// # use time::format_description::well_known::Temporal;
/// # use time_macros::datetime;
/// assert_eq!(
///     datetime!(1996-12-19 16:39:57 -08:00).format(&Temporal)?,
///     "1996-12-19T16:39:57-08:00"
/// );
/// # Ok::<_, time::Error>(())
/// ```
/// 
/// [`Temporal`]: https://tc39.es/proposal-temporal/#sec-temporal-iso8601grammar
/// [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557.html
/// [RFC 3339]: https://tools.ietf.org/html/rfc3339#section-5.6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Temporal;
