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
 * URL construction for session request matches expected format.
 */
#[test]
fn url_construction_session_since() {
    let since: u64 = 1234567890;
    let url = format!("{}{}/{}", L1_ENDPOINT, "session", since);

    assert_eq!(url, "https://l1.run/v1/session/1234567890");
}

/**
 * URL construction for session post matches expected format.
 */
#[test]
fn url_construction_session_post() {
    let url = format!("{}{}", L1_ENDPOINT, "session");

    assert_eq!(url, "https://l1.run/v1/session");
}

/**
 * Bearer authorization string is correctly formatted.
 */
#[test]
fn auth_header_format() {
    let sessionid = "test-session-abc";
    let auth = format!("{} {}", "Bearer", sessionid);

    assert_eq!(auth, "Bearer test-session-abc");
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
