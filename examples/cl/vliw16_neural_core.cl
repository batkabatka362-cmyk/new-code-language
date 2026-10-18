// ============================================================================
// CRON 16-Character VLIW Neural Core Demonstration (512-Bit Bundles)
// Demonstrates:
//   - 256 physical register residency (R[0x00]..R[0xFF]) with 0 DRAM spilling
//   - 3-Address FMA (_FM), ReLU/Max (_MX), Swizzle (_SW), FP16 (_FP)
//   - In-Slot 16-bit Immediate loading and SRAM load/store
// ============================================================================

// Bundle 0: Load 16-bit constants into distant register banks (Bank 0, 1, 2, 8)
B0000: '=001#0000000A0> '=012#000000140> '=02A#0000007B0> '=08F#000001C80>

// Bundle 1: 3-Address Arithmetic & Fused Multiply-Add
B0001: _PO03+010200000> _FM05+0102000A0> _MX06+050000000> _SW07R010200000>

// Bundle 2: In-Slot FP16 (1.0 + 2.5 = 3.5) & Sub-Byte Ternary BitNet Dot Product
B0002: '=010#00003C000> '=011#000041000> _FP12+101100000> _MD15.101100000>

// Bundle 3: 64KB SRAM Store and Load
B0003: _SR12#010001000> _LD18#010001000> _NO00#000000000> _NO00#000000000>
