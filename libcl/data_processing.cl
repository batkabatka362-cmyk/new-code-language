; CRON Standard Microcode Kernel: libcl/data_processing.cl
; Section 1: Data Preprocessing, Scaling, Normalization, Token Hash & DSP
@kernel data_processing_simd
.target silicon.4d_torus
.ipc_target 4.0

B0000: '==01#0A0> '==02#080> '==03#0C8> '==04#020>
B0001: _PO05-310> '==06#050> '==07#020> _NO00#000>
B0002: _PO08/520> _PO09-360> _NO00#000> _NO00#000>
B0003: _PO0A/970> _PK0B$8A0> _NO00#000> _NO00#000>
B0004: '==0C#040> _MD0D*AC0> _NO00#000> _NO00#000>
B0005: _PO0D+DD0> '==0E#070> _NO00#000> _NO00#000>
B0006: _MD02*D10> _PO03-D20> _NO00#000> _NO00#000>
B0007: _TT0D$2B0> _PO04+230> _NO00#000> _NO00#000>
B0008: _ST08#050> _ST0A#060> _ST0D#070> _NO00#000>
B0009: _SB00#070> _NO00#000> _NO00#000> !HL00#000(
