//! Indeterminate offset

use core::fmt;

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateOffset;

impl fmt::Display for IndeterminateOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for IndeterminateOffset {}

impl From<IndeterminateOffset> for crate::Error {
    fn from(_: IndeterminateOffset) -> Self {
        Self::IndeterminateOffset
    }
}
