# CRON Instruction Set Architecture (ISA) & .cl Machine Code Specification
**Target Architecture: 256-Core 4D-Torus ($4 \times 4 \times 4 \times 4$) Neuromorphic-Photonic VLIW Chip**

---

## 1. Architectural Model

```
       +-----------------------------------------------------------+
       |                  256 CORES (4D-TORUS MESH)                |
       |  Core ID = X + 4*Y + 16*Z + 64*W  (X,Y,Z,W in [0..3])     |
       +-----------------------------------------------------------+
                                   |
         +-------------------------+-------------------------+
         |                                                   |
   [ FIBER 0 (Compute) ]                               [ FIBER 1 (NoC/Sentry) ]
   - 4-Slot VLIW Execution Unit                        - NoC Router & DMA Engine
   - Photonic MZI Matrix Unit                          - Spatial Gather & Broadcast
   - Thermodynamic Reversible ALU                      - Thermal Watchdog Sentry
   - Neuromorphic STDP Synapse Array                   - Local Region Arena Reset
```

### 1.1 Registers per Core
- **General/Cognitive Registers**: $R_0$ to $R_{15}$ (32-bit registers, can hold packed 16x 2-bit ternary vectors or float/int values).
- **Photonic Wave Registers**: $W_0$ to $W_3$ (Stores coherent optical amplitude & phase tuples).
- **Reversible Shadow Registers**: $S_0$ to $S_7$ (Hardware-managed LIFO stack for zero-memory backwards differentiation $F^{-1}$).
- **Status & Predicate Registers**: $P_0$ (Predicate mask register), $T_{stat}$ (Thermal & error status).
- **Control & Status Registers (CSRs)**: `CYCLE_CNT`, `STALL_CNT`, `PRED_EXEC_CNT`, `VEC_BURST_CNT`.
- **Banked Register File**: 16 banks × 16 registers (Bank 0 mirrors local registers).

### 1.2 Hardware State per Core
- **LFSR PRNG**: 32-bit Galois LFSR (polynomial $x^{32} + x^{31} + x^{29} + x + 1 = \texttt{0xA000\_0003}$).
- **NoC Receive FIFO**: Variable-depth packet queue for inter-core communication.
- **Shadow Checkpoint Bank**: LIFO stack of full register snapshots for resilient compute.
- **Trap State**: `MCAUSE`, `MEPC`, `in_trap`, `trap_handler_addr` (RISC-V style).

---

## 2. VLIW Machine Code Format (.cl)

Every machine instruction corresponds to **1 execution cycle** (tact) consisting of **4 slots**:

```text
B<Cycle>: <Slot 0> <Slot 1> <Slot 2> <Slot 3>
```

Example:
```text
B0001: '=00#0A04> '=10#040B> _SH00$01B4> _SP00#0088>
B0002: _PK020$20> _TL030$04> _ST00$0110> _YD08$0205>
B0003: _OP041$0E> _FA054$20> _P0064$32> _RC07$0205>
B0004: _MD082$26> _BK095$01> _BL0A3$10> _RF0B6$04>
B0005: _GU049#00> _ST0C8$54> _SY0D7$28> _RS00#004B>
B0006: _FJ048#00> _DW00$0105> _SB04$0105> _HLT_____8!
```

### 2.1 Token Layout (10 Characters per Slot)

Each slot token is strictly **10 ASCII characters** long:

| Char Index | Field | Description | Example |
|---|---|---|---|
| `0` | **Prefix** | `_` (Native opcode) or `'` (Immediate/Config load) | `_` |
| `1..2` | **Opcode** | 2-letter operation mnemonic | `OP`, `FA`, `BK`, `RF`, `ST`, `SH` |
| `3..4` | **Target Reg / Core** | Destination register (hex `00`..`0F`) or Core ID | `04` ($R_4$) |
| `5` | **Source / Qualifier** | Source register index or sub-channel | `1` ($R_1$) |
| `6` | **Mode / Delimiter** | `$`, `#`, `@` mode identifier | `$` |
| `7..8` | **Immediate / Payload** | Instruction payload or argument hex | `0E`, `20`, `B4` |
| `9` | **Terminator / Parity** | `>` for standard slot, `!` for halt/critical barrier | `>` or `!` |

