/// This index events
pub mod events;
/// This index tx
pub mod tx;



#[cfg(test)]
pub mod tests {
    use alloy::{primitives::{address, b256}, providers::ProviderBuilder};

    use super::*;

    #[tokio::test]
    pub async fn test_query_events_works() {
        let rpc_url = "https://eth.merkle.io".parse().unwrap();
        let provider = ProviderBuilder::new().on_reqwest_http(rpc_url).unwrap();

        let block_num = 19664198u64;
        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let tranfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let events = events::evm::query_events(provider, uniswap_token_address, tranfer_event_signature, block_num.into()).await.unwrap();

        for log in events {
            println!("Uniswap token logs: {log:?}");
        }
    }
}
