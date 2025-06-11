Set-Location go_gin
Write-Host "Build Go"
go build main.go | Out-Null
Set-Location ..
Write-Host "Copy main.exe"
Copy-Item go_gin/main.exe -Destination .\all_program -Force | Out-Null
Write-Host "Success!!"