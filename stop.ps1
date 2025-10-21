Write-Host "Stopping Rust Song Request Manager..." -ForegroundColor Yellow

# Kill the rust-sr process
$processes = Get-Process -Name "rust-sr" -ErrorAction SilentlyContinue
if ($processes) {
    $processes | Stop-Process -Force
    Write-Host "Server stopped successfully" -ForegroundColor Green
} else {
    Write-Host "No server process found" -ForegroundColor Yellow
}

Read-Host "Press Enter to exit"
