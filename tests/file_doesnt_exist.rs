// tests/file_doesnt_exist.rs

use assert_cmd::Command;
use predicates::prelude::*; // Used for writing assertions // Run programs (with timeout support)

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("bar")
        .arg("bin/test.txt")
        .timeout(std::time::Duration::from_secs(10));

    cmd.assert().interrupted().stdout(
        predicate::str::contains("NodΞRunr")
            .or(predicate::str::contains("NODERUNR"))
            .or(predicate::str::contains("██████")),
    );

    Ok(())
}
