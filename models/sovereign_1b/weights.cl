; ============================================================================
; CRON Native .cl Weight Binding Directives Lowered from SafeTensors
; Zero-DRAM Scratchpad SRAM Banking with 1.58-bit Sub-Byte Packing
; ============================================================================

; Tensor: model.layers.0.mlp.down_proj.weight (shape: [64, 128], dtype: F32)
.weights bank=0 offset=0 [0x58098244, 0x20050AAA, 0x95969025, 0x5841904A]
; Tensor: model.layers.0.mlp.gate_proj.weight (shape: [128, 64], dtype: F32)
.weights bank=1 offset=0 [0x5226A686, 0xA0858869, 0x56842481, 0x00246A65]
; Tensor: model.layers.0.input_layernorm.weight (shape: [64], dtype: F32)
.weights bank=2 offset=0 [0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00]
; Tensor: model.layers.0.self_attn.q_proj.weight (shape: [64, 64], dtype: F32)
.weights bank=3 offset=0 [0x09640455, 0x50842486, 0x0A6A6626, 0x1990A059]
; Tensor: model.layers.0.post_attention_layernorm.weight (shape: [64], dtype: F32)
.weights bank=4 offset=0 [0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00]
; Tensor: lm_head.weight (shape: [128, 64], dtype: F32)
.weights bank=5 offset=0 [0x419A99A1, 0x562AA268, 0x8A446105, 0x50852A16]
; Tensor: model.layers.0.mlp.up_proj.weight (shape: [128, 64], dtype: F32)
.weights bank=6 offset=0 [0xA2261100, 0x56998294, 0xA1062929, 0x0056864A]
; Tensor: model.layers.0.self_attn.k_proj.weight (shape: [16, 64], dtype: F32)
.weights bank=7 offset=0 [0xA8AA9249, 0xA565200A, 0x59822942, 0x5401019A]
; Tensor: model.layers.0.self_attn.o_proj.weight (shape: [64, 64], dtype: F32)
.weights bank=8 offset=0 [0x60015444, 0x21199298, 0x91920460, 0x19811421]
; Tensor: model.layers.0.self_attn.v_proj.weight (shape: [16, 64], dtype: F32)
.weights bank=9 offset=0 [0x2A5A61A9, 0x62199950, 0x690A6996, 0x084A6564]
; Tensor: model.norm.weight (shape: [64], dtype: F32)
.weights bank=10 offset=0 [0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0x3C00]
; Tensor: model.embed_tokens.weight (shape: [128, 64], dtype: F32)
.weights bank=11 offset=0 [0x60418221, 0x2A621489, 0xA1561646, 0x851052AA]
