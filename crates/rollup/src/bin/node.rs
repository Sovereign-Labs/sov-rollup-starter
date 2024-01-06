//! This binary runs the rollup full node.

use anyhow::Context;
use clap::Parser;
#[cfg(feature = "celestia_da")]
use sov_celestia_adapter::CelestiaConfig;
#[cfg(feature = "mock_da")]
use sov_mock_da::MockDaConfig;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint};
use sov_modules_stf_blueprint::kernels::basic::BasicKernelGenesisConfig;
use sov_modules_stf_blueprint::kernels::basic::BasicKernelGenesisPaths;
#[cfg(feature = "celestia_da")]
use sov_rollup_starter::celestia_rollup::CelestiaRollup;
#[cfg(feature = "mock_da")]
use sov_rollup_starter::mock_rollup::MockRollup;
use sov_stf_runner::RollupProverConfig;
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
        &BasicKernelGenesisPaths {
            chain_state: "../test-data/genesis/demo-tests/mock/chain_state.json".into(),
        },
        rollup_config_path,
        RollupProverConfig::Execute,
    )
    .await?;
    rollup.run().await
}

#[cfg(feature = "mock_da")]
async fn new_rollup(
    rt_genesis_paths: &GenesisPaths,
    kernel_genesis_paths: &BasicKernelGenesisPaths,
    rollup_config_path: &str,
    prover_config: RollupProverConfig,
) -> Result<Rollup<MockRollup>, anyhow::Error> {
    info!("Reading rollup config from {rollup_config_path:?}");

    let rollup_config: RollupConfig<MockDaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;

    let mock_rollup = MockRollup {};

    let kernel_genesis = BasicKernelGenesisConfig {
        chain_state: serde_json::from_str(
            &std::fs::read_to_string(&kernel_genesis_paths.chain_state)
                .context("Failed to read chain state")?,
        )?,
    };

    mock_rollup
        .create_new_rollup(
            rt_genesis_paths,
            kernel_genesis,
            rollup_config,
            prover_config,
        )
        .await
}

#[cfg(feature = "celestia_da")]
async fn new_rollup(
    rt_genesis_paths: &GenesisPaths,
    kernel_genesis_paths: &BasicKernelGenesisPaths,
    rollup_config_path: &str,
    prover_config: RollupProverConfig,
) -> Result<Rollup<CelestiaRollup>, anyhow::Error> {
    info!(
        "Starting celestia rollup with config {}",
        rollup_config_path
    );

    let rollup_config: RollupConfig<CelestiaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;

    let kernel_genesis = BasicKernelGenesisConfig {
        chain_state: serde_json::from_str(
            &std::fs::read_to_string(&kernel_genesis_paths.chain_state)
                .context("Failed to read chain state")?,
        )?,
    };

    let mock_rollup = CelestiaRollup {};
    mock_rollup
        .create_new_rollup(
            rt_genesis_paths,
            kernel_genesis,
            rollup_config,
            prover_config,
        )
        .await
}
