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
    pub(crate) ip: String,
    pub(crate) release: String,
    pub(crate) uptime: String,
    pub(crate) cpu: String,
    pub(crate) mem: String,
    pub(crate) profile: String,
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
    ip: &str,
    release: &str,
    uptime: &str,
    cpu: &str,
    mem: &str,
    profile: &str,
) -> Registration {
    Registration {
        method: "reg".to_string(),
        ip: ip.to_string(),
        release: release.to_string(),
        uptime: uptime.to_string(),
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
 * New Session
 *
 * Request a new session from the API server.
 */
pub fn new(rt: &tokio::runtime::Runtime) -> String {
    /* Request IP address. */
    let response = rt.block_on(utils::ip::get());

    /* Set IP address. */
    let ip = extract_ip(response);
    // println!("\nIP -> {:?}", ip);

    /* Request release. */
    let release = cmd::sys::get_release().unwrap();

    /* Request uptime. */
    let uptime = cmd::sys::get_uptime().unwrap();

    /* Request cpu. */
    let cpu = cmd::sys::lscpu().unwrap();

    /* Request mem. */
    let mem = cmd::sys::mem().unwrap();

    /* Request system profile. */
    let profile = cmd::sys::system_profiler().unwrap();

    /* Build (registration) package. */
    let pkg = build_registration(&ip, &release, &uptime, &cpu, &mem, &profile);

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
