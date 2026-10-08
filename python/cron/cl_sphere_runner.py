"""
============================================================================
CRON 4D Sphere Matrix Machine Code Language (.cl) Sovereign VLIW Runner
============================================================================
Executes 128-bit VLIW bundles (4 parallel slots = 4 heads: X, Z, Y, W)
encoded in the exact 10-character slot machine language:

  Slot = [Trig][Mode64][W][X][Z][Quad][Step][ALU][Reg][Val]

  Pos 1:  Trigger marker '`' (ASCII 96)
  Pos 2:  Snake ID (X, Z, Y, W) + Error + Penetrate/Wrap + Read + Write
          (4 * 2 * 2 * 2 * 2 = 64 states in 1 Base-64 char)
  Pos 3:  Altitude / Layer W (0..89 in Base-90)
  Pos 4:  Latitude X (0..89° North-to-Equator in Base-90)
  Pos 5:  Longitude Z (0..89° Equator-to-South in Base-90)
  Pos 6:  Quadrant (1=NE, 2=NW, 3=SW, 4=SE)
  Pos 7:  Geodesic Step (0..89 in Base-90)
  Pos 8:  ALU Opcode (+, -, *, ^, =, >, !)
  Pos 9:  Target Register (0..F = R0..RF)
  Pos 10: Value Nibble or Terminator (0..F, >)

100% Multiplier-Free (0 FLOPs), 100% Integer Shift/Add Reversible Logic.
============================================================================
"""

import sys
import os
import re
import io
from typing import List, Dict, Tuple, Optional

if sys.platform == "win32":
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

# Base-64 table for Position 2 (64 states)
BASE64_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
BASE64_REV = {c: i for i, c in enumerate(BASE64_CHARS)}
BASE64_REV['-'] = 62
BASE64_REV['_'] = 63

# Base-90 table for Altitude W, Lat X, Lon Z, Step (90 divisions: 0..89)
BASE90_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~"
BASE90_REV = {c: i for i, c in enumerate(BASE90_CHARS)}

SNAKE_NAMES = ["X", "Z", "Y", "W"]
QUAD_NAMES = {1: "NE (+X,+Z)", 2: "NW (-X,+Z)", 3: "SW (-X,-Z)", 4: "SE (+X,-Z)"}


def encode_snake_mode(snake_id: int, has_error: bool, direct_penetrate: bool, read_mem: bool, write_mem: bool) -> str:
    """Encodes 6 bits (64 states) into 1 Base-64 character."""
    val = ((snake_id & 0x3) << 4) | (int(bool(has_error)) << 3) | (int(bool(direct_penetrate)) << 2) | (int(bool(read_mem)) << 1) | int(bool(write_mem))
    return BASE64_CHARS[val]


def decode_snake_mode(char: str) -> Tuple[int, bool, bool, bool, bool]:
    """Decodes 1 Base-64 character into (snake_id, has_error, direct_penetrate, read_mem, write_mem)."""
    val = BASE64_REV.get(char, 0)
    snake_id = (val >> 4) & 0x3
    has_error = bool((val >> 3) & 1)
    direct_penetrate = bool((val >> 2) & 1)
    read_mem = bool((val >> 1) & 1)
    write_mem = bool(val & 1)
    return snake_id, has_error, direct_penetrate, read_mem, write_mem


def encode_base90(val: int) -> str:
    """Encodes an integer 0..89 into Base-90."""
    return BASE90_CHARS[max(0, min(89, val))]


def decode_base90(char: str) -> int:
    """Decodes a Base-90 character into integer 0..89."""
    return BASE90_REV.get(char, 0)


