# CRON Ecosystem — Хийгдсэн ажлууд ба Дараагийн шатны төлөвлөгөө (Worklog & Roadmap)

Сүүлийн шинэчлэлт: 2026-09-29

---

## 0. Сүүлд хийгдсэн ажлууд (Recently Completed Tasks & Verification)

### A. LSP 3.17 & VSCode Өргөтгөлийн сайжруулалт
- [x] **SignatureHelp Provider (`textDocument/signatureHelp`)**:
  - `(` болон `,` тэмдэгтүүдээр автоматаар ажилладаг.
  - 10 суулгагдсан когнитив функцуудын (`pack_wave`, `compute_attention_head`, `step_synaptic_plasticity`, `ground_and_unify`, `assert_triple`, `consume` гэх мэт) параметр, тайлбар, идэвхтэй параметрийн байрлалыг тодорхойлно.
- [x] **DocumentSymbol Provider (`textDocument/documentSymbol`)**:
  - `.cr`: `def`, `struct`, `trait`, `brain`, `region`, `impl` блокуудыг Outline модонд гаргана.
  - `.cl`: `@CORE` заавар болон `B0000:` VLIW bundle шошгуудыг бүрэн таньж харуулна.
- [x] **GoToDefinition Provider (`textDocument/definition`)**:
  - Функц, бүтэц, төлөв, brain, impl тодорхойлолт болон `.cl` шошгуудын эх үүсвэр рүү (`F12`) үсрэх чадвар.
- [x] **Semantic Tokens Provider (`textDocument/semanticTokens/full`)**:
  - LSP 3.17 delta-encoding стандартаар AST түвшний өнгө ялгалт хийсэн:
    - Linear Affine `lin` хувьсагчдыг `readonly` горимоор тодотгох.
    - Когнитив блокууд (`brain`, `resilient_compute`, `region`).
    - Тоног төхөөрөмжийн регистрүүд (`R0..R15`, `V0..V7`, `A0..A3`), micro-op командууд (`ADD`, `FMA`, `SWIZZLE`, `ROUTE_DOR`, `SEND`, `RECV`).
- [x] **`is_cl_document` засав**:
  - `.cr` дотор `brain` блок байх үед алдаатайгаар `.cl` гэж таньдаг байсан алдааг `B####` толгой загварын шалгалтаар засав.
- [x] **VSCode Extension v1.2.0 & VSIX багцлалт**:
  - `vscode-cron/cron-lang-1.2.0.vsix` багцыг бүрэн хамаарлуудтай нь (`vscode-languageclient` 9.0+) хамт бүтээв.
  - Нийт **15/15 LSP тест** (`cargo test -p cron-lsp`) 100% ногоон.

### B. Компилятор (`cronc`) & Оношлогооны нарийвчлал
- [x] **Синтакс алдааны мөр, баганыг нарийн заах (`extract_line_col_from_err`)**:
  - Өмнө нь бүх синтакс алдааг `(1, 1)` гэж харуулдаг байсныг засаж, Parser болон Lexer-ийн бодит `(line, col)` байрлалыг гарган улаан зураасаар заадаг болов.
- [x] **Workspace-wide тестүүд**: `cargo test --workspace` төслийн бүх crate-ууд 0 алдаатай бүрэн давсан.

### C. Тоног төхөөрөмж ба Микро-архитектурын баталгаажуулалт (`homopolymer_quantum_agent.cl`)
- [x] **Edge Case & Latch Integrity**:
  - `_HL00$000!` (Halt/Commit) мөчид:
    - Write-Enable шугамыг тасалж (`WE=0`), Crossbar bus-ийг ground хийнэ.
    - `_88` заавраар 0-cycle arena санах ойг бүрэн цэвэрлэж (`quench`), хоцрогдсон өгөгдлийг 0 болгоно.
    - `_bb` заавраар 256-core NoC буферүүдийг 100% суллаж түгжинэ.
- [x] **Thermal & Bandwidth Saturation (`cron cl-power`)**:
  - Landauer энтрөпийн хяналт ($E = N \cdot k_B \cdot T \cdot \ln 2$): 128 бит арчигдаж, $3.675 \times 10^{-19}\text{ J}$ дулаан ялгарсан.
  - Чипийн дулаан: $T_j = 25.8^\circ\text{C}$ (95.8% TDP headroom).
  - 16-Bank PGAS дээр $GF(2^4)$ Galois swizzling ажиллаж 160 GB/s зурвасын өргөнд 0 bank conflict баталгаажсан.
- [x] **Stress Determinism (`cron cl-cosim`)**:
  - `ClJitCore` лавлагаа програм болон нийлэгждэг Verilog RTL (`cksl_core`) хоорондын parity: 3 такт, 3 тохирол, 0 зөрүүтэйгээр **100% Bit-Exact** батлагдсан.

---

## 1. Compiler & Diagnostics (`cronc`)
- [ ] **Multi-Error Semantic Recovery**:
  - `SemanticChecker`-ийг эхний алдаан дээр зогсохгүйгээр дараагийн функц, илэрхийллүүдийг үргэлжлүүлэн шалгадаг (`Vec<TypeError>` цуглуулдаг) горимтой болгох.
  - Олон төрлийн алдааг нэг дор илрүүлж LSP-д дамжуулах.
