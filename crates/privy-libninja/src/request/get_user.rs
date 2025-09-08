use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_user`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub privy_app_id: String,
    pub user_id: String,
}
impl FluentRequest<'_, GetUserRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetUserRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/users/{user_id}", user_id = self.params.user_id);
            let mut r = self.client.client.get(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get User

Get a user by user ID.*/
    pub fn get_user(
        &self,
        privy_app_id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, GetUserRequest> {
        FluentRequest {
            client: self,
            params: GetUserRequest {
                privy_app_id: privy_app_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
