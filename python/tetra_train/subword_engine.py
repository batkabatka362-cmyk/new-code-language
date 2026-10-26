#!/usr/bin/env python3
"""
============================================================================
CRON SOVEREIGN SUBWORD & MORPHOLOGICAL TOKENIZER ENGINE
============================================================================
Pure Python implementation matching `crates/cronc/src/bpe_tokenizer.rs` and
`crates/cronc/src/cl_sovereign_corpus.rs`.
Features:
- 2,405 Sovereign Morph-Tokens (English keywords, systems types, Mongolian words)
- 256 Byte-Fallback tokens (<0x00> .. <0xFF>) for 100% lossless coverage
- Special structural tokens (<|im_start|>, <|im_end|>, <|pad|>, <|bos|>, <|eos|>)
- Total Vocabulary Size: ~2,670 tokens
- Zero UNK drops
============================================================================
"""

import os
import sys
import re
from typing import List, Dict

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

SPECIAL_TOKENS = [
    "<|pad|>",
    "<|bos|>",
    "<|eos|>",
    "<|unk|>",
    "<|im_start|>",
    "<|im_end|>",
    "<|user|>",
    "<|assistant|>",
    "<|think|>",
    "</think>",
]

class SovereignSubwordTokenizer:
    def __init__(self):
        self.token_to_id: Dict[str, int] = {}
        self.id_to_token: Dict[int, str] = {}
        self.byte_fallback_offset = 0

        # 1. Special tokens
        for i, tok in enumerate(SPECIAL_TOKENS):
            self.token_to_id[tok] = i
            self.id_to_token[i] = tok

        # 2. Byte fallback (256 bytes)
        self.byte_fallback_offset = len(self.token_to_id)
        for b in range(256):
            b_tok = f"<0x{b:02X}>"
            t_id = self.byte_fallback_offset + b
            self.token_to_id[b_tok] = t_id
            self.id_to_token[t_id] = b_tok

        # 3. Load 2,405 morph tokens from cl_sovereign_corpus.rs
        self._load_morph_tokens()

        # Build regex for tokenization
        # Sort subwords by length descending for greedy longest-match
        sorted_subwords = sorted(
            [w for w in self.token_to_id.keys() if not w.startswith("<|") and not w.startswith("<0x")],
            key=len,
            reverse=True
        )
        self.subwords_set = set(sorted_subwords)
        print(f"[*] Токенайзер бэлэн боллоо: {len(self.token_to_id)} ширхэг токентой толь бичиг.")

    def _load_morph_tokens(self):
        base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
        corpus_rs = os.path.join(base_dir, "crates", "cronc", "src", "cl_sovereign_corpus.rs")
        
        morph_tokens = []
        if os.path.exists(corpus_rs):
            with open(corpus_rs, "r", encoding="utf-8", errors="ignore") as f:
                in_array = False
                for line in f:
                    if "pub static SOVEREIGN_MORPH_TOKENS" in line:
                        in_array = True
                        continue
                    if in_array:
                        if "];" in line:
                            break
                        m = re.search(r'"([^"]+)"', line)
                        if m:
                            morph_tokens.append(m.group(1))

        current_id = len(self.token_to_id)
        for w in morph_tokens:
            if w not in self.token_to_id:
                self.token_to_id[w] = current_id
                self.id_to_token[current_id] = w
                current_id += 1

    @property
    def vocab_size(self) -> int:
        return len(self.token_to_id)

    def encode(self, text: str) -> List[int]:
        tokens = []
        i = 0
        n = len(text)

        while i < n:
            # Check special tokens first
            matched_special = False
            for st in SPECIAL_TOKENS:
                if text.startswith(st, i):
                    tokens.append(self.token_to_id[st])
                    i += len(st)
                    matched_special = True
                    break
            if matched_special:
                continue

            # Greedy subword match (up to 32 chars)
            max_len = min(32, n - i)
            matched = False
            for l in range(max_len, 0, -1):
                sub = text[i : i + l]
                if sub in self.token_to_id:
                    tokens.append(self.token_to_id[sub])
                    i += l
                    matched = True
                    break

            if not matched:
                # Byte fallback
                b = text[i].encode('utf-8')
                for byte_val in b:
                    tokens.append(self.byte_fallback_offset + byte_val)
                i += 1

        return tokens

    def decode(self, token_ids: List[int]) -> str:
        res = bytearray()
        out_str = []
        
        for tid in token_ids:
            tok = self.id_to_token.get(tid, "")
            if tok.startswith("<0x") and tok.endswith(">"):
                try:
                    val = int(tok[3:5], 16)
                    res.append(val)
                except Exception:
                    pass
            else:
                if res:
                    out_str.append(res.decode('utf-8', errors='replace'))
                    res = bytearray()
                out_str.append(tok)

        if res:
            out_str.append(res.decode('utf-8', errors='replace'))

        return "".join(out_str)

if __name__ == "__main__":
    tok = SovereignSubwordTokenizer()
    sample = "<|im_start|>user\nСайн байна уу! 1.03-бит Тетраэдр тооцоолол 70B.<|im_end|>\n"
    ids = tok.encode(sample)
    print(f"Промпт: {sample}")
    print(f"Токенуудын ID ({len(ids)} токен): {ids[:20]}...")
    decoded = tok.decode(ids)
    print(f"Буцааж декодлосон: {decoded}")
    assert sample == decoded, "Lossless roundtrip must match exact!"
    print("[+] 100% Бүрэн алдаагүй Round-trip шалгалт амжилттай давлаа!")
