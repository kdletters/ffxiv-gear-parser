#!/bin/bash

# FFXIV Share Parser - Development Server Launcher
# This script starts a local HTTP server for testing the WASM application

PORT=8080
PYTHON_CMD=""

echo "🚀 FFXIV Share Parser - Starting Development Server..."
echo ""

# Check for python command
if command -v python &> /dev/null; then
    PYTHON_VERSION=$(python --version 2>&1)
    echo "✅ Found: $PYTHON_VERSION"
    PYTHON_CMD="python"
elif command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version 2>&1)
    echo "✅ Found: $PYTHON_VERSION"
    PYTHON_CMD="python3"
else
    echo "❌ Python not found!"
    echo ""
    echo "Please install Python 3:"
    echo ""

    # Detect OS and provide appropriate installation instructions
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "  Ubuntu/Debian:"
        echo "    sudo apt update && sudo apt install python3"
        echo ""
        echo "  Fedora/RHEL:"
        echo "    sudo dnf install python3"
        echo ""
        echo "  Arch Linux:"
        echo "    sudo pacman -S python"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "  macOS (using Homebrew):"
        echo "    brew install python3"
        echo ""
        echo "  Or download from: https://www.python.org/downloads/"
    else
        echo "  Download from: https://www.python.org/downloads/"
    fi

    echo ""
    exit 1
fi

# Check if pkg directory exists
if [ ! -d "pkg" ]; then
    echo ""
    echo "⚠️  Warning: 'pkg' directory not found!"
    echo "   Please run: wasm-pack build --target web"
    echo ""
fi

# Start the server
echo ""
echo "🌐 Starting HTTP server on port $PORT..."
echo "📂 Serving directory: $(pwd)"
echo ""
echo "🎮 Open your browser to:"
echo "   http://localhost:$PORT/index.html"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""
echo "─────────────────────────────────────────"
echo ""

# Start Python HTTP server
$PYTHON_CMD -m http.server $PORT