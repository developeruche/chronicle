use crate::Task;
use async_trait::async_trait;
use chronicle_primitives::ServerConfig;
use postgres::Client;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct ServerTask {
    pub config: ServerConfig,
}

#[async_trait]
impl Task for ServerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        Ok(())
    }
}

impl ServerTask {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}
