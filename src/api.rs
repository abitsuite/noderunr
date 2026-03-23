// src/api.rs

/* Initialize constants. */
const L1_ENDPOINT: &str = "https://l1.run/v1/";

/**
 * Build URL
 *
 * Constructs the full API URL for the given endpoint.
 */
#[cfg(test)]
pub(crate) fn build_url(endpoint: &str) -> String {
    format!("{}{}", L1_ENDPOINT, endpoint)
}

/**
 * Build URL (with base)
 *
 * Constructs the full API URL using a custom base URL.
 * Used by tests to inject mockito server URLs.
 */
pub(crate) fn build_url_with_base(base_url: &str, endpoint: &str) -> String {
    format!("{}{}", base_url, endpoint)
}

/**
 * Call (with base URL)
 *
 * Make a (remote) API call against a specified base URL.
 * This is the testable core; `call` delegates to it.
 */
pub async fn call_with_base_url(base_url: &str, _endpoint: &str, _json: &str) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = build_url_with_base(base_url, _endpoint);

    // let headers = [("Authorization", "Bearer YOUR_API_KEY"), ("X-Custom-Header", "value")];

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        // .headers(headers.into_iter().collect())
        .body(_json.to_string())
        .send()
        .await?;

    // TODO Validate status (200 OK)
    // println!("Status: {}", response.status());

    let response_body = response.text().await?;
    // println!("Response body:\n{}", response_body);

    /* Return response. */
    Ok(response_body)
}

/**
 * Call
 *
 * Make a (remote) API call.
 */
pub async fn call(_endpoint: &str, _json: &str) -> Result<String, Box<dyn std::error::Error>> {
    call_with_base_url(L1_ENDPOINT, _endpoint, _json).await
}

#[cfg(test)]
#[path = "api.test.rs"]
mod api_test;
