use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_fiat_accounts_by_user_id`].

On request success, this will return a [`PostV1UsersFiatAccountsByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersFiatAccountsByUserIdRequest {
    pub account: Option<serde_json::Value>,
    pub account_owner_name: String,
    pub address: Option<serde_json::Value>,
    pub bank_name: Option<String>,
    pub currency: String,
    pub first_name: Option<String>,
    pub iban: Option<serde_json::Value>,
    pub last_name: Option<String>,
    pub privy_app_id: String,
    pub provider: String,
    pub swift: Option<serde_json::Value>,
    pub user_id: String,
}
pub struct PostV1UsersFiatAccountsByUserIdRequired<'a> {
    pub account_owner_name: &'a str,
    pub currency: &'a str,
    pub privy_app_id: &'a str,
    pub provider: &'a str,
    pub user_id: &'a str,
}
impl FluentRequest<'_, PostV1UsersFiatAccountsByUserIdRequest> {
    ///Set the value of the account field.
    pub fn account(mut self, account: serde_json::Value) -> Self {
        self.params.account = Some(account);
        self
    }
    ///Set the value of the address field.
    pub fn address(mut self, address: serde_json::Value) -> Self {
        self.params.address = Some(address);
        self
    }
    ///Set the value of the bank_name field.
    pub fn bank_name(mut self, bank_name: &str) -> Self {
        self.params.bank_name = Some(bank_name.to_owned());
        self
    }
    ///Set the value of the first_name field.
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.params.first_name = Some(first_name.to_owned());
        self
    }
    ///Set the value of the iban field.
    pub fn iban(mut self, iban: serde_json::Value) -> Self {
        self.params.iban = Some(iban);
        self
    }
    ///Set the value of the last_name field.
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.params.last_name = Some(last_name.to_owned());
        self
    }
    ///Set the value of the swift field.
    pub fn swift(mut self, swift: serde_json::Value) -> Self {
        self.params.swift = Some(swift);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersFiatAccountsByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1UsersFiatAccountsByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/accounts", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.account {
                r = r.json(serde_json::json!({ "account" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "account_owner_name" : self.params.account_owner_name }
                    ),
                );
            if let Some(ref unwrapped) = self.params.address {
                r = r.json(serde_json::json!({ "address" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bank_name {
                r = r.json(serde_json::json!({ "bank_name" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "currency" : self.params.currency }));
            if let Some(ref unwrapped) = self.params.first_name {
                r = r.json(serde_json::json!({ "first_name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iban {
                r = r.json(serde_json::json!({ "iban" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.last_name {
                r = r.json(serde_json::json!({ "last_name" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "provider" : self.params.provider }));
            if let Some(ref unwrapped) = self.params.swift {
                r = r.json(serde_json::json!({ "swift" : unwrapped }));
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create a fiat account

Sets up external bank account object for the user through the configured default provider. Requires the user to already be KYC'ed.*/
    pub fn post_v1_users_fiat_accounts_by_user_id(
        &self,
        args: PostV1UsersFiatAccountsByUserIdRequired,
    ) -> FluentRequest<'_, PostV1UsersFiatAccountsByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersFiatAccountsByUserIdRequest {
                account: None,
                account_owner_name: args.account_owner_name.to_owned(),
                address: None,
                bank_name: None,
                currency: args.currency.to_owned(),
                first_name: None,
                iban: None,
                last_name: None,
                privy_app_id: args.privy_app_id.to_owned(),
                provider: args.provider.to_owned(),
                swift: None,
                user_id: args.user_id.to_owned(),
            },
        }
    }
}
