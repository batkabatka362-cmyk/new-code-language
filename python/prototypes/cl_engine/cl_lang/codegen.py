"""
================================================================================
CLIFFORD LANGUAGE (CL) MULTI-TARGET CODE GENERATOR v2.0
================================================================================
"""

import sys
from typing import List, Dict
from cl_lang.ast import ProgramNode, BundleDecl, SnakeDecl

MODE_CHAR = {
    'WRAP': 'W',
    'TUNNEL': 'T',
    'FLUID': 'F',
    'ANCHOR': 'A'
}

class CLCodeGen:
    @staticmethod
    def encode_snake_10char(s: SnakeDecl) -> str:
        m = MODE_CHAR.get(s.mode, 'W')
        s_idx = s.spin if s.spin in (0, 1, 2, 3) else (s.spin // 90) % 4
        return f"{m}{s.w:02d}{s.x:02d}{s.z:02d}{s_idx}{s.vol:02d}"

    @staticmethod
    def encode_bundle_vliw(bundle: BundleDecl) -> str:
        # Sort snakes by snake_id 0..3
        sorted_snakes = sorted(bundle.snakes, key=lambda s: s.snake_id)
        return "".join(CLCodeGen.encode_snake_10char(s) for s in sorted_snakes)

    @staticmethod
    def bundle_to_128bit_hex(bundle: BundleDecl) -> str:
        sorted_snakes = sorted(bundle.snakes, key=lambda s: s.snake_id)
        coords_84 = 0
        spin_8 = 0
        vol_32 = 0

        for i, s in enumerate(sorted_snakes):
            c21 = ((s.w & 0x7F) << 14) | ((s.x & 0x7F) << 7) | (s.z & 0x7F)
            coords_84 |= (c21 << (i * 21))

            s_idx = s.spin if s.spin in (0, 1, 2, 3) else (s.spin // 90) % 4
            spin_8 |= ((s_idx & 0x3) << (i * 2))

            vol_32 |= ((s.vol & 0xFF) << (i * 8))

        reg_128 = (vol_32 << 92) | (spin_8 << 84) | coords_84
        return f"{reg_128:032X}"

    @classmethod
    def emit_vliw(cls, prog: ProgramNode) -> str:
        """40-тэмдэгттэй VLIW машинд код үүсгэх"""
        lines = [f"# CL VLIW Microcode for Program: {prog.name}", f"# Format: 10-10-10-10 (40 chars per line)", ""]
        for b in prog.bundles:
            vliw = cls.encode_bundle_vliw(b)
            lines.append(f"{b.name}:{vliw}")
        return "\n".join(lines) + "\n"

    @classmethod
    def emit_verilog_hex(cls, prog: ProgramNode) -> str:
        """Verilog ROM memory initialization file (hex) үүсгэх"""
        lines = []
        for b in prog.bundles:
            lines.append(cls.bundle_to_128bit_hex(b))
        return "\n".join(lines) + "\n"

    @classmethod
    def emit_c_header(cls, prog: ProgramNode) -> str:
        """C SIMD streaming-д зориулсан толгой файл үүсгэх"""
        guard = f"CLIFFORD_{prog.name.upper()}_H"
        lines = [
            f"#ifndef {guard}",
            f"#define {guard}",
            "",
            "#include <stdint.h>",
            "",
            "typedef struct __attribute__((aligned(16))) {",
            "    uint64_t low64;   // Coords 0..63",
            "    uint64_t high64;  // Coords 64..83, Spins 84..91, Volumes 92..123",
            "} cl_universe_128_t;",
            "",
            f"// Compiled CL Bundles: {len(prog.bundles)}",
            f"static const cl_universe_128_t {prog.name.lower()}_rom[{len(prog.bundles)}] = {{"
        ]

        for b in prog.bundles:
            hex_str = cls.bundle_to_128bit_hex(b)
            val_128 = int(hex_str, 16)
            low = val_128 & 0xFFFFFFFFFFFFFFFF
            high = (val_128 >> 64) & 0xFFFFFFFFFFFFFFFF
            lines.append(f"    {{ 0x{low:016X}ULL, 0x{high:016X}ULL }}, // {b.name}")

        lines.extend([
            "};",
            "",
            f"#endif // {guard}",
            ""
        ])
        return "\n".join(lines)
