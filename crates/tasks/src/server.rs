use crate::Task;
use anyhow::bail;
use async_trait::async_trait;
use chronicle_primitives::ServerConfig;
use chronicle_server::{query::ChronicleQuery, run_chronicle_server};
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct ServerTask {
    pub config: ServerConfig,
}

#[async_trait]
impl Task for ServerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        let server_handle = tokio::spawn(async move {
            select! {
                server = run_chronicle_server(self.config, ChronicleQuery) => {
                    // Want this indexing to halt before
                    if server.is_err() {
                        info!("GraphQL server failed to start");
                    }
                }
                _ = shutdown_token.cancelled() => {
                    info!("Shutting down chronicle server");
                }
            }
        });

        match try_join!(server_handle) {
            Ok(_) => {
                info!("Server task completed");
            }
            Err(e) => bail!("Error running server: {:?}", e),
        }
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
