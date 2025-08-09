use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
// use uuid::Uuid;

use crate::api;
use crate::comm;
use crate::cmd;
use crate::utils;

#[derive(Serialize)]
struct Registration {
    method: String,
    ip: String,
    release: String,
    uptime: String,
    cpu: String,
    mem: String,
    profile: String,
}

#[derive(Debug, Default, Deserialize)]
struct RegistrationResponse {
    success: bool,
    result: Session,
}

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
struct Session {
    sessionid: String,
    hasAuth: bool,
    createdAt: u32, // seconds
}


/**
 * New Session
 *
 * Request a new session from the API server.
 */
pub fn new() -> String {
    /* Initialize locals. */
    let mut ip;

    /* Request IP address. */
    let response = utils::ip::get();

    /* Set IP address. */
    match response {
        Ok(_resp) => {
            ip = _resp["origin"].clone();
        },
        Err(err) => {
            ip = err.to_string();
        }
    }    
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
    let pkg = Registration {
        method: "reg".to_string(),
        ip: ip.to_string(),
        release: release.to_string(),
        uptime: uptime.to_string(),
        cpu: cpu.to_string(),
        mem: mem.to_string(),
        profile: profile.to_string(),
    };

    /* Encode to JSON. */
    let json_string = to_string(&pkg).unwrap();

    /* Make (remote) request. */
    let response = api::call("session", &json_string);

    let mut reg_response: Result<RegistrationResponse, serde_json::Error> = Ok(RegistrationResponse::default());

    /* Parse (registration) response. */
    match(&response) {
        Ok(_data) => {
            reg_response = from_str(_data);
        },
        Err(_) => println!("  Ugh! Your node registration failed!\n  Sorry about that, please try again...\n\n")
    }

    let mut registration: RegistrationResponse = RegistrationResponse::default();

    /* Parse (registration) response. */
    match(reg_response) {
        Ok(_data) => {
            /* Set registation (result). */
            registration = _data;

            /* Set session id. */
            let sessionid = registration.result.sessionid;

            println!("  NEW session created successfully!\n");
            println!("  [ {} ]\n", sessionid);

            println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
            println!("  https://L1.run/sid/#/{}", sessionid);

            /* Start monitoring session. */
            comm::monitor::by_session(&sessionid);

            /* Return session ID. */
            sessionid.to_string()
        },
        Err(_) => ("".to_string())
    }

}
