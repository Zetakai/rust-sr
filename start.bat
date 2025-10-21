@echo off
echo Starting Rust Song Request Manager...

REM Stop any existing server first
echo Stopping any existing server...
taskkill /F /IM rust-sr.exe >nul 2>nul
if %errorlevel% equ 0 (
    echo Existing server stopped
    timeout /t 2 /nobreak >nul
) else (
    echo No existing server found
)

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust not found! Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Check if .env file exists
if not exist .env (
    echo Creating .env file...
    echo YOUTUBE_API_KEY=your_youtube_api_key_here > .env
    echo Please edit .env file and add your YouTube API key
    pause
    exit /b 1
)

REM Check if API key is set
findstr /C:"your_youtube_api_key_here" .env >nul
if %errorlevel% equ 0 (
    echo Please edit .env file and add your YouTube API key
    pause
    exit /b 1
)

REM Build and run the application
echo Building and starting server...
cargo run

REM If cargo run fails, show error
if %errorlevel% neq 0 (
    echo Build failed! Check the error messages above.
    pause
)
