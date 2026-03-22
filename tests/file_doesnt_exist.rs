// tests/file_doesnt_exist.rs

use assert_cmd::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("noderunr")?;

    cmd.arg("bar")
        .arg("bin/test.txt")
        .timeout(std::time::Duration::from_secs(10));

    let output = cmd.output()?;

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout_str.contains("███╗")
            || stdout_str.contains("NodΞRunr")
            || stdout_str.contains("NODERUNR")
            || stdout_str.contains("██████"),
        "Expected banner in stdout, got: {}",
        stdout_str
    );

    Ok(())
}
