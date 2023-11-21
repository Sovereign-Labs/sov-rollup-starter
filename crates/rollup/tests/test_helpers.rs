use std::net::SocketAddr;

use sov_mock_da::{MockAddress, MockDaConfig};
use sov_modules_rollup_blueprint::RollupBlueprint;
use sov_rollup_starter::mock_rollup::MockRollup;
use sov_stf_runner::RollupProverConfig;
use sov_stf_runner::{RollupConfig, RpcConfig, RunnerConfig, StorageConfig};
use stf_starter::genesis_config::GenesisPaths;
use tokio::sync::oneshot;

pub async fn start_rollup(
    rpc_reporting_channel: oneshot::Sender<SocketAddr>,
    genesis_paths: GenesisPaths,
    rollup_prover_config: RollupProverConfig,
) {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let rollup_config = RollupConfig {
        storage: StorageConfig {
            path: temp_path.to_path_buf(),
        },
        runner: RunnerConfig {
            start_height: 0,
            rpc_config: RpcConfig {
                bind_host: "127.0.0.1".into(),
                bind_port: 0,
            },
        },
        da: MockDaConfig {
            sender_address: MockAddress::from([0; 32]),
        },
    };

    let mock_rollup = MockRollup {};

    let rollup = mock_rollup
        .create_new_rollup(&genesis_paths, rollup_config, rollup_prover_config)
        .await
        .unwrap();

    rollup
        .run_and_report_rpc_port(Some(rpc_reporting_channel))
        .await
        .unwrap();

    // Close the tempdir explicitly to ensure that rustc doesn't see that it's unused and drop it unexpectedly
    temp_dir.close().unwrap();
}
