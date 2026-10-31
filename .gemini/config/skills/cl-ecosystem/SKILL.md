---
name: cl-ecosystem
license: MIT
description: Comprehensive specification, strict syntax rules, and architectural enforcements for the CRON (CL) Zero-Verbal Programming Language and Sovereign AI Ecosystem. Use whenever authoring, reviewing, transpiling, or optimizing .cl files, BitNet 1.03-bit/1.58-bit models, DPSTA ternary packing, or Sovereign AI pipelines.
metadata:
  version: v1
  publisher: sovereign-ai
---

# CRON (CL) Zero-Verbal Constitution & Ecosystem Guide

CRON (CL) is a post-symbolic, zero-verbal mathematical programming language targeting direct silicon execution (VLIW, FPGA, CUDA PTX, ISO C23) and sub-watt neuromorphic processors without Python or C++ runtime overhead.

---

## 1. The Zero-Verbal Constitution (Үгийн Хатуу Цензур)

### Strictest Rule: ONLY 3 Cases Allow Words/Identities
Words and string literals are strictly forbidden in `.cl` source code except in exactly these three scenarios:
1. **URI Path References & Import Links:**
   - Library inclusion: `<< "libcl/math/rms_norm.cl"`
   - Safetensors tensor loading: `@"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.q_proj.weight"`
2. **Global Base Weight Bindings:**
   - Single-token or indexed mathematical base tensors: `E`, `W_q`, `W_k`, `W_v`, `W_o`, `W_g`, `W_u`, `W_d`, `W_h`, `γ_1`, `γ_2`, `γ_f`.
3. **Raw Input Tokens or Numeric Arrays:**
   - Literal token indices: `t = [1.0, 2.0, 3.0, 4.0]` or user text input `ι("prompt")`.

### BANNED IN ALL .CL CODE:
- **Procedural Variable Soup**: `H_0`, `H_norm`, `Attn_out`, `Q_proj`, `H_attn`, `H_mlp`, `H_bit`, `H_layer`, `Z_logits`, `Z_last`, `tok`, `out` are STRICTLY FORBIDDEN.
- **Traditional Keywords**: `let`, `var`, `fn`, `def`, `return`, `for`, `while`, `if`, `class`, `import` do not exist.
- **Fake Decoded Characters**: Do NOT emit dummy `δ(out)` strings; always evaluate target tensors directly into Softmax probabilities `ρ(Z)`.

---

## 2. Canonical 3-Block Architecture

Every valid `.cl` file must follow the standard 3-Block mathematical layout:

```cl
// ════════════════════════════════════════════════════════════════════════════
// BLOCK 1: IMPORT & BASE WEIGHT BINDINGS
// ════════════════════════════════════════════════════════════════════════════
<< "libcl/math/rms_norm.cl"
<< "libcl/math/swiglu_mlp.cl"

E   = @"models/sovereign_1b/model.safetensors:model.embed_tokens.weight"
W_q = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.q_proj.weight"
W_k = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.k_proj.weight"
W_v = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.v_proj.weight"
W_o = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.o_proj.weight"
W_g = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.gate_proj.weight"
W_u = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.up_proj.weight"
W_d = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.down_proj.weight"
W_h = @"models/sovereign_1b/model.safetensors:lm_head.weight"
γ_1 = @"models/sovereign_1b/model.safetensors:model.layers.0.input_layernorm.weight"
γ_2 = @"models/sovereign_1b/model.safetensors:model.layers.0.post_attention_layernorm.weight"
γ_f = @"models/sovereign_1b/model.safetensors:model.norm.weight"

// ════════════════════════════════════════════════════════════════════════════
// BLOCK 2: PURE MATHEMATICAL FUNCTIONAL COMPOSITION
// ════════════════════════════════════════════════════════════════════════════
F(X[B, D]) = X + Ψ_g(N(X, γ_1) @ W_q^T, N(X, γ_1) @ W_k^T, N(X, γ_1) @ W_v^T, 4, 1, 0.25) @ W_o^T
L(X[B, D]) = F(X) + S(N(F(X), γ_2), W_g^T, W_u^T, W_d^T)

// ════════════════════════════════════════════════════════════════════════════
// BLOCK 3: TARGET EVALUATION (CANONICAL TENSOR PROBABILITIES)
// ════════════════════════════════════════════════════════════════════════════
t = [1.0, 2.0, 3.0, 4.0]
ρ(N(L(E[t]), γ_f) @ W_h^T)
```

