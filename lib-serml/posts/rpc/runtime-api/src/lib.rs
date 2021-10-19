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
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

use slixon_posts::rpc::{FlatPost, FlatPostKind, RepliesByPostId};
use slixon_utils::{PostId, ChannelId};

sp_api::decl_runtime_apis! {
    pub trait PostsApi<AccountId, BlockNumber> where
        AccountId: Codec,
        BlockNumber: Codec
    {
        fn get_next_post_id() -> PostId;

        fn get_posts_by_ids(post_ids: Vec<PostId>, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;

        fn get_public_posts(kind_filter: Vec<FlatPostKind>, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;

        fn get_public_posts_by_channel_id(channel_id: ChannelId, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;
    
        fn get_unlisted_posts_by_channel_id(channel_id: ChannelId, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;

        fn get_public_post_ids_by_channel_id(channel_id: ChannelId) -> Vec<PostId>;

        fn get_unlisted_post_ids_by_channel_id(channel_id: ChannelId) -> Vec<PostId>;

        fn get_reply_ids_by_parent_id(parent_id: PostId) -> Vec<PostId>;

        fn get_reply_ids_by_parent_ids(parent_ids: Vec<PostId>) -> BTreeMap<PostId, Vec<PostId>>;

        fn get_replies_by_parent_id(parent_id: PostId, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;

        fn get_replies_by_parent_ids(parent_ids: Vec<PostId>, offset: u64, limit: u16) -> RepliesByPostId<AccountId, BlockNumber>;

        fn get_feed(account: AccountId, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;
    }
}