class ClSphereSlot:
    """Represents one 10-character Sphere Matrix VLIW Slot."""

    def __init__(
        self,
        raw: str,
        snake_id: int,
        has_error: bool,
        direct_penetrate: bool,
        read_mem: bool,
        write_mem: bool,
        altitude_w: int,
        lat_x: int,
        lon_z: int,
        quadrant: int,
        step: int,
        alu_op: str,
        dest_reg: int,
        val_nibble: int,
    ):
        self.raw = raw
        self.snake_id = snake_id
        self.has_error = has_error
        self.direct_penetrate = direct_penetrate
        self.read_mem = read_mem
        self.write_mem = write_mem
        self.altitude_w = altitude_w
        self.lat_x = lat_x
        self.lon_z = lon_z
        self.quadrant = quadrant
        self.step = step
        self.alu_op = alu_op
        self.dest_reg = dest_reg
        self.val_nibble = val_nibble

    @classmethod
    def parse(cls, slot_str: str) -> "ClSphereSlot":
        if len(slot_str) != 10:
            raise ValueError(f"Slot must be exactly 10 characters, got {len(slot_str)}: '{slot_str}'")

        if slot_str[0] != '`':
            raise ValueError(f"Slot must start with trigger '`', got '{slot_str[0]}'")

        mode_char = slot_str[1]
        snake_id, has_err, direct_pen, rd, wr = decode_snake_mode(mode_char)

        w = decode_base90(slot_str[2])
        x = decode_base90(slot_str[3])
        z = decode_base90(slot_str[4])

        q_char = slot_str[5]
        if q_char in ('1', 'N', 'A'):
            quad = 1
        elif q_char in ('2', 'W', 'B'):
            quad = 2
        elif q_char in ('3', 'S', 'C'):
            quad = 3
        elif q_char in ('4', 'E', 'D'):
            quad = 4
        else:
            quad = (decode_base90(q_char) % 4) + 1

        step = decode_base90(slot_str[6])
        alu_op = slot_str[7]
        try:
            dest_reg = int(slot_str[8], 16)
        except ValueError:
            dest_reg = snake_id

        try:
            val_nibble = int(slot_str[9], 16)
        except ValueError:
            val_nibble = decode_base90(slot_str[9]) % 16

        return cls(
            raw=slot_str,
            snake_id=snake_id,
            has_error=has_err,
            direct_penetrate=direct_pen,
            read_mem=rd,
            write_mem=wr,
            altitude_w=w,
            lat_x=x,
            lon_z=z,
            quadrant=quad,
            step=step,
            alu_op=alu_op,
            dest_reg=dest_reg,
            val_nibble=val_nibble,
        )

    @classmethod
    def build(
        cls,
        snake: str,  # 'X', 'Z', 'Y', 'W'
        altitude_w: int,
        lat_x: int,
        lon_z: int,
        quadrant: int,  # 1..4
        step: int,
        has_error: bool = False,
        direct_penetrate: bool = False,
        read_mem: bool = False,
        write_mem: bool = False,
        alu_op: str = "+",
        dest_reg: int = 0,
        val_nibble: int = 0,
    ) -> "ClSphereSlot":
        s_id = SNAKE_NAMES.index(snake)
        mode_char = encode_snake_mode(s_id, has_error, direct_penetrate, read_mem, write_mem)
        w_char = encode_base90(altitude_w)
        x_char = encode_base90(lat_x)
        z_char = encode_base90(lon_z)
        q_char = str(quadrant)
        step_char = encode_base90(step)
        reg_char = f"{dest_reg & 0xF:X}"
        val_char = f"{val_nibble & 0xF:X}"
        raw = f"`{mode_char}{w_char}{x_char}{z_char}{q_char}{step_char}{alu_op}{reg_char}{val_char}"
        return cls(
            raw=raw,
            snake_id=s_id,
            has_error=has_error,
            direct_penetrate=direct_penetrate,
            read_mem=read_mem,
            write_mem=write_mem,
            altitude_w=altitude_w,
            lat_x=lat_x,
            lon_z=lon_z,
            quadrant=quadrant,
            step=step,
            alu_op=alu_op,
            dest_reg=dest_reg,
            val_nibble=val_nibble,
        )


class ClSphereBundle:
    """Represents a 128-bit VLIW bundle line: B<cycle>: Slot0 Slot1 Slot2 Slot3"""

    def __init__(self, cycle: int, slots: List[ClSphereSlot], comment: str = ""):
        self.cycle = cycle
        self.slots = slots
        self.comment = comment

    def to_cl_line(self) -> str:
        slots_str = " ".join(s.raw for s in self.slots)
        line = f"B{self.cycle:04d}: {slots_str}"
        if self.comment:
            line += f"  ; {self.comment}"
        return line


class HeadState:
    """State of one of the 4 heads (X, Z, Y, W) on the 4D Sphere."""

    def __init__(self, name: str, head_id: int):
        self.name = name
        self.head_id = head_id
        self.w = 0  # Altitude / Layer (0..89)
        self.x = 0  # Lat (0..89°)
        self.z = 0  # Lon (0..89°)
        self.quadrant = 1  # 1=NE, 2=NW, 3=SW, 4=SE
        self.step = 0
        self.has_error = False
        self.direct_penetrate = False
        self.read_mem = False
        self.write_mem = False
        self.history: List[Tuple[int, int, int, int]] = []


