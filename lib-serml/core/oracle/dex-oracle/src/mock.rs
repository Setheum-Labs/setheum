// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-2021 Setheum Labs.
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

//! Mocks for the dex-oracle module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types, traits::Everything};
use frame_system::EnsureSignedBy;
use primitives::{DexShare, Moment, TokenSymbol};
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{IdentityLookup, Zero},
	DispatchError,
};
use sp_std::cell::RefCell;
use support::SwapLimit;

pub type AccountId = u128;
pub type BlockNumber = u64;

pub const SETM: CurrencyId = CurrencyId::Token(TokenSymbol::SETM);
pub const USDI: CurrencyId = CurrencyId::Token(TokenSymbol::USDI);
pub const DNAR: CurrencyId = CurrencyId::Token(TokenSymbol::DNAR);
pub const LP_USDI_DNAR: CurrencyId =
	CurrencyId::DexShare(DexShare::Token(TokenSymbol::USDI), DexShare::Token(TokenSymbol::DNAR));

mod dex_oracle {
	pub use super::super::*;
}

parameter_types! {
	pub static USDIDNARPair: TradingPair = TradingPair::from_currency_ids(USDI, DNAR).unwrap();
	pub static SETMDNARPair: TradingPair = TradingPair::from_currency_ids(SETM, DNAR).unwrap();
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = 1000;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

thread_local! {
	static USDI_DNAR_POOL: RefCell<(Balance, Balance)> = RefCell::new((Zero::zero(), Zero::zero()));
	static SETM_DNAR_POOL: RefCell<(Balance, Balance)> = RefCell::new((Zero::zero(), Zero::zero()));
}

pub fn set_pool(trading_pair: &TradingPair, pool_0: Balance, pool_1: Balance) {
	if *trading_pair == USDIDNARPair::get() {
		USDI_DNAR_POOL.with(|v| *v.borrow_mut() = (pool_0, pool_1));
	} else if *trading_pair == SETMDNARPair::get() {
		SETM_DNAR_POOL.with(|v| *v.borrow_mut() = (pool_0, pool_1));
	}
}

pub struct MockDEX;
impl DEXManager<AccountId, CurrencyId, Balance> for MockDEX {
	fn get_liquidity_pool(currency_id_0: CurrencyId, currency_id_1: CurrencyId) -> (Balance, Balance) {
		TradingPair::from_currency_ids(currency_id_0, currency_id_1)
			.map(|trading_pair| {
				if trading_pair == USDIDNARPair::get() {
					USDI_DNAR_POOL.with(|v| *v.borrow())
				} else if trading_pair == SETMDNARPair::get() {
					SETM_DNAR_POOL.with(|v| *v.borrow())
				} else {
					(0, 0)
				}
			})
			.unwrap_or_else(|| (0, 0))
	}

	fn get_liquidity_token_address(_currency_id_a: CurrencyId, _currency_id_b: CurrencyId) -> Option<H160> {
		unimplemented!()
	}

	fn get_swap_amount(_: &[CurrencyId], _: SwapLimit<Balance>) -> Option<(Balance, Balance)> {
		unimplemented!()
	}

	fn get_best_price_swap_path(
		_: CurrencyId,
		_: CurrencyId,
		_: SwapLimit<Balance>,
		_: Vec<Vec<CurrencyId>>,
	) -> Option<Vec<CurrencyId>> {
		unimplemented!()
	}

	fn swap_with_specific_path(
		_: &AccountId,
		_: &[CurrencyId],
		_: SwapLimit<Balance>,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn buyback_swap_with_specific_path(
		_: &AccountId,
		_: &[CurrencyId],
		_: SwapLimit<Balance>,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn swap_with_exact_target(
		_who: &AccountId,
		_path: &[CurrencyId],
		_exact_target_amount: Balance,
		_max_supply_amount: Balance,
	) -> DispatchResult {
		unimplemented!()
	}

	fn add_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_max_amount_a: Balance,
		_max_amount_b: Balance,
		_min_share_increment: Balance,
	) -> sp_std::result::Result<(Balance, Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn remove_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_remove_share: Balance,
		_min_withdrawn_a: Balance,
		_min_withdrawn_b: Balance,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

impl Config for Runtime {
	type DEX = MockDEX;
	type Time = Timestamp;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		DexOracle: dex_oracle::{Pallet, Call, Storage},
	}
);

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		t.into()
	}
}
