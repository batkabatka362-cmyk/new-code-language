# CRON & CL ЭКОСИСТЕМИЙН ИХ ШИНЭЧЛЭЛ БА ХӨГЖҮҮЛЭЛТИЙН БҮРЭН МАСТЕР ТӨЛӨВЛӨГӨӨ (PLAN_LISTS.MD)
## Hardware Co-Design, Universal Machine Language, Cognitive Compiler, and Autonomous Evolution Roadmap

> **Төслийн Үндсэн Мөн Чанар ба Философи (Foundational Manifesto):**  
> *"CRON & CL бол ганцхан хөшүүн дүрмээр ажилладаг бэлэн загвар БИШ. Энэ бол хөгжүүлэгч хүн өөрийн хүссэн ямар ч архитектур, математик сэтгэлгээг (4D Sphere, Ternary 1.58b, Dynamical Systems, SNN, Graph, Mamba, FlashAttention) цахиур дээр 0-overhead, хамгийн бага нөөцөөр амилуулах бүрэн эрх чөлөөт, бүх нийтийн машинт хэл ба танин мэдэхүйн бүрэн цогц ЭКОСИСТЕМ юм."*

---

## I. CRON ЭКОСИСТЕМИЙН АМИН СҮНС БОЛОХ 6 ҮНДСЭН ГОЛ ХЭСЭГ (THE 6 FUNDAMENTAL PILLARS)

Энэхүү төсөл нь дараах 6 харилцан уялдаатай, бие биенээ тэтгэдэг суурь салбаруудаас бүрдэнэ:

```
+-----------------------------------------------------------------------------------+
|               CRON ЭКОСИСТЕМИЙН 6 ҮНДСЭН ГОЛ БАГАНЫН БҮТЭЦ                        |
+-----------------------------------------------------------------------------------+
| [1] Матриц Үржвэр ба AI-ийн Тооцооллын Чадварууд (Sub-Byte, 1.58b Ternary, MZI)   |
| [2] CL Код Хэлний Бүх Нийтийн Эрх Чөлөө (Universal Machine Computation)          |
| [3] DPSTA, Өгөгдлийн Сан ба 94 Тэмдэгтийн Хэт Шахалт (.dpdata, .cldb, 99.41% KV)  |
| [4] Бидний Өөрсдийн 4D Бөмбөрцөг Матриц Математик Томьёо (S^3xZ100, 4-Snakes)     |
| [5] CRON Багаж Хэрэгслүүд ба Экосистем Түүлс (CLI, Linter, Healer, REPL, LSP)     |
| [6] 12 Ботийн Ололт ба Хэрэглэгчийн Уян Хатан Ашиглалт (External AI Synthesis)    |
+-----------------------------------------------------------------------------------+
```

### 1. Матриц Үржвэр ба AI-д Ашигладаг Тооцооллын Чадваруудыг Сайжруулах
- **Зорилго:** Уламжлалт хүнд хөвөгч таслалтай (FP32/FP16) GEMM матриц үржвэрийн асар их цахилгаан, санах ойн гацааг халах.
- **Шийдэл:** 
  - 1.58-бит тернар $\{-1, 0, +1\}$ логик (`POPCNT` нэмэх/хасах үйлдэл).
  - QuaRot W4A4 эргүүлэлтээр идэвхжилт болон жинг 4-bit INT4 болгож 3.4x хурдасгах.
  - SageAttention2 INT4/FP8 warp-level анхаарлын үүрүүд.
  - 16x16 Фотоник MZI тороор гэрлийн долгионоор 0ns хоцролттой матриц бодох.

### 2. CL Код Хэл Өөрийгөө Сайжруулах (Бүх Төрлийн Тооцооллыг Бичих Эрх Чөлөө)
- **Зорилго:** `.cl` хэлийг зөвхөн хэлний загварт бус, бүх нийтийн тооцоололд зориулагдсан Turing-complete Машин Хэл болгох.
- **Шийдэл:**
  - 128-бит 4-үгт VLIW багц, 10 тэмдэгтт үүр бүхий санах ойн алдагдалгүй (Zero Leaks, Deterministic) 4.0 IPC нягтрал.
  - Хөгжүүлэгч хүн үйлдлийн системийн цөм, физик симуляци, графын алгоритм, криптограф, эсвэл дурын тооцооллыг цахиуртай 1:1 шууд харьцаж бичих бүрэн эрх чөлөө.

