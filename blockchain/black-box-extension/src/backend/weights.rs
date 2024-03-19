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

//! Autogenerated weights for black-box_extension
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-31, STEPS: `20`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("./black-box-benchmark-chainspec.json"), DB CACHE: 1024

// Executed Command:
// target/production/setheum-node
// benchmark
// pallet
// --chain=./black-box-benchmark-chainspec.json
// --pallet=black-box_extension
// --extrinsic=*
// --steps=20
// --repeat=5
// --template=.maintain/pallet-weight-template.hbs
// --wasm-execution=compiled
// --output=black-box-extension/src/backend/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::{
    marker::PhantomData,
    vec::Vec
};
use crate::backend::ByteCount;

/// Weight functions needed for black-box_extension.
pub trait WeightInfo {
    /// Weight for `verify` call.
    ///
    /// # Arguments
    ///
    /// To be added and measured (we are waiting for the proving backend choice).
    fn verify() -> Weight;

    /// Weight of reading arguments for the `verify` call.
    ///
    /// # Arguments
    ///
    /// * `input_length` - length of the input buffer.
    fn verify_read_args(input_length: ByteCount) -> Weight;
}

impl<I: BenchmarkInfo> WeightInfo for I {
    fn verify() -> Weight {
        let (ref_times, proof_sizes) = [
            <I as BenchmarkInfo>::verify_1_1(),
            <I as BenchmarkInfo>::verify_1_8(),
            <I as BenchmarkInfo>::verify_1_64(),
            <I as BenchmarkInfo>::verify_1_512(),
            <I as BenchmarkInfo>::verify_1_4096(),
            <I as BenchmarkInfo>::verify_2_1(),
            <I as BenchmarkInfo>::verify_2_8(),
            <I as BenchmarkInfo>::verify_2_64(),
            <I as BenchmarkInfo>::verify_2_512(),
            <I as BenchmarkInfo>::verify_2_4096(),
            <I as BenchmarkInfo>::verify_8_1(),
            <I as BenchmarkInfo>::verify_8_8(),
            <I as BenchmarkInfo>::verify_8_64(),
            <I as BenchmarkInfo>::verify_8_512(),
            <I as BenchmarkInfo>::verify_8_4096(),
            <I as BenchmarkInfo>::verify_16_1(),
            <I as BenchmarkInfo>::verify_16_8(),
            <I as BenchmarkInfo>::verify_16_64(),
            <I as BenchmarkInfo>::verify_16_512(),
            <I as BenchmarkInfo>::verify_16_4096(),
            <I as BenchmarkInfo>::verify_64_1(),
            <I as BenchmarkInfo>::verify_64_8(),
            <I as BenchmarkInfo>::verify_64_64(),
            <I as BenchmarkInfo>::verify_64_512(),
            <I as BenchmarkInfo>::verify_64_4096(),
            <I as BenchmarkInfo>::verify_128_1(),
            <I as BenchmarkInfo>::verify_128_8(),
            <I as BenchmarkInfo>::verify_128_64(),
            <I as BenchmarkInfo>::verify_128_512(),
            <I as BenchmarkInfo>::verify_128_4096(),
        ].iter().map(|w|(w.ref_time(), w.proof_size())).unzip::<_,_,Vec<_>, Vec<_>>();
        Weight::from_parts(
            *ref_times.iter().max().unwrap(),
            *proof_sizes.iter().max().unwrap(),
        )
    }

    fn verify_read_args(input_length: ByteCount) -> Weight {
        <I as BenchmarkInfo>::verify_read_args(input_length)
    }
}

/// Benchmark results for black-box_extension.
trait BenchmarkInfo {
    fn verify_read_args(x: u32, ) -> Weight;
    fn verify_1_1() -> Weight;
    fn verify_1_8() -> Weight;
    fn verify_1_64() -> Weight;
    fn verify_1_512() -> Weight;
    fn verify_1_4096() -> Weight;
    fn verify_2_1() -> Weight;
    fn verify_2_8() -> Weight;
    fn verify_2_64() -> Weight;
    fn verify_2_512() -> Weight;
    fn verify_2_4096() -> Weight;
    fn verify_8_1() -> Weight;
    fn verify_8_8() -> Weight;
    fn verify_8_64() -> Weight;
    fn verify_8_512() -> Weight;
    fn verify_8_4096() -> Weight;
    fn verify_16_1() -> Weight;
    fn verify_16_8() -> Weight;
    fn verify_16_64() -> Weight;
    fn verify_16_512() -> Weight;
    fn verify_16_4096() -> Weight;
    fn verify_64_1() -> Weight;
    fn verify_64_8() -> Weight;
    fn verify_64_64() -> Weight;
    fn verify_64_512() -> Weight;
    fn verify_64_4096() -> Weight;
    fn verify_128_1() -> Weight;
    fn verify_128_8() -> Weight;
    fn verify_128_64() -> Weight;
    fn verify_128_512() -> Weight;
    fn verify_128_4096() -> Weight;
}

