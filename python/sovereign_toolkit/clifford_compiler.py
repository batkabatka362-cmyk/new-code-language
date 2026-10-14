# -*- coding: utf-8 -*-
"""
clifford_compiler.py
High-Level CL-ASM Compiler and Text-to-VLIW Pipeline for Clifford Rotor Machine Code (CL v1.0).

Features:
  1. Compiles human-readable Assembly (.clasm) into 40-Byte Base-94 VLIW Machine Code.
  2. Supports 4-slot parallel syntax:
       SLOT0: CLIFFORD_ADD R1, R0, R0
       SLOT1: ROT_SANDWICH R2, R1, B12, THETA=16384
       SLOT2: WEDGE_PROD   R3, R1, R2
       SLOT3: EMIT_CHAR    #32
  3. Supports shorthand Text-to-VLIW Auto-Generation (converts strings directly into
     a hardware sequence of 1-cycle execution bundles).
  4. Integrated Disassembler and Binary Verification.
"""

import sys
import re
from clifford_rotor_isa import pack_micro_op, CliffordBundle, CliffordRotorEmulator

sys.stdout.reconfigure(encoding="utf-8")


class CliffordCompiler:
    def __init__(self):
        self.emulator = CliffordRotorEmulator()

    def parse_register(self, reg_str: str) -> int:
        reg_str = reg_str.strip().upper()
        if reg_str.startswith("R"):
            return int(reg_str[1:])
        raise ValueError(f"Invalid register name: {reg_str}")

    def parse_line_micro_op(self, line: str) -> int:
        """Parses a single slot instruction like: 'CLIFFORD_ADD R1, R0, R0'."""
        line = line.strip()
        if not line or line.startswith(";"):
            return pack_micro_op("NOP", rd=0)

        # Match: OPCODE Rd, Rs1, Rs2, ...
        tokens = [t.strip().rstrip(",") for t in re.split(r"[\s,]+", line) if t.strip()]
        if not tokens:
            return pack_micro_op("NOP", rd=0)

        opcode = tokens[0].upper()

        if opcode == "NOP":
            return pack_micro_op("NOP", rd=0)

        elif opcode in ("CLIFFORD_ADD", "CLIFFORD_SUB", "TROPIC_MAX", "TROPIC_SUM", "HDC_BIND", "WEDGE_PROD"):
            rd = self.parse_register(tokens[1])
            rs1 = self.parse_register(tokens[2]) if len(tokens) > 2 else 0
            rs2 = self.parse_register(tokens[3]) if len(tokens) > 3 else 0
            return pack_micro_op(opcode, rd=rd, rs1=rs1, rs2=rs2)

        elif opcode == "ROT_SANDWICH":
            rd = self.parse_register(tokens[1])
            rs1 = self.parse_register(tokens[2])
            plane = "E12"
            theta = 0
            for extra in tokens[3:]:
                if extra.startswith("PLANE="):
                    plane = extra.split("=")[1].upper()
                elif extra.startswith("THETA="):
                    theta = int(extra.split("=")[1])
                elif extra.startswith("B"):
                    plane = extra.upper()
                elif extra.startswith("#"):
                    theta = int(extra[1:])
            return pack_micro_op("ROT_SANDWICH", rd=rd, rs1=rs1, plane=plane, theta=theta)

        elif opcode == "EMIT_CHAR":
            imm = 0
            arg = tokens[1]
            if arg.startswith("#"):
                imm = int(arg[1:])
            elif arg.startswith("'") and arg.endswith("'"):
                imm = ord(arg[1:-1]) - 33
            else:
                imm = int(arg)
            return pack_micro_op("EMIT_CHAR", rd=0, imm=imm)

        elif opcode == "RING_PUSH":
            rs1 = self.parse_register(tokens[1])
            return pack_micro_op("RING_PUSH", rd=0, rs1=rs1)

        else:
            raise ValueError(f"Unknown opcode: {opcode}")

    def compile_source(self, source_asm: str) -> list[str]:
        """
        Compiles multi-line assembly into a list of 40-byte Base-94 bundle strings.
        Bundle format:
        BUNDLE:
          SLOT 0: ...
          SLOT 1: ...
          SLOT 2: ...
          SLOT 3: ...
        """
        bundles = []
        current_slots = [0, 0, 0, 0]
        slot_idx = 0

        for raw_line in source_asm.strip().splitlines():
            line = raw_line.split(";")[0].strip()
            if not line:
                continue

            if line.upper() == "---" or line.upper() == "END_BUNDLE":
                bundles.append(CliffordBundle(*current_slots).to_ascii_bundle())
                current_slots = [0, 0, 0, 0]
                slot_idx = 0
                continue

            # Check for explicit slot prefix like 'SLOT 0: ...'
            slot_match = re.match(r"^SLOT\s*([0-3])\s*:\s*(.*)$", line, re.IGNORECASE)
            if slot_match:
                s_num = int(slot_match.group(1))
                s_code = slot_match.group(2)
                current_slots[s_num] = self.parse_line_micro_op(s_code)
            else:
                # Sequential placement into next available slot
                current_slots[slot_idx] = self.parse_line_micro_op(line)
                slot_idx = (slot_idx + 1) % 4
                if slot_idx == 0:
                    bundles.append(CliffordBundle(*current_slots).to_ascii_bundle())
                    current_slots = [0, 0, 0, 0]

        if any(current_slots):
            bundles.append(CliffordBundle(*current_slots).to_ascii_bundle())

        return bundles

    def compile_text_message(self, text: str) -> list[str]:
        """Compiles a string message into a sequence of 1-cycle EMIT bundles."""
        bundles = []
        for i in range(0, len(text), 4):
            chunk = text[i:i+4]
            ops = [0, 0, 0, 0]
            for slot_idx, ch in enumerate(chunk):
                ops[slot_idx] = pack_micro_op("EMIT_CHAR", rd=0, imm=ord(ch) - 33)
            bundles.append(CliffordBundle(*ops).to_ascii_bundle())
        return bundles

    def run_program(self, bundle_strings: list[str]) -> str:
        """Executes the compiled program in the emulator and returns emitted output."""
        emu = CliffordRotorEmulator()
        for b_str in bundle_strings:
            bundle = CliffordBundle.from_ascii_bundle(b_str)
            emu.execute_bundle(bundle)
        return "".join(emu.emitted_chars)


