import json
import glob
import os
import sys

sys.stdout.reconfigure(encoding='utf-8')

def main():
    print("==================================================")
    print("      DEEP CRON BOOK & OCR KNOWLEDGE AUDIT        ")
    print("==================================================")
    
    # 1. Read all OCR json files
    ocr_files = sorted(glob.glob('docs/ocr_data/ocr_pages_*.json'))
    print(f"Total OCR Page Bundles: {len(ocr_files)}")
    
    keywords = [
        "trait", "static trait", "vtable",
        "differential", "autodiff", "backward",
        "bitnet", "ternary", "1.58b",
        "4d torus", "torus", "noc", "mesh",
        "region", "arena", "linear", "affine",
        "hardware trap", "trap", "sentry", "self-healing",
        "stdp", "lif", "spike", "neuromorphic",
        "superposition", "quantum", "collapse", "born",
        "photonic", "mzi", "optical gemm", "interference",
        "fredkin", "toffoli", "reversible",
        "driver", "host driver", "pcie", "axi", "dma",
        "verilator", "rtl", "verilog",
        ".cl", ".cr", "vliw", "bundle", "parity",
        "proof_contract", "invariant", "ensures"
    ]
    
    found_snippets = {kw: [] for kw in keywords}
    
    total_pages_scanned = 0
    for ocr_file in ocr_files:
        try:
            with open(ocr_file, encoding='utf-8') as f:
                data = json.load(f)
                if isinstance(data, list):
                    for entry in data:
                        total_pages_scanned += 1
                        text = entry if isinstance(entry, str) else entry.get('text', '')
                        if not text and isinstance(entry, dict):
                            text = str(entry)
                        lower_text = text.lower()
                        for kw in keywords:
                            if kw in lower_text:
                                # take a snippet
                                idx = lower_text.find(kw)
                                start = max(0, idx - 80)
                                end = min(len(text), idx + 120)
                                snippet = text[start:end].replace('\n', ' ')
                                if len(found_snippets[kw]) < 5:
                                    found_snippets[kw].append((os.path.basename(ocr_file), snippet))
                elif isinstance(data, dict):
                    for page_num, text in data.items():
                        total_pages_scanned += 1
                        if isinstance(text, str):
                            lower_text = text.lower()
                            for kw in keywords:
                                if kw in lower_text:
                                    idx = lower_text.find(kw)
                                    start = max(0, idx - 80)
                                    end = min(len(text), idx + 120)
                                    snippet = text[start:end].replace('\n', ' ')
                                    if len(found_snippets[kw]) < 5:
                                        found_snippets[kw].append((f"{os.path.basename(ocr_file)}:p{page_num}", snippet))
        except Exception as e:
            pass

    print(f"Scanned pages across all bundles.\n")
    
    # Check what concepts appear most and what specific architectures/ideas are described
    print("--- KEYWORD CITATION DENSITY ---")
    for kw, snips in found_snippets.items():
        print(f"  [{kw:20}]: {len(snips)} sample citations found")

    # 2. Print all milestone titles to examine all chapter/direction titles
    print("\n--- MILESTONES / BOOK SECTIONS (ALL 182) ---")
    with open('docs/ocr_data/docx_milestones.json', encoding='utf-8') as f:
        milestones = json.load(f)
        
    for m in milestones:
        fname = m['file'].replace('.docx', '')
        # print if it contains directions ("чиглэл", "тархи", "алхам", "RTL", "Driver", "симуляци", "хөрвүүлэгч", etc.)
        print(f"  #{m['index']:03d}: {fname}")

if __name__ == '__main__':
    main()
