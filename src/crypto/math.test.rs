// src/crypto/math.test.rs

use super::math;

/**
 * Square of Positive Number
 */
#[test]
fn sqr_positive() {
    let result = math::sqr(4.0);
    assert!((result - 16.0).abs() < f64::EPSILON);
}

/**
 * Square of Zero
 */
#[test]
fn sqr_zero() {
    let result = math::sqr(0.0);
    assert!((result - 0.0).abs() < f64::EPSILON);
}

/**
 * Square of Negative Number
 *
 * Squaring a negative should produce a positive result.
 */
#[test]
fn sqr_negative() {
    let result = math::sqr(-3.0);
    assert!((result - 9.0).abs() < f64::EPSILON);
}

/**
 * Square of Fractional Number
 */
#[test]
fn sqr_fractional() {
    let result = math::sqr(0.5);
    assert!((result - 0.25).abs() < f64::EPSILON);
}

/**
 * Square of One
 */
#[test]
fn sqr_one() {
    let result = math::sqr(1.0);
    assert!((result - 1.0).abs() < f64::EPSILON);
}

/**
 * Square of Large Number
 */
#[test]
fn sqr_large() {
    let result = math::sqr(1_000_000.0);
    assert!((result - 1_000_000_000_000.0).abs() < f64::EPSILON);
}

/**
 * Square of NaN should be NaN.
 */
#[test]
fn sqr_nan() {
    let result = math::sqr(f64::NAN);
    assert!(result.is_nan(), "sqr(NaN) should be NaN, got {}", result);
}

/**
 * Square of positive infinity should be positive infinity.
 */
#[test]
fn sqr_positive_infinity() {
    let result = math::sqr(f64::INFINITY);
    assert!(
        result.is_infinite() && result.is_sign_positive(),
        "sqr(INFINITY) should be +INFINITY, got {}",
        result
    );
}

/**
 * Square of negative infinity should be positive infinity.
 */
#[test]
fn sqr_negative_infinity() {
    let result = math::sqr(f64::NEG_INFINITY);
    assert!(
        result.is_infinite() && result.is_sign_positive(),
        "sqr(NEG_INFINITY) should be +INFINITY, got {}",
        result
    );
}

/**
 * Square of a very small number should not underflow to zero.
 */
#[test]
fn sqr_very_small() {
    let result = math::sqr(1e-100);
    assert!(
        (result - 1e-200).abs() < 1e-210,
        "sqr(1e-100) should be ~1e-200, got {}",
        result
    );
}