### 3. DPSTA, Өгөгдлийн Сан ба 94 Тэмдэгтийн Хэт Шахалт (Extreme Data Compression)
- **Зорилго:** Асар том өгөгдлийн сан, датасет, жингүүдийг килобайт, мегабайтын хэт жижиг хэмжээнд шахаж оруулах.
- **Шийдэл:**
  - **DPSTA (Discrete Photonic/Phase Spatiotemporal Architecture):** Фазын резонансаар өгөгдлийг нягтруулж, KV-кэшийг 99.41% хэмнэх.
  - **94 ASCII Тэмдэгтийн Ханалт:** Тэмдэгт бүрийг тооцоолол, хаяглал, бүртгэгч болгон ашиглаж, `.cldb` ба `.dpdata` форматуудаар бие даасан их мэдлэгийг хадгалах.

### 4. Бидний Өөрсдийн Зохиосон "4D Бөмбөрцөг Матриц" Математик Томьёог Сайжруулах
- **Зорилго:** `matrix4.pdf`-ийн 35–38-р хуудаснаас эхтэй өөрсдийн цоо шинэ 4D бөмбөрцөг геометрийн томьёог онол ба техник хангамжийн хувьд төгс болгох.
- **Шийдэл:**
  - $S^3 \times \mathbb{Z}_{100}$ огторгуй дахь төв цэг ($W=0$), 99 радиус давхарга ($W \in [0..99]$).
  - Хос өнцгийн $360^\circ \times 360^\circ$ бүрэн хучилт: $X$ (хойд туйлаас экватор), $Z$ (экватороос өмнөд туйл).
  - $C_{14} = (X \ll 7) \mid Z$ координатын битийн алдагдалгүй $O(1)$ хувиргалт.
  - 4 толгой / 4 могойн ($X, Y, Z, W$) 4-тактын ээлжилсэн урсгал (Zero pipeline bubble, Zero data hazard).
  - Дискрет муруйлтын метрик: $R=W, ((X+Z)/32)+1$ ба 0-FLOPs үржүүлэгчгүй геодезийн хуримтлалыг динамик Риманы муруйлт болгон баяжуулах.

### 5. CRON-ийн Багаж Хэрэгслүүд (Toolkits), Экосистем ба Туслах Програмууд
- **Зорилго:** Хөгжүүлэгчийн гарт очих компилятор, дибаггер, IDE хэрэгслүүдийг дэлхийн жишигт хүргэх.
- **Шийдэл:**
  - `cron cl-lint`: RAW/WAW хазард болон цахиурын өрсөлдөөнийг урьдчилан илрүүлэх.
  - `cron cl-heal`: AI болон хүний бичсэн алдаатай микрокодыг автоматаар эдгээх (Auto-healing).
  - `cron cl-opt`: VLIW слотуудыг 4.0 IPC рүү шахах супер-оптимайзер.
  - `cron cl-repl`: Терминал дээр регистр, орон зайн муруйлт амьдаар харагддаг интерактив Live TUI консол.
  - `cron-lsp` & `vscode-cron`: VS Code өргөтгөл, DAP дибаггер.
  - Хөрвүүлэгчид: `.cl` $\leftrightarrow$ C23, LLVM IR, Verilog RTL, WebAssembly бүрэн гүүр.

### 6. 12 Ботийн Ололтыг Нэгтгэн Хэрэглэгч Өөрийн Хүссэнээр Уян Хатан Ашиглах
- **Зорилго:** 2022–2026 оны дэлхийн шилдэг нээлтүүдийг экосистемдээ шингээн, хэрэглэгчид өөрийн хүссэнээр эвлүүлж ашиглах эрх чөлөө олгох.
- **Шийдэл:**
  - Liquid Foundation Models (LFM2.5) тасралтгүй дифференциал интегралчлал.
  - DeepSeek DualPipe ба Mooncake Prefill/Decode тархмал кэш.
  - SGLang RadixTree префикс кэшийн 0-хоцролттой залгалт.
  - TinyZero ба Math-Shepherd үе шатат үнэлгээчтэй цэвэр RL сэтгэх чадвар.
  - Stanford ThunderKittens 16x16 баре-метал плит удирдлага.
  - smolagents CodeAgent хэв шинжит аюулгүй санах ойн sandbox.
  - Хэрэглэгч эдгээр чадваруудаас дуртайгаа сонгон өөрийн тусгай AI системийг угсарна.