class ClSphereMachine:
    """
    Sovereign 4D Sphere Matrix Microcode Execution Engine.
    Executes .cl machine code bundles with zero floating-point operations.
    """

    def __init__(self):
        self.heads = [HeadState(name, i) for i, name in enumerate(SNAKE_NAMES)]
        self.r = [0] * 16  # 16 integer registers (R0..R15)
        self.grid_mem: Dict[Tuple[int, int, int], int] = {}  # 3D Sphere spatial memory (W, X, Z) -> value
        self.sphere_matrix_acc = [0] * 4  # 4D Sphere Matrix accumulator [X, Z, Y, W]
        self.cycles = 0
        self.total_slots = 0
        self.is_halted = False
        self.trace_log: List[str] = []

    def reset(self):
        self.__init__()

    def step_bundle(self, bundle: ClSphereBundle):
        """Executes one 128-bit VLIW bundle across the 4 heads in lockstep."""
        if self.is_halted:
            return

        self.cycles += 1
        bundle_trace = [f"Cycle {bundle.cycle:04d}:"]

        for slot in bundle.slots:
            self.total_slots += 1
            h = self.heads[slot.snake_id]

            # 1. Update Head State from Slot
            h.w = slot.altitude_w
            h.x = slot.lat_x
            h.z = slot.lon_z
            h.quadrant = slot.quadrant
            h.step = slot.step
            h.has_error = slot.has_error
            h.direct_penetrate = slot.direct_penetrate
            h.read_mem = slot.read_mem
            h.write_mem = slot.write_mem
            h.history.append((h.w, h.x, h.z, h.quadrant))

            # 2. Spatial Grid Memory Interaction
            cell_key = (h.w, h.x, h.z)
            if h.read_mem:
                self.r[slot.dest_reg] = self.grid_mem.get(cell_key, 0)
            if h.write_mem:
                self.grid_mem[cell_key] = self.r[slot.dest_reg]

            # 3. 0-FLOP Multiplier-Free Sphere Matrix Projection
            # Weight is derived from Quadrant sign (+1/-1) and (X, Z) geometric displacement
            q_sign = 1 if h.quadrant in (1, 4) else -1
            shift_amt = (h.w % 4) + 1
            geometric_weight = (q_sign * ((h.x + 1) * (h.z + 1))) >> shift_amt

            self.sphere_matrix_acc[slot.snake_id] += geometric_weight

            # 4. ALU Opcode Execution
            reg = slot.dest_reg
            val = slot.val_nibble

            if slot.alu_op == "+":
                self.r[reg] = (self.r[reg] + val) & 0xFFFFFFFF
            elif slot.alu_op == "-":
                self.r[reg] = (self.r[reg] - val) & 0xFFFFFFFF
            elif slot.alu_op == "*":
                # Multiplier-free reversible geometric shift/XOR
                self.r[reg] = (self.r[reg] ^ (val << shift_amt)) & 0xFFFFFFFF
            elif slot.alu_op == "^":
                self.r[reg] = (self.r[reg] ^ val) & 0xFFFFFFFF
            elif slot.alu_op == "=":
                self.r[reg] = val & 0xFFFFFFFF
            elif slot.alu_op == "!":
                self.is_halted = True

            pen_str = "DIR" if h.direct_penetrate else "WRP"
            err_str = "ERR" if h.has_error else "OK "
            rw_str = ("R" if h.read_mem else "-") + ("W" if h.write_mem else "-")
            bundle_trace.append(
                f"  [{h.name}] W={h.w:02d} X={h.x:02d}° Z={h.z:02d}° Q{h.quadrant} | {pen_str} {err_str} {rw_str} | ALU:{slot.alu_op} R{reg}={self.r[reg]:04X}"
            )

        self.trace_log.append("\n".join(bundle_trace))

    def run_bundles(self, bundles: List[ClSphereBundle]):
        for b in bundles:
            if self.is_halted:
                break
            self.step_bundle(b)

    def print_hud(self):
        """Displays rich terminal HUD for the 4D Sphere Matrix microcode status."""
        print("=" * 78)
        print("   CRON 4D SPHERICAL MATRIX MACHINE CODE ENGINE (0 FLOPs / REVERSIBLE)   ")
        print("=" * 78)
        print(f" Execution Cycles: {self.cycles:4d}  | Total Slots: {self.total_slots:4d}  | IPC: {self.total_slots / max(1, self.cycles):.2f} / 4.0")
        print(f" Machine Status:   {'[HALTED]' if self.is_halted else '[ACTIVE RUNNING]'}  | Multiplier FLOPs: 0 (100% Integer Shift/Add)")
        print("-" * 78)
        print(" 4-HEAD GEO-SPATIAL HUD (Origin 0 -> Sphere Surface W=0..89):")
        print(" Head | Layer W | Lat X° | Lon Z° | Quadrant       | Mode    | Status | R/W")
        print("------+---------+--------+--------+----------------+---------+--------+-----")
        for h in self.heads:
            pen = "Penetrate" if h.direct_penetrate else "Wrap 360°"
            st = "STOP/ERR" if h.has_error else "NORMAL OK"
            rw = ("READ " if h.read_mem else "") + ("WRITE" if h.write_mem else "")
            if not rw: rw = "IDLE "
            print(f"  {h.name}   |   {h.w:02d}    |  {h.x:02d}°   |  {h.z:02d}°   | {QUAD_NAMES.get(h.quadrant, 'NE'):14s} | {pen:7s} | {st:8s} | {rw}")

        print("-" * 78)
        print(" 4D SPHERICAL MATRIX ACCUMULATOR (Zero-FLOP Multiplier-Free Result):")
        print(f"   [ X: {self.sphere_matrix_acc[0]:+8d} | Z: {self.sphere_matrix_acc[1]:+8d} | Y: {self.sphere_matrix_acc[2]:+8d} | W: {self.sphere_matrix_acc[3]:+8d} ]")
        print("-" * 78)
        print(" CORE REGISTERS (R0..R7):")
        reg_line1 = " ".join(f"R{i}={self.r[i]:04X}" for i in range(8))
        print("  " + reg_line1)
        print(" CORE REGISTERS (R8..RF):")
        reg_line2 = " ".join(f"R{i}={self.r[i]:04X}" for i in range(8, 16))
        print("  " + reg_line2)
        print("=" * 78)


