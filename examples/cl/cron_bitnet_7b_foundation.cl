.stage "cron_bitnet_7b_foundation;<\\", params="7.544B", precision="1.58b_ternary", d_model=4096, heads=32, kv_heads=8, intermediate=14336, zero_overhead=true
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
B0001: _SZ05$081> _SZ06$0B2> _SZ07$0A3> ~PF03@210>
B0002: _TX05$1B0> _TX06$110> _TX07$170> _PK08`270>
B0003: _SB00#060> _NO00#000> _NO00#000> _NO00#000>

.core [1, 0, 0, 0]:
@gqa_attention_qkv_stage:
B0004: ~RX01$070> ~RX02$0D0> ~RX03$0B0> _ZL00#000>
B0005: _RM04@1C0> _SZ05$1A0> _SZ06$2F0> _bb00#000>
B0006: _SZ07$3C0> ~PF08@450> _SB00#060> _NO00#000>
B0007: _WD07&430> _NO00#000> _NO00#000> _NO00#000>
B0008: _PO04+650> _FA07$1B2> _TT08$210> _CD09"1B0>
B0009: _TX05$240> _TX07$280> _TX08$2A0> _HL00#000!

.core [2, 0, 0, 0]:
@flash_attention_engine:
B0010: ~RX01$070> ~RX02$0D0> ~RX03$0B0> _CA04.430>
B0011: _QK01$4D0> _QV02$5E0> _SB00#060> _NO00#000>
B0012: _QA04$1F0> _NO00#000> _NO00#000> _NO00#000>
B0013: _SM06$400? _FA07$5F2> _KG08{070> _bb00#000>
B0014: _SZ08$6AF> _PO09+3A0> _RC0A^235> _SB00#060>
B0015: _TT0B$3A0> _PO0C+830> _OD0D}0E0> _bb00#000>
B0016: _PO01+BD0> _PS0E,B40> _bb00#000> _NO00#000>
B0017: _TX01$390> _HL00#000! _NO00#000> _NO00#000>

.core [3, 0, 0, 0]:
@swiglu_mlp_feedforward:
B0018: ~RX01$070> '==02#004> '==03#008> _BK0a=5E1>
B0019: _RM04@4D0> _bb00#000> _NO00#000> _NO00#000>
B0020: _XR0a$4C0> _NO00#000> _NO00#000> _NO00#000>
B0021: _SZ05$4B0> _SZ06$410> _SB00#060> _NO00#000>
B0022: _SI07$570> _GU08?940> _bb00#000> _NO00#000>
B0023: _PO08*730> _bb00#000> _NO00#000> _NO00#000>
B0024: _TX08$3F0> _TX01$390> _HL00#000! _NO00#000>

.core [3, 1, 0, 0]:
@swiglu_down_projection:
B0025: ~RX08$010> ~RX01$070> _LF0a_0F0> _NO00#000>
B0026: _TT09$630> _SZ0a$210> _SB00#060> _NO00#000>
B0027: _PO0B*9B0> _RF0c>6A4> _bb00#000> _NO00#000>
B0028: _PO01+BD0> _AW0d`000> _bb00#000> _NO00#000>
B0029: _TX01$390> _HL00#000! _NO00#000> _NO00#000>

.core [3, 2, 0, 0]:
@hybrid_mamba_ssm_stage:
B0030: ~RX01$070> '==02#010> _LI0b$055> _NO00#000>
B0031: _SS03@780> _HB04#100> _bb00#000> _NO00#000>
B0032: _FA04$3B2> _PO05+180> _SB00#060> _NO00#000>
B0033: _TX05$383> _HL00#000! _NO00#000> _NO00#000>

.core [3, 3, 3, 3]:
@lm_head_unembed_and_sampling:
B0034: ~RX05$0F0> '==01#020> _FJ00)0F8! _NO00#000>
B0035: _RM06@100> _bb00#000> _NO00#000> _NO00#000>
B0036: _SZ07$06F> _SZ08$252> _SB00#060> _NO00#000>
B0037: _SM09$7D0? _PO0A/480> _bb00#000> _NO00#000>
B0038: _FU00#000> _PO01+980> _FE00#000> _HL00!060>