- [ ] **Type Inference & Inlay Hints Metadata**:
  - `let` хувьсагчдын тодорхой бус төрлийг автоматаар тооцоолж LSP-д `InlayHint` өгөгдөл болгон бэлдэх.

---

## 2. LSP & Developer Tooling (`cron-lsp`, `vscode-cron`)
- [ ] **Inlay Hints Provider (`textDocument/inlayHint`)**:
  - Хувьсагчийн төрөл (`: f32`, `: wave_t`), болон linear resource төлөвийг редактор дээр шууд харуулах.
- [ ] **Code Action / Auto-Fix интеграци**:
  - `E0003` (Unconsumed linear type) гарсан үед шууд `consume(...)` автоматаар нэмэх quick-fix санал болгох.
  - `CL001..CL005` VLIW slot зөрчлийг `Autonomous Vibe-Loop Heal`-ээр нэг товшилтоор засах.
- [ ] **Debug Adapter Protocol (DAP)**:
  - `.cr` болон `.cl` програмыг bundle-by-bundle алхам алхмаар trace хийх, регистр (`R0..R15`, `V0..V7`)-ийн утгыг шалгах debugger.
- [ ] **VS Code 4D-Torus Mesh Visualizer**:
  - VS Code дотор 256/65536 core wafer mesh-ийн температур, пакетын урсгалыг харуулах Webview самбар.

---

## 3. Hardware Simulator & Wafer-Scale Engine (`cron-vm`, `cron-rt`)
- [ ] **Wafer-65536 Mesh Routing Optimization**:
  - 8D координатын систем (`Coord8D`) дээрх DOR чиглүүлэлтийг сайжруулах, пакет мөргөлдөөнийг 0 болгох.
- [ ] **Live Telemetry & Thermal Throttling Simulator**:
  - Бодит цагийн дулааны динамик, 4D тэнхлэгээр (`X+, Y-, Z+, W-`) багцыг автоматаар тойруулах resilient compute загварчлал.

---

## 4. Transpilers & Hardware Backends
- [ ] **C23 Transpiler (`cron c23`)**:
  - GCC -O3 болон Clang 18+ дээр BitNet ternary GEMM болон photonic SIMD үйлдлүүдийн биелэлтийн хурдыг баталгаажуулах.
- [ ] **LLVM JIT Backend (`cron jit`)**:
  - Dynamic AGI mind loop болон self-rewriting кодуудыг native машин код руу шууд хөрвүүлж биелүүлэх хугацааг 0 latency болгох.
- [ ] **Verilog RTL Synthesis**:
  - Силикон FPGA / ASIC синтезийн testbench-үүдийг бэлтгэх.

---

## 5. Мэдээллийн энтрөпи, санах ой ба архитектурын тогтвортой байдал (Information Capacity & Stability)
- [ ] **Cascading Drift Watchdog & Lyapunov Stabilization**:
  - RMSNorm тоног төхөөрөмжийн зааврын калибрацийг автоматжуулах, 4 тактын горимд олон давталтын дараа вектор задрахаас сэргийлсэн $L_2$ норм тогтворжуулагч хяналтын механизмыг Brain 6 sentry-д нэмэх.
- [ ] **Attractor Trapping & State Saturation Prevention**:
  - `cl_metaplasticity` (BCM дүрэм) ба Homeostasis динамик босгыг компиляторын VLIW төлөвлөгчид нэгтгэж, тодорхой битийн цикл түгжрэлд (loop trapping буюу $0, 2^{64}-1$ рүү унах) орохоос сэргийлэх deterministic entropy injection нэвтрүүлэх.
- [ ] **RAW Hazard & Software Pipelining Zero-Bubble Verification**:
  - `cronc::scheduler` дээр Modulo Scheduling шалгуурыг сайжруулж, хамааралтай үйлдлүүдийг автоматаар interleaving / multi-accumulator хийн, OoO процессорын дамжлагын саатлыг (pipeline bubble) 0 болгож IPC-ийг дээд цэгт хүргэх.
- [ ] **HDC Role-Filler Binding Scaling & Capacity Benchmark**:
  - 2048-бит болон 10,000-битийн гипервектор VSA санах ойн багтаамжийг 10 саяас 1 тэрбум баримтад суперпозици хийх үеийн SNR (Signal-to-Noise Ratio) болон тайлах (unbinding) нарийвчлалыг стресс тестээр баталгаажуулах.
- [ ] **Hybrid Context Memory (Streaming KV-Cache + Elastic HiPPO SSM)**:
  - 32,000–128,000 токен контекст цонхны үед өмнөх мэдээлэл арчигдахгүй (overwritten) байх, $O(N)$ шугаман санах ойн багтаамжийг хадгалах автомат шалгуурын тест нэмэх.
- [ ] **OS Jitter & Deterministic Tail-Latency Profiler**:
  - `cron bench --bare-metal` команд нэмж, Core Isolation (`isolcpus`), 0-cycle hardware arena горимд интеррапт болон context switch-ээс шалтгаалах детерминистик бус саатлыг (p99.99 latency) хэмжих.
