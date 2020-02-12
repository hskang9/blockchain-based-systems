//! # Loan Module
//!
//! ## Overview
//!
//! A module that enables lending/borrowing with the currency in substrate.
//!
//! ## Interface
//!
//! This module implements peer-to-peer loan and bond.
//!
//! ### Dispatchable Functions
//!
//! - `request` - Request loan and get bond_id to get approval from others.
//! - `approve` - Approver accepts loan request and lend money for the borrower
//! - `redeem` - Borrower ends loan by paying back the borrowed money with interest.
//! - `repossess` - Lender takes collateral from borrower when the bond expires.
//! - `sell_bond` - Lender sets bond for sale
//! - `buy_bond` - Other lender takses lender's bond for sale by paying it.
//!  

use crate::RandomnessCollectiveFlip;
use codec::{Decode, Encode};
use primitives::H256;
use rstd::prelude::*;
use sr_primitives::weights::SimpleDispatchInfo;
use support::dispatch::Parameter;
use support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result,
    ensure,
    traits::Randomness,
    traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReason, WithdrawReasons},
};
use system::ensure_signed;

pub type Reason = Vec<u8>;

#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Subscription<Balance: Parameter, AccountId: Parameter, BlockNumber: Parameter> {
    subscribers: Vec<AccountId>,
    to: AccountId,
    paying_for: Reason,
    amount: Balance,
    period: BlockNumber,
    next_payment: BlockNumber,
}

impl<Balance: Parameter, AccountId: Parameter, BlockNumber: Parameter>
    Subscription<Balance, AccountId, BlockNumber>
{
    pub fn new(
        subscriber: Option<AccountId>,
        to: AccountId,
        paying_for: Reason,
        amount: Balance,
        period: BlockNumber,
        next_payment: BlockNumber,
    ) -> Self {
        Subscription {
            subscribers,
            to,
            paying_for,
            amount,
            period,
            next_payment,
        }
    }

    pub fn add_subscriber(&mut self, subscriber: AccountId) {
        self.subscribers.push(subscriber);
    }
}

// Module's function and Methods of custom struct to be placed here
impl<T: Trait> Module<T> {
    pub fn activate_payment(
        mut subscription Subscription<T::Balance, T::AccountId, T::BlockNumber>,
        subscriber: T::AccountId
    ) -> Subscription<T::Balance, T::AccountId, T::BlockNumber> {
        <balances::Module<T> as Currency<_>>::transfer(
            &subscriber.clone(),
            &subcription.clone().to,
            subscription.clone().amount,
        )
        .expect("Transfer the lending amount from lender to borrower");
        subscription.next_payment += subcription.period;
        subscription
    }

    pub fn process_subscription((subscription_id, subscriber): (H256, T::AccountId)) -> Result {
        let mut updated_subscription = Self::subscription(subscription_id.clone());
        updated_subscription = Self::activate_payment(updated_subscription.clone(), subscriber.clone());
        <Subscriptions<T>>::mutate(subscription_id.clone(), |s| *s = updated_subscription.clone());
        let next = updated_subscription.clone().next_payment;
        if <SubscriptionCallBacks<T>>::exists(next.clone()) {
            <SubscriptionCallBacks<T>>::mutate(next.clone(), |c| {
                c.push(subscription_id);
            });
            Self::deposit_event(RawEvent::SubscriptionPaid(
                bond_id.clone(),
                before,
                updated_bond.clone().amount,
                next.clone(),
            ));
        } else {
            <SubscriptionCallBacks<T>>::insert(next.clone(), vec![subscription_id]);
            Self::deposit_event(RawEvent::SubscriptionPaid(
                bond_id.clone(),
                before,
                updated_bond.clone().amount,
                next.clone(),
            ));
        }
        return Ok(());
    }

    pub fn to_balance(u: u32, digit: &str) -> T::Balance {
        let power = |u: u32, p: u32| -> T::Balance {
            let mut base = T::Balance::from(u);
            for _i in 0..p {
                base *= T::Balance::from(10)
            }
            return base;
        };
        let result = match digit {
            "femto" => T::Balance::from(u),
            "nano" => power(u, 3),
            "micro" => power(u, 6),
            "milli" => power(u, 9),
            "one" => power(u, 12),
            "kilo" => power(u, 15),
            "mega" => power(u, 18),
            "giga" => power(u, 21),
            "tera" => power(u, 24),
            "peta" => power(u, 27),
            "exa" => power(u, 30),
            "zetta" => power(u, 33),
            "yotta" => power(u, 36),
            _ => T::Balance::from(u),
        };
        result
    }
}

/// The module's configuration trait.
pub trait Trait: system::Trait + balances::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Loan {
        pub Subscriptions get(subscription): map H256 => Subscription<T::Balance, T::AccountId, T::BlockNumber>;
        pub SubscriptionCallBacks get(callback): map T::BlockNumber => Vec<(H256, T::AccountId)>;
    }
}

