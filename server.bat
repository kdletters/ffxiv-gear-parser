@echo off
setlocal enabledelayedexpansion

REM FFXIV Share Parser - Development Server Launcher (Windows)
REM This script starts a local HTTP server for testing the WASM application

set PORT=8080
set PYTHON_CMD=

echo.
echo 🚀 FFXIV Share Parser - Starting Development Server...
echo.

REM Check for python command
where python >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%i in ('python --version 2^>^&1') do set PYTHON_VERSION=%%i
    echo ✅ Found: !PYTHON_VERSION!
    set PYTHON_CMD=python
    goto :found_python
)

REM Check for python3 command
where python3 >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%i in ('python3 --version 2^>^&1') do set PYTHON_VERSION=%%i
    echo ✅ Found: !PYTHON_VERSION!
    set PYTHON_CMD=python3
    goto :found_python
)

REM Python not found
echo ❌ Python not found!
echo.
echo Please install Python 3:
echo.
echo   1. Download from: https://www.python.org/downloads/
echo   2. Run the installer
echo   3. ⚠️  IMPORTANT: Check "Add Python to PATH" during installation
echo.
echo   Alternative - Using winget:
echo     winget install Python.Python.3.12
echo.
echo   Alternative - Using Chocolatey:
echo     choco install python
echo.
pause
exit /b 1

:found_python

REM Check if pkg directory exists
if not exist "pkg" (
    echo.
    echo ⚠️  Warning: 'pkg' directory not found!
    echo    Please run: wasm-pack build --target web
    echo.
)

REM Start the server
echo.
echo 🌐 Starting HTTP server on port %PORT%...
echo 📂 Serving directory: %CD%
echo.
echo 🎮 Open your browser to:
echo    http://localhost:%PORT%/index.html
echo.
echo Press Ctrl+C to stop the server
echo.
echo ─────────────────────────────────────────
echo.

REM Start Python HTTP server
%PYTHON_CMD% -m http.server %PORT%