use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_wallet_address`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersWalletAddressRequest {
    pub address: serde_json::Value,
    pub privy_app_id: String,
}
impl FluentRequest<'_, PostV1UsersWalletAddressRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersWalletAddressRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/wallet/address";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "address" : self.params.address }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by wallet address

Looks up a user by their wallet address.*/
    pub fn post_v1_users_wallet_address(
        &self,
        address: serde_json::Value,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PostV1UsersWalletAddressRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersWalletAddressRequest {
                address,
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
