// src/utils/ip.test.rs

use crate::utils::ip;

/**
 * ip::get() returns a result (Ok or Err) without panicking.
 *
 * NOTE: This requires network access. On CI without network,
 * an Err is acceptable.
 */
#[tokio::test]
async fn get_ip_does_not_panic() {
    let result = ip::get().await;

    match result {
        Ok(map) => {
            /* httpbin.org/ip returns {"origin": "x.x.x.x"} */
            assert!(
                map.contains_key("origin"),
                "Response should contain 'origin' key, got: {:?}",
                map
            );

            let origin = &map["origin"];
            assert!(
                !origin.is_empty(),
                "Origin IP should not be empty"
            );
        }
        Err(e) => {
            /* Network may not be available — just verify it's a real error. */
            let msg = format!("{}", e);
            assert!(
                !msg.is_empty(),
                "Error message should not be empty"
            );
        }
    }
}

/**
 * ip::get_with_url returns parsed JSON from mock server.
 */
#[tokio::test]
async fn get_with_url_mock_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"origin":"9.8.7.6"}"#)
        .create_async()
        .await;

    let url = format!("{}/", server.url());
    let result = ip::get_with_url(&url).await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let map = result.unwrap();
    assert_eq!(map.get("origin").unwrap(), "9.8.7.6");
}

/**
 * ip::get_with_url returns Err on connection refused.
 */
#[tokio::test]
async fn get_with_url_connection_refused() {
    let result = ip::get_with_url("http://127.0.0.1:1/").await;
    assert!(result.is_err());
}

/**
 * ip::get_with_url returns Err on invalid JSON response.
 */
#[tokio::test]
async fn get_with_url_invalid_json() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body("not json")
        .create_async()
        .await;

    let url = format!("{}/", server.url());
    let result = ip::get_with_url(&url).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

/**
 * ip::get_with_url handles multiple keys in response.
 */
#[tokio::test]
async fn get_with_url_multiple_keys() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"origin":"1.2.3.4","extra":"data"}"#)
        .create_async()
        .await;

    let url = format!("{}/", server.url());
    let result = ip::get_with_url(&url).await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let map = result.unwrap();
    assert_eq!(map.get("origin").unwrap(), "1.2.3.4");
    assert_eq!(map.get("extra").unwrap(), "data");
}

/**
 * ip::get_with_url handles empty JSON object.
 */
#[tokio::test]
async fn get_with_url_empty_object() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{}"#)
        .create_async()
        .await;

    let url = format!("{}/", server.url());
    let result = ip::get_with_url(&url).await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.is_empty());
}
