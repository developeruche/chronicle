use crate::Task;
use async_trait::async_trait;
use chronicle_primitives::ServerConfig;
use chronicle_server::{query::ChronicleQuery, run_chronicle_server};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct ServerTask {
    pub config: ServerConfig
}

#[async_trait]
impl Task for ServerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        tokio::spawn(async move {
            select! {
                server = run_chronicle_server(self.config, ChronicleQuery) => {
                    // Want this indexing to halt before
                    if server.is_err() {
                        info!("GraphQL server failed to start");
                    }
                }
                _ = shutdown_token.cancelled() => {
                    info!("Shutting down chain watcher");
                }
            }
        });
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
