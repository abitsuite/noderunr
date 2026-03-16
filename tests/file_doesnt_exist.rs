use predicates::prelude::*; // Used for writing assertions
use assert_cmd::Command; // Run programs (with timeout support)

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("bar")
        .arg("bin/test.txt")
        .timeout(std::time::Duration::from_secs(10));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
