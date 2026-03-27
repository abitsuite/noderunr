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

// ===============================================================
// Structured system info functions for enhanced registration
// ===============================================================

/**
 * Get Hostname
 *
 * Returns the system hostname as a trimmed string.
 */
pub fn get_hostname() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "hostname"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let output = Command::new("hostname").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

/**
 * Get OS Pretty Name
 *
 * Returns a human-readable OS name, e.g. "Ubuntu 22.04.3 LTS".
 *   Linux:   reads PRETTY_NAME from /etc/os-release
 *   macOS:   uses sw_vers to build "macOS <ProductVersion> (<BuildVersion>)"
 *   Windows: uses "ver" command output
 */
pub fn get_os_pretty() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "ver"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else if cfg!(target_os = "macos") {
        let output = Command::new("sw_vers").output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        /* Parse sw_vers output into a single line. */
        let mut product_name = String::new();
        let mut product_version = String::new();
        let mut build_version = String::new();

        for line in raw.lines() {
            if let Some(val) = line.strip_prefix("ProductName:") {
                product_name = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("ProductVersion:") {
                product_version = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("BuildVersion:") {
                build_version = val.trim().to_string();
            }
        }

        Ok(format!(
            "{} {} ({})",
            product_name, product_version, build_version
        ))
    } else {
        /* Linux: read /etc/os-release */
        let content = std::fs::read_to_string("/etc/os-release");

        match content {
            Ok(text) => {
                for line in text.lines() {
                    if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                        /* Strip surrounding quotes. */
                        let trimmed = val.trim_matches('"').trim().to_string();
                        return Ok(trimmed);
                    }
                }

                /* Fallback: PRETTY_NAME not found. */
                Ok("Linux (unknown distribution)".to_string())
            }
            Err(_) => {
                /* Fallback: /etc/os-release not readable. */
                Ok("Linux (unknown distribution)".to_string())
            }
        }
    }
}

/**
 * Get Architecture
 *
 * Returns the CPU architecture string, e.g. "x86_64", "aarch64".
 */
pub fn get_arch() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let arch =
            std::env::var("PROCESSOR_ARCHITECTURE").unwrap_or_else(|_| "unknown".to_string());

        Ok(arch)
    } else {
        let output = Command::new("uname").arg("-m").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

/**
 * Get Kernel Version
 *
 * Returns the kernel version string, e.g. "6.5.0-44-generic".
 *   Linux/macOS: uname -r
 *   Windows:     ver output (same as OS pretty on Windows)
 */
pub fn get_kernel() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd").args(["/C", "ver"]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let output = Command::new("uname").arg("-r").output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

/**
 * Get Machine ID
 *
 * Returns a stable machine identifier for deduplication.
 *   Linux:   /etc/machine-id
 *   macOS:   IOPlatformUUID from ioreg
 *   Windows: wmic csproduct get UUID
 */
pub fn get_machine_id() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "wmic", "csproduct", "get", "UUID"])
            .output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        /* Skip the header line ("UUID") and get the value. */
        let uuid = raw
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim()
            .to_string();

        Ok(uuid)
    } else if cfg!(target_os = "macos") {
        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        /* Find the IOPlatformUUID line. */
        for line in raw.lines() {
            if line.contains("IOPlatformUUID") {
                /* Extract the UUID value between quotes. */
                if let Some(start) = line.rfind('"') {
                    let before = &line[..start];
                    if let Some(prev_quote) = before.rfind('"') {
                        let uuid = &line[prev_quote + 1..start];
                        return Ok(uuid.to_string());
                    }
                }
            }
        }

        Ok("".to_string())
    } else {
        /* Linux: /etc/machine-id */
        let content = std::fs::read_to_string("/etc/machine-id");

        match content {
            Ok(text) => Ok(text.trim().to_string()),
            Err(_) => Ok("".to_string()),
        }
    }
}

/**
 * Get CPU Model
 *
 * Returns the CPU model name string.
 *   Linux:   parses "Model name" from lscpu
 *   macOS:   sysctl -n machdep.cpu.brand_string
 *   Windows: wmic cpu get Name
 */
pub fn get_cpu_model() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "wmic", "cpu", "get", "Name"])
            .output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        let model = raw
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim()
            .to_string();

        Ok(model)
    } else if cfg!(target_os = "macos") {
        let output = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        /* Linux: parse lscpu output. */
        let output = Command::new("lscpu").output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        for line in raw.lines() {
            if line.starts_with("Model name:") {
                if let Some(val) = line.strip_prefix("Model name:") {
                    return Ok(val.trim().to_string());
                }
            }
        }

        Ok("".to_string())
    }
}

/**
 * Get CPU Cores
 *
 * Returns the number of logical CPU cores as a u32.
 *   Linux:   nproc
 *   macOS:   sysctl -n hw.ncpu
 *   Windows: NUMBER_OF_PROCESSORS env var
 */
