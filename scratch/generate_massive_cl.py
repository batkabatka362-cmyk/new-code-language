"""
Generator for Massive 4D Sphere Matrix Microcode Benchmark (.cl)
Generates thousands of VLIW bundles (4 parallel slots = 4 heads per cycle)
exercising the full 4D Sphere across all 90 layers and all 4 quadrants.
"""

import sys
import os

BASE64_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
BASE90_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~"

def encode_mode(snake_id: int, has_error: bool, direct_pen: bool, rd: bool, wr: bool) -> str:
    val = ((snake_id & 0x3) << 4) | (int(has_error) << 3) | (int(direct_pen) << 2) | (int(rd) << 1) | int(wr)
    return BASE64_CHARS[val]

def generate_massive_cl(output_path: str, num_bundles: int = 10000):
    print(f"Generating {num_bundles} VLIW bundles ({num_bundles * 4:,} parallel slots)...")
    
    with open(output_path, "w", encoding="utf-8") as f:
        f.write("; ==============================================================================\n")
        f.write(f"; MASSIVE STRESS-TEST: {num_bundles:,} VLIW BUNDLES ({num_bundles * 4:,} SLOTS)\n")
        f.write("; Target: 256-Core 4D-Torus Architecture (0 FLOPs / Discrete Curvature Metric)\n")
        f.write("; ==============================================================================\n")
        f.write("@kernel massive_sphere_matrix_stress\n")
        f.write(".target silicon.sphere_isa\n")
        f.write(".ipc_target 4.0\n\n")

        for b in range(1, num_bundles + 1):
            slots = []
            for s_id in range(4):
                # Dynamic exploration parameters across 90 layers and 90 degrees
                w = (b + s_id * 22) % 90
                lat_x = (b * 3 + s_id * 17) % 90
                lon_z = (b * 5 + s_id * 29) % 90
                quad = ((b + s_id) % 4) + 1
                step = b % 90
                
                # Alternate read/write and penetration modes
                is_pen = (b % 10 == 0)
                is_rd = (b % 4 == 1)
                is_wr = (b % 4 == 3)
                is_err = False # Keep running smoothly
                
                mode_char = encode_mode(s_id, is_err, is_pen, is_rd, is_wr)
                w_char = BASE90_CHARS[w]
                x_char = BASE90_CHARS[lat_x]
                z_char = BASE90_CHARS[lon_z]
                q_char = str(quad)
                step_char = BASE90_CHARS[step]
                
                # Last slot of last bundle halts
                if b == num_bundles and s_id == 3:
                    alu_op = '!'
                else:
                    ops = ['+', '-', '*', '^']
                    alu_op = ops[(b + s_id) % 4]
                    
                reg_char = f"{s_id:X}"
                val_char = f"{(b * 7 + s_id) % 16:X}"
                
                slot = f"`{mode_char}{w_char}{x_char}{z_char}{q_char}{step_char}{alu_op}{reg_char}{val_char}"
                slots.append(slot)
                
            line = f"B{b:05d}: " + " ".join(slots) + "\n"
            f.write(line)

    file_size_mb = os.path.getsize(output_path) / (1024 * 1024)
    print(f"Successfully generated: {output_path} ({file_size_mb:.2f} MB)")

if __name__ == "__main__":
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 10000
    out = sys.argv[2] if len(sys.argv) > 2 else "examples/cl/massive_sphere_matrix.cl"
    generate_massive_cl(out, n)
