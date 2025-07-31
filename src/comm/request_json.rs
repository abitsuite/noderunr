/* Initialize constants. */
const L1GURU_ENDPOINT: &str = "https://layer1.guru/v1/";

/**
 * Request JSON
 *
 * Make a (remote) API call for JSON-formatted data.
 */
#[tokio::main]
pub async fn request_json(_endpoint: &str, _json: &str) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}", L1GURU_ENDPOINT, _endpoint);

    // let headers = [("Authorization", "Bearer YOUR_API_KEY"), ("X-Custom-Header", "value")];

    let client = reqwest::Client::new();
    let response = client.post(url)
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
