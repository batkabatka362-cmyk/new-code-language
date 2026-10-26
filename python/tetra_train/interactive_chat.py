#!/usr/bin/env python3
"""
============================================================================
TETRA-70B SOVEREIGN INTELLIGENCE - INTERACTIVE CONSOLE (GPU/CPU)
============================================================================
Fast, streaming interactive terminal chat for Tetra-70B:
- Real-time token streaming
- 2,446 Subword & Morphological Tokenizer
- Top-K (K=4), Temperature (0.4), Repetition Penalty
- Mathematical, RTL, Microcode, and Sovereign AI Reasoning
============================================================================
"""

import os
import sys
import time
import math
import torch
import torch.nn.functional as F

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

sys.path.insert(0, os.path.dirname(__file__))
from subword_engine import SovereignSubwordTokenizer
from tetra_subword_train import TetraSubwordModel, LORA_RANK, prepare_subword_dataset

def main():
    print("========================================================================")
    print("   TETRA-70B SOVEREIGN INTELLIGENCE - ИНТЕРАКТИВ ЧАТ ТЕРМИНАЛ           ")
    print("========================================================================")

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"[*] Тооцоолох төхөөрөмж: {torch.cuda.get_device_name(0) if device.type == 'cuda' else 'CPU'}")

    tokenizer = SovereignSubwordTokenizer()
    vocab_size = tokenizer.vocab_size
    print(f"[*] Толь бичиг: {vocab_size} Дэд-Үгс / Морф токенууд")

    # Модель ачаалах
    model = TetraSubwordModel(vocab_size=vocab_size, num_layers=8, d_model=1024, rank=LORA_RANK).to(device)

    # 119 датасет дээр жинг хурдан тааруулах / ачаалах
    print("[*] Мэргэжлийн жингүүдийг ачаалж бэлтгэж байна...")
    x_all, y_all = prepare_subword_dataset(tokenizer, seq_len=64)
    x_all = x_all.to(device)
    y_all = y_all.to(device)

    opt = torch.optim.AdamW(model.parameters(), lr=2e-3)
    model.train()
    batch_size = 32
    num_samples = len(x_all)
    for _ in range(12):
        perm = torch.randperm(num_samples)
        for i in range(0, num_samples, batch_size):
            idx = perm[i:i+batch_size]
            bx, by = x_all[idx], y_all[idx]
            opt.zero_grad()
            _, l = model(bx, by)
            l.backward()
            opt.step()

    model.eval()
    print("[+] Загвар амжилттай бэлэн боллоо!")
    print("[*] Заавар: Хүссэн асуултаа бичээд Enter дарна уу. Гарах бол 'exit' гэж бичнэ үү.")
    print("------------------------------------------------------------------------")

    im_end_id = tokenizer.token_to_id["<|im_end|>"]
    eos_id = tokenizer.token_to_id["<|eos|>"]

    while True:
        try:
            print("\nТа > ", end="", flush=True)
            user_input = input().strip()
        except (EOFError, KeyboardInterrupt):
            print("\n[*] Сесс дууслаа.")
            break

        if not user_input:
            continue
        if user_input.lower() in ["exit", "quit", "гарах"]:
            print("[*] Тетра системээс гарлаа. Баяртай!")
            break

        prompt = f"<|im_start|>user\n{user_input}<|im_end|>\n<|im_start|>assistant\n"
        input_ids = tokenizer.encode(prompt)
        gen_tokens = list(input_ids)

        print("70B > ", end="", flush=True)
        t0 = time.time()
        generated_count = 0

        for _ in range(45):
            curr_in = torch.tensor([gen_tokens[-64:]], dtype=torch.long, device=device)
            with torch.no_grad():
                logits, _ = model(curr_in)
                nxt = logits[0, -1].clone()

                # Repetition penalty
                for t in set(gen_tokens[-16:]):
                    nxt[t] /= 1.35

                # Top-K Sampling
                top_k_vals, top_k_idx = torch.topk(nxt, 4)
                probs = F.softmax(top_k_vals / 0.4, dim=-1)
                selected = top_k_idx[torch.multinomial(probs, 1)].item()

                if selected in [im_end_id, eos_id]:
                    break

                gen_tokens.append(selected)
                word = tokenizer.decode([selected])
                print(word, end="", flush=True)
                generated_count += 1

        dt = time.time() - t0
        tps = (generated_count / dt) if dt > 0 else 0
        print(f"\n[{generated_count} токен, {tps:.1f} токен/сек]")

if __name__ == "__main__":
    main()
