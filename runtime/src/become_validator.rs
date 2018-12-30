//// NOTE - note used yet, but will have to use some
////// Assert macros used in tests.
////extern crate sr_std;
////
////// Needed for tests (`with_externalities`).
////#[cfg(test)]
////extern crate sr_io;
////
////// Needed for the set of mock primitives used in our tests.
////#[cfg(test)]
////extern crate substrate_primitives;
////
////// Needed for various traits. In our case, `OnFinalise`.
////extern crate sr_primitives;
////
////// Needed for deriving `Encode` and `Decode` for `RawEvent`.
////#[macro_use]
////extern crate parity_codec_derive;
////extern crate parity_codec as codec;
////
////// Needed for type-safe access to storage DB.
////#[macro_use]
////extern crate srml_support as support;
////// `system` module provides us with all sorts of useful stuff and macros
////// depend on it being around.
////extern crate srml_system as system;
////// `balances` module is needed for our little example. It's not required in
////// general (though if you want your module to be able to work with tokens, then you
////// might find it useful).
////extern crate srml_balances as balances;
//
//
//// modules configuration Trait. All types and const go in here
//// if module is dependant on specific other modules, then their configuration traits should be added to our implied traits list
//



use srml_support::{StorageMap, dispatch::Result};
use system::ensure_signed;
use rstd::prelude::*;

pub trait Trait: system::Trait {}

// macro takes care of marshalling arrguments and dispatch for entry points
// extrinsics interact with these
// public calls, root calls, and inherent calls
	// must match agaisnt them with ::Root or ::Signed(AccountID)
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn become_validator(origin) -> Result {
			let sender = ensure_signed(origin)?;
			<ValidatorList<T>>::insert(sender.clone(), true);
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as ValidatorStorage {
		ValidatorList: map T::AccountId => bool;
	}

	// TODO - use the macro to easily make put, get kill
	// seems like storage would be this (from example)
		// `map KeyType => ValueType` (a map item).
}

//decl_event!(
//	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
//		OwnershipTransferred(AccountId, AccountId),
//	}
//);

//TODO - impl block (see right side 


// TODO  - test

// test one, should test become_validator, which depends on the validator calling with a specific block number, which shows the ethereym block where they deposited Graph Tokens. It would then check and pass. So provide it a real ethereum event. that event should look like so:

/*
 // From POS Spec v1

 Event NewValidator(
     indexChainID,
     validatorID,
     mainnetAddress,
     tokensStaked
 )

 which will then make a storage of multiple validators

 // figure out how to store this, based on other modules. likely it will be some sort of mapping

 // maybe some getter functions for each of these

*/
