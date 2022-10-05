// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
//
// This file is part of Zims.
//
// Copyright (C) 2019-2022 Setheum Labs.
// SPDX-License-Identifier: BUSL-1.1 (Business Source License 1.1)

//! Autogenerated weights for serp_setmint
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-26, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/setheum-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=serp_setmint
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./chassis/lib-serml/setmint/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for serp_setmint.
pub trait WeightInfo {
	fn authorize() -> Weight;
	fn unauthorize() -> Weight;
	fn unauthorize_all(c: u32, ) -> Weight;
	fn adjust_loan() -> Weight;
	fn transfer_loan_from() -> Weight;
	fn close_loan_has_debit_by_dex() -> Weight;
	fn expand_position_collateral() -> Weight;
	fn shrink_position_debit() -> Weight;
}

/// Weights for serp_setmint using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
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
		(245_614_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn transfer_loan_from() -> Weight {
		(166_925_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn close_loan_has_debit_by_dex() -> Weight {
		(369_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(32 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	fn expand_position_collateral() -> Weight {
		(183_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(20 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn shrink_position_debit() -> Weight {
		(186_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(16 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
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
		(245_614_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(26 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn transfer_loan_from() -> Weight {
		(166_925_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(21 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn close_loan_has_debit_by_dex() -> Weight {
		(369_989_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(32 as Weight))
			.saturating_add(RocksDbWeight::get().writes(14 as Weight))
	}
	fn expand_position_collateral() -> Weight {
		(183_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(20 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn shrink_position_debit() -> Weight {
		(186_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(16 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
}