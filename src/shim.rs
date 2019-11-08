use crate::Sign::{self, Negative, Positive, Zero};

/// Provide common methods that do not exist on all number types.
pub(crate) trait NumberExt: Sized + PartialOrd {
    /// Get the absolute value of the number.
    fn abs(self) -> Self;

    /// Get the value zero.
    fn zero() -> Self;

    /// Obtain the sign of the number.
    fn sign(self) -> Sign;
}

macro_rules! unsigned {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                #[inline(always)]
                fn abs(self) -> Self {
                    self
                }

                #[inline(always)]
                fn zero() -> Self {
                    0
                }

                #[inline(always)]
                fn sign(self) -> Sign {
                    if self > 0 {
                        Positive
                    } else {
                        Zero
                    }
                }
            }
        )*
    };
}

macro_rules! signed {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                #[inline(always)]
                fn abs(self) -> Self {
                    self.abs()
                }

                #[inline(always)]
                fn zero() -> Self {
                    0
                }

                #[inline(always)]
                fn sign(self) -> Sign {
                    if self > 0 {
                        Positive
                    } else if self < 0 {
                        Negative
                    } else {
                        Zero
                    }
                }
            }
        )*
    };
}

macro_rules! float {
    ($($type:ty),*) => {
        $(
            impl NumberExt for $type {
                #[inline(always)]
                fn abs(self) -> Self {
                    if self < 0. {
                        -self
                    } else {
                        self
                    }
                }

                #[inline(always)]
                fn zero() -> Self {
                    0.
                }

                #[inline(always)]
                fn sign(self) -> Sign {
                    if self == 0. {
                        Zero
                    } else if self.is_sign_positive() {
                        Positive
                    } else { // self.is_sign_negative()
                        Negative
                    }
                }
            }
        )*
    };
}

unsigned![u8, u16, u32, u64, u128];
signed![i8, i16, i32, i64, i128];
float![f32, f64];

#[cfg(test)]
mod test {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn abs() {
        assert_eq!(1_u8.abs(), 1);
        assert_eq!(1_u16.abs(), 1);
        assert_eq!(1_u32.abs(), 1);
        assert_eq!(1_u64.abs(), 1);
        assert_eq!(1_u128.abs(), 1);

        assert_eq!(1_i8.abs(), 1);
        assert_eq!(1_i16.abs(), 1);
        assert_eq!(1_i32.abs(), 1);
        assert_eq!(1_i64.abs(), 1);
        assert_eq!(1_i128.abs(), 1);

        assert_eq!((-1_i8).abs(), 1);
        assert_eq!((-1_i16).abs(), 1);
        assert_eq!((-1_i32).abs(), 1);
        assert_eq!((-1_i64).abs(), 1);
        assert_eq!((-1_i128).abs(), 1);

        assert_eq!(1_f32.abs(), 1.);
        assert_eq!(1_f64.abs(), 1.);

        assert_eq!((-1_f32).abs(), 1.);
        assert_eq!((-1_f64).abs(), 1.);
    }

    #[test]
    fn zero() {
        assert_eq!(u8::zero(), 0);
        assert_eq!(u16::zero(), 0);
        assert_eq!(u32::zero(), 0);
        assert_eq!(u64::zero(), 0);
        assert_eq!(u128::zero(), 0);

        assert_eq!(i8::zero(), 0);
        assert_eq!(i16::zero(), 0);
        assert_eq!(i32::zero(), 0);
        assert_eq!(i64::zero(), 0);
        assert_eq!(i128::zero(), 0);

        assert_eq!(f32::zero(), 0.);
        assert_eq!(f64::zero(), 0.);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn sign() {
        assert_eq!(1_u8.sign(), Positive);
        assert_eq!(1_u16.sign(), Positive);
        assert_eq!(1_u32.sign(), Positive);
        assert_eq!(1_u64.sign(), Positive);
        assert_eq!(1_u128.sign(), Positive);

        assert_eq!(0_u8.sign(), Zero);
        assert_eq!(0_u16.sign(), Zero);
        assert_eq!(0_u32.sign(), Zero);
        assert_eq!(0_u64.sign(), Zero);
        assert_eq!(0_u128.sign(), Zero);

        assert_eq!(1_i8.sign(), Positive);
        assert_eq!(1_i16.sign(), Positive);
        assert_eq!(1_i32.sign(), Positive);
        assert_eq!(1_i64.sign(), Positive);
        assert_eq!(1_i128.sign(), Positive);

        assert_eq!((-1_i8).sign(), Negative);
        assert_eq!((-1_i16).sign(), Negative);
        assert_eq!((-1_i32).sign(), Negative);
        assert_eq!((-1_i64).sign(), Negative);
        assert_eq!((-1_i128).sign(), Negative);

        assert_eq!(0_i8.sign(), Zero);
        assert_eq!(0_i16.sign(), Zero);
        assert_eq!(0_i32.sign(), Zero);
        assert_eq!(0_i64.sign(), Zero);
        assert_eq!(0_i128.sign(), Zero);

        assert_eq!(1_f32.sign(), Positive);
        assert_eq!(1_f64.sign(), Positive);

        assert_eq!((-1_f32).sign(), Negative);
        assert_eq!((-1_f64).sign(), Negative);

        assert_eq!(0_f32.sign(), Zero);
        assert_eq!(0_f64.sign(), Zero);
    }
}
