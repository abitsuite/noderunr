// src/utils/ip.rs

/* Import modules. */
use reqwest;
use std::collections::HashMap;

/* Initialize constants. */
const IP_ENDPOINT: &str = "https://httpbin.org/ip";

/**
 * Get IP (with URL)
 *
 * Retrieves IP address from a specified URL.
 * This is the testable core; `get` delegates to it.
 */
pub async fn get_with_url(url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    // println!("{:#?}\n", resp);

    Ok(resp)
}

/**
 * Get IP
 *
 * Retrieves IP address from a remote (web) data source.
 */
pub async fn get() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    get_with_url(IP_ENDPOINT).await
}

#[cfg(test)]
#[path = "ip.test.rs"]
mod ip_test;
