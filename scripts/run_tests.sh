#!/bin/bash

# UI Test Runner for b3scale-admin
# Runs WebAssembly tests in browser using wasm-bindgen-test

echo "🚀 Running b3scale-admin UI Tests..."
echo "============================================"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build and run tests using wasm-pack
echo "📦 Building and running tests with wasm-pack..."
wasm-pack test --headless --chrome

echo ""
echo "🎯 Alternative test methods:"
echo "   1. cargo test --target wasm32-unknown-unknown"
echo "   2. wasm-pack test --firefox --headless"
echo "   3. wasm-pack test --safari --headless"
echo ""

# Also run with cargo if wasm32 target is available
if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "🦀 Running with cargo test..."
    cargo test --target wasm32-unknown-unknown --lib
else
    echo "⚠️  wasm32-unknown-unknown target not installed. Run:"
    echo "   rustup target add wasm32-unknown-unknown"
fi

echo ""
echo "✅ Tests completed!"
echo "💡 To run tests manually:"
echo "   wasm-pack test --headless --chrome"
echo "   wasm-pack test --headless --firefox"