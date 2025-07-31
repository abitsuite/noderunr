/* Import modules. */
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use interactive_process::InteractiveProcess;

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
    let stdout = child.stdout
        .as_mut()
        .expect("Oops! Failed to initialize output for child.");

    /* Initialize intput buffer. */
    let stdout_reader = BufReader::new(stdout);

    /* Handle line inputs. */
    let stdout_lines = stdout_reader
        .lines();

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

    let mut stdout = child.stdout.as_mut().unwrap();
    let mut stderr = child.stderr.as_mut().unwrap();

    // let stdout = String::from_utf8(stdout).unwrap();
    println!("{:?}", stdout);

    // io::stdout().write_all(&mut stdout).unwrap();
    // io::stderr().write_all(&mut stderr).unwrap();
}

pub fn avax() -> Result<String, Box<dyn std::error::Error>> {
    /* Initialize locals. */
    let mut response;

    let output = Command::new("avalanche")
        .arg("--help")
        .output();

    match output {
        Ok(ref out) => {
            response = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
        },
        Err(ref err) => {
            response = format!("ERROR: {:?}", err.to_string());
        },
    };

    Ok(response)
}

pub fn avax_install() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("/usr/bin/bash");

    let mut proc = InteractiveProcess::new_with_exit_callback(
        &mut cmd, 
        |line| {
            println!("    ↳ {}", line.unwrap());
        },
        || {
            println!("\n    ✨ Avalanche has been successfully installed! ✨\n");
            avax_test();
        }
    ).unwrap();

    /* Quick pause */
    sleep(Duration::from_millis(10));

    /* Make (hidden) .noderunr directory (if required). */
    proc.send("mkdir -p $HOME/.noderunr/bin").unwrap();

    /* Short pause */
    sleep(Duration::from_secs(1));

    /* Change to noderunr directory. */
    proc.send("cd $HOME/.noderunr/bin").unwrap();

    /* Short pause */
    sleep(Duration::from_secs(1));

    proc.send("curl -sSfL https://raw.githubusercontent.com/ava-labs/avalanche-cli/main/scripts/install.sh | sh -s -- -b ./").unwrap();

    /* Quick pause */
    sleep(Duration::from_millis(10));

    // We're done with the process
    proc.close().kill().unwrap();

    Ok(response)
}

fn avax_test() -> Result<String, Box<dyn std::error::Error>> {
println!("Starting AVAX test...");
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("/usr/bin/bash");

    let mut proc = InteractiveProcess::new_with_exit_callback(
        &mut cmd, |line| {
            println!("    ↳ {}", line.unwrap());
        },
        || println!("Child exited.")
    ).unwrap();

    /* Quick pause */
    sleep(Duration::from_millis(10));

    proc.send("$HOME/.noderunr/bin/avalanche --help").unwrap();

    /* Short pause */
    sleep(Duration::from_secs(1));
    // sleep(Duration::from_millis(10));

    proc.send("$HOME/.noderunr/bin/avalanche --version").unwrap();

    /* Quick pause */
    sleep(Duration::from_millis(10));

    // We're done with the process
    proc.close().kill().unwrap();

    Ok(response)
}

pub fn avax_start() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("bash");

    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("    ↳ {}", line.unwrap());
    })
    .unwrap();

    proc.send("$HOME/.noderunr/bin/avalanche network start").unwrap();
    sleep(Duration::from_millis(10));

    proc.close().kill().unwrap();

    Ok(response)
}

pub fn avax_status() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("bash");

    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("    ↳ {}", line.unwrap());
    })
    .unwrap();

    proc.send("$HOME/.noderunr/bin/avalanche network status").unwrap();
    sleep(Duration::from_millis(10));

    proc.close().kill().unwrap();

    Ok(response)
}

pub fn avax_stop() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("bash");

    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("    ↳ {}", line.unwrap());
    })
    .unwrap();

    proc.send("$HOME/.noderunr/bin/avalanche network stop").unwrap();
    sleep(Duration::from_millis(10));

    proc.close().kill().unwrap();

    Ok(response)
}

pub fn build_avalanche() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("bash");

    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("    ↳ {}", line.unwrap());
    })
    .unwrap();

    /* Change to (home) directory. */
    proc.send("cd").unwrap();
    sleep(Duration::from_secs(1));

    /* Make (hidden) .noderunr directory (if required). */
    proc.send("mkdir -p .noderunr").unwrap();
    sleep(Duration::from_secs(1));

    /* Change to noderunr directory. */
    proc.send("cd .noderunr").unwrap();
    sleep(Duration::from_secs(1));

    // proc.send("git clone https://github.com/ava-labs/avalanchego.git").unwrap();
    // sleep(Duration::from_millis(1));

    proc.send("cd avalanchego").unwrap();
    sleep(Duration::from_secs(1));

    proc.send("export PATH=$PATH:$HOME/.noderunr/go/bin").unwrap();
    sleep(Duration::from_secs(1));

    proc.send("./scripts/build.sh").unwrap();
    sleep(Duration::from_millis(1));

    proc.send("./build/avalanchego").unwrap();
    sleep(Duration::from_millis(1));

    /// We're done with the process, but it is not self-terminating,
    /// so we can't use `proc.wait()`. Instead, we'll take the `Child` from
    /// the `InteractiveProcess` and kill it ourselves.
    proc.close().kill().unwrap();

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
