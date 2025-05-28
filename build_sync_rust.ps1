Set-Location test_dll
cargo build --release
Set-Location ..
Copy-Item test_dll/target/release/test_dll.dll -Destination go_gin -Force
Copy-Item test_dll/target/release/test_dll.dll -Destination all_program -Force