decl_module! {

    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        fn deposit_event() = default;
        fn on_finalize(block_number: T::BlockNumber) {
            let bonds = Self::callback(block_number);
            for i in bonds {
                if let Err(e) = Self::process_subscription(i) {
                    sr_primitives::print(e);
                }
            }
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn generate_subscription(origin, lender: T::AccountId, amount: T::Balance, expires_at: T::BlockNumber, interest: T::Balance, collateral: T::Balance, period: T::BlockNumber) -> Result {
            let provider = ensure_signed(origin)?;
            let subscription_hash = H256::from_slice(&RandomnessCollectiveFlip::random_seed().encode() as &[u8]);
            ensure!(!<Subscriptions<T>>::exists(subscription_hash.clone()), "Hash collision!");
            new_subscription = Subscription::new()
            <Subscriptions<T>>::insert(subscription_hash, new_bond.clone());
            Self::deposit_event(RawEvent::LoanRequested(new_bond.clone().lender, new_bond.clone().borrower, bond_hash.clone()));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn approve(origin, bond_id: H256) -> Result {
            let approver = ensure_signed(origin)?;
            ensure!(<Bonds<T>>::exists(bond_id.clone()), "bond does not exist");
            let mut the_bond = Self::bond(bond_id.clone());
            ensure!(the_bond.sender_is_lender(approver.clone()), "You are not the lender for this borrower");
            the_bond.approve(approver);
            let current_block = <system::Module<T>>::block_number();
            <Bonds<T>>::mutate(bond_id, |l| {*l = the_bond.clone()});
            <balances::Module<T>>::set_lock(COLLATERAL_ID, &the_bond.clone().borrower, the_bond.clone().collateral,  T::BlockNumber::from(100000000), WithdrawReasons::all());
            <balances::Module<T> as Currency<_>>::transfer(&the_bond.clone().lender, &the_bond.clone().borrower, the_bond.clone().amount).expect("Transfer the lending amount from lender to borrower");
            Self::process_bond(bond_id.clone(), current_block.clone()).expect("loan operation is stored in a callback storage");
            Self::deposit_event(RawEvent::LoanApproved(the_bond.clone().lender, the_bond.clone().borrower, current_block.clone()));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn redeem(origin, bond_id: H256) -> Result {
            let redeemer = ensure_signed(origin)?;
            ensure!(<Bonds<T>>::exists(bond_id.clone()), "Loan does not exist");
            let the_bond = Self::bond(bond_id.clone());
            ensure!(the_bond.clone().sender_is_borrower(redeemer), "You are not the redeemer for this loan");
            <balances::Module<T>>::remove_lock(COLLATERAL_ID, &the_bond.clone().borrower);
            <balances::Module<T> as Currency<_>>::transfer(&the_bond.clone().borrower, &the_bond.clone().lender, the_bond.clone().amount).expect("Transfer the owed amount from borrower to lender");
            <LoanCallBacks<T>>::remove(the_bond.clone().next_increment);
            <Bonds<T>>::remove(bond_id);
            Self::deposit_event(RawEvent::LoanRedeemed(the_bond.clone().lender, the_bond.clone().borrower, the_bond.clone().amount));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn repossess(origin, bond_id: H256) -> Result {
            let lender = ensure_signed(origin)?;
            ensure!(<Bonds<T>>::exists(bond_id.clone()), "Loan does not exist");
            let the_bond = Self::bond(bond_id.clone());
            ensure!(the_bond.clone().sender_is_lender(lender), "You are not the lender for this loan");
            let current_block = <system::Module<T>>::block_number();
            ensure!(the_bond.clone().expires_at.unwrap() < current_block, "The bond is not expired yet");
            <balances::Module<T>>::remove_lock(COLLATERAL_ID, &the_bond.clone().borrower);
            <balances::Module<T> as Currency<_>>::transfer(&the_bond.clone().borrower, &the_bond.clone().lender, the_bond.clone().collateral).expect("Transfer the collateral amount from borrower to lender");
            <LoanCallBacks<T>>::remove(the_bond.clone().next_increment);
            <Bonds<T>>::remove(bond_id);
            Self::deposit_event(RawEvent::LoanRepossessed(the_bond.clone().lender, the_bond.clone().borrower, the_bond.clone().collateral));
            Ok(())
        }

        pub fn sell_bond(origin, bond_id: H256, price: T::Balance ) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<Bonds<T>>::exists(bond_id.clone()), "bond does not exist");
            let mut the_bond = Self::bond(bond_id.clone());
            ensure!(the_bond.clone().sender_is_lender(sender.clone()), "You are not the lender for this bond");
            the_bond.request_transfer(price.clone());
            <Bonds<T>>::mutate(bond_id, |b| {*b = the_bond});
            Self::deposit_event(RawEvent::BondTransferRequested(bond_id.clone(), sender.clone(), price));
            Ok(())
        }

        pub fn buy_bond(origin, bond_id: H256) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<Bonds<T>>::exists(bond_id.clone()), "bond does not exist");
            let mut the_bond = Self::bond(bond_id.clone());
            ensure!(the_bond.clone().for_sale.is_some(), "The bond is not open for sale");
            <balances::Module<T> as Currency<_>>::transfer(&sender, &the_bond.clone().lender, the_bond.clone().for_sale.unwrap()).expect("Transfer the current debt amount from buyer to lender");
            the_bond.approve_transfer(sender.clone());
            <Bonds<T>>::mutate(bond_id, |b| {*b = the_bond});
            Self::deposit_event(RawEvent::BondTransferApproved(bond_id.clone(), sender.clone()));
            Ok(())
        }

    }
}

decl_event!(
    pub enum Event<T>
    where
        Time = <T as system::Trait>::BlockNumber,
        Price = <T as balances::Trait>::Balance,
        Lender = <T as system::Trait>::AccountId,
        Borrower = <T as system::Trait>::AccountId,
        Redeemed = <T as balances::Trait>::Balance,
        Before = <T as balances::Trait>::Balance,
        After = <T as balances::Trait>::Balance,
    {
        LoanRequested(Lender, Borrower, H256),
        LoanApproved(Lender, Borrower, Time),
        LoanRedeemed(Lender, Borrower, Redeemed),
        LoanRepossessed(Lender, Borrower, Price),
        LoanAmountIncreased(H256, Before, After, Time),
        BondTransferRequested(H256, Lender, Price),
        BondTransferApproved(H256, Lender),
    }
);
