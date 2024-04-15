use alloy::{
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::eth::{BlockNumberOrTag, Filter, Log},
};

/// This function queries events from a specified block number
/// `[Filter]`, having `address`, `last_block` and `event_signature` as parameters
pub async fn query_events(
    provider: impl Provider,
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

/// This function subscribes to events
pub fn subscribe_events() {
    println!("Subscribing to events");
}
