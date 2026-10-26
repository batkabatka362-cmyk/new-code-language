#!/usr/bin/env python3
"""
Extracts the 100 high-fidelity sovereign dialogues and builds an instruction-tuned QA dataset
with <|im_start|>user\n{q}<|im_end|>\n<|im_start|>assistant\n{a}<|im_end|> formatting.
"""
import os
import sys
import re
import json

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

def extract_sovereign_dialogues():
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
    corpus_file = os.path.join(base_dir, "crates", "cronc", "src", "cl_sovereign_corpus.rs")
    
    with open(corpus_file, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()

    # Regex to extract tuples from SOVEREIGN_KNOWLEDGE_BASE: ("question", "answer")
    pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
    matches = pattern.findall(content)
    print(f"[*] cl_sovereign_corpus.rs-аас {len(matches)} ширхэг алтан QA харилцан яриа олдлоо.")

    formatted_dataset = []
    for q, a in matches:
        formatted_text = f"<|im_start|>user\n{q}<|im_end|>\n<|im_start|>assistant\n{a}<|im_end|>\n"
        formatted_dataset.append({
            "instruction": q,
            "response": a,
            "text": formatted_text
        })

    # Add extra foundational QAs for 1.03-bit Tetrahedral engine & Cron CL ecosystem
    extra_qas = [
        ("1.03-бит тетраэдр тооцоолол гэж юу вэ", 
         "1.03-бит Тетраэдр тооцоолол нь 128 жинг 17 байтад (132-бит) багтаан, хөвөгч таслалтай үржүүлэлтийн оронд 4D симплексийн XNOR ба POPCOUNT бит үйлдлээр 70B загварыг энгийн компьютерт 490+ токен/сек хурдтай ажиллуулах технологи юм."),
        ("ssm гармоник цагираг буфер яагаад 9mb ram эзэлдэг вэ",
         "Уламжлалт Attention кэш нь O(N^2) санах ой алддаг бол SSM гармоник буфер нь O(1) тогтмол 256-бит цагираг буферээр долгионы фазыг хадгалдаг тул контекст хичнээн уртссан ч ердөө 9 MB RAM эзэлнэ."),
        ("cron cl машин кодын давуу тал юу вэ",
         "Cron CL машин код нь 128-битийн VLIW 4-үүртэй багцаар 4D-Torus сүлжээ болон техник хангамжийн регистрүүдтэй шууд холбогдож, ямар ч үйлдлийн системийн сааталгүйгээр 4.0 IPC хурдтай ажилладаг."),
        ("яагаад 70b загвар 16gb санах ойд багтаж байна вэ",
         "Уламжлалт FP16 70B загвар 140 GB санах ой шаарддаг бол 1.03-бит тетраэдр шахалтаар 9.30 GB болж багассан бөгөөд санах ойн шууд зураглалаар (Memory-Mapped I/O) ажилладаг тул энгийн PC-д төвөггүй багтана.")
    ]

    for q, a in extra_qas:
        formatted_text = f"<|im_start|>user\n{q}<|im_end|>\n<|im_start|>assistant\n{a}<|im_end|>\n"
        formatted_dataset.append({
            "instruction": q,
            "response": a,
            "text": formatted_text
        })

    out_file = os.path.join(base_dir, "python", "tetra_train", "sovereign_instruct_qa.jsonl")
    with open(out_file, "w", encoding="utf-8") as f:
        for entry in formatted_dataset:
            f.write(json.dumps(entry, ensure_ascii=False) + "\n")

    print(f"[+] Нийт {len(formatted_dataset)} ширхэг бүтэцчилсэн зааварчилгаатай өгөгдөл бэлэн боллоо: {out_file}")
    return out_file

if __name__ == "__main__":
    extract_sovereign_dialogues()
