//! An implementation that does not require pot account existence and can potentially kill the
//! pot account by withdrawing all the funds from it.

use frame_support::{
    sp_runtime::{traits::Convert, DispatchError},
    traits::{
        fungible::{Balanced, Credit, Inspect},
        tokens::{Fortitude, Precision, Preservation},
        Get, Imbalance,
    },
};

use super::{Config, CurrencySwap};

/// A marker type for the implementation that does not require pot accounts existence.
pub enum Marker {}

impl<T: Config> primitives_currency_swap::CurrencySwap<T::AccountIdFrom, T::AccountIdTo>
    for CurrencySwap<T, Marker>
{
    type From = T::CurrencyFrom;
    type To = T::CurrencyTo;
    type Error = DispatchError;

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
            Preservation::Expendable,
            Fortitude::Force,
        ) {
            Ok(imbalance) => imbalance,
            Err(error) => {
                return Err(primitives_currency_swap::Error {
                    cause: error,
                    incoming_imbalance,
                })
            }
        };

        T::CurrencyFrom::resolve(&T::PotFrom::get(), incoming_imbalance);

        Ok(outgoing_imbalance)
    }

    fn estimate_swapped_balance(
        balance: <Self::From as Inspect<T::AccountIdFrom>>::Balance,
    ) -> <Self::To as Inspect<T::AccountIdTo>>::Balance {
        T::BalanceConverter::convert(balance)
    }
}
