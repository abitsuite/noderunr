// src/node/session.test.rs

use super::*;
use serde_json::{from_str, to_string};
use std::collections::HashMap;

// ---------------------------------------------------------------
// Registration — construction and serialization
// ---------------------------------------------------------------

/**
 * build_registration produces a Registration with method "reg".
 */
#[test]
fn build_registration_method_is_reg() {
    let reg = build_registration("1.2.3.4", "Ubuntu 22.04", "5 days", "x86_64", "16GB", "linux");
    assert_eq!(reg.method, "reg");
}

/**
 * build_registration stores all fields correctly.
 */
#[test]
fn build_registration_all_fields() {
    let reg = build_registration("10.0.0.1", "Debian 12", "2h", "arm64", "8GB", "darwin");

    assert_eq!(reg.ip, "10.0.0.1");
    assert_eq!(reg.release, "Debian 12");
    assert_eq!(reg.uptime, "2h");
    assert_eq!(reg.cpu, "arm64");
    assert_eq!(reg.mem, "8GB");
    assert_eq!(reg.profile, "darwin");
}

/**
 * build_registration handles empty strings without panicking.
 */
#[test]
fn build_registration_empty_fields() {
    let reg = build_registration("", "", "", "", "", "");

    assert_eq!(reg.method, "reg");
    assert_eq!(reg.ip, "");
    assert_eq!(reg.release, "");
    assert_eq!(reg.uptime, "");
    assert_eq!(reg.cpu, "");
    assert_eq!(reg.mem, "");
    assert_eq!(reg.profile, "");
}

/**
 * build_registration handles unicode without panicking.
 */
#[test]
fn build_registration_unicode() {
    let reg = build_registration("🌍", "릴리스", "日本語", "процессор", "記憶體", "профіль");

    assert_eq!(reg.ip, "🌍");
    assert_eq!(reg.release, "릴리스");
}

// ---------------------------------------------------------------
// serialize_registration — JSON output
// ---------------------------------------------------------------

/**
 * serialize_registration produces valid JSON.
 */
#[test]
fn serialize_registration_valid_json() {
    let reg = build_registration("1.2.3.4", "Ubuntu", "1d", "x86", "4GB", "linux");
    let json_str = serialize_registration(&reg);

    /* Must parse back without error. */
    let parsed: Registration = from_str(&json_str).unwrap();
    assert_eq!(parsed, reg);
}

/**
 * serialize_registration JSON contains method "reg".
 */
#[test]
fn serialize_registration_contains_method() {
    let reg = build_registration("1.2.3.4", "Ubuntu", "1d", "x86", "4GB", "linux");
    let json_str = serialize_registration(&reg);

    assert!(json_str.contains("\"method\":\"reg\""));
}

/**
 * serialize_registration roundtrip preserves all fields.
 */
#[test]
fn serialize_registration_roundtrip() {
    let reg = build_registration("192.168.1.1", "Fedora 39", "3d 5h", "aarch64", "32GB", "profdata");
    let json_str = serialize_registration(&reg);
    let parsed: Registration = from_str(&json_str).unwrap();

    assert_eq!(parsed.method, "reg");
    assert_eq!(parsed.ip, "192.168.1.1");
    assert_eq!(parsed.release, "Fedora 39");
    assert_eq!(parsed.uptime, "3d 5h");
    assert_eq!(parsed.cpu, "aarch64");
    assert_eq!(parsed.mem, "32GB");
    assert_eq!(parsed.profile, "profdata");
}

// ---------------------------------------------------------------
// parse_registration_response — JSON parsing
// ---------------------------------------------------------------

/**
 * parse_registration_response succeeds on valid JSON.
 */
#[test]
fn parse_registration_response_valid() {
    let json_str = r#"{
        "success": true,
        "result": {
            "sessionid": "abc-123",
            "hasAuth": false,
            "createdAt": 1700000
        }
    }"#;

    let parsed = parse_registration_response(json_str).unwrap();

    assert!(parsed.success);
    assert_eq!(parsed.result.sessionid, "abc-123");
    assert!(!parsed.result.hasAuth);
    assert_eq!(parsed.result.createdAt, 1700000);
}

