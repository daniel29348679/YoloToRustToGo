# %%
from ultralytics import YOLO
from PIL import Image
import io


try:
    model = YOLO("yolov12/yolov12x.pt")
except Exception as e:
    model = YOLO("yolov12x.pt")


def predict(input_bytes):
    image = Image.open(io.BytesIO(input_bytes))

    results = model.predict(image)
    return str(results[0].boxes)


print("Model loaded successfully. From python")

# %%
if __name__ == "__main__":
    with open("cat.jpg", "rb") as f:
        input_bytes = f.read()
    result = predict(input_bytes)
    print(result)

# %%
