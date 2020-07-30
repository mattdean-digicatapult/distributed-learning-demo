#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait MlModelTrackerApi {
		fn get_aggregate_model() -> Vec<u32>;
	}
}
