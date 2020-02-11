/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, dispatch::Result, ensure};
use primitives::H256;
use sr_primitives::weights::SimpleDispatchInfo;
use codec::{Encode, Decode};
use system::{ensure_signed};
use rstd::prelude::*;

/*
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub enum Status {
    /// Allowing access
    Yes,
    /// Blocked
    Blocked,
    /// Temporary
    Temporary(u64)
}

impl Default for Status {
    fn default() -> Self { Status::Yes }
}
*/

//pub type Id = Vec<u8>;

/// Data storage type for each account
#[derive(Encode, Decode, Default, Clone, Eq, PartialEq)]
pub struct DID {
    /// public key to authenticate
    pub public_key: Vec<u8>,
    /// issuer did identifier
    pub issuer_id: Vec<u8>,
    /// hash from issuer tx public key
    pub issuer: H256,
    /// hash from owner tx public key
    pub owner: H256 
}

impl DID {
    pub fn new (public_key: Vec<u8>, issuer_id: Option<Vec<u8>>, issuer: H256, owner: H256) -> Self {
        if issuer_id.is_some() {
            DID {
                public_key,
                issuer_id: issuer_id.unwrap(),
                issuer,
                owner
            }
        } else {
            DID {
                public_key,
                issuer_id: vec!{1},
                issuer,
                owner
            }
        }
    }
}

// Module's function and Methods of custom struct to be placed here
impl<T: Trait> Module<T> {
    pub fn is_id_issuer(id:Vec<u8>, issuer: T::AccountId) -> bool {
        let access_who = Self::id(id.clone());
        let issuer_hash = H256::from_slice(&issuer.encode());
        access_who.issuer == issuer_hash    
    }

    pub fn is_id_owner(id: Vec<u8>, owner: T::AccountId) -> bool {
        let owner_who = Self::id(id.clone());
        let owner_hash = H256::from_slice(&owner.encode());
        owner_who.owner == owner_hash    
    }
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as DID {
        pub IDs get(id): map Vec<u8> => DID;
    }
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn register(origin, id: DIDIdentifier, issuer_id: DIDIdentifier, public_key: PublicKey, owner: T::AccountId) -> Result {
            let issuer = ensure_signed(origin)?;
            let issuer_hash = H256::from_slice(&issuer.encode());
            let owner_hash = H256::from_slice(&owner.encode());
            ensure!(!<IDs>::exists(id.clone()), "The id is already issued");
            if issuer_id == [0] {
                let did_claimer = DID::new(public_key.clone(), None, issuer_hash.clone(), owner_hash.clone());
                <IDs>::insert(id.clone(), did_claimer);
                Self::deposit_event(RawEvent::IdIssuerRegistered(id, owner));
            } else {
                ensure!(Self::is_id_issuer(issuer_id.clone(), issuer.clone()), "You are not the owner of this issuer_did");
                let did_claimer = DID::new(public_key.clone(), Some(issuer_id.clone()), issuer_hash.clone(), owner_hash.clone());
                <IDs>::insert(id.clone(), did_claimer);
                Self::deposit_event(RawEvent::IdIssued(id, owner.clone(), issuer_id));
            }
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn remove(origin, id: Vec<u8>) -> Result {
            let issuer = ensure_signed(origin)?;
            ensure!(<IDs>::exists(id.clone()), "The id does not exist");
            let did_claimer = Self::id(id.clone());
            let issuer_hash = H256::from_slice(&issuer.encode());
            ensure!(did_claimer.issuer == issuer_hash, "You are not the issuer of this identity");
            <IDs>::remove(id.clone());
            Self::deposit_event(RawEvent::IdRemoved(id, issuer));
            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(0)]
        pub fn update(origin, id: DIDIdentifier, public_key: PublicKey) -> Result {
            let issuer = ensure_signed(origin)?;
            ensure!(<IDs>::exists(id.clone()), "DID is not registered");
            let did_claimer = Self::id(id.clone());
            let issuer_hash = H256::from_slice(&issuer.encode());
            ensure!(did_claimer.issuer == issuer_hash, "You are not the issuer of this identity");
            
            // Update DID 
            let did_claimer = DID::new(public_key.clone(), Some(did_claimer.clone().issuer_id), issuer_hash.clone(), did_claimer.clone().owner);
            <IDs>::mutate(id.clone(), |a| *a = did_claimer);
            Self::deposit_event(RawEvent::IdChanged(id, public_key, issuer));
            Ok(())
        }
	}
}

pub type DIDIdentifier = Vec<u8>;
pub type PublicKey = Vec<u8>;
pub type IssuedBy = Vec<u8>;

decl_event!(
	pub enum Event<T> where SovereignAccount  = <T as system::Trait>::AccountId {

        IdIssuerRegistered(DIDIdentifier, SovereignAccount),
        IdIssued(DIDIdentifier, SovereignAccount, IssuedBy ),
        IdRemoved(DIDIdentifier, SovereignAccount ),
        IdChanged(DIDIdentifier, PublicKey, SovereignAccount ),
	}
);
