use privy_rust::PrivySigner;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Example usage - demonstrates how to use the Privy signer with tk-rs interface
    
    // Get credentials from environment
    let app_id = std::env::var("PRIVY_APP_ID")
        .expect("PRIVY_APP_ID environment variable not set");
    let app_secret = std::env::var("PRIVY_APP_SECRET")
        .expect("PRIVY_APP_SECRET environment variable not set");
    let wallet_id = std::env::var("PRIVY_WALLET_ID")
        .expect("PRIVY_WALLET_ID environment variable not set");
    let public_key = std::env::var("PRIVY_PUBLIC_KEY")
        .expect("PRIVY_PUBLIC_KEY environment variable not set");
    
    // Create the signer (4th parameter is unused, just for tk-rs compatibility)
    let signer = PrivySigner::new(
        app_id,
        app_secret,
        wallet_id,
        String::new(), // unused parameter for tk-rs compatibility
        public_key,
    )?;
    
    println!("Privy signer initialized!");
    println!("Public key: {}", signer.solana_pubkey());
    
    // Example: sign a message
    let message = b"Hello, Privy!";
    let signature = signer.sign_solana(message).await?;
    println!("Signature: {}", signature);
    
    Ok(())
}