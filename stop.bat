@echo off
echo Stopping Rust Song Request Manager...

REM Kill the rust-sr process
taskkill /F /IM rust-sr.exe >nul 2>nul
if %errorlevel% equ 0 (
    echo Server stopped successfully
) else (
    echo No server process found
)

pause
