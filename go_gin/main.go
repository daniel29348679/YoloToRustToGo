package main

/*
#cgo LDFLAGS: -L. -ltest_dll
// 指定 Rust build 出來的 DLL（libtest_dll.so / test_dll.dll / libtest_dll.dylib）
#include <stdlib.h>

extern const char* dll_yolo_predict(const unsigned char* data, size_t len);
extern void free_c_string(char* s);
*/
import "C"
import (
	"encoding/base64"
	"unsafe"

	"github.com/gin-gonic/gin"
)

// 接收 JSON 結構
type PredictRequest struct {
	ImageData string `json:"image_data"` // base64 encoded
}

type PredictResponse struct {
	Result string `json:"result"`
}

func main() {
	router := gin.Default()

	router.POST("/predict", func(c *gin.Context) {
		var req PredictRequest
		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(400, gin.H{"error": "invalid JSON"})
			return
		}

		// decode base64 string into []byte
		data, err := base64.StdEncoding.DecodeString(req.ImageData)
		if err != nil {
			c.JSON(400, gin.H{"error": "invalid base64 image_data"})
			return
		}

		// call Rust DLL
		resultPtr := C.dll_yolo_predict((*C.uchar)(unsafe.Pointer(&data[0])), C.size_t(len(data)))
		if resultPtr == nil {
			c.JSON(500, gin.H{"error": "rust DLL returned null"})
			return
		}
		defer C.free_c_string((*C.char)(unsafe.Pointer(resultPtr)))

		// convert C string to Go string
		result := C.GoString(resultPtr)

		// return result
		c.JSON(200, PredictResponse{Result: result})
	})

	router.Run(":8080")
}
