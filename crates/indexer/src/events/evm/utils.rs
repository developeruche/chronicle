use alloy::{
    dyn_abi::{DecodedEvent, DynSolEvent, DynSolType},
    primitives::{Address, Bytes, LogData, B256},
    providers::{Provider, RootProvider},
    pubsub::PubSubFrontend,
    rpc::types::eth::{BlockNumberOrTag, Filter},
};
use chronicle_primitives::indexer::ChronicleEvent;
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;

pub type EventStream<T> = Box<dyn Stream<Item = T> + Unpin>;

pub async fn query_events(
    provider: RootProvider<PubSubFrontend>,
    addr: Address,
    event_sig: B256,
    block_number: BlockNumberOrTag,
) -> Result<Vec<ChronicleEvent>, anyhow::Error> {
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(block_number);
    let log = provider.get_logs(&filter).await?;
    let chronicle_logs: Vec<ChronicleEvent> = log.into_iter().map(|log| log.into()).collect();
    Ok(chronicle_logs)
}

pub async fn subscribe_to_events<F>(
    provider: RootProvider<PubSubFrontend>,
    addr: Vec<Address>,
    event_sig: B256,
    mut callback: F,
) where
    F: FnMut(ChronicleEvent) + Send,
{
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider
        .subscribe_logs(&filter)
        .await
        .expect("Failed to subscribe to logs");
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        callback(log.into());
    }
}

/// This function is used to decode an event
/// params:
/// topics: Vec<B256> - The topics of the event
/// data: Bytes - The data of the event
/// decoder_format: DynSolType - The format of the event; example -> DynSolType::Tuple(
///     vec![
///         DynSolType::Uint(256)
///     ]
///  ),
/// indexed: Vec<DynSolType> - The indexed values of the event; example -> vec![DynSolType::Address]
pub fn decode_event(
    topics: Vec<B256>,
    data: Bytes,
    decoder_format: DynSolType,
    indexed: Vec<DynSolType>,
) -> Result<DecodedEvent, anyhow::Error> {
    let event: DynSolEvent = DynSolEvent::new_unchecked(Some(topics[0]), indexed, decoder_format);
    let log_data = LogData::new_unchecked(topics, data);
    let decoded_event = event.decode_log(&log_data, true)?;

    Ok(decoded_event)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloy::{
        primitives::{address, b256},
        providers::ProviderBuilder,
        rpc::client::WsConnect,
    };

    #[tokio::test]
    #[ignore]
    pub async fn test_query_events_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let block_num = 19664198u64;
        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let tranfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let events = query_events(
            provider,
            uniswap_token_address,
            tranfer_event_signature,
            block_num.into(),
        )
        .await
        .unwrap();

        for log in events {
            println!("Uniswap token logs: {log:?}");
        }
    }

    #[tokio::test]
    #[ignore]
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
    #[ignore]
    async fn test_subscribe_events_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let mut x = 0;

        let callback = |log: ChronicleEvent| {
            println!("Received log: {:?}", log);
            x += 1;
        };

        subscribe_to_events(
            provider,
            vec![uniswap_token_address],
            transfer_event_signature,
            callback,
        )
        .await;
    }

    #[tokio::test]
    async fn test_raw_subscribe_logs_can_be_decoded() {
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
            let decoded_log = decode_event(
                log.topics().to_owned(),
                log.inner.data.data,
                DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Address,
                    DynSolType::Uint(256),
                ]),
                vec![DynSolType::Address],
            );
            println!("Uniswap token logs: {decoded_log:?}");
            break;
        }
    }
}
