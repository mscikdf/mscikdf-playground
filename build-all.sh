#!/usr/bin/env bash
set -euo pipefail
clear

BIN_NAME="acegf_cli"
OUT_DIR="./lib"

mkdir -p "$OUT_DIR"

echo "==> Building $BIN_NAME"
echo

#######################################
# Detect host
#######################################
HOST_OS="$(uname -s)"
HOST_ARCH="$(uname -m)"

#######################################
# macOS builds (native)
#######################################
if [[ "$HOST_OS" == "Darwin" ]]; then
  echo "==> macOS build (native)"

  if [[ "$HOST_ARCH" == "arm64" ]]; then
    echo "   → macOS arm64"
    cargo build --release --bin "$BIN_NAME"
    cp "target/release/$BIN_NAME" "$OUT_DIR/${BIN_NAME}_macos_arm64"

    echo "   → macOS x86_64 (cross)"
    rustup target add x86_64-apple-darwin >/dev/null 2>&1 || true
    cargo build --release --target x86_64-apple-darwin --bin "$BIN_NAME"
    cp "target/x86_64-apple-darwin/release/$BIN_NAME" \
       "$OUT_DIR/${BIN_NAME}_macos_x86_64"

  elif [[ "$HOST_ARCH" == "x86_64" ]]; then
    echo "   → macOS x86_64"
    cargo build --release --bin "$BIN_NAME"
    cp "target/release/$BIN_NAME" "$OUT_DIR/${BIN_NAME}_macos_x86_64"

    echo "   → macOS arm64 (cross)"
    rustup target add aarch64-apple-darwin >/dev/null 2>&1 || true
    cargo build --release --target aarch64-apple-darwin --bin "$BIN_NAME"
    cp "target/aarch64-apple-darwin/release/$BIN_NAME" \
       "$OUT_DIR/${BIN_NAME}_macos_arm64"
  fi
fi

#######################################
# Linux builds (via cross)
#######################################
echo
echo "==> Linux builds (cross)"

command -v cross >/dev/null 2>&1 || {
  echo "❌ cross not installed"
  echo "   cargo install cross"
  exit 1
}

LINUX_TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
)

for TARGET in "${LINUX_TARGETS[@]}"; do
  echo "   → Linux $TARGET"
  cross build --release --target "$TARGET" --bin "$BIN_NAME"
  cp "target/$TARGET/release/$BIN_NAME" \
     "$OUT_DIR/${BIN_NAME}_linux_${TARGET%%-*}"
done

echo
echo "✅ Build completed"
ls -lh "$OUT_DIR"
