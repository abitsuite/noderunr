// src/cmd/network.rs

/* Import modules. */
use super::pty_runner::{run_interactive, run_interactive_with_exit_callback};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

/// Returns the shell command name appropriate for the current OS.
/// On Windows this is "cmd", on Unix-like systems this is "bash".
#[allow(dead_code)]
fn shell_cmd() -> &'static str {
    if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "bash"
    }
}

/// Returns the shell prefix args appropriate for the current OS.
/// On Windows: ["/C"], on Unix-like: ["-c"] (only used for one-shot commands).
#[allow(dead_code)]
fn shell_prefix() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        &["/C"]
    } else {
        &["-c"]
    }
}

/// Returns the default Avalanche CLI binary path.
pub(crate) fn avax_cli_bin() -> String {
    "$HOME/.noderunr/bin/avalanche".to_string()
}

/// Returns the default noderunr home directory path.
pub(crate) fn noderunr_home() -> String {
    "$HOME/.noderunr".to_string()
}

/// Returns the default noderunr bin directory path.
pub(crate) fn noderunr_bin_dir() -> String {
    format!("{}/bin", noderunr_home())
}

/// Builds the sequence of shell commands for avax_install.
/// Returns a Vec of commands to be joined and run.
pub(crate) fn build_avax_install_steps() -> Vec<(String, u64)> {
    let bin_dir = noderunr_bin_dir();

    vec![
        (format!("mkdir -p {}", bin_dir), 1000),
        (format!("cd {}", bin_dir), 1000),
        ("curl -sSfL https://raw.githubusercontent.com/ava-labs/avalanche-cli/main/scripts/install.sh | sh -s -- -b ./".to_string(), 10),
    ]
}

/// Builds the sequence of shell commands for avax_test.
/// Returns a Vec of commands to be joined and run.
pub(crate) fn build_avax_test_steps() -> Vec<(String, u64)> {
    let cli_bin = avax_cli_bin();

    vec![
        (format!("{} --help", cli_bin), 1000),
        (format!("{} --version", cli_bin), 10),
    ]
}

/// Builds the shell command for avax_start.
pub(crate) fn build_avax_start_cmd() -> String {
    format!("{} network start", avax_cli_bin())
}

/// Builds the shell command for avax_status.
pub(crate) fn build_avax_status_cmd() -> String {
    format!("{} network status", avax_cli_bin())
}

/// Builds the shell command for avax_stop.
pub(crate) fn build_avax_stop_cmd() -> String {
    format!("{} network stop", avax_cli_bin())
}

/// Builds the sequence of shell commands for build_avalanche.
/// Returns a Vec of commands to be joined and run.
pub(crate) fn build_avalanche_steps() -> Vec<(String, u64)> {
    vec![
        ("cd".to_string(), 1000),
        ("mkdir -p .noderunr".to_string(), 1000),
        ("cd .noderunr".to_string(), 1000),
        ("cd avalanchego".to_string(), 1000),
        ("export PATH=$PATH:$HOME/.noderunr/go/bin".to_string(), 1000),
        ("./scripts/build.sh".to_string(), 1),
        ("./build/avalanchego".to_string(), 1),
    ]
}

/// Helper: extracts just the command strings from a steps Vec.
fn steps_to_commands(steps: &[(String, u64)]) -> Vec<String> {
    steps.iter().map(|(cmd, _)| cmd.clone()).collect()
}

/**
 * Ping
 *
 * Starts a long-lived ping process on the provided destination.
 */
pub fn ping() {
    let mut child = Command::new("ping")
        .arg("www.yahoo.com")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Oops! Failed to execute child.");

    /* Initialize output for child. */
    let stdout = child
        .stdout
        .as_mut()
        .expect("Oops! Failed to initialize output for child.");

    /* Initialize intput buffer. */
    let stdout_reader = BufReader::new(stdout);

    /* Handle line inputs. */
    let stdout_lines = stdout_reader.lines();

    /* Handle output reader buffer. */
    for line in stdout_lines {
        println!("Read -> {:?}", line);
    }

    /* Wait for child. */
    // TODO: How can we retrieve the final output?
    let output = child
        .wait_with_output()
        .expect("Oops! Failed to wait for child.");
    assert!(output.status.success());
    println!("Final output -> {:?}\n", output);
}

