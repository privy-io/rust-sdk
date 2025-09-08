use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{OwnerInput, PolicyRule};
/**You should use this struct via [`PrivyLibninjaClient::patch_v1_policies_by_policy_id`].

On request success, this will return a [`Policy`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchV1PoliciesByPolicyIdRequest {
    pub name: Option<String>,
    pub owner: Option<OwnerInput>,
    pub owner_id: Option<serde_json::Value>,
    pub policy_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub rules: Option<Vec<PolicyRule>>,
}
impl FluentRequest<'_, PatchV1PoliciesByPolicyIdRequest> {
    ///Set the value of the name field.
    pub fn name(mut self, name: &str) -> Self {
        self.params.name = Some(name.to_owned());
        self
    }
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
    ///Set the value of the rules field.
    pub fn rules(mut self, rules: Vec<PolicyRule>) -> Self {
        self.params.rules = Some(rules);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PatchV1PoliciesByPolicyIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Policy>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}", policy_id = self.params.policy_id
            );
            let mut r = self.client.client.patch(url);
            if let Some(ref unwrapped) = self.params.name {
                r = r.json(serde_json::json!({ "name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.owner {
                r = r.json(serde_json::json!({ "owner" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.owner_id {
                r = r.json(serde_json::json!({ "owner_id" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.rules {
                r = r.json(serde_json::json!({ "rules" : unwrapped }));
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Update Policy

Update a policy by policy ID.*/
    pub fn patch_v1_policies_by_policy_id(
        &self,
        policy_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PatchV1PoliciesByPolicyIdRequest> {
        FluentRequest {
            client: self,
            params: PatchV1PoliciesByPolicyIdRequest {
                name: None,
                owner: None,
                owner_id: None,
                policy_id: policy_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
                rules: None,
            },
        }
    }
}
