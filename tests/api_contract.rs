// tests/api_contract.rs

//! API contract tests for NodΞRunr against https://l1.run/v1
//!
//! These tests verify:
//! 1. The live OpenAPI spec is reachable and contains expected endpoints
//! 2. The session registration flow works against a mock server
//! 3. HTTP status code behavior is documented and regression-proof
//! 4. CORS headers are present on the live server
//!
//! Tests marked #[ignore] hit the live server and should be run with:
//!   cargo test -- --ignored

// ---------------------------------------------------------------
// Live OpenAPI spec contract tests (require network)
// ---------------------------------------------------------------

/**
 * The /v1/openapi.json endpoint is reachable and returns valid JSON.
 */
#[tokio::test]
#[ignore]
async fn live_openapi_spec_is_reachable() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/openapi.json")
        .send()
        .await
        .expect("Failed to reach openapi.json");

    assert_eq!(
        resp.status().as_u16(),
        200,
        "openapi.json should return 200, got: {}",
        resp.status()
    );

    let body = resp.text().await.unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&body).expect("openapi.json should be valid JSON");

    assert!(
        json.get("paths").is_some(),
        "OpenAPI spec should contain 'paths' key"
    );
}

/**
 * The OpenAPI spec contains all expected endpoint paths.
 */
#[tokio::test]
#[ignore]
async fn live_openapi_spec_contains_expected_paths() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/openapi.json")
        .send()
        .await
        .expect("Failed to reach openapi.json");

    let body = resp.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let paths = json.get("paths").expect("Missing 'paths' in spec");

    let expected_paths = vec![
        "/activity",
        "/activity/{ActivityID}",
        "/command",
        "/command/request",
        "/command/response",
        "/command/{CommandID}",
        "/log",
        "/log/{LogID}",
        "/node",
        "/node/{NodeID}",
        "/session",
        "/session/{Since}",
    ];

    for path in &expected_paths {
        assert!(
            paths.get(*path).is_some(),
            "OpenAPI spec missing expected path: {}. Available paths: {:?}",
            path,
            paths.as_object().map(|o| o.keys().collect::<Vec<_>>())
        );
    }
}

/**
 * The OpenAPI spec declares expected HTTP methods for /session.
 */
#[tokio::test]
#[ignore]
async fn live_openapi_spec_session_methods() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/openapi.json")
        .send()
        .await
        .expect("Failed to reach openapi.json");

    let body = resp.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let paths = json.get("paths").unwrap();

    let session_path = paths.get("/session").expect("Missing /session path");
    assert!(
        session_path.get("post").is_some(),
        "/session should support POST"
    );

    let session_since = paths
        .get("/session/{Since}")
        .expect("Missing /session/{Since} path");
    assert!(
        session_since.get("get").is_some(),
        "/session/{{Since}} should support GET"
    );
}

/**
 * The live server returns CORS headers on a preflight request.
 */
#[tokio::test]
#[ignore]
async fn live_server_returns_cors_headers() {
    let client = reqwest::Client::new();
    let resp = client
        .request(reqwest::Method::OPTIONS, "https://l1.run/v1/session")
        .header("Origin", "https://example.com")
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .expect("Failed to send OPTIONS request");

    let headers = resp.headers();

    /* The server should return at least one CORS header. */
    let has_cors = headers.contains_key("access-control-allow-origin")
        || headers.contains_key("access-control-allow-methods")
        || headers.contains_key("access-control-allow-headers");

    assert!(
        has_cors,
        "Server should return CORS headers. Got headers: {:?}",
        headers
    );
}

/**
 * The live server returns CORS headers on a normal GET request.
 */
#[tokio::test]
#[ignore]
async fn live_server_get_returns_cors_origin() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://l1.run/v1/session/1")
        .header("Origin", "https://example.com")
        .send()
        .await
        .expect("Failed to send GET request");

    let acao = resp.headers().get("access-control-allow-origin");

    assert!(
        acao.is_some(),
        "GET /session/1 should return access-control-allow-origin header"
    );
}

// ---------------------------------------------------------------
// Session registration contract tests (mockito — no network)
// ---------------------------------------------------------------

/**
 * POST /session with a valid registration body returns a session response.
 */
