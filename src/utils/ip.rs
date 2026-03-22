// src/utils/ip.rs

/* Import modules. */
use reqwest;
use std::collections::HashMap;

/**
 * Get IP
 *
 * Retrieves IP address from a remote (web) data source.
 */
pub async fn get() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    // println!("{:#?}\n", resp);

    Ok(resp)
}

#[cfg(test)]
#[path = "ip.test.rs"]
mod ip_test;
