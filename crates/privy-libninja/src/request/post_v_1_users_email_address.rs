use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_email_address`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersEmailAddressRequest {
    pub address: String,
    pub privy_app_id: String,
}
impl FluentRequest<'_, PostV1UsersEmailAddressRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersEmailAddressRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/email/address";
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
    /**Lookup User by Email Address

Looks up a user by their email address.*/
    pub fn post_v1_users_email_address(
        &self,
        address: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PostV1UsersEmailAddressRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersEmailAddressRequest {
                address: address.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
