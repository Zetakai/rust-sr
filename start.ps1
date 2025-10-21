Write-Host "Starting Rust Song Request Manager..." -ForegroundColor Green

# Stop any existing server first
Write-Host "🛑 Stopping any existing server..." -ForegroundColor Yellow
$processes = Get-Process -Name "rust-sr" -ErrorAction SilentlyContinue
if ($processes) {
    $processes | Stop-Process -Force
    Write-Host "✅ Existing server stopped" -ForegroundColor Green
    Start-Sleep -Seconds 2
} else {
    Write-Host "ℹ️  No existing server found" -ForegroundColor Yellow
}

# Check if Rust is installed
try {
    $cargoVersion = cargo --version
    Write-Host "Rust found: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "Rust not found! Please install Rust from https://rustup.rs/" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Check if .env file exists
if (-not (Test-Path ".env")) {
    Write-Host "Creating .env file..." -ForegroundColor Yellow
    "YOUTUBE_API_KEY=your_youtube_api_key_here" | Out-File -FilePath ".env" -Encoding UTF8
    Write-Host "Please edit .env file and add your YouTube API key" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Check if API key is set
$envContent = Get-Content ".env"
if ($envContent -match "your_youtube_api_key_here") {
    Write-Host "Please edit .env file and add your YouTube API key" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Build and run the application
Write-Host "Building and starting server..." -ForegroundColor Green
cargo run

# If cargo run fails, show error
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed! Check the error messages above." -ForegroundColor Red
    Read-Host "Press Enter to exit"
}
