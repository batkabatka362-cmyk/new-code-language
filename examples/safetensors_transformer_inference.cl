// ════════════════════════════════════════════════════════════════════════════
// CRON ZERO-WORD REAL SAFETENSORS TRANSFORMER INFERENCE (GQA + SwiGLU)
// ════════════════════════════════════════════════════════════════════════════

<< "libcl/math/rms_norm.cl"
<< "libcl/math/swiglu_mlp.cl"

// [1] Жинхэнэ Safetensors загварын жингүүдийг ачаалах
E = @"models/sovereign_1b/model.safetensors:model.embed_tokens.weight"
γ_1 = @"models/sovereign_1b/model.safetensors:model.layers.0.input_layernorm.weight"
W_q = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.q_proj.weight"
W_k = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.k_proj.weight"
W_v = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.v_proj.weight"
W_o = @"models/sovereign_1b/model.safetensors:model.layers.0.self_attn.o_proj.weight"

γ_2 = @"models/sovereign_1b/model.safetensors:model.layers.0.post_attention_layernorm.weight"
W_gate = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.gate_proj.weight"
W_up = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.up_proj.weight"
W_down = @"models/sovereign_1b/model.safetensors:model.layers.0.mlp.down_proj.weight"

γ_final = @"models/sovereign_1b/model.safetensors:model.norm.weight"
W_head = @"models/sovereign_1b/model.safetensors:lm_head.weight"

// [2] Текст оролт ба токенчлол
prompt = "CRON"
tokens = ι(prompt)

// [3] Эмбеддинг авах
H_0 = E[tokens]

// [4] Input RMSNorm
H_norm1 = N(H_0, γ_1)

// [5] Query, Key, Value проекц (W^T)
Q = H_norm1 @ W_q^T
K = H_norm1 @ W_k^T
V = H_norm1 @ W_v^T

// [6] Grouped Query Attention (GQA: 4 query heads, 1 KV head, head_dim: 16)
Attn_out = Ψ_g(Q, K, V, 4, 1, 0.25)
H_attn = H_0 + (Attn_out @ W_o^T)

// [7] Post-Attention RMSNorm ба SwiGLU MLP
H_norm2 = N(H_attn, γ_2)
MLP_out = S(H_norm2, W_gate^T, W_up^T, W_down^T)
H_layer = H_attn + MLP_out

// [8] Final RMSNorm ба LM Head Logits
H_final = N(H_layer, γ_final)
Z_logits = H_final @ W_head^T

// [9] Сүүлийн токений logits дээрээс дараагийн токенийг түүвэрлэх
Z_last = Z_logits[3]
next_tok = χ(Z_last, 0.7, 0.9, 108)

out_seq = [next_tok]
s_out = δ(out_seq)

// [10] Загварын төлөвийг экспортлох
H_final >> "safetensors_inference_state.safetensors:H_final"
Z_logits >> "safetensors_inference_state.safetensors:Z_logits"
out_seq >> "safetensors_inference_state.safetensors:next_token"

s_out

