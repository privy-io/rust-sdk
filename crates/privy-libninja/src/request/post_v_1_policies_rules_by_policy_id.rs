use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_policies_rules_by_policy_id`].

On request success, this will return a [`RuleResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1PoliciesRulesByPolicyIdRequest {
    pub action: String,
    pub conditions: Vec<serde_json::Value>,
    pub method: String,
    pub name: String,
    pub policy_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
}
pub struct PostV1PoliciesRulesByPolicyIdRequired<'a> {
    pub action: &'a str,
    pub conditions: Vec<serde_json::Value>,
    pub method: &'a str,
    pub name: &'a str,
    pub policy_id: &'a str,
    pub privy_app_id: &'a str,
}
impl FluentRequest<'_, PostV1PoliciesRulesByPolicyIdRequest> {
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
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1PoliciesRulesByPolicyIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::RuleResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}/rules", policy_id = self.params.policy_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "action" : self.params.action }));
            r = r.json(serde_json::json!({ "conditions" : self.params.conditions }));
            r = r.json(serde_json::json!({ "method" : self.params.method }));
            r = r.json(serde_json::json!({ "name" : self.params.name }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create Policy Rule

Create a new rule for a policy.*/
    pub fn post_v1_policies_rules_by_policy_id(
        &self,
        args: PostV1PoliciesRulesByPolicyIdRequired,
    ) -> FluentRequest<'_, PostV1PoliciesRulesByPolicyIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1PoliciesRulesByPolicyIdRequest {
                action: args.action.to_owned(),
                conditions: args.conditions,
                method: args.method.to_owned(),
                name: args.name.to_owned(),
                policy_id: args.policy_id.to_owned(),
                privy_app_id: args.privy_app_id.to_owned(),
                privy_authorization_signature: None,
            },
        }
    }
}
