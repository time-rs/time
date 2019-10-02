use crate::sign::Sign::{self, Negative, Positive, Unknown, Zero};
use core::cmp::Ordering::{Equal, Greater, Less};

/// Provide common methods that do not exist on all number types.
pub(crate) trait NumberExt: Sized + PartialOrd {
    /// Get the absolute value of the number.
    fn abs(self) -> Self;

    /// Get the value zero.
    fn zero() -> Self;

    /// Obtain the sign of the number, if known.
    fn sign(self) -> Sign {
        match self.partial_cmp(&Self::zero()) {
            Some(Less) => Negative,
            Some(Equal) => Zero,
            Some(Greater) => Positive,
            None => Unknown,
        }
    }
}

macro_rules! unsigned {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                fn abs(self) -> Self { self }
                fn zero() -> Self { 0 }
            }
        )*
    };
}

macro_rules! signed {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                fn abs(self) -> Self { self.abs() }
                fn zero() -> Self { 0 }
            }
        )*
    };
}

macro_rules! float {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                fn abs(self) -> Self {
                    if self < 0. { -self }
                    else { self }
                }
                fn zero() -> Self { 0. }
            }
        )*
    };
}

unsigned![u8, u16, u32, u64, u128];
signed![i8, i16, i32, i64, i128];
float![f32, f64];
