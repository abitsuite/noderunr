// src/node/session.test.rs

use super::*;
use serde_json::{from_str, to_string};
use std::collections::HashMap;

// ---------------------------------------------------------------
// ServiceSnapshot — construction and serialization
// ---------------------------------------------------------------

/**
 * ServiceSnapshot can be constructed with all fields.
 */
#[test]
fn service_snapshot_all_fields() {
    let svc = ServiceSnapshot {
        name: "avalanchego".to_string(),
        display_name: "Avalanche".to_string(),
        installed: true,
        running: true,
        version: "1.11.3".to_string(),
        pid: 12345,
    };

    assert_eq!(svc.name, "avalanchego");
    assert_eq!(svc.display_name, "Avalanche");
    assert!(svc.installed);
    assert!(svc.running);
    assert_eq!(svc.version, "1.11.3");
    assert_eq!(svc.pid, 12345);
}

/**
 * ServiceSnapshot handles not-installed service.
 */
#[test]
fn service_snapshot_not_installed() {
    let svc = ServiceSnapshot {
        name: "nexad".to_string(),
        display_name: "Nexa".to_string(),
        installed: false,
        running: false,
        version: "".to_string(),
        pid: 0,
    };

    assert!(!svc.installed);
    assert!(!svc.running);
    assert_eq!(svc.pid, 0);
}

/**
 * ServiceSnapshot serde roundtrip preserves all fields.
 */
#[test]
fn service_snapshot_serde_roundtrip() {
    let svc = ServiceSnapshot {
        name: "dashd".to_string(),
        display_name: "Dash".to_string(),
        installed: true,
        running: false,
        version: "20.0.1".to_string(),
        pid: 0,
    };

    let json_str = to_string(&svc).unwrap();
    let parsed: ServiceSnapshot = from_str(&json_str).unwrap();

    assert_eq!(parsed, svc);
}

/**
 * ServiceSnapshot clone produces equal copy.
 */
#[test]
fn service_snapshot_clone() {
    let svc = ServiceSnapshot {
        name: "avalanchego".to_string(),
        display_name: "Avalanche".to_string(),
        installed: true,
        running: true,
        version: "1.11.3".to_string(),
        pid: 9999,
    };

    let cloned = svc.clone();
    assert_eq!(cloned, svc);
}

// ---------------------------------------------------------------
// Registration — construction and serialization
// ---------------------------------------------------------------

/**
 * build_registration produces a Registration with method "reg".
 */
#[test]
fn build_registration_method_is_reg() {
    let reg = build_registration(
        "host1",
        "Ubuntu 22.04",
        "x86_64",
        "6.5.0",
        "machine-id-abc",
        "1.2.3.4",
        "AMD EPYC",
        8,
        32768,
        500,
        "5 days",
        42,
        (0.5, 0.4, 0.3),
        vec![],
        "Ubuntu 22.04",
        "x86_64",
        "16GB",
        "linux",
    );
    assert_eq!(reg.method, "reg");
}

/**
 * build_registration stores all identity fields correctly.
 */
#[test]
fn build_registration_identity_fields() {
    let reg = build_registration(
        "validator-01",
        "Debian 12",
        "aarch64",
        "6.1.0-22",
        "deadbeef12345678",
        "10.0.0.1",
        "Cortex-A76",
        4,
        8192,
        240,
        "2h",
        18,
        (0.1, 0.2, 0.15),
        vec![],
        "Debian 12",
        "arm64",
        "8GB",
        "darwin",
    );

    assert_eq!(reg.hostname, "validator-01");
    assert_eq!(reg.os, "Debian 12");
    assert_eq!(reg.arch, "aarch64");
    assert_eq!(reg.kernel, "6.1.0-22");
    assert_eq!(reg.machine_id, "deadbeef12345678");
}

/**
 * build_registration stores all hardware fields correctly.
 */
#[test]
fn build_registration_hardware_fields() {
    let reg = build_registration(
        "host",
        "os",
        "arch",
        "kern",
        "mid",
        "10.0.0.1",
        "Intel Xeon E5-2680",
        16,
        65536,
        1000,
        "10d",
        55,
        (1.0, 0.8, 0.6),
        vec![],
        "release",
        "cpu",
        "mem",
        "profile",
    );

    assert_eq!(reg.ip, "10.0.0.1");
    assert_eq!(reg.cpu_model, "Intel Xeon E5-2680");
    assert_eq!(reg.cpu_cores, 16);
    assert_eq!(reg.mem_total_mb, 65536);
    assert_eq!(reg.disk_total_gb, 1000);
}