#[tokio::test]
async fn mock_session_registration_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/session")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            r#"{
            "success": true,
            "result": {
                "sessionid": "test-session-id-12345",
                "hasAuth": false,
                "createdAt": 1700000000
            }
        }"#,
        )
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());

    let registration_json = serde_json::json!({
        "method": "reg",
        "ip": "1.2.3.4",
        "release": "Ubuntu 22.04",
        "uptime": "5 days",
        "cpu": "x86_64",
        "mem": "16GB",
        "profile": "linux"
    })
    .to_string();

    let result = noderunr::api::call_with_base_url(&base_url, "session", &registration_json).await;

    mock.assert_async().await;
    assert!(result.is_ok(), "Registration call should succeed");

    let body = result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["result"]["sessionid"], "test-session-id-12345");
    assert_eq!(parsed["result"]["hasAuth"], false);
    assert!(parsed["result"]["createdAt"].is_number());
}

/**
 * POST /session returns success=false when the server rejects registration.
 */
#[tokio::test]
async fn mock_session_registration_rejected() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/session")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            r#"{
            "success": false,
            "result": {
                "sessionid": "",
                "hasAuth": false,
                "createdAt": 0
            }
        }"#,
        )
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::api::call_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let body = result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(parsed["success"], false);
    assert_eq!(parsed["result"]["sessionid"], "");
}

/**
 * POST /session — server returns 500 — current client still returns Ok with body.
 * This documents the current behavior where HTTP status is NOT checked.
 * If someone adds status validation, this test MUST be updated.
 */
#[tokio::test]
async fn mock_session_registration_server_error_passes_through() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/session")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::api::call_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;

    /* CURRENT BEHAVIOR: reqwest does not error on 500 — body is returned as Ok. */
    /* If HTTP status checking is added (the TODO in api.rs), this test should */
    /* be updated to expect Err or a specific error type. */
    assert!(
        result.is_ok(),
        "Current client passes through 500 as Ok. \
         If this fails, someone added status checking — update this test."
    );
    assert_eq!(result.unwrap(), "Internal Server Error");
}

/**
 * POST /session — server returns 401 — current client still returns Ok with body.
 */
#[tokio::test]
async fn mock_session_registration_401_passes_through() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/session")
        .with_status(401)
        .with_body(r#"{"error":"Unauthorized"}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::api::call_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;

    assert!(
        result.is_ok(),
        "Current client passes through 401 as Ok. \
         If this fails, someone added status checking — update this test."
    );

    let body = result.unwrap();
    assert!(body.contains("Unauthorized"));
}

/**
 * POST /session — server returns 429 (rate limited) — passes through.
 */
#[tokio::test]
async fn mock_session_registration_429_rate_limited() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/session")
        .with_status(429)
        .with_body(r#"{"error":"Too Many Requests"}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::api::call_with_base_url(&base_url, "session", "{}").await;

    mock.assert_async().await;

    assert!(
        result.is_ok(),
        "Current client passes through 429 as Ok. \
         If this fails, someone added status checking — update this test."
    );
}

// ---------------------------------------------------------------
// Session polling contract tests (GET /session/{Since})
// ---------------------------------------------------------------

/**
 * GET /session/{Since} with valid auth returns session data.
 */
#[tokio::test]
async fn mock_session_poll_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/session/1700000000000")
        .match_header("Authorization", "Bearer test-session-abc")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            r#"{
            "success": true,
            "result": {
                "sessionid": "test-session-abc",
                "act": null,
                "log": null,
                "req": [{"commandid": "cmd-001", "exec": "uname", "created_at": 1700000000001}],
                "res": null,
                "rpt": null,
                "created_at": 1700000,
                "last_since": 1700000000002
            }
        }"#,
        )
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::comm::monitor::request_json_async_with_base(
        &base_url,
        "test-session-abc",
        1700000000000,
    )
    .await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let body = result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["result"]["sessionid"], "test-session-abc");
    assert_eq!(parsed["result"]["last_since"], 1700000000002u64);

    let req_array = parsed["result"]["req"].as_array().unwrap();
    assert_eq!(req_array.len(), 1);
    assert_eq!(req_array[0]["exec"], "uname");
}

/**
 * GET /session/{Since} with no pending commands returns empty req.
 */
