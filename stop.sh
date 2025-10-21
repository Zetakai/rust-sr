#!/bin/bash

echo "🛑 Stopping Rust Song Request Manager..."

# Find and kill the rust-sr process
PIDS=$(pgrep -f "rust-sr")

if [ -n "$PIDS" ]; then
    echo "🔍 Found rust-sr processes: $PIDS"
    kill -TERM $PIDS
    sleep 2
    
    # Force kill if still running
    if pgrep -f "rust-sr" > /dev/null; then
        echo "🔨 Force stopping processes..."
        pkill -f "rust-sr"
    fi
    
    echo "✅ Server stopped successfully"
else
    echo "ℹ️  No server process found"
fi
