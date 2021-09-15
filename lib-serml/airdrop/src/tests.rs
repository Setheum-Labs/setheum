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

//! Unit tests for the airdrop module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Airdrop, Event, ExtBuilder, Origin, System, SETHEUM, ALICE, BOB, CHARLIE, DNAR};
use sp_runtime::traits::BadOrigin;

#[test]
fn airdrop_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(Airdrop::airdrop(Origin::signed(BOB), ALICE, DNAR, 10000), BadOrigin,);
		assert_ok!(Airdrop::airdrop(Origin::root(), ALICE, DNAR, 10000));
		System::assert_last_event(Event::airdrop(RawEvent::Airdrop(ALICE, DNAR, 10000)));
		assert_eq!(Airdrop::airdrops(ALICE, DNAR), 10000);
	});
}

#[test]
fn update_airdrop_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Airdrop::airdrop(Origin::root(), ALICE, SETHEUM, 10000));
		assert_ok!(Airdrop::airdrop(Origin::root(), ALICE, SETHEUM, 10000));
		assert_eq!(Airdrop::airdrops(ALICE, SETHEUM), 20000);
		assert_noop!(Airdrop::update_airdrop(Origin::signed(BOB), ALICE, SETHEUM, 0), BadOrigin,);
		assert_ok!(Airdrop::update_airdrop(Origin::root(), ALICE, SETHEUM, 0));
		System::assert_last_event(Event::airdrop(RawEvent::UpdateAirdrop(ALICE, SETHEUM, 0)));
		assert_eq!(Airdrop::airdrops(ALICE, SETHEUM), 0);
	});
}

#[test]
fn genesis_config_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Airdrop::airdrops(CHARLIE, DNAR), 150);
		assert_eq!(Airdrop::airdrops(CHARLIE, SETHEUM), 80);
	});
}
