.stage "cron_bitnet_7b_foundation", params="7.544B", precision="1.58b_ternary", d_model=4096, heads=32, kv_heads=8, intermediate=14336, zero_overhead=true
.tensor %Q: [32, 128], %K: [8, 128], %V: [8, 128], %O: [4096, 4096]
.fuse [RMSNorm -> OpticalGEMM & RoPE -> FlashAttention-2 | SwiGLU -> MambaSSM]
.flow (Core[0,0,0,0] -> Core[1,0,0,0] -> Core[2,0,0,0] -> Core[3,0,0,0] -> Core[3,1,0,0] -> Core[3,2,0,0] -> Core[3,3,3,3]) {dor=XYZW}
.layout {TP=4, EP=4, CP=4, PP=4, dim="4x4x4x4", chip="256_core_torus"}

.weights bank=0, offset=0: [0.015, -0.032, 0.088, 0.004, -0.012, 0.045, 0.076, -0.019]
.weights bank=1, offset=0: [1.000, 1.025, 0.985, 1.012, 0.995, 1.030, 0.978, 1.005]
.weights bank=2, offset=0: [1.0, 0.0, -1.0, 1.0, -1.0, 0.0, 0.0, 1.0]
.weights bank=3, offset=0: [-1.0, 1.0, 0.0, 1.0, 0.0, -1.0, 1.0, 0.0]
.weights bank=4, offset=0: [1.000, 0.992, 1.015, 1.008, 0.988, 1.020, 1.002, 0.995]
.weights bank=5, offset=0: [1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 1.0]
.weights bank=6, offset=0: [0.0, 1.0, -1.0, 1.0, 1.0, 0.0, -1.0, 0.0]
.weights bank=7, offset=0: [0.884, 0.116, 0.912, 0.088]

.core [0, 0, 0, 0]:
@prompt_embedding_entry:
B0000: '==01#010> '==02#020> '==03#030> '==04#040>
B0001: _MD05$001> _MD06$002> _MD07$003> _bb00#000>
B0002: _TX05$100> _TX06$100> _TX07$100> _PK08`200>
B0003: _SB00#000; _WH09\000> _YD00/005> _HL00#000!

.core [1, 0, 0, 0]:
@gqa_attention_qkv_stage:
B0000: ~RX01$000> ~RX02$000> ~RX03$000> _ZL00#000>
B0001: _RM04@100> _PO02+000> _PO03+000> _bb00#000>
B0002: _OP05%00E> _MD06*2A2> _WD07&420> _SB00#000;
B0003: _PO04+600> _FA07$142> _TT08$200> _CD09"100>
B0004: _TX05$200> _TX07$200> _TX08$200> _HL00#000!

.core [2, 0, 0, 0]:
@flash_attention_engine:
B0000: ~RX01$000> ~RX02$000> ~RX03$000> _CA04.400>
B0001: _OP04$18F> _MD05*2A2> _CS06[000> _SB00#000;
B0002: _SM06$400? _FA07$542> _KG08{000> _bb00#000>
B0003: _OP08$68F> _PO09+300> _RC0A^2Q5> _SB00#000;
B0004: _TT0B$300> _PO0C+800> _OD0D}000> _bb00#000>
B0005: _PO01+B00> _PS0E,B00> _bb00#000>
B0006: _TX01$300> _HL00#000!

.core [3, 0, 0, 0]:
@swiglu_mlp_feedforward:
B0000: ~RX01$000> '==02#004> '==03#008> _BK0a=501>
B0001: _RM04@400> _BL0b<310> _bb00#000>
B0002: _TT05$400> _TT06$400> _SB00#000;
B0003: _SI07$500> _GU08?900> _bb00#000>
B0004: _PO08*760> _bb00#000>
B0005: _TX08$310> _TX01$310> _HL00#000!

.core [3, 1, 0, 0]:
@swiglu_down_projection:
B0000: ~RX08$000> ~RX01$000> _LF0a_000>
B0001: _TT09$600> _MD0A.280> _SB00#000;
B0002: _PO0B*900> _RF0c>604> _bb00#000>
B0003: _PO01+B00> _AW0d`000> _bb00#000>
B0004: _TX01$320> _HL00#000!

.core [3, 2, 0, 0]:
@hybrid_mamba_ssm_stage:
B0000: ~RX01$000> '==02#010> _LI0b$005>
B0001: _SS03@700> _bb00#000>
B0002: _FA04$342> _PO05+100> _SB00#000;
B0003: _TX05$333> _HL00#000!

.core [3, 3, 3, 3]:
@lm_head_unembed_and_sampling:
B0000: ~RX05$000> '==01#020> _FJ00)0Z8!
B0001: _RM06@100> _bb00#000>
B0002: _OP07$08F> _MD08*2A2> _SB00#000;
B0003: _SM09$700? _PO0A/400> _bb00#000>
B0004: _FU00#000> _PO01+900> _FE00#000> _HL00!000>