/**
 * build_registration stores runtime snapshot fields correctly.
 */
#[test]
fn build_registration_runtime_fields() {
    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu", 1, 1, 1, "3 days, 5:22", 87,
        (2.5, 1.8, 1.2),
        vec![],
        "r", "c", "m", "p",
    );

    assert_eq!(reg.uptime, "3 days, 5:22");
    assert_eq!(reg.disk_used_pct, 87);
    assert_eq!(reg.load_avg, (2.5, 1.8, 1.2));
}

/**
 * build_registration stores services correctly.
 */
#[test]
fn build_registration_with_services() {
    let services = vec![
        ServiceSnapshot {
            name: "avalanchego".to_string(),
            display_name: "Avalanche".to_string(),
            installed: true,
            running: true,
            version: "1.11.3".to_string(),
            pid: 12345,
        },
        ServiceSnapshot {
            name: "nexad".to_string(),
            display_name: "Nexa".to_string(),
            installed: false,
            running: false,
            version: "".to_string(),
            pid: 0,
        },
    ];

    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu", 1, 1, 1, "up", 0,
        (0.0, 0.0, 0.0),
        services.clone(),
        "r", "c", "m", "p",
    );

    assert_eq!(reg.services.len(), 2);
    assert_eq!(reg.services[0].name, "avalanchego");
    assert!(reg.services[0].installed);
    assert!(reg.services[0].running);
    assert_eq!(reg.services[0].pid, 12345);
    assert_eq!(reg.services[1].name, "nexad");
    assert!(!reg.services[1].installed);
}

/**
 * build_registration preserves legacy fields.
 */
#[test]
fn build_registration_legacy_fields() {
    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu", 1, 1, 1, "up", 0,
        (0.0, 0.0, 0.0),
        vec![],
        "Linux validator-01 6.5.0 ...",
        "CPU NODE SOCKET ...",
        "               total ...",
        "H/W path ...",
    );

    assert_eq!(reg.release, "Linux validator-01 6.5.0 ...");
    assert_eq!(reg.cpu, "CPU NODE SOCKET ...");
    assert_eq!(reg.mem, "               total ...");
    assert_eq!(reg.profile, "H/W path ...");
}

/**
 * build_registration handles empty strings without panicking.
 */
#[test]
fn build_registration_empty_fields() {
    let reg = build_registration(
        "", "", "", "", "", "", "", 0, 0, 0, "", 0,
        (0.0, 0.0, 0.0),
        vec![],
        "", "", "", "",
    );

    assert_eq!(reg.method, "reg");
    assert_eq!(reg.hostname, "");
    assert_eq!(reg.os, "");
    assert_eq!(reg.arch, "");
    assert_eq!(reg.kernel, "");
    assert_eq!(reg.machine_id, "");
    assert_eq!(reg.ip, "");
    assert_eq!(reg.cpu_model, "");
    assert_eq!(reg.cpu_cores, 0);
    assert_eq!(reg.mem_total_mb, 0);
    assert_eq!(reg.disk_total_gb, 0);
    assert_eq!(reg.uptime, "");
    assert_eq!(reg.disk_used_pct, 0);
    assert_eq!(reg.load_avg, (0.0, 0.0, 0.0));
    assert!(reg.services.is_empty());
    assert_eq!(reg.release, "");
    assert_eq!(reg.cpu, "");
    assert_eq!(reg.mem, "");
    assert_eq!(reg.profile, "");
}

/**
 * build_registration handles unicode without panicking.
 */
#[test]
fn build_registration_unicode() {
    let reg = build_registration(
        "ホスト名", "릴리스", "アーキ", "カーネル", "マシンID", "🌍", "процессор",
        4, 8192, 100, "日本語", 50,
        (1.0, 2.0, 3.0),
        vec![],
        "릴리스", "процессор", "記憶體", "профіль",
    );

    assert_eq!(reg.hostname, "ホスト名");
    assert_eq!(reg.ip, "🌍");
    assert_eq!(reg.cpu_model, "процессор");
}

/**
 * build_registration handles max numeric values.
 */
#[test]
fn build_registration_max_values() {
    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu",
        u32::MAX, u64::MAX, u64::MAX, "up", u8::MAX,
        (f64::MAX, f64::MAX, f64::MAX),
        vec![],
        "r", "c", "m", "p",
    );

    assert_eq!(reg.cpu_cores, u32::MAX);
    assert_eq!(reg.mem_total_mb, u64::MAX);
    assert_eq!(reg.disk_total_gb, u64::MAX);
    assert_eq!(reg.disk_used_pct, u8::MAX);
}

