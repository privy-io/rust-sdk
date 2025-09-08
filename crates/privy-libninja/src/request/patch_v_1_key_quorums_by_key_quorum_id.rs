use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::patch_v1_key_quorums_by_key_quorum_id`].

On request success, this will return a [`KeyQuorum`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchV1KeyQuorumsByKeyQuorumIdRequest {
    pub authorization_threshold: Option<f64>,
    pub display_name: Option<String>,
    pub key_quorum_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub public_keys: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
}
impl FluentRequest<'_, PatchV1KeyQuorumsByKeyQuorumIdRequest> {
    ///Set the value of the authorization_threshold field.
    pub fn authorization_threshold(mut self, authorization_threshold: f64) -> Self {
        self.params.authorization_threshold = Some(authorization_threshold);
        self
    }
    ///Set the value of the display_name field.
    pub fn display_name(mut self, display_name: &str) -> Self {
        self.params.display_name = Some(display_name.to_owned());
        self
    }
    ///Set the value of the privy_authorization_signature field.
    pub fn privy_authorization_signature(
        mut self,
        privy_authorization_signature: &str,
    ) -> Self {
        self
            .params
            .privy_authorization_signature = Some(
            privy_authorization_signature.to_owned(),
        );
        self
    }
    ///Set the value of the public_keys field.
    pub fn public_keys(
        mut self,
        public_keys: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .public_keys = Some(
            public_keys.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the user_ids field.
    pub fn user_ids(
        mut self,
        user_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .user_ids = Some(
            user_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PatchV1KeyQuorumsByKeyQuorumIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::KeyQuorum>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/key_quorums/{key_quorum_id}", key_quorum_id = self.params
                .key_quorum_id
            );
            let mut r = self.client.client.patch(url);
            if let Some(ref unwrapped) = self.params.authorization_threshold {
                r = r.json(serde_json::json!({ "authorization_threshold" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.display_name {
                r = r.json(serde_json::json!({ "display_name" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.public_keys {
                r = r.json(serde_json::json!({ "public_keys" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_ids {
                r = r.json(serde_json::json!({ "user_ids" : unwrapped }));
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Update key quorum

Update a key quorum by key quorum ID.*/
    pub fn patch_v1_key_quorums_by_key_quorum_id(
        &self,
        key_quorum_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PatchV1KeyQuorumsByKeyQuorumIdRequest> {
        FluentRequest {
            client: self,
            params: PatchV1KeyQuorumsByKeyQuorumIdRequest {
                authorization_threshold: None,
                display_name: None,
                key_quorum_id: key_quorum_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
                public_keys: None,
                user_ids: None,
            },
        }
    }
}