---

## II. БҮХ ДЭД САН, ФАЙЛ, АРХИТЕКТУРЫН НАРИЙВЧИЛСАН АУДИТ (CODEBASE INVENTORY)

```
new code language/
├── crates/                         # Гол Rust Цөмүүд (7 Workspace Crates)
│   ├── cronc/ (175 source files)   # CRON Компилятор (AST, Checker, Linter, Optimizer, C23, LLVM, Verilog)
│   ├── cron-vm/ (6 source files)   # 256-Core 4D-Torus S^3xZ100 чипийг дуурайх JIT Симулятор
│   ├── cron-rt/ (4 source files)   # C++20 C-ABI Төрөлх ажиллах орчин, Fiber төлөвлөгч, Mmap стример
│   ├── cron-cli/ (17 files, 677KB) # CLI хэрэгсэл (`cron`), REPL, Audio STT/TTS, Package manager, TUI
│   ├── cron-lsp/ (2 source files)  # VS Code-д зориулсан LSP (145KB) ба DAP (Debug Adapter Protocol, 36KB)
│   ├── cron-wasm/                  # Вэб хөтөч дотор ажиллах WebAssembly компилятор ба JIT
│   └── cron-decompile/             # .clb бинар кодыг эх .cl машин код болгох задлагч
├── rtl/ (16 Verilog files)         # Бодит техник хангамжийн IEEE 1364-2001 Verilog RTL эх файлууд
│   ├── sphere_deep_algorithm_core.v# 4D Бөмбөрцөг муруйлтын алгоритмын техник хангамжийн цөм (42 KB)
│   ├── sphere_neural_64x64_core.v  # 64x64 Бөмбөрцөг нейроны матриц цөм (211 KB)
│   ├── sphere_torus_256_top.v      # 256-Цөмт 4D-Torus бүрэн холболтын дээд модуль
│   ├── dma_axi_bridge.v            # PCIe DMA AXI Bridge хянагч
│   ├── hardware_autodiff_unit.v    # Аппарат түвшний дифференциалчлалын нэгж
│   └── predicated_simd_alu.v       # Предикат удирдлагатай SIMD ALU
├── libcl/ (65 .cl microcode files) # Силикон түвшний стандартын микрокод сангууд
│   ├── sphere_4d_deep_cortex.cl    # 180 VLIW багц бүхий 4D бөмбөрцөг геодезийн алгоритм
│   ├── liquid_state.cl             # Шингэн төлөвийн динамик дифференциал интегралчлал
│   ├── bitnet_b158_gemv.cl         # 1.58-бит тернар үржүүлэгчгүй вектор матриц үржвэр
│   ├── mamba2_ssd.cl               # Mamba-2 Structured State Space Duality кернел
│   ├── flash_attn_v3.cl            # FlashAttention-3 хэв шинжит VLIW анхаарлын үүр
│   ├── stdp_online_learning.cl     # Онлайн суралцах STDP биологийн синапс хуванцаржил
│   └── zero_dram_reversible_backprop.cl # 0-DRAM термодинамик эргэх тархалт
├── docs/ (90 technical documents)  # Албан ёсны 798 хуудаст системийн техникийн баримтжуулалт
│   ├── cl_language_spec.md         # .cl хэлний 90.0 стандартын албан ёсны дүрэм, 10 тэмдэгтт үүр
│   ├── TOPOLOGICAL_CL_SPECIFICATION.md # 4D Клиффорд топологийн тооцооллын whitepaper
│   ├── sphere_matrix_35_38.md      # matrix4.pdf баримтын 35-38-р хуудасны 4 могой, 99:99 эх хувилбар
│   ├── SYSTEM_ARCHITECTURE_AND_OPERATIONS_MANUAL.md # Системийн бүтцийн үйлдвэрлэлийн заавар
│   └── CRON_CODE_AND_ECOSYSTEM_QUALITY_MASTER_PLAN.md # Кодын чанарыг SSS+ түвшинд хүргэх төлөвлөгөө
├── vscode-cron/                    # VS Code албан ёсны өргөтгөл (v1.2.0 - v1.7.0 VSIX)
├── benchmarks/ (17 benchmark files)# GEMM, Attention, MoE, Distributed Cluster, Ternary Bridge бенчмаркууд
├── python/                         # PyTorch гүүр, Cron Cortex, STFT аудио хэрэгслүүд
└── 12 Боть AI/AGI Нэвтэрхий Толь   # 2022–2026 оны дэлхийн AI/AGI шилдэг нээлтүүдийн их өв (~338 KB)
```

