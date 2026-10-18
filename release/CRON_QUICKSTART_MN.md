# CRON: Хөгжүүлэгчийн Түргэн Эхлэх Гарын Авлага (Quickstart Guide)

**Хувилбар:** 2.0 (SSS+ Tier Verified)  
**Зорилтот Систем:** 256-Цөмт 4D-Torus, BitNet 1.58b Тернари Тооцоолол, 128-бит VLIW Машин Хэл (`.cl`)

---

## 1. Системийн Шаардлага ба Суулгах

CRON төслийг Rust хөгжүүлэлтийн орчин болон Python ашиглан 1 минутад ажиллуулах боломжтой:

```bash
# 1. Төслийн үндсэн хавтас руу орох
cd "c:/Users/local_ybjuj27/Desktop/new code language"

# 2. Бүх сан болон CLI хэрэгслийг шалгах
cargo test --workspace

# 3. CRON нэгдсэн CLI хэрэгслийг угсрах
cargo build --release -p cron-cli
```

---

## 2. Гол Командуудын Лавлах

### 2.1 Машин Кодыг Шалгах, Оновчлох, Ажиллуулах
```bash
# .cl микрокодын синтакс болон аюулгүй байдлыг шалгах
cron cl-lint examples/cl/mini_transformer_attention.cl

# VLIW слотуудыг нягтруулж 4.0 IPC болгон оновчлох
cron cl-opt examples/cl/mini_transformer_attention.cl -o optimized.cl

# JIT хөдөлгүүрээр 100% bit-exact шууд ажиллуулах
cron cl-run examples/cl/mini_transformer_attention.cl

# Бие даасан ISO C23 эх код руу хөрвүүлэх
cron cl-c23 examples/cl/mini_transformer_attention.cl -o kernel.c

# C23 эх кодыг хост дээр шууд компилдож ажиллуулах
cron cl-native examples/cl/mini_transformer_attention.cl -r
```

### 2.2 4D-Torus ба Силикон Профайлер
```bash
# 4D-Torus 9-порт виртуал сувгийн чиглүүлэлтийг дууриалгах
cron cl-router sim

# 16-банк санах ойн Galois Field GF(2^4) мөргөлдөөнгүй байдлыг шалгах
cron cl-memcheck examples/cl/mini_transformer_attention.cl

# Ландауэрийн термодинамик дулаан ба DVFS горимыг хэмжих
cron cl-power examples/cl/mini_transformer_attention.cl

# 256-цөмт цахиурын PPA (Power, Performance, Area) гүйцэтгэлийг шинжлэх
cron cl-perf examples/cl/mini_transformer_attention.cl
```

### 2.3 AI Загвар ба Сэтгэхүйн Хөдөлгүүр
```bash
# BitNet 1.58b тернари LLM микро-кернел үүсгэх
cron cl-infer synth --layers 2 --heads 4 --dim 64

# DeepSeek-R1 маягийн <think> бодолтын зам үүсгэх (MCTS)
cron reason "Prove that sqrt(2) is irrational via 4D Clifford space" --steps 3

# OpenAI-тай нийцтэй өндөр ачааллын REST сервер асаах (16MB RAM)
cron serve --port 8080
```

### 2.4 Дэлхийн Экосистемтэй Холбох (Interop)
```bash
# C++20 single-header сан гаргах
cron cl-bridge export-c-headers -o cron.hpp

# C# .NET 8/9 холболтын эх код гаргах
cron cl-bridge export-csharp -o CronNative.cs

# Python DLPack гүүр гаргах
cron cl-bridge export-python -o cron_runtime.py

# Веб хөтөч дээр ажиллах WebAssembly 3D симулятор үүсгэх
cron playground
```

---

## 3. Эрдэм Шинжилгээний Нийтлэлүүд
- **[CRON ArXiv Paper (Markdown)](CRON_RESEARCH_PAPER_ARXIV.md)**
- **[CRON ArXiv Paper (LaTeX)](arxiv_paper.tex)**
- **[CRON Benchmark Whitepaper](CRON_BENCHMARK_WHITEPAPER.md)**
