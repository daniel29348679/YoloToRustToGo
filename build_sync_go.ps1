Set-Location go_gin
go build main.go
Set-Location ..
Copy-Item go_gin/main.exe -Destination .\all_program -Force