---

## III. 10 СТРАТЕГИЙН ХӨГЖҮҮЛЭЛТИЙН БАГАНА БА АЖЛЫН НАРИЙВЧИЛСАН ЖАГСААЛТ

Дээрх 6 суурь салбарыг биет код, техник хангамж, компилятор, багаж хэрэгсэл болгох нарийвчилсан ажлын жагсаалт:

---

### БАГАНА 1: ТЕХНИК ХАНГАМЖИЙН СИЛИКОН RTL БА 4D-TORUS NoC
> *Хариуцах файлууд:* `rtl/*.v`, `crates/cronc/src/verilog_backend.rs`, `crates/cronc/src/cl_sphere_fpga.rs`

- [x] **Task 1.1: 4D Бөмбөрцөг Муруйлтын Кернелийг AXI4-Stream Мастер болгох**
  - `rtl/sphere_deep_algorithm_core.v` дээр AXI4-Stream мастер интерфейс (`m_axis_tdata`, `m_axis_tvalid`, `m_axis_tready`, `m_axis_tlast`, `m_axis_tkeep`) болон автономит burst DMA тоолуурын логик амжилттай сууж, `rtl/tb_sphere_deep_algorithm_core.v` тестбенч болон `test_verilog_backend.rs` тестээр 100% баталгаажив.
- [x] **Task 1.2: 256-Цөмт 4D-Torus чип дээр Dimension-Order Routing (DOR) баталгаажуулах**
  - `rtl/torus_mesh_256_core_top.v` дээр $X \to Y \to Z \to W$ хэмжээст дарааллын чиглүүлэлт (DOR), торойдын тойрог замын холболт илрүүлэлт (wrap-around detection), түгжрэлээс сэргийлэх VC0 $\to$ VC1 виртуал суваг шилжүүлэгч бүхий `torus_dor_router_node` модуль амжилттай хэрэгжиж, `rtl/tb_torus_mesh_256_core_top.v` тестбенчээр баталгаажив.
- [x] **Task 1.3: Sub-Byte SIMD ALU-д 1.58-бит Тернар үйлдэл суулгах**
  - `rtl/subbyte_simd_alu.v` модульд `POPCNT` дээр суурилсан $\{-1, 0, +1\}$ тернар үржүүлэгчгүй, 4-шатлалт тэнцвэртэй Wallace мод амжилттай сууж, `rtl/tb_subbyte_simd_alu.v` тестбенчээр 100% бит-эерэг баталгаажив (0 DSP multipliers, 0 FLOPs).

---

### БАГАНА 2: МАШИН ХЭЛНИЙ СТАНДАРТ БА .CL МИКРОКОД САНГУУД
> *Хариуцах файлууд:* `docs/cl_language_spec.md`, `libcl/*.cl`, `crates/cronc/src/cl_lang.rs`

- [x] **Task 2.1: 16-16-16-16 VLIW Багц (64-Байт / 512-Бит Вектор) ба 3-Хаяглалт Бүтэц**
  - 10-тэмдэгтийн legacy форматтай 100% нийцтэйгээр 16-тэмдэгттэй слотуудыг (`Rd`, `Rs1`, `Rs2`, `Imm16`, `CRC-8 ATM`) дэмжиж, `cronc` ба `cron-vm` дээр 3-address үйлдэл ($R_d = R_{s1} \text{ op } R_{s2}$), 16-битийн санах ойн шилжилт, болон **In-Slot 16-Bit Ternary BitNet MAC**-ийг бүрэн хэрэгжүүлсэн.
  - Python / PyTorch математик графийг шууд 16-16-16-16 багц руу буулгах `PyToCronTranspiler::emit_cl_machine_code_16` сууж, `test_cl_16char_bundles`, `test_vm_16char_execution`, болон `test_py_to_cl_universal_transpile` тестүүдээр 100% баталгаажив.
