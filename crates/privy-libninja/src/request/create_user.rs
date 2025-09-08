use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{CustomMetadata, LinkedAccountInput};
/**You should use this struct via [`PrivyLibninjaClient::create_user`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub custom_metadata: Option<CustomMetadata>,
    pub linked_accounts: Vec<LinkedAccountInput>,
    pub privy_app_id: String,
    pub wallets: Option<Vec<serde_json::Value>>,
}
impl FluentRequest<'_, CreateUserRequest> {
    ///Set the value of the custom_metadata field.
    pub fn custom_metadata(mut self, custom_metadata: CustomMetadata) -> Self {
        self.params.custom_metadata = Some(custom_metadata);
        self
    }
    ///Set the value of the wallets field.
    pub fn wallets(mut self, wallets: Vec<serde_json::Value>) -> Self {
        self.params.wallets = Some(wallets);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateUserRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.custom_metadata {
                r = r.json(serde_json::json!({ "custom_metadata" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "linked_accounts" : self.params.linked_accounts }
                    ),
                );
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.wallets {
                r = r.json(serde_json::json!({ "wallets" : unwrapped }));
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create User

Create a new user with linked accounts. Optionally pre-generate embedded wallets for the user.*/
    pub fn create_user(
        &self,
        linked_accounts: Vec<LinkedAccountInput>,
        privy_app_id: &str,
    ) -> FluentRequest<'_, CreateUserRequest> {
        FluentRequest {
            client: self,
            params: CreateUserRequest {
                custom_metadata: None,
                linked_accounts,
                privy_app_id: privy_app_id.to_owned(),
                wallets: None,
            },
        }
    }
}
