#!/usr/bin/env python3
"""
1.03-Бит Тетраэдр Симплекс Загварын Сүүдэрт Градиентийн Сургалтын Хөдөлгүүр.
- Shadow Manifold STE (Straight-Through Estimator)
- Tetrahedral Repulsion Field Loss (109.47° Dihedral bond protection)
- GPU (CUDA) хурдатгалтай
- Сургалтын дараа шууд 1.03-бит binary (.bin) болгон экспортлоно.
"""
import sys
import os
import time
import math
import torch
import torch.nn as nn
from torch.optim import AdamW

# UTF-8 тохируулах
if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

# Модуль зам
sys.path.insert(0, os.path.dirname(__file__))
try:
    from tetra_model import TetraLM, DPSTA_CHAR_BASE, DPSTA_CHAR_COUNT
    from export_bin import export_model_to_bin
except ImportError:
    from .tetra_model import TetraLM, DPSTA_CHAR_BASE, DPSTA_CHAR_COUNT
    from .export_bin import export_model_to_bin

# Сургалтын өгөгдөл (Монгол хэл ба AGI суурь текст)
TRAINING_TEXT = """
Монгол Улс бол хөх тэнгэрийн орон билээ.
Бид 1.03-бит тетраэдр симплекс архитектур ба DPSTA фазын долгионыг ашиглан 
хөвөгч таслалгүй, зөвхөн логик бит үйлдлээр их наяд параметртэй оюун ухааныг бүтээж байна.
Тетраэдр геометр нь нүүрстөрөгчийн алмаз шиг хамгийн тогтвортой 109.47 градусын өнцөгтэй.
Уламжлалт хүнд хөвөгч тооноос татгалзаж, дискрет фазын долгионоор тооцоолно.
Энэ бол цэвэр монгол сэтгэлгээ, шинжлэх ухааны цоо шинэ эрин үе мөн.
Mongol AGI architecture uses tetrahedron simplex geometry and discrete phase waves.
Zero FLOPs, pure bitwise XNOR and POPCOUNT operations enable trillion parameter models on ordinary PCs.
Резонансын KV кэш нь санах ойг 32 дахин шахаж, 1 сая токеныг 8GB санах ойд багтаана.
Эрдэм мэдлэг бол хүч чадал, хиймэл оюун бол бидний ирээдүй мөн.
"""

def load_expanded_corpus() -> str:
    """Төслийн баримт бичгүүд болон монгол текстийг нэгтгэн өргөтгөсөн датасет бэлтгэх"""
    corpus = [TRAINING_TEXT]
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
    foundations_dir = os.path.join(base_dir, "docs", "foundations")
    if os.path.exists(foundations_dir):
        for fname in ["full_notes_184_224.txt", "matrix4_full_transcript.txt", "notes_121_183.txt"]:
            fpath = os.path.join(foundations_dir, fname)
            if os.path.exists(fpath):
                try:
                    with open(fpath, "r", encoding="utf-8", errors="ignore") as f:
                        corpus.append(f.read())
                except Exception:
                    pass
    readme_path = os.path.join(base_dir, "README.md")
    if os.path.exists(readme_path):
        try:
            with open(readme_path, "r", encoding="utf-8", errors="ignore") as f:
                corpus.append(f.read())
        except Exception:
            pass
    return "\n".join(corpus)

def prepare_dataset(text: str, seq_len: int = 128):
    """ASCII кодоор дараалал үүсгэх"""
    # 32..126 тэмдэгтүүдийг шүүх
    ascii_bytes = [ord(c) for c in text if 32 <= ord(c) <= 126]
    
    # Хэрэв текст богино бол давтах
    while len(ascii_bytes) < 32768:
        ascii_bytes.extend(ascii_bytes[:4096])

    tensor_data = torch.tensor(ascii_bytes, dtype=torch.long)
    num_seqs = (len(tensor_data) - 1) // seq_len

    inputs = []
    targets = []
    for i in range(num_seqs):
        start = i * seq_len
        inputs.append(tensor_data[start : start + seq_len])
        targets.append(tensor_data[start + 1 : start + seq_len + 1])

    return torch.stack(inputs), torch.stack(targets)

