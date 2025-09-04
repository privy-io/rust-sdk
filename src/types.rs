use privy_api::Client;

#[derive(Clone, Debug)]
pub struct PrivySigner {
    pub(crate) app_id: String,
    #[allow(dead_code)]
    pub(crate) app_secret: String,
    pub(crate) wallet_id: String,
    pub(crate) client: Client,
    pub(crate) public_key: String,
}

#[derive(serde::Serialize)]
pub enum Method {
    PATCH,
    POST,
    PUT,
    GET,
    DELETE,
}

#[derive(serde::Serialize)]
pub struct PrivySignerBuilder {
    version: u32,
    method: Method,
    url: String,
    body: Option<serde_json::Value>,
    headers: Option<serde_json::Value>,
}

impl PrivySignerBuilder {
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

    #[must_use]
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

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
    use super::*;
    use serde_json::json;
    use test_case::test_case;

    #[test]
    fn test_canonicalization_matches_docs_example() {
        let builder = PrivySignerBuilder::new(
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
        let builder = PrivySignerBuilder::new(Method::GET, "https://example.com".to_string())
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
        let builder = PrivySignerBuilder::new(Method::POST, "https://example.com".to_string())
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
        let builder = PrivySignerBuilder::new(Method::POST, "https://example.com".to_string())
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
