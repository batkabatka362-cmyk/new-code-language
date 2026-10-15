import sys
import time
import urllib.request
import json

sys.path.insert(0, "python")
import cron

print("[1] Launching CRON OpenAI-Compatible REST Server on port 8089...")
cron.serve(port=8089, host="127.0.0.1")
time.sleep(0.5)

print("[2] Querying GET /health...")
health = json.loads(urllib.request.urlopen("http://127.0.0.1:8089/health").read().decode("utf-8"))
print("    HEALTH RESPONSE:", health)
assert health["status"] == "healthy"
assert health["cores"] == 256

print("[3] Querying GET /v1/models...")
models = json.loads(urllib.request.urlopen("http://127.0.0.1:8089/v1/models").read().decode("utf-8"))
model_ids = [m["id"] for m in models["data"]]
print("    MODELS AVAILABLE:", model_ids)
assert "cron-r1-7b-ternary" in model_ids

print("[4] Querying POST /v1/chat/completions (Standard JSON)...")
req_body = json.dumps({
    "model": "cron-r1-7b-ternary",
    "messages": [{"role": "user", "content": "Synthesize 4D-Torus attention kernel"}]
}).encode("utf-8")
req = urllib.request.Request("http://127.0.0.1:8089/v1/chat/completions", data=req_body, headers={"Content-Type": "application/json"})
chat_res = json.loads(urllib.request.urlopen(req).read().decode("utf-8"))
content = chat_res["choices"][0]["message"]["content"]
print("    COMPLETION RECEIVED (Length: {} chars):".format(len(content)))
print("    PREVIEW:\n" + content[:150] + "...")

print("[5] Querying POST /v1/chat/completions (SSE Streaming)...")
stream_body = json.dumps({
    "model": "cron-r1-7b-ternary",
    "messages": [{"role": "user", "content": "Hello CRON Silicon"}],
    "stream": True
}).encode("utf-8")
stream_req = urllib.request.Request("http://127.0.0.1:8089/v1/chat/completions", data=stream_body, headers={"Content-Type": "application/json"})
resp = urllib.request.urlopen(stream_req)
chunks = []
for line in resp:
    decoded = line.decode("utf-8").strip()
    if decoded.startswith("data: ") and decoded != "data: [DONE]":
        payload = json.loads(decoded[6:])
        delta = payload["choices"][0]["delta"].get("content", "")
        chunks.append(delta)

streamed_text = "".join(chunks)
print("    STREAMED CHUNKS COUNT: {}".format(len(chunks)))
print("    STREAMED TEXT LENGTH: {} chars".format(len(streamed_text)))
assert len(chunks) > 5
assert len(streamed_text) > 0

print("\n[SUCCESS] ALL OPENAI REST SERVER ENDPOINTS FULLY VERIFIED!")
