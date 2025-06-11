# YOLOv12 Setup and Execution Guide

## Initialization

1. Download the [yolov12x.pt](https://github.com/sunsmarterjie/yolov12/releases/download/v1.0/yolov12x.pt) model file and save it in the `yolov12` directory.
2. Install `cbindgen` (a toolchain for converting Rust scripts to C headers):
   ```
   cargo install cbindgen
   ```

## Quick Build 
1. **run**:
    ```
    ./build_all.ps1
    ```
2. You can find all files collected in the all_program dir.

## Build Process

1. **Python Sync**:
   - Modify the Python file in the `yolov12` directory.
   - Execute `sync_python.ps1` to copy the Python file to all required locations.

2. **Rust Sync**:
   - Edit the Rust file in the `test_dll` directory.
   - Run `build_sync_rust.ps1` to synchronize and build.

3. **Go Sync**:
   - Update the Go file in the `go_gin` directory.
   - Run `build_sync_go.ps1` to synchronize and build.


## Testing

1. **Rust Tests**:
   ```
   cargo test
   ```

2. **Go Build**:
   ```
   go build main.go
   ```

3. **Run All Programs**:
   - Open two terminals:
     - **Terminal 1**:
       ```
       ./all_program/main.exe
       ```
     - **Terminal 2**:
       ```
       py all_program/test_predict.py
       ```
   - **Expected Output**:
     ```
     ⚙️ Load model 'yolov12x.pt' result: 0
     🔍 Prediction result: {"cls": 15.0, "conf": 0.9500255584716797, "data": [[15.704132080078125, 88.18228149414062, 1437.7071533203125, 1080.0, 0.9500255584716797, 15.0]], "id": null, "is_track": false, "orig_shape": [1080, 1440], "shape": [1, 6], "xywh": [[726.7056274414062, 584.0911254882812, 1422.0030517578125, 991.8177490234375]], "xywhn": [[0.5046566724777222, 0.5408251285552979, 0.9875021576881409, 0.9183497428894043]], "xyxy": [[15.704132080078125, 88.18228149414062, 1437.7071533203125, 1080.0]], "xyxyn": [[0.010905647650361061, 0.0816502571105957, 0.998407781124115, 1.0]]}
     📋 Available models: ['yolov12x.pt']
     ```

## Running the Program

1. Navigate to the `all_program` directory:
   ```
   cd all_program
   ```
2. Execute the main program:
   ```
   ./main.exe
   ```