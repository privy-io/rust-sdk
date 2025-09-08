use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{PolicyChainType, OwnerInput, PolicyRule};
/**You should use this struct via [`PrivyLibninjaClient::post_v1_policies`].

On request success, this will return a [`Policy`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1PoliciesRequest {
    pub chain_type: PolicyChainType,
    pub name: String,
    pub owner: Option<OwnerInput>,
    pub owner_id: Option<serde_json::Value>,
    pub privy_app_id: String,
    pub privy_idempotency_key: Option<String>,
    pub rules: Vec<PolicyRule>,
    pub version: String,
}
pub struct PostV1PoliciesRequired<'a> {
    pub chain_type: PolicyChainType,
    pub name: &'a str,
    pub privy_app_id: &'a str,
    pub rules: Vec<PolicyRule>,
    pub version: &'a str,
}
impl FluentRequest<'_, PostV1PoliciesRequest> {
    ///Set the value of the owner field.
    pub fn owner(mut self, owner: OwnerInput) -> Self {
        self.params.owner = Some(owner);
        self
    }
    ///Set the value of the owner_id field.
    pub fn owner_id(mut self, owner_id: serde_json::Value) -> Self {
        self.params.owner_id = Some(owner_id);
        self
    }
    ///Set the value of the privy_idempotency_key field.
    pub fn privy_idempotency_key(mut self, privy_idempotency_key: &str) -> Self {
        self.params.privy_idempotency_key = Some(privy_idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostV1PoliciesRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Policy>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/policies";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "chain_type" : self.params.chain_type }));
            r = r.json(serde_json::json!({ "name" : self.params.name }));
            if let Some(ref unwrapped) = self.params.owner {
                r = r.json(serde_json::json!({ "owner" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.owner_id {
                r = r.json(serde_json::json!({ "owner_id" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_idempotency_key {
                r = r.header("privy-idempotency-key", &unwrapped.to_string());
            }
            r = r.json(serde_json::json!({ "rules" : self.params.rules }));
            r = r.json(serde_json::json!({ "version" : self.params.version }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create Policy

Create a new policy.*/
    pub fn post_v1_policies(
        &self,
        args: PostV1PoliciesRequired,
    ) -> FluentRequest<'_, PostV1PoliciesRequest> {
        FluentRequest {
            client: self,
            params: PostV1PoliciesRequest {
                chain_type: args.chain_type,
                name: args.name.to_owned(),
                owner: None,
                owner_id: None,
                privy_app_id: args.privy_app_id.to_owned(),
                privy_idempotency_key: None,
                rules: args.rules,
                version: args.version.to_owned(),
            },
        }
    }
}