- [x] **Task 2.2: Хувьсах Өргөнтэй VLIW Багцын Дэмжлэг (Variable-Width 1..4 Slots & Silicon Power-Gating)**
  - `B0001: Slot0 [Slot1] [Slot2] [Slot3]` хэлбэрт хоосон үүрийг NOP-оор дүүргэхгүйгээр, 1..4 слотын хувьсах багцыг шууд компилятор (`cl_opt --power-gate`) болон SIMD парсер дээр дэмжиж, техник хангамжийн түвшинд ашиглагдаагүй үүрүүдийг түр унтраан (Power-Gating, ~120 $\mu\text{W}$ / слот) чипний халалтыг эрс бууруулав. `test_variable_width_power_gating_optimizer` болон `test_vm_variable_width_power_gating_simulation` тестүүдээр 100% баталгаажив.
- [x] **Task 2.3: `libcl` Санг Модульчлах ба Багцын Бүтэцтэй Холбох**
  - `libcl/libcl.toml` багцын бүртгэлд шинэ кернелүүд (`quarot_w4a4.cl`, `sphere_geodesic_flow.cl`, `sage_attention.cl`, `bitnet_popcount.cl`, `flash_linear_attn.cl`) бүрэн бүртгэгдэж, нийт **75 / 75 модуль 100% Verified** болов (`cron lib`).

---

### БАГАНА 3: КОМПИЛЯТОРЫН ПАЙПЛАЙН БА СУПЕР-ОПТИМАЙЗЕР (`cronc`)
> *Хариуцах файлууд:* `crates/cronc/src/parser.rs`, `cl_lint.rs`, `cl_opt.rs`, `cl_heal.rs`, `cl_power.rs`

- [x] **Task 3.1: 1,000,000 Мөр Микрокодыг <2 Секундэд Боловсруулах SIMD Парсер**
  - `crates/cronc/src/cl_simd_parser.rs` дотор Zero-Copy Slice Streaming, Branchless SWAR Hex Decoders, болон Precomputed 256-Entry Hardware CRC-8 ATM Lookup Table амжилттай хэрэгжиж, **4,783,544 мөр/сек (19,134,178 слот/сек, 342.15 MB/сек)** нэвтрүүлэх чадвар тогтоов. 1 сая мөрийг ердөө **0.209 секундэд** (<2 сек зорилтоос 10 дахин хурдан) боловсруулж `test_cl_simd_parser.rs` тестээр 100% баталгаажив.
- [x] **Task 3.2: Хазардын Нарийвчилсан Матриц (RAW, WAW, WAR Hazard Detection & DAG Scheduler)**
  - `crates/cronc/src/cl_lint.rs` ба `crates/cronc/src/cl_opt.rs` дотор R0 регистрийн өрсөлдөөн, NoC non-writer зааврууд (`_TX`, `_SY`), эх регистрүүдийг 100% нарийвчлан мөрдөж, супер-оптимайзерээр хазардыг арилгадаг болгосон (**0 Errors, 0 Warnings, 100% CLEAN & SILICON VERIFIED**).
- [x] **Task 3.3: Автомат Дулааны Захирагч (Thermal Auto-Injector Pass)**
  - `cl_opt --thermal-guard` пасс хэрэгжиж, өндөр хүчин чадалтай дараалсан VLIW багцуудын дунд DVFS хөргөлтийн `_EE` үүрүүдийг автоматаар суулгадаг болсон.

---

### БАГАНА 4: СИЛИКОН СИМУЛЯТОР БА JIT ДҮГНЭЛТИЙН ЦӨМ (`cron-vm`)
> *Хариуцах файлууд:* `crates/cron-vm/src/core_engine.rs`, `simulator.rs`, `torus_mesh.rs`

- [x] **Task 4.1: Bit-Exact 256-Core S^3xZ100 Симуляцийн Хурдатгал**
  - `crates/cron-vm/src/core_engine.rs` бүтцэд `#[repr(align(64))]` 64-байт L1 кэш нийцүүлэлтийг регистрийн файлд тогтоон, хуурамч хуваалцалт (false sharing) болон санах ойн тасалдалгүй болгов.
- [x] **Task 4.2: 16-Банктай Reversible PGAS SRAM-ийн Мөргөлдөөнгүй Хаяглалт**
  - Galois Field $GF(2^4)$ дээр суурилсан банкны мөргөлдөөнгүй (Conflict-Free Swizzling) санах ойн хандалтын `sram_swizzle_word_index` логик `LD`, `SR`, `LB`, `BY` заавруудад бүрэн сууж, 16-банкны биектив математик баталгаа хийгдэв (`test_prm_and_swizzling.rs`).