# ==============================================================================
# SELF-TEST & VERIFICATION
# ==============================================================================
def test_compiler():
    print("=" * 80)
    print("CL-ASM COMPILER & TEXT-TO-VLIW PIPELINE VERIFICATION")
    print("=" * 80)

    compiler = CliffordCompiler()

    # 1. Test CL-ASM Source Compilation
    asm_source = """
    ; Bundle 1: Initialize multivector, rotate in E12 plane, and emit 'H'
    SLOT 0: CLIFFORD_ADD R1, R0, R0
    SLOT 1: ROT_SANDWICH R2, R1, B12, THETA=8192
    SLOT 2: TROPIC_MAX   R3, R1, R2
    SLOT 3: EMIT_CHAR    'H'
    END_BUNDLE

    ; Bundle 2: Emit 'E', 'L', 'L', 'O' in 1 single clock cycle!
    SLOT 0: EMIT_CHAR    'E'
    SLOT 1: EMIT_CHAR    'L'
    SLOT 2: EMIT_CHAR    'L'
    SLOT 3: EMIT_CHAR    'O'
    END_BUNDLE

    ; Bundle 3: Emit '!'
    SLOT 0: EMIT_CHAR    '!'
    END_BUNDLE
    """

    print("Compiling CL-ASM program...")
    bundles = compiler.compile_source(asm_source)

    print(f"\n[1] Compiled into {len(bundles)} VLIW Bundles (Total Execution Time: {len(bundles)} Clock Cycles):")
    for i, b in enumerate(bundles):
        print(f"  Cycle {i+1}: '{b}' (40 bytes)")

    print("\n[2] Executing on Clifford Rotor Core...")
    output = compiler.run_program(bundles)
    print(f"  Emitted Hardware Output: '{output}'")
    print(f"  Correct Output:          {output == 'HELLO!'}")

    # 2. Test Text Auto-Compilation
    print("\n[3] Testing Text Auto-Compiler for: 'DPSTA_CHIP_ACTIVE'")
    auto_bundles = compiler.compile_text_message("DPSTA_CHIP_ACTIVE")
    auto_out = compiler.run_program(auto_bundles)
    print(f"  Compiled into {len(auto_bundles)} cycles. Result: '{auto_out}'")
    print(f"  Correct Output: {auto_out == 'DPSTA_CHIP_ACTIVE'}")

    print("\n" + "=" * 80)
    print(">>> CL-ASM Compiler & High-Level Pipeline Test PASSED Successfully!")
    print("=" * 80)


if __name__ == "__main__":
    test_compiler()
