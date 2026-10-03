; ==============================================================================
; CRON Low-Level Machine Language (.cl) - Direct VLIW Hello World
; Target: 256-Core 4D-Torus Neuromorphic / Photonic Silicon Supercomputer
; Architecture: 128-bit VLIW Bundles (4 parallel hardware slots per clock cycle)
; Opcode PR: Direct Console / Host UART Character Stream Output
; ==============================================================================

; Cycle 1: Parallel UART stream -> Output 'H', 'e', 'l', 'l'
B0001: _PR00#480> _PR00#650> _PR00#6C0> _PR00#6C0>

; Cycle 2: Parallel UART stream -> Output 'o', ',', ' ', 'W'
B0002: _PR00#6F0> _PR00#2C0> _PR00#200> _PR00#570>

; Cycle 3: Parallel UART stream -> Output 'o', 'r', 'l', 'd'
B0003: _PR00#6F0> _PR00#720> _PR00#6C0> _PR00#640>

; Cycle 4: Parallel UART stream -> Output '!', '\n' and HALT silicon core
B0004: _PR00#210> _PR00#0A0> _HL00#000! _NO00#000>
