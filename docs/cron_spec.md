# CRON Language Specification (Version 2.0)
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

### 2.1 Module Structure

```ebnf
Program         ::= ModuleDecl EntryDecl? TopLevelItem* MainBlock?
ModuleDecl      ::= ".MODULE" Identifier
EntryDecl       ::= ".ENTRY" Identifier
MainBlock       ::= "_main:" Statement* ".END"

TopLevelItem    ::= FunctionDecl
                  | StructDecl
                  | EnumDecl
                  | TraitDecl
                  | ImplDecl
                  | ConstDecl
                  | ImportDecl
                  | TypeAliasDecl
```

### 2.2 Declarations

```ebnf
FunctionDecl    ::= ("async")? ("inline")? "def" Identifier GenericParams?
                    "(" ParamList? ")" ("->" Type)? Block
ParamList       ::= Param ("," Param)*
Param           ::= ("lin")? ("grad")? Identifier ":" Type

StructDecl      ::= "struct" Identifier GenericParams? "{" FieldList "}"
FieldList       ::= Field ("," Field)* ","?
Field           ::= Identifier ":" Type

EnumDecl        ::= "enum" Identifier GenericParams? "{" VariantList "}"
VariantList     ::= Variant ("," Variant)* ","?
Variant         ::= Identifier ("(" TypeList ")")?

TraitDecl       ::= "trait" Identifier GenericParams? "{" TraitMethod* "}"
TraitMethod     ::= "def" Identifier "(" ParamList? ")" ("->" Type)?

ImplDecl        ::= "impl" GenericParams? (Identifier "for")? Identifier
                    GenericArgs? "{" FunctionDecl* "}"

GenericParams   ::= "<" Identifier ("," Identifier)* ">"
GenericArgs     ::= "<" Type ("," Type)* ">"

ImportDecl      ::= "import" ModulePath ("as" Identifier)?
TypeAliasDecl   ::= "type" Identifier GenericParams? "=" Type
ConstDecl       ::= "const" Identifier ":" Type "=" Expression

ModulePath      ::= Identifier ("::" Identifier)*
```

### 2.3 Statements

```ebnf
Statement       ::= LetStmt
                  | AssignStmt
                  | IfStmt
                  | WhileStmt
                  | ForStmt
                  | MatchStmt
                  | RegionStmt
                  | ResilientStmt
                  | BrainStmt
                  | SuperpositionStmt
                  | ExprStmt
                  | ReturnStmt
                  | ExportStmt
                  | ForkStmt
                  | SimulateStmt

LetStmt         ::= "let" ("mut")? ("lin")? ("grad")? Identifier
                    (":" Type)? "=" Expression
AssignStmt      ::= Identifier "=" Expression
IfStmt          ::= "if" Expression Block ("else" (IfStmt | Block))?
WhileStmt       ::= "while" Expression Block
ForStmt         ::= "for" Identifier "in" Expression Block
MatchStmt       ::= "match" Expression "{" MatchArm+ "}"
MatchArm        ::= Pattern "=>" (Expression | Block) ","?
Pattern         ::= Literal | Identifier | "_" | EnumPattern
EnumPattern     ::= Identifier "::" Identifier ("(" PatternList ")")?

RegionStmt      ::= "region" StringLiteral? ("[" RegionAttrs "]")? Block
ResilientStmt   ::= "resilient" ("[" ResilientAttrs "]")? Block
                    ("fallback" Block)?
BrainStmt       ::= "brain" Identifier "[" BrainAttrs "]" Block
SuperpositionStmt ::= "superposition" "[" SuperposAttrs "]" Block

ReturnStmt      ::= "return" Expression?
ExportStmt      ::= "export" Identifier "as" Identifier
ForkStmt        ::= "fork" Expression "to" Expression
SimulateStmt    ::= "simulate" Expression ("with" Expression)?
```

### 2.4 Expressions

