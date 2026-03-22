// src/api.test.rs

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
 * build_url constructs the expected URL for a nested path.
 */
#[test]
fn build_url_nested_path() {
    let url = super::build_url("node/status");
    assert_eq!(url, "https://l1.run/v1/node/status");
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
    let url = super::build_url_with_base("http://localhost:1234/", "session");
    assert_eq!(url, "http://localhost:1234/session");
}

/**
 * build_url_with_base handles empty base.
 */
#[test]
fn build_url_with_base_empty_base() {
    let url = super::build_url_with_base("", "session");
    assert_eq!(url, "session");
}

/**
 * call_with_base_url posts JSON to mock server and returns response body.
 */
#[tokio::test]
async fn call_with_base_url_posts_json() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .match_header("Content-Type", "application/json")
        .match_body(r#"{"test":"data"}"#)
        .with_status(200)
        .with_body(r#"{"success":true}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::call_with_base_url(&base_url, "session", r#"{"test":"data"}"#).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"success":true}"#);
}

/**
 * call_with_base_url returns error body on non-200 status.
 */
#[tokio::test]
async fn call_with_base_url_non_200() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::call_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;
    /* reqwest does not error on 500 — it returns the body. */
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Internal Server Error");
}

/**
 * call_with_base_url returns Err on connection refused.
 */
#[tokio::test]
async fn call_with_base_url_connection_refused() {
    let result = super::call_with_base_url(
        "http://127.0.0.1:1/",
        "session",
        "{}",
    ).await;

    assert!(result.is_err());
}

/**
 * call_with_base_url sends empty body correctly.
 */
#[tokio::test]
async fn call_with_base_url_empty_body() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/endpoint")
        .match_body("")
        .with_status(200)
        .with_body("ok")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = super::call_with_base_url(&base_url, "endpoint", "").await;

    mock.assert_async().await;
    assert_eq!(result.unwrap(), "ok");
}
