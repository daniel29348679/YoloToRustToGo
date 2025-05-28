# %%
import base64
import json
import requests


def predict_image(image_path, server_url="http://localhost:8080/predict"):
    # 讀取圖片為 bytes
    with open(image_path, "rb") as f:
        image_bytes = f.read()

    # base64 編碼
    encoded = base64.b64encode(image_bytes).decode("utf-8")

    # 建立 JSON payload
    payload = {"image_data": encoded}

    # 發送 POST 請求
    response = requests.post(server_url, json=payload)

    # 處理回應
    if response.status_code == 200:
        result = response.json().get("result")
        print("🔍 Prediction result:", result)
    else:
        print("❌ Error:", response.status_code, response.text)


if __name__ == "__main__":
    # 指定要測試的圖片
    predict_image("cat.jpg")

# %%
