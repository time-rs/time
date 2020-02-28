#![allow(deprecated)]

use core::ops::{Div, DivAssign, Mul, MulAssign, Neg, Not};
use Sign::{Negative, Positive, Zero};

/// Contains the sign of a value: positive, negative, or zero.
///
/// For ease of use, `Sign` implements [`Mul`] and [`Div`] on all signed numeric
/// types. `Sign`s can also be multiplied and divided by another `Sign`, which
/// follows the same rules as real numbers.
#[repr(i8)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
    serde(try_from = "crate::serde::Sign", into = "crate::serde::Sign")
)]
#[deprecated(
    since = "0.2.7",
    note = "The only use for this (obtaining the sign of a `Duration`) can be replaced with \
            `Duration::is_{positive|negative|zero}`"
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sign {
    /// A positive value.
    Positive = 1,

    /// A negative value.
    Negative = -1,

    /// A value that is exactly zero.
    Zero = 0,
}

impl Default for Sign {
    /// `Sign` defaults to `Zero`.
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert_eq!(Sign::default(), Sign::Zero);
    /// ```
    #[inline(always)]
    fn default() -> Self {
        Zero
    }
}

macro_rules! sign_mul {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Mul<$type> for Sign {
                type Output = $type;

                #[allow(trivial_numeric_casts)]
                #[inline(always)]
                fn mul(self, rhs: $type) -> Self::Output {
                    (self as i8) as $type * rhs
                }
            }

            impl Mul<Sign> for $type {
                type Output = Self;

                #[inline(always)]
                fn mul(self, rhs: Sign) -> Self::Output {
                    rhs * self
                }
            }

            impl MulAssign<Sign> for $type {
                #[inline(always)]
                fn mul_assign(&mut self, rhs: Sign) {
                    if rhs.is_negative() {
                        *self = -*self;
                    }
                }
            }

            impl Div<Sign> for $type {
                type Output = Self;

                #[inline(always)]
                fn div(self, rhs: Sign) -> Self::Output {
                    self * rhs
                }
            }

            impl DivAssign<Sign> for $type {
                #[inline(always)]
                fn div_assign(&mut self, rhs: Sign) {
                    *self *= rhs
                }
            }
        )*
    };
}
sign_mul![i8, i16, i32, i64, i128, f32, f64];

impl Mul<Sign> for Sign {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, _) | (_, Zero) => Zero,
            (Positive, Positive) | (Negative, Negative) => Positive,
            (Positive, Negative) | (Negative, Positive) => Negative,
        }
    }
}

impl MulAssign<Sign> for Sign {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<Sign> for Sign {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl DivAssign<Sign> for Sign {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs
    }
}

impl Neg for Sign {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Not for Sign {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        self.negate()
    }
}

impl Sign {
    /// Return the opposite of the current sign.
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert_eq!(Sign::Positive.negate(), Sign::Negative);
    /// assert_eq!(Sign::Negative.negate(), Sign::Positive);
    /// assert_eq!(Sign::Zero.negate(), Sign::Zero);
    /// ```
    #[inline(always)]
    pub fn negate(self) -> Self {
        match self {
            Positive => Negative,
            Negative => Positive,
            Zero => Zero,
        }
    }

    /// Is the sign positive?
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert!(Sign::Positive.is_positive());
    /// assert!(!Sign::Negative.is_positive());
    /// assert!(!Sign::Zero.is_positive());
    /// ```
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self as u8 == Positive as u8
    }