/**
 * parse_registration_response returns Err on invalid JSON.
 */
#[test]
fn parse_registration_response_invalid_json() {
    let result = parse_registration_response("not json at all");
    assert!(result.is_err());
}

/**
 * parse_registration_response returns Err on empty string.
 */
#[test]
fn parse_registration_response_empty() {
    let result = parse_registration_response("");
    assert!(result.is_err());
}

/**
 * parse_registration_response returns Err on partial JSON.
 */
#[test]
fn parse_registration_response_partial() {
    let result = parse_registration_response(r#"{"success": true}"#);
    assert!(result.is_err());
}

/**
 * parse_registration_response handles success=false.
 */
#[test]
fn parse_registration_response_failure() {
    let json_str = r#"{
        "success": false,
        "result": {
            "sessionid": "",
            "hasAuth": false,
            "createdAt": 0
        }
    }"#;

    let parsed = parse_registration_response(json_str).unwrap();

    assert!(!parsed.success);
    assert_eq!(parsed.result.sessionid, "");
}

/**
 * parse_registration_response handles hasAuth=true.
 */
#[test]
fn parse_registration_response_has_auth() {
    let json_str = r#"{
        "success": true,
        "result": {
            "sessionid": "auth-session",
            "hasAuth": true,
            "createdAt": 999
        }
    }"#;

    let parsed = parse_registration_response(json_str).unwrap();

    assert!(parsed.result.hasAuth);
    assert_eq!(parsed.result.createdAt, 999);
}

/**
 * RegistrationResponse default has expected zero/empty values.
 */
#[test]
fn registration_response_default() {
    let resp = RegistrationResponse::default();

    assert!(!resp.success);
    assert_eq!(resp.result.sessionid, "");
    assert!(!resp.result.hasAuth);
    assert_eq!(resp.result.createdAt, 0);
}

/**
 * Session default has expected zero/empty values.
 */
#[test]
fn session_default() {
    let session = Session::default();

    assert_eq!(session.sessionid, "");
    assert!(!session.hasAuth);
    assert_eq!(session.createdAt, 0);
}

// ---------------------------------------------------------------
// extract_ip — pure extraction from HashMap result
// ---------------------------------------------------------------

/**
 * extract_ip returns the origin value on Ok.
 */
#[test]
fn extract_ip_ok_with_origin() {
    let mut map = HashMap::new();
    map.insert("origin".to_string(), "1.2.3.4".to_string());

    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> = Ok(map);
    let ip = extract_ip(result);

    assert_eq!(ip, "1.2.3.4");
}

/**
 * extract_ip returns empty string when origin key is missing.
 */
#[test]
fn extract_ip_ok_missing_origin() {
    let map = HashMap::new();

    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> = Ok(map);
    let ip = extract_ip(result);

    assert_eq!(ip, "");
}

/**
 * extract_ip returns error message on Err.
 */
#[test]
fn extract_ip_err() {
    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> =
        Err("network timeout".into());
    let ip = extract_ip(result);

    assert_eq!(ip, "network timeout");
}

/**
 * extract_ip handles empty origin value.
 */
#[test]
fn extract_ip_empty_origin() {
    let mut map = HashMap::new();
    map.insert("origin".to_string(), "".to_string());

    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> = Ok(map);
    let ip = extract_ip(result);

    assert_eq!(ip, "");
}

/**
 * extract_ip handles IPv6 address.
 */
#[test]
fn extract_ip_ipv6() {
    let mut map = HashMap::new();
    map.insert("origin".to_string(), "::1".to_string());

    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> = Ok(map);
    let ip = extract_ip(result);

    assert_eq!(ip, "::1");
}

/**
 * extract_ip handles comma-separated IPs (proxy scenario).
 */
#[test]
fn extract_ip_comma_separated() {
    let mut map = HashMap::new();
    map.insert("origin".to_string(), "1.2.3.4, 5.6.7.8".to_string());

    let result: Result<HashMap<String, String>, Box<dyn std::error::Error>> = Ok(map);
    let ip = extract_ip(result);

    assert_eq!(ip, "1.2.3.4, 5.6.7.8");
}
