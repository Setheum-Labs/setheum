#!/usr/bin/env bash

set -e

# cargo clean
WASM_BUILD_TYPE=release cargo run --features with-newrome-runtime --features with-sevm -- build-spec --raw --chain newrome-latest > ./resources/newrome-dist.json
