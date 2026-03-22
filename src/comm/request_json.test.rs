// src/comm/request_json.test.rs

/**
 * L1_ENDPOINT constant is a valid HTTPS URL.
 */
#[test]
fn l1_endpoint_is_https() {
    let endpoint = super::L1_ENDPOINT;

    assert!(
        endpoint.starts_with("https://"),
        "L1_ENDPOINT should start with https://, got: {}",
        endpoint
    );
}

/**
 * L1_ENDPOINT ends with a trailing slash for path joining.
 */
#[test]
fn l1_endpoint_has_trailing_slash() {
    let endpoint = super::L1_ENDPOINT;

    assert!(
        endpoint.ends_with('/'),
        "L1_ENDPOINT should end with '/', got: {}",
        endpoint
    );
}

/**
 * URL construction matches expected format.
 */
#[test]
fn url_construction_format() {
    let endpoint = "session";
    let url = format!("{}{}", super::L1_ENDPOINT, endpoint);

    assert_eq!(url, "https://l1.run/v1/session");
}

/**
 * build_url constructs the expected URL for "session".
 */
#[test]
fn build_url_session() {
    let url = super::build_url("session");
    assert_eq!(url, "https://l1.run/v1/session");
}

/**
 * build_url handles empty endpoint.
 */
#[test]
fn build_url_empty() {
    let url = super::build_url("");
    assert_eq!(url, "https://l1.run/v1/");
}
