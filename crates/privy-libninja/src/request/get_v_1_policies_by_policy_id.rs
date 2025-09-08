use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_v1_policies_by_policy_id`].

On request success, this will return a [`Policy`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetV1PoliciesByPolicyIdRequest {
    pub policy_id: String,
    pub privy_app_id: String,
}
impl FluentRequest<'_, GetV1PoliciesByPolicyIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetV1PoliciesByPolicyIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Policy>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}", policy_id = self.params.policy_id
            );
            let mut r = self.client.client.get(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get Policy

Get a policy by policy ID.*/
    pub fn get_v1_policies_by_policy_id(
        &self,
        policy_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, GetV1PoliciesByPolicyIdRequest> {
        FluentRequest {
            client: self,
            params: GetV1PoliciesByPolicyIdRequest {
                policy_id: policy_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
