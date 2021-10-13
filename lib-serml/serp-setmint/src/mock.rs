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

//! Mocks for the serp-setmint module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types};
use frame_system::{offchain::SendTransactionTypes, EnsureSignedBy};
use orml_traits::parameter_type_with_key;
use primitives::{Balance, TokenSymbol};
use sp_core::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{AccountIdConversion, IdentityLookup},
	FixedPointNumber, ModuleId,
};
use sp_std::cell::RefCell;
use support::{AuctionManager, ExchangeRate, SerpTreasury, Price, PriceProvider, Rate, Ratio};

mod serp_setmint {
	pub use super::super::*;
}

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type AuctionId = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;
pub const SETM: CurrencyId = CurrencyId::Token(TokenSymbol::SETM);
pub const SETUSD: CurrencyId = CurrencyId::Token(TokenSymbol::SETUSD);
pub const SETR: CurrencyId = CurrencyId::Token(TokenSymbol::SETR);
pub const BTC: CurrencyId = CurrencyId::Token(TokenSymbol::RENBTC);
pub const DNAR: CurrencyId = CurrencyId::Token(TokenSymbol::DNAR);

parameter_types! {
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
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Runtime>;
	type MaxLocks = ();
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SETM;
}

impl orml_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const LoansModuleId: ModuleId = ModuleId(*b"set/loan");
}

impl loans::Config for Runtime {
	type Event = Event;
	type Convert = cdp_engine::DebitExchangeRateConvertor<Runtime>;
	type Currency = Tokens;
	type RiskManager = CDPEngineModule;
	type CDPTreasury = CDPTreasuryModule;
	type ModuleId = LoansModuleId;
	type OnUpdateLoan = ();
}

pub struct MockPriceSource;
impl PriceProvider<CurrencyId> for MockPriceSource {
	fn get_relative_price(_base: CurrencyId, _quote: CurrencyId) -> Option<Price> {
		Some(Price::one())
	}

	fn get_price(_currency_id: CurrencyId) -> Option<Price> {
		Some(Price::one())
	}

	fn lock_price(_currency_id: CurrencyId) {}

	fn unlock_price(_currency_id: CurrencyId) {}
}

pub struct MockAuctionManager;
impl AuctionManager<AccountId> for MockAuctionManager {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type AuctionId = AuctionId;

	fn new_collateral_auction(
		_refund_recipient: &AccountId,
		_currency_id: Self::CurrencyId,
		_amount: Self::Balance,
		_target: Self::Balance,
	) -> DispatchResult {
		Ok(())
	}

	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		Ok(())
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_collateral_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		Default::default()
	}
}

pub struct MockSerpTreasury;
impl SerpTreasury<AccountId> for MockSerpTreasury {
	type Balance = Balance;
	type CurrencyId = CurrencyId;

	/// Deliver System StableCurrency Inflation
	fn issue_stablecurrency_inflation() -> DispatchResult {
		unimplemented!()
	}

	/// SerpUp ratio for BuyBack Swaps to burn Dinar
	fn get_buyback_serpup(
		_amount: Balance,
		_currency_id: CurrencyId,
	) -> DispatchResult {
		unimplemented!()
	}

	/// SerpUp ratio for Setheum Foundation's Charity Fund
	fn get_charity_fund_serpup(
		_amount: Balance,
		_currency_id: CurrencyId
	) -> DispatchResult {
		unimplemented!()
	}
	
	/// SerpUp ratio for SetPay Cashdrops
	fn get_cashdrop_serpup(
		_amount: Balance,
		_currency_id: CurrencyId
	) -> DispatchResult {
		unimplemented!()
	}

	/// SerpUp ratio for BuyBack Swaps to burn Dinar
	fn get_buyback_serplus(
		_amount: Balance,
		_currency_id: CurrencyId,
	) -> DispatchResult {
		unimplemented!()
	}

	/// SerpUp ratio for Setheum Foundation's Charity Fund
	fn get_charity_fund_serplus(
		_amount: Balance,
		_currency_id: CurrencyId
	) -> DispatchResult {
		unimplemented!()
	}
	
	/// SerpUp ratio for SetPay Cashdrops
	fn get_cashdrop_serplus(
		_amount: Balance,
		_currency_id: CurrencyId
	) -> DispatchResult {
		unimplemented!()
	}

	/// issue serpup surplus(stable currencies) to their destinations according to the serpup_ratio.
	fn on_serplus(
		_currency_id: CurrencyId,
		_amount: Balance,
	) -> DispatchResult {
		unimplemented!()
	}

	/// issue serpup surplus(stable currencies) to their destinations according to the serpup_ratio.
	fn on_serpup(
		_currency_id: CurrencyId,
		_amount: Balance,
	) -> DispatchResult {
		unimplemented!()
	}

