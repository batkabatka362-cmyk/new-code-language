; ============================================================================
; CRON In-Silicon Machine Code (.cl): High-Speed DMA Network Packet Server
; Target: silicon.hard_backend / 256-Core 4D-Torus NoC
; Architecture: Zero-Copy Circular DMA Ring Buffer & 4D-Torus Routing
; IPC Target: 4.0 (Full 4-Slot VLIW Execution)
; ============================================================================

@kernel high_speed_dma_server
.target silicon.hard_backend
.ipc_target 4.0

; Cycle 0: Setup DMA Ring Head/Tail pointers and Ring Capacity Mask (0x3FF = 1024)
B0000: '==01#000E '==02#040; '==03#03F] '==04#010A

; Cycle 1: Lock-Free Read from Ring Buffer, Extract Core ID & Dimension Differences
B0001: _CS05M2047 _SR05M100" _LD06M500^ _SB07M600!

; Cycle 2: Execute 4D Dimension-Order Routing (DOR: X -> Y -> Z -> W) & Transmit Packet
B0002: _bb08M700S ~RM09M800# _TX0AM102- !HL00#000(
