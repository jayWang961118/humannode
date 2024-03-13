// DO NOT EDIT!
//! Autogenerated weights for `pallet_vesting`

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
  fn unlock() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `488`
    //  Estimated: `0`
    // Minimum execution time: 30_000_000 picoseconds.
    Weight::from_parts(30_000_000, 0)
      .saturating_add(T::DbWeight::get().reads(6))
      .saturating_add(T::DbWeight::get().writes(3))
  }
  fn update_schedule() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `488`
    //  Estimated: `0`
    // Minimum execution time: 32_000_000 picoseconds.
    Weight::from_parts(32_000_000, 0)
      .saturating_add(T::DbWeight::get().reads(6))
      .saturating_add(T::DbWeight::get().writes(3))
  }
}
