#[tokio::test]
async fn test_requests_include_privy_client_header() {
    use httpmock::prelude::*;
    use privy_rs::{PrivyClient, client::PrivyClientOptions};

    // Start a mock server
    let server = MockServer::start();

    // Create a mock endpoint that requires the privy-client header
    // The header should have format "rust:{version}" where version is like "0.1.0-alpha.3"
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/users")
            // Verify the privy-client header exists
            .header_exists("privy-client")
            // Use a custom function to validate the header value format
            .is_true(|req: &HttpMockRequest| {
                // Get the headers and check for privy-client
                let headers = req.headers();
                if let Some(header_value) = headers.get("privy-client") {
                    // Convert the HeaderValue to a string and check the format
                    if let Ok(value_str) = header_value.to_str() {
                        // The header should start with "rust:" and contain version info
                        return value_str.starts_with("rust:") && value_str.contains('.');
                    }
                }
                false
            });
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "data": []
            }));
    });

    // Create a PrivyClient pointing to the mock server
    let client = PrivyClient::new_with_options(
        "test_app_id".to_string(),
        "test_app_secret".to_string(),
        PrivyClientOptions {
            base_url: server.base_url(),
            ..Default::default()
        },
    )
    .expect("Failed to create client");

    // Make a request to trigger the mock using the public users() API
    // The list method takes (order_by: Option<&str>, order_direction: Option<&str>)
    let _ = client.users().list(None, None).await;

    // If we get here without the mock failing, it means the header was present
    // and had the correct format. Now verify the mock was actually called.
    mock.assert();
}
