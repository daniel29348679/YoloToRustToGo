package main

/*
#cgo LDFLAGS: -L. -lrust_dll
// 指定 Rust build 出來的 DLL（librust_dll.so / rust_dll.dll / librust_dll.dylib）
#include <rust_dll.h>
*/
import "C"
import (
	"encoding/base64"
	"fmt"
	"unsafe"

	"os"

	"github.com/gin-gonic/gin"
)

// 接收 JSON 結構
type PredictRequest struct {
	Model     string `json:"model"`      // 模型名稱，例如 "yolov12x.pt"
	ImageData string `json:"image_data"` // base64 encoded
}

type PredictResponse struct {
	Result string `json:"result"`
}

type LoadModelRequest struct {
	Model string `json:"model"` // 模型名稱，例如 "yolov12x.pt"
}

type LoadModelResponse struct {
	Result int32 `json:"result"` // 成功或失敗的訊息Result
}

type GetModelNamesResponse struct {
	Result string `json:"result"`
}

func main() {
	router := gin.Default()

	router.POST("/predict", func(c *gin.Context) {
		// json = {"model": "yolov12x.pt", "image_data": "base64_encoded_string"}
		var req PredictRequest
		if err := c.ShouldBindJSON(&req); err != nil {
			fmt.Fprintf(os.Stderr, "[Go] Error binding JSON: %v\n", err)
			c.JSON(400, gin.H{"error": "invalid JSON"})
			return
		}

		model_ptr := C.CString(req.Model)
		defer C.free(unsafe.Pointer(model_ptr))

		// decode base64 string into []byte
		data, err := base64.StdEncoding.DecodeString(req.ImageData)
		if err != nil {
			fmt.Fprintf(os.Stderr, "[Go] Error decoding base64: %v\n", err)
			c.JSON(400, gin.H{"error": "invalid base64 image_data"})
			return
		}

		// call Rust DLL
		resultPtr := C.Dll_yolo_predict(model_ptr, (*C.uchar)(unsafe.Pointer(&data[0])), C.size_t(len(data)))
		if resultPtr == nil {
			c.JSON(500, gin.H{"error": "rust DLL returned null"})
			return
		}
		defer C.Free_c_string((*C.char)(unsafe.Pointer(resultPtr)))

		// convert C string to Go string
		result := C.GoString(resultPtr)

		// return result
		c.JSON(200, PredictResponse{Result: result})
	})

	router.POST("/load_model", func(c *gin.Context) {
		// json = {"model": "yolov12x.pt"}
		var req LoadModelRequest
		if err := c.ShouldBindJSON(&req); err != nil {
			fmt.Fprintf(os.Stderr, "[Go] Error binding JSON: %v\n", err)
			c.JSON(400, gin.H{"error": "invalid JSON"})
			return
		}

		model_ptr := C.CString(req.Model)
		defer C.free(unsafe.Pointer(model_ptr))

		result := C.Dll_load_model(model_ptr)

		c.JSON(200, LoadModelResponse{Result: int32(result)})
	})

	router.GET("/get_model_names", func(c *gin.Context) {
		// call Rust DLL to get model names
		resultPtr := C.Dll_get_model_names()
		if resultPtr == nil {
			c.JSON(500, gin.H{"error": "rust DLL returned null"})
			return
		}
		defer C.Free_c_string((*C.char)(unsafe.Pointer(resultPtr)))

		// convert C string to Go string
		result := C.GoString(resultPtr)

		// return result
		c.JSON(200, GetModelNamesResponse{Result: result})
	})

	router.Run(":8080")
}
