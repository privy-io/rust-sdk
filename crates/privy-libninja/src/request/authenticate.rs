use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::authenticate`].

On request success, this will return a [`AuthenticateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub encryption_type: Option<String>,
    pub privy_app_id: String,
    pub recipient_public_key: Option<String>,
    pub user_jwt: String,
}
impl FluentRequest<'_, AuthenticateRequest> {
    ///Set the value of the encryption_type field.
    pub fn encryption_type(mut self, encryption_type: &str) -> Self {
        self.params.encryption_type = Some(encryption_type.to_owned());
        self
    }
    ///Set the value of the recipient_public_key field.
    pub fn recipient_public_key(mut self, recipient_public_key: &str) -> Self {
        self.params.recipient_public_key = Some(recipient_public_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AuthenticateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AuthenticateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets/authenticate";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.encryption_type {
                r = r.json(serde_json::json!({ "encryption_type" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.recipient_public_key {
                r = r.json(serde_json::json!({ "recipient_public_key" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user_jwt" : self.params.user_jwt }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    ///Obtain a session key to enable wallet access.
    pub fn authenticate(
        &self,
        privy_app_id: &str,
        user_jwt: &str,
    ) -> FluentRequest<'_, AuthenticateRequest> {
        FluentRequest {
            client: self,
            params: AuthenticateRequest {
                encryption_type: None,
                privy_app_id: privy_app_id.to_owned(),
                recipient_public_key: None,
                user_jwt: user_jwt.to_owned(),
            },
        }
    }
}
