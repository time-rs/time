#![allow(clippy::missing_docs_in_private_items)]

use core::{
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
    time::Duration,
};

/// Check if a range contains the given value. Equivalent to
/// `range.contains(&item)`, but works on older compilers.
pub(crate) fn range_contains<T, U>(range: &impl RangeBounds<T>, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: ?Sized + PartialOrd<T>,
{
    (match range.start_bound() {
        Included(start) => start <= item,
        Excluded(start) => start < item,
        Unbounded => true,
    }) && (match range.end_bound() {
        Included(end) => item <= end,
        Excluded(end) => item < end,
        Unbounded => true,
    })
}

pub(crate) trait EuclidShim {
    /// Get the Euclidean remainder.
    fn rem_euclid_shim(self, rhs: Self) -> Self;
}

macro_rules! impl_euclid_shim_signed {
    ($($type:ty),* $(,)?) => {
        $(
            impl EuclidShim for $type {
                #[inline]
                fn rem_euclid_shim(self, rhs: Self) -> Self {
                    let r = self % rhs;
                    if r < 0 {
                        if rhs < 0 {
                            r - rhs
                        } else {
                            r + rhs
                        }
                    } else {
                        r
                    }
                }
            }
        )*
    };
}
impl_euclid_shim_signed![i8, i16, i32, i64, i128, isize];

macro_rules! impl_euclid_shim_unsigned {
    ($($type:ty),* $(,)?) => {
        $(
            impl EuclidShim for $type {
                #[inline]
                fn rem_euclid_shim(self, rhs: Self) -> Self {
                    self % rhs
                }
            }
        )*
    };
}
impl_euclid_shim_unsigned![u8, u16, u32, u64, u128, usize];

pub(crate) trait DurationShim {
    /// Get the number of seconds in a `Duration` as a 64 bit float.
    fn as_secs_f64(&self) -> f64;
}
impl DurationShim for Duration {
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn as_secs_f64(&self) -> f64 {
        (self.as_secs() as f64) + (self.as_nanos() as f64) / (1_000_000_000.)
    }
}
