# CRON Language Specification (Version 1.0)
**Human-Semantic Cognitive Architecture Blueprint for Next-Gen 4D-Torus AGI Hardware**

---

## 1. Overview & Core Philosophy

CRON is a high-level, human-readable semantic language designed specifically for next-generation non-von Neumann cognitive hardware: a **256-core 4D-Torus ($4 \times 4 \times 4 \times 4$) mesh architecture** integrated with:
1. **Mach-Zehnder Interferometer (MZI) Photonic Accelerators** for zero-latency optical GEMM.
2. **Fredkin/Toffoli Reversible Logic & Memory** for 0-entropy dissipation, memory-free auto-differentiation ($F^{-1}$).
3. **Neuromorphic Spike-Timing-Dependent Plasticity (STDP)** for continuous live synaptic weight adaptation without massive gradient pre-training.
4. **Symbolic Unification Engines** for zero-hallucination causal reasoning.
5. **Dual-Fiber Concurrency** (Fiber 0: Main Compute; Fiber 1: Background NoC IO & Sentry).

CRON uses clean, expressive, modern syntax with **Linear Type Safety** (`lin`), **Scoped Arena Regions** (`region`), and **First-Class 6-Brain Primitives**.

---

## 2. Syntax & Grammar (EBNF)

```ebnf
Module          ::= ModuleDecl EntryDecl? (FunctionDecl | ConstDecl)*
ModuleDecl      ::= ".MODULE" Identifier
EntryDecl       ::= ".ENTRY" Identifier

FunctionDecl    ::= ("async")? "def" Identifier "(" ParamList? ")" ("->" Type)? Block
ParamList       ::= Param ("," Param)*
Param           ::= ("lin")? Identifier ":" Type

Block           ::= "{" Statement* "}"

Statement       ::= LetStmt
                  | RegionStmt
                  | ResilientStmt
                  | AssignStmt
                  | ExprStmt
                  | ReturnStmt
                  | ExportStmt

LetStmt         ::= "let" ("lin")? ("grad")? Identifier (":" Type)? "=" Expression
RegionStmt      ::= "region" Identifier ("[" RegionAttrs "]")? Block
ResilientStmt   ::= "resilient_compute" "[" ResilientAttrs "]" Block ("fallback" Block)?
ReturnStmt      ::= "return" Expression?
ExportStmt      ::= "export" Identifier "as" Identifier

Expression      ::= Literal
                  | Identifier
                  | BinaryExpr
                  | CallExpr
                  | ConsumeExpr
                  | AwaitExpr
                  | SpawnExpr

ConsumeExpr     ::= "consume" "(" Identifier ")"
AwaitExpr       ::= "await" Expression
SpawnExpr       ::= "spawn" Expression
CallExpr        ::= Identifier "(" ArgList? ")"
ArgList         ::= Arg ("," Arg)*
Arg             ::= (Identifier "=")? Expression
```

---

## 3. Type System

### 3.1 Primitive Types
- `u8`, `u16`, `u32`, `u64`: Unsigned integers.
- `i8`, `i16`, `i32`, `i64`: Signed integers.
- `f16`, `f32`: Floating-point numbers.
- `ternary2`: 2-bit ternary values ($\{-1, 0, +1\}$) packed 16 per 32-bit register.
- `bool`: Boolean flag.

### 3.2 Photonic & Optical Types
- `wave_t`: Coherent optical wave packet represented by amplitude and phase components:
  ```cron
  let w: wave_t = pack_wave(amp=[64, 32, 16, 8], phase=[32, 32, 64, 64])
  ```

### 3.3 Linear Ownership & Safety (`lin`)
Values annotated with `lin` enforce **affine/linear type discipline**:
- Must be used **exactly once** or explicitly consumed (`consume(x)`).
- Cannot be silently dropped or duplicated, preventing memory leaks and hardware resource collisions.
- Powers zero-memory reverse auto-differentiation:
  ```cron
  let grad lin opt_latent = optical_gemm(wave=consume(w_seed))
  let lin d_weights = backward(opt_latent)
  ```

### 3.4 Scoped Region Arenas (`region`)
Dynamic memory allocation is confined to explicit regions. Exiting a region instantly triggers an **arena reset (0 cycles)** without garbage collection overhead:
```cron
region TileProcessingArena [target=SELF] {
    let packed_w = pack_subbyte(ext_hbm_base as u32, precision=2)
    // All temporary registers and buffers inside are reclaimed at '}'
}
```

---

## 4. The 6-Brain Hardware-Software Primitives

| Brain | Name | Native CRON Primitives | RTL Machine Mapping |
|---|---|---|---|
| **Brain 1** | Symbolic & Causal Graph | `ground_and_unify(latent, axiom)`, `ground_to_symbol(tile)` | `_UN` (Unify), `_SY` (Symbolify) |
| **Brain 2** | Photonic Continuous Latent | `optical_gemm(wave=...)`, `pack_wave(...)`, `mzi_modulate(...)` | `_OP` (MZI GEMM), `_FA` (Autodiff Tap) |
| **Brain 3** | Thermodynamic Reversible Memory | `backward(latent)`, `reversible_entangle(...)`, `invert(gate)` | `_BK` (Backward), `_RF` (Reversible Fredkin) |
| **Brain 4** | Neuromorphic STDP Plasticity | `step_synaptic_plasticity(acc, rate=...)`, `subbyte_dot(...)` | `_ST` (STDP Update), `_GU` (Weight Step) |
| **Brain 5** | Quantum-Superposition Planner | `quantum_collapse_eval(concept)`, `superposition_branch(...)` | `_QP` (Quantum Plan), `_CO` (Collapse) |
| **Brain 6** | Metacognitive Sentry | `resilient_compute`, `assert_homotopy_invariant(...)` | `_SH` (Self-Healing), `_PTC` (Patch Cache) |

---

## 5. Concurrency & 4D-Torus Spatial Networking

### 5.1 Dual-Fiber Architecture
Every core has two hardware fibers running in zero-overhead cooperative/hardware-scheduled concurrency:
- **Fiber 0**: High-throughput math, photonics, and neural pipeline.
- **Fiber 1**: Network-on-Chip (NoC) asynchronous message gathering, DMA prefetching, and thermal watchdog sentry.

```cron
async def background_fiber_task(channel: u32) -> u32 {
    let recvd = await spatial_gather(axis=channel)
    return recvd
}

// In main fiber:
let task_handle = spawn background_fiber_task(channel=2)
let bg_result = await task_handle
```

### 5.2 4D Torus Spatial Primitives
- `spatial_broadcast(data, axis=X+)`
- `spatial_gather(axis=X-)`
- `stage_prefetch(from=X+, buffer=STAGING_BUF_1)`
- `recover_from_neighbor(axis=X-)`
