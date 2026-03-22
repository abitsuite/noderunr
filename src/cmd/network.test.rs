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

// ---------------------------------------------------------------
// Pure helper function tests — no process spawning required
// ---------------------------------------------------------------

/**
 * avax_cli_bin returns expected binary path.
 */
#[test]
fn avax_cli_bin_path() {
    let path = avax_cli_bin();
    assert!(
        path.contains(".noderunr/bin/avalanche"),
        "avax_cli_bin should contain '.noderunr/bin/avalanche', got: {}",
        path
    );
}

/**
 * noderunr_home returns expected home path.
 */
#[test]
fn noderunr_home_path() {
    let path = noderunr_home();
    assert!(
        path.contains(".noderunr"),
        "noderunr_home should contain '.noderunr', got: {}",
        path
    );
}

/**
 * noderunr_bin_dir returns expected bin path.
 */
#[test]
fn noderunr_bin_dir_path() {
    let path = noderunr_bin_dir();
    assert!(
        path.contains(".noderunr/bin"),
        "noderunr_bin_dir should contain '.noderunr/bin', got: {}",
        path
    );
}

/**
 * build_avax_install_steps returns non-empty step list.
 */
#[test]
fn build_avax_install_steps_non_empty() {
    let steps = build_avax_install_steps();
    assert!(
        !steps.is_empty(),
        "build_avax_install_steps should return at least one step"
    );
}

/**
 * build_avax_install_steps contains mkdir as first step.
 */
#[test]
fn build_avax_install_steps_starts_with_mkdir() {
    let steps = build_avax_install_steps();
    assert!(
        steps[0].0.contains("mkdir"),
        "First install step should be mkdir, got: {}",
        steps[0].0
    );
}

/**
 * build_avax_install_steps contains curl step.
 */
#[test]
fn build_avax_install_steps_has_curl() {
    let steps = build_avax_install_steps();
    let has_curl = steps.iter().any(|(cmd, _)| cmd.contains("curl"));
    assert!(
        has_curl,
        "Install steps should contain a curl command"
    );
}

/**
 * build_avax_install_steps each step has non-zero sleep.
 */
#[test]
fn build_avax_install_steps_sleep_values() {
    let steps = build_avax_install_steps();
    for (cmd, sleep_ms) in &steps {
        assert!(
            *sleep_ms > 0,
            "Step '{}' should have positive sleep, got: {}",
            cmd,
            sleep_ms
        );
    }
}

/**
 * build_avax_test_steps returns non-empty step list.
 */
#[test]
fn build_avax_test_steps_non_empty() {
    let steps = build_avax_test_steps();
    assert!(
        !steps.is_empty(),
        "build_avax_test_steps should return at least one step"
    );
}

/**
 * build_avax_test_steps contains --help step.
 */
#[test]
fn build_avax_test_steps_has_help() {
    let steps = build_avax_test_steps();
    let has_help = steps.iter().any(|(cmd, _)| cmd.contains("--help"));
    assert!(
        has_help,
        "Test steps should contain a --help command"
    );
}

/**
 * build_avax_test_steps contains --version step.
 */
#[test]
fn build_avax_test_steps_has_version() {
    let steps = build_avax_test_steps();
    let has_version = steps.iter().any(|(cmd, _)| cmd.contains("--version"));
    assert!(
        has_version,
        "Test steps should contain a --version command"
    );
}

/**
 * build_avax_start_cmd contains "network start".
 */
#[test]
fn build_avax_start_cmd_format() {
    let cmd = build_avax_start_cmd();
    assert!(
        cmd.contains("network start"),
        "avax_start command should contain 'network start', got: {}",
        cmd
    );
    assert!(
        cmd.contains("avalanche"),
        "avax_start command should reference avalanche binary, got: {}",
        cmd
    );
}

/**
 * build_avax_status_cmd contains "network status".
 */
#[test]
fn build_avax_status_cmd_format() {
    let cmd = build_avax_status_cmd();
    assert!(
        cmd.contains("network status"),
        "avax_status command should contain 'network status', got: {}",
        cmd
    );
    assert!(
        cmd.contains("avalanche"),
        "avax_status command should reference avalanche binary, got: {}",
        cmd
    );
}

/**
 * build_avax_stop_cmd contains "network stop".
 */
#[test]
fn build_avax_stop_cmd_format() {
    let cmd = build_avax_stop_cmd();
    assert!(
        cmd.contains("network stop"),
        "avax_stop command should contain 'network stop', got: {}",
        cmd
    );
    assert!(
        cmd.contains("avalanche"),
        "avax_stop command should reference avalanche binary, got: {}",
        cmd
    );
}

/**
 * build_avalanche_steps returns non-empty step list.
 */
#[test]
fn build_avalanche_steps_non_empty() {
    let steps = build_avalanche_steps();
    assert!(
        !steps.is_empty(),
        "build_avalanche_steps should return at least one step"
    );
}

/**
 * build_avalanche_steps contains mkdir step.
 */
#[test]
fn build_avalanche_steps_has_mkdir() {
    let steps = build_avalanche_steps();
    let has_mkdir = steps.iter().any(|(cmd, _)| cmd.contains("mkdir"));
    assert!(
        has_mkdir,
        "Build steps should contain a mkdir command"
    );
}