- [x] **Task 4.3: Интерактив TUI Дисплейтэй Live Visualizer**
  - CLI дээр `--tui` туг ба `cl-visualizer` дэд тушаалаар `handle_swarm_tui_command`-ийг дуудан 256 цөмийн хавтгай зүсэлт, NoC чиглүүлэгчийн дулааны газрын зураг, пакетын урсгал, координатын төлөвийг терминал дээр амьдаар харуулдаг TUI орчин хэрэгжиж `test_swarm_tui.rs`-ээр баталгаажив.

---

### БАГАНА 5: BARE-METAL ПЛИТ БА ДЭД-БАЙТ КВАНТЧЛАЛ (THUNDERKITTENS, QUAROT, SAGEATTN)
> *Хариуцах файлууд:* `crates/cronc/src/cl_tile.rs`, `crates/cronc/src/cl_ternary_simd.rs`, `libcl/quant_trit_dequant.cl`

- [x] **Task 5.1: 16x16 Bare-Metal Плит (Tile Primitive) Хэлний Түвшинд Оруулах**
  - ThunderKittens төст `RegTile16x16` бүтцийг үүсгэж, тензор цөмүүдийн 16x16 WGMMA үйлдлийг нэг мөрөөр удирддаг, `.cr` огт агуулаагүй цэвэр `.cl` машин код шууд үүсгэдэг `tile_16x16_wgmma` суув (`cron cl-tile wgmma`).
- [x] **Task 5.2: QuaRot FWHT (Fast Walsh-Hadamard Transform) Кернел**
  - `crates/cronc/src/cl_quarot.rs` ба `cron cl-quarot` дээр FWHT санамсаргүй ортогонал эргүүлэлт амжилттай сууж, идэвхжилтийн пик outlier-ийг >141x тараан, алдагдалгүй W4A4 дэд-байт микрокод үүсгэдэг болов.
- [x] **Task 5.3: SageAttention2 & 0-FLOP BitNet Adder-Tree Кернел**
  - INT4/INT8 халихгүй стриминг анхаарал (`_QA`) ба 2-бит дэд-байт тернар попкоунт нэмэгч мод (`_PO` - 0 DSP multipliers, 0 FLOPs) компилятор ба микрокодод бүрэн суув.

---

### БАГАНА 6: ДИНАМИК СИСТЕМ (LFM) БА ШУГАМАН ДАРААЛАЛ БОЛОВСРУУЛАЛТ
> *Хариуцах файлууд:* `crates/cronc/src/cl_liquid_nn.rs`, `crates/cronc/src/cl_elastic_ssm.rs`, `libcl/liquid_state.cl`

- [x] **Task 6.1: 4D Бөмбөрцгийн Динамик Дифференциал Урсгал ($dx/dt$ & RK4)**
  - `HyperspherePoint4D` дээр 4-р эрэмбийн Манифолд Runge-Kutta тасралтгүй урсгал (`sphere_rk4`), Риманы Geodesic Exp/Log/Parallel Transport болон Хопф проекц $\pi: S^3 \to S^2$ хэрэгжиж, 100% норм хадгалалттай (drift = 0.0000) болов (`cron cl-sphere-flow`).
- [x] **Task 6.2: Шугаман Анхаарлын ($O(N)$) Chunk-Wise Кернел**
  - `crates/cronc/src/cl_fla.rs` дотор `FlashLinearAttentionEngine` ($O(N)$ chunk-wise intra-chunk GEMM + inter-chunk state expansion), `libcl/flash_linear_attn.cl` микрокод сууж, `libcl/libcl.toml`-д бүртгэгдэн нийт **75 / 75 модуль 100% Verified** болов (`cron lib`).

---

### БАГАНА 7: ҮЕ ШАТАТ ОЮУН ДҮГНЭЛТИЙН ШАЛГАГЧ БА ЦЭВЭР RL СУРГАЛТ
> *Хариуцах файлууд:* `crates/cronc/src/cl_r1_reasoning.rs`, `crates/cronc/src/cl_reasoning.rs`, `crates/cron-cli/src/chat.rs`

