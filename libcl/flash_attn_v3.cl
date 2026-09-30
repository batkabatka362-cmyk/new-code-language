; CRON Standard Microcode Kernel: libcl/flash_attn_v3.cl
; FlashAttention-3 Asynchronous Tile-MMA & Online Softmax with FP8 Scaling
@kernel flash_attention_v3
.target silicon.ai_isa
.ipc_target 4.0

B0000: '=00#3F80h '=01#3F00+ _OP02$010d _TT03$0204
B0001: _SM04$030s _RM05$040m _OP06$050= _FA07$060c
B0002: _RF08$070R _TO09$0804 _ST0A$090j _NO00#000L
B0003: _DW00$0A0< _RS00#050* _NO00#000L _HL00#000H
