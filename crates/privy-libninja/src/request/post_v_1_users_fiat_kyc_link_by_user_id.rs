use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_fiat_kyc_link_by_user_id`].

On request success, this will return a [`PostV1UsersFiatKycLinkByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersFiatKycLinkByUserIdRequest {
    pub email: String,
    pub endorsements: Option<Vec<String>>,
    pub full_name: Option<String>,
    pub privy_app_id: String,
    pub provider: String,
    pub redirect_uri: Option<String>,
    pub type_: Option<String>,
    pub user_id: String,
}
pub struct PostV1UsersFiatKycLinkByUserIdRequired<'a> {
    pub email: &'a str,
    pub privy_app_id: &'a str,
    pub provider: &'a str,
    pub user_id: &'a str,
}
impl FluentRequest<'_, PostV1UsersFiatKycLinkByUserIdRequest> {
    ///Set the value of the endorsements field.
    pub fn endorsements(
        mut self,
        endorsements: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .endorsements = Some(
            endorsements.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the full_name field.
    pub fn full_name(mut self, full_name: &str) -> Self {
        self.params.full_name = Some(full_name.to_owned());
        self
    }
    ///Set the value of the redirect_uri field.
    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.params.redirect_uri = Some(redirect_uri.to_owned());
        self
    }
    ///Set the value of the type_ field.
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersFiatKycLinkByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1UsersFiatKycLinkByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/kyc_link", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "email" : self.params.email }));
            if let Some(ref unwrapped) = self.params.endorsements {
                r = r.json(serde_json::json!({ "endorsements" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.full_name {
                r = r.json(serde_json::json!({ "full_name" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "provider" : self.params.provider }));
            if let Some(ref unwrapped) = self.params.redirect_uri {
                r = r.json(serde_json::json!({ "redirect_uri" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(serde_json::json!({ "type" : unwrapped }));
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get a KYC link for a user

Returns a KYC link for a user*/
    pub fn post_v1_users_fiat_kyc_link_by_user_id(
        &self,
        args: PostV1UsersFiatKycLinkByUserIdRequired,
    ) -> FluentRequest<'_, PostV1UsersFiatKycLinkByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersFiatKycLinkByUserIdRequest {
                email: args.email.to_owned(),
                endorsements: None,
                full_name: None,
                privy_app_id: args.privy_app_id.to_owned(),
                provider: args.provider.to_owned(),
                redirect_uri: None,
                type_: None,
                user_id: args.user_id.to_owned(),
            },
        }
    }
}
