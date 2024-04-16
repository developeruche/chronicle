use alloy::{
    primitives::{Address, B256},
    providers::{Provider, ReqwestProvider},
    rpc::types::eth::{BlockNumberOrTag, Filter, Log}
};
use futures_core::stream::Stream;


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
pub async fn subscribe_events(
    provier: ReqwestProvider,
    addr: Address,
    event_sig: B256,
) -> Result<EventStream<Vec<Log>>, anyhow::Error> {
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(BlockNumberOrTag::Latest);
    let event = provier.watch_logs(&filter).await?;
    let stream = event.into_stream();

    Ok(Box::new(stream))
}
