#![no_main]
//! This binary implements the verification logic for the rollup. This is the code that runs inside
//! of the zkvm in order to generate proofs for the rollup.

use sov_celestia_adapter::types::Namespace;
use sov_celestia_adapter::verifier::CelestiaVerifier;
use sov_modules_api::default_context::ZkDefaultContext;
use sov_modules_stf_template::kernels::basic::BasicKernel;
use sov_modules_stf_template::AppTemplate;
use sov_risc0_adapter::guest::Risc0Guest;
use sov_state::ZkStorage;
use stf_starter::runtime::Runtime;
use stf_starter::AppVerifier;

risc0_zkvm::guest::entry!(main);
/// The namespace for the rollup on Celestia. Must be kept in sync with the "rollup/lib.rs"
pub const ROLLUP_NAMESPACE: Namespace = Namespace::const_v0(*b"sov-ibc-01");

pub fn main() {
    let guest = Risc0Guest::new();
    let storage = ZkStorage::new();
    let app: AppTemplate<ZkDefaultContext, _, _, Runtime<_, _>, BasicKernel<_>> =
        AppTemplate::new();

    let mut stf_verifier = AppVerifier::new(
        app,
        CelestiaVerifier {
            rollup_namespace: ROLLUP_NAMESPACE,
        },
    );

    stf_verifier
        .run_block(guest, storage)
        .expect("Prover must be honest");
}