	/// buy back and burn surplus(stable currencies) with swap by DEX.
	fn on_serpdown(
		_currency_id: CurrencyId,
		_amount: Balance,
	) -> DispatchResult {
		unimplemented!()
	}

	/// get the minimum supply of a setcurrency - by key
	fn get_minimum_supply(
		_currency_id: CurrencyId
	) -> Balance {
		unimplemented!()
	}

	/// issue standard to `who`
	fn issue_standard(
		_currency_id: CurrencyId,
		_who: &AccountId,
		_standard: Balance
	) -> DispatchResult {
		unimplemented!()
	}

	/// burn standard(stable currency) of `who`
	fn burn_standard(
		_currency_id: CurrencyId,
		_who: &AccountId,
		_standard: Balance
	) -> DispatchResult {
		unimplemented!()
	}

	/// issue setter of amount setter to `who`
	fn issue_setter(
		_who: &AccountId,
		_setter: Balance
	) -> DispatchResult {
		unimplemented!()
	}

	/// burn setter of `who`
	fn burn_setter(
		_who: &AccountId,
		_setter: Balance
	) -> DispatchResult {
		unimplemented!()
	}

	/// deposit reserve asset (Setter (SETR)) to serp treasury by `who`
	fn deposit_setter(
		_from: &AccountId,
		_amount: Balance
	) -> DispatchResult {
		unimplemented!()
	}

	/// claim cashdrop of `currency_id` relative to `transfer_amount` for `who`
	fn claim_cashdrop(
		_currency_id: CurrencyId,
		_who: &AccountId,
		_transfer_amount: Balance
	) -> DispatchResult {
		unimplemented!()
	}
}

thread_local! {
	static IS_SHUTDOWN: RefCell<bool> = RefCell::new(false);
}

pub fn mock_shutdown() {
	IS_SHUTDOWN.with(|v| *v.borrow_mut() = true)
}

pub struct MockEmergencyShutdown;
impl EmergencyShutdown for MockEmergencyShutdown {
	fn is_shutdown() -> bool {
		IS_SHUTDOWN.with(|v| *v.borrow_mut())
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub const GetSetUSDCurrencyId: CurrencyId = SETUSD;
	pub const MaxAuctionsCount: u32 = 10_000;
	pub const CDPTreasuryModuleId: ModuleId = ModuleId(*b"set/cdpt");
	pub TreasuryAccount: AccountId = ModuleId(*b"set/smtr").into_account();
}

impl cdp_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetSetUSDCurrencyId = GetSetUSDCurrencyId;
	type AuctionManagerHandler = MockAuctionManager;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type DEX = ();
	type MaxAuctionsCount = MaxAuctionsCount;
	type ModuleId = CDPTreasuryModuleId;
	type SerpTreasury = MockSerpTreasury;
	type WeightInfo = ();
}

parameter_types! {
	pub CollateralCurrencyIds: Vec<CurrencyId> = vec![BTC, DNAR];
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(3, 2);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::one();
	pub DefaultLiquidationPenalty: Rate = Rate::saturating_from_rational(10, 100);
	pub const MinimumDebitValue: Balance = 2;
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl cdp_engine::Config for Runtime {
	type Event = Event;
	type PriceSource = MockPriceSource;
	type CollateralCurrencyIds = CollateralCurrencyIds;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
	type MinimumDebitValue = MinimumDebitValue;
    type GetSetUSDCurrencyId = GetSetUSDCurrencyId;
	type CDPTreasury = CDPTreasuryModule;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type UnsignedPriority = UnsignedPriority;
	type EmergencyShutdown = MockEmergencyShutdown;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

parameter_types! {
	pub StableCurrencyIds: Vec<CurrencyId> = vec![
		SETR,
		SETUSD,
	];
	pub const DepositPerAuthorization: Balance = 100;
}

impl Config for Runtime {
	type Event = Event;
	type Currency = PalletBalances;
	type DepositPerAuthorization = DepositPerAuthorization;
	type WeightInfo = ();
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		SerpMintModule: serp_setmint::{Module, Storage, Call, Event<T>},
		Tokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},
		PalletBalances: pallet_balances::{Module, Call, Storage, Event<T>},
		Currencies: orml_currencies::{Module, Call, Event<T>},
		LoansModule: loans::{Module, Storage, Call, Event<T>},
		CDPTreasuryModule: cdp_treasury::{Module, Storage, Call, Event<T>},
		CDPEngineModule: cdp_engine::{Module, Storage, Call, Event<T>, Config, ValidateUnsigned},
	}
);

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> SendTransactionTypes<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

pub struct ExtBuilder {
	endowed_native: Vec<(AccountId, Balance)>,
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_native: vec![(ALICE, 1000)],
			endowed_accounts: vec![
				(ALICE, BTC, 1000),
				(BOB, BTC, 1000),
				(ALICE, DNAR, 1000),
				(BOB, DNAR, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: self.endowed_native,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}