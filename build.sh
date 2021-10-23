set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/world_of_wasm.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p www/
wasm-opt -o www/world_of_wasm.wasm -Oz $BINARY
ls -lh www/world_of_wasm.wasm
