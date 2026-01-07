#!/usr/bin/env pwsh

# FFXIV Share Parser - Development Server Launcher (PowerShell)
# This script starts a local HTTP server for testing the WASM application

$Port = 8080
$PythonCmd = $null

Write-Host ""
Write-Host "🚀 FFXIV Share Parser - Starting Development Server..." -ForegroundColor Cyan
Write-Host ""

# Check for python command
if (Get-Command python -ErrorAction SilentlyContinue) {
    $version = python --version 2>&1
    Write-Host "✅ Found: $version" -ForegroundColor Green
    $PythonCmd = "python"
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    $version = python3 --version 2>&1
    Write-Host "✅ Found: $version" -ForegroundColor Green
    $PythonCmd = "python3"
}
else {
    Write-Host "❌ Python not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Python 3:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  1. Download from: https://www.python.org/downloads/"
    Write-Host "  2. Run the installer"
    Write-Host "  3. ⚠️  IMPORTANT: Check 'Add Python to PATH' during installation" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Alternative - Using winget:"
    Write-Host "    winget install Python.Python.3.12"
    Write-Host ""
    Write-Host "  Alternative - Using Chocolatey:"
    Write-Host "    choco install python"
    Write-Host ""

    Read-Host "Press Enter to exit"
    exit 1
}

# Check if pkg directory exists
if (-not (Test-Path "pkg")) {
    Write-Host ""
    Write-Host "⚠️  Warning: 'pkg' directory not found!" -ForegroundColor Yellow
    Write-Host "   Please run: wasm-pack build --target web"
    Write-Host ""
}

# Start the server
Write-Host ""
Write-Host "🌐 Starting HTTP server on port $Port..." -ForegroundColor Cyan
Write-Host "📂 Serving directory: $(Get-Location)"
Write-Host ""
Write-Host "🎮 Open your browser to:" -ForegroundColor Green
Write-Host "   http://localhost:$Port/index.html" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press Ctrl+C to stop the server" -ForegroundColor Yellow
Write-Host ""
Write-Host "─────────────────────────────────────────"
Write-Host ""

# Start Python HTTP server
& $PythonCmd -m http.server $Port