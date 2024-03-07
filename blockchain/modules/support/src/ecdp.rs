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

use parity_scale_codec::FullCodec;
use primitives::EcdpEcdpPosition;
use sp_core::U256;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
	prelude::*,
};

use crate::{dex::*, ExchangeRate, Ratio};

pub trait EcdpEmergencyShutdown {
	fn is_shutdown() -> bool;
}

pub trait EcdpAuctionsManager<AccountId> {
	type CurrencyId;
	type Balance;
	type AuctionId: FullCodec + Debug + Clone + Eq + PartialEq;

	fn new_collateral_auction(
		refund_recipient: &AccountId,
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
	) -> DispatchResult;
	fn cancel_auction(id: Self::AuctionId) -> DispatchResult;
	fn get_total_collateral_in_auction(id: Self::CurrencyId) -> Self::Balance;
	fn get_total_target_in_auction() -> Self::Balance;
}

pub trait EcdpUssdRiskManager<AccountId, CurrencyId, Balance, DebitBalance> {
	fn get_debit_value(currency_id: CurrencyId, debit_balance: DebitBalance) -> Balance;

	fn check_position_valid(
		currency_id: CurrencyId,
		collateral_balance: Balance,
		debit_balance: DebitBalance,
		check_required_ratio: bool,
	) -> DispatchResult;

	fn check_debit_cap(currency_id: CurrencyId, total_debit_balance: DebitBalance) -> DispatchResult;
}

#[cfg(feature = "std")]
impl<AccountId, CurrencyId, Balance: Default, DebitBalance> EcdpUssdRiskManager<AccountId, CurrencyId, Balance, DebitBalance>
	for ()
{
	fn get_debit_value(_currency_id: CurrencyId, _debit_balance: DebitBalance) -> Balance {
		Default::default()
	}

	fn check_position_valid(
		_currency_id: CurrencyId,
		_collateral_balance: Balance,
		_debit_balance: DebitBalance,
		_check_required_ratio: bool,
	) -> DispatchResult {
		Ok(())
	}

	fn check_debit_cap(_currency_id: CurrencyId, _total_debit_balance: DebitBalance) -> DispatchResult {
		Ok(())
	}
}

/// An abstraction of cdp treasury for SlickUSD ECDP Protocol.
pub trait EcdpUssdTreasury<AccountId> {
	type Balance;
	type CurrencyId;

	/// get surplus amount of cdp treasury
	fn get_surplus_pool() -> Self::Balance;

	/// get debit amount of cdp treasury
	fn get_debit_pool() -> Self::Balance;

	/// get collateral assets amount of cdp treasury
	fn get_total_collaterals(id: Self::CurrencyId) -> Self::Balance;

	/// calculate the proportion of specific debit amount for the whole system
	fn get_debit_proportion(amount: Self::Balance) -> Ratio;

	/// issue debit for cdp treasury
	fn on_system_debit(amount: Self::Balance) -> DispatchResult;

	/// issue surplus(USSD) for cdp treasury
	fn on_system_surplus(amount: Self::Balance) -> DispatchResult;

	/// issue debit to `who`
	/// if backed flag is true, means the debit to issue is backed on some
	/// assets, otherwise will increase same amount of debit to system debit.
	fn issue_debit(who: &AccountId, debit: Self::Balance, backed: bool) -> DispatchResult;

	/// burn debit(USSD) of `who`
	fn burn_debit(who: &AccountId, debit: Self::Balance) -> DispatchResult;

	/// deposit surplus(USSD) to cdp treasury by `from`
	fn deposit_surplus(from: &AccountId, surplus: Self::Balance) -> DispatchResult;

	/// withdraw surplus(USSD) from cdp treasury to `to`
	fn withdraw_surplus(to: &AccountId, surplus: Self::Balance) -> DispatchResult;

	/// deposit collateral assets to cdp treasury by `who`
	fn deposit_collateral(from: &AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;

	/// withdraw collateral assets of cdp treasury to `who`
	fn withdraw_collateral(to: &AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;
}

pub trait EcdpUssdTreasuryExtended<AccountId>: SlickUsdTreasury<AccountId> {
	fn swap_collateral_to_ussd(
		currency_id: Self::CurrencyId,
		limit: SwapLimit<Self::Balance>,
		collateral_in_auction: bool,
	) -> sp_std::result::Result<(Self::Balance, Self::Balance), DispatchError>;

	fn create_collateral_auctions(
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
		refund_receiver: AccountId,
		splited: bool,
	) -> sp_std::result::Result<u32, DispatchError>;

	fn remove_liquidity_for_lp_collateral(
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
	) -> sp_std::result::Result<(Self::Balance, Self::Balance), DispatchError>;

	fn max_auction() -> u32;
}

/// Functionality of SlickUSD ECDP Protocol to be exposed to EVM.
pub trait EcdpUssdManager<AccountId, CurrencyId, Amount, Balance> {
	/// Adjust ECDP loan
	fn adjust_loan(
		who: &AccountId,
		currency_id: CurrencyId,
		collateral_adjustment: Amount,
		debit_adjustment: Amount,
	) -> DispatchResult;
	/// Close ECDP loan using DEX
	fn close_loan_by_dex(who: AccountId, currency_id: CurrencyId, max_collateral_amount: Balance) -> DispatchResult;
	/// Get open ECDP corresponding to an account and collateral `CurrencyId`
	fn get_position(who: &AccountId, currency_id: CurrencyId) -> EcdpEcdpPosition;
	/// Get liquidation ratio for collateral `CurrencyId`
	fn get_collateral_parameters(currency_id: CurrencyId) -> Vec<U256>;
	/// Get current ratio of collateral to debit of open ECDP
	fn get_current_collateral_ratio(who: &AccountId, currency_id: CurrencyId) -> Option<Ratio>;
	/// Get exchange rate of debit units to debit value for a currency_id
	fn get_debit_exchange_rate(currency_id: CurrencyId) -> ExchangeRate;
}

/// Functionality of Setter ECDP Protocol to be exposed to EVM.
pub trait EcdpSetrManager<AccountId, Amount, Balance> {
	/// Adjust ECDP loan
	fn adjust_loan(
		who: &AccountId,
		collateral_adjustment: Amount,
		debit_adjustment: Amount,
	) -> DispatchResult;
	/// Close ECDP loan using DEX
	fn close_loan_by_dex(who: AccountId, max_collateral_amount: Balance) -> DispatchResult;
	/// Get open ECDP corresponding to an account and collateral
	fn get_position(who: &AccountId) -> EcdpEcdpPosition;
	/// Get liquidation ratio for collateral
	fn get_collateral_parameters() -> Vec<U256>;
	/// Get current ratio of collateral to debit of open ECDP
	fn get_current_collateral_ratio(who: &AccountId) -> Option<Ratio>;
	/// Get exchange rate of debit units to debit value for a currency_id
	fn get_debit_exchange_rate() -> ExchangeRate;
}
