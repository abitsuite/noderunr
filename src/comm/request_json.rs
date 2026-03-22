// src/comm/request_json.rs

/* Initialize constants. */
const L1_ENDPOINT: &str = "https://l1.run/v1/";

/**
 * Build URL
 *
 * Constructs the full API URL for the given endpoint.
 */
pub(crate) fn build_url(endpoint: &str) -> String {
    format!("{}{}", L1_ENDPOINT, endpoint)
}

/**
 * Request JSON
 *
 * Make a (remote) API call for JSON-formatted data.
 */
pub async fn request_json(
    _endpoint: &str,
    _json: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = build_url(_endpoint);

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

#[cfg(test)]
#[path = "request_json.test.rs"]
mod request_json_test;
