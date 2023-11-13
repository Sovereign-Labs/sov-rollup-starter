//! This binary runs the rollup full node.

use anyhow::Context;
use clap::Parser;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint, RollupProverConfig};
#[cfg(feature = "mock_da")]
use sov_rollup_starter::mock_rollup::MockRollup;
#[cfg(feature = "celestia_da")]
use sov_rollup_starter::celestia_rollup::CelestiaRollup;
#[cfg(feature = "mock_da")]
use sov_mock_da::MockDaConfig;
#[cfg(feature = "celestia_da")]
use sov_celestia_adapter::CelestiaConfig;
use sov_stf_runner::{from_toml_path, RollupConfig};
use std::str::FromStr;
use stf_starter::genesis_config::GenesisPaths;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

// config and genesis for mock da
#[cfg(feature = "mock_da")]
const DEFAULT_CONFIG_PATH: &str = "../../rollup_config.toml";
#[cfg(feature = "mock_da")]
const DEFAULT_GENESIS_PATH: &str = "../../test-data/genesis/mock/";

// config and genesis for local docker celestia
#[cfg(feature = "celestia_da")]
const DEFAULT_CONFIG_PATH: &str = "../../celestia_rollup_config.toml";
#[cfg(feature = "celestia_da")]
const DEFAULT_GENESIS_PATH: &str = "../../test-data/genesis/celestia/";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the rollup config.
    #[arg(long, default_value = DEFAULT_CONFIG_PATH)]
    rollup_config_path: String,

    /// The path to the genesis config.
    #[arg(long, default_value = DEFAULT_GENESIS_PATH)]
    genesis_paths: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initializing logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        //.with(EnvFilter::from_default_env())
        .with(EnvFilter::from_str("info,hyper=info").unwrap())
        .init();

    let args = Args::parse();
    let rollup_config_path = args.rollup_config_path.as_str();

    let genesis_paths = args.genesis_paths.as_str();

    let rollup = new_rollup(
        &GenesisPaths::from_dir(genesis_paths),
        rollup_config_path,
        Some(RollupProverConfig::Execute),
    )
    .await?;
    rollup.run().await
}

#[cfg(feature = "mock_da")]
async fn new_rollup(
    genesis_paths: &GenesisPaths,
    rollup_config_path: &str,
    prover_config: Option<RollupProverConfig>,
) -> Result<Rollup<MockRollup>, anyhow::Error> {
    info!("Reading rollup config from {rollup_config_path:?}");

    let rollup_config: RollupConfig<MockDaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;

    let mock_rollup = MockRollup {};
    mock_rollup
        .create_new_rollup(genesis_paths, rollup_config, prover_config)
        .await
}

#[cfg(feature = "celestia_da")]
async fn new_rollup(
    genesis_paths: &GenesisPaths,
    rollup_config_path: &str,
    prover_config: Option<RollupProverConfig>,
) -> Result<Rollup<CelestiaRollup>, anyhow::Error> {
    info!(
        "Starting celestia rollup with config {}",
        rollup_config_path
    );

    let rollup_config: RollupConfig<CelestiaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;

    let celestia_rollup = CelestiaRollup {};
    celestia_rollup
        .create_new_rollup(genesis_paths, rollup_config, prover_config)
        .await
}
