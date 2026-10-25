@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

B0000: '==01#000E '==02#040; '==03#03F] '==04#010A

; Cycle 1: Optical Waveguide Direct Transmit to Next Adjacent Chip (Ring Stage 1)
B0001: _CS05M2047 _SR05M100" _LD06M500^ _SB07M600!

; Cycle 2: Optical Accumulation Reduction and Cluster-Wide Global Barrier
B0002: _bb08M700S ~RM09M800# _TX0AM102- !HL00#000(