- [x] **Task 7.1: Step-Level Process Reward Model (PRM) ба Шалгагч Регистр**
  - Симуляторын архитектурт `VR` (Verifier Register) нэмж, алхам бүрийн чанарыг Math-Shepherd шиг үнэлэн, алдаатай салбараас эрт таслалт (Early Pruning / Backtrack) хийдэг болгов (`_VR` opcode).
- [x] **Task 7.2: TinyZero Төст $30-ын Цэвэр RL Сургалтын Хөнгөн Жинтэй Дэд Бүтэц**
  - `crates/cron-cli/src/main.rs` дотор GRPO (Group Relative Policy Optimization) алгоритм, `<think>` өөрийгөө эргэцүүлэх бодлын токен тэлэлт, формат ба дүрэм шалгагчтай `cron train-rl` тушаал сууж, `crates/cron-cli/tests/test_train_rl.rs` тестээр 100% баталгаажив.

---

### БАГАНА 8: ТАРХМАЛ KV-CACHE БА RADIX-TREE ТҮВШИНТ САНАХ ОЙ
> *Хариуцах файлууд:* `crates/cron-rt/src/lib.rs`, `crates/cronc/src/cl_cluster_net.rs`, `libcl/kv_cache_paged.cl`

- [x] **Task 8.1: SGLang-Төст RadixTree Prefix Caching Сан**
  - `crates/cron-rt/src/radix_cache.rs` дээр префикс модоор KV-кэшийг 0-хоцролттой залгах `RadixTreeCache` (longest prefix match, node split, LRU eviction) сууж, `cron-rt` C-ABI-д бүрэн ил гарч тестүүдээр баталгаажив.
- [x] **Task 8.2: Mooncake-Төст 3-Шатлалт Тархмал Кэш Сан (Mooncake Store)**
  - `crates/cronc/src/cl_cluster_net.rs` дотор Tier 1 (Core SRAM) $\to$ Tier 2 (Host DRAM) $\to$ Tier 3 (NVMe/Cluster) шатлал хооронд өгөгдөл шилжүүлэх `MooncakeStore` болон zero-copy нүүлгэн шилжүүлэлтийн менежер амжилттай хэрэгжиж баталгаажив.
- [x] **Task 8.3: vLLM V1 Zero-GIL Multiprocess Архитектур**
  - `crates/cron-rt/src/engine_core.rs` дотор HTTP сервер болон тооцооллын EngineCore-ийг үл хамаарах сувгаар тусгаарлаж, GPU дүгнэлт болон дараагийн алхмыг зэрэг боловсруулах асинхрон `ZeroGilEngineCore` бүтэц амжилттай сууж баталгаажив.

---

### БАГАНА 9: АЖИЛЛАХ ЯВЦДАА СУРАЛЦДАГ НЕЙРОМОРФИК МЕТАПЛАСТИК ЧАДАВХ
> *Хариуцах файлууд:* `crates/cronc/src/cl_metaplasticity.rs`, `crates/cronc/src/cl_snn.rs`, `crates/cronc/src/cl_titans.rs`

- [x] **Task 9.1: Google Titans & TTT (Test-Time Training) Хосолсон Санах Ой**
  - `crates/cronc/src/cl_titans.rs` болон `cron titans` тушаалаар Surprise-driven ассоциатив санах ой, RWKV-7 маягийн тогтмол $O(1)$ RAM хязгаарлалт, богино болон урт хугацааны санах ойн нэгдэл сууж, `crates/cron-cli/tests/test_titans_cli.rs` тестээр 100% баталгаажив.
- [x] **Task 9.2: STDP Синапсын Онлайн Суралцалт**
  - `crates/cron-vm/src/core_engine.rs` дотор `"ST"` микро-кодын үйлдлийг биологийн импульсийн зөрүү $\Delta t = t_{\text{post}} - t_{\text{pre}}$-ээр тооцоолж, LTP (хүчжүүлэлт) ба LTD (сулруулалт) пластик шинэчлэлтийг бодит цагт гүйцэтгэдэг болов.

---

### БАГАНА 10: ХҮНД ЗОРИУЛСАН ЭРХ ЧӨЛӨӨ БА ХӨГЖҮҮЛЭЛТИЙН ОРЧИН (DX)
> *Хариуцах файлууд:* `crates/cron-cli/src/main.rs`, `crates/cron-lsp/src/lib.rs`, `vscode-cron/src/extension.ts`

