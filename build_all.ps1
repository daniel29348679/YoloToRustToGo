Write-Host "Copy python"
./sync_python.ps1 | Out-Null
Write-Host "Build an copy rust"
./build_sync_rust.ps1 | Out-Null
Write-Host "Build an copy go"
./build_sync_go.ps1 | Out-Null
Write-Host "Success"