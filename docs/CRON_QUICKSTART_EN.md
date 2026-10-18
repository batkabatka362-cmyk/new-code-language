# CRON: Developer Quickstart Guide

**Version:** 2.0 (Verified SSS+ Tier Industrial Systems & Language Research)  
**Target Architecture:** 256-Core 4D-Torus, BitNet 1.58b Multiplier-Free Silicon, 128-bit VLIW Microcode (`.cl`)

---

## 1. Quick Installation & Verification

Build and verify the full workspace in under one minute:

```bash
# 1. Navigate to workspace root
cd "c:/Users/local_ybjuj27/Desktop/new code language"

# 2. Run full test suite across all 7 workspace crates
cargo test --workspace

# 3. Build optimized release binary of the CRON CLI
cargo build --release -p cron-cli
```

---

## 2. CLI Command Quick Reference

### 2.1 Microcode Linting, Optimization & Native Execution
```bash
# Lint .cl microcode for structural integrity and hazards
cron cl-lint examples/cl/mini_transformer_attention.cl

# Super-optimize VLIW slots to reach peak 4.0 IPC
cron cl-opt examples/cl/mini_transformer_attention.cl -o optimized.cl

# Direct in-memory execution via sub-microsecond JIT
cron cl-run examples/cl/mini_transformer_attention.cl

# Lower .cl microcode to standalone ISO C23 source
cron cl-c23 examples/cl/mini_transformer_attention.cl -o kernel.c

# Compile and run natively via host C compiler
cron cl-native examples/cl/mini_transformer_attention.cl -r
```

### 2.2 4D-Torus Silicon & Memory Telemetry
```bash
# Simulate 4D-Torus 9-port virtual channel router
cron cl-router sim

# Verify 16-bank PGAS SRAM conflict-free invariance
cron cl-memcheck examples/cl/mini_transformer_attention.cl

# Compute Landauer thermodynamic dissipation and DVFS state
cron cl-power examples/cl/mini_transformer_attention.cl

# Profile 256-core PPA (Power, Performance, Area)
cron cl-perf examples/cl/mini_transformer_attention.cl
```

### 2.3 Foundation Models & Cognitive Engines
```bash
# Synthesize BitNet 1.58b ternary LLM micro-kernel
cron cl-infer synth --layers 2 --heads 4 --dim 64

# Generate DeepSeek-R1 style deductive <think> trajectory
cron reason "Prove that sqrt(2) is irrational via 4D Clifford space" --steps 3

# Launch OpenAI-compatible REST server (/v1/chat/completions)
cron serve --port 8080
```

### 2.4 Universal Cross-Platform Interoperability
```bash
# Export single-header C++20 library
cron cl-bridge export-c-headers -o cron.hpp

# Export C# .NET 8/9 P/Invoke bindings
cron cl-bridge export-csharp -o CronNative.cs

# Export Python DLPack zero-copy tensor runtime
cron cl-bridge export-python -o cron_runtime.py

# Generate standalone single-file WebAssembly playground
cron playground
```

---

## 3. Scientific Whitepapers & Documentation
- **[CRON ArXiv Paper (Markdown)](CRON_RESEARCH_PAPER_ARXIV.md)**
- **[CRON ArXiv Paper (LaTeX)](arxiv_paper.tex)**
- **[CRON Empirical Benchmark Whitepaper](CRON_BENCHMARK_WHITEPAPER.md)**
