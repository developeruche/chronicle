use alloy::{
    primitives::Address,
    providers::{Provider, RootProvider},
    pubsub::PubSubFrontend,
    rpc::types::eth::{BlockTransactions, Transaction},
};
use futures_util::stream::StreamExt;

pub async fn subscribe_transactions<F>(
    index_address: Address,
    provider: RootProvider<PubSubFrontend>,
    mut callback: F,
) -> Result<(), anyhow::Error>
where
    F: FnMut(Vec<Transaction>),
{
    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream();

    while let Some(block) = stream.next().await {
        match block.transactions {
            BlockTransactions::Full(txs) => {
                let filtered_txs = txs
                    .into_iter()
                    .filter(|tx| {
                        println!("Captured Tx: {:?}", tx.hash);
                        let to_address = match tx.to {
                            Some(to) => to,
                            None => Address::ZERO,
                        };

                        tx.from == index_address || to_address == index_address
                    })
                    .collect::<Vec<Transaction>>();

                callback(filtered_txs);
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloy::{primitives::address, providers::ProviderBuilder, rpc::client::WsConnect};

    #[tokio::test]
    #[ignore]
    pub async fn test_subscribe_transactions_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
        let usdc_token_address = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

        let callback = |tx: Vec<Transaction>| {
            println!("Received Tx: {:?}", tx);
        };

        subscribe_transactions(usdc_token_address, provider, callback)
            .await
            .unwrap();
    }
}
