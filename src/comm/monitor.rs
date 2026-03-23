// src/comm/monitor.rs

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use std::sync::atomic::{AtomicU64, Ordering};
use std::{thread, time};

use crate::cmd;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Action {
    pub(crate) actionid: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) created_at: u64, // milliseconds
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ExecResponse {
    pub(crate) sessionid: String,
    pub(crate) method: String,
    pub(crate) resp: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Log {
    pub(crate) body: String,
    pub(crate) created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Request {
    pub(crate) exec: String,
    pub(crate) created_at: u64, // milliseconds
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Session {
    pub(crate) sessionid: String,
    pub(crate) act: Option<Vec<Action>>,
    pub(crate) log: Option<Vec<Action>>,
    pub(crate) req: Option<Vec<Request>>,
    pub(crate) res: Option<Vec<Action>>,
    pub(crate) rpt: Option<Vec<Action>>,
    pub(crate) created_at: u32, // seconds
    pub(crate) last_since: u64, // milliseconds
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub(crate) struct SessionRequest {
    pub(crate) sessionid: String,
    pub(crate) since: u64, // milliseconds
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct SessionResponse {
    pub(crate) success: bool,
    pub(crate) result: Session,
}

/* Initialize constants. */
pub(crate) const L1_ENDPOINT: &str = "https://l1.run/v1/";

/* Initialize globals. */
static LAST_SINCE: AtomicU64 = AtomicU64::new(1);

/**
 * Build Request URL
 *
 * Constructs the full URL for a session request.
 */
#[cfg(test)]
pub(crate) fn build_request_url(since: u64) -> String {
    build_request_url_with_base(L1_ENDPOINT, since)
}

/**
 * Build Request URL (with base)
 *
 * Constructs the full URL for a session request using a custom base URL.
 */
pub(crate) fn build_request_url_with_base(base_url: &str, since: u64) -> String {
    format!("{}{}/{}", base_url, "session", since)
}

/**
 * Build Auth Header
 *
 * Constructs the bearer authorization header value.
 */
pub(crate) fn build_auth_header(sessionid: &str) -> String {
    format!("{} {}", "Bearer", sessionid)
}

/**
 * Build Response URL
 *
 * Constructs the full URL for a session response post.
 */
#[cfg(test)]
pub(crate) fn build_response_url() -> String {
    build_response_url_with_base(L1_ENDPOINT)
}

/**
 * Build Response URL (with base)
 *
 * Constructs the full URL for a session response post using a custom base URL.
 */
pub(crate) fn build_response_url_with_base(base_url: &str) -> String {
    format!("{}{}", base_url, "session")
}

/**
 * Build Exec Response JSON
 *
 * Constructs the JSON string for an exec response.
 */
pub(crate) fn build_exec_response_json(sessionid: &str, response: &str) -> String {
    let exec_response = ExecResponse {
        sessionid: sessionid.to_string(),
        method: "res".to_string(),
        resp: response.to_string(),
    };

    to_string(&exec_response).unwrap()
}

/**
 * Request JSON (async, with base URL)
 *
 * Make a (remote) API call using a custom base URL.
 * This is the testable core; request_json_async delegates to it.
 */
pub(crate) async fn request_json_async_with_base(
    base_url: &str,
    _sessionid: &str,
    _since: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = build_request_url_with_base(base_url, _since);

    /* Set bearer authorization. */
    let auth = build_auth_header(_sessionid);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let response_body = response.text().await?;
    // let response_body = response.text()?;

    /* Return response. */
    Ok(response_body)
}

/**
 * Request JSON
 *
 * Make a (remote) API call.
 */
async fn request_json_async(
    _sessionid: &str,
    _since: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    request_json_async_with_base(L1_ENDPOINT, _sessionid, _since).await
}

/**
 * Respond JSON (async, with base URL)
 *
 * Make a (remote) API response using a custom base URL.
 * This is the testable core; response_json_async delegates to it.
 */
pub(crate) async fn response_json_async_with_base(
    base_url: &str,
    _sessionid: &str,
    _response: String,
) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = build_response_url_with_base(base_url);

    let json_string = build_exec_response_json(_sessionid, &_response);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_string.to_string())
        .send()
        .await?;

    let response_body = response.text().await?;

    /* Return response. */
    Ok(response_body)
}

/**
 * Respond JSON
 *
 * Make a (remote) API response.
 */
async fn response_json_async(
    _sessionid: &str,
    _response: String,
) -> Result<String, Box<dyn std::error::Error>> {
    response_json_async_with_base(L1_ENDPOINT, _sessionid, _response).await
}

/**
 * Resolve Exec
 *
 * Pure command dispatch: maps an exec string to a response string.
 * Returns None if the request list is empty.
 * This function contains NO network I/O and is fully testable.
 */
pub(crate) fn resolve_exec(_resp: &[Request]) -> Option<String> {
    /* Validate response. */
    if _resp.is_empty() {
        return None;
    }

    let exec = &_resp[0].exec;

    // println!("\n***HANDLING (VEC) EXEC {:?}", &exec);

    if exec == "avax" || exec == "avalanche" {
        let response = match cmd::network::avax() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `avax`: {}", err),
        };
        return Some(response);
    }

    if exec == "install avax" || exec == "install avalanche" {
        let response = match cmd::network::avax_install() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `avax_install`: {}", err),
        };
        return Some(response);
    }

    if exec == "start avax" || exec == "start avalanche" {
        let response = match cmd::network::avax_start() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `avax_start`: {}", err),
        };
        return Some(response);
    }

    if exec == "avax status" || exec == "avalanche status" {
        let response = match cmd::network::avax_status() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `avax_status`: {}", err),
        };
        return Some(response);
    }

    if exec == "build avax" || exec == "build avalanche" {
        let response = match cmd::network::build_avalanche() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `install avax`: {}", err),
        };
        return Some(response);
    }

    if exec == "df" {
        let response = match cmd::sys::df() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `df`: {}", err),
        };
        return Some(response);
    }

    if exec == "du" {
        let response = match cmd::sys::du() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `du`: {}", err),
        };
        return Some(response);
    }

    if exec == "install go" || exec == "install golang" {
        let response = match cmd::sys::install_golang() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `install go`: {}", err),
        };
        return Some(response);
    }

    if exec == "ls" {
        let response = match cmd::sys::ls() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `ls`: {}", err),
        };
        return Some(response);
    }

    if exec == "lsblk" {
        let response = match cmd::sys::lsblk() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `lsblk`: {}", err),
        };
        return Some(response);
    }

    if exec == "lscpu" {
        let response = match cmd::sys::lscpu() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `lscpu`: {}", err),
        };
        return Some(response);
    }

    if exec == "lshw" {
        let response = match cmd::sys::lshw() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `lshw`: {}", err),
        };
        return Some(response);
    }

    if exec == "mem" {
        let response = match cmd::sys::mem() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `mem`: {}", err),
        };
        return Some(response);
    }

    if exec == "ps" {
        let response = match cmd::sys::ps() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `ps`: {}", err),
        };
        return Some(response);
    }

    if exec == "profiler" {
        let response = match cmd::sys::system_profiler() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `system_profiler`: {}", err),
        };
        return Some(response);
    }

    if exec == "uname" {
        let response = match cmd::sys::get_uname() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `uname`: {}", err),
        };
        return Some(response);
    }

    if exec == "uptime" {
        let response = match cmd::sys::get_uptime() {
            Ok(val) => val,
            Err(err) => format!("ERROR: Could NOT execute `uptime`: {}", err),
        };
        return Some(response);
    }

    /*************************************/
    /* HELP */
    /*************************************/

    if exec == "help" {
        let response =
            "Oops! Help is temporarily unavailable. Please try again later...".to_string();
        return Some(response);
    }

    /*************************************/
    /* UNIMPLEMENTED */
    /*************************************/

    if exec == "arb" || exec == "arbitrum" {
        let response = "ERROR! Arbitrum is NOT implemented.".to_string();
        return Some(response);
    }

    if exec == "base" {
        let response = "ERROR! Base is NOT implemented.".to_string();
        return Some(response);
    }

    if exec == "nexa" {
        let response = "ERROR! Nexa is NOT implemented.".to_string();
        return Some(response);
    }

    if exec == "op" || exec == "optimism" {
        let response = "ERROR! Optimism is NOT implemented.".to_string();
        return Some(response);
    }

    if exec == "sol" || exec == "solana" {
        let response = "ERROR! Solana is NOT implemented.".to_string();
        return Some(response);
    }

    let response = format!(
        "ERROR! [ {} ] is an UNKNOWN command. Try &lt;help&gt; for more options.",
        exec
    );
    Some(response)
}