```ebnf
Expression      ::= Literal
                  | Identifier
                  | BinaryExpr
                  | UnaryExpr
                  | CallExpr
                  | MethodCallExpr
                  | FieldAccessExpr
                  | IndexExpr
                  | ConsumeExpr
                  | AwaitExpr
                  | SpawnExpr
                  | SpawnAtExpr
                  | ChannelSendExpr
                  | ChannelRecvExpr
                  | GradExpr
                  | CastExpr
                  | IfExpr
                  | ArrayExpr
                  | TupleExpr
                  | StructInitExpr
                  | ArrayRepeatExpr

BinaryExpr      ::= Expression BinaryOp Expression
UnaryExpr       ::= UnaryOp Expression
CallExpr        ::= Identifier "(" ArgList? ")"
MethodCallExpr  ::= Expression "." Identifier "(" ArgList? ")"
FieldAccessExpr ::= Expression "." Identifier
IndexExpr       ::= Expression "[" Expression "]"
ConsumeExpr     ::= "consume" "(" Identifier ")"
AwaitExpr       ::= "await" Expression
SpawnExpr       ::= "spawn" Expression
SpawnAtExpr     ::= "spawn_at" "(" Expression "," Expression ")"
ChannelSendExpr ::= Expression "<|" Expression
ChannelRecvExpr ::= Expression "|>"
GradExpr        ::= "grad" "(" Expression ")"
CastExpr        ::= Expression "as" Type
IfExpr          ::= "if" Expression "then" Expression "else" Expression
ArrayExpr       ::= "[" (Expression ("," Expression)*)? "]"
TupleExpr       ::= "(" Expression ("," Expression)+ ")"
StructInitExpr  ::= Identifier GenericArgs? "{" FieldInit ("," FieldInit)* "}"
ArrayRepeatExpr ::= "[" Expression ";" Expression "]"

ArgList         ::= Arg ("," Arg)*
Arg             ::= (Identifier "=")? Expression
FieldInit       ::= Identifier ":" Expression

BinaryOp        ::= "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^"
                  | "<<" | ">>" | "==" | "!=" | "<" | "<=" | ">" | ">="
                  | "and" | "or" | "&&" | "||"
UnaryOp         ::= "-" | "not" | "!" | "~"
```

### 2.5 Types

```ebnf
Type            ::= PrimitiveType
                  | ArrayType
                  | TupleType
                  | GenericType
                  | FunctionType
                  | Identifier

PrimitiveType   ::= "u8" | "u16" | "u32" | "u64"
                  | "i8" | "i16" | "i32" | "i64"
                  | "f16" | "f32" | "f64"
                  | "bool" | "ternary2" | "wave_t"
ArrayType       ::= "[" Type "]"
TupleType       ::= "(" Type ("," Type)+ ")"
GenericType     ::= Identifier "<" Type ("," Type)* ">"
FunctionType    ::= "fn" "(" TypeList? ")" "->" Type
```

### 2.6 Literals

```ebnf
Literal         ::= IntLiteral | FloatLiteral | HexLiteral | BoolLiteral
                  | StringLiteral | CharLiteral
IntLiteral      ::= [0-9]+ ("_" [0-9]+)*
FloatLiteral    ::= [0-9]+ "." [0-9]+ (("e"|"E") ("+"|"-")? [0-9]+)?
HexLiteral      ::= "0x" [0-9a-fA-F]+ ("_" [0-9a-fA-F]+)*
BoolLiteral     ::= "true" | "false"
StringLiteral   ::= '"' [^"]* '"'
```

---

## 3. Type System

### 3.1 Primitive Types
- `u8`, `u16`, `u32`, `u64`: Unsigned integers.
- `i8`, `i16`, `i32`, `i64`: Signed integers.
- `f16`, `f32`, `f64`: Floating-point numbers.
- `ternary2`: 2-bit ternary values ($\{-1, 0, +1\}$) packed 16 per 32-bit register.
- `bool`: Boolean flag.

### 3.2 Photonic & Optical Types
- `wave_t`: Coherent optical wave packet represented by amplitude and phase components:
  ```cron
  let w: wave_t = pack_wave(amp=[64, 32, 16, 8], phase=[32, 32, 64, 64])
  ```

### 3.3 Parametric Generics & Zero-Cost Monomorphization
Generic structs and functions are supported with angle-bracket syntax. The compiler performs full monomorphization at compile time, emitting separate concrete types for each instantiation:
```cron
struct Container<T> {
    value: T,
    tag: i32
}

def wrap<T>(val: T) -> Container<T> {
    return Container<T> { value: val, tag: 0 }
}

_main:
    let ci = wrap<i32>(42)        // Monomorphized to Container_i32
    let cf = wrap<f64>(3.14)      // Monomorphized to Container_f64
```

### 3.4 Linear Ownership & Safety (`lin`)
Values annotated with `lin` enforce **affine/linear type discipline**:
- Must be used **exactly once** or explicitly consumed (`consume(x)`).
- Cannot be silently dropped or duplicated, preventing memory leaks and hardware resource collisions.
- Powers zero-memory reverse auto-differentiation:
  ```cron
  let grad lin opt_latent = optical_gemm(wave=consume(w_seed))
  let lin d_weights = backward(opt_latent)
  ```

### 3.5 Scoped Region Arenas (`region`)
Dynamic memory allocation is confined to explicit regions. Exiting a region instantly triggers an **arena reset (0 cycles)** without garbage collection overhead:
```cron
region "TileProcessingArena" [target=SELF] {
    let packed_w = pack_subbyte(ext_hbm_base as u32, precision=2)
    // All temporary registers and buffers inside are reclaimed at '}'
}
```