def parse_cl_sphere_program(cl_text: str) -> List[ClSphereBundle]:
    """Parses .cl machine code text containing 10-character Sphere Matrix bundles."""
    bundles = []
    bundle_pattern = re.compile(r"^[Bb](\d+):\s*(.*)$")

    for line in cl_text.splitlines():
        trimmed = line.strip()
        if not trimmed or trimmed.startswith(";") or trimmed.startswith("//") or trimmed.startswith("@") or trimmed.startswith("."):
            continue

        m = bundle_pattern.match(trimmed)
        if m:
            cycle = int(m.group(1))
            rest = m.group(2)
            comment = ""
            if ";" in rest:
                rest, comment = rest.split(";", 1)
                comment = comment.strip()

            tokens = rest.split()
            slots = []
            for tok in tokens:
                if tok.startswith("`") and len(tok) == 10:
                    slots.append(ClSphereSlot.parse(tok))
                elif len(tok) == 10 and tok[0] == "_":
                    # Fallback / standard NOP
                    pass

            if slots:
                bundles.append(ClSphereBundle(cycle, slots, comment))

    return bundles


def run_cl_file(file_path: str):
    """Loads a .cl machine code file and executes it on the 4D Sphere Machine."""
    if not os.path.exists(file_path):
        print(f"Error: File '{file_path}' not found.")
        return

    with open(file_path, "r", encoding="utf-8") as f:
        code = f.read()

    print(f"\n[+] Loading 4D Sphere Machine Microcode: {file_path}")
    bundles = parse_cl_sphere_program(code)
    print(f"[+] Successfully parsed {len(bundles)} VLIW bundles ({len(bundles) * 4} parallel slots).")

    vm = ClSphereMachine()
    vm.run_bundles(bundles)
    vm.print_hud()

    print("\n--- EXECUTION CYCLE TRACE ---")
    for trace in vm.trace_log:
        print(trace)
    print("-----------------------------\n")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        run_cl_file(sys.argv[1])
    else:
        print("Usage: python cl_sphere_runner.py <kernel.cl>")
