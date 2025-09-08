use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_v1_users_fiat_kyc_by_user_id`].

On request success, this will return a [`GetV1UsersFiatKycByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetV1UsersFiatKycByUserIdRequest {
    pub privy_app_id: String,
    pub provider: String,
    pub user_id: String,
}
impl FluentRequest<'_, GetV1UsersFiatKycByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetV1UsersFiatKycByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::GetV1UsersFiatKycByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/kyc", user_id = self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.query("provider", &self.params.provider.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get KYC status for a user

Get the current KYC verification status for a user from the configured provider*/
    pub fn get_v1_users_fiat_kyc_by_user_id(
        &self,
        privy_app_id: &str,
        provider: &str,
        user_id: &str,
    ) -> FluentRequest<'_, GetV1UsersFiatKycByUserIdRequest> {
        FluentRequest {
            client: self,
            params: GetV1UsersFiatKycByUserIdRequest {
                privy_app_id: privy_app_id.to_owned(),
                provider: provider.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
