# CRON Ecosystem — Дараагийн шатны төлөвлөгөө ба ажлуудын жагсаалт (Roadmap & Backlog)

Сүүлийн шинэчлэлт: 2026-09-26

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
