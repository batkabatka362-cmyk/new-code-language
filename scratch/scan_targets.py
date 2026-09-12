import os
import glob
import json
import sys

sys.stdout.reconfigure(encoding='utf-8')

# Search OCR dumps for specific sections:
# 1. Static Trait system without VTable
# 2. RAW / WAW hazard resolution & NOP insertion in compiler
# 3. Hardware Traps & Interrupt Service Routine (ISR) on Div-by-Zero / Overflow
# 4. Performance counters (CYCLE_CNT, STALL_CNT, MZI_ENERGY, TEMPERATURE)
# 5. DMA Double-Buffering / 2D/3D Tile Prefetch
# 6. Python SDK / Host Runtime Bridge
# 7. Sub-byte ternary BitNet 1.58b SIMD operators
# 8. Function ABI / Calling Convention

targets = [
    "trait",
    "hazard",
    "raw hazard",
    "hardware trap",
    "cycle_cnt",
    "stall_cnt",
    "double-buffer",
    "python",
    "lfsr",
    "abi",
    "calling convention"
]

print("Scanning OCR data for targeted architectural deep-dives...\n")

ocr_files = sorted(glob.glob('docs/ocr_data/ocr_pages_*.json'))
matches = {t: [] for t in targets}

for f in ocr_files:
    try:
        with open(f, encoding='utf-8') as fp:
            data = json.load(fp)
            entries = []
            if isinstance(data, list):
                entries = data
            elif isinstance(data, dict):
                entries = data.values()
            
            for item in entries:
                text = item if isinstance(item, str) else item.get('text', '')
                if not text and isinstance(item, dict):
                    text = str(item)
                low = text.lower()
                for t in targets:
                    if t in low:
                        # find snippet
                        idx = low.find(t)
                        snip = text[max(0, idx-60):min(len(text), idx+180)].replace('\n', ' ')
                        if len(matches[t]) < 3:
                            matches[t].append((os.path.basename(f), snip))
    except Exception as e:
        pass

for t, snips in matches.items():
    print(f"=== Target: '{t}' ({len(snips)} matches) ===")
    for src, s in snips:
        print(f"  [{src}] {s}")
    print()
