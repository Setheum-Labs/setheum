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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;

use slixon_utils::ChannelId;

sp_api::decl_runtime_apis! {
    pub trait ChannelFollowsApi<AccountId> where
        AccountId: Codec
    {
        fn get_channel_ids_followed_by_account(account: AccountId) -> Vec<ChannelId>;

        fn filter_followed_channel_ids(account: AccountId, channel_ids: Vec<ChannelId>) -> Vec<ChannelId>;
    }
}