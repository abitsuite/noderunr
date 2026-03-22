// tests/sys.rs

/**
 * sys::df — Returns Ok on all platforms
 *
 * On Linux/macOS this calls `df -h`, on Windows `wmic logicaldisk`.
 * We only assert it doesn't panic and returns Ok.
 */
#[test]
fn sys_df_does_not_panic() {
    let result = noderunr::cmd::sys::df();

    assert!(
        result.is_ok(),
        "sys::df() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::du — Returns Ok on all platforms
 */
#[test]
fn sys_du_does_not_panic() {
    let result = noderunr::cmd::sys::du();

    assert!(
        result.is_ok(),
        "sys::du() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::ls — Returns Ok on all platforms
 */
#[test]
fn sys_ls_does_not_panic() {
    let result = noderunr::cmd::sys::ls();

    assert!(
        result.is_ok(),
        "sys::ls() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::lsblk — Returns Ok on all platforms
 */
#[test]
fn sys_lsblk_does_not_panic() {
    let result = noderunr::cmd::sys::lsblk();

    assert!(
        result.is_ok(),
        "sys::lsblk() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::lscpu — Returns Ok on all platforms
 *
 * lscpu() uses match internally, so it should never Err — but the
 * response body may be an error string on unsupported platforms.
 */
#[test]
fn sys_lscpu_does_not_panic() {
    let result = noderunr::cmd::sys::lscpu();

    assert!(
        result.is_ok(),
        "sys::lscpu() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::lshw — Returns Ok on all platforms
 */
#[test]
fn sys_lshw_does_not_panic() {
    let result = noderunr::cmd::sys::lshw();

    assert!(
        result.is_ok(),
        "sys::lshw() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::mem — Returns Ok on all platforms
 */
#[test]
fn sys_mem_does_not_panic() {
    let result = noderunr::cmd::sys::mem();

    assert!(
        result.is_ok(),
        "sys::mem() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::ps — Returns Ok on all platforms
 */
#[test]
fn sys_ps_does_not_panic() {
    let result = noderunr::cmd::sys::ps();

    assert!(
        result.is_ok(),
        "sys::ps() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::get_uname — Returns Ok on all platforms
 *
 * On Linux/macOS this calls `uname -a`, on Windows `cmd /C ver`.
 */
#[test]
fn sys_get_uname_does_not_panic() {
    let result = noderunr::cmd::sys::get_uname();

    assert!(
        result.is_ok(),
        "sys::get_uname() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::get_uname — Output is non-empty
 */
#[test]
fn sys_get_uname_non_empty() {
    let result = noderunr::cmd::sys::get_uname().unwrap();

    assert!(
        !result.trim().is_empty(),
        "sys::get_uname() returned empty string"
    );
}

/**
 * sys::get_uptime — Returns Ok on all platforms
 */
#[test]
fn sys_get_uptime_does_not_panic() {
    let result = noderunr::cmd::sys::get_uptime();

    assert!(
        result.is_ok(),
        "sys::get_uptime() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::get_release — Returns Ok on all platforms
 */
#[test]
fn sys_get_release_does_not_panic() {
    let result = noderunr::cmd::sys::get_release();

    assert!(
        result.is_ok(),
        "sys::get_release() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::get_release — Output is non-empty
 */
#[test]
fn sys_get_release_non_empty() {
    let result = noderunr::cmd::sys::get_release().unwrap();

    assert!(
        !result.trim().is_empty(),
        "sys::get_release() returned empty string"
    );
}

/**
 * sys::system_profiler — Returns Ok on all platforms
 *
 * On macOS this calls `system_profiler`, on Windows `systeminfo`,
 * on Linux it may return an error string in the body (but Ok).
 */
#[test]
fn sys_system_profiler_does_not_panic() {
    let result = noderunr::cmd::sys::system_profiler();

    assert!(
        result.is_ok(),
        "sys::system_profiler() returned Err: {}",
        result.unwrap_err()
    );
}

/**
 * sys::install_golang — Returns Err on Windows, Ok on Unix
 *
 * We only verify it doesn't panic. On Windows it should return a
 * descriptive Err; on Unix it would attempt to run bash.
 */
#[test]
fn sys_install_golang_does_not_panic() {
    let result = noderunr::cmd::sys::install_golang();

    if cfg!(target_os = "windows") {
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
    /* On Unix we don't assert Ok because bash+go may not be installed in CI. */
}
