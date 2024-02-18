// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-26, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/setheum
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_nft
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./blockchain/modules/nft/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_nft.
pub trait WeightInfo {
	fn create_class() -> Weight;
	fn mint(i: u32, ) -> Weight;
	fn transfer() -> Weight;
	fn burn() -> Weight;
	fn burn_with_remark(b: u32, ) -> Weight;
	fn destroy_class() -> Weight;
	fn update_class_properties() -> Weight;
}

/// Weights for module_nft using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
	fn create_class() -> Weight {
		Weight::from_parts(177_661_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn mint(i: u32, ) -> Weight {
		Weight::from_parts(44_387_000, 0)
			// Standard Error: 46_000
			.saturating_add(Weight::from_parts(72_699_000, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(i as u64)))
	}
	fn transfer() -> Weight {
		Weight::from_parts(266_936_000, 0)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	fn burn() -> Weight {
		Weight::from_parts(189_094_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn burn_with_remark(b: u32, ) -> Weight {
		Weight::from_parts(196_036_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn destroy_class() -> Weight {
		Weight::from_parts(217_091_000, 0)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn update_class_properties() -> Weight {
		Weight::from_parts(52_914_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_class() -> Weight {
		Weight::from_parts(177_661_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn mint(i: u32, ) -> Weight {
		Weight::from_parts(44_387_000, 0)
			// Standard Error: 46_000
			.saturating_add(Weight::from_parts(72_699_000, 0).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(i as u64)))
	}
	fn transfer() -> Weight {
		Weight::from_parts(266_936_000, 0)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn burn() -> Weight {
		Weight::from_parts(189_094_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn burn_with_remark(b: u32, ) -> Weight {
		Weight::from_parts(196_036_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn destroy_class() -> Weight {
		Weight::from_parts(217_091_000, 0)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn update_class_properties() -> Weight {
		Weight::from_parts(52_914_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
