//! The mock for the pallet.

// Allow simple integer arithmetic in tests.
#![allow(clippy::integer_arithmetic)]

use frame_support::{
    parameter_types, sp_io,
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    traits::{ConstU32, ConstU64, FindAuthor},
    weights::Weight,
    ConsensusEngineId, PalletId,
};
use pallet_evm::{EnsureAddressNever, FixedGasWeightMapping, IdentityAddressMapping};
use sp_core::{H160, H256, U256};
use sp_std::str::FromStr;

use crate::{self as pallet_bridge_pot_balances};

pub(crate) const EXISTENTIAL_DEPOSIT_NATIVE: u64 = 20;
pub(crate) const EXISTENTIAL_DEPOSIT_EVM: u64 = 10;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type AccountId = u64;
pub(crate) type EvmAccountId = H160;
type Balance = u64;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        EVM: pallet_evm,
        EvmSystem: pallet_evm_system,
        EvmBalances: pallet_evm_balances,
        NativeToEvmSwapBridgePot: pallet_pot::<Instance1>,
        EvmToNativeSwapBridgePot: pallet_pot::<Instance2>,
        BridgePotBalances: pallet_bridge_pot_balances,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<EXISTENTIAL_DEPOSIT_NATIVE>;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl pallet_evm_system::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountId = EvmAccountId;
    type Index = u64;
    type AccountData = pallet_evm_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
}

impl pallet_evm_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountId = EvmAccountId;
    type Balance = Balance;
    type ExistentialDeposit = ConstU64<EXISTENTIAL_DEPOSIT_EVM>;
    type AccountStore = EvmSystem;
    type DustRemoval = ();
}

pub struct FixedGasPrice;

impl pallet_evm::FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> (U256, Weight) {
        // Return some meaningful gas price and weight
        (1_000_000_000u128.into(), Weight::from_ref_time(7u64))
    }
}

pub struct FindAuthorTruncated;

impl FindAuthor<H160> for FindAuthorTruncated {
    fn find_author<'a, I>(_digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        Some(H160::from_str("1234500000000000000000000000000000000000").unwrap())
    }
}

parameter_types! {
    pub BlockGasLimit: U256 = U256::max_value();
    pub WeightPerGas: Weight = Weight::from_ref_time(20_000);
}

impl pallet_evm::Config for Test {
    type AccountProvider = EvmSystem;
    type FeeCalculator = FixedGasPrice;
    type GasWeightMapping = FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
    type CallOrigin =
        EnsureAddressNever<<Self::AccountProvider as pallet_evm::AccountProvider>::AccountId>;
    type WithdrawOrigin =
        EnsureAddressNever<<Self::AccountProvider as pallet_evm::AccountProvider>::AccountId>;
    type AddressMapping = IdentityAddressMapping;
    type Currency = EvmBalances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = ();
    type PrecompilesValue = ();
    type ChainId = ();
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type FindAuthor = FindAuthorTruncated;
}

parameter_types! {
    pub const NativeToEvmSwapBridgePotPalletId: PalletId = PalletId(*b"hmcs/ne1");
    pub const EvmToNativeSwapBridgePotPalletId: PalletId = PalletId(*b"hmcs/en1");
}

type PotInstanceNativeToEvmSwapBridge = pallet_pot::Instance1;
type PotInstanceEvmToNativeSwapBridge = pallet_pot::Instance2;

impl pallet_pot::Config<PotInstanceNativeToEvmSwapBridge> for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountId = AccountId;
    type PalletId = NativeToEvmSwapBridgePotPalletId;
    type Currency = Balances;
}

impl pallet_pot::Config<PotInstanceEvmToNativeSwapBridge> for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountId = EvmAccountId;
    type PalletId = EvmToNativeSwapBridgePotPalletId;
    type Currency = EvmBalances;
}

parameter_types! {
    pub NativeToEvmSwapBridgePotAccountId: AccountId = NativeToEvmSwapBridgePot::account_id();
    pub EvmToNativeSwapBridgePotAccountId: EvmAccountId = EvmToNativeSwapBridgePot::account_id();
}

impl pallet_bridge_pot_balances::Config for Test {
    type NativeToEvmSwapBridgePot = NativeToEvmSwapBridgePotAccountId;
    type EvmToNativeSwapBridgePot = EvmToNativeSwapBridgePotAccountId;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext_with(genesis_config: GenesisConfig) -> sp_io::TestExternalities {
    let storage = genesis_config.build_storage().unwrap();
    storage.into()
}

pub fn runtime_lock() -> std::sync::MutexGuard<'static, ()> {
    static MOCK_RUNTIME_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    // Ignore the poisoning for the tests that panic.
    // We only care about concurrency here, not about the poisoning.
    match MOCK_RUNTIME_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

pub trait TestExternalitiesExt {
    fn execute_with_ext<R, E>(&mut self, execute: E) -> R
    where
        E: for<'e> FnOnce(&'e ()) -> R;
}

impl TestExternalitiesExt for frame_support::sp_io::TestExternalities {
    fn execute_with_ext<R, E>(&mut self, execute: E) -> R
    where
        E: for<'e> FnOnce(&'e ()) -> R,
    {
        let guard = runtime_lock();
        let result = self.execute_with(|| execute(&guard));
        drop(guard);
        result
    }
}
