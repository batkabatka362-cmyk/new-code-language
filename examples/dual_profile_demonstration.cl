<< "libcl/math/rms_norm.cl"
<< "libcl/math/swiglu_mlp.cl"

// ════════════════════════════════════════════════════════════════════════════
// BLOCK 1: IMPORT & BASE WEIGHT BINDINGS (ZERO INTERMEDIATE PROCEDURAL VARS)
// ════════════════════════════════════════════════════════════════════════════
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
