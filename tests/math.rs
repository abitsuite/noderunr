// tests/math.rs

/**
 * Square of Positive Number
 */
#[test]
fn sqr_positive() {
    let result = noderunr::crypto::math::sqr(4.0);
    assert!((result - 16.0).abs() < f64::EPSILON);
}

/**
 * Square of Zero
 */
#[test]
fn sqr_zero() {
    let result = noderunr::crypto::math::sqr(0.0);
    assert!((result - 0.0).abs() < f64::EPSILON);
}

/**
 * Square of Negative Number
 *
 * Squaring a negative should produce a positive result.
 */
#[test]
fn sqr_negative() {
    let result = noderunr::crypto::math::sqr(-3.0);
    assert!((result - 9.0).abs() < f64::EPSILON);
}

/**
 * Square of Fractional Number
 */
#[test]
fn sqr_fractional() {
    let result = noderunr::crypto::math::sqr(0.5);
    assert!((result - 0.25).abs() < f64::EPSILON);
}

/**
 * Square of One
 */
#[test]
fn sqr_one() {
    let result = noderunr::crypto::math::sqr(1.0);
    assert!((result - 1.0).abs() < f64::EPSILON);
}

/**
 * Square of Large Number
 */
#[test]
fn sqr_large() {
    let result = noderunr::crypto::math::sqr(1_000_000.0);
    assert!((result - 1_000_000_000_000.0).abs() < f64::EPSILON);
}
