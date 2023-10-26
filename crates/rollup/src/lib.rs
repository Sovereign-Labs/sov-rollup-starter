//#![deny(missing_docs)]
//#![doc = include_str!("../README.md")]

use async_trait::async_trait;
use sov_db::ledger_db::LedgerDB;
use sov_modules_api::default_context::{DefaultContext, ZkDefaultContext};
use sov_modules_api::Spec;
use sov_modules_rollup_template::{register_rpc, RollupTemplate, WalletTemplate};
use sov_risc0_adapter::host::Risc0Host;
use sov_rollup_interface::mocks::{MockDaConfig, MockDaService, MockDaSpec};
use sov_rollup_interface::services::da::DaService;
use sov_state::config::Config as StorageConfig;
use sov_state::{DefaultStorageSpec, ZkStorage};
use sov_stf_runner::RollupConfig;
use stf_starter::Runtime;

/// Rollup with [`MockDaService`].
pub struct StarterRollup {}

#[async_trait]
impl RollupTemplate for StarterRollup {
    type DaService = MockDaService;
    type DaSpec = MockDaSpec;

    type DaConfig = MockDaConfig;
    type Vm = Risc0Host<'static>;

    type ZkContext = ZkDefaultContext;
    type NativeContext = DefaultContext;

    type StorageManager = sov_state::storage_manager::ProverStorageManager<DefaultStorageSpec>;
    type ZkRuntime = Runtime<Self::ZkContext, Self::DaSpec>;

    type NativeRuntime = Runtime<Self::NativeContext, Self::DaSpec>;

    fn create_rpc_methods(
        &self,
        storage: &<Self::NativeContext as Spec>::Storage,
        ledger_db: &LedgerDB,
        da_service: &Self::DaService,
    ) -> anyhow::Result<jsonrpsee::RpcModule<()>> {
        register_rpc::<Self::NativeRuntime, Self::NativeContext, Self::DaService>(
            storage, ledger_db, da_service,
        )
    }

    async fn create_da_service(
        &self,
        rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> Self::DaService {
        MockDaService::new(rollup_config.da.sender_address)
    }

    fn create_storage_manager(&self, rollup_config: &RollupConfig<Self::DaConfig>) -> anyhow::Result<Self::StorageManager> {
        let storage_config = StorageConfig {
            path: rollup_config.storage.path.clone(),
        };
        sov_state::storage_manager::ProverStorageManager::new(storage_config)
    }

    fn create_zk_storage(
        &self,
        _rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> <Self::ZkContext as Spec>::Storage {
        ZkStorage::new()
    }

    fn create_vm(&self) -> Self::Vm {
        Risc0Host::new(risc0_starter::MOCK_DA_ELF)
    }

    fn create_verifier(&self) -> <Self::DaService as DaService>::Verifier {
        Default::default()
    }
}

impl WalletTemplate for StarterRollup {}
