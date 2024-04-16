use alloy::{
    primitives::{Address, B256}, providers::{Provider, ReqwestProvider, RootProvider}, pubsub::PubSubFrontend,
    rpc::types::eth::{BlockNumberOrTag, Filter, Log}
};
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;



pub type EventStream<T> = Box<dyn Stream<Item = T> + Unpin>;

/// This function queries events from a specified block number
/// `[Filter]`, having `address`, `last_block` and `event_signature` as parameters
pub async fn query_events(
    provider: ReqwestProvider,
    addr: Address,
    event_sig: B256,
    block_number: BlockNumberOrTag,
) -> Result<Vec<Log>, anyhow::Error> {
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(block_number);
    let log = provider.get_logs(&filter).await?;
    Ok(log)
}


/// This creates a filter and subscribes to an event returning the event stream <T: Stream<Item = Resp> + Unpin>
pub async fn subscribe_to_events<F>(
    provider: RootProvider<PubSubFrontend>,
    addr: Vec<Address>,
    event_sig: B256,
    mut callback: F,
)
where
    F: FnMut(Log)
{
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await.expect("Failed to subscribe to logs");
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        callback(log);
    }
}



#[cfg(test)]
pub mod tests {
    use alloy::{primitives::{address, b256}, providers::ProviderBuilder, rpc::client::WsConnect};
    use super::*;

    #[tokio::test]
    pub async fn test_query_events_works() {
        let rpc_url = "https://eth.merkle.io".parse().unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url).unwrap();

        let block_num = 19664198u64;
        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let tranfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let events = query_events(provider, uniswap_token_address, tranfer_event_signature, block_num.into()).await.unwrap();

        for log in events {
            println!("Uniswap token logs: {log:?}");
        }
    }

    #[tokio::test]
    async fn test_raw_subscribe_logs() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let addresses = vec![uniswap_token_address];
        let filter = Filter::new()
            .address(addresses)
            .event_signature(transfer_event_signature)
            .from_block(BlockNumberOrTag::Latest);


        let sub = provider.subscribe_logs(&filter).await.unwrap();
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            println!("Uniswap token logs: {log:?}");
            break;
        }
    }


    #[tokio::test]
    async fn test_subscribe_events_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let callback = |log: Log| {
                println!("Received log: {:?}", log);
                let opt: Option<String> = None;
                opt.unwrap();
        };

        subscribe_to_events(provider, vec![uniswap_token_address], transfer_event_signature, callback).await;
    }
}
