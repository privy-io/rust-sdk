use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CustomMetadata;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_custom_metadata_by_user_id`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersCustomMetadataByUserIdRequest {
    pub custom_metadata: CustomMetadata,
    pub privy_app_id: String,
    pub user_id: String,
}
impl FluentRequest<'_, PostV1UsersCustomMetadataByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersCustomMetadataByUserIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/custom_metadata", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "custom_metadata" : self.params.custom_metadata }
                    ),
                );
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create Custom Metadata

Adds custom metadata to a user by user ID.*/
    pub fn post_v1_users_custom_metadata_by_user_id(
        &self,
        custom_metadata: CustomMetadata,
        privy_app_id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, PostV1UsersCustomMetadataByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersCustomMetadataByUserIdRequest {
                custom_metadata,
                privy_app_id: privy_app_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
