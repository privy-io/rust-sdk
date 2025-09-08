use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::patch_v1_policies_rules_by_rule_id`].

On request success, this will return a [`PatchV1PoliciesRulesByRuleIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchV1PoliciesRulesByRuleIdRequest {
    pub action: String,
    pub conditions: Vec<serde_json::Value>,
    pub method: String,
    pub name: String,
    pub policy_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub rule_id: String,
}
pub struct PatchV1PoliciesRulesByRuleIdRequired<'a> {
    pub action: &'a str,
    pub conditions: Vec<serde_json::Value>,
    pub method: &'a str,
    pub name: &'a str,
    pub policy_id: &'a str,
    pub privy_app_id: &'a str,
    pub rule_id: &'a str,
}
impl FluentRequest<'_, PatchV1PoliciesRulesByRuleIdRequest> {
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
for FluentRequest<'a, PatchV1PoliciesRulesByRuleIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PatchV1PoliciesRulesByRuleIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}/rules/{rule_id}", policy_id = self.params
                .policy_id, rule_id = self.params.rule_id
            );
            let mut r = self.client.client.patch(url);
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
    /**Update Policy Rule

Update a rule by policy ID and rule ID.*/
    pub fn patch_v1_policies_rules_by_rule_id(
        &self,
        args: PatchV1PoliciesRulesByRuleIdRequired,
    ) -> FluentRequest<'_, PatchV1PoliciesRulesByRuleIdRequest> {
        FluentRequest {
            client: self,
            params: PatchV1PoliciesRulesByRuleIdRequest {
                action: args.action.to_owned(),
                conditions: args.conditions,
                method: args.method.to_owned(),
                name: args.name.to_owned(),
                policy_id: args.policy_id.to_owned(),
                privy_app_id: args.privy_app_id.to_owned(),
                privy_authorization_signature: None,
                rule_id: args.rule_id.to_owned(),
            },
        }
    }
}