// ---------------------------------------------------------------
// serialize_registration — JSON output
// ---------------------------------------------------------------

/**
 * serialize_registration produces valid JSON.
 */
#[test]
fn serialize_registration_valid_json() {
    let reg = build_registration(
        "host", "Ubuntu", "x86_64", "6.5.0", "mid123",
        "1.2.3.4", "AMD EPYC", 8, 32768, 500, "1d", 42,
        (0.5, 0.4, 0.3),
        vec![],
        "Ubuntu", "x86", "4GB", "linux",
    );
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
    let reg = build_registration(
        "h", "o", "a", "k", "m", "1.2.3.4", "cpu", 1, 1, 1, "1d", 0,
        (0.0, 0.0, 0.0),
        vec![],
        "r", "c", "m", "p",
    );
    let json_str = serialize_registration(&reg);

    assert!(json_str.contains("\"method\":\"reg\""));
}

/**
 * serialize_registration roundtrip preserves all fields.
 */
#[test]
fn serialize_registration_roundtrip() {
    let services = vec![
        ServiceSnapshot {
            name: "avalanchego".to_string(),
            display_name: "Avalanche".to_string(),
            installed: true,
            running: true,
            version: "1.11.3".to_string(),
            pid: 42,
        },
    ];

    let reg = build_registration(
        "validator-01",
        "Fedora 39",
        "aarch64",
        "6.6.0",
        "fedora-machine-id",
        "192.168.1.1",
        "Apple M2",
        10,
        16384,
        256,
        "3d 5h",
        65,
        (1.2, 0.9, 0.7),
        services,
        "Fedora 39",
        "aarch64",
        "32GB",
        "profdata",
    );
    let json_str = serialize_registration(&reg);
    let parsed: Registration = from_str(&json_str).unwrap();

    assert_eq!(parsed.method, "reg");
    assert_eq!(parsed.hostname, "validator-01");
    assert_eq!(parsed.os, "Fedora 39");
    assert_eq!(parsed.arch, "aarch64");
    assert_eq!(parsed.kernel, "6.6.0");
    assert_eq!(parsed.machine_id, "fedora-machine-id");
    assert_eq!(parsed.ip, "192.168.1.1");
    assert_eq!(parsed.cpu_model, "Apple M2");
    assert_eq!(parsed.cpu_cores, 10);
    assert_eq!(parsed.mem_total_mb, 16384);
    assert_eq!(parsed.disk_total_gb, 256);
    assert_eq!(parsed.uptime, "3d 5h");
    assert_eq!(parsed.disk_used_pct, 65);
    assert_eq!(parsed.load_avg, (1.2, 0.9, 0.7));
    assert_eq!(parsed.services.len(), 1);
    assert_eq!(parsed.services[0].name, "avalanchego");
    assert!(parsed.services[0].running);
    assert_eq!(parsed.release, "Fedora 39");
    assert_eq!(parsed.cpu, "aarch64");
    assert_eq!(parsed.mem, "32GB");
    assert_eq!(parsed.profile, "profdata");
}

/**
 * serialize_registration includes services array in JSON.
 */
#[test]
fn serialize_registration_includes_services() {
    let services = vec![
        ServiceSnapshot {
            name: "nexad".to_string(),
            display_name: "Nexa".to_string(),
            installed: true,
            running: false,
            version: "1.4.1".to_string(),
            pid: 0,
        },
        ServiceSnapshot {
            name: "dashd".to_string(),
            display_name: "Dash".to_string(),
            installed: true,
            running: true,
            version: "20.0.1".to_string(),
            pid: 5678,
        },
    ];

    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu", 1, 1, 1, "up", 0,
        (0.0, 0.0, 0.0),
        services,
        "r", "c", "m", "p",
    );

    let json_str = serialize_registration(&reg);

    assert!(json_str.contains("\"nexad\""));
    assert!(json_str.contains("\"dashd\""));
    assert!(json_str.contains("\"Nexa\""));
    assert!(json_str.contains("\"Dash\""));
    assert!(json_str.contains("\"1.4.1\""));
    assert!(json_str.contains("5678"));
}

/**
 * serialize_registration handles empty services array.
 */
#[test]
fn serialize_registration_empty_services() {
    let reg = build_registration(
        "h", "o", "a", "k", "m", "ip", "cpu", 1, 1, 1, "up", 0,
        (0.0, 0.0, 0.0),
        vec![],
        "r", "c", "m", "p",
    );

    let json_str = serialize_registration(&reg);
    assert!(json_str.contains("\"services\":[]"));
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
