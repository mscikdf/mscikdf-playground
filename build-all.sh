#!/usr/bin/env bash
set -euo pipefail

echo "=== Building macOS version ==="
cargo build --release --bin mscikdf_cli
cp target/release/mscikdf_cli cli/mscikdf_cli_macos

echo "=== Building Linux GNU version ==="
cross build --release --target x86_64-unknown-linux-gnu --bin mscikdf_cli
cp target/x86_64-unknown-linux-gnu/release/mscikdf_cli cli/mscikdf_cli_linux

say "Done!"
