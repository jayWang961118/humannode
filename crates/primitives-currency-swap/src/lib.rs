//! Currency swap related primitives.

// Either generate code at stadard mode, or `no_std`, based on the `std` feature presence.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    sp_runtime::DispatchError,
    traits::fungible::{Balanced, Credit, Inspect},
};

/// Currency swap interface.
pub trait CurrencySwap<AccountIdFrom, AccountIdTo> {
    /// The currency to convert from.
    type From: Inspect<AccountIdFrom> + Balanced<AccountIdFrom>;

    /// The currency to convert to.
    type To: Inspect<AccountIdTo> + Balanced<AccountIdTo>;

    /// A possible error happens during the actual swap logic.
    type Error: Into<DispatchError>;

    /// The actual swap logic.
    fn swap(
        imbalance: Credit<AccountIdFrom, Self::From>,
    ) -> Result<Credit<AccountIdTo, Self::To>, ErrorFor<Self, AccountIdFrom, AccountIdTo>>;

    /// Obtain the estimated resulted balance value.
    fn estimate_swapped_balance(
        balance: FromBalanceFor<Self, AccountIdFrom, AccountIdTo>,
    ) -> ToBalanceFor<Self, AccountIdFrom, AccountIdTo>;
}

/// An easy way to access the [`Currency::Balance`] of [`CurrencySwap::From`] of `T`.
pub type FromBalanceFor<T, AccountIdFrom, AccountIdTo> =
    <<T as CurrencySwap<AccountIdFrom, AccountIdTo>>::From as Inspect<AccountIdFrom>>::Balance;

/// An easy way to access the [`Currency::Balance`] of [`CurrencySwap::To`] of `T`.
pub type ToBalanceFor<T, AccountIdFrom, AccountIdTo> =
    <<T as CurrencySwap<AccountIdFrom, AccountIdTo>>::To as Inspect<AccountIdTo>>::Balance;

/// A type alias for compact declaration of the error type for the [`CurrencySwap::swap`] call.
pub type ErrorFor<T, AccountIdFrom, AccountIdTo> = Error<
    Credit<AccountIdFrom, <T as CurrencySwap<AccountIdFrom, AccountIdTo>>::From>,
    <T as CurrencySwap<AccountIdFrom, AccountIdTo>>::Error,
>;

/// An error that can occur while doing a currency swap.
#[derive(Debug)]
pub struct Error<I, E> {
    /// The underlying cause of this error.
    pub cause: E,
    /// The original imbalance that was passed to the swap operation.
    pub incoming_imbalance: I,
}
