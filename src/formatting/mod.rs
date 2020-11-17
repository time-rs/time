//! Formatting for the time crate.

pub(crate) mod format;
mod format_description;

#[cfg(feature = "alloc")]
pub use format_description::parse::parse_format_description;
pub use format_description::{modifier, Component, FormatDescription};

/// Errors that can be returned.
pub mod error {
    pub use crate::formatting::format::Error;
    #[cfg(feature = "alloc")]
    pub use crate::formatting::format_description::error::InvalidFormatDescription;
}
