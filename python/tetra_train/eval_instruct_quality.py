#!/usr/bin/env python3
"""
Scientific Evaluation & Generation Verification Script
Measures:
1. Top-1 Accuracy on Validation Set (%)
2. Perplexity (PPL)
3. Actual text generation on evaluation prompts
"""
import os
import sys
import math
import torch
import torch.nn.functional as F

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

sys.path.insert(0, os.path.dirname(__file__))
from tetra_model import DPSTA_CHAR_BASE, DPSTA_CHAR_COUNT
from tetra_70b_instruct_tune import Tetra70BInstructModel, load_instruct_dataset, LORA_RANK

def evaluate():
    print("========================================================================")
    print("   САЙЖРУУЛАЛТЫГ ШИНЖЛЭХ УХААНЫ ҮНДЭСТЭЙГЭЭР ХЭМЖИХ ШАЛГУУР (EVALUATION)  ")
    print("========================================================================")
    
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    x_all, y_all = load_instruct_dataset()
    x_all = x_all.to(device)
    y_all = y_all.to(device)
    
    # 80/20 train/val split
    n = len(x_all)
    val_n = max(1, int(n * 0.2))
    val_x = x_all[-val_n:]
    val_y = y_all[-val_n:]

    model = Tetra70BInstructModel(num_layers=8, d_model=1024, rank=LORA_RANK).to(device)
    
    # Сургаагүй үеийн (Untrained Baseline) хэмжилт
    model.eval()
    batch_size = 32
    with torch.no_grad():
        all_preds_base = []
        all_targets_base = []
        base_losses = []
        for i in range(0, len(val_x), batch_size):
            bx = val_x[i:i+batch_size]
            by = val_y[i:i+batch_size]
            l, loss = model(bx, by)
            base_losses.append(loss.item())
            all_preds_base.append(l.argmax(dim=-1))
            all_targets_base.append(torch.clamp(by - DPSTA_CHAR_BASE, 0, DPSTA_CHAR_COUNT - 1))
        
        preds_base = torch.cat(all_preds_base)
        targets = torch.cat(all_targets_base)
        loss_base_val = sum(base_losses) / len(base_losses)
        acc_base = (preds_base == targets).float().mean().item() * 100.0
        ppl_base = math.exp(loss_base_val)

    print(f"\n1. СУРГАЛТЫН ӨМНӨХ БОДИТ БАЙДАЛ (Baseline):")
    print(f"   • Cross-Entropy Loss : {loss_base_val:.4f}")
    print(f"   • Perplexity (Эргэлзээ): {ppl_base:.2f}")
    print(f"   • Дараагийн үсгийг зөв таах магадлал (Top-1 Accuracy): {acc_base:.2f}% (Санамсаргүй шахуу)")

    # 2. Сургалтын дараах хэмжилт
    print(f"\n2. СУРГАЛТЫН ДАРААХ БОДИТ БАЙДАЛ (Trained):")
    optimizer = torch.optim.AdamW(model.parameters(), lr=2e-3)
    train_x = x_all[:-val_n]
    train_y = y_all[:-val_n]
    batch_size = 32

    for epoch in range(15):
        model.train()
        perm = torch.randperm(len(train_x))
        for i in range(0, len(train_x), batch_size):
            bx = train_x[perm[i:i+batch_size]]
            by = train_y[perm[i:i+batch_size]]
            optimizer.zero_grad()
            _, loss = model(bx, by)
            loss.backward()
            optimizer.step()

    model.eval()
    with torch.no_grad():
        all_preds = []
        all_targets = []
        val_losses = []
        for i in range(0, len(val_x), batch_size):
            bx = val_x[i:i+batch_size]
            by = val_y[i:i+batch_size]
            l, loss = model(bx, by)
            val_losses.append(loss.item())
            all_preds.append(l.argmax(dim=-1))
            all_targets.append(torch.clamp(by - DPSTA_CHAR_BASE, 0, DPSTA_CHAR_COUNT - 1))

        preds_trained = torch.cat(all_preds)
        targets_trained = torch.cat(all_targets)
        loss_trained = sum(val_losses) / len(val_losses)
        acc_trained = (preds_trained == targets_trained).float().mean().item() * 100.0
        ppl_trained = math.exp(loss_trained)

    print(f"   • Cross-Entropy Loss : {loss_trained:.4f} (Буурсан: {loss_base_val - loss_trained:.4f})")
    print(f"   • Perplexity (Эргэлзээ): {ppl_trained:.2f} (Буурсан: {ppl_base - ppl_trained:.2f})")
    print(f"   • Дараагийн үсгийг зөв таах магадлал (Top-1 Accuracy): {acc_trained:.2f}% (Өссөн: +{acc_trained - acc_base:.2f}%)")

    print(f"\n3. БОДИТ ТЕКСТ ҮҮСГЭЛТИЙН ХАРЬЦУУЛАЛТ:")
    prompt = "<|im_start|>user\nsain baina uu<|im_end|>\n<|im_start|>assistant\n"
    print(f"   Промпт: \"{prompt}\"")
    
    input_ids = torch.tensor([ord(c) for c in prompt if 32 <= ord(c) <= 126], dtype=torch.long, device=device).unsqueeze(0)
    generated = list(input_ids[0].cpu().numpy())
    
    for _ in range(60):
        curr_in = torch.tensor([generated[-128:]], dtype=torch.long, device=device)
        with torch.no_grad():
            l, _ = model(curr_in)
            next_logits = l[0, -1].clone()
            
            # Repetition penalty (давталтыг хасах)
            for token in set(generated[-20:]):
                t_idx = token - DPSTA_CHAR_BASE
                if 0 <= t_idx < DPSTA_CHAR_COUNT:
                    next_logits[t_idx] /= 1.3
            
            # Top-K Sampling (K=5)
            top_k_logits, top_k_indices = torch.topk(next_logits, 5)
            probs = F.softmax(top_k_logits / 0.5, dim=-1)
            selected_idx = top_k_indices[torch.multinomial(probs, 1)].item()
            next_token = selected_idx + DPSTA_CHAR_BASE
            
            generated.append(next_token)
            if next_token == ord('\n') and len(generated) > len(prompt) + 15:
                break
    
    out_str = "".join([chr(c) for c in generated])
    print(f"   Гаралт: \n{out_str}")
    print("========================================================================")

if __name__ == "__main__":
    evaluate()
