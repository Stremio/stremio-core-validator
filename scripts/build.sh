#!/bin/sh
set -ex
wasm-pack build --no-typescript --out-dir wasm_build --release --target nodejs
mv ./wasm_build/stremio_core_validator_bg.wasm stremio_core_validator_bg.wasm
mv ./wasm_build/stremio_core_validator.js stremio_core_validator.js