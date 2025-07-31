use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

use std::process::exit;
use std::{thread, time};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

use crate::cmd;

#[derive(Debug, Deserialize)]
struct Action {
    actionid: Option<String>,
    body: Option<String>,
    target: Option<String>,
    created_at: u64, // milliseconds
}

#[derive(Debug, Serialize)]
struct ExecResponse {
    sessionid: String,
    method: String,
    resp: String,
}

#[derive(Debug, Deserialize)]
struct Log {
    body: String,
    created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize)]
struct Request {
    exec: String,
    created_at: u64, // milliseconds
}

#[derive(Serialize)]
struct Session {
    sessionid: String,
    since: u64, // milliseconds
}

#[derive(Debug, Default, Deserialize)]
struct SessionResponse {
    sessionid: String,
    act: Option<Vec<Action>>,
    log: Option<Vec<Action>>,
    req: Option<Vec<Request>>,
    res: Option<Vec<Action>>,
    rpt: Option<Vec<Action>>,
    created_at: u32, // seconds
    last_since: u64 // milliseconds
}

/* Initialize constants. */
const L1_ENDPOINT: &str = "https://L1.run/v1/";

/* Initialize globals. */
static mut LAST_SINCE: u64 = 1;


/**
 * Request JSON
 *
 * Make a (remote) API call.
 */
#[tokio::main]
async fn request_json(_sessionid: &str, _since: u64) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}", L1_ENDPOINT, "session");

    let session = Session {
        sessionid: _sessionid.to_string(),
        since: _since,
    };

    let json_string = to_string(&session).unwrap();
    // let json_string = to_string(&session)?;

    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .body(json_string.to_string())
        // .body(&json_string)
        .send()
        .await?;

    let response_body = response.text().await?;
    // let response_body = response.text()?;

    /* Return response. */
    Ok(response_body)
}

/**
 * Respond JSON
 *
 * Make a (remote) API response.
 */
 #[tokio::main]
 async fn response_json(_sessionid: &str, _response: String) -> Result<String, Box<dyn std::error::Error>> {
     /* Set URL (for remote API). */
     let url = format!("{}{}", L1_ENDPOINT, "session");
 
     let exec_response = ExecResponse {
         sessionid: _sessionid.to_string(),
         method: "res".to_string(),
         resp: _response,
     };
 
     let json_string = to_string(&exec_response).unwrap();
 
     let client = reqwest::Client::new();
     let response = client.post(url)
         .header("Content-Type", "application/json")
         .body(json_string.to_string())
         .send()
         .await?;
 
     let response_body = response.text().await?;
 
     /* Return response. */
     Ok(response_body)
 }