### 2.2 Parity & Zero-Ambiguity Rule
- Machine tokens are checked for parity across the 10 characters to prevent corrupted AI generations or bitflips.
- Every bundle is validated by `cronc` and verified in hardware/simulator before cycle execution.

---

## 3. Complete Opcode Reference

### 3.1 Photonic & Optical Compute Unit

| Opcode | Name | Semantics |
|---|---|---|
| `_OP` / `_WD` | **Optical GEMM** | $R_{dst} \leftarrow \text{MZI\_GEMM}(R_{src}, \text{Arg})$ — Photonic Mach-Zehnder interferometer matrix multiply |
| `_FA` | **Forward Autodiff Tap** | Push $R_{src}$ onto reversible stack; $R_{dst} \leftarrow R_{src}$ — Tap intermediate state for $\nabla F$ |
| `_CD` | **Hardware CORDIC** | $R_{dst} \leftarrow (\cos\theta \ll 16) \mid \sin\theta$ — Fixed-point sin/cos in 1 cycle (Q15 format) |

### 3.2 Reversible Thermodynamic ALU (Brain 3)

| Opcode | Name | Semantics |
|---|---|---|
| `_BK` | **Backward Invert** | $R_{dst} \leftarrow F^{-1}(\text{pop}(S))$ — Pop reversible stack, XOR-invert for gradient computation |
| `_RF` | **Reversible Fredkin Gate** | $\text{swap}(R_{dst}, R_{src})$ — Controlled reversible swap, zero entropy dissipation |
| `_TO` | **Toffoli 3-Wire Gate** | $\text{if } R_{10}[0] \wedge R_{11}[0]: R_{dst}[0] \oplus\!\!= 1$ — Universal reversible gate |

### 3.3 Predicated SIMD ALU

| Opcode | Name | Semantics |
|---|---|---|
| `_PO` / `_P0` / `_P1` | **Predicated ALU** | $R_{dst} \leftarrow R_{dst} \diamond R_{src}$ — Mode char selects op: `+`,`-`,`*`,`/`,`%`,`&`,`\|`,`^`,`~`,`=`,`<`,`>`,`?`,`!`,`G` |
| `_MD` | **Sub-Byte MAC** | 16× 2-bit ternary dot product or $R_{dst} \leftarrow R_{dst} \times R_{src}$ / $R_{dst} \div R_{src}$ |
| `_PK` | **Pack Sub-byte** | Pack 32-bit scalar stream into 2-bit ternary representation: $R_{dst} \leftarrow \texttt{0x5555\_AAAA}$ |
| `_BL` | **Blend Staged Halo** | $R_{dst} \leftarrow R_{src} \mid \texttt{0x8000}$ — Merge halo region data |
| `_PS` | **Parallel Prefix Sum** | Kogge-Stone adder tree: byte-lane cumulative sum across 4 packed bytes |
| `_TT` | **Tensor Tile Transpose** | 4×4 2-bit matrix transpose with strided swizzle pattern |
| `_IR` | **In-Network Reduction** | $R_{dst} \leftarrow R_{dst} + R_{src}$ — Accumulate in-flight reduction result |

### 3.4 Neuromorphic Engine (Brain 4)

| Opcode | Name | Semantics |
|---|---|---|
| `_ST` | **STDP Synapse Update** | Increment all synapse weights: $w_i \leftarrow \min(w_i + 2, 127)$ |
| `_LI` | **LIF Neuron Step** | Leaky integrate-and-fire: membrane decay 7/8 + current; spike if $\geq$ threshold |
| `_LF` | **LIF Spike Generator** | Simple spike: $R_{dst} \leftarrow (R_{src} > 100) ? 1 : 0$ |

### 3.5 Cognitive & Symbolic Engine (Brain 1)

| Opcode | Name | Semantics |
|---|---|---|
| `_SY` | **Symbolic Grounding** | $R_{dst} \leftarrow \texttt{0xCAFE\_BABE}$ — Compress continuous vector to symbol ID |
| `_KG` | **Knowledge Graph Query** | $R_{dst} \leftarrow 1$ — Assert or query knowledge graph edge |
| `_HE` | **Hyper-Edge Associator** | $R_{dst} \leftarrow \texttt{0xCAFE\_0000} \mid (R_{src}[7:0] \ll 8) \mid R_{dst}[7:0]$ |
| `_CA` | **Cross-Attention Gate** | $R_{dst} \leftarrow (R_{dst} \times R_{src}[7:0]) \gg 8$ — Attention-gated activation |

