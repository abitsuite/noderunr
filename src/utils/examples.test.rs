// src/utils/examples.test.rs

use super::examples;

/**
 * by_ref returns the dereferenced value plus one.
 */
#[test]
fn by_ref_adds_one() {
    let val = 41;
    let result = examples::by_ref(&val);

    assert_eq!(result, 42, "by_ref(&41) should return 42");
    /* Original value is unchanged (passed by reference). */
    assert_eq!(val, 41, "Original value should be unchanged");
}

/**
 * by_ref works with zero.
 */
#[test]
fn by_ref_zero() {
    let val = 0;
    let result = examples::by_ref(&val);

    assert_eq!(result, 1, "by_ref(&0) should return 1");
}

/**
 * by_ref works with negative numbers.
 */
#[test]
fn by_ref_negative() {
    let val = -1;
    let result = examples::by_ref(&val);

    assert_eq!(result, 0, "by_ref(&-1) should return 0");
}

/**
 * modifies sets the mutable reference to 1.337.
 */
#[test]
fn modifies_sets_value() {
    let mut val = 99.9;
    examples::modifies(&mut val);

    assert!(
        (val - 1.337).abs() < f64::EPSILON,
        "modifies() should set value to 1.337, got {}",
        val
    );
}

/**
 * modifies overwrites zero.
 */
#[test]
fn modifies_from_zero() {
    let mut val = 0.0;
    examples::modifies(&mut val);

    assert!(
        (val - 1.337).abs() < f64::EPSILON,
        "modifies() should set 0.0 to 1.337, got {}",
        val
    );
}

/**
 * modifies overwrites negative.
 */
#[test]
fn modifies_from_negative() {
    let mut val = -500.0;
    examples::modifies(&mut val);

    assert!(
        (val - 1.337).abs() < f64::EPSILON,
        "modifies() should set -500.0 to 1.337, got {}",
        val
    );
}
