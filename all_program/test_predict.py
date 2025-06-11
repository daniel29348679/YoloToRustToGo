# %%
import base64
import requests

base_url = "http://localhost:8080"  # 預設伺服器 URL


def predict_image(image_path, model_name="yolov12x.pt"):
    global base_url
    server_url = f"{base_url}/predict"

    with open(image_path, "rb") as f:
        image_bytes = f.read()
    encoded = base64.b64encode(image_bytes).decode("utf-8")

    payload = {
        "model": model_name,
        "image_data": encoded,
    }

    response = requests.post(server_url, json=payload)
    if response.status_code == 200:
        result = response.json().get("result")
        print("🔍 Prediction result:", result)
    else:
        print("❌ Predict Error:", response.status_code, response.text)


def load_model(model_name="yolov12x.pt"):
    global base_url
    server_url = f"{base_url}/load_model"

    payload = {
        "model": model_name,
    }
    response = requests.post(server_url, json=payload)
    if response.status_code == 200:
        result = response.json().get("result")
        print(f"⚙️ Load model '{model_name}' result:", result)
    else:
        print("❌ Load Model Error:", response.status_code, response.text)


def get_model_names():
    global base_url
    server_url = f"{base_url}/get_model_names"
    response = requests.get(server_url)
    if response.status_code == 200:
        result = response.json().get("result")
        print("📋 Available models:", result)
    else:
        print("❌ Get Model Names Error:", response.status_code, response.text)


if __name__ == "__main__":
    # 範例使用
    load_model("yolov12x.pt")
    predict_image("cat.jpg", model_name="yolov12x.pt")
    get_model_names()

# %%