/**
 * build_avalanche_steps contains build.sh step.
 */
#[test]
fn build_avalanche_steps_has_build_script() {
    let steps = build_avalanche_steps();
    let has_build = steps.iter().any(|(cmd, _)| cmd.contains("build.sh"));
    assert!(
        has_build,
        "Build steps should contain a build.sh command"
    );
}

/**
 * build_avalanche_steps contains PATH export step.
 */
#[test]
fn build_avalanche_steps_has_path_export() {
    let steps = build_avalanche_steps();
    let has_export = steps.iter().any(|(cmd, _)| cmd.contains("export PATH"));
    assert!(
        has_export,
        "Build steps should contain a PATH export command"
    );
}

/**
 * build_avalanche_steps contains avalanchego launch step.
 */
#[test]
fn build_avalanche_steps_has_avalanchego() {
    let steps = build_avalanche_steps();
    let has_avago = steps.iter().any(|(cmd, _)| cmd.contains("avalanchego"));
    assert!(
        has_avago,
        "Build steps should contain an avalanchego command"
    );
}

/**
 * build_avalanche_steps each step has non-zero sleep.
 */
#[test]
fn build_avalanche_steps_sleep_values() {
    let steps = build_avalanche_steps();
    for (cmd, sleep_ms) in &steps {
        assert!(
            *sleep_ms > 0,
            "Step '{}' should have positive sleep, got: {}",
            cmd,
            sleep_ms
        );
    }
}

/**
 * build_avalanche_steps step count matches expected.
 */
#[test]
fn build_avalanche_steps_count() {
    let steps = build_avalanche_steps();
    assert_eq!(
        steps.len(),
        7,
        "build_avalanche_steps should return 7 steps, got: {}",
        steps.len()
    );
}

/**
 * build_avax_install_steps step count matches expected.
 */
#[test]
fn build_avax_install_steps_count() {
    let steps = build_avax_install_steps();
    assert_eq!(
        steps.len(),
        3,
        "build_avax_install_steps should return 3 steps, got: {}",
        steps.len()
    );
}

/**
 * build_avax_test_steps step count matches expected.
 */
#[test]
fn build_avax_test_steps_count() {
    let steps = build_avax_test_steps();
    assert_eq!(
        steps.len(),
        2,
        "build_avax_test_steps should return 2 steps, got: {}",
        steps.len()
    );
}

// ---------------------------------------------------------------
// Live function tests — actually call the functions on Unix
// These spawn bash briefly and return Ok("").
// ---------------------------------------------------------------

/**
 * avax_start — Returns Ok on non-Windows (spawns bash briefly).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_start_returns_ok_on_unix() {
    let result = avax_start();

    assert!(
        result.is_ok(),
        "avax_start() should return Ok on Unix, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * avax_status — Returns Ok on non-Windows (spawns bash briefly).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_status_returns_ok_on_unix() {
    let result = avax_status();

    assert!(
        result.is_ok(),
        "avax_status() should return Ok on Unix, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * avax_stop — Returns Ok on non-Windows (spawns bash briefly).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_stop_returns_ok_on_unix() {
    let result = avax_stop();

    assert!(
        result.is_ok(),
        "avax_stop() should return Ok on Unix, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * avax_start — Result is an empty string (no command output captured).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_start_returns_empty_response() {
    let result = avax_start().unwrap();

    assert_eq!(
        result, "",
        "avax_start() should return empty string, got: {}",
        result
    );
}

/**
 * avax_status — Result is an empty string (no command output captured).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_status_returns_empty_response() {
    let result = avax_status().unwrap();

    assert_eq!(
        result, "",
        "avax_status() should return empty string, got: {}",
        result
    );
}

/**
 * avax_stop — Result is an empty string (no command output captured).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_stop_returns_empty_response() {
    let result = avax_stop().unwrap();

    assert_eq!(
        result, "",
        "avax_stop() should return empty string, got: {}",
        result
    );
}

/**
 * build_avalanche — Returns Ok on non-Windows (spawns bash briefly).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn build_avalanche_returns_ok_on_unix() {
    let result = build_avalanche();

    assert!(
        result.is_ok(),
        "build_avalanche() should return Ok on Unix, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * build_avalanche — Result is an empty string (no command output captured).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn build_avalanche_returns_empty_response() {
    let result = build_avalanche().unwrap();

    assert_eq!(
        result, "",
        "build_avalanche() should return empty string, got: {}",
        result
    );
}

/**
 * avax_install — Returns Ok on non-Windows (spawns bash briefly).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_install_returns_ok_on_unix() {
    let result = avax_install();

    assert!(
        result.is_ok(),
        "avax_install() should return Ok on Unix, got: {:?}",
        result.unwrap_err()
    );
}

/**
 * avax_install — Result is an empty string (no command output captured).
 */
#[test]
#[cfg(not(target_os = "windows"))]
fn avax_install_returns_empty_response() {
    let result = avax_install().unwrap();

    assert_eq!(
        result, "",
        "avax_install() should return empty string, got: {}",
        result
    );
}
