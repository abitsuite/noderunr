// tests/version.rs

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use assert_cmd::Command; // Run programs (with timeout support)

/**
 * Version Flag (Long)
 *
 * Verify that `--version` outputs the version string and exits cleanly.
 */
#[test]
fn version_flag_long() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("--version")
        .timeout(std::time::Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("NodΞRunr"));

    Ok(())
}

/**
 * Version Flag (Short)
 *
 * Verify that `-V` outputs the version string and exits cleanly.
 */
#[test]
fn version_flag_short() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("-V")
        .timeout(std::time::Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("NodΞRunr"));

    Ok(())
}

/**
 * Help Flag
 *
 * Verify that `--help` outputs usage information and exits cleanly.
 */
#[test]
fn help_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("--help")
        .timeout(std::time::Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("effortless SysOps"));

    Ok(())
}
