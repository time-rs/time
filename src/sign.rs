use crate::shim::NumberExt;
use core::ops::{Div, DivAssign, Mul, MulAssign};
use Sign::{Negative, Positive, Unknown, Zero};

/// Contains the sign of a value: positive, negative, zero, or unknown.
///
/// For ease of use, `Sign` implements [`Mul`](core::ops::Mul) and
/// [`Div`](core::ops::Div) on all signed numeric types. Where the value is
/// `Unknown`, the sign of the value is left unchanged. `Sign`s can also be
/// multiplied and divided by another `Sign`, which follows the same rules as
/// real numbers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sign {
    /// A positive value.
    Positive,

    /// A negative value.
    Negative,

    /// A value that is exactly zero.
    Zero,

    /// A value with an unknown sign, likely `NaN`.
    Unknown,
}

impl Default for Sign {
    /// `Sign` defaults to `Unknown`.
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert_eq!(Sign::default(), Sign::Unknown);
    /// ```
    fn default() -> Self {
        Unknown
    }
}

macro_rules! sign_mul {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Mul<$type> for Sign {
                type Output = $type;

                /// Negate the sign of the provided number if `self == Sign::Negative`.
                fn mul(self, rhs: $type) -> Self::Output {
                    match self {
                        Positive | Unknown => rhs,
                        Negative => -rhs,
                        Zero => <$type>::zero(),
                    }
                }
            }

            impl Mul<Sign> for $type {
                type Output = Self;

                /// Negate the sign of the provided number if `rhs == Sign::Negative`.
                fn mul(self, rhs: Sign) -> Self::Output {
                    match rhs {
                        Positive | Unknown => self,
                        Negative => -self,
                        Zero => Self::zero(),
                    }
                }
            }

            impl MulAssign<Sign> for $type {
                /// Negate the sign of the provided number if `rhs == Sign::Negative`.
                fn mul_assign(&mut self, rhs: Sign) {
                    if rhs.is_negative() {
                        *self = -*self;
                    }
                }
            }

            impl Div<Sign> for $type {
                type Output = Self;

                /// Negate the sign of the provided number if `rhs == Sign::Negative`.
                fn div(self, rhs: Sign) -> Self::Output {
                    self * rhs
                }
            }

            impl DivAssign<Sign> for $type {
                /// Negate the sign of the provided number if `rhs == Sign::Negative`.
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

    /// Multiplying signs follows how signs interact with real numbers.
    ///
    /// - If either side is `Sign::Zero`, the result is `Sign::Zero`.
    /// - If either side is `Sign::Unknown`, the result is `Sign::Unknown`.
    /// - If the left and right are the same, the result is `Sign::Positive`.
    /// - Otherwise, the result is `Sign::Negative`.
    ///
    /// |          | Negative | Zero | Positive | Unknown |
    /// |----------|----------|------|----------|---------|
    /// | Negative | Positive | Zero | Negative | Unknown |
    /// | Zero     | Zero     | Zero | Zero     | Zero    |
    /// | Positive | Negative | Zero | Positive | Unknown |
    /// | Unknown  | Unknown  | Zero | Unknown  | Unknown |
    ///
    /// ```rust
    /// # use time::Sign::*;
    /// assert_eq!(Unknown * Positive, Unknown);
    /// assert_eq!(Unknown * Negative, Unknown);
    /// assert_eq!(Unknown * Zero, Zero);
    /// assert_eq!(Unknown * Unknown, Unknown);
    /// assert_eq!(Positive * Unknown, Unknown);
    /// assert_eq!(Negative * Unknown, Unknown);
    /// assert_eq!(Zero * Unknown, Zero);
    /// assert_eq!(Zero * Positive, Zero);
    /// assert_eq!(Zero * Negative, Zero);
    /// assert_eq!(Zero * Zero, Zero);
    /// assert_eq!(Positive * Zero, Zero);
    /// assert_eq!(Negative * Zero, Zero);
    /// assert_eq!(Positive * Positive, Positive);
    /// assert_eq!(Positive * Negative, Negative);
    /// assert_eq!(Negative * Positive, Negative);
    /// assert_eq!(Negative * Negative, Positive);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, _) | (_, Zero) => Zero,
            (Unknown, _) | (_, Unknown) => Unknown,
            (Positive, Positive) | (Negative, Negative) => Positive,
            (Positive, Negative) | (Negative, Positive) => Negative,
        }
    }
}

