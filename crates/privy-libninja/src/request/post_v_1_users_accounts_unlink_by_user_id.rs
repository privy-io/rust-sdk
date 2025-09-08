use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_accounts_unlink_by_user_id`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersAccountsUnlinkByUserIdRequest {
    pub handle: String,
    pub privy_app_id: String,
    pub provider: Option<String>,
    pub type_: String,
    pub user_id: String,
}
pub struct PostV1UsersAccountsUnlinkByUserIdRequired<'a> {
    pub handle: &'a str,
    pub privy_app_id: &'a str,
    pub type_: &'a str,
    pub user_id: &'a str,
}
impl FluentRequest<'_, PostV1UsersAccountsUnlinkByUserIdRequest> {
    ///Set the value of the provider field.
    pub fn provider(mut self, provider: &str) -> Self {
        self.params.provider = Some(provider.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersAccountsUnlinkByUserIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/accounts/unlink", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "handle" : self.params.handle }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.provider {
                r = r.json(serde_json::json!({ "provider" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Unlink a user linked account

Unlinks a user linked account.*/
    pub fn post_v1_users_accounts_unlink_by_user_id(
        &self,
        args: PostV1UsersAccountsUnlinkByUserIdRequired,
    ) -> FluentRequest<'_, PostV1UsersAccountsUnlinkByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersAccountsUnlinkByUserIdRequest {
                handle: args.handle.to_owned(),
                privy_app_id: args.privy_app_id.to_owned(),
                provider: None,
                type_: args.type_.to_owned(),
                user_id: args.user_id.to_owned(),
            },
        }
    }
}
