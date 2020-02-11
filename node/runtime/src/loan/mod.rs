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

use support::{decl_module, decl_storage, decl_event, dispatch::Result, ensure, traits::Randomness, traits::{LockableCurrency, LockIdentifier, WithdrawReason, WithdrawReasons,
	Currency}};
use primitives::H256;
use sr_primitives::weights::SimpleDispatchInfo;
use crate::RandomnessCollectiveFlip;
use codec::{Encode, Decode};
use system::{ensure_signed};
use support::dispatch::Parameter;
use rstd::prelude::*;
 
const COLLATERAL_ID: LockIdentifier = *b"loan    ";


#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Bond<Balance: Parameter, AccountId: Parameter, BlockNumber: Parameter> {
    lender: AccountId,
    borrower: AccountId,
    amount: Balance,
    expires_at: Option<BlockNumber>,
    interest: Balance,
    collateral: Balance,
    period: BlockNumber,
    for_sale: Option<Balance>,
    next_increment: BlockNumber
}

impl<Balance: Parameter, AccountId: Parameter, BlockNumber: Parameter> Bond<Balance, AccountId, BlockNumber> {
    pub fn new (lender: AccountId, borrower: AccountId, amount: Balance, expires_at: Option<BlockNumber>, interest: Balance, collateral: Balance, period: BlockNumber, next_increment: BlockNumber) -> Self {
        if expires_at.is_some() {
            Bond {
                lender,
                borrower,
                amount,
                expires_at,
                interest,
                collateral,
                period,
                for_sale: None,
                next_increment
            }
        } else {
            Bond {
                lender,
                borrower,
                amount,
                expires_at: None,
                interest,
                collateral,
                period,
                for_sale: None,
                next_increment
            }
        }
        
    }

    pub fn approve(&mut self, lender: AccountId) {
        self.lender = lender;
    }

    pub fn sender_is_lender(&self, lender: AccountId) -> bool {
        self.lender == lender && self.borrower != lender   
    }

    pub fn sender_is_borrower(&self, borrower: AccountId) -> bool {
        self.borrower == borrower && self.lender != borrower
    }

    pub fn request_transfer(&mut self, price: Balance) {
        self.for_sale = Some(price);
    }
    pub fn approve_transfer(&mut self, approver: AccountId) {
        self.lender = approver;
        self.for_sale = None;
    }
}

// Module's function and Methods of custom struct to be placed here
impl<T: Trait> Module<T> {

    pub fn activate_bond(mut bond: Bond<T::Balance, T::AccountId, T::BlockNumber>, current: T::BlockNumber) -> Bond<T::Balance, T::AccountId, T::BlockNumber> {
        // (1 + interest/ 1000) * <bond amount>
        bond.amount += (bond.amount * bond.interest) / Self::to_balance(1, "kilo");
        bond.next_increment = current + bond.period;
        bond
    }

    pub fn process_bond(bond_id: H256, current_block: T::BlockNumber) -> Result {
        let mut updated_bond = Self::bond(bond_id.clone());
        let before = updated_bond.amount;
        updated_bond = Self::activate_bond(updated_bond.clone(), current_block);
        <Bonds<T>>::mutate(bond_id.clone(), |l| {*l = updated_bond.clone()});
        let next = updated_bond.clone().next_increment;
        if <LoanCallBacks<T>>::exists(next.clone()) {
            <LoanCallBacks<T>>::mutate(next.clone(), |c| {c.push(bond_id);});
            Self::deposit_event(RawEvent::LoanAmountIncreased(bond_id.clone(), before, updated_bond.clone().amount, next.clone()));
        } else {
            <LoanCallBacks<T>>::insert(next.clone(), vec!{bond_id});
            Self::deposit_event(RawEvent::LoanAmountIncreased(bond_id.clone(), before, updated_bond.clone().amount, next.clone()));    
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
		let result = match digit  {
			"femto" => T::Balance::from(u),
			"nano" =>  power(u, 3),
			"micro" => power(u, 6),
			"milli" => power(u, 9),
			"one" => power(u,12),
			"kilo" => power(u, 15),
			"mega" => power(u, 18),
			"giga" => power(u, 21),
			"tera" => power(u, 24),
			"peta" => power(u, 27),
			"exa" => power(u, 30),
			"zetta" => power(u, 33),
			"yotta" => power(u, 36),
			_ => T::Balance::from(u)
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
        pub Bonds get(bond): map H256 => Bond<T::Balance, T::AccountId, T::BlockNumber>;
        pub LoanCallBacks get(callback): map T::BlockNumber => Vec<H256>;
    }
}

decl_module! {
	
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		fn deposit_event() = default;
        
        fn on_finalize(block_number: T::BlockNumber) {
			let bonds = Self::callback(block_number);
			for i in bonds {
				if let Err(e) = Self::process_bond(i, block_number) {
					sr_primitives::print(e);
				}
			}
		}

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn request(origin, lender: T::AccountId, amount: T::Balance, expires_at: T::BlockNumber, interest: T::Balance, collateral: T::Balance, period: T::BlockNumber) -> Result {
            let borrower = ensure_signed(origin)?;
            ensure!(borrower != lender, "You cannot just borrow money from oneself");
            let new_bond = Bond::new(lender, borrower, amount, Some(expires_at.clone()), interest.clone(), collateral, period, T::BlockNumber::from(0));
            let bond_hash = H256::from_slice(&RandomnessCollectiveFlip::random_seed().encode() as &[u8]);
            ensure!(!<Bonds<T>>::exists(bond_hash.clone()), "Hash collision!");
            <Bonds<T>>::insert(bond_hash, new_bond.clone());
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
	pub enum Event<T> where Time = <T as system::Trait>::BlockNumber, Price = <T as balances::Trait>::Balance, Lender  = <T as system::Trait>::AccountId, Borrower = <T as system::Trait>::AccountId, Redeemed = <T as balances::Trait>::Balance, Before = <T as balances::Trait>::Balance, After = <T as balances::Trait>::Balance {
        LoanRequested(Lender, Borrower, H256),
        LoanApproved(Lender, Borrower, Time),
        LoanRedeemed(Lender, Borrower, Redeemed),
        LoanRepossessed(Lender, Borrower, Price),
        LoanAmountIncreased(H256, Before, After, Time),
        BondTransferRequested(H256, Lender, Price),
        BondTransferApproved(H256, Lender),
	}
);
