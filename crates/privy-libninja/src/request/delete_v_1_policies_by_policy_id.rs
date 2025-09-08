use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::delete_v1_policies_by_policy_id`].

On request success, this will return a [`DeleteV1PoliciesByPolicyIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteV1PoliciesByPolicyIdRequest {
    pub policy_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
}
impl FluentRequest<'_, DeleteV1PoliciesByPolicyIdRequest> {
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
for FluentRequest<'a, DeleteV1PoliciesByPolicyIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::DeleteV1PoliciesByPolicyIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/policies/{policy_id}", policy_id = self.params.policy_id
            );
            let mut r = self.client.client.delete(url);
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
    /**Delete Policy

Delete a policy by policy ID.*/
    pub fn delete_v1_policies_by_policy_id(
        &self,
        policy_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, DeleteV1PoliciesByPolicyIdRequest> {
        FluentRequest {
            client: self,
            params: DeleteV1PoliciesByPolicyIdRequest {
                policy_id: policy_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
            },
        }
    }
}
