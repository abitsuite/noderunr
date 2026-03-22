// src/comm/monitor.test.rs

use super::*;
use serde_json::{from_str, to_string};

/**
 * L1_ENDPOINT constant is a valid HTTPS URL.
 */
#[test]
fn l1_endpoint_is_https() {
    assert!(
        L1_ENDPOINT.starts_with("https://"),
        "L1_ENDPOINT should start with https://, got: {}",
        L1_ENDPOINT
    );
}

/**
 * L1_ENDPOINT ends with a trailing slash for path joining.
 */
#[test]
fn l1_endpoint_has_trailing_slash() {
    assert!(
        L1_ENDPOINT.ends_with('/'),
        "L1_ENDPOINT should end with '/', got: {}",
        L1_ENDPOINT
    );
}

/**
 * build_request_url constructs the expected URL.
 */
#[test]
fn build_request_url_format() {
    let url = build_request_url(1234567890);
    assert_eq!(url, "https://l1.run/v1/session/1234567890");
}

/**
 * build_request_url handles zero.
 */
#[test]
fn build_request_url_zero() {
    let url = build_request_url(0);
    assert_eq!(url, "https://l1.run/v1/session/0");
}

/**
 * build_request_url handles max u64.
 */
#[test]
fn build_request_url_max() {
    let url = build_request_url(u64::MAX);
    assert!(url.contains("18446744073709551615"));
}

/**
 * build_request_url_with_base uses custom base URL.
 */
#[test]
fn build_request_url_with_base_custom() {
    let url = build_request_url_with_base("http://localhost:1234/", 42);
    assert_eq!(url, "http://localhost:1234/session/42");
}

/**
 * build_response_url_with_base uses custom base URL.
 */
#[test]
fn build_response_url_with_base_custom() {
    let url = build_response_url_with_base("http://localhost:5678/");
    assert_eq!(url, "http://localhost:5678/session");
}

/**
 * build_auth_header constructs the expected format.
 */
#[test]
fn build_auth_header_format() {
    let auth = build_auth_header("test-session-abc");
    assert_eq!(auth, "Bearer test-session-abc");
}

/**
 * build_auth_header handles empty session id.
 */
#[test]
fn build_auth_header_empty() {
    let auth = build_auth_header("");
    assert_eq!(auth, "Bearer ");
}

/**
 * build_response_url constructs the expected URL.
 */
#[test]
fn build_response_url_format() {
    let url = build_response_url();
    assert_eq!(url, "https://l1.run/v1/session");
}

/**
 * build_exec_response_json produces valid JSON.
 */
#[test]
fn build_exec_response_json_valid() {
    let json_str = build_exec_response_json("sess-123", "output data");
    let parsed: ExecResponse = from_str(&json_str).unwrap();

    assert_eq!(parsed.sessionid, "sess-123");
    assert_eq!(parsed.method, "res");
    assert_eq!(parsed.resp, "output data");
}

/**
 * build_exec_response_json handles empty strings.
 */
#[test]
fn build_exec_response_json_empty() {
    let json_str = build_exec_response_json("", "");
    let parsed: ExecResponse = from_str(&json_str).unwrap();

    assert_eq!(parsed.sessionid, "");
    assert_eq!(parsed.resp, "");
}

/**
 * Action struct can be serialized and deserialized.
 */
#[test]
fn action_serde_roundtrip() {
    let action = Action {
        actionid: Some("act-001".to_string()),
        body: Some("test body".to_string()),
        target: Some("node-1".to_string()),
        created_at: 1700000000000,
    };

    let json = to_string(&action).unwrap();
    let parsed: Action = from_str(&json).unwrap();

    assert_eq!(parsed.actionid.unwrap(), "act-001");
    assert_eq!(parsed.body.unwrap(), "test body");
    assert_eq!(parsed.target.unwrap(), "node-1");
    assert_eq!(parsed.created_at, 1700000000000);
}

