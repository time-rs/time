//! Extension traits for things either not implemented or not yet stable in the MSRV.

/// `[u8]::strip_prefix()`
///
/// Stabilized in Rust 1.51. Once on stable, Standback can be used until the MSRV is met on
/// 2021-09-25.
pub(crate) trait SliceStripPrefix<T> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn strip_prefix_(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq;
}

impl<T> SliceStripPrefix<T> for [T] {
    #[must_use = "returns the subslice without modifying the original"]
    fn strip_prefix_(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let n = prefix.len();
        if n <= self.len() {
            let (head, tail) = self.split_at(n);
            if head == prefix {
                return Some(tail);
            }
        }
        None
    }
}

/// Equivalent of `foo.parse()` for slices.
pub(crate) trait IntegerParseBytes<T> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(&self) -> Option<T>;
}

impl<T: Integer> IntegerParseBytes<T> for [u8] {
    fn parse_bytes(&self) -> Option<T> {
        T::parse_bytes(self)
    }
}

/// Marker trait for all integer types, including `NonZero*`
pub(crate) trait Integer: Sized {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(src: &[u8]) -> Option<Self>;
}

macro_rules! impl_parse_bytes {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                if src.is_empty() {
                    return None;
                }

                #[allow(unused_comparisons)]
                let is_signed_ty = 0 > Self::MIN;

                let (is_positive, digits) = match src {
                    [b'+'] | [b'-'] => return None,
                    [b'+', remaining @ ..] => (true, remaining),
                    [b'-', remaining @ ..] if is_signed_ty => (false, remaining),
                    _ => (true, src),
                };

                let mut result: Self = 0;
                #[allow(trivial_numeric_casts)]
                if is_positive {
                    // The number is positive
                    for &c in digits {
                        let x = (c as char).to_digit(10)?;
                        result = result.checked_mul(10)?.checked_add(x as Self)?;
                    }
                } else {
                    // The number is negative
                    for &c in digits {
                        let x = (c as char).to_digit(10)?;
                        result = result.checked_mul(10)?.checked_sub(x as Self)?;
                    }
                }
                Some(result)
            }
        }
    )*)
}
impl_parse_bytes! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

macro_rules! impl_parse_bytes_nonzero {
    ($($t:ty)*) => {$(
        impl Integer for $t {
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                Self::new(src.parse_bytes()?)
            }
        }
    )*}
}

impl_parse_bytes_nonzero! {
    core::num::NonZeroU8
    core::num::NonZeroU16
    core::num::NonZeroU32
    core::num::NonZeroU64
    core::num::NonZeroU128
    core::num::NonZeroUsize
    core::num::NonZeroI8
    core::num::NonZeroI16
    core::num::NonZeroI32
    core::num::NonZeroI64
    core::num::NonZeroI128
    core::num::NonZeroIsize
}
