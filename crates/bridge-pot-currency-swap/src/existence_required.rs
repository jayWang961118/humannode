//! An implementation that requires and ensures pot account existence.

use frame_support::{
    sp_runtime::{traits::Convert, DispatchError},
    traits::{
        fungible::{Balanced, Credit, Inspect},
        tokens::{Fortitude, Precision, Preservation},
        Get, Imbalance,
    },
};

use super::{Config, CurrencySwap};

/// A marker type for the implementation that requires pot accounts existence.
pub enum Marker {}

/// An error that can occur while doing the swap operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Unable to resolve the incoming balance into the corresponding pot.
    ResolvingIncomingImbalance,
    /// Unable to withdraw the outgoing balance from the corresponding pot.
    IssuingOutgoingImbalance(DispatchError),
}

impl From<Error> for DispatchError {
    fn from(value: Error) -> Self {
        match value {
            Error::ResolvingIncomingImbalance => {
                DispatchError::Other("swap pot account does not exist")
            }
            Error::IssuingOutgoingImbalance(err) => err,
        }
    }
}

impl<T: Config> primitives_currency_swap::CurrencySwap<T::AccountIdFrom, T::AccountIdTo>
    for CurrencySwap<T, Marker>
{
    type From = T::CurrencyFrom;
    type To = T::CurrencyTo;
    type Error = Error;

    fn swap(
        incoming_imbalance: Credit<T::AccountIdFrom, Self::From>,
    ) -> Result<
        Credit<T::AccountIdTo, Self::To>,
        primitives_currency_swap::ErrorFor<Self, T::AccountIdFrom, T::AccountIdTo>,
    > {
        let amount = incoming_imbalance.peek();

        let outgoing_imbalance = match T::CurrencyTo::withdraw(
            &T::PotTo::get(),
            T::BalanceConverter::convert(amount),
            Precision::Exact,
            Preservation::Preserve,
            Fortitude::Force,
        ) {
            Ok(imbalance) => imbalance,
            Err(error) => {
                return Err(primitives_currency_swap::Error {
                    cause: Error::IssuingOutgoingImbalance(error),
                    incoming_imbalance,
                });
            }
        };

        match T::CurrencyFrom::resolve(&T::PotFrom::get(), incoming_imbalance) {
            Ok(()) => {}
            Err(imbalance) => {
                T::CurrencyTo::resolve(&T::PotTo::get(), outgoing_imbalance);
                return Err(primitives_currency_swap::Error {
                    cause: Error::ResolvingIncomingImbalance,
                    incoming_imbalance: imbalance,
                });
            }
        }

        Ok(outgoing_imbalance)
    }

    fn estimate_swapped_balance(
        balance: <Self::From as Inspect<T::AccountIdFrom>>::Balance,
    ) -> <Self::To as Inspect<T::AccountIdTo>>::Balance {
        T::BalanceConverter::convert(balance)
    }
}
