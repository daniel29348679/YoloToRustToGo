# %%
from ultralytics import YOLO
from PIL import Image
import io
import os
from json import dumps
from sys import stderr

models = {}


def Load_model(model_name) -> int:
    try:
        current_dir = os.path.dirname(os.path.abspath(__file__))
        model_path = os.path.join(current_dir, model_name)
        if model_name not in models:
            models[model_name] = YOLO(model_path)
    except Exception as e:
        return 1
    print(f"[Python] Model {model_name} loaded successfully.")
    return 0


def Predict(model_name, input_bytes) -> str:
    try:
        image = Image.open(io.BytesIO(input_bytes))
        if model_name not in models:
            return f"Model not loaded."
        results = models[model_name].predict(image)
    except Exception as e:
        print(f"[Python] Error processing image: {e}", file=stderr)
        return f"[Python] Error processing image: {e}"

    return boxes_to_json(results[0].boxes)


def boxes_to_json(box) -> str:
    """
    ultralytics.engine.results.Boxes object with attributes:

    cls: tensor([15.], device='cuda:0')
    conf: tensor([0.9500], device='cuda:0')
    data: tensor([[1.5704e+01, 8.8182e+01, 1.4377e+03, 1.0800e+03, 9.5003e-01, 1.5000e+01]], device='cuda:0')
    id: None
    is_track: False
    orig_shape: (1080, 1440)
    shape: torch.Size([1, 6])
    xywh: tensor([[ 726.7056,  584.0911, 1422.0031,  991.8177]], device='cuda:0')
    xywhn: tensor([[0.5047, 0.5408, 0.9875, 0.9183]], device='cuda:0')
    xyxy: tensor([[  15.7041,   88.1823, 1437.7072, 1080.0000]], device='cuda:0')
    xyxyn: tensor([[0.0109, 0.0817, 0.9984, 1.0000]], device='cuda:0')
    """

    box_dict = {
        "cls": box.cls.item(),
        "conf": box.conf.item(),
        "data": box.data.cpu().numpy().tolist(),
        "id": box.id,
        "is_track": box.is_track,
        "orig_shape": box.orig_shape,
        "shape": box.shape,
        "xywh": box.xywh.cpu().numpy().tolist(),
        "xywhn": box.xywhn.cpu().numpy().tolist(),
        "xyxy": box.xyxy.cpu().numpy().tolist(),
        "xyxyn": box.xyxyn.cpu().numpy().tolist(),
    }
    return dumps(box_dict)


def Get_model_names() -> str:
    return str(list(models.keys()))


print("[Python] Model loaded successfully!")

# %%
if __name__ == "__main__":
    with open("cat.jpg", "rb") as f:
        input_bytes = f.read()
    model_name = "yolov12x.pt"
    Load_model(model_name)
    result = Predict(model_name, input_bytes)
    print(result)
    print(Get_model_names())

# %%
