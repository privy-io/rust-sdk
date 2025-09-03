use privy_api::Client;

#[derive(Clone, Debug)]
pub struct PrivySigner {
    pub(crate) app_id: String,
    #[allow(dead_code)]
    pub(crate) app_secret: String,
    pub(crate) wallet_id: String,
    pub(crate) client: Client,
    pub(crate) public_key: String,
}
