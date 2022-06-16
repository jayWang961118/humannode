//! The implementation of the various bits and pieces that we use throughut the ssytem to ensure
//! the fixed supply.

use core::marker::PhantomData;

use frame_support::traits::fungible::Inspect;
use frame_support::traits::{
    Currency as CurrencyT, Imbalance, OnUnbalanced, SameOrOther, SignedImbalance, TryDrop,
};

use super::*;

/// A wrapper around [`Balances`] that attempts to ensure fixed supply but panicing on
/// any of the operations that would result in a change of the total issuance.
pub struct Currency(Balances);

/// The [`PositiveImbalance`] wrapper that panics on non-zero imbalance drop.
/// Ensures the fixed fee by preventing the operations that change the total issuance.
#[derive(Default)]
pub struct PositiveImbalance(Option<<Balances as CurrencyT<AccountId>>::PositiveImbalance>);

/// The [`NegativeImbalance`] wrapper that panics on non-zero imbalance drop.
/// Ensures the fixed fee by preventing the operations that change the total issuance.
#[derive(Default)]
pub struct NegativeImbalance(Option<<Balances as CurrencyT<AccountId>>::NegativeImbalance>);

impl PositiveImbalance {
    fn new(val: <Balances as CurrencyT<AccountId>>::PositiveImbalance) -> Self {
        Self(Some(val))
    }

    fn must_take(&mut self) -> <Balances as CurrencyT<AccountId>>::PositiveImbalance {
        self.0.take().unwrap()
    }

    fn must_ref(&self) -> &<Balances as CurrencyT<AccountId>>::PositiveImbalance {
        self.0.as_ref().unwrap()
    }
}

impl NegativeImbalance {
    fn new(val: <Balances as CurrencyT<AccountId>>::NegativeImbalance) -> Self {
        Self(Some(val))
    }

    fn must_take(&mut self) -> <Balances as CurrencyT<AccountId>>::NegativeImbalance {
        self.0.take().unwrap()
    }

    fn must_ref(&self) -> &<Balances as CurrencyT<AccountId>>::NegativeImbalance {
        self.0.as_ref().unwrap()
    }

    fn must_mut(&mut self) -> &mut <Balances as CurrencyT<AccountId>>::NegativeImbalance {
        self.0.as_mut().unwrap()
    }
}

impl CurrencyT<AccountId> for Currency {
    type Balance = <Balances as CurrencyT<AccountId>>::Balance;

    type PositiveImbalance = PositiveImbalance;

    type NegativeImbalance = NegativeImbalance;

    fn total_balance(who: &AccountId) -> Self::Balance {
        <Balances as CurrencyT<AccountId>>::total_balance(who)
    }

    fn can_slash(who: &AccountId, value: Self::Balance) -> bool {
        <Balances as CurrencyT<AccountId>>::can_slash(who, value)
    }

    fn total_issuance() -> Self::Balance {
        <Balances as CurrencyT<AccountId>>::total_issuance()
    }

    fn minimum_balance() -> Self::Balance {
        <Balances as CurrencyT<AccountId>>::minimum_balance()
    }

    fn burn(_amount: Self::Balance) -> Self::PositiveImbalance {
        panic!("no");
    }

    fn issue(_amount: Self::Balance) -> Self::NegativeImbalance {
        panic!("no");
    }

    fn free_balance(who: &AccountId) -> Self::Balance {
        <Balances as CurrencyT<AccountId>>::free_balance(who)
    }

    fn ensure_can_withdraw(
        who: &AccountId,
        amount: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
        new_balance: Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        <Balances as CurrencyT<AccountId>>::ensure_can_withdraw(who, amount, reasons, new_balance)
    }

    fn transfer(
        source: &AccountId,
        dest: &AccountId,
        value: Self::Balance,
        existence_requirement: frame_support::traits::ExistenceRequirement,
    ) -> frame_support::dispatch::DispatchResult {
        <Balances as CurrencyT<AccountId>>::transfer(source, dest, value, existence_requirement)
    }

    fn slash(who: &AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        let (imbalance, amount) = <Balances as CurrencyT<AccountId>>::slash(who, value);
        (NegativeImbalance::new(imbalance), amount)
    }

    fn deposit_into_existing(
        who: &AccountId,
        value: Self::Balance,
    ) -> Result<Self::PositiveImbalance, sp_runtime::DispatchError> {
        <Balances as CurrencyT<AccountId>>::deposit_into_existing(who, value)
            .map(PositiveImbalance::new)
    }

    fn deposit_creating(who: &AccountId, value: Self::Balance) -> Self::PositiveImbalance {
        PositiveImbalance::new(<Balances as CurrencyT<AccountId>>::deposit_creating(
            who, value,
        ))
    }

    fn withdraw(
        who: &AccountId,
        value: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
        liveness: frame_support::traits::ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, sp_runtime::DispatchError> {
        <Balances as CurrencyT<AccountId>>::withdraw(who, value, reasons, liveness)
            .map(NegativeImbalance::new)
    }

    fn make_free_balance_be(
        who: &AccountId,
        balance: Self::Balance,
    ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        match <Balances as CurrencyT<AccountId>>::make_free_balance_be(who, balance) {
            SignedImbalance::Positive(val) => {
                SignedImbalance::Positive(PositiveImbalance::new(val))
            }
            SignedImbalance::Negative(val) => {
                SignedImbalance::Negative(NegativeImbalance::new(val))
            }
        }
    }
}

impl Inspect<AccountId> for Currency {
    type Balance = <Balances as Inspect<AccountId>>::Balance;