/// Weights for black-box_extension using the Substrate node and recommended hardware.
pub struct AlephWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> BenchmarkInfo for AlephWeight<T> {
    /// The range of component `x` is `[0, 10000000]`.
    fn verify_read_args(x: u32, ) -> Weight {
        // Minimum execution time: 276 nanoseconds.
        Weight::from_parts(3_923_964_u64, 0)
            // Standard Error: 8
            .saturating_add(Weight::from_parts(194_u64, 0).saturating_mul(x as u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_1() -> Weight {
        // Minimum execution time: 4_781_232 nanoseconds.
        Weight::from_parts(4_982_180_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_8() -> Weight {
        // Minimum execution time: 4_340_715 nanoseconds.
        Weight::from_parts(5_165_108_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_64() -> Weight {
        // Minimum execution time: 4_485_284 nanoseconds.
        Weight::from_parts(4_981_745_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_512() -> Weight {
        // Minimum execution time: 4_803_506 nanoseconds.
        Weight::from_parts(6_006_316_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_4096() -> Weight {
        // Minimum execution time: 4_122_847 nanoseconds.
        Weight::from_parts(5_316_021_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_1() -> Weight {
        // Minimum execution time: 4_675_330 nanoseconds.
        Weight::from_parts(5_947_865_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_8() -> Weight {
        // Minimum execution time: 4_344_787 nanoseconds.
        Weight::from_parts(6_456_823_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_64() -> Weight {
        // Minimum execution time: 4_342_887 nanoseconds.
        Weight::from_parts(6_298_361_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_512() -> Weight {
        // Minimum execution time: 4_743_598 nanoseconds.
        Weight::from_parts(5_603_127_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_4096() -> Weight {
        // Minimum execution time: 4_443_366 nanoseconds.
        Weight::from_parts(4_548_845_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_1() -> Weight {
        // Minimum execution time: 4_667_318 nanoseconds.
        Weight::from_parts(4_848_928_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_8() -> Weight {
        // Minimum execution time: 4_194_110 nanoseconds.
        Weight::from_parts(4_359_367_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_64() -> Weight {
        // Minimum execution time: 4_624_304 nanoseconds.
        Weight::from_parts(4_953_812_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_512() -> Weight {
        // Minimum execution time: 4_386_560 nanoseconds.
        Weight::from_parts(4_843_617_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_4096() -> Weight {
        // Minimum execution time: 5_471_325 nanoseconds.
        Weight::from_parts(6_721_609_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_1() -> Weight {
        // Minimum execution time: 5_506_409 nanoseconds.
        Weight::from_parts(5_831_128_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_8() -> Weight {
        // Minimum execution time: 4_628_534 nanoseconds.
        Weight::from_parts(5_042_080_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_64() -> Weight {
        // Minimum execution time: 4_280_379 nanoseconds.
        Weight::from_parts(4_417_556_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_512() -> Weight {
        // Minimum execution time: 4_396_583 nanoseconds.
        Weight::from_parts(4_646_136_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_4096() -> Weight {
        // Minimum execution time: 4_217_327 nanoseconds.
        Weight::from_parts(4_363_187_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_1() -> Weight {
        // Minimum execution time: 4_453_245 nanoseconds.
        Weight::from_parts(4_896_630_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_8() -> Weight {
        // Minimum execution time: 4_429_134 nanoseconds.
        Weight::from_parts(4_878_891_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_64() -> Weight {
        // Minimum execution time: 4_575_227 nanoseconds.
        Weight::from_parts(4_861_997_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_512() -> Weight {
        // Minimum execution time: 4_046_863 nanoseconds.
        Weight::from_parts(4_223_781_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_4096() -> Weight {
        // Minimum execution time: 4_455_213 nanoseconds.
        Weight::from_parts(4_650_260_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_1() -> Weight {
        // Minimum execution time: 4_747_609 nanoseconds.
        Weight::from_parts(5_797_267_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_8() -> Weight {
        // Minimum execution time: 4_441_624 nanoseconds.
        Weight::from_parts(4_687_956_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_64() -> Weight {
        // Minimum execution time: 4_552_014 nanoseconds.
        Weight::from_parts(4_855_655_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_512() -> Weight {
        // Minimum execution time: 4_710_129 nanoseconds.
        Weight::from_parts(5_953_031_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_4096() -> Weight {
        // Minimum execution time: 4_501_463 nanoseconds.
        Weight::from_parts(4_810_933_000_u64, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
}

// For backwards compatibility and tests
impl BenchmarkInfo for () {
    /// The range of component `x` is `[0, 10000000]`.
    fn verify_read_args(x: u32, ) -> Weight {
        // Minimum execution time: 276 nanoseconds.
        Weight::from_parts(3_923_964_u64, 0)
            // Standard Error: 8
            .saturating_add(Weight::from_parts(194_u64, 0).saturating_mul(x as u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_1() -> Weight {
        // Minimum execution time: 4_781_232 nanoseconds.
        Weight::from_parts(4_982_180_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_8() -> Weight {
        // Minimum execution time: 4_340_715 nanoseconds.
        Weight::from_parts(5_165_108_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_64() -> Weight {
        // Minimum execution time: 4_485_284 nanoseconds.
        Weight::from_parts(4_981_745_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_512() -> Weight {
        // Minimum execution time: 4_803_506 nanoseconds.
        Weight::from_parts(6_006_316_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_1_4096() -> Weight {
        // Minimum execution time: 4_122_847 nanoseconds.
        Weight::from_parts(5_316_021_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_1() -> Weight {
        // Minimum execution time: 4_675_330 nanoseconds.
        Weight::from_parts(5_947_865_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_8() -> Weight {
        // Minimum execution time: 4_344_787 nanoseconds.
        Weight::from_parts(6_456_823_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_64() -> Weight {
        // Minimum execution time: 4_342_887 nanoseconds.
        Weight::from_parts(6_298_361_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_512() -> Weight {
        // Minimum execution time: 4_743_598 nanoseconds.
        Weight::from_parts(5_603_127_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_2_4096() -> Weight {
        // Minimum execution time: 4_443_366 nanoseconds.
        Weight::from_parts(4_548_845_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_1() -> Weight {
        // Minimum execution time: 4_667_318 nanoseconds.
        Weight::from_parts(4_848_928_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_8() -> Weight {
        // Minimum execution time: 4_194_110 nanoseconds.
        Weight::from_parts(4_359_367_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_64() -> Weight {
        // Minimum execution time: 4_624_304 nanoseconds.
        Weight::from_parts(4_953_812_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_512() -> Weight {
        // Minimum execution time: 4_386_560 nanoseconds.
        Weight::from_parts(4_843_617_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_8_4096() -> Weight {
        // Minimum execution time: 5_471_325 nanoseconds.
        Weight::from_parts(6_721_609_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_1() -> Weight {
        // Minimum execution time: 5_506_409 nanoseconds.
        Weight::from_parts(5_831_128_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_8() -> Weight {
        // Minimum execution time: 4_628_534 nanoseconds.
        Weight::from_parts(5_042_080_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_64() -> Weight {
        // Minimum execution time: 4_280_379 nanoseconds.
        Weight::from_parts(4_417_556_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_512() -> Weight {
        // Minimum execution time: 4_396_583 nanoseconds.
        Weight::from_parts(4_646_136_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_16_4096() -> Weight {
        // Minimum execution time: 4_217_327 nanoseconds.
        Weight::from_parts(4_363_187_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_1() -> Weight {
        // Minimum execution time: 4_453_245 nanoseconds.
        Weight::from_parts(4_896_630_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_8() -> Weight {
        // Minimum execution time: 4_429_134 nanoseconds.
        Weight::from_parts(4_878_891_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_64() -> Weight {
        // Minimum execution time: 4_575_227 nanoseconds.
        Weight::from_parts(4_861_997_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_512() -> Weight {
        // Minimum execution time: 4_046_863 nanoseconds.
        Weight::from_parts(4_223_781_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_64_4096() -> Weight {
        // Minimum execution time: 4_455_213 nanoseconds.
        Weight::from_parts(4_650_260_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_1() -> Weight {
        // Minimum execution time: 4_747_609 nanoseconds.
        Weight::from_parts(5_797_267_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_8() -> Weight {
        // Minimum execution time: 4_441_624 nanoseconds.
        Weight::from_parts(4_687_956_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_64() -> Weight {
        // Minimum execution time: 4_552_014 nanoseconds.
        Weight::from_parts(4_855_655_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_512() -> Weight {
        // Minimum execution time: 4_710_129 nanoseconds.
        Weight::from_parts(5_953_031_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    // Storage: `VkStorage::VerificationKeys` (r:1 w:0)
    // Proof: `VkStorage::VerificationKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn verify_128_4096() -> Weight {
        // Minimum execution time: 4_501_463 nanoseconds.
        Weight::from_parts(4_810_933_000_u64, 0)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
}
