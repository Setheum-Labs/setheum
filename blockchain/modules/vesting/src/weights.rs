//! Autogenerated weights for orlm_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-04, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// /target/release/setheum-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./vesting/src/weights.rs
// --template
// ..maintain/orml-weight-template.hbs



#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for orml_vesting.
pub trait WeightInfo {
	fn vested_transfer() -> Weight;
	fn claim(i: u32, ) -> Weight;
	fn update_vesting_schedules(i: u32, ) -> Weight;
}

/// Default weights.
impl WeightInfo for () {
	fn vested_transfer() -> Weight {
		Weight::from_parts(69_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn claim(i: u32, ) -> Weight {
		Weight::from_parts(31_747_000, 0)
			// Standard Error: 4_000
			.saturating_add(Weight::from_parts(63_000, 0).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn update_vesting_schedules(i: u32, ) -> Weight {
		Weight::from_parts(29_457_000, 0)
			// Standard Error: 4_000
			.saturating_add(Weight::from_parts(117_000, 0).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}
