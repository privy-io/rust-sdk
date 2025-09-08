use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_twitter_subject`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersTwitterSubjectRequest {
    pub privy_app_id: String,
    pub subject: String,
}
impl FluentRequest<'_, PostV1UsersTwitterSubjectRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersTwitterSubjectRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/twitter/subject";
            let mut r = self.client.client.post(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "subject" : self.params.subject }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by Twitter Subject

Looks up a user by their Twitter subject.*/
    pub fn post_v1_users_twitter_subject(
        &self,
        privy_app_id: &str,
        subject: &str,
    ) -> FluentRequest<'_, PostV1UsersTwitterSubjectRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersTwitterSubjectRequest {
                privy_app_id: privy_app_id.to_owned(),
                subject: subject.to_owned(),
            },
        }
    }
}
