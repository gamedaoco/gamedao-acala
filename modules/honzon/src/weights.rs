
//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-26, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_honzon
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/honzon/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_honzon.
pub trait WeightInfo {
	fn authorize() -> Weight;
	fn unauthorize() -> Weight;
	fn unauthorize_all(c: u32, ) -> Weight;
	fn adjust_loan() -> Weight;
	fn transfer_loan_from() -> Weight;
}

/// Weights for module_honzon using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn authorize() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize_all(c: u32, ) -> Weight {
		(13_875_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((1_018_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	fn adjust_loan() -> Weight {
		(160_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(24 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn transfer_loan_from() -> Weight {
		(114_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn authorize() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn unauthorize() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn unauthorize_all(c: u32, ) -> Weight {
		(13_875_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((1_018_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	fn adjust_loan() -> Weight {
		(160_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(24 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn transfer_loan_from() -> Weight {
		(114_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(21 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
}