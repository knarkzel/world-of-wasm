set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/world_of_wasm.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p docs/
wasm-opt -o docs/world_of_wasm.wasm -Oz $BINARY
ls -lh docs/world_of_wasm.wasm