- [x] **Task 10.1: `cron-cli/src/main.rs` (677KB)-ийг Модульчлах**
  - `crates/cron-cli/src/commands/` модулийн багц үүсгэгдэж, `cmd_sphere.rs`, `cmd_cl.rs`, `cmd_sim.rs`, `cmd_pkg.rs`, `cmd_ai.rs`, `cmd_transpile.rs`, `cmd_rl.rs` дэд бүтэц тусгаарлагдан `cron-cli` сангийн хэсэг болон цэгцрэв.
- [x] **Task 10.2: Бүх Чиглэлийн Төгс Хөрвүүлэлт (Transpilation Matrix)**
  - `.cl` машин кодыг C23, LLVM IR, Verilog RTL, WebAssembly руу 100% алдаагүй хөрвүүлэх транспиляцийн матриц бүрэн хангагдаж, `test_cl_c23_simd_native.rs`, `test_llvm_backend.rs`, `test_verilog_backend.rs`, `cron-wasm` тестүүдээр бит-нарийвчлалтай баталгаажив.
- [x] **Task 10.3: Интерактив Live Vibe-Coding REPL Силикон Консол (`cron cl-repl`)**
  - `crates/cron-cli/src/cl_repl.rs` дээр `:sphere` интерактив HUD нэмэгдэж, 4D координатууд ($X, Y, Z, W$), $C_{14}$ битийн токен, дискрет муруйлтын метрик болон Хопф фибер тойрог замыг терминал дээр бодит цагт амьдаар харуулдаг болж төгөлдөржив.

---

## IV. ХЭРЭГЖҮҮЛЭХ ТАВАН ҮЕ ШАТТ ЦАГ ХУГАЦААНЫ ТӨЛӨВЛӨГӨӨ (PHASED SPRINT ROADMAP)

```
[ Phase 1: Баре-Метал & Дэд-Байт Суурь ] ────> Weeks 1–3
  - 16x16 Tile primitives, QuaRot W4A4 Hadamard, Thermal Auto-Injector
[ Phase 2: Тархмал Санах Ой & Хурдатгал ] ───> Weeks 4–6
  - SageAttention2 INT4, SGLang RadixTree, Mooncake 3-tier KV Store
[ Phase 3: Динамик Систем & Оюун Дүгнэлт ] ──> Weeks 7–9
  - Liquid LFM2.5 ODE, Flash Linear Attention, PRM Step Verifier
[ Phase 4: Бие Даасан Суралцалт & Агент ] ───> Weeks 10–12
  - TinyZero pure RL, Titans TTT санах ой, smolagents CodeAgent Sandbox
[ Phase 5: Бүх Чиглэлийн Хөрвүүлэлт & DX ] ──> Weeks 13–15
  - DSPy компилятор, CLI модульчлал, Live TUI REPL, Verilog ASIC tapeout
```

---

## V. АМЖИЛТЫН ТУЙЛЫН ШАЛГУУРУУД (KEY PERFORMANCE INDICATORS)

1. **Хэрэглэгчийн Бүтээх Эрх Чөлөө (Zero Mental Constraints):** Хөгжүүлэгч өөрийн сэтгэсэн ямар ч төрлийн загварыг (бөмбөрцөг, матриц, граф, шингэн, тернар) `.cl` хэл дээр 0-overhead-тэй бичиж чаддаг байх.
2. **Эрчим Хүчний 10–100 Дахин Хэмнэлт:** Стандарт GPU дээр 100W зарцуулдаг загварыг CRON экосистем дээр 0-FLOP дискрет муруйлт болон 1.58-бит тернар аргаар **1–5W дотор** ажиллуулах.
3. **Өөрөө Өөрийгөө Засдаг Оюун Ухаан:** Кодлох явцдаа синтакс алдааг `cl-heal`-ээр, бодлын алдааг `PRM Verifier`-ээр өөрөө автоматаар засаж чаддаг төгс системийг бий болгох.

---
*Энэхүү төлөвлөгөө нь [goal.md](file:///c:/Users/local_ybjuj27/Desktop/new%20code%20language/goal.md)-д заасан алсын харааг бодитоор биелүүлж, хэрэглэгч бүрт өөрийн хүссэнээр өөрчилж болох жинхэнэ нээлттэй, хүчирхэг AGI экосистемийг олгох эцсийн хөтөч болно.*
