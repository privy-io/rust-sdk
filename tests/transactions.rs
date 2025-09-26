use anyhow::Result;
use privy_rs::generated::types::{
    WalletChainType, WalletTransactionsAsset, WalletTransactionsAssetString,
    WalletTransactionsChain,
};

mod common;

#[tokio::test]
#[ignore = "currently untested in other SDKs and is failing"]
async fn test_transactions_get() -> Result<()> {
    let client = common::get_test_client()?;

    // Get a wallet to fetch transactions from
    let wallet_id =
        common::get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    // Get the first transaction from wallet transactions
    let transactions_response = debug_response!(client.wallets().transactions().get(
        &wallet_id,
        &WalletTransactionsAsset::String(WalletTransactionsAssetString::Sol),
        WalletTransactionsChain::Base,
        None,      // No cursor for first page
        Some(1.0), // Limit to 1 transaction to get one ID
    ))
    .await?;

    let transactions = transactions_response.into_inner();

    // If we have transactions, test getting one by ID
    if !transactions.transactions.is_empty() {
        let transaction_id = &transactions.transactions[0].privy_transaction_id;

        // Test the transactions.get() endpoint
        let result = client.transactions().get(transaction_id).await?;

        println!("Retrieved transaction: {:?}", result);
        assert_eq!(result.into_inner().id, *transaction_id);
    } else {
        println!(
            "No transactions found for wallet {}, skipping transaction get test",
            wallet_id
        );
    }

    Ok(())
}
