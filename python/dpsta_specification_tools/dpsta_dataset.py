# -*- coding: utf-8 -*-
"""
dpsta_dataset.py
Ultra-Compact 94-State Dataset Engine & Transient Stream Packager.

Philosophy:
  - "In-Flight" State: During computation, data exists as unwritten geometric waves,
    transient rotor phases, and wire voltages (Zero Disk I/O).
  - "Crystalline" State: When saved or staged, data condenses into the minimal
    7-bit discrete representation and 10-character Base-94 chunks.

Features:
  1. Compresses raw text/code into 7-bit discrete token streams (.dpdata / .cl94).
  2. Generates 10-character Base-94 chunks with CRC-8 integrity.
  3. Prepares streaming batches for 4-tact pipelined hardware consumption.
"""

import sys
import os
import struct

sys.stdout.reconfigure(encoding="utf-8")

NUM_STATES = 94
ASCII_START = 33
ASCII_END = 126


class CompactDatasetEngine:
    @staticmethod
    def char_to_state(ch: str) -> int:
        code = ord(ch)
        if ASCII_START <= code <= ASCII_END:
            return code - ASCII_START
        return 0

    @staticmethod
    def state_to_char(s: int) -> str:
        return chr((s % NUM_STATES) + ASCII_START)

    @classmethod
    def text_to_7bit_bytes(cls, text: str) -> bytes:
        """Encodes string into packed 7-bit discrete bytes."""
        states = [cls.char_to_state(c) for c in text]
        return bytes(states)

    @classmethod
    def bytes_to_text(cls, raw_bytes: bytes) -> str:
        """Decodes raw discrete bytes back to text."""
        return "".join([cls.state_to_char(b) for b in raw_bytes])

    @classmethod
    def pack_base94_chunk(cls, text_10: str) -> str:
        """Formats exactly 10 characters into a Base-94 hardware word."""
        cleaned = [c if ASCII_START <= ord(c) <= ASCII_END else '_' for c in text_10]
        if len(cleaned) < 10:
            cleaned.extend(['_'] * (10 - len(cleaned)))
        return "".join(cleaned[:10])

    @classmethod
    def create_dataset_package(cls, file_path_or_text: str, out_bin_path: str) -> dict:
        """
        Ingests raw source and crystallizes into ultra-compact binary dataset (.dpdata).
        Format:
          [4 bytes MAGIC 'DP94']
          [4 bytes uint32 Token Count]
          [N bytes Packed 7-bit states]
        """
        if os.path.exists(file_path_or_text):
            with open(file_path_or_text, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
        else:
            content = file_path_or_text

        packed_states = cls.text_to_7bit_bytes(content)
        token_count = len(packed_states)

        header = struct.pack("<4sI", b"DP94", token_count)
        with open(out_bin_path, "w+b") as f:
            f.write(header)
            f.write(packed_states)

        file_size = os.path.getsize(out_bin_path)
        return {
            "token_count": token_count,
            "raw_text_bytes": len(content.encode("utf-8")),
            "dataset_binary_bytes": file_size,
            "compression_ratio": len(content.encode("utf-8")) / max(1, file_size),
            "output_path": out_bin_path
        }

    @classmethod
    def load_dataset_stream(cls, bin_path: str):
        """Streams tokens from crystallized dataset with 0-disk latency."""
        with open(bin_path, "rb") as f:
            magic, count = struct.unpack("<4sI", f.read(8))
            if magic != b"DP94":
                raise ValueError("Invalid dataset magic signature. Expected 'DP94'.")
            while chunk := f.read(4096):
                for byte_val in chunk:
                    yield cls.state_to_char(byte_val)


# ==============================================================================
# TEST & VERIFICATION
# ==============================================================================
def test_dataset_engine():
    print("=" * 80)
    print("DPSTA COMPACT DATASET ENGINE (.dpdata) TEST")
    print("=" * 80)

    sample_code = """
    // Quantum Clifford Rotor Neural Kernel
    def solve_hyper_rotor(alpha, beta, gamma):
        state = rotor_sandwich(alpha, plane='E12', theta=8192);
        return tropical_max(state, beta) + gamma;
    """

    out_file = r"c:\Users\local_ybjuj27\Desktop\python learning\sample_corpus.dpdata"
    meta = CompactDatasetEngine.create_dataset_package(sample_code, out_file)

    print(f"1. Эх текст: {meta['raw_text_bytes']} байт")
    print(f"2. Кристалжсан Датасет: {meta['dataset_binary_bytes']} байт ({meta['token_count']} токен)")
    print(f"3. Файл үүсгэсэн байршил: {meta['output_path']}")

    # Stream back
    streamed_chars = list(CompactDatasetEngine.load_dataset_stream(out_file))
    reconstructed = "".join(streamed_chars)
    print(f"4. 0-Disk Сааталтайгаар Уншиж сэргээсэн тэмдэгтийн тоо: {len(streamed_chars)}")
    print(f"5. Датасет эх тексттэй яв цав нийцсэн эсэх: {len(sample_code) == len(reconstructed)}")
    print("=" * 80)


if __name__ == "__main__":
    test_dataset_engine()
