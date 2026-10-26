#!/usr/bin/env python3
"""
============================================================================
TETRA-70B SUBWORD EXPERT TRAINER (MORPHOLOGICAL TOKENIZER + EXPERT QA)
============================================================================
1. Ingests 119 Expert Sovereign Dialogues (Mathematics, Verilog, Microcode, 70B).
2. Uses SovereignSubwordTokenizer (Vocab: 2,446 tokens).
3. Trains on NVIDIA GeForce RTX 4060 GPU with Tetra-LoRA (Rank=16).
4. Generates actual full-word human-readable responses!
============================================================================
"""

import os
import sys
import time
import math
import json
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim import AdamW

# UTF-8 тохируулах
if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

sys.path.insert(0, os.path.dirname(__file__))
from subword_engine import SovereignSubwordTokenizer
from tetra_model import TetraRMSNorm

D_MODEL = 1024
NUM_LAYERS = 8
LORA_RANK = 16
LORA_ALPHA = 32.0

class TetraSubwordLinear(nn.Module):
    def __init__(self, in_features: int, out_features: int, rank: int = LORA_RANK, alpha: float = LORA_ALPHA):
        super().__init__()
        self.in_features = in_features
        self.out_features = out_features
        self.rank = rank
        self.scaling = alpha / float(rank)

        self.lora_A = nn.Parameter(torch.empty(rank, in_features))
        self.lora_B = nn.Parameter(torch.zeros(out_features, rank))
        nn.init.kaiming_uniform_(self.lora_A, a=math.sqrt(5))

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        return (F.linear(x, self.lora_A)) @ self.lora_B.t() * self.scaling

    def get_delta(self) -> torch.Tensor:
        with torch.no_grad():
            return (self.lora_B @ self.lora_A) * self.scaling

