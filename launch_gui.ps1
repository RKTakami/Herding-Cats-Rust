# PowerShell script to launch Herding Cats Rust with proper GUI display
Write-Host "Starting Herding Cats Rust Application..." -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "The application window should appear shortly." -ForegroundColor White
Write-Host "Window Title: 'Herding Cats Word Processor'" -ForegroundColor Cyan
Write-Host ""
Write-Host "If you don't see the window:" -ForegroundColor Yellow
Write-Host "1. Check your taskbar for the application" -ForegroundColor White
Write-Host "2. Try Alt+Tab to cycle through windows" -ForegroundColor White
Write-Host "3. Check if the window is minimized" -ForegroundColor White
Write-Host "4. Check other monitors if you have multiple displays" -ForegroundColor White
Write-Host ""

# Change to the correct directory
Set-Location "target\debug"

# Launch the application
$Process = Start-Process -FilePath "herding-cats-rust.exe" -PassThru -WindowStyle Normal

Write-Host "Application launched with Process ID: $($Process.Id)" -ForegroundColor Green
Write-Host "Waiting for window to appear..." -ForegroundColor White

# Wait a bit for the window to initialize
Start-Sleep -Seconds 3

# Check if process is still running
if (Get-Process -Id $Process.Id -ErrorAction SilentlyContinue) {
    Write-Host "✅ Application is running successfully!" -ForegroundColor Green
    Write-Host "Process ID: $($Process.Id)" -ForegroundColor Cyan
    Write-Host "Memory Usage: $((Get-Process -Id $Process.Id).WorkingSet64 / 1MB -as [int]) MB" -ForegroundColor Cyan
}
else {
    Write-Host "❌ Application failed to start properly" -ForegroundColor Red
}

Write-Host ""
Write-Host "To close the application:" -ForegroundColor Yellow
Write-Host "1. Click the X button on the window" -ForegroundColor White
Write-Host "2. Or press Ctrl+C in this terminal" -ForegroundColor White
Write-Host ""
Write-Host "Monitoring application... Press Ctrl+C to stop monitoring" -ForegroundColor Gray

try {
    # Keep monitoring until the process ends
    while (Get-Process -Id $Process.Id -ErrorAction SilentlyContinue) {
        Start-Sleep -Seconds 5
        $Memory = (Get-Process -Id $Process.Id).WorkingSet64 / 1MB -as [int]
        Write-Host "App running... Memory: $Memory MB" -ForegroundColor DarkGray -NoNewline
        Write-Host "`r" -NoNewline
    }
    Write-Host ""
    Write-Host "✅ Application has been closed" -ForegroundColor Green
}
catch {
    Write-Host ""
    Write-Host "❌ Monitoring stopped" -ForegroundColor Red
}
