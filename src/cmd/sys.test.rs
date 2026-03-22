// src/cmd/sys.test.rs

use super::sys;

/**
 * sys::df — Returns Ok and produces output.
 */
#[test]
fn df_returns_ok() {
    let result = sys::df();
    assert!(result.is_ok(), "sys::df() returned Err: {}", result.unwrap_err());
}

/**
 * sys::df — Output contains filesystem-related content.
 */
#[test]
fn df_output_has_content() {
    let result = sys::df().unwrap();
    assert!(!result.trim().is_empty(), "sys::df() returned empty string");

    if cfg!(not(target_os = "windows")) {
        /* On Linux/macOS, df -h output typically contains "Filesystem" header. */
        assert!(
            result.contains("Filesystem") || result.contains("filesystem") || result.contains("/"),
            "sys::df() output does not look like df output: {}",
            &result[..result.len().min(200)]
        );
    }
}

/**
 * sys::du — Returns Ok.
 */
#[test]
fn du_returns_ok() {
    let result = sys::du();
    assert!(result.is_ok(), "sys::du() returned Err: {}", result.unwrap_err());
}

/**
 * sys::ls — Returns Ok and produces non-empty output.
 */
#[test]
fn ls_returns_ok_and_non_empty() {
    let result = sys::ls();
    assert!(result.is_ok(), "sys::ls() returned Err: {}", result.unwrap_err());

    let output = result.unwrap();
    assert!(!output.trim().is_empty(), "sys::ls() returned empty string");
}

/**
 * sys::lsblk — Returns Ok.
 */
#[test]
fn lsblk_returns_ok() {
    let result = sys::lsblk();
    assert!(result.is_ok(), "sys::lsblk() returned Err: {}", result.unwrap_err());
}

/**
 * sys::lscpu — Returns Ok.
 */
#[test]
fn lscpu_returns_ok() {
    let result = sys::lscpu();
    assert!(result.is_ok(), "sys::lscpu() returned Err: {}", result.unwrap_err());
}

/**
 * sys::lshw — Returns Ok.
 */
#[test]
fn lshw_returns_ok() {
    let result = sys::lshw();
    assert!(result.is_ok(), "sys::lshw() returned Err: {}", result.unwrap_err());
}

/**
 * sys::mem — Returns Ok.
 */
#[test]
fn mem_returns_ok() {
    let result = sys::mem();
    assert!(result.is_ok(), "sys::mem() returned Err: {}", result.unwrap_err());
}

/**
 * sys::ps — Returns Ok and output contains at least one process.
 */
#[test]
fn ps_returns_ok_and_has_processes() {
    let result = sys::ps();
    assert!(result.is_ok(), "sys::ps() returned Err: {}", result.unwrap_err());

    let output = result.unwrap();
    assert!(!output.trim().is_empty(), "sys::ps() returned empty string");

    /* The output should have multiple lines (header + at least one process). */
    let line_count = output.lines().count();
    assert!(
        line_count >= 2,
        "sys::ps() should list at least one process, got {} lines",
        line_count
    );
}

/**
 * sys::get_uname — Returns Ok and is non-empty.
 */
#[test]
fn get_uname_returns_ok_and_non_empty() {
    let result = sys::get_uname();
    assert!(result.is_ok(), "sys::get_uname() returned Err: {}", result.unwrap_err());

    let output = result.unwrap();
    assert!(
        !output.trim().is_empty(),
        "sys::get_uname() returned empty string"
    );
}

/**
 * sys::get_uname — On Linux, output contains "Linux".
 */
#[test]
fn get_uname_contains_os_identifier() {
    let result = sys::get_uname().unwrap();

    if cfg!(target_os = "linux") {
        assert!(
            result.contains("Linux"),
            "On Linux, uname should contain 'Linux', got: {}",
            result.trim()
        );
    }
}

/**
 * sys::get_uptime — Returns Ok.
 */
#[test]
fn get_uptime_returns_ok() {
    let result = sys::get_uptime();
    assert!(result.is_ok(), "sys::get_uptime() returned Err: {}", result.unwrap_err());
}

/**
 * sys::get_release — Returns Ok and is non-empty.
 */
#[test]
fn get_release_returns_ok_and_non_empty() {
    let result = sys::get_release();
    assert!(result.is_ok(), "sys::get_release() returned Err: {}", result.unwrap_err());

    let output = result.unwrap();
    assert!(
        !output.trim().is_empty(),
        "sys::get_release() returned empty string"
    );
}

/**
 * sys::system_profiler — Returns Ok.
 */
#[test]
fn system_profiler_returns_ok() {
    let result = sys::system_profiler();
    assert!(
        result.is_ok(),
        "sys::system_profiler() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::install_golang — Returns Err on Windows with a descriptive message.
 */
#[test]
fn install_golang_windows_guard() {
    if cfg!(target_os = "windows") {
        let result = sys::install_golang();
        assert!(
            result.is_err(),
            "sys::install_golang() should return Err on Windows"
        );

        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("not supported on Windows"),
            "Error message should mention Windows, got: {}",
            err_msg
        );
    }
}
