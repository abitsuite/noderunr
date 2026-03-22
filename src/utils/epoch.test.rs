// src/utils/epoch.test.rs

use super::epoch;

use std::time::{SystemTime, UNIX_EPOCH};

/**
 * Epoch Seconds Sanity Check
 *
 * Verify that epoch::seconds() returns a reasonable timestamp.
 * The value must be greater than 1,700,000,000 (Nov 2023)
 * and within 2 seconds of our own measurement.
 */
#[test]
fn epoch_seconds_sanity() {
    let before = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let result = epoch::seconds();

    let after = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    /* Must be a reasonable modern timestamp. */
    assert!(
        result > 1_700_000_000,
        "Epoch seconds {} is unreasonably low",
        result
    );

    /* Must be within our before/after window. */
    assert!(
        result >= before && result <= after,
        "Epoch seconds {} not between {} and {}",
        result,
        before,
        after
    );
}

/**
 * Epoch seconds is below a sane ceiling (year 2100).
 */
#[test]
fn epoch_seconds_sane_ceiling() {
    let result = epoch::seconds();

    assert!(
        result < 4_102_444_800,
        "Epoch seconds {} is unreasonably high",
        result
    );
}

/**
 * Epoch Milliseconds Sanity Check
 *
 * Verify that epoch::milliseconds() returns a value roughly 1000x
 * greater than seconds().
 */
#[test]
fn epoch_milliseconds_sanity() {
    let secs = epoch::seconds();
    let millis = epoch::milliseconds();

    /* Millis must be approximately secs * 1000 (within 2 seconds tolerance). */
    assert!(
        millis >= secs * 1000,
        "Milliseconds {} should be >= seconds {} * 1000",
        millis,
        secs
    );

    assert!(
        millis < (secs + 2) * 1000,
        "Milliseconds {} is too far from seconds {}",
        millis,
        secs
    );
}

/**
 * Epoch Aliases
 *
 * Verify that now() and ms()/millis() are consistent aliases.
 */
#[test]
fn epoch_aliases_consistent() {
    let now = epoch::now();
    let secs = epoch::seconds();

    /* now() is an alias for seconds(). */
    assert!(
        now >= secs && now <= secs + 1,
        "now() {} should be within 1 second of seconds() {}",
        now,
        secs
    );

    let ms1 = epoch::ms();
    let ms2 = epoch::millis();

    /* ms() and millis() are both aliases for milliseconds(). */
    assert!(
        ms2 >= ms1 && ms2 - ms1 < 100,
        "ms() {} and millis() {} should be within 100ms of each other",
        ms1,
        ms2
    );
}

/**
 * Two successive calls to milliseconds() should be monotonically
 * non-decreasing.
 */
#[test]
fn epoch_milliseconds_monotonic() {
    let first = epoch::milliseconds();
    let second = epoch::milliseconds();

    assert!(
        second >= first,
        "Second call {} should be >= first call {}",
        second,
        first
    );
}
