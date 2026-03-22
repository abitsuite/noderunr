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
    let endpoint = "nodes";
    let json_body = r#"{"id": "test"}"#;
    let url = format!("{}{}", super::L1_ENDPOINT, endpoint);

    assert_eq!(url, "https://l1.run/v1/nodes");

    /* Verify the json body can be converted to a String for the request. */
    let body = json_body.to_string();
    assert!(body.contains("test"));
}