    fn total_issuance() -> Self::Balance {
        <Balances as Inspect<AccountId>>::total_issuance()
    }

    fn minimum_balance() -> Self::Balance {
        <Balances as Inspect<AccountId>>::minimum_balance()
    }

    fn balance(who: &AccountId) -> Self::Balance {
        <Balances as Inspect<AccountId>>::balance(who)
    }

    fn reducible_balance(who: &AccountId, keep_alive: bool) -> Self::Balance {
        <Balances as Inspect<AccountId>>::reducible_balance(who, keep_alive)
    }

    fn can_deposit(
        who: &AccountId,
        amount: Self::Balance,
        mint: bool,
    ) -> frame_support::traits::tokens::DepositConsequence {
        <Balances as Inspect<AccountId>>::can_deposit(who, amount, mint)
    }

    fn can_withdraw(
        who: &AccountId,
        amount: Self::Balance,
    ) -> frame_support::traits::tokens::WithdrawConsequence<Self::Balance> {
        <Balances as Inspect<AccountId>>::can_withdraw(who, amount)
    }
}

impl Imbalance<Balance> for PositiveImbalance {
    type Opposite = NegativeImbalance;

    fn zero() -> Self {
        Self::new(<Balances as CurrencyT<AccountId>>::PositiveImbalance::zero())
    }

    fn drop_zero(mut self) -> Result<(), Self> {
        self.must_take().drop_zero().map_err(Self::new)
    }

    fn split(mut self, amount: Balance) -> (Self, Self) {
        let (left, right) = self.must_take().split(amount);
        (Self::new(left), Self::new(right))
    }

    fn merge(mut self, mut other: Self) -> Self {
        Self::new(self.must_take().merge(other.must_take()))
    }

    fn subsume(&mut self, mut other: Self) {
        self.must_take().subsume(other.must_take())
    }

    fn offset(mut self, mut other: Self::Opposite) -> SameOrOther<Self, Self::Opposite> {
        match self.must_take().offset(other.must_take()) {
            SameOrOther::None => SameOrOther::None,
            SameOrOther::Same(val) => SameOrOther::Same(Self::new(val)),
            SameOrOther::Other(val) => SameOrOther::Other(NegativeImbalance::new(val)),
        }
    }

    fn peek(&self) -> Balance {
        self.must_ref().peek()
    }
}

impl Imbalance<Balance> for NegativeImbalance {
    type Opposite = PositiveImbalance;

    fn zero() -> Self {
        Self::new(<Balances as CurrencyT<AccountId>>::NegativeImbalance::zero())
    }

    fn drop_zero(mut self) -> Result<(), Self> {
        self.must_take().drop_zero().map_err(Self::new)
    }

    fn split(mut self, amount: Balance) -> (Self, Self) {
        let (left, right) = self.must_take().split(amount);
        (Self::new(left), Self::new(right))
    }

    fn merge(mut self, mut other: Self) -> Self {
        Self::new(self.must_take().merge(other.must_take()))
    }

    fn subsume(&mut self, mut other: Self) {
        self.must_mut().subsume(other.must_take())
    }

    fn offset(mut self, mut other: Self::Opposite) -> SameOrOther<Self, Self::Opposite> {
        match self.must_take().offset(other.must_take()) {
            SameOrOther::None => SameOrOther::None,
            SameOrOther::Same(val) => SameOrOther::Same(Self::new(val)),
            SameOrOther::Other(val) => SameOrOther::Other(PositiveImbalance::new(val)),
        }
    }

    fn peek(&self) -> Balance {
        self.must_ref().peek()
    }
}

impl TryDrop for PositiveImbalance {
    fn try_drop(mut self) -> Result<(), Self> {
        self.must_take().try_drop().map_err(Self::new)
    }
}

impl TryDrop for NegativeImbalance {
    fn try_drop(mut self) -> Result<(), Self> {
        self.must_take().try_drop().map_err(Self::new)
    }
}

impl Drop for PositiveImbalance {
    fn drop(&mut self) {
        let val = match &self.0 {
            None => return,
            Some(val) => val,
        };

        if val != &<Balances as CurrencyT<AccountId>>::PositiveImbalance::zero() {
            panic!("dropping a non-zero positive imbalance")
        }
    }
}

impl Drop for NegativeImbalance {
    fn drop(&mut self) {
        let val = match &self.0 {
            None => return,
            Some(val) => val,
        };

        if val != &<Balances as CurrencyT<AccountId>>::NegativeImbalance::zero() {
            panic!("dropping a non-zero negative imbalance")
        }
    }
}

/// An imbalance handler that will panic on any non-zero imbalance, effectively preventing
/// the system from adjusting the total issuance in any direction, while also aborting (ideally)
/// the action that caused in this adjustment.
///
/// This is just a failsafe mechanism, the real fix is to avoid the operation that would lead to
/// the change in the total issuance in the first place, rather than `panic`-crash them here.
pub struct ImbalanceHandler<Imbalance>(PhantomData<Imbalance>);

impl<Imbalance: TryDrop> OnUnbalanced<Imbalance> for ImbalanceHandler<Imbalance> {
    fn on_nonzero_unbalanced(_amount: Imbalance) {
        panic!("non-zero imbalance not settled");
    }
}
