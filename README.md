# CRON Programming Language Ecosystem
**Next-Generation Cognitive Architecture Language for 256-Core 4D-Torus Neuromorphic Hardware**

```
  ██████╗██████╗  ██████╗ ███╗   ██╗
 ██╔════╝██╔══██╗██╔═══██╗████╗  ██║
 ██║     ██████╔╝██║   ██║██╔██╗ ██║
 ██║     ██╔══██╗██║   ██║██║╚██╗██║
 ╚██████╗██║  ██║╚██████╔╝██║ ╚████║
  ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
```

---

## 1. Тойм ба Архитектур (Overview)

CRON бол уламжлалт von Neumann архитектур (CPU/GPU, санах ойн түгжрэл, өндөр дулаан ялгаруулалт)-ын хязгаарлалтыг даван туулах зорилгоор бүтээгдсэн, **Hardware-Software Co-Design** зарчимтай цоо шинэ програмчлалын хэл юм.

### Хоёр Ихэр Хэлний Загвар (Dual-Language Architecture):

<div align="center">

| **.cl (Cognitive Low-level)** | **.cr (Cognitive Representation)** |
|:---:|:---:|
| <img src="docs/assets/cl_icon.svg" width="160" alt=".cl Language Icon"/> | <img src="docs/assets/cr_icon.svg" width="160" alt=".cr Language Icon"/> |
| **Machine-Native VLIW** | **Human-Facing Cognitive Blueprint** |
| `Electric Neon Gold / Amber` | `Holographic Cyan / Violet` |
| 128-бит VLIW, 10 тэмдэгттэй үүрүүд | Шугаман төрөл (`lin`), Бүсийн Арена |
| [docs/cl_language_spec.md](docs/cl_language_spec.md) | [docs/cron_spec.md](docs/cron_spec.md) |

</div>

1. **`.cl` (Machine-Native VLIW / AI Co-Pilot Format)**:
   - 1 такт = 4 зааврын үүр (Slot 0, 1, 2, 3) бүхий 128 битийн VLIW багц.
   - Үүр бүр 10 тэмдэгттэй хатуу бүтэцтэй, 8 дахь тэмдэгт нь Parity хамгаалалттай.
   - 256 цөмт 4D-Torus чипийн RTL удирдлагын утсуудтай 1:1 шууд бууна.
2. **`.cr` (CRON High-Level Blueprint / Human Interface)**:
   - Python/Rust-ийн хослол шиг тунгалаг, цэвэрхэн бүтэц.
   - **Linear Types (`let lin ...`)**: Санах ойн цоорхой (memory leak)-г компиляторын түвшинд 100% хаана.
   - **Scoped Arena Regions (`region { ... }`)**: Гарахад 0-тактад санах ойг цэвэрлэнэ (0 Garbage Collection).
   - **6 Тархины примитивүүд**: Фотоник MZI, буцах логик, STDP синапс, учир шалтгааны бэлгэдэл дүрэм.

---

## 2. 6 Тархины Төрөлх Примитивүүд (The 6-Brain Primitives)

| Тархи | Нэршил | .cr Примитивүүд | Техник хангамжийн нэгж | .cl Заавар |
|---|---|---|---|---|
| **Brain 1** | Symbolic & Causal Graph | `ground_and_unify()`, `ground_to_symbol()` | Causal Graph Accelerator | `_UN`, `_SY` |
| **Brain 2** | Photonic Latent Engine | `optical_gemm(wave=...)`, `pack_wave()` | MZI Photonic Mesh | `_OP`, `_FA` |
| **Brain 3** | Reversible Memory | `backward()`, `reversible_entangle()` | Fredkin/Toffoli Reversible ALU | `_BK`, `_RF` |
| **Brain 4** | Neuromorphic STDP | `step_synaptic_plasticity()`, `subbyte_dot()` | STDP Plasticity Array | `_ST`, `_GU` |
| **Brain 5** | Superposition Planner | `quantum_collapse_eval()` | Decision Superposition Tree | `_QP`, `_CO` |
| **Brain 6** | Metacognitive Sentry | `resilient_compute`, `assert_homotopy` | Thermal Sentry Watchdog Fiber | `_SH`, `_PTC` |

---

## 3. Төслийн Бүтэц (Directory Structure)

```
new code language/
├── Cargo.toml                       # Root Rust Workspace
├── docs/
│   ├── cron_spec.md                 # CRON хэлний албан ёсны дүрэм, EBNF грамматик
│   └── isa_spec.md                  # 4D-Torus ба .cl VLIW зааврын архитектур
├── examples/
│   ├── unified_six_brain_agi.cr     # 6 Тархи бүхий жинхэнэ CRON код
│   ├── unified_six_brain_agi.cl     # Хөрвүүлэгдсэн VLIW машины код
│   └── unified_six_brain_agi_roundtrip.cr # Декомпилятороор буцааж гаргасан код
└── crates/
    ├── cronc/                       # CRON Compiler (Lexer, Parser, Checker, Codegen)
    ├── cron-vm/                     # 256-Core 4D-Torus Cycle-Accurate Simulator
    ├── cron-decompile/              # Machine-to-Blueprint Reverse Decompiler (.cl -> .cr)
    └── cron-cli/                    # Нэгдсэн CLI хэрэгсэл (`cron`)
```

---

## 4. Хэрэглэх Заавар (CLI Usage)

```bash
# 1. Синтакс болон Linear type алдааг шалгах
cargo run --bin cron -- check examples/unified_six_brain_agi.cr

# 2. .cr эх кодыг .cl VLIW машины код руу хөрвүүлэх (Compile)
cargo run --bin cron -- build examples/unified_six_brain_agi.cr -o output.cl

# 3. 256-цөмт 4D-Torus Симулятор дээр бүтэн ажиллуулах (Run & Telemetry)
cargo run --bin cron -- run examples/unified_six_brain_agi.cr

# 4. .cl машины кодыг буцаан .cr хэл рүү хөрвүүлэх (Decompile)
cargo run --bin cron -- decompile output.cl -o decompiled.cr

# 5. Нэгдсэн тестүүдийг ажиллуулах
cargo test
```
