use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_fiat_offramp_by_user_id`].

On request success, this will return a [`PostV1UsersFiatOfframpByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersFiatOfframpByUserIdRequest {
    pub amount: String,
    pub destination: serde_json::Value,
    pub privy_app_id: String,
    pub provider: String,
    pub source: serde_json::Value,
    pub user_id: String,
}
pub struct PostV1UsersFiatOfframpByUserIdRequired<'a> {
    pub amount: &'a str,
    pub destination: serde_json::Value,
    pub privy_app_id: &'a str,
    pub provider: &'a str,
    pub source: serde_json::Value,
    pub user_id: &'a str,
}
impl FluentRequest<'_, PostV1UsersFiatOfframpByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersFiatOfframpByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1UsersFiatOfframpByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/offramp", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r.json(serde_json::json!({ "destination" : self.params.destination }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "provider" : self.params.provider }));
            r = r.json(serde_json::json!({ "source" : self.params.source }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Initiate an offramp transaction

Triggers the offramp flow and gets the on-chain address to send funds to*/
    pub fn post_v1_users_fiat_offramp_by_user_id(
        &self,
        args: PostV1UsersFiatOfframpByUserIdRequired,
    ) -> FluentRequest<'_, PostV1UsersFiatOfframpByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersFiatOfframpByUserIdRequest {
                amount: args.amount.to_owned(),
                destination: args.destination,
                privy_app_id: args.privy_app_id.to_owned(),
                provider: args.provider.to_owned(),
                source: args.source,
                user_id: args.user_id.to_owned(),
            },
        }
    }
}
