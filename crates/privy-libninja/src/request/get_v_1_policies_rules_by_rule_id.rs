use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_v1_policies_rules_by_rule_id`].

On request success, this will return a [`RuleResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetV1PoliciesRulesByRuleIdRequest {
    pub policy_id: String,
    pub rule_id: String,
}
impl FluentRequest<'_, GetV1PoliciesRulesByRuleIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetV1PoliciesRulesByRuleIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::RuleResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}/rules/{rule_id}", policy_id = self.params
                .policy_id, rule_id = self.params.rule_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get Policy Rule

Get a rule by policy ID and rule ID.*/
    pub fn get_v1_policies_rules_by_rule_id(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> FluentRequest<'_, GetV1PoliciesRulesByRuleIdRequest> {
        FluentRequest {
            client: self,
            params: GetV1PoliciesRulesByRuleIdRequest {
                policy_id: policy_id.to_owned(),
                rule_id: rule_id.to_owned(),
            },
        }
    }
}