### 3.6 Traits & Implementations
```cron
trait Computable {
    def compute(self) -> u32
    def reset(self)
}

impl Computable for Sensor {
    def compute(self) -> u32 {
        return self.value * 2
    }
    def reset(self) {
        self.value = 0
    }
}
```

### 3.7 Enums & Pattern Matching
```cron
enum Result<T> {
    Ok(T),
    Err(i32)
}

_main:
    let r: Result<u32> = Result::Ok(42)
    match r {
        Result::Ok(val) => {
            let doubled = val * 2
        }
        Result::Err(code) => {
            let error_code = code
        }
    }
```

---

## 4. The 6-Brain Hardware-Software Primitives

| Brain | Name | Native CRON Primitives | RTL Machine Mapping |
|---|---|---|---|
| **Brain 1** | Symbolic & Causal Graph | `ground_and_unify(latent, axiom)`, `ground_to_symbol(tile)` | `_UN` (Unify), `_SY` (Symbolify), `_KG` (Knowledge Graph), `_HE` (Hyper-Edge) |
| **Brain 2** | Photonic Continuous Latent | `optical_gemm(wave=...)`, `pack_wave(...)`, `mzi_modulate(...)` | `_OP` (MZI GEMM), `_FA` (Autodiff Tap), `_CD` (CORDIC) |
| **Brain 3** | Thermodynamic Reversible Memory | `backward(latent)`, `reversible_entangle(...)`, `invert(gate)` | `_BK` (Backward), `_RF` (Reversible Fredkin), `_TO` (Toffoli) |
| **Brain 4** | Neuromorphic STDP Plasticity | `step_synaptic_plasticity(acc, rate=...)`, `subbyte_dot(...)` | `_ST` (STDP Update), `_GU` (Weight Step), `_LI` (LIF), `_LF` (Spike Gen) |
| **Brain 5** | Quantum-Superposition Planner | `quantum_collapse_eval(concept)`, `superposition_branch(...)` | `_SW` (Superposition Wave), `_OD` (Chaos Diffusion), `_CA` (Cross-Attention) |
| **Brain 6** | Metacognitive Sentry | `resilient_compute`, `assert_homotopy_invariant(...)` | `_SH` (Self-Healing), `_SC` (Secure Cache), `_AW` (Arbiter Weight) |

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

### 5.2 Channel-Based Communication
```cron
// Send data to another core via typed channel
let ch: Channel<u32> = Channel::new()
ch <| 42          // Send (non-blocking)
let val = ch |>   // Receive (blocking)
```

### 5.3 Spatial Spawn
```cron
// Spawn computation on a specific core in the 4D torus
let result = spawn_at(core_id=0x2A, compute_kernel(data))
```

### 5.4 4D Torus Spatial Primitives
- `spatial_broadcast(data, axis=X+)`
- `spatial_gather(axis=X-)`
- `stage_prefetch(from=X+, buffer=STAGING_BUF_1)`
- `recover_from_neighbor(axis=X-)`

---

## 6. Brain DSL Blocks

The `brain` keyword introduces domain-specific blocks that map directly to hardware subsystems:

```cron
brain SymbolicReasoner [engine=BRAIN_1] {
    let symbol = ground_to_symbol(latent_vector)
    let unified = ground_and_unify(symbol, causal_axiom)
}

brain PhotonicLayer [engine=BRAIN_2] {
    let wave = pack_wave(amp=[64, 32], phase=[0, 128])
    let result = optical_gemm(wave=wave)
}
```

---

## 7. Compiler Optimization Pipeline

The CRON compiler (`cronc`) applies a 5-pass optimization pipeline after semantic validation:

1. **Constant Folding** — Evaluate compile-time constant expressions
2. **Dead Code Elimination** — Remove unreachable code and unused bindings
3. **Strength Reduction** — Replace expensive ops (`× 2^n` → `<< n`, `/ 2^n` → `>> n`, `% 2^n` → `& (2^n-1)`)
4. **Common Subexpression Elimination** — Reuse previously computed values
5. **Loop Invariant Code Motion** — Hoist invariant computations out of loops

Pipeline order: `Parse → Autodiff → Semantic Check → Optimize → Codegen`

---

## 8. Backend Targets

| Target | Command | Description |
|---|---|---|
| CRON VLIW `.cl` | `cron build` | 256-core 4D-torus VLIW machine code |
| C23 AOT | `cron c23` | High-performance C23 for GCC/Clang `-O3` |
| LLVM IR | `cron llvm` | LLVM intermediate representation |
| NVIDIA PTX | `cron ptx` | CUDA GPU kernel code |
| Apple Metal | `cron metal` | Apple GPU shader code |
| Verilog RTL | `cron verilog` | Hardware synthesis for FPGA/ASIC |
| JIT | `cron jit` | In-memory just-in-time execution |
