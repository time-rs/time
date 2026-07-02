//! Use the well-known [RFC6265 format] when serializing and deserializing an [`OffsetDateTime`].
//! Serialization emits the RFC6265 `sane-cookie-date` form, while deserialization accepts the
//! broader `cookie-date` syntax used for `Expires` attribute values.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [RFC6265 format]: https://datatracker.ietf.org/doc/html/rfc6265
//! [with]: https://serde.rs/field-attrs.html#with

#[cfg(feature = "parsing")]
use core::marker::PhantomData;

#[cfg(feature = "parsing")]
use serde_core::Deserializer;
#[cfg(feature = "formatting")]
use serde_core::ser::Error as _;
#[cfg(feature = "formatting")]
use serde_core::{Serialize, Serializer};

#[cfg(feature = "parsing")]
use super::Visitor;
use crate::OffsetDateTime;
use crate::format_description::well_known::Rfc6265;

/// Serialize an [`OffsetDateTime`] using the well-known RFC6265 format.
#[cfg(feature = "formatting")]
#[inline]
pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    datetime
        .format(&Rfc6265)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its RFC6265 representation.
#[cfg(feature = "parsing")]
#[inline]
pub fn deserialize<'a, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'a>,
{
    deserializer.deserialize_str(Visitor::<Rfc6265>(PhantomData))
}

/// Use the well-known [RFC6265 format] when serializing and deserializing an
/// [`Option<OffsetDateTime>`].
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// Note: Due to [serde-rs/serde#2878], you will need to apply `#[serde(default)]` if you want a
/// missing field to deserialize as `None`.
///
/// [RFC6265 format]: https://datatracker.ietf.org/doc/html/rfc6265
/// [with]: https://serde.rs/field-attrs.html#with
/// [serde-rs/serde#2878]: https://github.com/serde-rs/serde/issues/2878
pub mod option {
    use super::*;

    /// Serialize an [`Option<OffsetDateTime>`] using the well-known RFC6265 format.
    #[cfg(feature = "formatting")]
    #[inline]
    pub fn serialize<S>(option: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        option
            .map(|odt| odt.format(&Rfc6265))
            .transpose()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`Option<OffsetDateTime>`] from its RFC6265 representation.
    #[cfg(feature = "parsing")]
    #[inline]
    pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_option(Visitor::<Option<Rfc6265>>(PhantomData))
    }
}