pub fn get_cpu_cores() -> Result<u32, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let val = std::env::var("NUMBER_OF_PROCESSORS").unwrap_or_else(|_| "0".to_string());

        Ok(val.trim().parse::<u32>().unwrap_or(0))
    } else if cfg!(target_os = "macos") {
        let output = Command::new("sysctl").args(["-n", "hw.ncpu"]).output()?;

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(raw.parse::<u32>().unwrap_or(0))
    } else {
        let output = Command::new("nproc").output()?;

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(raw.parse::<u32>().unwrap_or(0))
    }
}

/**
 * Get Total Memory (MB)
 *
 * Returns total physical memory in megabytes.
 *   Linux:   reads MemTotal from /proc/meminfo (in kB, converts to MB)
 *   macOS:   sysctl -n hw.memsize (in bytes, converts to MB)
 *   Windows: wmic OS get TotalVisibleMemorySize (in kB, converts to MB)
 */
pub fn get_mem_total_mb() -> Result<u64, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "wmic", "OS", "get", "TotalVisibleMemorySize"])
            .output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        let kb_str = raw
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .unwrap_or("0")
            .trim();

        let kb = kb_str.parse::<u64>().unwrap_or(0);

        Ok(kb / 1024)
    } else if cfg!(target_os = "macos") {
        let output = Command::new("sysctl").args(["-n", "hw.memsize"]).output()?;

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let bytes = raw.parse::<u64>().unwrap_or(0);

        Ok(bytes / (1024 * 1024))
    } else {
        /* Linux: /proc/meminfo */
        let content = std::fs::read_to_string("/proc/meminfo")?;

        for line in content.lines() {
            if let Some(val) = line.strip_prefix("MemTotal:") {
                let trimmed = val.trim().trim_end_matches("kB").trim();
                let kb = trimmed.parse::<u64>().unwrap_or(0);
                return Ok(kb / 1024);
            }
        }

        Ok(0)
    }
}

/**
 * Get Total Disk (GB)
 *
 * Returns total disk size of the root filesystem in gigabytes.
 *   Linux:   df / --output=size (in 1K blocks, converts to GB)
 *   macOS:   df / (parses 512-byte blocks column, converts to GB)
 *   Windows: wmic logicaldisk where DeviceID='C:' get Size (bytes to GB)
 */
pub fn get_disk_total_gb() -> Result<u64, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args([
                "/C",
                "wmic",
                "logicaldisk",
                "where",
                "DeviceID='C:'",
                "get",
                "Size",
            ])
            .output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        let bytes_str = raw
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .unwrap_or("0")
            .trim();

        let bytes = bytes_str.parse::<u64>().unwrap_or(0);

        Ok(bytes / (1024 * 1024 * 1024))
    } else {
        /* Linux and macOS: use df with POSIX output. */
        let output = Command::new("df").args(["-k", "/"]).output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        /* Skip header line, parse first data line. */
        if let Some(data_line) = raw.lines().nth(1) {
            let parts: Vec<&str> = data_line.split_whitespace().collect();

            /* Column index 1 is total 1K-blocks on both Linux and macOS. */
            if parts.len() >= 2 {
                let kb = parts[1].parse::<u64>().unwrap_or(0);
                return Ok(kb / (1024 * 1024));
            }
        }

        Ok(0)
    }
}

/**
 * Get Disk Used Percent
 *
 * Returns the usage percentage of the root filesystem as a u8 (0-100).
 */
pub fn get_disk_used_pct() -> Result<u8, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let size_output = Command::new("cmd")
            .args([
                "/C",
                "wmic",
                "logicaldisk",
                "where",
                "DeviceID='C:'",
                "get",
                "Size",
            ])
            .output()?;

        let free_output = Command::new("cmd")
            .args([
                "/C",
                "wmic",
                "logicaldisk",
                "where",
                "DeviceID='C:'",
                "get",
                "FreeSpace",
            ])
            .output()?;

        let size_raw = String::from_utf8_lossy(&size_output.stdout).to_string();
        let free_raw = String::from_utf8_lossy(&free_output.stdout).to_string();

        let parse_second_line = |raw: &str| -> u64 {
            raw.lines()
                .skip(1)
                .find(|l| !l.trim().is_empty())
                .unwrap_or("0")
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        };

        let size = parse_second_line(&size_raw);
        let free = parse_second_line(&free_raw);

        if size == 0 {
            return Ok(0);
        }

        let used = size.saturating_sub(free);
        let pct = (used as f64 / size as f64 * 100.0) as u8;

        Ok(pct)
    } else {
        /* Linux and macOS: parse df output for Use%. */
        let output = Command::new("df").args(["-k", "/"]).output()?;

        let raw = String::from_utf8_lossy(&output.stdout).to_string();

        if let Some(data_line) = raw.lines().nth(1) {
            let parts: Vec<&str> = data_line.split_whitespace().collect();

            /* Find the column that ends with '%'. */
            for part in &parts {
                if part.ends_with('%') {
                    let num_str = part.trim_end_matches('%');
                    let pct = num_str.parse::<u8>().unwrap_or(0);
                    return Ok(pct);
                }
            }
        }

        Ok(0)
    }
}

