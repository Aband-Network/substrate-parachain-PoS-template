
//! Autogenerated weights for pallet_collators
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `wjy-PC`, CPU: `AMD Ryzen 9 7950X 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/aband
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_collators
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/pallet-weight-template.hbs
// --output
// ./pallets/collators/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collators.
pub trait WeightInfo {
	fn close_pos() -> Weight;
	fn open_pos() -> Weight;
	fn set_collators() -> Weight;
	fn add_collator() -> Weight;
	fn remove_collator() -> Weight;
	fn set_nimbus_id() -> Weight;
}

/// Weights for pallet_collators using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Collators IsClosedPoS (r:0 w:1)
	fn close_pos() -> Weight {
		// Minimum execution time: 13_054 nanoseconds.
		Weight::from_ref_time(14_037_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Collators IsClosedPoS (r:0 w:1)
	fn open_pos() -> Weight {
		// Minimum execution time: 13_004 nanoseconds.
		Weight::from_ref_time(13_555_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:0 w:4)
	// Storage: Collators Collators (r:0 w:1)
	fn set_collators() -> Weight {
		// Minimum execution time: 37_720 nanoseconds.
		Weight::from_ref_time(41_037_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:1 w:1)
	// Storage: Collators Collators (r:1 w:1)
	fn add_collator() -> Weight {
		// Minimum execution time: 22_753 nanoseconds.
		Weight::from_ref_time(31_970_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Collators (r:1 w:1)
	fn remove_collator() -> Weight {
		// Minimum execution time: 23_474 nanoseconds.
		Weight::from_ref_time(25_277_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:1 w:1)
	// Storage: Collators Collators (r:1 w:0)
	fn set_nimbus_id() -> Weight {
		// Minimum execution time: 38_291 nanoseconds.
		Weight::from_ref_time(45_414_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Collators IsClosedPoS (r:0 w:1)
	fn close_pos() -> Weight {
		// Minimum execution time: 13_054 nanoseconds.
		Weight::from_ref_time(14_037_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Collators IsClosedPoS (r:0 w:1)
	fn open_pos() -> Weight {
		// Minimum execution time: 13_004 nanoseconds.
		Weight::from_ref_time(13_555_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:0 w:4)
	// Storage: Collators Collators (r:0 w:1)
	fn set_collators() -> Weight {
		// Minimum execution time: 37_720 nanoseconds.
		Weight::from_ref_time(41_037_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:1 w:1)
	// Storage: Collators Collators (r:1 w:1)
	fn add_collator() -> Weight {
		// Minimum execution time: 22_753 nanoseconds.
		Weight::from_ref_time(31_970_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Collators (r:1 w:1)
	fn remove_collator() -> Weight {
		// Minimum execution time: 23_474 nanoseconds.
		Weight::from_ref_time(25_277_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Collators IsClosedPoS (r:1 w:0)
	// Storage: Collators Mapping (r:1 w:1)
	// Storage: Collators Collators (r:1 w:0)
	fn set_nimbus_id() -> Weight {
		// Minimum execution time: 38_291 nanoseconds.
		Weight::from_ref_time(45_414_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}