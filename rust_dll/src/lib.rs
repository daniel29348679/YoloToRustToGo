mod prelude;
use prelude::*;

#[ctor]
fn init() {
    Init_python_interpreter()
}

#[unsafe(no_mangle)]
pub extern "C" fn Init_python_interpreter() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();
        let path = std::env::current_dir().unwrap();
        sys.getattr("path")
            .unwrap()
            .call_method1("insert", (0, path.to_str().unwrap()))
            .unwrap();

        match py.import("yolov12.yolo") {
            Ok(_) => {
                println!("[Rust] Successfully imported yolov12.yolo");
            }
            Err(e) => {
                eprintln!("[Rust] Error importing yolov12.yolo: {}", e);
            }
        }
    });
}

pub fn yolo_predict(model: &String, bytes: Vec<u8>) -> String {
    let mut result = String::new();
    Python::with_gil(|py| {
        let yolo = py.import("yolov12.yolo").unwrap();
        let predict = yolo.getattr("Predict").unwrap();
        let output = predict.call1((model, bytes)).unwrap();
        result = output.extract().unwrap();
    });
    result
}

#[unsafe(no_mangle)]
pub extern "C" fn Dll_yolo_predict(
    model_str: *const c_char,
    data_ptr: *const c_uchar,
    data_len: usize,
) -> *const c_char {
    if model_str.is_null() {
        eprintln!("[Rust] Model string pointer is null or length is zero.");
        return std::ptr::null();
    }
    if data_ptr.is_null() || data_len == 0 {
        eprintln!("[Rust] Data pointer is null or length is zero.");
        return std::ptr::null();
    }

    // 從指標轉換 model_str 為 Rust 字符串
    let model_cstr = unsafe { CStr::from_ptr(model_str) };
    if model_cstr.to_bytes().is_empty() {
        eprintln!("[Rust] Model string is empty.");
        return std::ptr::null();
    }
    // 嘗試將 CStr 轉換為 Rust 字符串
    let model_str = match model_cstr.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            eprintln!("[Rust] Failed to convert model string from CStr.");
            return std::ptr::null();
        }
    };
    // 安全地從指標轉 Vec<u8>
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len as usize).to_vec() };

    let result_str = yolo_predict(&model_str, bytes);
    if result_str.is_empty() {
        eprintln!("[Rust] Prediction result is empty.");
        return std::ptr::null();
    }

    // 回傳 CString，記憶體會由呼叫者釋放
    match CString::new(result_str) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null(), // 包含 null byte 時轉換會失敗
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Free_c_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // 從指標重建 CString 並 drop，讓 Rust 釋放記憶體
        let _ = CString::from_raw(s);
    }
}

pub fn load_model(model_str: &String) -> i32 {
    if model_str.is_empty() {
        eprintln!("[Rust] Model string is empty.");
        return -1;
    }

    Python::with_gil(|py| {
        let yolo = py.import("yolov12.yolo").unwrap();
        let load_model = yolo.getattr("Load_model").unwrap();
        let result = load_model.call1((model_str,));
        match result {
            Ok(res) => res.extract::<i32>().unwrap_or(-1),
            Err(e) => {
                eprintln!("[Rust] Error loading model: {}", e);
                -1
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn Dll_load_model(model_str: *const c_char) -> i32 {
    if model_str.is_null() {
        eprintln!("[Rust] Model string pointer is null.");
        return -1;
    }

    // 從指標轉換 model_str 為 Rust 字符串
    let model_cstr = unsafe { CStr::from_ptr(model_str) };
    if model_cstr.to_bytes().is_empty() {
        eprintln!("[Rust] Model string is empty.");
        return -1;
    }

    // 嘗試將 CStr 轉換為 Rust 字符串
    let model_str = match model_cstr.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            eprintln!("[Rust] Failed to convert model string from CStr.");
            return -1;
        }
    };

    load_model(&model_str)
}

pub fn get_model_names() -> String {
    Python::with_gil(|py| {
        let yolo = py.import("yolov12.yolo").unwrap();
        let get_model_names = yolo.getattr("Get_model_names").unwrap();
        let output = get_model_names.call0().unwrap();
        output.extract().unwrap()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn Dll_get_model_names() -> *const c_char {
    let result = get_model_names();
    if result.is_empty() {
        eprintln!("[Rust] No model names found.");
        return std::ptr::null();
    }

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => {
            eprintln!("[Rust] Failed to convert model names to CString.");
            std::ptr::null()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict() {
        Init_python_interpreter();
        assert_eq!(load_model(&"yolov12x.pt".to_string()), 0);
        let bytes = fs::read("yolov12/cat.jpg").unwrap();
        let result = yolo_predict(&"yolov12x.pt".to_string(), bytes);

        println!("Prediction result: {}", result);
        assert_eq!(get_model_names(), "[\'yolov12x.pt\']");
    }
}