impl MulAssign<Sign> for Sign {
    /// Multiplying signs follows how signs interact with real numbers.
    ///
    /// ```rust
    /// # use time::Sign::*;
    /// let mut sign = Positive;
    /// sign *= Positive;
    /// assert_eq!(sign, Positive);
    /// sign *= Negative;
    /// assert_eq!(sign, Negative);
    /// sign *= Positive;
    /// assert_eq!(sign, Negative);
    /// sign *= Negative;
    /// assert_eq!(sign, Positive);
    /// sign *= Unknown;
    /// assert_eq!(sign, Unknown);
    /// sign *= Positive;
    /// assert_eq!(sign, Unknown);
    /// sign *= Negative;
    /// assert_eq!(sign, Unknown);
    /// sign *= Zero;
    /// assert_eq!(sign, Zero);
    /// sign *= Positive;
    /// assert_eq!(sign, Zero);
    /// sign *= Negative;
    /// assert_eq!(sign, Zero);
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<Sign> for Sign {
    type Output = Self;

    /// Dividing signs follows how signs interact with real numbers.
    ///
    /// |          | Negative | Zero | Positive | Unknown |
    /// |----------|----------|------|----------|---------|
    /// | Negative | Positive | Zero | Negative | Unknown |
    /// | Zero     | Zero     | Zero | Zero     | Zero    |
    /// | Positive | Negative | Zero | Positive | Unknown |
    /// | Unknown  | Unknown  | Zero | Unknown  | Unknown |
    /// ```rust
    /// # use time::Sign::*;
    /// assert_eq!(Unknown / Positive, Unknown);
    /// assert_eq!(Unknown / Negative, Unknown);
    /// assert_eq!(Unknown / Zero, Zero);
    /// assert_eq!(Unknown / Unknown, Unknown);
    /// assert_eq!(Positive / Unknown, Unknown);
    /// assert_eq!(Negative / Unknown, Unknown);
    /// assert_eq!(Zero / Unknown, Zero);
    /// assert_eq!(Zero / Positive, Zero);
    /// assert_eq!(Zero / Negative, Zero);
    /// assert_eq!(Zero / Zero, Zero);
    /// assert_eq!(Positive / Zero, Zero);
    /// assert_eq!(Negative / Zero, Zero);
    /// assert_eq!(Positive / Positive, Positive);
    /// assert_eq!(Positive / Negative, Negative);
    /// assert_eq!(Negative / Positive, Negative);
    /// assert_eq!(Negative / Negative, Positive);
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl DivAssign<Sign> for Sign {
    /// Dividing signs follows how signs interact with real numbers.
    ///
    /// ```rust
    /// # use time::Sign::*;
    /// let mut sign = Positive;
    /// sign /= Positive;
    /// assert_eq!(sign, Positive);
    /// sign /= Negative;
    /// assert_eq!(sign, Negative);
    /// sign /= Positive;
    /// assert_eq!(sign, Negative);
    /// sign /= Negative;
    /// assert_eq!(sign, Positive);
    /// sign /= Unknown;
    /// assert_eq!(sign, Unknown);
    /// sign /= Positive;
    /// assert_eq!(sign, Unknown);
    /// sign /= Negative;
    /// assert_eq!(sign, Unknown);
    /// sign /= Zero;
    /// assert_eq!(sign, Zero);
    /// sign /= Positive;
    /// assert_eq!(sign, Zero);
    /// sign /= Negative;
    /// assert_eq!(sign, Zero);
    /// ```
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs
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
    /// assert_eq!(Sign::Unknown.negate(), Sign::Unknown);
    /// ```
    pub fn negate(self) -> Self {
        match self {
            Positive => Negative,
            Negative => Positive,
            Zero => Zero,
            Unknown => Unknown,
        }
    }

    /// Is the sign positive?
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert!(Sign::Positive.is_positive());
    /// assert!(!Sign::Negative.is_positive());
    /// assert!(!Sign::Zero.is_positive());
    /// assert!(!Sign::Unknown.is_positive());
    /// ```
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
    /// assert!(!Sign::Unknown.is_negative());
    /// ```
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
    /// assert!(!Sign::Unknown.is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self as u8 == Zero as u8
    }

    /// Is the sign of the value unknown?
    ///
    /// ```rust
    /// # use time::Sign;
    /// assert!(!Sign::Positive.is_unknown());
    /// assert!(!Sign::Negative.is_unknown());
    /// assert!(!Sign::Zero.is_unknown());
    /// assert!(Sign::Unknown.is_unknown());
    /// ```
    pub const fn is_unknown(self) -> bool {
        self as u8 == Unknown as u8
    }
}