/**
 * Get Load Average
 *
 * Returns (load1, load5, load15) as a tuple of f64.
 *   Linux:   reads /proc/loadavg
 *   macOS:   sysctl -n vm.loadavg
 *   Windows: returns (0.0, 0.0, 0.0) — not natively available
 */
pub fn get_load_avg() -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        /* Load average is not a native concept on Windows. */
        Ok((0.0, 0.0, 0.0))
    } else if cfg!(target_os = "macos") {
        let output = Command::new("sysctl").args(["-n", "vm.loadavg"]).output()?;

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();

        /* sysctl output: "{ 0.52 0.48 0.45 }" — strip braces. */
        let cleaned = raw.trim_start_matches('{').trim_end_matches('}').trim();

        let parts: Vec<&str> = cleaned.split_whitespace().collect();

        let l1 = parts
            .first()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let l5 = parts
            .get(1)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let l15 = parts
            .get(2)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok((l1, l5, l15))
    } else {
        /* Linux: /proc/loadavg format: "0.52 0.48 0.45 1/234 5678" */
        let content = std::fs::read_to_string("/proc/loadavg")?;

        let parts: Vec<&str> = content.split_whitespace().collect();

        let l1 = parts
            .first()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let l5 = parts
            .get(1)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let l15 = parts
            .get(2)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok((l1, l5, l15))
    }
}

/**
 * Service Status
 *
 * Returned by detect_service() — describes whether a known
 * service is installed and/or running on this system.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceStatus {
    pub name: String,
    pub display_name: String,
    pub installed: bool,
    pub running: bool,
    pub version: String,
    pub pid: u32,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus {
            name: String::new(),
            display_name: String::new(),
            installed: false,
            running: false,
            version: String::new(),
            pid: 0,
        }
    }
}

/**
 * Detect Service
 *
 * Checks whether a named service binary is installed (on PATH or known location)
 * and whether it is currently running.
 *
 * Supported services:
 *   "avalanchego"  — Avalanche node daemon
 *   "nexad"        — Nexa node daemon (https://nexa.org)
 *   "dashd"        — Dash node daemon (https://dash.org)
 *
 * Returns a ServiceStatus struct.
 * On Windows, returns not-installed/not-running (detection not yet implemented).
 */
pub fn detect_service(service_name: &str) -> ServiceStatus {
    if cfg!(target_os = "windows") {
        return ServiceStatus {
            name: service_name.to_string(),
            display_name: display_name_for(service_name).to_string(),
            ..Default::default()
        };
    }

    let display = display_name_for(service_name);

    /* Check if binary is on PATH via `which`. */
    let which_output = Command::new("which").arg(service_name).output();

    let installed = match &which_output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    };

    /* Check if process is running via `pgrep`. */
    let pgrep_output = Command::new("pgrep").arg("-x").arg(service_name).output();

    let (running, pid) = match &pgrep_output {
        Ok(out) => {
            if out.status.success() {
                let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();

                /* pgrep may return multiple PIDs — take the first. */
                let first_pid = raw
                    .lines()
                    .next()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(0);

                (true, first_pid)
            } else {
                (false, 0)
            }
        }
        Err(_) => (false, 0),
    };

    /* Attempt to get version via `<binary> --version`. */
    let version = if installed {
        let ver_output = Command::new(service_name).arg("--version").output();

        match ver_output {
            Ok(out) => {
                let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();

                if raw.is_empty() {
                    /* Some binaries print version to stderr. */
                    String::from_utf8_lossy(&out.stderr).trim().to_string()
                } else {
                    raw
                }
            }
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    ServiceStatus {
        name: service_name.to_string(),
        display_name: display.to_string(),
        installed,
        running,
        version,
        pid,
    }
}

/**
 * Detect All Known Services
 *
 * Runs detect_service() for each known service and returns the results.
 */
pub fn detect_all_services() -> Vec<ServiceStatus> {
    let known_services = ["avalanchego", "nexad", "dashd"];

    known_services
        .iter()
        .map(|name| detect_service(name))
        .collect()
}

/**
 * Display Name For
 *
 * Maps a binary/process name to a human-readable display name.
 */
fn display_name_for(service_name: &str) -> &str {
    match service_name {
        "avalanchego" => "Avalanche",
        "nexad" => "Nexa",
        "dashd" => "Dash",
        _ => service_name,
    }
}
