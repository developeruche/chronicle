use crate::Task;
use async_trait::async_trait;
use chronicle_primitives::IndexerConfig;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct IndexerTask {
    pub config: IndexerConfig,
}

#[async_trait]
impl Task for IndexerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        Ok(())
    }
}

impl IndexerTask {
    pub fn new(config: IndexerConfig) -> Self {
        Self { config }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}