/**
 * Action struct handles None fields.
 */
#[test]
fn action_serde_with_none_fields() {
    let action = Action {
        actionid: None,
        body: None,
        target: None,
        created_at: 0,
    };

    let json = to_string(&action).unwrap();
    let parsed: Action = from_str(&json).unwrap();

    assert!(parsed.actionid.is_none());
    assert!(parsed.body.is_none());
    assert!(parsed.target.is_none());
    assert_eq!(parsed.created_at, 0);
}

/**
 * ExecResponse struct serializes correctly.
 */
#[test]
fn exec_response_serde_roundtrip() {
    let resp = ExecResponse {
        sessionid: "sess-123".to_string(),
        method: "res".to_string(),
        resp: "some output".to_string(),
    };

    let json = to_string(&resp).unwrap();
    assert!(json.contains("sess-123"));
    assert!(json.contains("\"method\":\"res\""));

    let parsed: ExecResponse = from_str(&json).unwrap();
    assert_eq!(parsed.sessionid, "sess-123");
    assert_eq!(parsed.method, "res");
    assert_eq!(parsed.resp, "some output");
}

/**
 * Log struct can be serialized and deserialized.
 */
#[test]
fn log_serde_roundtrip() {
    let log = Log {
        body: "log entry here".to_string(),
        created_at: 1700000001000,
    };

    let json = to_string(&log).unwrap();
    let parsed: Log = from_str(&json).unwrap();

    assert_eq!(parsed.body, "log entry here");
    assert_eq!(parsed.created_at, 1700000001000);
}

/**
 * Request struct can be serialized and deserialized.
 */
#[test]
fn request_serde_roundtrip() {
    let req = Request {
        exec: "df".to_string(),
        created_at: 1700000002000,
    };

    let json = to_string(&req).unwrap();
    let parsed: Request = from_str(&json).unwrap();

    assert_eq!(parsed.exec, "df");
    assert_eq!(parsed.created_at, 1700000002000);
}

/**
 * Session struct default has empty/zero values.
 */
#[test]
fn session_default() {
    let session = Session::default();

    assert_eq!(session.sessionid, "");
    assert!(session.act.is_none());
    assert!(session.log.is_none());
    assert!(session.req.is_none());
    assert!(session.res.is_none());
    assert!(session.rpt.is_none());
    assert_eq!(session.created_at, 0);
    assert_eq!(session.last_since, 0);
}

/**
 * Session struct full serde roundtrip.
 */
#[test]
fn session_serde_roundtrip() {
    let session = Session {
        sessionid: "sess-full".to_string(),
        act: Some(vec![Action {
            actionid: Some("a1".to_string()),
            body: None,
            target: None,
            created_at: 100,
        }]),
        log: None,
        req: Some(vec![Request {
            exec: "uname".to_string(),
            created_at: 200,
        }]),
        res: None,
        rpt: None,
        created_at: 1000,
        last_since: 2000,
    };

    let json = to_string(&session).unwrap();
    let parsed: Session = from_str(&json).unwrap();

    assert_eq!(parsed.sessionid, "sess-full");
    assert_eq!(parsed.act.as_ref().unwrap().len(), 1);
    assert_eq!(parsed.req.as_ref().unwrap()[0].exec, "uname");
    assert_eq!(parsed.created_at, 1000);
    assert_eq!(parsed.last_since, 2000);
}

/**
 * SessionRequest struct serializes correctly.
 */
#[test]
fn session_request_serde_roundtrip() {
    let req = SessionRequest {
        sessionid: "sr-001".to_string(),
        since: 9999999,
    };

    let json = to_string(&req).unwrap();
    let parsed: SessionRequest = from_str(&json).unwrap();

    assert_eq!(parsed.sessionid, "sr-001");
    assert_eq!(parsed.since, 9999999);
}

/**
 * SessionResponse struct default has expected values.
 */
