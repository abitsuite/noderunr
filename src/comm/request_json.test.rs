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

/**
 * build_url_with_base uses custom base URL.
 */
#[test]
fn build_url_with_base_custom() {
    let url = super::build_url_with_base("http://localhost:9999/", "data");
    assert_eq!(url, "http://localhost:9999/data");
}

/**
 * request_json_with_base_url posts JSON to mock server and returns response body.
 */
#[tokio::test]
async fn request_json_with_base_url_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .match_header("Content-Type", "application/json")
        .match_body(r#"{"key":"value"}"#)
        .with_status(200)
        .with_body(r#"{"result":"ok"}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::request_json_with_base_url(
        &base_url,
        "session",
        r#"{"key":"value"}"#,
    ).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"result":"ok"}"#);
}

/**
 * request_json_with_base_url returns Err on connection refused.
 */
#[tokio::test]
async fn request_json_with_base_url_connection_refused() {
    let result = super::request_json_with_base_url(
        "http://127.0.0.1:1/",
        "session",
        "{}",
    ).await;

    assert!(result.is_err());
}

/**
 * request_json_with_base_url handles 500 response.
 */
#[tokio::test]
async fn request_json_with_base_url_server_error() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .with_status(500)
        .with_body("error")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::request_json_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "error");
}

/**
 * request_json_with_base_url handles empty response body.
 */
#[tokio::test]
async fn request_json_with_base_url_empty_response() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/data")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::request_json_with_base_url(&base_url, "data", "{}").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}
