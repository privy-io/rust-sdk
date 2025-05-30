use privy_rust::types::{PrivyConfig, PrivyError};
use privy_rust::PrivySignerBlocking;
use solana_sdk::signer::Signer;

#[tokio::main]
async fn main() -> Result<(), PrivyError> {
    // Example usage - this demonstrates how to use the Privy signer
    // In Kora, this would be integrated into their CLI arg parsing
    
    // Load from environment variables
    let config = PrivyConfig::from_env();
    
    // Check if required env vars are set
    if config.app_id.is_none() || config.app_secret.is_none() || config.wallet_id.is_none() {
        eprintln!("Error: Missing required environment variables.");
        eprintln!("Please set:");
        eprintln!("  PRIVY_APP_ID=<your-app-id>");
        eprintln!("  PRIVY_APP_SECRET=<your-app-secret>");
        eprintln!("  PRIVY_WALLET_ID=<your-wallet-id>");
        eprintln!("");
        eprintln!("For testing, you can use the test-with-creds.sh script.");
        return Err(PrivyError::MissingConfig("environment variables"));
    }
    
    // Build the signer
    let signer = config.build()?;
    
    // Create a blocking version for use with Solana SDK
    let blocking_signer = PrivySignerBlocking::new(signer)?;
    
    println!("Privy signer initialized!");
    println!("Public key: {}", blocking_signer.pubkey());
    
    // Example: sign a message
    let message = b"Hello, Privy!";
    let signature = blocking_signer.sign_message(message);
    println!("Signature: {}", signature);
    
    Ok(())
}
