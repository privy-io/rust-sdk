use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::search_users`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersRequest {
    pub body: serde_json::Value,
    pub privy_app_id: String,
}
impl FluentRequest<'_, SearchUsersRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SearchUsersRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/search";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Search Users by Search Term

Search users by search term, emails, phone numbers, or wallet addresses.*/
    pub fn search_users(
        &self,
        body: serde_json::Value,
        privy_app_id: &str,
    ) -> FluentRequest<'_, SearchUsersRequest> {
        FluentRequest {
            client: self,
            params: SearchUsersRequest {
                body,
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