def train_tetra_model(scaled: bool = True):
    print("=" * 80)
    print("  1.03-БИТ ТЕТРАЭДР СИМПЛЕКС СҮҮДЭРТ ГРАДИЕНТИЙН СУРГАЛТ (SCALED TRAINING)  ")
    print("=" * 80)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    if device.type == "cuda":
        print(f"[+] CUDA идэвхтэй: {torch.cuda.get_device_name(0)}")
        print(f"    Боломжит VRAM: {torch.cuda.get_device_properties(0).total_memory / (1024**3):.2f} GB")
    else:
        print("[*] CPU горимд ажиллаж байна.")

    # 1. Загварыг байгуулах (Өргөтгөсөн 16.8M параметр)
    d_model = 512 if scaled else 256
    num_layers = 8 if scaled else 4
    n_heads = 8 if scaled else 4
    seq_len = 128
    batch_size = 32 if device.type == "cuda" else 16
    epochs = 20
    lr = 1.5e-3

    model = TetraLM(d_model=d_model, num_layers=num_layers, n_heads=n_heads).to(device)
    total_params = sum(p.numel() for p in model.parameters())
    print(f"[*] Тетраэдр Загвар: Давхарга={num_layers}, d_model={d_model}, Нийт Параметр={total_params:,}")

    # 2. Өгөгдөл бэлтгэх (Өргөтгөсөн датасет)
    expanded_text = load_expanded_corpus()
    x_all, y_all = prepare_dataset(expanded_text, seq_len=seq_len)
    num_samples = len(x_all)
    print(f"[*] Өргөтгөсөн сургалтын өгөгдөл: {num_samples} багц ({len(expanded_text):,} тэмдэгт, seq_len={seq_len})")

    # 3. Оптимизатор
    optimizer = AdamW(model.parameters(), lr=lr, weight_decay=1e-4)
    scheduler = torch.optim.lr_scheduler.CosineAnnealingLR(optimizer, T_max=epochs, eta_min=1e-5)

    print("\n[Сургалтын цикл эхэллээ (Temperature Annealing Soft-to-Hard)]: ")
    print("-" * 80)
    print(f"{'Эпох':^6} | {'Temp':^6} | {'Cross-Entropy':^14} | {'Repulsion Loss':^16} | {'Нийт Алдагдал':^14} | {'Хугацаа':^10}")
    print("-" * 80)

    start_train_time = time.time()

    for epoch in range(1, epochs + 1):
        # 1.0 -> 0.01 температурын зөөлөн хөлдөөлт (Gumbel/Tanh Annealing)
        current_temp = max(0.01, 1.0 - ((epoch - 1) / max(1, epochs - 1)) * 0.99)
        model.set_temperature(current_temp)

        model.train()
        epoch_ce = 0.0
        epoch_rep = 0.0
        epoch_total = 0.0
        steps = 0

        permutation = torch.randperm(num_samples)
        for i in range(0, num_samples, batch_size):
            indices = permutation[i : i + batch_size]
            b_x = x_all[indices].to(device)
            b_y = y_all[indices].to(device)

            optimizer.zero_grad()
            logits, loss = model(b_x, b_y)

            # Алдагдлыг салгаж харах
            with torch.no_grad():
                target_idx = torch.clamp(b_y - DPSTA_CHAR_BASE, 0, DPSTA_CHAR_COUNT - 1)
                ce_part = nn.functional.cross_entropy(logits.view(-1, DPSTA_CHAR_COUNT), target_idx.view(-1)).item()
                rep_part = (sum(l.repulsion_loss() for l in model.layers) / len(model.layers)).item()

            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()

            epoch_ce += ce_part
            epoch_rep += rep_part
            epoch_total += loss.item()
            steps += 1

        scheduler.step()

        if epoch % 5 == 0 or epoch == 1:
            avg_ce = epoch_ce / steps
            avg_rep = epoch_rep / steps
            avg_tot = epoch_total / steps
            print(f"{epoch:^6} | {current_temp:^6.2f} | {avg_ce:^14.4f} | {avg_rep:^16.4f} | {avg_tot:^14.4f} | {time.time() - start_train_time:^8.2f}s")

    total_time = time.time() - start_train_time
    print("-" * 72)
    print(f"[+] Сургалт амжилттай дууслаа! Нийт хугацаа: {total_time:.2f} сек")

    # 4. Загварыг 1.03-бит binary диск файлд экспортлох
    output_bin = "tetra_trained_model.bin"
    export_model_to_bin(model, output_bin)

    print("\n[+] Загвар амжилттай хадгалагдлаа: " + output_bin)
    print("=" * 72)

if __name__ == "__main__":
    train_tetra_model()
