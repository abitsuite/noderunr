// src/cmd/sys.rs

/* Import modules. */
use super::pty_runner::run_interactive;
use std::process::Command;

pub fn df() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "wmic", "logicaldisk", "get", "size,freespace,caption"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("df").arg("-h").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn du() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let userprofile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users".to_string());
        let output = Command::new("cmd")
            .args(["/C", "dir", &userprofile, "/s", "/a"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let command = "du -hd 2 $HOME".to_string();

        let output = Command::new("sh").arg("-c").arg(command).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn ls() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let userprofile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users".to_string());
        let output = Command::new("cmd")
            .args(["/C", "dir", &userprofile, "/a"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let command = "ls $HOME -la".to_string();

        let output = Command::new("sh").arg("-c").arg(command).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn lsblk() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "wmic", "diskdrive", "list", "brief"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("lsblk").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn lscpu() -> Result<String, Box<dyn std::error::Error>> {
    let response;

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args([
                "/C",
                "wmic",
                "cpu",
                "get",
                "Name,NumberOfCores,NumberOfLogicalProcessors",
            ])
            .output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    } else {
        let output = Command::new("lscpu").arg("-e").output();
        // .expect("failed to execute lscpu");

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    }

    Ok(response)
}

pub fn lshw() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "systeminfo"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("lshw").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn mem() -> Result<String, Box<dyn std::error::Error>> {
    let response;

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args([
                "/C",
                "wmic",
                "OS",
                "get",
                "FreePhysicalMemory,TotalVisibleMemorySize",
            ])
            .output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    } else {
        let output = Command::new("free").arg("-h").output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    }

    Ok(response)
}

pub fn ps() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "tasklist"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("ps").arg("aux").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn get_uname() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "ver"]).output()?;

        let ver = String::from_utf8_lossy(&output.stdout).to_string();

        let arch_output = Command::new("cmd")
            .args(["/C", "echo", "%PROCESSOR_ARCHITECTURE%"])
            .output()?;

        let arch = String::from_utf8_lossy(&arch_output.stdout).to_string();

        Ok(format!("{} {}", ver.trim(), arch.trim()))
    } else {
        let output = Command::new("uname").arg("-a").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn get_uptime() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "net", "stats", "workstation"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("uptime").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/**
 * Get Release
 *
 * Request the system release details.
 */
pub fn get_release() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "ver"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = Command::new("uname").arg("-a").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/**
 * Install Golang
 *
 * Insall the latest stable release of Golang.
 */
pub fn install_golang() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("install_golang is not supported on Windows. Please install Go manually from https://go.dev/dl/".into());
    }

    let commands = vec![
        /* Change to (home) directory. */
        "cd".to_string(),
        // "echo \"export PATH=$PATH:$HOME/.noderunr/go/bin\" >> .profile".to_string(),
        /* Make (hidden) .noderunr directory (if required). */
        "mkdir -p .noderunr".to_string(),
        /* Change to noderunr directory. */
        "cd .noderunr".to_string(),
        // "wget https://go.dev/dl/go1.23.3.linux-amd64.tar.gz".to_string(),
        "export PATH=$PATH:$HOME/.noderunr/go/bin".to_string(),
        // "rm -rf $HOME/.noderunr/go && tar -C $HOME/.noderunr -xzf go1.23.3.linux-amd64.tar.gz".to_string(),
        "go version".to_string(),
    ];

    let response = run_interactive(&commands, |line| {
        println!("    ↳ {}", line);
    })?;

    Ok(response)
}

pub fn system_profiler() -> Result<String, Box<dyn std::error::Error>> {
    let response;

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "systeminfo"]).output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    } else if cfg!(target_os = "macos") {
        let output = Command::new("system_profiler")
            .arg("SPHardwareDataType")
            .output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    } else {
        let output = Command::new("lshw").output();

        match output {
            Ok(_output) => response = String::from_utf8_lossy(&_output.stdout).to_string(),
            Err(_err) => response = _err.to_string(),
        }
    }

    Ok(response)
}