pub fn ping2() {
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let stdout = child.stdout.as_mut().unwrap();
    let _stderr = child.stderr.as_mut().unwrap();

    // let stdout = String::from_utf8(stdout).unwrap();
    println!("{:?}", stdout);

    // io::stdout().write_all(&mut stdout).unwrap();
    // io::stderr().write_all(&mut stderr).unwrap();

    /* Wait for child to prevent zombie process. */
    let _ = child.wait();
}

pub fn avax() -> Result<String, Box<dyn std::error::Error>> {
    /* Initialize locals. */
    let output = Command::new("avalanche").arg("--help").output();

    let response = match output {
        Ok(ref _out) => String::from_utf8_lossy(&output.unwrap().stdout).to_string(),
        Err(ref err) => {
            format!("ERROR: {:?}", err.to_string())
        }
    };

    Ok(response)
}

pub fn avax_install() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err(
            "avax_install is not supported on Windows. Please install Avalanche CLI manually."
                .into(),
        );
    }

    let steps = build_avax_install_steps();
    let commands = steps_to_commands(&steps);

    let response = run_interactive_with_exit_callback(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
        || {
            println!("\n    ✨ Avalanche has been successfully installed! ✨\n");
            let _ = avax_test();
        },
    )?;

    Ok(response)
}

fn avax_test() -> Result<String, Box<dyn std::error::Error>> {
    println!("Starting AVAX test...");

    if cfg!(target_os = "windows") {
        return Err("avax_test is not supported on Windows.".into());
    }

    let steps = build_avax_test_steps();
    let commands = steps_to_commands(&steps);

    let response = run_interactive(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
    )?;

    Ok(response)
}

pub fn avax_start() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("avax_start is not supported on Windows.".into());
    }

    let commands = vec![build_avax_start_cmd()];

    let response = run_interactive(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
    )?;

    Ok(response)
}

pub fn avax_status() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("avax_status is not supported on Windows.".into());
    }

    let commands = vec![build_avax_status_cmd()];

    let response = run_interactive(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
    )?;

    Ok(response)
}

pub fn avax_stop() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("avax_stop is not supported on Windows.".into());
    }

    let commands = vec![build_avax_stop_cmd()];

    let response = run_interactive(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
    )?;

    Ok(response)
}

pub fn build_avalanche() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("build_avalanche is not supported on Windows.".into());
    }

    let steps = build_avalanche_steps();
    let commands = steps_to_commands(&steps);

    let response = run_interactive(
        &commands,
        |line| {
            println!("    ↳ {}", line);
        },
    )?;

    // let cmd1 = Command::new("cd")
    //     .arg("/tmp");

    // let cmd2 = Command::new("mkdir")
    //     .arg("noderunr");

    // let cmd3 = Command::new("cd")
    //     .arg("noderunr");

    // let cmd4 = Command::new("git")
    //     .arg("clone")
    //     .arg("https://github.com/ava-labs/avalanchego.git");

    // let cmd = cmd1
    //     .command("&&").unwrap()
    //     .join(cmd2)
    //     .command("&&").unwrap()
    //     .join(cmd3)
    //     .command("&&").unwrap()
    //     .join(cmd4);

    // let output = cmd.output();

    // match output {
    //     Ok(ref out) => {
    //         response = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
    //     },
    //     Err(ref err) => {
    //         response = format!("ERROR! {:?}", err.to_string());
    //     },
    // };

    Ok(response)
}

// pub fn avalanche_check_config() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/???
// }

// pub fn avalanche_check_db_mainnet() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/db/mainnet
// }

// pub fn avalanche_check_logs() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/logs
// }

// pub fn avalanche_check_plugins() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/plugins
// }

// pub fn avalanche_check_profiles() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/profiles
// }

// pub fn avalanche_check_staking() -> Result<String, Box<dyn std::error::Error>> {
//     // TODO /home/dev/.avalanchego/staking
// }

#[cfg(test)]
#[path = "network.test.rs"]
mod network_test;
