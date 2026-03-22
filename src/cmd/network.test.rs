// src/cmd/network.test.rs

use super::*;

/**
 * shell_cmd returns "bash" on non-Windows.
 */
#[test]
fn shell_cmd_returns_expected() {
    let cmd = shell_cmd();

    if cfg!(target_os = "windows") {
        assert_eq!(cmd, "cmd");
    } else {
        assert_eq!(cmd, "bash");
    }
}

/**
 * shell_prefix returns expected args for current OS.
 */
#[test]
fn shell_prefix_returns_expected() {
    let prefix = shell_prefix();

    if cfg!(target_os = "windows") {
        assert_eq!(prefix, &["/C"]);
    } else {
        assert_eq!(prefix, &["-c"]);
    }
}

/**
 * avax() returns Ok (may contain ERROR if avalanche CLI not installed).
 */
#[test]
fn avax_returns_ok() {
    let result = avax();

    assert!(
        result.is_ok(),
        "avax() should return Ok, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * avax() output is a non-empty string.
 */
#[test]
fn avax_output_is_non_empty() {
    let result = avax().unwrap();

    assert!(
        !result.is_empty(),
        "avax() should return non-empty output"
    );
}

/**
 * avax_install — Returns Err on Windows.
 */
#[test]
fn avax_install_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = avax_install();

        assert!(
            result.is_err(),
            "avax_install() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}

/**
 * avax_start — Returns Err on Windows.
 */
#[test]
fn avax_start_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = avax_start();

        assert!(
            result.is_err(),
            "avax_start() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}

/**
 * avax_status — Returns Err on Windows.
 */
#[test]
fn avax_status_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = avax_status();

        assert!(
            result.is_err(),
            "avax_status() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}

/**
 * avax_stop — Returns Err on Windows.
 */
#[test]
fn avax_stop_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = avax_stop();

        assert!(
            result.is_err(),
            "avax_stop() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}

/**
 * build_avalanche — Returns Err on Windows.
 */
#[test]
fn build_avalanche_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = build_avalanche();

        assert!(
            result.is_err(),
            "build_avalanche() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}
