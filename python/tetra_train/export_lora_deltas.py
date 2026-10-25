#!/usr/bin/env python3
"""
Export learned LoRA delta matrices from tetra_70b_cl_adapter.pt into raw FP32 binary format
for ultra-fast C++ OpenMP fusion into the 9.30 GB 70B foundation binary.
"""
import os
import sys
import torch

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

def export_deltas():
    adapter_path = "tetra_70b_cl_adapter.pt"
    out_path = "tetra_lora_deltas.bin"

    print(f"[*] Адаптер уншиж байна: {adapter_path}")
    state = torch.load(adapter_path, map_location="cpu")
    scaling = 32.0 / 16.0 # alpha / rank

    deltas = []
    # 8 layers
    for l in range(8):
        # Q LoRA: (1024, 16) @ (16, 1024) -> (1024, 1024)
        q_A = state[f"layers.{l}.q_lora.lora_A"]
        q_B = state[f"layers.{l}.q_lora.lora_B"]
        delta_q = (q_B @ q_A) * scaling
        deltas.append(delta_q.flatten())

        # V LoRA: (128, 16) @ (16, 1024) -> (128, 1024)
        v_A = state[f"layers.{l}.v_lora.lora_A"]
        v_B = state[f"layers.{l}.v_lora.lora_B"]
        delta_v = (v_B @ v_A) * scaling
        deltas.append(delta_v.flatten())

        # MLP Gate: (2048, 16) @ (16, 1024) -> (2048, 1024)
        g_A = state[f"layers.{l}.mlp_gate.lora_A"]
        g_B = state[f"layers.{l}.mlp_gate.lora_B"]
        delta_g = (g_B @ g_A) * scaling
        deltas.append(delta_g.flatten())

        # MLP Down: (1024, 16) @ (16, 2048) -> (1024, 2048)
        d_A = state[f"layers.{l}.mlp_down.lora_A"]
        d_B = state[f"layers.{l}.mlp_down.lora_B"]
        delta_d = (d_B @ d_A) * scaling
        deltas.append(delta_d.flatten())

    all_deltas = torch.cat(deltas).to(torch.float32)
    print(f"[+] Нийт дельта тооцоологдсон жингийн тоо: {all_deltas.numel():,} ({all_deltas.element_size() * all_deltas.numel() / (1024*1024):.2f} MB)")
    
    with open(out_path, "wb") as f:
        f.write(all_deltas.numpy().tobytes())
    print(f"[+] Экспортлогдлоо: {out_path}")

if __name__ == "__main__":
    export_deltas()
