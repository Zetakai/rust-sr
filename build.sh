#!/bin/bash

set -e

echo "Starting build process..."

# Check if we're in a Cloudflare Workers environment
if [ -n "$CF_PAGES" ]; then
    echo "Running in Cloudflare Pages environment"
    
    # Install Rust toolchain
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    
    # Install wasm-pack
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    source $HOME/.cargo/env
    
    # Add wasm32 target
    rustup target add wasm32-unknown-unknown
else
    echo "Running in local environment"
    # Check if wasm-pack is available
    if ! command -v wasm-pack &> /dev/null; then
        echo "Installing wasm-pack..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
fi

# Build the worker
echo "Building Rust worker..."
cd worker
wasm-pack build --target web --out-dir ../dist

echo "Build completed successfully!"
