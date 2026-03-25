// tests/api_endpoints.rs

//! Server endpoint smoke tests against https://l1.run/v1
//!
//! These tests verify that every documented API endpoint responds
//! with expected status codes. They hit the LIVE server.
//!
//! Run with: cargo test --test api_endpoints -- --ignored

// ---------------------------------------------------------------
// Activity endpoints
// ---------------------------------------------------------------

/**
 * GET /activity returns a response (may be 401 without auth).
 */
#[tokio::test]
#[ignore]
async fn live_get_activity_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/activity")
        .send()
        .await
        .expect("Failed to reach GET /activity");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403 || status == 404,
        "GET /activity unexpected status: {}",
        status
    );
}

/**
 * GET /activity/{ActivityID} returns a response for a fake ID.
 */
#[tokio::test]
#[ignore]
async fn live_get_activity_by_id_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/activity/nonexistent-id")
        .send()
        .await
        .expect("Failed to reach GET /activity/nonexistent-id");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 404,
        "GET /activity/nonexistent-id unexpected status: {}",
        status
    );
}

// ---------------------------------------------------------------
// Command endpoints
// ---------------------------------------------------------------

/**
 * GET /command/request returns a response.
 */
#[tokio::test]
#[ignore]
async fn live_get_command_request_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/command/request")
        .send()
        .await
        .expect("Failed to reach GET /command/request");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403,
        "GET /command/request unexpected status: {}",
        status
    );
}

/**
 * GET /command/response returns a response.
 */
#[tokio::test]
#[ignore]
async fn live_get_command_response_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/command/response")
        .send()
        .await
        .expect("Failed to reach GET /command/response");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403,
        "GET /command/response unexpected status: {}",
        status
    );
}

/**
 * GET /command/{CommandID} returns a response for a fake ID.
 */
#[tokio::test]
#[ignore]
async fn live_get_command_by_id_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/command/fake-command-id")
        .send()
        .await
        .expect("Failed to reach GET /command/fake-command-id");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 404,
        "GET /command/fake-command-id unexpected status: {}",
        status
    );
}

// ---------------------------------------------------------------
// Log endpoints
// ---------------------------------------------------------------

/**
 * GET /log returns a response.
 */
#[tokio::test]
#[ignore]
async fn live_get_log_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/log")
        .send()
        .await
        .expect("Failed to reach GET /log");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403 || status == 404,
        "GET /log unexpected status: {}",
        status
    );
}

/**
 * GET /log/{LogID} returns a response for a fake ID.
 */
#[tokio::test]
#[ignore]
async fn live_get_log_by_id_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/log/nonexistent-log-id")
        .send()
        .await
        .expect("Failed to reach GET /log/nonexistent-log-id");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 404,
        "GET /log/nonexistent-log-id unexpected status: {}",
        status
    );
}

// ---------------------------------------------------------------
// Node endpoints
// ---------------------------------------------------------------

/**
 * GET /node returns a response.
 */
#[tokio::test]
#[ignore]
async fn live_get_node_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/node")
        .send()
        .await
        .expect("Failed to reach GET /node");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403,
        "GET /node unexpected status: {}",
        status
    );
}

/**
 * GET /node/{NodeID} returns a response for a fake ID.
 */
#[tokio::test]
#[ignore]
async fn live_get_node_by_id_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/node/nonexistent-node-id")
        .send()
        .await
        .expect("Failed to reach GET /node/nonexistent-node-id");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 404,
        "GET /node/nonexistent-node-id unexpected status: {}",
        status
    );
}

// ---------------------------------------------------------------
// Session endpoints
// ---------------------------------------------------------------

/**
 * GET /session/{Since} returns a response.
 */
#[tokio::test]
#[ignore]
async fn live_get_session_since_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/session/1")
        .send()
        .await
        .expect("Failed to reach GET /session/1");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 401 || status == 403 || status == 404,
        "GET /session/1 unexpected status: {}",
        status
    );
}

/**
 * POST /session with empty body returns a response (not a network error).
 */
#[tokio::test]
#[ignore]
async fn live_post_session_empty_body_responds() {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://l1.run/v1/session")
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("Failed to reach POST /session");

    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 400 || status == 401 || status == 422,
        "POST /session with empty body unexpected status: {}",
        status
    );
}

/**
 * The Swagger UI page is reachable.
 */
#[tokio::test]
#[ignore]
async fn live_swagger_ui_is_reachable() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/")
        .send()
        .await
        .expect("Failed to reach Swagger UI");

    assert_eq!(
        resp.status().as_u16(),
        200,
        "Swagger UI should return 200"
    );

    let body = resp.text().await.unwrap();
    assert!(
        body.contains("swagger-ui") || body.contains("SwaggerUI"),
        "Swagger UI page should contain swagger-ui reference"
    );
}

// ---------------------------------------------------------------
// Response format consistency
// ---------------------------------------------------------------

/**
 * All GET endpoints return JSON content-type (when they return 200).
 */
#[tokio::test]
#[ignore]
async fn live_get_endpoints_return_json_content_type() {
    let client = reqwest::Client::new();

    let endpoints = vec![
        "https://l1.run/v1/activity",
        "https://l1.run/v1/command/request",
        "https://l1.run/v1/command/response",
        "https://l1.run/v1/log",
        "https://l1.run/v1/node",
        "https://l1.run/v1/session/1",
    ];

    for url in &endpoints {
        let resp = client.get(*url).send().await;

        match resp {
            Ok(r) => {
                if r.status().as_u16() == 200 {
                    let content_type = r
                        .headers()
                        .get("content-type")
                        .map(|v| v.to_str().unwrap_or(""))
                        .unwrap_or("");

                    assert!(
                        content_type.contains("application/json"),
                        "GET {} returned 200 but content-type is '{}', expected application/json",
                        url,
                        content_type
                    );
                }
            }
            Err(e) => {
                panic!("Failed to reach {}: {}", url, e);
            }
        }
    }
}
