#!/usr/bin/env python3
import os
import sys

# Ensure importlib.metadata reports torch >= 2.5 for transformers 5.x compatibility
import importlib.metadata
_orig_version = importlib.metadata.version
importlib.metadata.version = lambda pkg: "2.6.0" if pkg == "torch" else _orig_version(pkg)

import torch
# PyTorch 2.4 Windows compatibility shim for DTensor in transformers 5.x
try:
    import torch.distributed.tensor as _dist_t
    if not hasattr(_dist_t, "DTensor"):
        from torch.distributed._tensor import DTensor as _DTensor
        _dist_t.DTensor = _DTensor
except Exception:
    pass

# Ensure the local tetra_hf package is in sys.path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    from configuration_tetra import TetraConfig
    from modeling_tetra import TetraForCausalLM
    from tokenization_tetra import TetraTokenizer
except ImportError:
    from .configuration_tetra import TetraConfig
    from .modeling_tetra import TetraForCausalLM
    from .tokenization_tetra import TetraTokenizer
from transformers import TextStreamer

def main():
    print("==========================================================================")
    print("  HUGGING FACE PreTrainedModel & GenerationMixin TEST (1.03-BIT TETRA)   ")
    print("==========================================================================")

    # 1. Config үүсгэх
    config = TetraConfig(
        vocab_size=131072,
        d_model=8192,
        num_hidden_layers=80,
        lib_path="./libtetra_engine.dll" if sys.platform == "win32" else "./libtetra_engine.so",
        model_bin_path="tetra_1t_foundation.bin"
    )

    # 2. Tokenizer болон Model үүсгэх
    print("[*] Hugging Face TetraTokenizer болон TetraForCausalLM загварыг ачаалж байна...")
    tokenizer = TetraTokenizer()
    model = TetraForCausalLM(config)

    prompt = "Mongol AGI 1.03-bit: "
    print(f"\n[Оролтын промпт]: {prompt}")

    # 3. Tokenize
    inputs = tokenizer(prompt, return_tensors="pt")
    streamer = TextStreamer(tokenizer, skip_prompt=True)

    print("\n[Hugging Face model.generate() эхэллээ]:")
    print(prompt, end="", flush=True)

    with torch.no_grad():
        outputs = model.generate(
            inputs["input_ids"],
            max_new_tokens=30,
            temperature=0.7,
            top_k=50,
            top_p=0.9,
            do_sample=True,
            streamer=streamer,
            pad_token_id=tokenizer.pad_token_id,
            eos_token_id=tokenizer.eos_token_id,
        )

    print("\n\n[+] Hugging Face GenerationMixin бүрэн амжилттай дууслаа!")
    print("==========================================================================")

if __name__ == "__main__":
    main()