fn _handle_exec(_sessionid: &str, _resp: Vec<Request>) {
// println!("\n***HANDLING (VEC) RESPONSE {:?}", _resp);

    /* Validate response. */
    if (_resp.len() > 0) {
        let exec = &_resp[0].exec;

// println!("\n***HANDLING (VEC) EXEC {:?}", &exec);

        if (exec == "avax" || exec == "avalanche") {
            let response = cmd::network::avax().expect("Oops! Could NOT execute `avax`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "install avax" || exec == "install avalanche") {
            let response = cmd::network::avax_install().expect("Oops! Could NOT execute `avax_install`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "start avax" || exec == "start avalanche") {
            let response = cmd::network::avax_start().expect("Oops! Could NOT execute `avax_start`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "avax status" || exec == "avalanche status") {
            let response = cmd::network::avax_status().expect("Oops! Could NOT execute `avax_status`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "build avax" || exec == "build avalanche") {
            let response = cmd::network::build_avalanche().expect("Oops! Could NOT execute `install avax`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "df") {
            let response = cmd::sys::df().expect("Oops! Could NOT execute `df`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "du") {
            let response = cmd::sys::du().expect("Oops! Could NOT execute `du`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "install go" || exec == "install golang") {
            let response = cmd::sys::install_golang().expect("Oops! Could NOT execute `install go`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "ls") {
            let response = cmd::sys::ls().expect("Oops! Could NOT execute `ls`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "lsblk") {
            let response = cmd::sys::lsblk().expect("Oops! Could NOT execute `lsblk`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "lscpu") {
            let response = cmd::sys::lscpu().expect("Oops! Could NOT execute `lscpu`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "lshw") {
            let response = cmd::sys::lshw().expect("Oops! Could NOT execute `lshw`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "mem") {
            let response = cmd::sys::mem().expect("Oops! Could NOT execute `mem`.");
            response_json(_sessionid, response);
            return ();
        }
    
        if (exec == "ps") {
            let response = cmd::sys::ps().expect("Oops! Could NOT execute `ps`.");
            response_json(_sessionid, response);
            return ();
        }
    
        if (exec == "profiler") {
            let response = cmd::sys::system_profiler().expect("Oops! Could NOT execute `system_profiler`.");
            response_json(_sessionid, response);
            return ();
        }
    
        if (exec == "uname") {
            let response = cmd::sys::get_uname().expect("Oops! Could NOT execute `uname`.");
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "uptime") {
            let response = cmd::sys::get_uptime().expect("Oops! Could NOT execute `uptime`.");
            response_json(_sessionid, response);
            return ();
        }
    
        /*************************************/
        /* HELP */
        /*************************************/

        if (exec == "help") {
            let response = "Oops! Help is temporarily unavailable. Please try again later...".to_string();
            response_json(_sessionid, response);
            return ();
        }

        /*************************************/
        /* UNIMPLEMENTED */
        /*************************************/

        if (exec == "arb" || exec == "arbitrum") {
            let response = "ERROR! Arbitrum is NOT implemented.".to_string();
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "base") {
            let response = "ERROR! Base is NOT implemented.".to_string();
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "nexa") {
            let response = "ERROR! Nexa is NOT implemented.".to_string();
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "op" || exec == "optimism") {
            let response = "ERROR! Optimism is NOT implemented.".to_string();
            response_json(_sessionid, response);
            return ();
        }

        if (exec == "sol" || exec == "solana") {
            let response = "ERROR! Solana is NOT implemented.".to_string();
            response_json(_sessionid, response);
            return ();
        }

        let response = format!("ERROR! [ {} ] is an UNKNOWN command. Try &lt;help&gt; for more options.", exec);
        response_json(_sessionid, response);
        return ();
    }

    // let response = "ERROR! A FATAL ERROR OCCURED :(".to_string();
    // response_json(_sessionid, response);
}

pub fn by_session(_sessionid: &str) {
    println!("\n  Waiting for Client command...\n");

    let mut response: Result<String, Box<dyn std::error::Error>>;

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        unsafe {
            /* Make (remote) JSON (data) request. */
            response = request_json(_sessionid, LAST_SINCE);
// println!("\nRAW---\n{:?}\n", response);
        }

        // let session_resp: Result<_, Box<dyn std::error::Error>>;
        let mut session_resp: Result<SessionResponse, serde_json::Error> = Ok(SessionResponse::default());
        // let session_resp = SessionResponse::default();

        match(&response) {
            Ok(_data) => {
                session_resp = from_str(_data);
            },
            Err(_) => println!("\n  ERROR: Failed to receive a response from API server."),
        }
// println!("\nSR---\n{:?}\n", session_resp);

        let mut remote_data: SessionResponse = SessionResponse::default();
        // let mut remote_data: Option<SessionResponse> = None;
        // let mut remote_data: SessionResponse;

        match(session_resp) {
            Ok(_data) => {
                // remote_data = session_resp.unwrap();
                remote_data = _data;

                unsafe {
                    /* Update last since. */
                    LAST_SINCE = remote_data.last_since
                }
            },
            // Err(_) => println!("ERROR: Failed to receive any remote data."),
            Err(_) => (),
        }
// println!("\nRD---\n{:?}\n", remote_data); // Output: Person { name: "Jane Doe", age: 25 }

// println!("");
// println!("  SESSION ID -> {}", remote_data.sessionid);
// println!("      ACTION -> {:?}", remote_data.act);
// println!("     REQUEST -> {:?}", remote_data.req);
// println!("     CREATED -> {}", remote_data.created_at);
// println!("  LAST SINCE -> {}", remote_data.last_since);

        match remote_data.req {
            Some(_data) => _handle_exec(&remote_data.sessionid, _data),
            None => (),
        }
    }
}
