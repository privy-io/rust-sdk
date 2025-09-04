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
    ///
    /// # Panics
    /// Panics if the resulting json is not valid utf-8.
    pub fn canonicalize(&self) -> Result<String, serde_json::Error> {
        let mut output = Vec::new();
        let serializer = CanonicalSerializer::new(&mut output);
        self.serialize(serializer)?;
        Ok(String::from_utf8(output).expect("valid utf-8"))
    }
}

use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::BTreeMap;

struct CanonicalSerializer<W> {
    output: W,
}

impl<W: std::io::Write> CanonicalSerializer<W> {
    fn new(output: W) -> Self {
        Self { output }
    }
}

impl<W: std::io::Write> Serializer for CanonicalSerializer<W> {
    type Ok = ();
    type Error = serde_json::Error;
    type SerializeSeq = CanonicalSeq<W>;
    type SerializeTuple = CanonicalSeq<W>;
    type SerializeTupleStruct = CanonicalSeq<W>;
    type SerializeTupleVariant = CanonicalSeq<W>;
    type SerializeMap = CanonicalMap<W>;
    type SerializeStruct = CanonicalMap<W>;
    type SerializeStructVariant = CanonicalMap<W>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{v}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "\"{v}\"").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        // Use serde_json's proper string escaping
        let escaped = serde_json::to_string(v)?;
        write!(self.output, "{escaped}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::Error;
        Err(serde_json::Error::custom("bytes not supported"))
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "null").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(mut self) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "null").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_unit_struct(mut self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "null").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_unit_variant(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "\"{variant}\"").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        write!(self.output, "{{\"{variant}\":").map_err(serde_json::Error::io)?;
        let mut temp_output = Vec::new();
        value.serialize(CanonicalSerializer::new(&mut temp_output))?;
        self.output
            .write_all(&temp_output)
            .map_err(serde_json::Error::io)?;
        write!(self.output, "}}").map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn serialize_seq(mut self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        write!(self.output, "[").map_err(serde_json::Error::io)?;
        Ok(CanonicalSeq {
            output: self.output,
            first: true,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        write!(self.output, "{{\"{variant}\":[").map_err(serde_json::Error::io)?;
        Ok(CanonicalSeq {
            output: self.output,
            first: true,
        })
    }

    fn serialize_map(mut self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        write!(self.output, "{{").map_err(serde_json::Error::io)?;
        Ok(CanonicalMap::new(self.output))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        write!(self.output, "{{\"{variant}\":{{").map_err(serde_json::Error::io)?;
        Ok(CanonicalMap::new(self.output))
    }
}

struct CanonicalSeq<W> {
    output: W,
    first: bool,
}

impl<W: std::io::Write> SerializeSeq for CanonicalSeq<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        if !self.first {
            write!(self.output, ",").map_err(serde_json::Error::io)?;
        }
        self.first = false;
        let mut temp_output = Vec::new();
        value.serialize(CanonicalSerializer::new(&mut temp_output))?;
        self.output
            .write_all(&temp_output)
            .map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "]").map_err(serde_json::Error::io)?;
        Ok(())
    }
}

impl<W: std::io::Write> serde::ser::SerializeTuple for CanonicalSeq<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<W: std::io::Write> serde::ser::SerializeTupleStruct for CanonicalSeq<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<W: std::io::Write> serde::ser::SerializeTupleVariant for CanonicalSeq<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "]}}").map_err(serde_json::Error::io)?;
        Ok(())
    }
}

struct CanonicalMap<W> {
    output: W,
    entries: BTreeMap<String, String>,
}

impl<W: std::io::Write> CanonicalMap<W> {
    fn new(output: W) -> Self {
        Self {
            output,
            entries: BTreeMap::new(),
        }
    }
}

impl<W: std::io::Write> SerializeMap for CanonicalMap<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: Serialize + ?Sized,
        V: Serialize + ?Sized,
    {
        let mut key_buf = Vec::new();
        key.serialize(CanonicalSerializer::new(&mut key_buf))?;
        let key_str = String::from_utf8(key_buf).expect("valid utf-8");

        let mut value_buf = Vec::new();
        value.serialize(CanonicalSerializer::new(&mut value_buf))?;
        let value_str = String::from_utf8(value_buf).expect("valid utf-8");

        self.entries.insert(key_str, value_str);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut first = true;
        for (key, value) in self.entries {
            if !first {
                write!(self.output, ",").map_err(serde_json::Error::io)?;
            }
            first = false;
            write!(self.output, "{key}:{value}").map_err(serde_json::Error::io)?;
        }
        write!(self.output, "}}").map_err(serde_json::Error::io)?;
        Ok(())
    }
}

impl<W: std::io::Write> serde::ser::SerializeStruct for CanonicalMap<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

impl<W: std::io::Write> serde::ser::SerializeStructVariant for CanonicalMap<W> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut first = true;
        for (key, value) in self.entries {
            if !first {
                write!(self.output, ",").map_err(serde_json::Error::io)?;
            }
            first = false;
            write!(self.output, "{key}:{value}").map_err(serde_json::Error::io)?;
        }
        write!(self.output, "}}}}").map_err(serde_json::Error::io)?;
        Ok(())
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
        let mut output = Vec::new();
        let serializer = CanonicalSerializer::new(&mut output);
        json.serialize(serializer)
            .expect("serialization should succeed");
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, expected);
    }
}
