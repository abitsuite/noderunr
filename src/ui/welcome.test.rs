// src/ui/welcome.test.rs

use super::welcome;

/**
 * banner() does not panic.
 */
#[test]
fn banner_does_not_panic() {
    welcome::banner();
}

/**
 * banner_alt() does not panic.
 */
#[test]
fn banner_alt_does_not_panic() {
    welcome::banner_alt();
}

/**
 * banner_alt_2() does not panic.
 */
#[test]
fn banner_alt_2_does_not_panic() {
    welcome::banner_alt_2();
}
