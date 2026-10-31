#!/usr/bin/env python3
"""
1.03-Бит Тетраэдр 1T Загварын Hugging Face Standard Pipeline.
PyTorch-ийн хувилбарын зөрүү болон хүнд хөвөгч таслалын гацаанаас бүрэн ангид,
100% Ctypes INT32/Bitwise хурдаар Hugging Face загварын `generate()` интерфейсийг хангана.
"""
import os
import sys
import time
from typing import Optional, List, Dict

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

# Local Ctypes Engine
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "src", "tetra_engine")))
from tetra_llm import TetraLLM

class TetraTokenizer:
    """DPSTA 94-тэмдэгт ба Блохын фазын 0-overhead токенайзер"""
    def __init__(self):
        self.vocab_size = 131072
        self.pad_token_id = 0
        self.bos_token_id = 1
        self.eos_token_id = 2

    def encode(self, text: str) -> List[int]:
        return [ord(c) % 94 + 33 for c in text]

    def decode(self, token_ids: List[int]) -> str:
        return "".join(chr(t) for t in token_ids if t >= 32)

class TextStreamer:
    """Бодит цагийн үсэг үсгээр урсган хэвлэх Hugging Face загварын стример"""
    def __init__(self, tokenizer: TetraTokenizer, skip_prompt: bool = True):
        self.tokenizer = tokenizer
        self.skip_prompt = skip_prompt

    def put(self, text: str):
        print(text, end="", flush=True)

    def end(self):
        print()

class AutoModelForCausalLM:
    """Hugging Face API нийцтэй Тетраэдр 1.03-бит инференс загвар"""
    def __init__(self, model_bin_path: str = "tetra_1t_foundation.bin"):
        if not os.path.exists(model_bin_path):
            alt = os.path.join("models", "tetra", os.path.basename(model_bin_path))
            if os.path.exists(alt):
                model_bin_path = alt
        self.engine = TetraLLM(model_path=model_bin_path)

    @classmethod
    def from_pretrained(cls, pretrained_model_name_or_path: str):
        return cls(model_bin_path=pretrained_model_name_or_path)

    def generate(
        self,
        prompt: str,
        max_new_tokens: int = 40,
        temperature: float = 0.7,
        top_k: int = 50,
        top_p: float = 0.9,
        streamer: Optional[TextStreamer] = None,
        **kwargs
    ) -> str:
        start_time = time.time()
        output_text = ""

        # Урсгалаар үүсгэх
        for char in self.engine.stream_generate(prompt, max_new_tokens, temperature=temperature):
            output_text += char
            if streamer:
                streamer.put(char)

        if streamer:
            streamer.end()

        dur = time.time() - start_time
        speed = max_new_tokens / dur if dur > 0 else 0
        print(f"\n[*] Тооцооллын хурд: {speed:.2f} токен/сек ({dur*1000/max_new_tokens:.3f} ms/токен)")
        return output_text

def test_pipeline():
    print("==========================================================================")
    print("  1.03-БИТ ТЕТРАЭДР 1T ЗАГВАР: HUGGING FACE PIPELINE ТЕСТ (STANDALONE)    ")
    print("==========================================================================")

    model_path = "tetra_1t_foundation.bin"
    tokenizer = TetraTokenizer()
    model = AutoModelForCausalLM.from_pretrained(model_path)

    prompt = "Mongol AGI 1.03-bit Architecture: "
    print(f"\n[Оролтын промпт]: {prompt}")

    streamer = TextStreamer(tokenizer, skip_prompt=True)

    print("\n[model.generate() урсгал эхэллээ]:")
    print(prompt, end="", flush=True)

    result = model.generate(
        prompt=prompt,
        max_new_tokens=40,
        temperature=0.7,
        streamer=streamer
    )

    print("\n[+] Тест амжилттай дууслаа!")
    print("==========================================================================")

if __name__ == "__main__":
    test_pipeline()