fn _handle_exec(rt: &tokio::runtime::Runtime, _sessionid: &str, _resp: Vec<Request>) {
    // println!("\n***HANDLING (VEC) RESPONSE {:?}", _resp);

    if let Some(response) = resolve_exec(&_resp) {
        let _ = rt.block_on(response_json_async(_sessionid, response));
    }

    // let response = "ERROR! A FATAL ERROR OCCURED :(".to_string();
    // response_json(_sessionid, response);
}

pub fn by_session(rt: &tokio::runtime::Runtime, _sessionid: &str) {
    println!("\n  Waiting for Client command...\n");

    let mut response: Result<String, Box<dyn std::error::Error>>;

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();

        thread::sleep(ten_seconds);

        assert!(now.elapsed() >= ten_seconds);

        /* Make (remote) JSON (data) request. */
        response = rt.block_on(request_json_async(
            _sessionid,
            LAST_SINCE.load(Ordering::Relaxed),
        ));
        // println!("\nRAW---\n{:?}\n", response);

        // let session_resp: Result<_, Box<dyn std::error::Error>>;
        let mut session_resp: Result<SessionResponse, serde_json::Error> =
            Ok(SessionResponse::default());
        // let session_resp = SessionResponse::default();

        match &response {
            Ok(_data) => {
                session_resp = from_str(_data);
            }
            Err(_) => println!("\n  ERROR: Failed to receive a response from API server."),
        }
        // println!("\nSR---\n{:?}\n", session_resp);

        let mut remote_data: SessionResponse = SessionResponse::default();
        // let mut remote_data: Option<SessionResponse> = None;
        // let mut remote_data: SessionResponse;

        if let Ok(_data) = session_resp {
            // remote_data = session_resp.unwrap();
            /* Set remote data (result). */
            remote_data = _data;

            /* Update last since. */
            LAST_SINCE.store(remote_data.result.last_since, Ordering::Relaxed);
        }
        // println!("\nRD (result)---\n{:?}\n", remote_data.result); // Output: Person { name: "Jane Doe", age: 25 }

        // println!("");
        // println!("  SESSION ID -> {}", remote_data.result.sessionid);
        // println!("      ACTION -> {:?}", remote_data.result.act);
        // println!("     REQUEST -> {:?}", remote_data.result.req);
        // println!("     CREATED -> {}", remote_data.result.created_at);
        // println!("  LAST SINCE -> {}", remote_data.result.last_since);

        if let Some(_data) = remote_data.result.req {
            _handle_exec(rt, &remote_data.result.sessionid, _data)
        }
    }
}

#[cfg(test)]
#[path = "monitor.test.rs"]
mod monitor_test;
