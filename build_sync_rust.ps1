Set-Location rust_dll
Write-Host "Building Rust project..."
cargo build --release | Out-Null
cbindgen --config cbindgen.toml --crate rust_dll --output rust_dll.h | Out-Null
Set-Location ..
Write-Host "Copy Rust DLL"
Copy-Item rust_dll/target/release/rust_dll.* -Destination go_gin -Force | Out-Null
Copy-Item rust_dll/target/release/rust_dll.* -Destination all_program -Force | Out-Null
Copy-Item rust_dll/rust_dll.h -Destination go_gin -Force | Out-Null
Copy-Item rust_dll/rust_dll.h -Destination all_program -Force | Out-Null
Write-Host "Success!!"