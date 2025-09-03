use privy_api::Client;

#[derive(Clone, Debug)]
pub struct PrivySigner {
    pub app_id: String,
    pub app_secret: String,
    pub wallet_id: String,
    pub client: Client,
    pub public_key: String,
}
