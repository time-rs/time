//! Serde serializer/deserializers for well-known types.
//!
//! Use these modules in combination with serde's [`#[with]`][with] attribute.
//!
//! [with]: https://serde.rs/field-attrs.html#with

/// Treat an `OffsetDateTime` as a [RFC 3339] string for the purposes of serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// [with]: https://serde.rs/field-attrs.html#with
pub mod rfc3339 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::error::{Format, Parse};
    use crate::format_description::well_known::Rfc3339;
    use crate::OffsetDateTime;

    /// Serialize an `Option<OffsetDateTime>` in RFC 3339 format.
    pub fn serialize<S: Serializer>(
        datetime: &OffsetDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .format(&Rfc3339)
            .map_err(Format::into_invalid_serde_value::<S>)?
            .serialize(serializer)
    }

    /// Deserialize an `Option<OffsetDateTime>` from RFC 3339 format.
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<OffsetDateTime, D::Error> {
        OffsetDateTime::parse(<&str>::deserialize(deserializer)?, &Rfc3339)
            .map_err(Parse::to_invalid_serde_value::<D>)
    }

    /// Treat an `Option<OffsetDateTime>` as a [RFC 3339] string for the
    /// purposes of serde.
    ///
    /// Use this module in combination with serde's [`#[with]`][with] attribute.
    ///
    /// [RFC 3339]: https://tools.ietf.org/html/rfc3339#section-5.6
    /// [with]: https://serde.rs/field-attrs.html#with
    pub mod option {
        #[allow(clippy::wildcard_imports)]
        use super::*;

        /// Serialize an `Option<OffsetDateTime>` in RFC 3339 format.
        pub fn serialize<S: Serializer>(
            option: &Option<OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            option
                .map(|dt| {
                    dt.format(&Rfc3339)
                        .map_err(Format::into_invalid_serde_value::<S>)
                })
                .transpose()?
                .serialize(serializer)
        }

        /// Deserialize an `Option<OffsetDateTime>` from RFC 3339 format.
        pub fn deserialize<'a, D: Deserializer<'a>>(
            deserializer: D,
        ) -> Result<Option<OffsetDateTime>, D::Error> {
            Option::deserialize(deserializer)?
                .map(|s| OffsetDateTime::parse(s, &Rfc3339))
                .transpose()
                .map_err(Parse::to_invalid_serde_value::<D>)
        }
    }
}
