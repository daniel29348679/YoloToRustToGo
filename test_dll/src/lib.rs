mod prelude;
use prelude::*;

#[ctor::ctor]
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

#[unsafe(no_mangle)]
pub extern "C" fn call_yolo_predict(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    Python::with_gil(|py| {
        let yolo = py.import("yolov12.yolo").unwrap();
        let predict = yolo.getattr("predict").unwrap();
        let output = predict.call1((bytes,)).unwrap();
        result = output.extract().unwrap();
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict() {
        init_python_interpreter();
        let bytes = fs::read("yolov12/cat.jpg").unwrap();
        let result = call_yolo_predict(bytes.clone());
        let result = call_yolo_predict(bytes);

        println!("Prediction result: {}", result);
    }
}