    /// Is the sign negative?
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert!(!Sign::Positive.is_negative());
    /// assert!(Sign::Negative.is_negative());
    /// assert!(!Sign::Zero.is_negative());
    /// ```
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        self as u8 == Negative as u8
    }

    /// Is the value exactly zero?
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert!(!Sign::Positive.is_zero());
    /// assert!(!Sign::Negative.is_zero());
    /// assert!(Sign::Zero.is_zero());
    /// ```
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self as u8 == Zero as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! op_assign {
        ($a:ident $op:tt $b:ident) => {{
            let mut v = $a;
            v $op $b;
            v
        }};
    }

    #[test]
    fn default() {
        assert_eq!(Sign::default(), Zero);
    }

    #[test]
    fn sign_mul_int() {
        assert_eq!(Positive * 2, 2);
        assert_eq!(Negative * 2, -2);
        assert_eq!(Zero * 2, 0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn sign_mul_float() {
        assert_eq!(Positive * 2., 2.);
        assert_eq!(Negative * 2., -2.);
        assert_eq!(Zero * 2., 0.);
    }

    #[test]
    fn sign_mul_sign() {
        assert_eq!(Zero * Positive, Zero);
        assert_eq!(Zero * Negative, Zero);
        assert_eq!(Zero * Zero, Zero);
        assert_eq!(Positive * Zero, Zero);
        assert_eq!(Negative * Zero, Zero);
        assert_eq!(Positive * Positive, Positive);
        assert_eq!(Positive * Negative, Negative);
        assert_eq!(Negative * Positive, Negative);
        assert_eq!(Negative * Negative, Positive);
    }

    #[test]
    fn sign_mul_assign_sign() {
        assert_eq!(op_assign!(Zero *= Positive), Zero);
        assert_eq!(op_assign!(Zero *= Negative), Zero);
        assert_eq!(op_assign!(Zero *= Zero), Zero);
        assert_eq!(op_assign!(Positive *= Zero), Zero);
        assert_eq!(op_assign!(Negative *= Zero), Zero);
        assert_eq!(op_assign!(Positive *= Positive), Positive);
        assert_eq!(op_assign!(Positive *= Negative), Negative);
        assert_eq!(op_assign!(Negative *= Positive), Negative);
        assert_eq!(op_assign!(Negative *= Negative), Positive);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn sign_div_sign() {
        assert_eq!(Zero / Positive, Zero);
        assert_eq!(Zero / Negative, Zero);
        assert_eq!(Zero / Zero, Zero);
        assert_eq!(Positive / Zero, Zero);
        assert_eq!(Negative / Zero, Zero);
        assert_eq!(Positive / Positive, Positive);
        assert_eq!(Positive / Negative, Negative);
        assert_eq!(Negative / Positive, Negative);
        assert_eq!(Negative / Negative, Positive);
    }

    #[test]
    fn sign_div_assign_sign() {
        assert_eq!(op_assign!(Zero /= Positive), Zero);
        assert_eq!(op_assign!(Zero /= Negative), Zero);
        assert_eq!(op_assign!(Zero /= Zero), Zero);
        assert_eq!(op_assign!(Positive /= Zero), Zero);
        assert_eq!(op_assign!(Negative /= Zero), Zero);
        assert_eq!(op_assign!(Positive /= Positive), Positive);
        assert_eq!(op_assign!(Positive /= Negative), Negative);
        assert_eq!(op_assign!(Negative /= Positive), Negative);
        assert_eq!(op_assign!(Negative /= Negative), Positive);
    }

    #[test]
    fn neg() {
        assert_eq!(-Positive, Negative);
        assert_eq!(-Negative, Positive);
        assert_eq!(-Zero, Zero);
    }

    #[test]
    fn not() {
        assert_eq!(!Positive, Negative);
        assert_eq!(!Negative, Positive);
        assert_eq!(!Zero, Zero);
    }

    #[test]
    fn negate() {
        assert_eq!(Positive.negate(), Negative);
        assert_eq!(Negative.negate(), Positive);
        assert_eq!(Zero.negate(), Zero);
    }

    #[test]
    fn is_positive() {
        assert!(Positive.is_positive());
        assert!(!Negative.is_positive());
        assert!(!Zero.is_positive());
    }

    #[test]
    fn is_negative() {
        assert!(!Positive.is_negative());
        assert!(Negative.is_negative());
        assert!(!Zero.is_negative());
    }

    #[test]
    fn is_zero() {
        assert!(!Positive.is_zero());
        assert!(!Negative.is_zero());
        assert!(Zero.is_zero());
    }
}
