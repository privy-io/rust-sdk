use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_farcaster_fid`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersFarcasterFidRequest {
    pub fid: f64,
    pub privy_app_id: String,
}
impl FluentRequest<'_, PostV1UsersFarcasterFidRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersFarcasterFidRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/farcaster/fid";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "fid" : self.params.fid }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by Farcaster ID

Looks up a user by their Farcaster ID.*/
    pub fn post_v1_users_farcaster_fid(
        &self,
        fid: f64,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PostV1UsersFarcasterFidRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersFarcasterFidRequest {
                fid,
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
