//! Provides convenient way for declaring custom serializer/deserializer modules.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//!
//! [with]: https://serde.rs/field-attrs.html#with

/// Declares a custom format based on the given [`FormatItem`].
///
/// The syntax accepted by this macro is the same as [`format_description::parse()`], which can
/// be found in [the book](https://time-rs.github.io/book/api/format-description.html).
///
/// # Usage
///
/// There are two ways to use this:
/// - `declare_format_string!("<format string>")`: pollutes the current namespace; you'll usually
///   use this inside of a `mod mod_name { }` block.
/// - `declare_format_string!(mod_name, "<format string>")`: puts a module named `mod_name` in the
///   current namespace.
///
/// # Examples
///
/// Use the [`Rfc3339`] format.
///
/// ```
/// # use time::OffsetDateTime;
/// # use serde::{Serialize, Deserialize};
/// mod rfc3339_format {
///   use time::declare_format;
///   use time::format_description::well_known::Rfc3339;
///   declare_format!(Rfc3339);
/// }
///
/// #[derive(Serialize, Deserialize)]
/// struct SerializesWithRfc3339 {
///     #[serde(with = "rfc3339_format")]
///     dt: OffsetDateTime,
/// }
/// #
/// # // otherwise rustdoc tests don't work because we put a module in `main()`
/// # fn main() {}
/// ```
///
/// [`FormatItem`]: crate::format_description::FormatItem
/// [`Rfc3339`]: crate::format_description::well_known::Rfc3339
#[macro_export]
macro_rules! declare_format {
    ($i:ident, $e:tt) => {
        mod $i {
            time::declare_format!($e);
        }
    };
    ($e:tt) => {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use time::OffsetDateTime;
        use time::error::{Parse, Format};

        /// Serialize an `OffsetDateTime` as the given format.
        pub fn serialize<S: Serializer>(
            datetime: &OffsetDateTime,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            datetime.format(&$e)
                .map_err(Format::into_invalid_serde_value::<S>)?
                .serialize(serializer)
        }

        /// Deserialize an `OffsetDateTime` from the given format.
        pub fn deserialize<'a, D: Deserializer<'a>>(
            deserializer: D,
        ) -> Result<OffsetDateTime, D::Error> {
            OffsetDateTime::parse(<_>::deserialize(deserializer)?, &$e)
                .map_err(Parse::to_invalid_serde_value::<D>)
        }
    }
}

/// Declares a custom format based on the provided string.
///
/// The syntax accepted by this macro is the same as [`format_description::parse()`], which can
/// be found in [the book](https://time-rs.github.io/book/api/format-description.html).
///
/// # Usage
///
/// There are two ways to use this:
/// - `declare_format_string!("<format string>")`: pollutes the current namespace; you'll usually
///   use this inside of a `mod mod_name { }` block.
/// - `declare_format_string!(mod_name, "<format string>")`: puts a module named `mod_name` in the
///   current namespace.
///
/// # Examples
///
/// ```
/// # use time::{declare_format_string, OffsetDateTime};
/// # use serde::{Serialize, Deserialize};
/// // Makes a module `mod my_format { ... }`.
/// declare_format_string!(my_format, "hour=[hour], minute=[minute]");
///
/// #[derive(Serialize, Deserialize)]
/// struct SerializesWithCustom {
///     #[serde(with = "my_format")]
///     dt: OffsetDateTime,
/// }
/// #
/// # // otherwise rustdoc tests don't work because we put a module in `main()`
/// # fn main() {}
/// ```
///
/// [`format_description::parse()`]: crate::format_description::parse()
#[macro_export]
macro_rules! declare_format_string {
    ($i:ident, $e:tt) => {
        mod $i {
            time::declare_format_string!($e);
        }
    };
    ($e:tt) => {
        use time::declare_format;
        use time::format_description::FormatItem;
        use time::macros::format_description;

        const FORMAT: &'static [FormatItem<'static>] = format_description!($e);

        declare_format!(FORMAT);
    };
}