#[tokio::test]
async fn mock_session_poll_no_commands() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/session/0")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            r#"{
            "success": true,
            "result": {
                "sessionid": "idle-session",
                "act": null,
                "log": null,
                "req": null,
                "res": null,
                "rpt": null,
                "created_at": 1700000,
                "last_since": 1700000000100
            }
        }"#,
        )
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result =
        noderunr::comm::monitor::request_json_async_with_base(&base_url, "idle-session", 0).await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let body = result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert!(
        parsed["result"]["req"].is_null(),
        "req should be null when no commands are pending"
    );
}

/**
 * POST /command (exec response) sends correctly structured JSON.
 */
#[tokio::test]
async fn mock_session_exec_response_body() {
    let mut server = mockito::Server::new_async().await;

    let expected_json =
        noderunr::comm::monitor::build_exec_response_json("cmd-resp-001", "Linux 5.15.0 x86_64");

    let mock = server
        .mock("POST", "/command")
        .match_header("Authorization", "Bearer resp-session-id")
        .match_header("Content-Type", "application/json")
        .match_body(expected_json.as_str())
        .with_status(200)
        .with_body(r#"{"success":true}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = noderunr::comm::monitor::response_json_async_with_base(
        &base_url,
        "resp-session-id",
        "cmd-resp-001",
        "Linux 5.15.0 x86_64".to_string(),
    )
    .await;

    mock.assert_async().await;
    assert!(result.is_ok());
}

/**
 * POST /command (exec response) — the JSON body has method "res".
 */
#[tokio::test]
async fn mock_session_exec_response_method_is_res() {
    let json_str = noderunr::comm::monitor::build_exec_response_json("cmd-any", "any output");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(
        parsed["method"], "res",
        "Exec response method should be 'res'"
    );
    assert_eq!(parsed["commandid"], "cmd-any");
    assert_eq!(parsed["response"], "any output");
}

// ---------------------------------------------------------------
// Full request/response cycle (mock both GET and POST)
// ---------------------------------------------------------------

/**
 * Simulate a full poll → exec → respond cycle against mock server.
 * This is the closest we can get to testing by_session without refactoring.
 */
#[tokio::test]
async fn mock_full_poll_exec_respond_cycle() {
    let mut server = mockito::Server::new_async().await;

    /* Step 1: Poll returns a "help" command. */
    let poll_mock = server
        .mock("GET", "/session/1")
        .match_header("Authorization", "Bearer cycle-sess")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "result": {
                "sessionid": "cycle-sess",
                "act": null,
                "log": null,
                "req": [{"commandid": "cmd-cycle-001", "exec": "help", "created_at": 100}],
                "res": null,
                "rpt": null,
                "created_at": 50,
                "last_since": 200
            }
        }"#,
        )
        .create_async()
        .await;

    /* Step 2: After executing, the client posts the response to /command. */
    let respond_mock = server
        .mock("POST", "/command")
        .match_header("Authorization", "Bearer cycle-sess")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_body(r#"{"success":true}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());

    /* Poll. */
    let poll_result =
        noderunr::comm::monitor::request_json_async_with_base(&base_url, "cycle-sess", 1).await;

    poll_mock.assert_async().await;
    assert!(poll_result.is_ok());

    /* Parse the poll response. */
    let poll_body = poll_result.unwrap();
    let poll_json: noderunr::comm::monitor::SessionResponse =
        serde_json::from_str(&poll_body).unwrap();

    assert!(poll_json.success);
    assert_eq!(poll_json.result.last_since, 200);

    /* Execute: resolve_exec on the pending request. */
    let req_list = poll_json.result.req.unwrap();
    let exec_result = noderunr::comm::monitor::resolve_exec(&req_list);
    assert!(exec_result.is_some());

    let exec_output = exec_result.unwrap();
    assert!(exec_output.contains("Help is temporarily unavailable"));

    /* Extract the command ID from the pending request. */
    let commandid = req_list[0].commandid.as_deref().unwrap_or("unknown");

    /* Respond. */
    let respond_result = noderunr::comm::monitor::response_json_async_with_base(
        &base_url,
        "cycle-sess",
        commandid,
        exec_output,
    )
    .await;

    respond_mock.assert_async().await;
    assert!(respond_result.is_ok());
}
