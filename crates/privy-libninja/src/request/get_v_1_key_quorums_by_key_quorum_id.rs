use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_v1_key_quorums_by_key_quorum_id`].

On request success, this will return a [`KeyQuorum`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetV1KeyQuorumsByKeyQuorumIdRequest {
    pub key_quorum_id: String,
    pub privy_app_id: String,
}
impl FluentRequest<'_, GetV1KeyQuorumsByKeyQuorumIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetV1KeyQuorumsByKeyQuorumIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::KeyQuorum>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/key_quorums/{key_quorum_id}", key_quorum_id = self.params
                .key_quorum_id
            );
            let mut r = self.client.client.get(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get key quorum

Get a key quorum by ID.*/
    pub fn get_v1_key_quorums_by_key_quorum_id(
        &self,
        key_quorum_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, GetV1KeyQuorumsByKeyQuorumIdRequest> {
        FluentRequest {
            client: self,
            params: GetV1KeyQuorumsByKeyQuorumIdRequest {
                key_quorum_id: key_quorum_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
