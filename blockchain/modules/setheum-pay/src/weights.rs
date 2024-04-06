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

//! Autogenerated weights for module_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-19, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/setheum-node
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution
// compiled
// --extrinsic=*
// --pallet=module_payment
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output
// ./pallets/payment/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_payment.
pub trait WeightInfo {
	fn pay(x: u32, ) -> Weight;
	fn release() -> Weight;
	fn cancel() -> Weight;
	fn resolve_payment() -> Weight;
	fn request_refund() -> Weight;
	fn dispute_refund() -> Weight;
	fn request_payment() -> Weight;
	fn accept_and_pay() -> Weight;
	fn remove_task() -> Weight;
}

/// Weights for module_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		Weight::from_parts(55_900_000, 0)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		Weight::from_parts(48_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		Weight::from_parts(21_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		Weight::from_parts(17_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		Weight::from_parts(58_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		Weight::from_parts(55_900_000, 0)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		Weight::from_parts(48_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		Weight::from_parts(21_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		Weight::from_parts(17_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		Weight::from_parts(58_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		Weight::from_parts(4_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}