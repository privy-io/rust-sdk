use serde::Serialize;

pub const PRIVY_AUTHORIZATION_HEADER: &str = "privy-authorization-signature";

/// Create canonical request data for signing
///
/// # Errors
/// This can fail if JSON serialization fails
pub fn build_canonical_request<S: Serialize>(
    app_id: String,
    method: Method,
    url: String,
    body: S,
    idempotency_key: Option<String>,
) -> Result<String, serde_json::Error> {
    let mut headers = serde_json::Map::new();
    headers.insert("privy-app-id".into(), serde_json::Value::String(app_id));
    if let Some(key) = idempotency_key {
        headers.insert(
            "privy-idempotency-key".to_string(),
            serde_json::Value::String(key),
        );
    }

    WalletApiRequestSignatureInput::new(method, url)
        .headers(serde_json::Value::Object(headers))
        .body(body)
        .canonicalize()
}

/// The HTTP method used in the request.
///
/// Note that `GET` requests do not need
/// signatures by definition.
#[derive(serde::Serialize, Debug)]
pub enum Method {
    /// `PATCH` requests are used to update an existing resource.
    PATCH,
    /// `POST` requests are used to create a new resource.
    POST,
    /// `PUT` requests are used to update an existing resource.
    PUT,
    /// `GET` requests are used to retrieve an existing resource.
    DELETE,
}

impl TryFrom<&reqwest::Method> for Method {
    type Error = ();

    fn try_from(value: &reqwest::Method) -> Result<Self, Self::Error> {
        match *value {
            reqwest::Method::PATCH => Ok(Method::PATCH),
            reqwest::Method::POST => Ok(Method::POST),
            reqwest::Method::PUT => Ok(Method::PUT),
            reqwest::Method::DELETE => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}

/// The wallet API request signature input is used
/// during the signing process as a canonical representation
/// of the request. Ensure that you serialize this struct
/// with the `serde_json_canonicalizer` to get the appropriate
/// RFC-8785 canonicalized string. For more information, see
/// <https://datatracker.ietf.org/doc/html/rfc8785>
///
/// Note: Version is currently hardcoded to 1.
#[derive(serde::Serialize)]
pub struct WalletApiRequestSignatureInput<S: Serialize> {
    version: u32,
    method: Method,
    url: String,
    body: Option<S>,
    headers: Option<serde_json::Value>,
}

impl<S: Serialize> WalletApiRequestSignatureInput<S> {
    /// Create a new request builder.
    #[must_use]
    pub fn new(method: Method, url: String) -> Self {
        Self {
            version: 1,
            method,
            url,
            body: None,
            headers: None,
        }
    }

    /// Set the request body.
    #[must_use]
    pub fn body(mut self, body: S) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the request headers.
    #[must_use]
    pub fn headers(mut self, headers: serde_json::Value) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Canonicalize the request body.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    pub fn canonicalize(self) -> Result<String, serde_json::Error> {
        serde_json_canonicalizer::to_string(&self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use test_case::test_case;

    use super::*;

    #[test]
    fn test_canonicalization_matches_docs_example() {
        let builder = WalletApiRequestSignatureInput::new(
            Method::PATCH,
            "https://api.privy.io/v1/wallets/clw4cc3a700b811p865d21b7b".to_string(),
        )
        .body(json!({
            "policy_ids": ["pol_123abc"]
        }))
        .headers(json!({
            "privy-app-id": "your-privy-app-id",
            "privy-idempotency-key": "a-unique-uuid-for-the-request"
        }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");
        let expected = r#"{"body":{"policy_ids":["pol_123abc"]},"headers":{"privy-app-id":"your-privy-app-id","privy-idempotency-key":"a-unique-uuid-for-the-request"},"method":"PATCH","url":"https://api.privy.io/v1/wallets/clw4cc3a700b811p865d21b7b","version":1}"#;

        assert_eq!(canonical, expected);
    }

    #[test]
    fn test_key_ordering() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "z_last": "last",
                    "a_first": "first",
                    "m_middle": "middle"
                }))
                .headers(json!({
                    "z-header": "last",
                    "a-header": "first"
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Keys should be sorted alphabetically at all levels
        assert!(canonical.contains(r#"{"a_first":"first","m_middle":"middle","z_last":"last"}"#));
        assert!(canonical.contains(r#"{"a-header":"first","z-header":"last"}"#));
    }

    #[test]
    fn test_nested_object_sorting() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "outer": {
                        "z_inner": "last",
                        "a_inner": "first"
                    }
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Nested object keys should also be sorted
        assert!(canonical.contains(r#"{"a_inner":"first","z_inner":"last"}"#));
    }

    #[test]
    fn test_array_preservation() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "items": ["third", "first", "second"]
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Array order should be preserved (not sorted)
        assert!(canonical.contains(r#"["third","first","second"]"#));
    }

    #[test_case(
        &json!({"name": "John", "age": 30}),
        r#"{"age":30,"name":"John"}"#;
        "simple object"
    )]
    #[test_case(
        &json!({"name": "John", "address": {"street": "123 Main St", "city": "Boston"}}),
        r#"{"address":{"city":"Boston","street":"123 Main St"},"name":"John"}"#;
        "nested object"
    )]
    #[test_case(
        &json!({"name": "John", "numbers": [1, 2, 3]}),
        r#"{"name":"John","numbers":[1,2,3]}"#;
        "array"
    )]
    #[test_case(
        &json!({"name": "John", "age": null}),
        r#"{"age":null,"name":"John"}"#;
        "null value"
    )]
    #[test_case(
        &json!({"name": "John", "age": 30, "address": {"street": "123 Main St", "city": "Boston"}, "hobbies": ["reading", "gaming"], "middleName": null}),
        r#"{"address":{"city":"Boston","street":"123 Main St"},"age":30,"hobbies":["reading","gaming"],"middleName":null,"name":"John"}"#;
        "complex object"
    )]
    fn test_json_canonicalization(json: &serde_json::Value, expected: &str) {
        let result =
            serde_json_canonicalizer::to_string(json).expect("canonicalization should succeed");
        assert_eq!(result, expected);
    }
}
