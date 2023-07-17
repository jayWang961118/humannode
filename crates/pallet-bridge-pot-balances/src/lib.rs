//! Bridge pot balances.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, StorageVersion};
pub use pallet::*;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

// We have to temporarily allow some clippy lints. Later on we'll send patches to substrate to
// fix them at their end.
#[allow(clippy::missing_docs_in_private_items)]
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;

    use super::*;

    /// The Bridge Pot Balances Pallet
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configuration trait of this pallet.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_balances::Config
        + pallet_evm_balances::Config<Balance = <Self as pallet_balances::Config>::Balance>
    {
        /// The bridge pot account for the native currency.
        type NativeToEvmSwapBridgePot: Get<<Self as frame_system::Config>::AccountId>;
        /// The bridge pot account for the evm currency.
        type EvmToNativeSwapBridgePot: Get<<Self as pallet_evm_balances::Config>::AccountId>;
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config>(PhantomData<T>);

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let total_native = pallet_balances::Pallet::<T>::total_issuance();
            let total_evm = pallet_evm_balances::Pallet::<T>::total_issuance();

            let native_to_evm_swap_bridge_pot_balance =
                pallet_balances::Pallet::<T>::total_balance(&T::NativeToEvmSwapBridgePot::get());
            let evm_to_native_swap_bridge_pot_balance =
                pallet_evm_balances::Pallet::<T>::total_balance(&T::EvmToNativeSwapBridgePot::get());

            let ed_native = <T as pallet_balances::Config>::ExistentialDeposit::get();
            let ed_evm = <T as pallet_evm_balances::Config>::ExistentialDeposit::get();

            assert!(
                ((native_to_evm_swap_bridge_pot_balance - ed_native)
                    == (total_evm - evm_to_native_swap_bridge_pot_balance))
                    && ((evm_to_native_swap_bridge_pot_balance - ed_evm)
                        == (total_native - native_to_evm_swap_bridge_pot_balance)),
                "invalid bridge pot balances values at genesis"
            );
        }
    }
}
