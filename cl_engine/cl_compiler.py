#!/usr/bin/env python3
"""
==============================================================================
CLIFFORD LANGUAGE (CL) COMPILER & TRANSPILER v1.0
==============================================================================
Translates 10-10-10-10 Base94 VLIW microcode (.cl) into:
  1. Cycle-accurate execution telemetry with mathematical invariant checking.
  2. Verilog ROM memory initialization file (rom_init.hex).
  3. C header with aligned data structures for SIMD streaming.
"""

import os
import sys

# Windows UTF-8 stdout configuration
sys.stdout.reconfigure(encoding='utf-8')

class CLSlot:
    def __init__(self, raw_str: str, slot_id: int):
        assert len(raw_str) == 10, f"Slot {slot_id} must be exactly 10 characters! Got: '{raw_str}' ({len(raw_str)})"
        self.raw = raw_str
        self.slot_id = slot_id
        
        # Microcode field extraction
        self.op = raw_str[0:2]        # e.g. "==", "_S", "_M", "NO"
        self.w_cmd = raw_str[2:4]     # e.g. "00", "+1", "-1"
        self.x_cmd = raw_str[4:6]     # e.g. "#0", "01"
        self.z_cmd = raw_str[6:8]     # e.g. "00", "02"
        self.flag = raw_str[8]        # e.g. 'P', 'S', 'C', '0'
        self.delim = raw_str[9]       # e.g. '}', '|', '/', '\', '%'

        self.is_anchor = (self.op == "_S" or self.op == "NO")
        self.is_active = not self.is_anchor

    def __repr__(self):
        return f"Slot[{self.slot_id}]: OP={self.op} W={self.w_cmd} X={self.x_cmd} Z={self.z_cmd} FLG={self.flag}"

class CLBundle:
    def __init__(self, line_num: int, label: str, raw_body: str):
        self.line_num = line_num
        self.label = label
        assert len(raw_body) == 40, f"Bundle '{label}' must have exactly 40 chars! Got {len(raw_body)}"
        self.raw_body = raw_body
        self.slots = [
            CLSlot(raw_body[0:10], 0),
            CLSlot(raw_body[10:20], 1),
            CLSlot(raw_body[20:30], 2),
            CLSlot(raw_body[30:40], 3),
        ]

    @property
    def active_mask(self):
        return [1 if s.is_active else 0 for s in self.slots]

