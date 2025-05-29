mod prelude;
use prelude::*;

#[ctor]
fn init() {
    init_python_interpreter()
}

#[unsafe(no_mangle)]
pub extern "C" fn init_python_interpreter() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();
        let path = std::env::current_dir().unwrap();
        sys.getattr("path")
            .unwrap()
            .call_method1("insert", (0, path.to_str().unwrap()))
            .unwrap();

        let _yolo = py.import("yolov12.yolo").unwrap();
    });
}

pub fn yolo_predict(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    Python::with_gil(|py| {
        let yolo = py.import("yolov12.yolo").unwrap();
        let predict = yolo.getattr("predict").unwrap();
        let output = predict.call1((bytes,)).unwrap();
        result = output.extract().unwrap();
    });
    result
}

#[unsafe(no_mangle)]
pub extern "C" fn dll_yolo_predict(data_ptr: *const c_uchar, data_len: usize) -> *const c_char {
    if data_ptr.is_null() || data_len == 0 {
        return std::ptr::null();
    }

    // 安全地從指標轉 Vec<u8>
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len as usize).to_vec() };

    let result_str = yolo_predict(bytes);

    // 回傳 CString，記憶體會由呼叫者釋放
    match CString::new(result_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null(), // 包含 null byte 時轉換會失敗
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_c_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // 從指標重建 CString 並 drop，讓 Rust 釋放記憶體
        let _ = CString::from_raw(s);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict() {
        init_python_interpreter();
        let bytes = fs::read("yolov12/cat.jpg").unwrap();
        let _result = yolo_predict(bytes.clone());
        let result = yolo_predict(bytes);

        println!("Prediction result: {}", result);
    }
}
