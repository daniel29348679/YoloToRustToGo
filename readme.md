## Initialization

Download the [yolov12x.pt](https://github.com/sunsmarterjie/yolov12/releases/download/v1.0/yolov12x.pt) file and place it into the `yolov12` directory.

## Build


`sync_python.ps1`
Edit python file in the yolov12 folder and run sync_python.ps1 .
The python file will copy to all the location that necessary.

`build_sync_rust.ps1`
Edit rust file in the test_dll folder and run build_sync_rust.ps1 .

`build_sync_go.ps1`
Edit go file in the go_gin folder and run build_sync_go.ps1 .

## Test

rust: `cargo test`

go: `go build main.go`

## Run
```
cd all_program
./main.exe
```
