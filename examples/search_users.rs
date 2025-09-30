//! Search Users Example
//!
//! This example demonstrates how to search for users using various criteria.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Search for users by email, phone, wallet address, or other criteria
//! - Handle search results and user data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_SEARCH_TERM`: Search query (optional, defaults to "alex@arlyon.dev")
//!
//! ## Usage
//! ```bash
//! cargo run --example search_users
//! ```

use anyhow::Result;
use privy_rs::{PrivyClient, generated::types::SearchUsersBody};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Get search term from environment and initialize client
    let search_term =
        std::env::var("PRIVY_SEARCH_TERM").unwrap_or_else(|_| "alex@arlyon.dev".to_string());
    let client = PrivyClient::new_from_env()?;

    tracing::info!(
        "initialized privy client from environment, search_term: {}",
        search_term
    );

    // Search for users by email address or other criteria
    let search_result = client
        .users()
        // TODO: search term rename is not working
        .search(&SearchUsersBody::Variant0 { search_term })
        .await?;

    tracing::info!("search result: {:?}", search_result);

    Ok(())
}
