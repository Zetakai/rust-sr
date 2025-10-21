#!/bin/bash

echo "🚀 Starting Rust Song Request Manager..."

# Stop any existing server first
echo "🛑 Stopping any existing server..."
PIDS=$(pgrep -f "rust-sr")
if [ -n "$PIDS" ]; then
    echo "🔍 Found existing server processes: $PIDS"
    kill -TERM $PIDS
    sleep 2
    # Force kill if still running
    if pgrep -f "rust-sr" > /dev/null; then
        echo "🔨 Force stopping processes..."
        pkill -f "rust-sr"
    fi
    echo "✅ Existing server stopped"
else
    echo "ℹ️  No existing server found"
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found! Please install Rust from https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found: $(cargo --version)"

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file..."
    echo "YOUTUBE_API_KEY=your_youtube_api_key_here" > .env
    echo "❌ Please edit .env file and add your YouTube API key"
    echo "   nano .env"
    exit 1
fi

# Check if API key is set
if grep -q "your_youtube_api_key_here" .env; then
    echo "❌ Please edit .env file and add your YouTube API key"
    echo "   nano .env"
    exit 1
fi

echo "✅ Environment configured"

# Build and run the application
echo "🔨 Building and starting server..."
cargo run

# If cargo run fails, show error
if [ $? -ne 0 ]; then
    echo "❌ Build failed! Check the error messages above."
    exit 1
fi
