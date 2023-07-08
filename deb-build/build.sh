#!/bin/bash
set -ex

if ! command -v cargo &> /dev/null
then
    echo rust tools installing...ll
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    export PATH="$HOME/.cargo/bin:$PATH"
fi

BUILD_RUST="$(pwd)/build-rust"

mkdir -p ${BUILD_RUST}

cargo build --release --target-dir ${BUILD_RUST}