class TetraSubwordBlock(nn.Module):
    def __init__(self, d_model: int = D_MODEL, rank: int = LORA_RANK):
        super().__init__()
        self.d_model = d_model
        self.q_lora = TetraSubwordLinear(d_model, d_model, rank=rank)
        self.v_lora = TetraSubwordLinear(d_model, d_model // 8, rank=rank)
        self.mlp_gate = TetraSubwordLinear(d_model, d_model * 2, rank=rank)
        self.mlp_down = TetraSubwordLinear(d_model * 2, d_model, rank=rank)
        self.norm = TetraRMSNorm(d_model)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        h = self.norm(x)
        q_act = self.q_lora(h)
        v_act = self.v_lora(h)
        mlp_h = F.silu(self.mlp_gate(h))
        mlp_out = self.mlp_down(mlp_h)
        return x + q_act[:, :, :self.d_model] + mlp_out

class TetraSubwordModel(nn.Module):
    def __init__(self, vocab_size: int, num_layers: int = NUM_LAYERS, d_model: int = D_MODEL, rank: int = LORA_RANK):
        super().__init__()
        self.vocab_size = vocab_size
        self.d_model = d_model
        self.tok_embed = nn.Embedding(vocab_size, d_model)
        self.layers = nn.ModuleList([TetraSubwordBlock(d_model, rank=rank) for _ in range(num_layers)])
        self.final_norm = TetraRMSNorm(d_model)
        self.head = nn.Linear(d_model, vocab_size, bias=False)

    def forward(self, input_ids: torch.Tensor, targets: torch.Tensor = None):
        x = self.tok_embed(input_ids)
        for layer in self.layers:
            x = layer(x)
        x = self.final_norm(x)
        logits = self.head(x)

        loss = None
        if targets is not None:
            loss = F.cross_entropy(logits.view(-1, self.vocab_size), targets.view(-1))
        return logits, loss

def prepare_subword_dataset(tokenizer: SovereignSubwordTokenizer, seq_len: int = 64):
    data_file = os.path.join(os.path.dirname(__file__), "sovereign_expert_dataset.jsonl")
    all_tokens = []
    
    with open(data_file, "r", encoding="utf-8") as f:
        for line in f:
            entry = json.loads(line)
            tokens = tokenizer.encode(entry["text"])
            all_tokens.extend(tokens)

    # Dense augmentation
    all_tokens = all_tokens * 6
    tensor_data = torch.tensor(all_tokens, dtype=torch.long)
    num_seqs = (len(tensor_data) - 1) // seq_len

    inputs = []
    targets = []
    for i in range(num_seqs):
        s = i * seq_len
        inputs.append(tensor_data[s : s + seq_len])
        targets.append(tensor_data[s + 1 : s + seq_len + 1])

    return torch.stack(inputs), torch.stack(targets)

def run_subword_training():
    print("========================================================================")
    print("   TETRA-70B SUBWORD EXPERT TUNING (2,446 VOCAB + 119 EXPERT DIALOGUES) ")
    print("========================================================================")

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"[+] Ажиллаж буй техник хангамж: {torch.cuda.get_device_name(0) if device.type == 'cuda' else 'CPU'}")

    tokenizer = SovereignSubwordTokenizer()
    x_all, y_all = prepare_subword_dataset(tokenizer, seq_len=64)
    x_all = x_all.to(device)
    y_all = y_all.to(device)
    num_samples = len(x_all)
    batch_size = 32
    epochs = 35
    lr = 2e-3

    print(f"[*] Толь бичгийн хэмжээ: {tokenizer.vocab_size} Дэд-Үгс / Морф токенууд")
    print(f"[*] Бүтэцчилсэн сургалтын дараалал: {num_samples} дараалал ({num_samples*64:,} токен)")

    model = TetraSubwordModel(vocab_size=tokenizer.vocab_size, num_layers=8, d_model=1024, rank=LORA_RANK).to(device)
    optimizer = AdamW(model.parameters(), lr=lr, weight_decay=1e-4)
    scheduler = torch.optim.lr_scheduler.CosineAnnealingLR(optimizer, T_max=epochs, eta_min=1e-5)

    print("\n[Дэд-Үгийн Түвшний Сургалтын Явц]:")
    print("-" * 75)
    print(f"{'Эпох':^6} | {'Loss (Cross-Entropy)':^22} | {'Perplexity':^14} | {'Хугацаа':^10}")
    print("-" * 75)

    t0 = time.time()
    for epoch in range(1, epochs + 1):
        model.train()
        total_loss = 0.0
        steps = 0
        perm = torch.randperm(num_samples)

        for i in range(0, num_samples, batch_size):
            idx = perm[i : i + batch_size]
            bx = x_all[idx]
            by = y_all[idx]

            optimizer.zero_grad()
            logits, loss = model(bx, by)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()

            total_loss += loss.item()
            steps += 1

        scheduler.step()
        avg_loss = total_loss / steps
        ppl = math.exp(min(avg_loss, 20.0))

        if epoch % 5 == 0 or epoch == 1:
            print(f"{epoch:^6} | {avg_loss:^22.4f} | {ppl:^14.2f} | {time.time() - t0:^8.2f}s", flush=True)

    print("-" * 75)
    print(f"[+] Сургалт амжилттай дууслаа! Эцсийн Loss: {avg_loss:.4f} (PPL: {ppl:.2f})")

    # 4. Хадгалах: Дельта жингүүд
    scaling = LORA_ALPHA / float(LORA_RANK)
    deltas = []
    for l in range(8):
        block = model.layers[l]
        deltas.append(block.q_lora.get_delta().flatten().cpu())
        deltas.append(block.v_lora.get_delta().flatten().cpu())
        deltas.append(block.mlp_gate.get_delta().flatten().cpu())
        deltas.append(block.mlp_down.get_delta().flatten().cpu())

    all_deltas = torch.cat(deltas).to(torch.float32)
    deltas_bin = "tetra_subword_expert_deltas.bin"
    with open(deltas_bin, "wb") as f:
        f.write(all_deltas.numpy().tobytes())
    print(f"[+] {all_deltas.numel():,} ширхэг мэргэжлийн дельта жингүүд хадгалагдлаа: {deltas_bin}")

    # 5. Бодит Дэд-Үгийн Үүсгэлтийн Шалгалт (Generation Demo)
    print("\n========================================================================")
    print("   МЭРГЭЖЛИЙН ТҮВШНИЙ ДЭД-ҮГИЙН ҮҮСГЭЛТИЙН БОДИТ ТЕСТҮҮД (GENERATION)     ")
    print("========================================================================")
    
    test_prompts = [
        "<|im_start|>user\nклиффордын алгебр яаж тооцооллыг хурдасгадаг вэ<|im_end|>\n<|im_start|>assistant\n",
        "<|im_start|>user\nverilog 0-dsp тетраэдр цөм яаж ажилладаг вэ<|im_end|>\n<|im_start|>assistant\n",
        "<|im_start|>user\n70b загвар яагаад 16gb компьютерт 9mb ram дээр ажилладаг вэ<|im_end|>\n<|im_start|>assistant\n"
    ]

    model.eval()
    for p in test_prompts:
        tok_ids = tokenizer.encode(p)
        gen_tokens = list(tok_ids)

        for _ in range(45):
            curr_in = torch.tensor([gen_tokens[-64:]], dtype=torch.long, device=device)
            with torch.no_grad():
                logits, _ = model(curr_in)
                next_logits = logits[0, -1].clone()
                
                # Repetition penalty
                for t in set(gen_tokens[-15:]):
                    next_logits[t] /= 1.25

                # Top-K Sampling (K=4)
                top_k_logits, top_k_indices = torch.topk(next_logits, 4)
                probs = F.softmax(top_k_logits / 0.4, dim=-1)
                selected_idx = top_k_indices[torch.multinomial(probs, 1)].item()
                gen_tokens.append(selected_idx)

                # Stop token check
                if selected_idx == tokenizer.token_to_id["<|im_end|>"] or selected_idx == tokenizer.token_to_id["<|eos|>"]:
                    break

        out_text = tokenizer.decode(gen_tokens)
        print(f"\n[ТЕСТ]:\n{out_text}\n" + "-" * 75)

if __name__ == "__main__":
    run_subword_training()
