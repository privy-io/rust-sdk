use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_users`].

On request success, this will return a [`GetUsersResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersRequest {
    pub cursor: Option<String>,
    pub limit: Option<f64>,
    pub privy_app_id: String,
}
impl FluentRequest<'_, GetUsersRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: f64) -> Self {
        self.params.limit = Some(limit);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetUsersRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetUsersResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users";
            let mut r = self.client.client.get(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.query("cursor", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.limit {
                r = r.query("limit", &unwrapped.to_string());
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Gets Users

Get all users in your app.*/
    pub fn get_users(&self, privy_app_id: &str) -> FluentRequest<'_, GetUsersRequest> {
        FluentRequest {
            client: self,
            params: GetUsersRequest {
                cursor: None,
                limit: None,
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
