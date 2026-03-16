// tests/lib_version.rs

/**
 * Library Version Format
 *
 * Verify that get_version() returns a properly formatted version string.
 */
#[test]
fn lib_version_format() {
    let version = noderunr::get_version();

    /* Must start with 'v'. */
    assert!(
        version.starts_with('v'),
        "Version '{}' does not start with 'v'",
        version
    );

    /* Must contain the alpha tag. */
    assert!(
        version.contains("(alpha)"),
        "Version '{}' does not contain '(alpha)'",
        version
    );
}

/**
 * Library Version Not Empty
 *
 * Verify that get_version() returns a non-empty string.
 */
#[test]
fn lib_version_not_empty() {
    let version = noderunr::get_version();

    assert!(!version.is_empty(), "Version string should not be empty");
}

/**
 * Library Version Contains Numeric
 *
 * Verify that get_version() contains at least one numeric character
 * (i.e., an actual version number is present).
 */
#[test]
fn lib_version_contains_numeric() {
    let version = noderunr::get_version();

    assert!(
        version.chars().any(|c| c.is_ascii_digit()),
        "Version '{}' does not contain any numeric characters",
        version
    );
}