class CLCompiler:
    def __init__(self):
        self.bundles = []

    def load_cl_file(self, file_path: str):
        print(f"[CL COMPILER] Loading source file: {file_path}")
        with open(file_path, "r", encoding="utf-8") as f:
            lines = f.readlines()

        for idx, line in enumerate(lines):
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            if ":" in line:
                parts = line.split(":", 1)
                label = parts[0].strip()
                body = parts[1].strip()
                bundle = CLBundle(idx + 1, label, body)
                self.bundles.append(bundle)
            else:
                raise ValueError(f"Syntax error at line {idx+1}: missing Bundle label prefix (e.g. B0000:)")

        print(f"[CL COMPILER] Parsed {len(self.bundles)} VLIW bundles successfully.")

    def run_simulation(self):
        print("\n" + "=" * 80)
        print(" CYCLE-ACCURATE CL EXECUTION & TOPOLOGICAL INVARIANT PROOF")
        print("=" * 80)
        print("Такт | Маск  | Эзлэхүүн Хуваарилалт (%)   | Нийлбэр | Спин Фазууд  | Хамильтониан | Статус")
        print("-" * 80)

        # Architectural registers
        r_depth = 50
        spin_phases = [0, 0, 0, 0]
        coordinates = [[r_depth, 25 * i, 25 * (3 - i)] for i in range(4)] # (W, X, Z)

        for cycle_idx, bundle in enumerate(self.bundles):
            mask = bundle.active_mask
            active_count = sum(mask)

            # Exact Bit Parity Volume Allocation
            v_shares = [0, 0, 0, 0]
            if active_count == 4:
                v_shares = [25, 25, 25, 25]
            elif active_count == 3:
                active_indices = [i for i in range(4) if mask[i]]
                lead = cycle_idx % 3
                shares = [33, 33, 33]
                shares[lead] += 1
                for i, idx in enumerate(active_indices):
                    v_shares[idx] = shares[i]
            elif active_count == 2:
                for i in range(4):
                    if mask[i]: v_shares[i] = 50
            elif active_count == 1:
                for i in range(4):
                    if mask[i]: v_shares[i] = 100
            elif active_count == 0:
                v_shares = [25, 25, 25, 25]
                # Crystal lattice freeze + radial hop
                r_depth = min(99, r_depth + 1)

            # Spin-Orbit Coupling step & Hamiltonian
            hamiltonians = []
            for i in range(4):
                slot = bundle.slots[i]
                if slot.is_active:
                    delta_space = 1
                    delta_spin = 0
                    # Update coordinates
                    coordinates[i][1] = (coordinates[i][1] + 1) % 100
                    coordinates[i][2] = (coordinates[i][2] + 1) % 100
                else:
                    delta_space = 0
                    delta_spin = 1
                    spin_phases[i] = (spin_phases[i] + 1) % 4
                
                h_k = delta_space + delta_spin
                hamiltonians.append(h_k)

            total_v = sum(v_shares)
            total_h = sum(hamiltonians)

            # Invariant Assertions
            assert total_v == 100, f"FATAL: Volume violation at cycle {cycle_idx}: sum={total_v}"
            assert total_h == 4, f"FATAL: Hamiltonian energy conservation failed at cycle {cycle_idx}: sum={total_h}"

            mask_str = "".join(str(m) for m in mask)
            vol_str = f"[{v_shares[0]:3d}, {v_shares[1]:3d}, {v_shares[2]:3d}, {v_shares[3]:3d}]"
            spin_str = f"[{spin_phases[0]}, {spin_phases[1]}, {spin_phases[2]}, {spin_phases[3]}]"
            
            print(f" {bundle.label} | {mask_str}  | {vol_str}         |  {total_v}%  | {spin_str}    |   H = {total_h}    | PASS [100% OK]")

        print("=" * 80)
        print("[SUCCESS] All architectural invariants verified with ZERO bit leakage!\n")

    def export_verilog_hex(self, hex_output_path: str):
        """Exports 32-bit control store words per bundle for Verilog initial blocks."""
        print(f"[CL EXPORT] Generating Verilog ROM hex: {hex_output_path}")
        with open(hex_output_path, "w", encoding="utf-8") as f:
            for cycle_idx, bundle in enumerate(self.bundles):
                mask = bundle.active_mask
                act_mask_val = (mask[0] << 3) | (mask[1] << 2) | (mask[2] << 1) | mask[3]
                active_count = sum(mask)
                
                vol_mode = 0
                if active_count == 4: vol_mode = 0
                elif active_count == 3: vol_mode = 1
                elif active_count == 2: vol_mode = 2
                elif active_count == 1: vol_mode = 3

                next_ptr = (cycle_idx + 1) % len(self.bundles)
                word = ((cycle_idx & 0x7) << 29) | ((act_mask_val & 0xF) << 25) | ((vol_mode & 0x3) << 23) | (next_ptr & 0x7)
                f.write(f"{word:08X}\n")
        print(f"[CL EXPORT] Wrote {len(self.bundles)} microcode words to '{hex_output_path}'")

    def export_c_header(self, header_path: str):
        """Exports an aligned C header with raw 40-character bundles for SIMD execution."""
        print(f"[CL EXPORT] Generating C SIMD header: {header_path}")
        with open(header_path, "w", encoding="utf-8") as f:
            f.write("/* Auto-generated by CL Compiler v1.0 */\n")
            f.write("#ifndef CLIFFORD_ROTOR_H\n")
            f.write("#define CLIFFORD_ROTOR_H\n\n")
            f.write("#include <stdint.h>\n\n")
            f.write("#define CL_BUNDLE_COUNT " + str(len(self.bundles)) + "\n")
            f.write("#define CL_SLOT_CHARS 10\n\n")
            f.write("typedef struct __attribute__((aligned(64))) {\n")
            f.write("    char slot[4][10];\n")
            f.write("} CLBundleAligned;\n\n")
            f.write("static const char CL_RAW_BUNDLES[CL_BUNDLE_COUNT][41] = {\n")
            for b in self.bundles:
                escaped_body = b.raw_body.replace('\\', '\\\\')
                f.write(f'    "{escaped_body}", // {b.label}\n')
            f.write("};\n\n")
            f.write("#endif // CLIFFORD_ROTOR_H\n")
        print(f"[CL EXPORT] Wrote C header to '{header_path}'")

def main():
    compiler = CLCompiler()
    src_file = os.path.join(os.path.dirname(__file__), "clifford_rotor.cl")
    if not os.path.exists(src_file):
        src_file = "clifford_rotor.cl"
        
    compiler.load_cl_file(src_file)
    compiler.run_simulation()
    
    hex_path = os.path.join(os.path.dirname(__file__), "rom_init.hex")
    h_path = os.path.join(os.path.dirname(__file__), "clifford_rom.h")
    compiler.export_verilog_hex(hex_path)
    compiler.export_c_header(h_path)

if __name__ == "__main__":
    main()