#[test]
fn session_response_default() {
    let resp = SessionResponse::default();

    assert!(!resp.success);
    assert_eq!(resp.result.sessionid, "");
    assert_eq!(resp.result.last_since, 0);
}

/**
 * SessionResponse struct full serde roundtrip.
 */
#[test]
fn session_response_serde_roundtrip() {
    let resp = SessionResponse {
        success: true,
        result: Session {
            sessionid: "resp-sess".to_string(),
            act: None,
            log: None,
            req: None,
            res: None,
            rpt: None,
            created_at: 500,
            last_since: 600,
        },
    };

    let json = to_string(&resp).unwrap();
    let parsed: SessionResponse = from_str(&json).unwrap();

    assert!(parsed.success);
    assert_eq!(parsed.result.sessionid, "resp-sess");
    assert_eq!(parsed.result.created_at, 500);
    assert_eq!(parsed.result.last_since, 600);
}

/**
 * SessionResponse deserializes from realistic JSON.
 */
#[test]
fn session_response_from_realistic_json() {
    let json_str = r#"{
        "success": true,
        "result": {
            "sessionid": "abc-def-123",
            "act": null,
            "log": null,
            "req": [{"exec": "ps", "created_at": 1700000000000}],
            "res": null,
            "rpt": null,
            "created_at": 1700000,
            "last_since": 1700000000001
        }
    }"#;

    let parsed: SessionResponse = from_str(json_str).unwrap();

    assert!(parsed.success);
    assert_eq!(parsed.result.sessionid, "abc-def-123");
    assert_eq!(parsed.result.req.as_ref().unwrap().len(), 1);
    assert_eq!(parsed.result.req.as_ref().unwrap()[0].exec, "ps");
    assert_eq!(parsed.result.last_since, 1700000000001);
}

/**
 * ExecResponse JSON string output matches expected structure.
 */
#[test]
fn exec_response_json_string_format() {
    let exec_response = ExecResponse {
        sessionid: "test-sid".to_string(),
        method: "res".to_string(),
        resp: "command output here".to_string(),
    };

    let json_string = to_string(&exec_response).unwrap();

    assert!(json_string.contains("\"sessionid\":\"test-sid\""));
    assert!(json_string.contains("\"method\":\"res\""));
    assert!(json_string.contains("\"resp\":\"command output here\""));
}

/**
 * resolve_exec returns None for empty request list.
 */
#[test]
fn resolve_exec_empty_returns_none() {
    let result = resolve_exec(&[]);
    assert!(result.is_none());
}

/**
 * resolve_exec handles "df" command.
 */
