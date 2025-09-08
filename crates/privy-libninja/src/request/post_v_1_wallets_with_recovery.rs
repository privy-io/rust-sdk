use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_wallets_with_recovery`].

On request success, this will return a [`PostV1WalletsWithRecoveryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1WalletsWithRecoveryRequest {
    pub primary_signer: serde_json::Value,
    pub privy_app_id: String,
    pub recovery_user: serde_json::Value,
    pub wallets: Vec<serde_json::Value>,
}
pub struct PostV1WalletsWithRecoveryRequired<'a> {
    pub primary_signer: serde_json::Value,
    pub privy_app_id: &'a str,
    pub recovery_user: serde_json::Value,
    pub wallets: Vec<serde_json::Value>,
}
impl FluentRequest<'_, PostV1WalletsWithRecoveryRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1WalletsWithRecoveryRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1WalletsWithRecoveryResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets_with_recovery";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "primary_signer" : self.params.primary_signer }),
                );
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r
                .json(
                    serde_json::json!({ "recovery_user" : self.params.recovery_user }),
                );
            r = r.json(serde_json::json!({ "wallets" : self.params.wallets }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    ///Create wallets with an associated recovery user.
    pub fn post_v1_wallets_with_recovery(
        &self,
        args: PostV1WalletsWithRecoveryRequired,
    ) -> FluentRequest<'_, PostV1WalletsWithRecoveryRequest> {
        FluentRequest {
            client: self,
            params: PostV1WalletsWithRecoveryRequest {
                primary_signer: args.primary_signer,
                privy_app_id: args.privy_app_id.to_owned(),
                recovery_user: args.recovery_user,
                wallets: args.wallets,
            },
        }
    }
}
