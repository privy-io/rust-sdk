use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::patch_v1_users_fiat_kyc_by_user_id`].

On request success, this will return a [`PatchV1UsersFiatKycByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchV1UsersFiatKycByUserIdRequest {
    pub body: serde_json::Value,
    pub privy_app_id: String,
    pub user_id: String,
}
impl FluentRequest<'_, PatchV1UsersFiatKycByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PatchV1UsersFiatKycByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PatchV1UsersFiatKycByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/kyc", user_id = self.params.user_id
            );
            let mut r = self.client.client.patch(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Update KYC status for a user

Update the KYC verification status for a user from the configured provider*/
    pub fn patch_v1_users_fiat_kyc_by_user_id(
        &self,
        body: serde_json::Value,
        privy_app_id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, PatchV1UsersFiatKycByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PatchV1UsersFiatKycByUserIdRequest {
                body,
                privy_app_id: privy_app_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
