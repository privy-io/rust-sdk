use std::sync::OnceLock;
use std::borrow::Cow;
use httpclient::Client;
pub mod request;
pub mod model;
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
pub fn default_http_client() -> Client {
    Client::new().base_url("https://api.privy.io")
}
static SHARED_HTTPCLIENT: OnceLock<Client> = OnceLock::new();
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a PrivyLibninjaClient,
    pub params: T,
}
pub struct PrivyLibninjaClient {
    client: Cow<'static, Client>,
    authentication: PrivyLibninjaAuth,
}
impl PrivyLibninjaClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
            authentication: PrivyLibninjaAuth::from_env(),
        }
    }
    pub fn with_auth(authentication: PrivyLibninjaAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication,
        }
    }
    pub fn new(client: Client, authentication: PrivyLibninjaAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    }
}
impl PrivyLibninjaClient {
    pub(crate) fn _authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            PrivyLibninjaAuth::AppId { privy_app_id } => {
                r = r.header("privy-app-id", privy_app_id);
            }
        }
        r
    }
}
pub enum PrivyLibninjaAuth {
    AppId { privy_app_id: String },
}
impl PrivyLibninjaAuth {
    pub fn from_env() -> Self {
        Self::AppId {
            privy_app_id: std::env::var("PRIVY_LIBNINJA_PRIVY_APP_ID")
                .expect("Environment variable PRIVY_LIBNINJA_PRIVY_APP_ID is not set."),
        }
    }
}