---

## 3. Mathematical Operator Registry

| Operator | Syntax | Description | Silicon Lowering |
| :--- | :--- | :--- | :--- |
| **GQA Attention** | `Ψ_g(Q, K, V, nh, nkv, scale)` | Grouped Query Attention | FlashAttention-3 / C23 / WGMMA |
| **Causal Attention** | `Ψ(Q, K, V, scale)` | Single-Head Causal Attention | Fused AVX / PTX |
| **Streaming Cache** | `Ψ_c(q, k, v, K, V, scale)` | Streaming KV-Cache Step | Circular SRAM Ring Buffer |
| **RMSNorm** | `N(X, γ)` | Root Mean Square LayerNorm | Vectorized InvSqrt |
| **SwiGLU MLP** | `S(X, W_1, W_2, W_3)` | Gated Feed-Forward Network | Fused Elementwise SiLU |
| **BitNet Linear** | `B(X, W, γ)` | 1.58-bit Ternary Linear | Multiplier-free Popcount MAC |
| **Ternary Quant** | `τ(W)` | Weight Quantization $\{-1, 0, +1\}$ | DPSTA 5-in-1-byte Packing |
| **Softmax** | `ρ(Z)` | Probabilities over Last Dim | Numerically Stable Max-Subtract |
| **RoPE** | `Φ(X, pos)` | Rotary Position Embedding | CORDIC / Complex Rotation |
| **Autodiff Grad** | `∇(L, W)` | Reverse-Mode Analytical Gradient | Tape-Free Backpropagation |
| **Descent Step** | `Δ(W) = W - η * ∇(L, W)` | Gradient Update Formulation | In-Place In-Memory Step |
| **Fixpoint Iter** | `W -> Δ^N` | Train Loop for $N$ Epochs | Zero-Heap Register Loop |
| **Identity / Zero**| `I(N)`, `O(M, N)` | Identity / Zero Matrix | Static Compile-Time Const |

---

## 4. Dual Compilation Profiles

CRON operates in two primary execution targets:
1. **Max-Quality (`--profile max-quality`)**:
   - Hardware: NVIDIA Hopper WGMMA, CUDA PTX, AVX-512, Full IEEE 754 Float32.
   - Purpose: Maximum mathematical precision and throughput on heavy enterprise GPUs.
2. **Min-Resource (`--profile min-resource`)**:
   - Hardware: Sub-watt silicon, VLIW, 0-Malloc Stack C23, Microcontrollers, FPGA.
   - Precision: 1.03-bit / 1.58-bit Ternary, DPSTA 5-weights-per-byte compaction.
   - Arithmetic: Multiplier-free addition/subtraction accumulation (19.9x RAM savings).
3. **Comparative Telemetry (`--compare-profiles`)**:
   - Runs both engines concurrently in isolated threads and outputs side-by-side memory/latency gain tables.

---

## 5. CLI Verification Commands

Always verify changes using:
```bash
# Evaluate .cl file in Balanced profile
cargo run -p cron-cli -- math examples/dual_profile_demonstration.cl

# Evaluate side-by-side Max-Quality vs Min-Resource
cargo run -p cron-cli -- math examples/dual_profile_demonstration.cl --compare-profiles

# Run full ecosystem test suite
cargo test -p cron-cli --test test_phase22_dual_profile
cargo test --workspace
```
