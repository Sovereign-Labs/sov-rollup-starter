#![deny(missing_docs)]
//! StarterRollup provides a minimal self-contained rollup implementation

use async_trait::async_trait;
use sov_celestia_adapter::types::Namespace;
use sov_celestia_adapter::verifier::{CelestiaSpec, CelestiaVerifier, RollupParams};
use sov_celestia_adapter::{CelestiaConfig, CelestiaService};
use sov_db::ledger_db::LedgerDB;
use sov_modules_api::default_context::{DefaultContext, ZkDefaultContext};
use sov_modules_api::Spec;
use sov_modules_stf_template::kernels::basic::BasicKernel;
use sov_risc0_adapter::host::Risc0Host;
use sov_rollup_interface::services::da::DaService;
use sov_state::config::Config as StorageConfig;
use sov_state::{DefaultStorageSpec, ZkStorage};
use sov_stf_runner::RollupConfig;
use stf_starter::Runtime;

/// Rollup with [`CelestiaService`].
pub struct StarterRollup {}

/// The namespace for the rollup on Celestia. Must be kept in sync with the "guest-celestia"
pub const ROLLUP_NAMESPACE: Namespace = Namespace::const_v0(*b"sov-ibc-01");

/// This is the place, where all the rollup components come together and
/// they can be easily swapped with alternative implementations as needed.
#[async_trait]
impl sov_modules_rollup_blueprint::RollupBlueprint for StarterRollup {
    /// This component defines the Data Availability layer.
    type DaService = CelestiaService;
    /// DaSpec & DaConfig are derived from DaService.
    type DaSpec = CelestiaSpec;
    type DaConfig = CelestiaConfig;

    /// The concrete ZkVm used in the rollup.
    type Vm = Risc0Host<'static>;

    /// Context for the Zero Knowledge environment.
    type ZkContext = ZkDefaultContext;
    /// Context for the ZNative environment.
    type NativeContext = DefaultContext;

    /// Manager for the native storage lifecycle.
    type StorageManager = sov_state::storage_manager::ProverStorageManager<DefaultStorageSpec>;

    /// Runtime for the Zero Knowledge environment.
    type ZkRuntime = Runtime<Self::ZkContext, Self::DaSpec>;
    /// Runtime for the Native environment.
    type NativeRuntime = Runtime<Self::NativeContext, Self::DaSpec>;
    /// Kernels.
    type NativeKernel = BasicKernel<Self::NativeContext>;
    type ZkKernel = BasicKernel<Self::ZkContext>;

    /// This function generates RPC methods for the rollup, allowing for extension with custom endpoints.
    fn create_rpc_methods(
        &self,
        storage: &<Self::NativeContext as Spec>::Storage,
        ledger_db: &LedgerDB,
        da_service: &Self::DaService,
    ) -> anyhow::Result<jsonrpsee::RpcModule<()>> {
        sov_modules_rollup_blueprint::register_rpc::<
            Self::NativeRuntime,
            Self::NativeContext,
            Self::DaService,
        >(storage, ledger_db, da_service)
    }

    // Below, we provide the methods for setting up dependencies for the Rollup.

    async fn create_da_service(
        &self,
        rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> Self::DaService {
        CelestiaService::new(
            rollup_config.da.clone(),
            RollupParams {
                namespace: ROLLUP_NAMESPACE,
            },
        )
        .await
    }

    fn create_storage_manager(
        &self,
        rollup_config: &RollupConfig<Self::DaConfig>,
    ) -> anyhow::Result<Self::StorageManager> {
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
        Risc0Host::new(risc0_starter::CELESTIA_DA_ELF)
    }

    fn create_verifier(&self) -> <Self::DaService as DaService>::Verifier {
        CelestiaVerifier {
            rollup_namespace: ROLLUP_NAMESPACE,
        }
    }
}

// Here we get a `free` Wallet implementation.
impl sov_modules_rollup_blueprint::WalletBlueprint for StarterRollup {}

#[cfg(any(test, feature = "mocks"))]
/// Provides a version of the rollup which runs on mock da
pub mod mocks {
    use super::*;
    use sov_mock_da::{MockDaConfig, MockDaService, MockDaSpec, MockDaVerifier};

    /// Rollup with [`MockDa`].
    pub struct MockDaRollup {}

    /// This is the place, where all the rollup components come together and
    /// they can be easily swapped with alternative implementations as needed.
    #[async_trait]
    impl sov_modules_rollup_blueprint::RollupBlueprint for MockDaRollup {
        /// This component defines the Data Availability layer.
        type DaService = MockDaService;
        /// DaSpec & DaConfig are derived from DaService.
        type DaSpec = MockDaSpec;
        type DaConfig = MockDaConfig;

        /// The concrete ZkVm used in the rollup.
        type Vm = Risc0Host<'static>;

        /// Context for the Zero Knowledge environment.
        type ZkContext = ZkDefaultContext;
        /// Context for the ZNative environment.
        type NativeContext = DefaultContext;

        /// Manager for the native storage lifecycle.
        type StorageManager = sov_state::storage_manager::ProverStorageManager<DefaultStorageSpec>;

        /// Runtime for the Zero Knowledge environment.
        type ZkRuntime = Runtime<Self::ZkContext, Self::DaSpec>;
        /// Runtime for the Native environment.
        type NativeRuntime = Runtime<Self::NativeContext, Self::DaSpec>;
        /// Kernels.
        type NativeKernel = BasicKernel<Self::NativeContext>;
        type ZkKernel = BasicKernel<Self::ZkContext>;

        /// This function generates RPC methods for the rollup, allowing for extension with custom endpoints.
        fn create_rpc_methods(
            &self,
            storage: &<Self::NativeContext as Spec>::Storage,
            ledger_db: &LedgerDB,
            da_service: &Self::DaService,
        ) -> anyhow::Result<jsonrpsee::RpcModule<()>> {
            sov_modules_rollup_blueprint::register_rpc::<
                Self::NativeRuntime,
                Self::NativeContext,
                Self::DaService,
            >(storage, ledger_db, da_service)
        }

        // Below, we provide the methods for setting up dependencies for the Rollup.

        async fn create_da_service(
            &self,
            rollup_config: &RollupConfig<Self::DaConfig>,
        ) -> Self::DaService {
            MockDaService::new(rollup_config.da.sender_address)
        }

        fn create_storage_manager(
            &self,
            rollup_config: &RollupConfig<Self::DaConfig>,
        ) -> anyhow::Result<Self::StorageManager> {
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
            MockDaVerifier::default()
        }
    }

    // Here we get a `free` Wallet implementation.
    impl sov_modules_rollup_blueprint::WalletBlueprint for MockDaRollup {}
}
