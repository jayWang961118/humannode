//! The utils we share among tests and benches - generally consider them both `dev`.

// The code may or may not be used depending on the feature flags - so omit the noise altogether and
// disable the check for the entire module.
#![allow(dead_code)]

use crypto_utils::{authority_keys_from_seed, get_account_id_from_seed};
use sp_runtime::app_crypto::sr25519;

use super::*;

/// The public key for the accounts.
pub type AccountPublic = <Signature as Verify>::Signer;

/// A helper function to return [`AccountId`] based on runtime data and provided seed.
pub fn account_id(seed: &str) -> AccountId {
    get_account_id_from_seed::<sr25519::Public, AccountPublic, AccountId>(seed)
}

/// A helper function to return authorities keys based on runtime data and provided seed.
pub fn authority_keys(seed: &str) -> (AccountId, BabeId, GrandpaId, ImOnlineId) {
    authority_keys_from_seed::<sr25519::Public, AccountPublic, AccountId>(seed)
}

/// A helper function to get a corresponding EVM truncated address for provided AccountId.
pub fn substrate_account_to_evm_account(account_id: AccountId) -> H160 {
    crypto_utils::substrate_account_to_evm_account(account_id)
}