### 3.6 Chaos & Dynamical Systems (Brain 5)

| Opcode | Name | Semantics |
|---|---|---|
| `_OD` | **Chaos Diffusion** | $R_{dst} \leftarrow R_{src} \times 11 + 7 \oplus (R_{src} \gg 3)$ — Lorenz attractor step |
| `_SW` | **Superposition Wave** | Predicated execution: skip rest of cycle if $R_{src} = 0$ |

### 3.7 Resource Management (Brain 6)

| Opcode | Name | Semantics |
|---|---|---|
| `_RS` | **Region Arena Reset** | $R_1 \leftarrow 0$ — Reclaim all temporary region registers in 0 cycles |
| `_AW` | **Arbiter Weight Update** | $R_{dst} \leftarrow \min(R_{dst} + R_{src}[3:0], 255)$ — Dynamic priority scheduling |

### 3.8 Gradient & Learning

| Opcode | Name | Semantics |
|---|---|---|
| `_GU` | **Gradient Step Update** | $W \leftarrow W - \eta \cdot \nabla W$: $R_{dst} \leftarrow R_{dst} - R_{src}/2$ |

### 3.9 Network-on-Chip (4D Torus)

| Opcode | Name | Semantics |
|---|---|---|
| `_SB` | **Spatial Broadcast** | Broadcast $R_{dst}$ across 4D torus mesh axis |
| `_TX` | **NoC Channel Send** | Inject wormhole packet: $R_{src} \to$ NoC transmit FIFO |
| `_RX` | **NoC Channel Recv** | Pop from receive FIFO: $R_{dst} \leftarrow \text{FIFO.pop()}$ (blocking) |
| `_PL` | **NoC FIFO Poll** | Non-blocking poll: $R_{dst} \leftarrow (\text{FIFO} \neq \emptyset) ? 1 : 0$ |
| `_WH` | **Wormhole Tunnel** | Deterministic 4D hyper-torus routing: $R_{dst} \leftarrow \texttt{0x5500\_0000} \mid R_{src}[23:0]$ |
| `_DF` | **Deflection Route** | Adaptive deflection routing fallback: $R_{dst} \leftarrow 1$ |
| `_DW` | **DMA Write-back** | Scatter to High-Bandwidth Memory (HBM3) |
| `_TL` | **4D Torus Tile** | $R_{dst} \leftarrow \text{CoreID}$ — Load core's 4D coordinate |

### 3.10 Hardware Fibers & Control

| Opcode | Name | Semantics |
|---|---|---|
| `_SP` | **Spawn Fiber** | Activate Fiber 1 (async hardware coroutine) |
| `_FJ` | **Fiber Join** | Await Fiber 1 completion and merge result |
| `_YD` | **Coroutine Yield** | Yield execution to spatial gather / fiber switch |
| `_SH` | **Self-Healing Sentry** | Activate thermal watchdog with threshold from immediate |
| `_PT` | **Parity Telemetry** | Pipeline prefetch hint (NOP with telemetry annotation) |
| `_HLT` | **Halt Execution** | Stop core execution and signal host driver |

### 3.11 Control & Status Registers

| Opcode | Name | Semantics |
|---|---|---|
| `_RC` | **Resilient Compute / CSR** | Mode `!` imm=0: Shadow bank SAVE; Mode `!` imm=1: Shadow bank RESTORE; Mode `C`: CSR Read |
| `_RT` | **Return from Trap** | Clear trap state: `in_trap ← false`, `mcause ← 0` (equivalent to MRET) |
| `_RN` | **LFSR Random** | Advance Galois LFSR by one step: $R_{dst} \leftarrow \text{LFSR.next()}$ |

### 3.12 Hardware Security

| Opcode | Name | Semantics |
|---|---|---|
| `_AC` | **Capability Token** | $R_{dst} \leftarrow \texttt{0xC4F0\_0001}$ — Issue hardware capability token |
| `_SN` | **Bounds Sanitization** | $R_{dst} \leftarrow R_{src} \wedge \texttt{0x00FF\_FFFF}$ — 24-bit address masking |
| `_SC` | **Secure I-Cache Patch** | $R_{dst} \leftarrow 1$ — Validate instruction cache integrity |

### 3.13 Atomic Operations

