#[cfg(feature = "mock_da")]
pub mod mock_rollup;

#[cfg(feature = "celestia_da")]
pub mod celestia_rollup;

#[cfg(feature = "mock_da")]
use mock_rollup::StarterRollup;

#[cfg(feature = "celestia_da")]
use celestia_rollup::StarterRollup;

// Wallet implementation doesn't depend on the adapter so creating the impl here
impl sov_modules_rollup_blueprint::WalletBlueprint for StarterRollup {}
