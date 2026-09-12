# CRON Architecture: Official Application Binary Interface (ABI) Specification

**Target Processor**: 256-Core ($4 \times 4 \times 4 \times 4$) 4D-Torus Neuromorphic / Photonic Cognitive Silicon  
**Version**: 1.0 (Official Standard)  
**Execution Model**: Stackless, Register-Dedicated, 1-Cycle Hardware Shadow Bank

---

## 1. Stackless Architectural Frame (Zero-DRAM Calling)

Traditional CPU architectures allocate stack frames in off-chip DRAM or L1 cache using stack pointers (`SP`), incurring memory load/store overhead on every function call. 

CRON eliminates DRAM stack frames entirely:
- **Zero Memory Overhead**: Function calls and returns execute in 0 overhead cycles simultaneously alongside VLIW instruction execution.
- **Immunity to Stack Overflow**: Memory corruption via stack overflow is physically impossible.
- **16 General-Purpose Dedicated Registers**: Explicitly partitioned into caller-saved, callee-saved, argument, and return roles.
- **1-Cycle Hardware Shadow Checkpoint Bank**: 16 hardware registers (`CR0..CR15`) latch the entire register file in 1 clock cycle via `_RC00#000>` without touching memory.

---

## 2. Core Register Allocation & Calling Conventions

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      CORE REGISTER ALLOCATION                           │
├───────────────┬───────────────────────────────┬─────────────────────────┤
│ Register      │ ABI Name                      │ Dedicated Function      │
├───────────────┼───────────────────────────────┼─────────────────────────┤
│ R0            │ $rv (Return Value) / $acc     │ Function return / accum │
│ R1 - R3       │ $a0 - $a2 (Arguments / Graph) │ Function parameters 1-3 │
│ R4 - R6       │ $t0 - $t2 (Temp / Neural)     │ Caller-saved temporaries│
│ R7 - R9       │ $s0 - $s2 (Saved / Spatial)   │ Callee-saved preserved  │
│ R10 - R12     │ $im0 - $im2 (Imagination)     │ Brain 5 Latent states   │
│ R13 - R14     │ $ar0 - $ar1 (Arbiter Meta)    │ Brain 6 Decision weights│
│ R15           │ $bp (Base Tile Pointer)       │ Memory / Region base    │
└───────────────┴───────────────────────────────┴─────────────────────────┘
      │                                                     ▲
      │ 1-Cycle Latch (_RC00#000>)                          │ 1-Cycle Restore (_RC00#001>)
      ▼                                                     │
┌─────────────────────────────────────────────────────────────────────────┐
│              16-ENTRY HARDWARE SHADOW CHECKPOINT BANK                   │
│   CR0..CR15 (Latches all 16 registers in 1 silicon clock cycle)         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Preservation Rules
- **Caller-Saved Registers** (`$rv`, `$t0..$t2`, `$im0..$im2`): The caller must assume these registers will be overwritten across nested function dispatches.
- **Callee-Saved Registers** (`$s0..$s2`, `$ar0..$ar1`, `$bp`): The callee must preserve these across its lifetime.
- **Nested Preservation**: If a callee requires nested dispatches, it issues `_RC00#000>` to latch all 16 registers into the hardware shadow bank in a single cycle, and `_RC00#001>` to restore them upon completion.

---

## 3. Function Argument Passing & Return Conventions

### Argument Ingress
1. **Primary Arguments**:
   - Argument 0: Passed in register `R1` (`$a0`)
   - Argument 1: Passed in register `R2` (`$a1`)
   - Argument 2: Passed in register `R3` (`$a2`)
2. **Extended Arguments (> 3 parameters)**:
   - Packed into a 32-bit quad-pointer representation using Brain 1 topological packing (`_TPC`), passing the pack reference in `R3`.

### Return Value Egress
1. **Scalar / 32-bit Return**: Always returned in register `R0` (`$rv`).
2. **64-bit Dual Wide Ingress (Brain 2 Neural Accumulators)**:
   - High 32 bits: `R1` (`$a0`)
   - Low 32 bits: `R0` (`$rv`)
3. **Multi-Value Tuple Return**:
   - Component 0: `R0`
   - Component 1: `R1`
   - Component 2: `R2`

---

## 4. 176-Bit Cross-Brain Interconnect ABI

When routing cognitive packets across the 176-bit dedicated crossbar bus linking all 6 brains, bitfields are rigidly partitioned:

| Bit Range | Width | Originating Brain | Payload Type |
|:---:|:---:|:---:|:---|
| `[15:0]` | 16-bit | **Brain 1 (Symbolic)** | Compressed Knowledge Graph Node Offset Pointer |
| `[31:16]` | 16-bit | **Brain 4 (Neuromorphic)** | STDP Genomic Synapse Plasticity Mask & Fitness Score |
| `[63:32]` | 32-bit | **Brain 3 (Quantum Phase)** | 4D Clifford Phase Vector (Complex Phasor) |
| `[111:64]` | 48-bit | **Brain 5 (Imagination)** | 3D Lorenz Chaos Diffusion Coordinates (X, Y, Z @ 16-bit) |
| `[175:112]` | 64-bit | **Brain 2 (Neural BitNet)** | Dual 32-bit Ternary MAC Accumulator Stream |

---

## 5. Compilation Mapping Example

### High-Level `.cr` Source:
```crystal
module MathCognitionABI

def eval_phase(vector_in: u32, phase_key: u32) -> u32 {
    // 1. Latch context into hardware shadow bank (1-cycle zero-stack overhead)
    checkpoint()

    // 2. Compute Hadamard Hologram on Brain 5
    let holo = fourier_hologram(vector_in, phase_key)

    // 3. Return hologram accumulator
    return holo
}
```

### Machine-Native `.cl` VLIW Bundle:
```cl
; --- BUNDLE 0001: ABI Function Entry & Execution ---
; Slot 0: _RC00#000>  -> Hardware Checkpoint (CR0..CR15 = R0..R15)
; Slot 1: _FH001$02.0> -> R0 = Hadamard(R1, R2) [R1=$a0, R2=$a1, R0=$rv]
; Slot 2: _NO00#000>  -> NOP pad
; Slot 3: _NO00#000>  -> NOP pad
B0001: _RC00#000> _FH001$02.0> _NO00#000> _NO00#000>
```
