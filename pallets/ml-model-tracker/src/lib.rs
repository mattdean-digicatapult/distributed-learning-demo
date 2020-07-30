#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use sp_std::prelude::*;
use frame_support::{ensure, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const MODEL_LENGTH: usize = 2;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Model<AccountId, ParentBlock> {
	owner: AccountId,
	parent_block: ParentBlock,
	model: Vec<u32>,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as MlModelTracker {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		ModelsByOwner get(fn models_by_owner): map hasher(blake2_128_concat) T::AccountId => Model<T::AccountId, T::Hash>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		UpdatedModel(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		// Error returned when making remote http fetching
		HttpFetchingError,
		// Invalid model size
		InvalidModel,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		// type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn update_model(origin, parent_block: T::Hash, model: Vec<u32>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			ensure!(model.len() == MODEL_LENGTH, Error::<T>::InvalidModel);

			// Update storage.
			<ModelsByOwner<T>>::insert(sender.clone(), Model {
				owner: sender.clone(),
				parent_block: parent_block.clone(),
				model: model.clone()
			});

			// Emit an event.
			Self::deposit_event(RawEvent::UpdatedModel(sender));
			// Return a successful DispatchResult
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn aggregate_model() -> Result<Vec<u32>, Error<T>> {
		let mut result: Vec<u64> = vec![0; MODEL_LENGTH];
		let mut count: u64 = 0;

		for model in <ModelsByOwner<T>>::iter_values() {
			count += 1;
			for i in 0..MODEL_LENGTH {
				result[i] = result[i] + (model.model[i] as u64);
    	}
		}

		if count == 0 {
			Ok(vec![0u32; MODEL_LENGTH])
		} else {
			let result = result.iter().map(| v | ((v / count) as u32)).collect::<Vec<u32>>();
			Ok(result)
		}
	}
}
