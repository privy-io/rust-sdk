use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_v1_users_fiat_accounts_by_user_id`].

On request success, this will return a [`GetV1UsersFiatAccountsByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetV1UsersFiatAccountsByUserIdRequest {
    pub privy_app_id: String,
    pub provider: String,
    pub user_id: String,
}
impl FluentRequest<'_, GetV1UsersFiatAccountsByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetV1UsersFiatAccountsByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::GetV1UsersFiatAccountsByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/accounts", user_id = self.params.user_id
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
    /**Get user's fiat accounts

Returns the IDs of all external fiat accounts (used for offramping) for the user*/
    pub fn get_v1_users_fiat_accounts_by_user_id(
        &self,
        privy_app_id: &str,
        provider: &str,
        user_id: &str,
    ) -> FluentRequest<'_, GetV1UsersFiatAccountsByUserIdRequest> {
        FluentRequest {
            client: self,
            params: GetV1UsersFiatAccountsByUserIdRequest {
                privy_app_id: privy_app_id.to_owned(),
                provider: provider.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
