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

## 3. Opcode Reference Table

| Opcode | Name | Unit | Semantics |
|---|---|---|---|
| `_OP` | **Optical GEMM** | Photonic MZI | $R_{dst} \leftarrow \text{MZI\_GEMM}(R_{src}, \text{Arg})$ |
| `_FA` | **Forward Autodiff Tap** | Photonic MZI | Tap intermediate phase state into reversible register |
| `_BK` | **Backward Autodiff** | Reversible ALU | $R_{dst} \leftarrow F^{-1}(R_{src})$ compute gradient without DRAM load |
| `_RF` | **Reversible Fredkin** | Reversible ALU | Controlled reversible swap without entropy dissipation |
| `_MD` | **Sub-Byte MAC** | SIMD ALU | 16x 2-bit ternary dot product accumulation |
| `_PK` | **Pack Sub-byte** | SIMD ALU | Pack 32-bit scalar stream into 2-bit ternary registers |
| `_ST` | **STDP Synapse Update**| STDP Engine | Update synaptic weights based on spike timing delta |
| `_GU` | **Gradient Step** | ALU | $W \leftarrow W - \eta \cdot \nabla W$ in 1 cycle |
| `_UN` | **Unify Causal Axiom**| Symbolic Engine| Unify latent vector with symbolic rule axiom |
| `_SY` | **Symbolify State** | Symbolic Engine| Compress continuous vector into graph symbol ID |
| `_QP` | **Quantum Plan Eval** | Superposition | Evaluate superposition paths and collapse to optimal choice |
| `_SH` | **Self-Healing Sentry**| Watchdog Fiber | Activate sentry watchdog with thermal threshold |
| `_RS` | **Region Reset** | Memory Arena | Reclaim all temporary region registers in 0 cycles |
| `_SP` | **Spawn Fiber** | Hardware Fibers| Spawn Fiber 1 asynchronous background routine |
| `_FJ` | **Fiber Join** | Hardware Fibers| Await Fiber 1 completion and merge result |
| `_SB` | **Spatial Broadcast** | 4D Torus NoC | Broadcast packet across 4D torus axis |
| `_DW` | **DMA Wait** | 4D Torus NoC | Await DMA / asynchronous transfer completion |
| `_HLT`| **Halt Execution** | Control | Stop core execution and signal host driver |

---

## 4. 4D Torus Routing & Coordinate Calculation

For core coordinates $(x, y, z, w)$ with $x,y,z,w \in \{0, 1, 2, 3\}$:
- **Core ID**:
  $$\text{ID} = x + 4y + 16z + 64w$$
- **Neighbor Routing**:
  $$\text{Neighbor}(X+) = ((x + 1) \bmod 4, y, z, w)$$
  $$\text{Neighbor}(X-) = ((x - 1 + 4) \bmod 4, y, z, w)$$
  (And analogously for axes $Y, Z, W$).
- Toroidal wrap-around guarantees a maximum topological diameter of 8 hops across all 256 cores.
