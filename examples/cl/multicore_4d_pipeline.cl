; ============================================================================
; CRON Multi-Core 4D-Torus Distributed Pipeline Example (.cl)
; Architecture: 256-Core 4D-Torus Hybrid Optical/Neuromorphic Silicon
;
; Demonstrates:
;   - Multi-Core Spatial Directives (.core [x, y, z, w]:)
;   - Core [0, 0, 0, 0] Producer injecting activations into NoC channel (_TX)
;   - Core [1, 2, 0, 0] Consumer receiving from NoC FIFO (_RX) & computing GEMM (_OP)
;   - Dimension-Order Routing (DOR: X -> Y -> Z -> W) across 3 torus hops
; ============================================================================

.core [0, 0, 0, 0]:
@producer_entry:
; Cycle 0: Initialize Activation Vector in R1
B0000: '==01#0FF> '==02#00A> _NO00#000> _NO00#000>
; Cycle 1: Optical MZI Initial Transform & Transmit to Core [1, 2, 0, 0]
B0001: _OP01$28F> _TX01$100> _NO00#000> _NO00#000>
; Cycle 2: Halt Producer
B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>

.core [1, 2, 0, 0]:
@consumer_entry:
; Cycle 0: Wait & Pop Packet from Core Mailbox FIFO into R2
B0000: _RX02$000> '==03#004> _NO00#000> _NO00#000>
; Cycle 1: Sub-Byte SIMD MAC & Optical GEMM on Ingested Activations
B0001: _MD00$204> _OP01$38F> _PO04$201> _NO00#000>
; Cycle 2: Spatial Broadcast Result across Local Halo & Halt
B0002: _SB00$100> _HL00#000! _NO00#000> _NO00#000>
