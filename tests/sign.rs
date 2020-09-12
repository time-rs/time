#![allow(deprecated)]

use time::{Sign, Sign::*};

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
fn int_div_sign() {
    assert_eq!(2 / Positive, 2);
    assert_eq!(2 / Negative, -2);
    assert_eq!(2 / Zero, 0);
}

#[test]
fn int_mul_assign_sign() {
    let mut v = 2;
    v *= Positive;
    assert_eq!(v, 2);
    v *= Negative;
    assert_eq!(v, -2);
    v *= Zero;
    assert_eq!(v, 0);
}

#[test]
fn int_div_assign_sign() {
    let mut v = 2;
    v /= Positive;
    assert_eq!(v, 2);
    v /= Negative;
    assert_eq!(v, -2);
    v /= Zero;
    assert_eq!(v, 0);
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
