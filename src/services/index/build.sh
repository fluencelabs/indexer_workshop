#!/bin/sh

# This script builds project to Wasm and puts all created binary Wasm module into the artifacts dir
marine build --release

# rm artifacts/* || true
mkdir -p artifacts

cp ./target/wasm32-wasi/release/index.wasm artifacts/facade.wasm
# wget https://github.com/fluencelabs/sqlite/releases/download/v0.16.0_w/sqlite3.wasm artifacts/
# curl -L https://github.com/fluencelabs/sqlite/releases/download/v0.16.0_w/sqlite3.wasm -o artifacts/sqlite3.wasm
# marine set version --input artifacts/sqlite3.wasm --version 0.6.0

cp artifacts/* ../../../artifacts/index/
