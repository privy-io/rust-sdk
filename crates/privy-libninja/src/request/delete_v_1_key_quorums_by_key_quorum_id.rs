use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::delete_v1_key_quorums_by_key_quorum_id`].

On request success, this will return a [`DeleteV1KeyQuorumsByKeyQuorumIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteV1KeyQuorumsByKeyQuorumIdRequest {
    pub key_quorum_id: String,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
}
impl FluentRequest<'_, DeleteV1KeyQuorumsByKeyQuorumIdRequest> {
    ///Set the value of the privy_authorization_signature field.
    pub fn privy_authorization_signature(
        mut self,
        privy_authorization_signature: &str,
    ) -> Self {
        self
            .params
            .privy_authorization_signature = Some(
            privy_authorization_signature.to_owned(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteV1KeyQuorumsByKeyQuorumIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::DeleteV1KeyQuorumsByKeyQuorumIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/key_quorums/{key_quorum_id}", key_quorum_id = self.params
                .key_quorum_id
            );
            let mut r = self.client.client.delete(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Delete key quorum

Delete a key quorum by key quorum ID.*/
    pub fn delete_v1_key_quorums_by_key_quorum_id(
        &self,
        key_quorum_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, DeleteV1KeyQuorumsByKeyQuorumIdRequest> {
        FluentRequest {
            client: self,
            params: DeleteV1KeyQuorumsByKeyQuorumIdRequest {
                key_quorum_id: key_quorum_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
            },
        }
    }
}
