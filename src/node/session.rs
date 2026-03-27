// src/node/session.rs

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use noderunr::api;
use noderunr::cmd;
use noderunr::comm;
use noderunr::utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Registration {
    pub(crate) method: String,

    // --- Identity ---
    pub(crate) hostname: String,
    pub(crate) os: String,
    pub(crate) arch: String,
    pub(crate) kernel: String,
    pub(crate) machine_id: String,

    // --- Hardware ---
    pub(crate) ip: String,
    pub(crate) cpu_model: String,
    pub(crate) cpu_cores: u32,
    pub(crate) mem_total_mb: u64,
    pub(crate) disk_total_gb: u64,

    // --- Runtime snapshot ---
    pub(crate) uptime: String,
    pub(crate) disk_used_pct: u8,
    pub(crate) load_avg: (f64, f64, f64),

    // --- Services ---
    pub(crate) services: Vec<ServiceSnapshot>,

    // --- Raw legacy fields (preserved for backward compat) ---
    pub(crate) release: String,
    pub(crate) cpu: String,
    pub(crate) mem: String,
    pub(crate) profile: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct ServiceSnapshot {
    pub(crate) name: String,
    pub(crate) display_name: String,
    pub(crate) installed: bool,
    pub(crate) running: bool,
    pub(crate) version: String,
    pub(crate) pid: u32,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub(crate) struct RegistrationResponse {
    pub(crate) success: bool,
    pub(crate) result: Session,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub(crate) struct Session {
    pub(crate) sessionid: String,
    pub(crate) hasAuth: bool,
    pub(crate) createdAt: u32, // seconds
}

/**
 * Build Registration
 *
 * Pure function: builds a Registration struct from the given system info.
 * Contains NO network I/O and is fully testable.
 */
pub(crate) fn build_registration(
    hostname: &str,
    os: &str,
    arch: &str,
    kernel: &str,
    machine_id: &str,
    ip: &str,
    cpu_model: &str,
    cpu_cores: u32,
    mem_total_mb: u64,
    disk_total_gb: u64,
    uptime: &str,
    disk_used_pct: u8,
    load_avg: (f64, f64, f64),
    services: Vec<ServiceSnapshot>,
    release: &str,
    cpu: &str,
    mem: &str,
    profile: &str,
) -> Registration {
    Registration {
        method: "reg".to_string(),
        hostname: hostname.to_string(),
        os: os.to_string(),
        arch: arch.to_string(),
        kernel: kernel.to_string(),
        machine_id: machine_id.to_string(),
        ip: ip.to_string(),
        cpu_model: cpu_model.to_string(),
        cpu_cores,
        mem_total_mb,
        disk_total_gb,
        uptime: uptime.to_string(),
        disk_used_pct,
        load_avg,
        services,
        release: release.to_string(),
        cpu: cpu.to_string(),
        mem: mem.to_string(),
        profile: profile.to_string(),
    }
}

/**
 * Serialize Registration
 *
 * Pure function: serializes a Registration to a JSON string.
 */
pub(crate) fn serialize_registration(pkg: &Registration) -> String {
    to_string(pkg).unwrap()
}

/**
 * Parse Registration Response
 *
 * Pure function: parses a JSON string into a RegistrationResponse.
 * Returns Err if the JSON is invalid.
 */
pub(crate) fn parse_registration_response(
    data: &str,
) -> Result<RegistrationResponse, serde_json::Error> {
    from_str(data)
}

/**
 * Extract IP
 *
 * Pure function: extracts the IP string from the ip::get() result.
 */
pub(crate) fn extract_ip(
    response: Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>>,
) -> String {
    match response {
        Ok(resp) => resp.get("origin").cloned().unwrap_or_default(),
        Err(err) => err.to_string(),
    }
}

/**
 * Collect System Info
 *
 * Gathers all structured system information for registration.
 * Separated from new() so it can be unit tested independently
 * (it still calls cmd::sys functions, but no network I/O).
 */
pub(crate) fn collect_system_info(
    rt: &tokio::runtime::Runtime,
) -> (
    String,               // hostname
    String,               // os
    String,               // arch
    String,               // kernel
    String,               // machine_id
    String,               // ip
    String,               // cpu_model
    u32,                  // cpu_cores
    u64,                  // mem_total_mb
    u64,                  // disk_total_gb
    String,               // uptime
    u8,                   // disk_used_pct
    (f64, f64, f64),      // load_avg
    Vec<ServiceSnapshot>, // services
    String,               // release (raw)
    String,               // cpu (raw)
    String,               // mem (raw)
    String,               // profile (raw)
) {
    /* Request IP address. */
    let ip_response = rt.block_on(utils::ip::get());
    let ip = extract_ip(ip_response);
    // println!("\nIP -> {:?}", ip);

    /* Structured identity fields. */
    let hostname = cmd::sys::get_hostname().unwrap_or_default();
    let os = cmd::sys::get_os_pretty().unwrap_or_default();
    let arch = cmd::sys::get_arch().unwrap_or_default();
    let kernel = cmd::sys::get_kernel().unwrap_or_default();
    let machine_id = cmd::sys::get_machine_id().unwrap_or_default();

    /* Structured hardware fields. */
    let cpu_model = cmd::sys::get_cpu_model().unwrap_or_default();
    let cpu_cores = cmd::sys::get_cpu_cores().unwrap_or(0);
    let mem_total_mb = cmd::sys::get_mem_total_mb().unwrap_or(0);
    let disk_total_gb = cmd::sys::get_disk_total_gb().unwrap_or(0);

    /* Runtime snapshot. */
    let uptime = cmd::sys::get_uptime().unwrap_or_default();
    let disk_used_pct = cmd::sys::get_disk_used_pct().unwrap_or(0);
    let load_avg = cmd::sys::get_load_avg().unwrap_or((0.0, 0.0, 0.0));

    /* Detect installed/running services. */
    let detected = cmd::sys::detect_all_services();
    let services: Vec<ServiceSnapshot> = detected
        .iter()
        .map(|s| ServiceSnapshot {
            name: s.name.clone(),
            display_name: s.display_name.clone(),
            installed: s.installed,
            running: s.running,
            version: s.version.clone(),
            pid: s.pid,
        })
        .collect();

    /* Raw legacy fields (preserved for backward compat). */
    let release = cmd::sys::get_release().unwrap_or_default();
    let cpu = cmd::sys::lscpu().unwrap_or_default();
    let mem = cmd::sys::mem().unwrap_or_default();
    let profile = cmd::sys::system_profiler().unwrap_or_default();

    (
        hostname,
        os,
        arch,
        kernel,
        machine_id,
        ip,
        cpu_model,
        cpu_cores,
        mem_total_mb,
        disk_total_gb,
        uptime,
        disk_used_pct,
        load_avg,
        services,
        release,
        cpu,
        mem,
        profile,
    )
}

/**
 * New Session
 *
 * Request a new session from the API server.
 */
pub fn new(rt: &tokio::runtime::Runtime) -> String {
    /* Collect all system information. */
    let (
        hostname,
        os,
        arch,
        kernel,
        machine_id,
        ip,
        cpu_model,
        cpu_cores,
        mem_total_mb,
        disk_total_gb,
        uptime,
        disk_used_pct,
        load_avg,
        services,
        release,
        cpu,
        mem,
        profile,
    ) = collect_system_info(rt);

    /* Build (registration) package. */
    let pkg = build_registration(
        &hostname,
        &os,
        &arch,
        &kernel,
        &machine_id,
        &ip,
        &cpu_model,
        cpu_cores,
        mem_total_mb,
        disk_total_gb,
        &uptime,
        disk_used_pct,
        load_avg,
        services,
        &release,
        &cpu,
        &mem,
        &profile,
    );

    /* Encode to JSON. */
    let json_string = serialize_registration(&pkg);

    /* Make (remote) request. */
    let response = rt.block_on(api::call("session", &json_string));

    let mut reg_response: Result<RegistrationResponse, serde_json::Error> =
        Ok(RegistrationResponse::default());

    /* Parse (registration) response. */
    match (&response) {
        Ok(_data) => {
            reg_response = parse_registration_response(_data);
        }
        Err(_) => println!(
            "  Ugh! Your node registration failed!\n  Sorry about that, please try again...\n\n"
        ),
    }

    let mut registration: RegistrationResponse = RegistrationResponse::default();

    /* Parse (registration) response. */
    match (reg_response) {
        Ok(_data) => {
            /* Set registation (result). */
            registration = _data;

            /* Set session id. */
            let sessionid = registration.result.sessionid;

            println!("  NEW session created successfully!\n");
            println!("  [ {} ]\n", sessionid);

            println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
            println!("  https://L1.run/id/{}", sessionid);

            /* Start monitoring session. */
            comm::monitor::by_session(rt, &sessionid);

            /* Return session ID. */
            sessionid.to_string()
        }
        Err(_) => ("".to_string()),
    }
}

#[cfg(test)]
#[path = "session.test.rs"]
mod session_test;
