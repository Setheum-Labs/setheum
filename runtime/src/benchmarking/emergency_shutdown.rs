// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// ٱلَّذِينَ يَأْكُلُونَ ٱلرِّبَوٰا۟ لَا يَقُومُونَ إِلَّا كَمَا يَقُومُ ٱلَّذِى يَتَخَبَّطُهُ ٱلشَّيْطَـٰنُ مِنَ ٱلْمَسِّ ۚ ذَٰلِكَ بِأَنَّهُمْ قَالُوٓا۟ إِنَّمَا ٱلْبَيْعُ مِثْلُ ٱلرِّبَوٰا۟ ۗ وَأَحَلَّ ٱللَّهُ ٱلْبَيْعَ وَحَرَّمَ ٱلرِّبَوٰا۟ ۚ فَمَن جَآءَهُۥ مَوْعِظَةٌ مِّن رَّبِّهِۦ فَٱنتَهَىٰ فَلَهُۥ مَا سَلَفَ وَأَمْرُهُۥٓ إِلَى ٱللَّهِ ۖ وَمَنْ عَادَ فَأُو۟لَـٰٓئِكَ أَصْحَـٰبُ ٱلنَّارِ ۖ هُمْ فِيهَا خَـٰلِدُونَ

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

use crate::{
	dollar, AccountId, CdpTreasury, CollateralCurrencyIds, CurrencyId, EmergencyShutdown, GetSetUSDId, Price,
	Runtime,
};

use super::utils::{feed_price, set_balance};
use frame_benchmarking::{account, whitelisted_caller};
use frame_system::RawOrigin;
use module_support::CDPTreasury;
use orml_benchmarking::runtime_benchmarks;
use sp_runtime::traits::One;
use sp_std::vec;

const SEED: u32 = 0;

const STABLECOIN: CurrencyId = GetSetUSDId::get();

runtime_benchmarks! {
	{ Runtime, emergency_shutdown }

	emergency_shutdown {
		let c in 0 .. CollateralCurrencyIds::get().len().saturating_sub(1) as u32;
		let currency_ids = CollateralCurrencyIds::get();
		let mut values = vec![];

		for i in 0 .. c {
			values.push((currency_ids[i as usize], Price::one()));
		}
		feed_price(values)?;
	}: _(RawOrigin::Root)

	open_collateral_refund {
		EmergencyShutdown::emergency_shutdown(RawOrigin::Root.into())?;
	}: _(RawOrigin::Root)

	refund_collaterals {
		let c in 0 .. CollateralCurrencyIds::get().len().saturating_sub(1) as u32;
		let currency_ids = CollateralCurrencyIds::get();
		let funder: AccountId = account("funder", 0, SEED);
		let caller: AccountId = whitelisted_caller();
		let mut values = vec![];

		for i in 0 .. c {
			let currency_id = currency_ids[i as usize];
			values.push((currency_id, Price::one()));
			set_balance(currency_id, &funder, 100 * dollar(currency_id));
			CdpTreasury::deposit_collateral(&funder, currency_id, 100 * dollar(currency_id))?;
		}
		feed_price(values)?;

		CdpTreasury::issue_debit(&caller, 1_000 * dollar(STABLECOIN), true)?;
		CdpTreasury::issue_debit(&funder, 1_000 * dollar(STABLECOIN), true)?;

		EmergencyShutdown::emergency_shutdown(RawOrigin::Root.into())?;
		EmergencyShutdown::open_collateral_refund(RawOrigin::Root.into())?;
	}: _(RawOrigin::Signed(caller),  1_000 * dollar(STABLECOIN))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use orml_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(new_test_ext(),);
}