| Opcode | Name | Semantics |
|---|---|---|
| `_CS` | **Compare-and-Swap** | If $R_{dst} = R_{src}$: $R_{dst} \leftarrow \text{imm}$, $R_0 \leftarrow 1$; else $R_0 \leftarrow 0$ |

---

## 4. Homopolymer Macro Opcodes

Homopolymer opcodes are **twin-character** tokens (e.g., `CC`, `DD`, `FF`) that trigger chip-wide macro operations. They are designed for maximum decode simplicity — the hardware recognizes repeated characters as a single-cycle macro command.

### 4.1 Uppercase Homopolymers

| Opcode | Name | Semantics |
|---|---|---|
| `AA` | **Dual-Bank SIMD Broadcast** | Auto-accumulate $R_{10} \leftarrow R_{10}^2$ across Bank 0 and Bank 10 |
| `BB` | **Brain-Bridge Sync** | Cross-neuromorphic synchronization: copy STDP weights → wave register amplitudes |
| `CC` | **Cache Invalidation** | Chip-wide 256-core I/D cache & pipeline flush; clear stall counters and shadow bank |
| `DD` | **Direct DMA Transfer** | Zero-overhead 4D-torus NoC DMA burst: $\text{Bank}[d][r] \leftarrow R_{src}$ |
| `EE` | **DVFS Energy Scaling** | Enter eco-mode: voltage/frequency scaling, thermal dissipation to 25°C baseline |
| `FF` | **Fredkin Full Fold** | Pop entire reversible stack, accumulate sum: $R_{dst} \leftarrow \sum S_i$ |

### 4.2 Numeric Homopolymers

| Opcode | Name | Semantics |
|---|---|---|
| `00` | **Zero-Fill Reset** | (Reserved for future zero-fill operations) |
| `11` | **Photonic Laser Pump** | Strobe all 4 wave register amplitudes to maximum (255) |
| `88` | **Arena Instant Reset** | Clear reversible stack in 0 cycles (hardware arena reclamation) |
| `99` | **Sentry Watchdog Trip** | Set thermal threshold to 180°C; increment sentry trip counter |

### 4.3 Lowercase Homopolymers

| Opcode | Name | Semantics |
|---|---|---|
| `aa` | **All-to-All Scatter** | Hypercube scatter: spatial broadcast count += 4 (all axes simultaneously) |
| `ee` | **Spike Broadcast** | Event-driven neuromorphic: increment all STDP weights by 1 (soft Hebbian) |

---

## 5. 4D Torus Routing & Coordinate Calculation

For core coordinates $(x, y, z, w)$ with $x,y,z,w \in \{0, 1, 2, 3\}$:
- **Core ID**:
  $$\text{ID} = x + 4y + 16z + 64w$$
- **Neighbor Routing**:
  $$\text{Neighbor}(X+) = ((x + 1) \bmod 4, y, z, w)$$
  $$\text{Neighbor}(X-) = ((x - 1 + 4) \bmod 4, y, z, w)$$
  (And analogously for axes $Y, Z, W$).
- Toroidal wrap-around guarantees a maximum topological diameter of 8 hops across all 256 cores.

---

## 6. Hardware Trap Mechanism

The CRON ISA supports RISC-V-style hardware traps with the following behavior:

1. **Trap Sources**: Division by zero (`MCAUSE = 0x0001`), assertion failure (`MCAUSE = 0x0002`)
2. **On Trap**: Save faulting cycle to `MEPC`, set `MCAUSE`, enter trap state
3. **Recovery**: Execute `_RT` (Return from Trap) to clear trap state and resume execution
4. **Shadow Checkpointing**: `_RC` with mode `!` saves/restores full register snapshots for fault tolerance

---

## 7. Feedback-Directed Optimization (FDO)

The CRON toolchain supports a profile-guided optimization loop:

1. **Compile**: `cronc` generates `.cl` VLIW bundles
2. **Simulate**: `cron-vm` executes and collects per-bundle execution profiles (`.prof`)
3. **Analyze**: Hot-bundle detection (≥5× baseline), cold-path identification, NOP density analysis
4. **Recompile**: `cronc --fdo` re-packs NOP-heavy bundles, annotates hot paths with `_PT` prefetch hints
5. **Verify**: Re-simulate to confirm cycle reduction and functional equivalence
