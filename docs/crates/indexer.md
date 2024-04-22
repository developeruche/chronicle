## Chronicle-Indexer
--------------------------------
This crate is the core of the Chronicle, it is responsible for indexing transactions and modules (Smart contract, Parachain, and so on) Events.

The `Event` part of this crate is divided into two main components:
1. A component responsible for query events that from a specifiec block number to the latest block number.
2. Another component responsible for subscribing to new events and indexing them.


this module implements the `ChronicleEventIndexer` trait which posses methods like `query_events` and `subscribe_events` which are responsible for querying and subscribing to events respectively.

```rust
#[async_trait]
pub trait ChronicleEventIndexer {
    type SubProvider;
    type ContractAddress;
    type EventSignature;
    type BlockNumber;

    /// This function queries events from a specified block number
    /// `[Filter]`, having `address`, `last_block` and `event_signature` as parameters
    async fn query_events(
        &self,
        provider: Self::SubProvider,
        addr: Self::ContractAddress,
        event_sig: Self::EventSignature,
        block_nuber: Self::BlockNumber,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error>;

    /// This creates a filter and subscribes to an event returning the event
    /// stream <T: Stream<Item = Resp> + Unpin>
    async fn subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error>;
}
```

Any blockchain and it to be suported by the Chronicle must implement the `ChronicleEventIndexer` trait.


As it is well know that Chronicle indexs Events, Chronicle also indexs transactions, this is done by the `ChronicleTransactionIndexer` trait.

```rust
/// This transaction indexer trait would be used across all supported chains
#[async_trait]
pub trait ChronicleTransactionIndexer {
    type SubProvider;
    type TargetAddress;

    /// This function subscribes to blocks and filters transactions based on the index address.
    /// Uses a callback closure to output the filter tx
    async fn subscribe_transactions<F>(
        &self,
        index_address: Self::TargetAddress,
        provider: Self::SubProvider,
        callback: F,
    ) -> Result<(), anyhow::Error>
    where
        F: FnMut(Vec<ChronicleTransaction>);
}

```
