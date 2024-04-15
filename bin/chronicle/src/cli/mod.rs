use chronicle_primitives::Config;
use clap::Parser;
use toml::from_str;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliConfig {
    #[arg(short, long)]
    pub config_path: String,
}

/// Main entry point for the CLI
///
/// Parses the CLI arguments and runs the appropriate subcommand.
/// Listens for a ctrl-c signal and shuts down all components when received.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = CliConfig::parse();
    let config_str = std::fs::read_to_string(&config.config_path)?;
    let config: Config = from_str(&config_str)?;

    // server config
    let server_config = config.into_server();
    //indexer config
    let indexer_config = config.into_indexer();

    tracing::info!("Starting Chronicle with config: {:?}", config);

    Ok(())
}
