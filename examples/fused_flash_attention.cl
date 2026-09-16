; ============================================================================
; CRON SSS+ AI-Native Silicon Machine Language (.cl)
; Kernel: FusedFlashAttention (Zero-Allocation Streaming Kernel)
; Algorithm: Q K^T -> scale -> softmax -> V (FlashAttention Pipeline)
; Target: 256-Core 4D-Torus Photonic Neuromorphic Silicon Core
; Eliminates Intermediate High-Bandwidth Memory (HBM/DRAM) Roundtrips
; Microarchitectural Guarantees:
;   - Single-Pass VLIW Streaming Pipeline anchored by _FU and _FE
;   - Zero intermediate DRAM writes (_DW eliminated)
;   - Peak SRAM tile occupancy <= 256 elements
; ============================================================================
; Bundle Format: B<cycle>: <slot0> <slot1> <slot2> <slot3>
; 10 Chars per Slot: [0]=prefix, [1..2]=op, [3]=bank, [4]=dest, [5]=mode, [6]=src, [7]=parity, [8]=imm, [9]=term
; ============================================================================

; Cycle 0: Arena reset, Stream Fusion Start (_FU), and Query/Key vector loads
B0000: _RS00$000> _FU00$000> '=01#000A> '=02#0014>

; Cycle 1: Optical MZI QK^T GEMM, Scale Factor Multiply, and Value Transpose
B0001: _OP04$100> _PO05*402> _TT03$200> _NO00$000>

; Cycle 2: Ternary Softmax Dot, Output Accumulation, Stream Fusion End (_FE), and Torus Barrier
B0002: _MD06.503> _PO07+601> _FE00$000> _bb00$000>

; Cycle 3: Spatial Broadcast of fused attention output across 4D Torus & Halt
B0003: _SB07$000> _NO00$000> _NO00$000> _HL00$000>