#[test]
fn resolve_exec_df() {
    let reqs = vec![Request { exec: "df".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "du" command.
 */
#[test]
fn resolve_exec_du() {
    let reqs = vec![Request { exec: "du".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "ls" command.
 */
#[test]
fn resolve_exec_ls() {
    let reqs = vec![Request { exec: "ls".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "lsblk" command.
 */
#[test]
fn resolve_exec_lsblk() {
    let reqs = vec![Request { exec: "lsblk".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "lscpu" command.
 */
#[test]
fn resolve_exec_lscpu() {
    let reqs = vec![Request { exec: "lscpu".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "lshw" command.
 */
#[test]
fn resolve_exec_lshw() {
    let reqs = vec![Request { exec: "lshw".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "mem" command.
 */
#[test]
fn resolve_exec_mem() {
    let reqs = vec![Request { exec: "mem".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "ps" command.
 */
#[test]
fn resolve_exec_ps() {
    let reqs = vec![Request { exec: "ps".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "profiler" command.
 */
#[test]
fn resolve_exec_profiler() {
    let reqs = vec![Request { exec: "profiler".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "uname" command.
 */
#[test]
fn resolve_exec_uname() {
    let reqs = vec![Request { exec: "uname".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "uptime" command.
 */
#[test]
fn resolve_exec_uptime() {
    let reqs = vec![Request { exec: "uptime".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "avax" command.
 */
#[test]
fn resolve_exec_avax() {
    let reqs = vec![Request { exec: "avax".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "avalanche" alias.
 */
#[test]
fn resolve_exec_avalanche_alias() {
    let reqs = vec![Request { exec: "avalanche".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "help" command.
 */
#[test]
fn resolve_exec_help() {
    let reqs = vec![Request { exec: "help".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());

    let response = result.unwrap();
    assert!(response.contains("Help is temporarily unavailable"));
}

/**
 * resolve_exec handles "install go" command.
 */
#[test]
fn resolve_exec_install_go() {
    let reqs = vec![Request { exec: "install go".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles "install golang" alias.
 */
#[test]
fn resolve_exec_install_golang() {
    let reqs = vec![Request { exec: "install golang".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some());
}

/**
 * resolve_exec handles unimplemented "arb" command.
 */
#[test]
fn resolve_exec_arb_unimplemented() {
    let reqs = vec![Request { exec: "arb".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Arbitrum is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "arbitrum" alias.
 */
#[test]
fn resolve_exec_arbitrum_alias() {
    let reqs = vec![Request { exec: "arbitrum".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Arbitrum is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "base" command.
 */
#[test]
fn resolve_exec_base_unimplemented() {
    let reqs = vec![Request { exec: "base".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Base is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "nexa" command.
 */
#[test]
fn resolve_exec_nexa_unimplemented() {
    let reqs = vec![Request { exec: "nexa".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Nexa is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "op" command.
 */
#[test]
fn resolve_exec_op_unimplemented() {
    let reqs = vec![Request { exec: "op".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Optimism is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "optimism" alias.
 */
#[test]
fn resolve_exec_optimism_alias() {
    let reqs = vec![Request { exec: "optimism".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Optimism is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "sol" command.
 */
#[test]
fn resolve_exec_sol_unimplemented() {
    let reqs = vec![Request { exec: "sol".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Solana is NOT implemented"));
}

/**
 * resolve_exec handles unimplemented "solana" alias.
 */
#[test]
fn resolve_exec_solana_alias() {
    let reqs = vec![Request { exec: "solana".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("Solana is NOT implemented"));
}

/**
 * resolve_exec handles unknown command.
 */
#[test]
fn resolve_exec_unknown_command() {
    let reqs = vec![Request { exec: "foobar_unknown".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs).unwrap();
    assert!(result.contains("UNKNOWN command"));
    assert!(result.contains("foobar_unknown"));
}

// ---------------------------------------------------------------
// resolve_exec — network command branches (exercise lines 194-223)
// ---------------------------------------------------------------

/**
 * resolve_exec handles "install avax" command.
 */
#[test]
fn resolve_exec_install_avax() {
    let reqs = vec![Request { exec: "install avax".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('install avax') should return Some");
}

/**
 * resolve_exec handles "install avalanche" alias.
 */
#[test]
fn resolve_exec_install_avalanche() {
    let reqs = vec![Request { exec: "install avalanche".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('install avalanche') should return Some");
}

/**
 * resolve_exec handles "start avax" command.
 */
#[test]
fn resolve_exec_start_avax() {
    let reqs = vec![Request { exec: "start avax".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('start avax') should return Some");
}

/**
 * resolve_exec handles "start avalanche" alias.
 */
#[test]
fn resolve_exec_start_avalanche() {
    let reqs = vec![Request { exec: "start avalanche".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('start avalanche') should return Some");
}

/**
 * resolve_exec handles "avax status" command.
 */
#[test]
fn resolve_exec_avax_status() {
    let reqs = vec![Request { exec: "avax status".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('avax status') should return Some");
}

/**
 * resolve_exec handles "avalanche status" alias.
 */
#[test]
fn resolve_exec_avalanche_status() {
    let reqs = vec![Request { exec: "avalanche status".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('avalanche status') should return Some");
}

/**
 * resolve_exec handles "build avax" command.
 */
#[test]
fn resolve_exec_build_avax() {
    let reqs = vec![Request { exec: "build avax".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('build avax') should return Some");
}

/**
 * resolve_exec handles "build avalanche" alias.
 */
#[test]
fn resolve_exec_build_avalanche() {
    let reqs = vec![Request { exec: "build avalanche".to_string(), created_at: 100 }];
    let result = resolve_exec(&reqs);
    assert!(result.is_some(), "resolve_exec('build avalanche') should return Some");
}

// ---------------------------------------------------------------
// mockito — network I/O tests for request_json_async / response_json_async
// ---------------------------------------------------------------

/**
 * request_json_async_with_base sends GET with auth header and returns body.
 */
#[tokio::test]
async fn request_json_async_with_base_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/session/12345")
        .match_header("Authorization", "Bearer test-sess-id")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_body(r#"{"success":true,"result":{"sessionid":"test-sess-id","act":null,"log":null,"req":null,"res":null,"rpt":null,"created_at":0,"last_since":99999}}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = request_json_async_with_base(&base_url, "test-sess-id", 12345).await;

    mock.assert_async().await;
    assert!(result.is_ok());

    let body = result.unwrap();
    assert!(body.contains("\"success\":true"));
    assert!(body.contains("\"last_since\":99999"));
}

/**
 * request_json_async_with_base returns Err on connection refused.
 */
#[tokio::test]
async fn request_json_async_with_base_connection_refused() {
    let result = request_json_async_with_base(
        "http://127.0.0.1:1/",
        "sess-id",
        1,
    ).await;

    assert!(result.is_err());
}

/**
 * request_json_async_with_base handles 500 response.
 */
#[tokio::test]
async fn request_json_async_with_base_server_error() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/session/1")
        .with_status(500)
        .with_body("server error")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = request_json_async_with_base(&base_url, "sess", 1).await;

    mock.assert_async().await;
    /* reqwest does not error on 500 — it returns the body. */
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "server error");
}

/**
 * request_json_async_with_base handles empty response body.
 */
#[tokio::test]
async fn request_json_async_with_base_empty_body() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/session/0")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = request_json_async_with_base(&base_url, "sess", 0).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

/**
 * response_json_async_with_base posts exec response JSON and returns body.
 */
#[tokio::test]
async fn response_json_async_with_base_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_body(r#"{"ok":true}"#)
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = response_json_async_with_base(
        &base_url,
        "sess-abc",
        "command output here".to_string(),
    ).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"ok":true}"#);
}

/**
 * response_json_async_with_base returns Err on connection refused.
 */
#[tokio::test]
async fn response_json_async_with_base_connection_refused() {
    let result = response_json_async_with_base(
        "http://127.0.0.1:1/",
        "sess",
        "data".to_string(),
    ).await;

    assert!(result.is_err());
}

/**
 * response_json_async_with_base handles 500 response.
 */
#[tokio::test]
async fn response_json_async_with_base_server_error() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/session")
        .with_status(500)
        .with_body("internal error")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = response_json_async_with_base(
        &base_url,
        "sess",
        "output".to_string(),
    ).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "internal error");
}

/**
 * response_json_async_with_base sends correctly structured JSON body.
 */
#[tokio::test]
async fn response_json_async_with_base_body_structure() {
    let mut server = mockito::Server::new_async().await;

    let expected_body = build_exec_response_json("my-sess", "my output");

    let mock = server.mock("POST", "/session")
        .match_body(expected_body.as_str())
        .with_status(200)
        .with_body("ok")
        .create_async()
        .await;

    let base_url = format!("{}/", server.url());
    let result = response_json_async_with_base(
        &base_url,
        "my-sess",
        "my output".to_string(),
    ).await;

    mock.assert_async().await;
    assert!(result.is_ok());
}
