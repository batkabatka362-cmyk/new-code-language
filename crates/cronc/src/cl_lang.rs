// ============================================================================
// CRON AGI & Silicon Cipher Machine Language (.cl) Engine & Validator
//
// OFFICIAL MANIFESTO:
//   CL is NOT C++, NOT Python, and NOT a verbose human scripting language.
//   CL is a dense, cipher-compact 10-char VLIW microcode language designed
//   exclusively for AGI/AI Autonomous Agents and Hard Silicon Backends:
//     - 256-Core 4D-Torus S^3 x Z100 Processor
//     - Multiplier-Free O(0) FLOPs Spherical Matrix Wave Interference Core
//     - Zero-DRAM 16-Bank Reversible PGAS SRAM Register File
//     - RTL Synthesizable Hard Silicon (SkyWater Sky130, Xilinx UltraScale+)
//
// ZERO VERBAL WORDS POLICY:
//   English keywords, classes, methods, and verbal DSL statements are
//   STRICTLY FORBIDDEN in .cl code. All .cl files must consist purely of
//   system directives (@kernel, .target) and 10-character VLIW cipher slots.
//
// Syntax Specification:
//   - Bundle: B<cycle_num>: <slot0> <slot1> <slot2> <slot3>
//   - Slot:   Exactly 10 ASCII characters per token.
//             [0]    Prefix: '_' (Operation) or '\'' (Immediate load)
//             [1..2] Opcode: 2-char mnemonic (OP, FA, PO, MD, BK, BL, RF, etc.)
//             [3..4] Dest Register: R0..RF (hex 00..0F)
//             [5]    Mode / Delimiter: '$', '#', '@', etc.
//             [6]    Src Register / High Parameter
//             [7]    Parity / Check Token / CRC-8 ATM
//             [8]    Immediate / Low Parameter
//             [9]    Terminator: '>' (Execute) or '!' (Halt/Trap)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ClSlot {
    pub raw: String,
    pub prefix: char,
    pub opcode: String,
    pub dest_reg: Option<usize>,
    pub mode: char,
    pub src_reg: Option<usize>,
    pub parity_token: char,
    pub imm_token: char,
    pub terminator: char,
    // 16-character slot extensions (3-Address code & 16-bit immediate)
    pub src2_reg: Option<usize>,
    pub imm_u16: u16,
    pub is_16char: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ClBundle {
    pub cycle: usize,
    pub slots: [ClSlot; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClWeightBinding {
    pub bank: usize,
    pub offset: usize,
    pub values: Vec<u32>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClNativeType {
    Trit16,
    Int4x8,
    Fp16x2,
    Geom4d,
    Bit32,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClTensorArg {
    pub name: String,
    pub dtype: String,
    pub dims: Vec<usize>,
}

impl ClTensorArg {
    pub fn native_type(&self) -> ClNativeType {
        match self.dtype.to_lowercase().as_str() {
            "trit16" | "ternary" | "1.58b" => ClNativeType::Trit16,
            "int4x8" | "int4" | "i4" => ClNativeType::Int4x8,
            "fp16x2" | "fp16" | "f16" => ClNativeType::Fp16x2,
            "geom4d" | "rotor4d" | "clifford" => ClNativeType::Geom4d,
            "bit32" | "u32" | "i32" | "int" => ClNativeType::Bit32,
            _ => ClNativeType::Custom(self.dtype.clone()),
        }
    }

    pub fn total_bits(&self) -> usize {
        let elements = if self.dims.is_empty() { 1 } else { self.dims.iter().product() };
        match self.native_type() {
            ClNativeType::Trit16 => elements * 2,
            ClNativeType::Int4x8 => elements * 4,
            ClNativeType::Fp16x2 => elements * 16,
            ClNativeType::Geom4d => elements * 32 * 4,
            ClNativeType::Bit32 => elements * 32,
            ClNativeType::Custom(_) => elements * 32,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClKernelDef {
    pub name: String,
    pub inputs: Vec<ClTensorArg>,
    pub outputs: Vec<ClTensorArg>,
    pub attributes: std::collections::HashMap<String, String>,
    pub body_statements: Vec<String>,
}

impl ClKernelDef {
    pub fn total_input_bits(&self) -> usize {
        self.inputs.iter().map(|arg| arg.total_bits()).sum()
    }

    pub fn total_output_bits(&self) -> usize {
        self.outputs.iter().map(|arg| arg.total_bits()).sum()
    }
}

/// Rich semantic directives supported in .cl machine language
#[derive(Debug, Clone, PartialEq)]
pub enum ClDirective {
    Stage {
        id: String,
        params: String,
        precision: String,
        d_model: usize,
        heads: usize,
        kv_heads: usize,
        intermediate: usize,
        zero_overhead: bool,
    },
    Tensor {
        name: String,
        dims: Vec<usize>,
    },
    Fuse {
        chain: String,
    },
    Flow {
        route: String,
        dor: String,
    },
    Layout {
        tp: usize,
        ep: usize,
        cp: usize,
        pp: usize,
        dim: String,
        chip: String,
    },
    Weights(ClWeightBinding),
    Clifford {
        rotor: String,
        vector: String,
        algebra: String,
    },
    Core {
        coords: [usize; 4],
    },
    Circuit {
        core_id: usize,
        layer: String,
        excitatory: usize,
        inhibitory: usize,
        tau_us: usize,
        dopamine: f32,
    },
    Loop {
        count: usize,
        stride: usize,
    },
    Barrier {
        name: String,
        cores: String,
    },
    MemorySpace {
        space: String,
        size_kb: usize,
        bank: usize,
    },
    Kernel(ClKernelDef),
    Custom {
        name: String,
        content: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct ClReport {
    pub total_bundles: usize,
    pub total_slots: usize,
    pub opcodes_verified: usize,
    pub parity_verified: usize,
    pub crc_verified: usize,
    pub strict_crc_verified: usize,
    pub hazards: Vec<String>,
    pub labels_found: usize,
    pub cores_partitioned: usize,
    pub circuits_configured: usize,
    pub weights_bound: usize,
    pub total_weight_bytes: usize,
    pub directives_found: usize,
    pub directives_parsed: Vec<ClDirective>,
}

pub const KNOWN_OPCODES: &[&str] = &[
    "OP", // Optical GEMM (Brain 2)
    "FA", // Forward Autodiff Tap (Brain 2)
    "PO", // Predicated SIMD ALU
    "MD", // Mult-Dot Sub-Byte SIMD MAC
    "BK", // Backward Invert Reversible Pass (Brain 3)
    "CB", // Halo Cache Blend
    "RF", // Reversible Fredkin Swap Gate (Brain 3)
    "GU", // Gradient Update
    "ST", // STDP Synapse Update / Staging Prefetch (Brain 4)
    "SY", // Symbolic Causal Unification (Brain 1)
    "RS", // Region Arena 0-cycle Reset
    "PK", // Sub-byte SIMD Packing
    "TL", // 4D Tiling Coordinate Calculation
    "YD", // Coroutine Fiber Yield
    "SP", // Fiber Spawn
    "FJ", // Fiber Join
    "DW", // DMA Transfer Wait
    "SB", // Spatial Broadcast Across 4D Torus / Subtraction
    "SH", // Self-Healing Sentry Config (Brain 6)
    "RC", // Re-route Fallback Channel
    "TO", // Toffoli 3-Wire Reversible Gate (Brain 3)
    "WD", // Wavelength Division Multiplexing Photonic (Brain 2)
    "MZ", // Mach-Zehnder Interferometer Photonic Coupling (Brain 2)
    "KG", // Knowledge Graph Triple Query (Brain 1)
    "SW", // Superposition Wave Branch (Brain 5)
    "PT", // Parity Telemetry & Health Sentry (Brain 6)
    "HE", // Hyper-Edge Association Matcher (Brain 1)
    "OD", // Lorenz Chaos Attractor Diffusion (Brain 5)
    "CS", // Compare-And-Swap Hardware Atomic
    "TT", // In-Register 4x4 Transpose (Brain 2)
    "PS", // Parallel Prefix-Sum SIMD Scan
    "CA", // Cross-Attention Gating (Brain 5)
    "CD", // CORDIC Sin/Cos Trigonometric Engine
    "AW", // Arbiter Weight Update (Brain 6)
    "WH", // NoC Wormhole Bypass Tunnel
    "LF", // Neuromorphic LIF Spike Generator (Brain 4)
    "LI", // Neuromorphic LIF Neuron Step (Brain 4)
    "TX", // NoC Channel Send Wormhole Packet Injection
    "RX", // Core Mailbox Channel Recv FIFO Pop
    "IR", // In-Network Flight Reduction
    "DF", // Adaptive Deflection Routing
    "AC", // Hardware Capability Token
    "SN", // Hardware Bounds Sanitization
    "SC", // Hardware Secure I-Cache Patch
    "RN", // 32-bit Galois LFSR PRNG
    "PL", // Non-Blocking NoC FIFO Poll
    "RT", // Return from Hardware Trap (MRET)
    "HL", // Halt Execution
    "NO", // NOP (No Operation)
    "=0", // Immediate Load Low / Core 0
    "=1", // Immediate Load High / Core 1
    "=2", // Immediate Load Core 2
    "=3", // Immediate Load Core 3
    "==", // Generic Immediate Load
    "JP", // Unconditional Branch/Jump
    "BZ", // Branch if Zero
    "BN", // Branch if Non-Zero
    "BL", // Branch if Less/Negative
    "BG", // Branch if Greater
    "JL", // Jump/Branch if Less
    "bb", // 256-Core Chip-Wide Global Synchronization Barrier
    "FU", // Fusion Stream Anchor Start (Milestone #012)
    "FE", // Fusion Stream Anchor End / Commit (Milestone #012)
    // Milestone #021: Esolang-Inspired AI Silicon Coprocessor Opcodes
    "TI", // Brainfuck Tape Pointer Increment ($tp0++)
    "TD", // Brainfuck Tape Pointer Decrement ($tp0--)
    "TR", // Brainfuck Tape Read & Auto-Advance -> Dest Reg
    "TW", // Brainfuck Tape Write & Auto-Advance <- Src Reg
    "ZL", // Brainfuck Zero-Overhead Hardware Loop Counter Set
    "TC", // Malbolge 16-Trit SIMD Crazy Operation / Activation LUT
    "TM", // Malbolge Multiplier-Free BitNet b1.58 Trit-MAC
    "DE", // Befunge Systolic Push East (+X)
    "PW", // Befunge Systolic Push West (-X)
    "DN", // Befunge Systolic Push North (+Y)
    "DS", // Befunge Systolic Push South (-Y)
    "UN", // Prolog 1-Cycle Hardware Symbolic Index Matcher / Unifier
    // Milestone #029: Dedicated AI Silicon ISA Extensions
    "RM", // RMSNorm Normalizer Step
    "SM", // Streaming Online Flash-Softmax
    "SZ", // 2:4 Structured Sparsity Zero-Skip Dot Product (2x Speedup)
    "QK", // 4-Bit Sub-Byte Key Quantize & Pack into SRAM RingTape
    "QV", // 4-Bit Sub-Byte Value Quantize & Pack into SRAM RingTape
    "QA", // Sub-Byte Streaming FlashAttention Step
    "XR", // Mixture of Experts (MoE) 4D-Torus Dynamic Packet Router
    "PF", // Zero-Overhead DMA Prefetch Double-Buffering
    "HB", // On-Silicon Neuromorphic Synaptic Plasticity Step (Online Hebbian / Oja)
    "SI", // SiLU Activation (SwiGLU)
    "GE", // GELU Activation
    "SS", // Selective Scan SSM Step (Mamba)
    // Milestone #044: 4D Clifford Topological Engine & TNM v1.0 Opcodes
    "TP", // 4D Topological Phase Token Encode / S^3 Coordinate Mapping
    "WF", // 4D Wave Interference Resonance Attention (+2, +1, 0, 0 FLOPs)
    "CN", // 4D Curved Needle Non-Collision Deflector Step (|Wi - Wj| >= 1)
    "HP", // 4D Homeostasis Parity Rotation [34, 33, 33] Cycle (H == 100%)
    "LY", // 4D Cortical Layer Transition / Clamp (Apical -> Sensory -> Motor)
    // DPSTA & TBT Topological Brain Transformer Silicon Opcodes
    "DP", // DPSTA 4-Tact Zero-FLOPs Discrete 94-State Transition Step
    "HT", // Hierarchical TBT 3-Tier Attractor Memory Ring Resonance
    "CR", // Clifford Algebra Cl(4,0) Spin(4) Multivector Rotor Rotation (v' = R*v*~R)
    "TS", // Tropical Semi-Ring (Max, +) Winner-Take-All Arbiter Step
    "UP", // Sub-Byte 1.58-bit Ternary Unpack / Decode into 8-bit registers
    "VR", // Step-Level Process Reward Model (PRM) Verifier Register & Early Prune
    "FM", // Fused Multiply-Add (FMA) 3-Address Operation
    "MX", // Max / Min / ReLU / Clamp 3-Address Operation
    "FP", // In-Slot 16-Bit Floating Point (FP16 IEEE 754) Operation
    // Extended Arithmetic, Logical & Neuromorphic Opcodes
    "AD", // Add
    "SU", // Subtract (Canonical 0-FLOPs Subtraction)
    "ML", // Multiply
    "DV", // Divide
    "EX", // Exponential
    "SQ", // Square Root
    "FX", // Fixed-Point Convert
    "CP", // Copy / Register Checkpoint
    "XO", // Bitwise XOR
    "RO", // Rotate Bits
    "MA", // Bitmask Filter
    "CO", // Compare Equal/Predicate
    "DA", // Dopamine Neuromodulator Tap
    "SE", // Serotonin Neuromodulator Tap
    "NE", // Norepinephrine Neuromodulator Tap
    "RV", // Reversible Vector Transform
    "LD", // Local PGAS Bank Load / 64KB Scratchpad SRAM Load Word
    "SR", // 64KB Scratchpad SRAM Store Register Word
    "LB", // 64KB Scratchpad SRAM Load Byte
    "BY", // 64KB Scratchpad SRAM Store Byte
    "JS", // Jump to Subroutine (Call)
    "LT", // Branch if Less Than Zero
    "CI", // Causal Intervention do(X = x) (Brain 1)
    "CF", // Counterfactual Abduction & Evaluation (Brain 1)
    "PR", // Hardware Console / UART Character Print Output
    "M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7",
    "M8", "M9", "MB", "MC", "ME", "MF", // Matrix Macro Opcodes
    // Accumulator & Activation Bank Macro Opcodes
    "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7",
    "A8", "A9", "AA", "AB", "AE", "AF",
    // Barrier & Brain-Bridge Macro Opcodes
    "B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7",
    "B8", "B9", "BA", "BC", "BD", "BE", "BF",
    // Sensory & State Macro Opcodes
    "S0", "S1", "S2", "S3", "S4", "S5", "S6", "S7",
    "S8", "S9", "SA", "SD", "SF",
    "_N", // NOP Alias
    // Extended Homopolymer Macro Opcodes
    "CC", // Chip-Wide 256-Core I/D Cache & Pipeline Invalidation
    "DD", // Zero-Overhead Direct 4D-Torus NoC DMA Transfer
    "EE", // Energy-Aware Dynamic Voltage and Frequency Scaling (DVFS)
    "BB", // Brain-Bridge Cross-Neuromorphic Synchronization
    "FF", // Fredkin Reversible Full Fold
    "00", // Null Quiesce / Sleep
    "11", // Photonic Laser Pump Strobe
    "88", // Region Arena Instantaneous 0-Cycle Reset
    "99", // Global Hardware Sentry Watchdog Trip
    "aa", // All-to-all NoC Hypercube Scatter
    "cc", // Core-to-Core Cache Coherence Handshake
    "co", // Core Outward Coherence Route
    "dd", // Deterministic Deflection Clear
    "ee", // Event-Driven Neuromorphic Spike Broadcast
    "ff", // Fast-Fourier / Wavelength Multiplex Trigger
];

pub const ALPHABET_94_STR: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

pub fn get_alphabet_94() -> Vec<char> {
    (33u8..=126u8).map(|b| b as char).collect()
}

#[derive(Debug, Clone, Default)]
pub struct AlphabetAuditReport {
    pub unique_characters_used: usize,
    pub total_characters_scanned: usize,
    pub coverage_percentage: f64,
    pub missing_characters: Vec<char>,
    pub character_frequencies: std::collections::HashMap<char, usize>,
    pub entropy_bits_per_char: f64,
    pub total_bundles: usize,
    pub total_slots: usize,
}

pub fn audit_alphabet_coverage(cl_code: &str) -> AlphabetAuditReport {
    use std::collections::HashMap;
    let mut freqs: HashMap<char, usize> = HashMap::new();
    let mut total_scanned = 0;
    let mut total_bundles = 0;
    let mut total_slots = 0;

    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }
        for c in trimmed.chars() {
            if !c.is_whitespace() {
                *freqs.entry(c).or_insert(0) += 1;
                total_scanned += 1;
            }
        }
        if let Some((b_part, slots_part)) = trimmed.split_once(':') {
            if b_part.starts_with('B') || b_part.starts_with('b') {
                total_bundles += 1;
                total_slots += slots_part.split_whitespace().count();
            }
        }
    }

    let alphabet = get_alphabet_94();
    let mut missing = Vec::new();
    for &c in &alphabet {
        if !freqs.contains_key(&c) {
            missing.push(c);
        }
    }

    let unique_used = alphabet.len() - missing.len();
    let coverage = (unique_used as f64 / 94.0) * 100.0;

    // Shannon entropy: H = -sum(p * log2(p))
    let mut entropy = 0.0;
    if total_scanned > 0 {
        for &count in freqs.values() {
            let p = count as f64 / total_scanned as f64;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }
    }

    AlphabetAuditReport {
        unique_characters_used: unique_used,
        total_characters_scanned: total_scanned,
        coverage_percentage: coverage,
        missing_characters: missing,
        character_frequencies: freqs,
        entropy_bits_per_char: entropy,
        total_bundles,
        total_slots,
    }
}

#[inline(always)]
pub fn hex_digit_val(b: u8) -> Option<usize> {
    match b {
        b'0'..=b'9' => Some((b - b'0') as usize),
        b'a'..=b'f' => Some((b - b'a' + 10) as usize),
        b'A'..=b'F' => Some((b - b'A' + 10) as usize),
        _ => None,
    }
}

#[inline(always)]
pub fn parse_hex2_bytes(b0: u8, b1: u8) -> Option<usize> {
    let hi = hex_digit_val(b0)?;
    let lo = hex_digit_val(b1)?;
    Some((hi << 4) | lo)
}

#[inline(always)]
pub fn is_valid_cl_prefix(b: u8) -> bool {
    matches!(
        b,
        b'_' | b'\'' | b'~' | b'@' | b'%' | b'&' | b'^' | b'|'
        | b'$' | b'#' | b':' | b'\\' | b'*' | b'+' | b'-' | b'/'
        | b'?' | b'!' | b'=' | b'<' | b'>' | b'.' | b'(' | b')' | b'`'
    )
}

#[inline(always)]
pub fn is_non_writer_opcode(op: &[u8; 2]) -> bool {
    matches!(
        op,
        b"SH" | b"RS" | b"HL" | b"DW" | b"YD" | b"NO"
        | b"JP" | b"BZ" | b"BN" | b"BL" | b"BG" | b"JL" | b"LT" | b"JS" | b"SR" | b"BY"
        | b"TI" | b"TD" | b"TW" | b"ZL" | b"DE" | b"PW" | b"DN" | b"DS" | b"bb" | b"BB"
        | b"RT" | b"FE" | b"PR" | b"CC" | b"DD" | b"EE" | b"11" | b"88" | b"99" | b"aa"
        | b"cc" | b"co" | b"dd" | b"ee" | b"ff" | b"00" | b"FF"
        | b"TX" | b"SY"
    )
}

#[inline(always)]
pub fn parse_hex4_bytes(slice: &[u8]) -> Option<u16> {
    if slice.len() < 4 {
        return None;
    }
    let d0 = hex_digit_val(slice[0])? as u16;
    let d1 = hex_digit_val(slice[1])? as u16;
    let d2 = hex_digit_val(slice[2])? as u16;
    let d3 = hex_digit_val(slice[3])? as u16;
    Some((d0 << 12) | (d1 << 8) | (d2 << 4) | d3)
}

pub fn parse_slot(raw: &str) -> Result<ClSlot, String> {
    let bytes = raw.as_bytes();
    if bytes.len() != 10 && bytes.len() != 16 {
        return Err(format!(
            "Slot width violation: Each .cl slot must be 10 or 16 characters, found {} ('{}')",
            raw.len(),
            raw
        ));
    }

    let prefix_b = bytes[0];
    if !is_valid_cl_prefix(prefix_b) {
        return Err(format!("Invalid slot prefix '{}' in '{}'", prefix_b as char, raw));
    }
    let prefix = prefix_b as char;

    let op_bytes: [u8; 2] = [bytes[1], bytes[2]];
    let opcode: String = if prefix_b == b'`' {
        "SM".to_string()
    } else if (op_bytes[0] == b'_' && op_bytes[1] == b'N') || raw.starts_with("__NOP") {
        "NO".to_string()
    } else {
        match std::str::from_utf8(&op_bytes) {
            Ok(s) => s.to_string(),
            Err(_) => return Err(format!("Invalid non-UTF8 opcode bytes in '{}'", raw)),
        }
    };

    let is_writer = !is_non_writer_opcode(&op_bytes);

    if bytes.len() == 16 {
        let term_b = bytes[15];
        if !term_b.is_ascii_graphic() {
            return Err(format!(
                "Invalid slot terminator '{}' in '{}' (expected valid CL terminator)",
                term_b as char, raw
            ));
        }
        let terminator = term_b as char;

        let dest_reg = if is_writer {
            parse_hex2_bytes(bytes[3], bytes[4])
        } else {
            None
        };

        let mode = bytes[5] as char;
        let src_reg = parse_hex2_bytes(bytes[6], bytes[7]);
        let src2_reg = parse_hex2_bytes(bytes[8], bytes[9]);
        let imm_u16 = parse_hex4_bytes(&bytes[10..14]).unwrap_or(0);
        let imm_token = bytes[13] as char;
        let parity_token = bytes[14] as char;

        Ok(ClSlot {
            raw: raw.to_string(),
            prefix,
            opcode,
            dest_reg,
            mode,
            src_reg,
            parity_token,
            imm_token,
            terminator,
            src2_reg,
            imm_u16,
            is_16char: true,
        })
    } else {
        let term_b = bytes[9];
        if !term_b.is_ascii_graphic() {
            return Err(format!(
                "Invalid slot terminator '{}' in '{}' (expected valid CL terminator)",
                term_b as char, raw
            ));
        }
        let terminator = term_b as char;

        let dest_reg = if is_writer {
            parse_hex2_bytes(bytes[3], bytes[4])
        } else {
            None
        };

        let mode = bytes[5] as char;
        let src_reg = hex_digit_val(bytes[6]);
        let parity_token = bytes[7] as char;
        let imm_token = bytes[8] as char;
        let imm_u16 = hex_digit_val(bytes[8]).unwrap_or(0) as u16;

        Ok(ClSlot {
            raw: raw.to_string(),
            prefix,
            opcode,
            dest_reg,
            mode,
            src_reg,
            parity_token,
            imm_token,
            terminator,
            src2_reg: None,
            imm_u16,
            is_16char: false,
        })
    }
}

/// Parse `.weights` directive lines:
/// Syntax forms supported:
/// 1. `.weights bank=1, offset=0: [0x3F800000, 0x40000000, 0x3E800000]`
/// 2. `.weights bank=2: [1.0, -0.5, 2.0, 0.0]`
/// 3. `.weights "weights.bin", bank=3, offset=0, size=16`
pub fn parse_weights_directive(line: &str) -> Result<ClWeightBinding, String> {
    let trimmed = line.trim();
    if !trimmed.starts_with(".weights") {
        return Err(format!("Expected .weights directive, got '{}'", trimmed));
    }
    let rest = trimmed[".weights".len()..].trim();

    let mut bank = 0usize;
    let mut offset = 0usize;
    let mut file_path = None;
    let mut values = Vec::new();

    // Check for quoted file path
    if let Some(start_quote) = rest.find('"') {
        if let Some(end_quote) = rest[start_quote + 1..].find('"') {
            let path = &rest[start_quote + 1..start_quote + 1 + end_quote];
            file_path = Some(path.to_string());
        }
    }

    // Check for bracketed values: [...]
    if let Some(open_b) = rest.find('[') {
        if let Some(close_b) = rest.rfind(']') {
            let inner = &rest[open_b + 1..close_b];
            for token in inner.split(',') {
                let tok = token.trim();
                if tok.is_empty() {
                    continue;
                }
                if tok.starts_with("0x") || tok.starts_with("0X") {
                    let hex_str = &tok[2..];
                    let val = u32::from_str_radix(hex_str, 16)
                        .map_err(|e| format!("Invalid hex weight '{}': {}", tok, e))?;
                    values.push(val);
                } else if tok.contains('.') || tok.contains('e') || tok.contains('E') {
                    let f = tok.parse::<f32>()
                        .map_err(|e| format!("Invalid float weight '{}': {}", tok, e))?;
                    values.push(f.to_bits());
                } else {
                    let val = tok.parse::<u32>()
                        .map_err(|e| format!("Invalid integer weight '{}': {}", tok, e))?;
                    values.push(val);
                }
            }
        }
    }

    // Extract bank= and offset= from key-value pairs
    for token in rest.split([',', ':', ' ', '\t']) {
        let tok = token.trim();
        if let Some(val_str) = tok.strip_prefix("bank=") {
            let clean = val_str.trim_matches(|c: char| !c.is_ascii_hexdigit());
            if let Ok(b) = clean.parse::<usize>() {
                bank = b;
            }
        } else if let Some(val_str) = tok.strip_prefix("offset=") {
            let clean = val_str.trim_matches(|c: char| !c.is_ascii_hexdigit() && c != 'x' && c != 'X');
            if clean.starts_with("0x") || clean.starts_with("0X") {
                if let Ok(off) = usize::from_str_radix(&clean[2..], 16) {
                    offset = off;
                }
            } else if let Ok(off) = clean.parse::<usize>() {
                offset = off;
            }
        }
    }

    // If file_path is specified and file exists, load bytes
    if let Some(ref path) = file_path {
        if let Ok(bytes) = std::fs::read(path) {
            for chunk in bytes.chunks_exact(4) {
                let val = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                values.push(val);
            }
        }
    }

    Ok(ClWeightBinding {
        bank,
        offset,
        values,
        file_path,
    })
}

/// Helper that splits comma-separated directive tokens while respecting quotes and parentheses
fn split_directive_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut paren_depth = 0;

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                current.push(c);
            }
            '(' | '[' | '{' if !in_quotes => {
                paren_depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' if !in_quotes => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
                current.push(c);
            }
            ',' if !in_quotes && paren_depth == 0 => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    tokens.push(trimmed.to_string());
                }
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        tokens.push(trimmed.to_string());
    }
    tokens
}

/// Parse any semantic directive line (.clifford, .stage, .tensor, .fuse, .flow, .layout, .core, .weights)
pub fn parse_directive(line: &str) -> Result<ClDirective, String> {
    let trimmed = line.trim();
    if trimmed.starts_with(".weights") {
        return parse_weights_directive(trimmed).map(ClDirective::Weights);
    }

    if let Some(rest) = trimmed.strip_prefix(".clifford") {
        let rest = rest.trim();
        let mut rotor = "Rotor4D".to_string();
        let mut vector = "Vector4D".to_string();
        let mut algebra = "Cl(4,0)".to_string();
        for token in split_directive_tokens(rest) {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("rotor=") {
                rotor = val.trim().trim_matches('"').to_string();
            } else if let Some(val) = tok.strip_prefix("vector=") {
                vector = val.trim().trim_matches('"').to_string();
            } else if let Some(val) = tok.strip_prefix("algebra=") {
                algebra = val.trim().trim_matches('"').to_string();
            }
        }
        return Ok(ClDirective::Clifford { rotor, vector, algebra });
    }

    if let Some(rest) = trimmed.strip_prefix(".stage") {
        let rest = rest.trim();
        let mut id = String::new();
        let mut params = String::new();
        let mut precision = "f32".to_string();
        let mut d_model = 64;
        let mut heads = 4;
        let mut kv_heads = 4;
        let mut intermediate = 128;
        let mut zero_overhead = true;

        if let Some(start_q) = rest.find('"') {
            if let Some(end_q) = rest[start_q + 1..].find('"') {
                id = rest[start_q + 1..start_q + 1 + end_q].to_string();
            }
        }

        for token in split_directive_tokens(rest) {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("params=") {
                params = val.trim_matches('"').to_string();
            } else if let Some(val) = tok.strip_prefix("precision=") {
                precision = val.trim_matches('"').to_string();
            } else if let Some(val) = tok.strip_prefix("d_model=") {
                if let Ok(n) = val.parse::<usize>() { d_model = n; }
            } else if let Some(val) = tok.strip_prefix("heads=") {
                if let Ok(n) = val.parse::<usize>() { heads = n; }
            } else if let Some(val) = tok.strip_prefix("kv_heads=") {
                if let Ok(n) = val.parse::<usize>() { kv_heads = n; }
            } else if let Some(val) = tok.strip_prefix("intermediate=") {
                if let Ok(n) = val.parse::<usize>() { intermediate = n; }
            } else if let Some(val) = tok.strip_prefix("zero_overhead=") {
                zero_overhead = val == "true";
            }
        }
        return Ok(ClDirective::Stage { id, params, precision, d_model, heads, kv_heads, intermediate, zero_overhead });
    }

    if let Some(rest) = trimmed.strip_prefix(".fuse") {
        let rest = rest.trim();
        return Ok(ClDirective::Fuse { chain: rest.to_string() });
    }

    if let Some(rest) = trimmed.strip_prefix(".tensor") {
        let rest = rest.trim();
        let mut dims = Vec::new();
        let name = if let Some(colon_pos) = rest.find(':') {
            let n = rest[..colon_pos].trim().to_string();
            let dim_part = &rest[colon_pos + 1..];
            if let Some(open) = dim_part.find('[') {
                if let Some(close) = dim_part.find(']') {
                    for d in dim_part[open + 1..close].split(',') {
                        if let Ok(v) = d.trim().parse::<usize>() {
                            dims.push(v);
                        }
                    }
                }
            }
            n
        } else {
            rest.to_string()
        };
        return Ok(ClDirective::Tensor { name, dims });
    }

    if let Some(rest) = trimmed.strip_prefix(".flow") {
        let rest = rest.trim();
        let mut route = rest.to_string();
        let mut dor = "XYZW".to_string();
        if let Some(dor_pos) = rest.find("dor=") {
            dor = rest[dor_pos + 4..].trim_matches(|c: char| c == '}' || c == ' ' || c == '"').to_string();
            if let Some(brace_pos) = rest.find('{') {
                route = rest[..brace_pos].trim().to_string();
            }
        }
        return Ok(ClDirective::Flow { route, dor });
    }

    if let Some(rest) = trimmed.strip_prefix(".layout") {
        let rest = rest.trim();
        let mut tp = 1; let mut ep = 1; let mut cp = 1; let mut pp = 1;
        let mut dim = "4x4x4x4".to_string();
        let mut chip = "256_core_torus".to_string();
        for token in rest.trim_matches(|c| c == '{' || c == '}').split(',') {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("TP=") { if let Ok(n) = val.parse::<usize>() { tp = n; } }
            else if let Some(val) = tok.strip_prefix("EP=") { if let Ok(n) = val.parse::<usize>() { ep = n; } }
            else if let Some(val) = tok.strip_prefix("CP=") { if let Ok(n) = val.parse::<usize>() { cp = n; } }
            else if let Some(val) = tok.strip_prefix("PP=") { if let Ok(n) = val.parse::<usize>() { pp = n; } }
            else if let Some(val) = tok.strip_prefix("dim=") { dim = val.trim_matches('"').to_string(); }
            else if let Some(val) = tok.strip_prefix("chip=") { chip = val.trim_matches('"').to_string(); }
        }
        return Ok(ClDirective::Layout { tp, ep, cp, pp, dim, chip });
    }

    if let Some(rest) = trimmed.strip_prefix(".core") {
        let rest = rest.trim().trim_end_matches(':').trim();
        let mut coords = [0usize; 4];
        if let Some(open) = rest.find('[') {
            if let Some(close) = rest.find(']') {
                let parts: Vec<&str> = rest[open + 1..close].split(',').collect();
                for (i, p) in parts.iter().enumerate().take(4) {
                    if let Ok(val) = p.trim().parse::<usize>() {
                        coords[i] = val;
                    }
                }
            }
        }
        return Ok(ClDirective::Core { coords });
    }

    if let Some(rest) = trimmed.strip_prefix(".circuit") {
        let rest = rest.trim();
        let mut core_id = 0;
        let mut layer = "L23".to_string();
        let mut excitatory = 80;
        let mut inhibitory = 20;
        let mut tau_us = 15000;
        let mut dopamine = 0.5f32;

        for part in rest.split_whitespace() {
            if let Some(val) = part.strip_prefix("core=") {
                if let Ok(c) = val.parse::<usize>() { core_id = c; }
            } else if let Some(val) = part.strip_prefix("layer=") {
                layer = val.to_string();
            } else if let Some(val) = part.strip_prefix("exc=") {
                if let Ok(e) = val.parse::<usize>() { excitatory = e; }
            } else if let Some(val) = part.strip_prefix("inh=") {
                if let Ok(i) = val.parse::<usize>() { inhibitory = i; }
            } else if let Some(val) = part.strip_prefix("tau=") {
                if let Ok(t) = val.parse::<usize>() { tau_us = t; }
            } else if let Some(val) = part.strip_prefix("da=") {
                if let Ok(d) = val.parse::<f32>() { dopamine = d; }
            }
        }
        return Ok(ClDirective::Circuit {
            core_id,
            layer,
            excitatory,
            inhibitory,
            tau_us,
            dopamine,
        });
    }

    if let Some(rest) = trimmed.strip_prefix(".loop") {
        let rest = rest.trim();
        let mut count = 1usize;
        let mut stride = 1usize;
        for token in split_directive_tokens(rest.trim_end_matches('{').trim()) {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("count=") {
                if let Ok(c) = val.parse::<usize>() { count = c; }
            } else if let Some(val) = tok.strip_prefix("stride=") {
                if let Ok(s) = val.parse::<usize>() { stride = s; }
            }
        }
        return Ok(ClDirective::Loop { count, stride });
    }

    if let Some(rest) = trimmed.strip_prefix(".barrier") {
        let rest = rest.trim();
        let mut name = "sync".to_string();
        let mut cores = "all".to_string();
        for token in split_directive_tokens(rest) {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("name=") {
                name = val.trim_matches('"').to_string();
            } else if let Some(val) = tok.strip_prefix("cores=") {
                cores = val.trim_matches('"').to_string();
            }
        }
        return Ok(ClDirective::Barrier { name, cores });
    }

    if trimmed.starts_with(".sram") || trimmed.starts_with(".hbm") || trimmed.starts_with(".noc") {
        let (space, rest) = if let Some(r) = trimmed.strip_prefix(".sram") {
            ("sram", r.trim())
        } else if let Some(r) = trimmed.strip_prefix(".hbm") {
            ("hbm", r.trim())
        } else if let Some(r) = trimmed.strip_prefix(".noc") {
            ("noc", r.trim())
        } else {
            ("unknown", trimmed)
        };
        let mut size_kb = if space == "sram" { 64 } else if space == "hbm" { 16777216 } else { 0 };
        let mut bank = 0usize;
        for token in split_directive_tokens(rest) {
            let tok = token.trim();
            if let Some(val) = tok.strip_prefix("size=") {
                let v = val.trim_end_matches("KB").trim_end_matches("kb").trim_end_matches("MB").trim_end_matches("GB");
                if let Ok(s) = v.parse::<usize>() {
                    if val.contains("GB") || val.contains("gb") {
                        size_kb = s * 1024 * 1024;
                    } else if val.contains("MB") || val.contains("mb") {
                        size_kb = s * 1024;
                    } else {
                        size_kb = s;
                    }
                }
            } else if let Some(val) = tok.strip_prefix("bank=") {
                if let Ok(b) = val.parse::<usize>() { bank = b; }
            }
        }
        return Ok(ClDirective::MemorySpace { space: space.to_string(), size_kb, bank });
    }

    if trimmed.starts_with(".kernel") {
        return parse_kernel_header(trimmed).map(ClDirective::Kernel);
    }

    let name = trimmed.split_whitespace().next().unwrap_or(trimmed).to_string();
    Ok(ClDirective::Custom { name, content: trimmed.to_string() })
}

pub fn verify_cl_program(content: &str) -> Result<ClReport, String> {
    let mut report = ClReport::default();
    let mut line_num = 0;
    let mut in_kernel: Option<ClKernelDef> = None;

    for line in content.lines() {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        // Handle inside multi-line .kernel block
        if let Some(ref mut k) = in_kernel {
            if trimmed == "}" || trimmed.ends_with('}') {
                let mut completed = in_kernel.take().unwrap();
                if trimmed.len() > 1 && trimmed != "}" {
                    let stmt = trimmed.trim_end_matches('}').trim();
                    if !stmt.is_empty() {
                        completed.body_statements.push(stmt.to_string());
                    }
                }
                report.directives_found += 1;
                report.directives_parsed.push(ClDirective::Kernel(completed));
            } else if trimmed != "{" {
                k.body_statements.push(trimmed.to_string());
            }
            continue;
        }

        if trimmed == "{" || trimmed == "}" {
            continue;
        }

        // Check for Label or Kernel definitions (e.g. @kernel foo or @label: or L00:)
        if trimmed.starts_with('@')
            || (trimmed.starts_with('L') && trimmed.ends_with(':') && trimmed.len() <= 20)
        {
            report.labels_found += 1;
            continue;
        }

        // Check for .kernel directives (which may start a block)
        if trimmed.starts_with(".kernel") {
            if let Ok(k) = parse_kernel_header(trimmed) {
                if trimmed.contains('{') {
                    in_kernel = Some(k);
                } else {
                    report.directives_found += 1;
                    report.directives_parsed.push(ClDirective::Kernel(k));
                }
            }
            continue;
        }

        // Check for Semantic Directives
        if trimmed.starts_with('.') {
            if let Ok(directive) = parse_directive(trimmed) {
                match &directive {
                    ClDirective::Core { .. } => {
                        report.cores_partitioned += 1;
                    }
                    ClDirective::Circuit { .. } => {
                        report.circuits_configured += 1;
                        report.directives_found += 1;
                    }
                    ClDirective::Weights(binding) => {
                        report.weights_bound += 1;
                        report.total_weight_bytes += binding.values.len() * 4;
                        report.directives_found += 1;
                    }
                    _ => {
                        report.directives_found += 1;
                    }
                }
                report.directives_parsed.push(directive);
            }
            continue;
        }

        let (cycle_part, slots_str) = match trimmed.split_once(':') {
            Some((c, s)) => (c.trim(), s.trim()),
            None => {
                return Err(format!(
                    "Syntax Error at line {}: Missing bundle cycle header (expected 'B<cycle>:', got '{}')",
                    line_num, trimmed
                ));
            }
        };

        if !cycle_part.starts_with('B') && !cycle_part.starts_with('b') {
            return Err(format!(
                "Syntax Error at line {}: Missing bundle cycle header (expected 'B<cycle>:', got '{}')",
                line_num, cycle_part
            ));
        }

        let mut slot_count = 0;
        let mut dest_regs_mask: u16 = 0;
        let mut optical_count_in_bundle = 0;

        for slot_str in slots_str.split_whitespace() {
            slot_count += 1;
            if slot_count > 4 {
                return Err(format!(
                    "Bundle Error at line {}: Each .cl VLIW bundle must contain between 1 and 4 slots, found > 4",
                    line_num
                ));
            }

            let slot = parse_slot(slot_str)
                .map_err(|e| format!("Line {}: {}", line_num, e))?;

            // Opcode verification
            if !KNOWN_OPCODES.contains(&slot.opcode.as_str()) {
                return Err(format!(
                    "Line {}: Unknown .cl opcode mnemonic '{}' in slot '{}'",
                    line_num, slot.opcode, slot_str
                ));
            }
            report.opcodes_verified += 1;
            report.total_slots += 1;

            // Optical structural hazard check (1 physical MZI mesh per core)
            if slot.opcode == "OP" {
                optical_count_in_bundle += 1;
                if optical_count_in_bundle > 1 {
                    let msg = format!(
                        "Line {}: Structural hazard: Multiple Photonic MZI Optical operations ({}) scheduled in cycle {}",
                        line_num, optical_count_in_bundle, cycle_part
                    );
                    report.hazards.push(msg);
                }
            }

            // RAW (Read-After-Write) hazard check within same cycle
            if let Some(src) = slot.src_reg {
                let s = (src % 16) as usize;
                if s > 0 && (dest_regs_mask & (1u16 << s)) != 0 {
                    let msg = format!(
                        "Line {}: Read-After-Write (RAW) latency hazard: Register R{:X} read in same cycle before write commits in {}",
                        line_num, s, cycle_part
                    );
                    report.hazards.push(msg);
                }
            }
            if let Some(src2) = slot.src2_reg {
                let s2 = (src2 % 16) as usize;
                if s2 > 0 && (dest_regs_mask & (1u16 << s2)) != 0 {
                    let msg = format!(
                        "Line {}: Read-After-Write (RAW) latency hazard: Register R{:X} (src2) read in same cycle before write commits in {}",
                        line_num, s2, cycle_part
                    );
                    report.hazards.push(msg);
                }
            }

            // Parity token check & CRC-8 Token Integrity (Pages 70-80)
            if slot.parity_token.is_ascii_hexdigit() || slot.parity_token.is_ascii_alphanumeric() {
                report.parity_verified += 1;
            }
            if verify_token_crc8(slot_str) {
                report.strict_crc_verified += 1;
            }
            if verify_token_crc8(slot_str) || slot.parity_token.is_ascii_hexdigit() {
                report.crc_verified += 1;
            }

            // Hazard check: Write-After-Write (WAW) conflict detection
            if let Some(dest) = slot.dest_reg {
                let d = (dest % 16) as usize;
                if d > 0 && slot.opcode != "HL" && slot.opcode != "NO" {
                    let mask = 1u16 << d;
                    if (dest_regs_mask & mask) != 0 {
                        let msg = format!(
                            "Line {}: Write-After-Write (WAW) hazard detected on Register R{:X} within cycle {}",
                            line_num, d, cycle_part
                        );
                        report.hazards.push(msg);
                    } else {
                        dest_regs_mask |= mask;
                    }
                }
            }
        }

        if slot_count == 0 {
            return Err(format!(
                "Bundle Error at line {}: Each .cl VLIW bundle must contain between 1 and 4 slots, found 0",
                line_num
            ));
        }

        report.total_bundles += 1;
    }

    if report.total_bundles == 0 && report.directives_found == 0 {
        return Err("Empty .cl program: No valid VLIW instruction bundles or directives found".to_string());
    }

    Ok(report)
}

/// CRC-8 ATM Polynomial: x^8 + x^2 + x + 1 (0x07) (Pages 70-80)
/// Zero-cost instruction token integrity calculation for 128-bit / 512-bit VLIW instructions.
pub fn compute_crc8_atm(data: &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Pack a 9-character instruction slot with 1-character hex CRC-8 nibble at index 7.
pub fn pack_slot_with_crc8(
    prefix: char,
    op: &str,
    dest_hex: &str,
    mode: char,
    src_char: char,
    imm_char: char,
    term: char,
) -> String {
    let payload = format!("{}{}{}{}{}{}", prefix, op, dest_hex, mode, src_char, imm_char);
    let crc8 = compute_crc8_atm(payload.as_bytes());
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let crc_nibble = HEX_DIGITS[(crc8 & 0x0F) as usize] as char;
    format!(
        "{}{}{}{}{}{}{}{}",
        prefix, op, dest_hex, mode, src_char, crc_nibble, imm_char, term
    )
}

/// Pack a 16-character instruction slot with 1-character hex CRC-8 nibble at index 14.
/// Layout: prefix(1) + op(2) + dest_hex(2) + mode(1) + src1_hex(2) + src2_hex(2) + imm_hex4(4) + crc_nibble(1) + term(1) = 16 chars.
pub fn pack_slot_16_with_crc8(
    prefix: char,
    op: &str,
    dest_hex: &str,
    mode: char,
    src1_hex: &str,
    src2_hex: &str,
    imm_hex4: &str,
    term: char,
) -> String {
    let payload = format!(
        "{}{}{}{}{}{}{}",
        prefix, op, dest_hex, mode, src1_hex, src2_hex, imm_hex4
    );
    let crc8 = compute_crc8_atm(payload.as_bytes());
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let crc_nibble = HEX_DIGITS[(crc8 & 0x0F) as usize] as char;
    format!("{}{}{}", payload, crc_nibble, term)
}

/// Verify CRC-8 integrity of a 10-character or 16-character .cl instruction token
/// Verify CRC-8 integrity of a 10-character or 16-character .cl instruction token
pub fn verify_token_crc8(raw: &str) -> bool {
    if raw.len() == 10 {
        let chars: Vec<char> = raw.chars().collect();
        if chars[9] == compute_crc8_token(&raw[..9]) {
            return true;
        }
        let payload = format!(
            "{}{}{}{}{}{}{}{}",
            chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[8]
        );
        let actual_crc = compute_crc8_atm(payload.as_bytes()) & 0x0F;
        let expected_nibble = chars[7].to_digit(16).unwrap_or(0xFF) as u8;
        actual_crc == expected_nibble
    } else if raw.len() == 16 {
        let payload = &raw[..14];
        let actual_crc = compute_crc8_atm(payload.as_bytes()) & 0x0F;
        let expected_nibble = raw.as_bytes()[14] as char;
        let expected = expected_nibble.to_digit(16).unwrap_or(0xFF) as u8;
        actual_crc == expected
    } else {
        false
    }
}

#[inline]
pub fn compute_crc8_token(raw9: &str) -> char {
    let crc = compute_crc8_atm(raw9.as_bytes());
    const ASCII_MAP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[\\]^_`{|}~";
    ASCII_MAP[(crc as usize) % ASCII_MAP.len()] as char
}

/// Unified canonical parser for immediate values and branch targets in CL instruction tokens.
/// Standard 10-character slots store 12-bit immediate in chars [6..9] and parity/term in [9].
/// Branch targets store full 16-bit cycle target in chars [6..10].
pub fn parse_slot_immediate(slot: &str) -> Option<u32> {
    let bytes = slot.as_bytes();
    if bytes.len() == 16 {
        if let Some(val) = parse_hex4_bytes(&bytes[10..14]) {
            return Some(val as u32);
        }
    }
    if bytes.len() == 10 {
        if let Some(hash_pos) = bytes.iter().position(|&b| b == b'#') {
            // Check if slot has valid canonical CRC-8 token at index 9
            if let Ok(raw9) = std::str::from_utf8(&bytes[..9]) {
                if compute_crc8_token(raw9) == bytes[9] as char {
                    // Valid CRC-8 sealed instruction: immediate is strictly 3 hex digits in bytes[6..9]
                    let mut val = 0u32;
                    if hash_pos + 1 < 9 {
                        for &b in &bytes[hash_pos + 1..9] {
                            if let Some(d) = hex_digit_val(b) {
                                val = (val << 4) | (d as u32);
                            }
                        }
                        return Some(val);
                    }
                }
            }

            let is_branch = bytes[0] == b'!'
                || &bytes[1..3] == b"JP" || &bytes[1..3] == b"BZ"
                || &bytes[1..3] == b"BN" || &bytes[1..3] == b"BL"
                || &bytes[1..3] == b"BG" || &bytes[1..3] == b"JL"
                || &bytes[1..3] == b"JS";

            let rest = if bytes.len() == 10 && hash_pos + 1 < bytes.len() {
                if is_branch {
                    &bytes[hash_pos + 1..]
                } else {
                    &bytes[hash_pos + 1..bytes.len() - 1]
                }
            } else {
                &bytes[hash_pos + 1..]
            };
            if is_branch {
                let mut val = 0u32;
                let mut count = 0;
                for &b in rest.iter().take(4) {
                    if let Some(d) = hex_digit_val(b) {
                        val = (val << 4) | (d as u32);
                        count += 1;
                    } else {
                        break;
                    }
                }
                return if count > 0 { Some(val) } else { None };
            }

            let mut v3 = 0u32;
            let mut c3 = 0;
            for &b in rest.iter().take(3) {
                if let Some(d) = hex_digit_val(b) {
                    v3 = (v3 << 4) | (d as u32);
                    c3 += 1;
                } else {
                    break;
                }
            }

            let mut v4 = 0u32;
            let mut c4 = 0;
            for &b in rest.iter().take(4) {
                if let Some(d) = hex_digit_val(b) {
                    v4 = (v4 << 4) | (d as u32);
                    c4 += 1;
                } else {
                    break;
                }
            }

            if c4 > 0 && c3 > 0 && v3 == 0 && v4 > 0 {
                return Some(v4);
            }
            if c3 > 0 {
                return Some(v3);
            }
            None
        } else {
            hex_digit_val(bytes[8]).map(|d| d as u32)
        }
    } else if let Some(hash_pos) = bytes.iter().position(|&b| b == b'#') {
        let mut val = 0u32;
        let mut count = 0;
        for &b in &bytes[hash_pos + 1..] {
            if let Some(d) = hex_digit_val(b) {
                val = (val << 4) | (d as u32);
                count += 1;
            } else {
                break;
            }
        }
        if count > 0 { Some(val) } else { None }
    } else {
        let mut val = 0u32;
        let mut count = 0;
        for &b in bytes.iter().skip(3) {
            if let Some(d) = hex_digit_val(b) {
                val = (val << 4) | (d as u32);
                count += 1;
            }
        }
        if count > 0 { Some(val) } else { None }
    }
}

/// Convert IEEE 754 half-precision (16-bit) bit pattern to f32
pub fn fp16_bits_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp = ((bits >> 10) & 0x1F) as u32;
    let mant = (bits & 0x3FF) as u32;

    if exp == 0 {
        if mant == 0 {
            f32::from_bits(sign << 31)
        } else {
            let mut m = mant;
            let mut e = 0;
            while (m & 0x400) == 0 {
                m <<= 1;
                e += 1;
            }
            m &= 0x3FF;
            let exp_f32 = (127 - 15 + 1 - e) as u32;
            f32::from_bits((sign << 31) | (exp_f32 << 23) | (m << 13))
        }
    } else if exp == 31 {
        f32::from_bits((sign << 31) | (0xFF << 23) | (mant << 13))
    } else {
        let exp_f32 = (exp + (127 - 15)) as u32;
        f32::from_bits((sign << 31) | (exp_f32 << 23) | (mant << 13))
    }
}

/// Convert f32 to IEEE 754 half-precision (16-bit) bit pattern
pub fn f32_to_fp16_bits(val: f32) -> u16 {
    let bits = val.to_bits();
    let sign = ((bits >> 31) & 1) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let mant = (bits & 0x7FFFFF) as u32;

    if exp == 0xFF {
        (sign << 15) | (0x1F << 10) | ((mant >> 13) as u16).max(if mant != 0 { 1 } else { 0 })
    } else {
        let new_exp = exp - 127 + 15;
        if new_exp >= 31 {
            (sign << 15) | (0x1F << 10)
        } else if new_exp <= 0 {
            0
        } else {
            (sign << 15) | ((new_exp as u16) << 10) | ((mant >> 13) as u16)
        }
    }
}

/// Pack 16 ternary values (-1, 0, +1) into a 32-bit word using BitNet b1.58 encoding (00=0, 01=+1, 10=-1)
pub fn pack_trit16(trits: &[i8; 16]) -> u32 {
    let mut val = 0u32;
    for (i, &t) in trits.iter().enumerate() {
        let code = match t {
            1 => 1u32,
            -1 => 2u32,
            _ => 0u32,
        };
        val |= code << (i * 2);
    }
    val
}

/// Unpack 16 ternary values (-1, 0, +1) from a 32-bit word
pub fn unpack_trit16(val: u32) -> [i8; 16] {
    let mut trits = [0i8; 16];
    for i in 0..16 {
        let code = (val >> (i * 2)) & 0x3;
        trits[i] = match code {
            1 => 1,
            2 => -1,
            _ => 0,
        };
    }
    trits
}

/// Pack 8 4-bit integers into a 32-bit word (QuaRot / W4A4)
pub fn pack_int4x8(nibbles: &[i8; 8]) -> u32 {
    let mut val = 0u32;
    for (i, &n) in nibbles.iter().enumerate() {
        val |= (((n as u8) & 0x0F) as u32) << (i * 4);
    }
    val
}

/// Unpack 8 4-bit integers from a 32-bit word
pub fn unpack_int4x8(val: u32) -> [i8; 8] {
    let mut nibbles = [0i8; 8];
    for i in 0..8 {
        let raw = ((val >> (i * 4)) & 0x0F) as u8;
        nibbles[i] = if raw >= 8 { (raw as i8) - 16 } else { raw as i8 };
    }
    nibbles
}

/// Pack two f32 floats into a 32-bit word as two IEEE 754 half-precision FP16 floats
pub fn pack_fp16x2(a: f32, b: f32) -> u32 {
    let bits_a = f32_to_fp16_bits(a) as u32;
    let bits_b = f32_to_fp16_bits(b) as u32;
    (bits_b << 16) | bits_a
}

/// Unpack two IEEE 754 half-precision FP16 floats from a 32-bit word
pub fn unpack_fp16x2(val: u32) -> (f32, f32) {
    let bits_a = (val & 0xFFFF) as u16;
    let bits_b = ((val >> 16) & 0xFFFF) as u16;
    (fp16_bits_to_f32(bits_a), fp16_bits_to_f32(bits_b))
}

/// Parse a tensor argument, e.g. "%Q: trit16[32, 128]" or "%out: fp16x2[16]"
pub fn parse_tensor_arg(s: &str) -> Option<ClTensorArg> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let (name, rest) = if let Some((n, r)) = trimmed.split_once(':') {
        (n.trim().to_string(), r.trim())
    } else {
        (trimmed.to_string(), "f32")
    };

    let mut dtype = rest.to_string();
    let mut dims = Vec::new();
    if let Some(open) = rest.find('[') {
        if let Some(close) = rest.rfind(']') {
            dtype = rest[..open].trim().to_string();
            let inside = &rest[open + 1..close];
            for d in inside.split(',') {
                if let Ok(v) = d.trim().parse::<usize>() {
                    dims.push(v);
                }
            }
        }
    }
    Some(ClTensorArg { name, dtype, dims })
}

/// Parse a .kernel directive header:
/// Syntax: `.kernel <name>(%in1: type[dims], ...) -> (%out1: type[dims]) [attr=val, ...]`
pub fn parse_kernel_header(line: &str) -> Result<ClKernelDef, String> {
    let trimmed = line.trim();
    let after_kernel = trimmed.strip_prefix(".kernel")
        .ok_or_else(|| format!("Expected .kernel, got '{}'", line))?
        .trim();

    let clean = after_kernel.trim_end_matches('{').trim();

    let (name_and_attrs, args_part) = if let Some(open_p) = clean.find('(') {
        (&clean[..open_p], Some(&clean[open_p..]))
    } else {
        (clean, None)
    };

    let mut name = name_and_attrs.trim().to_string();
    let mut attributes = std::collections::HashMap::new();

    if name.contains('=') {
        for token in split_directive_tokens(name_and_attrs) {
            let tok = token.trim();
            if let Some((k, v)) = tok.split_once('=') {
                let key = k.trim().to_string();
                let val = v.trim().trim_matches('"').to_string();
                if key == "name" {
                    name = val.clone();
                }
                attributes.insert(key, val);
            }
        }
    }

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    if let Some(args_str) = args_part {
        let (in_str, out_str) = if let Some((i_part, o_part)) = args_str.split_once("->") {
            (i_part.trim(), Some(o_part.trim()))
        } else {
            (args_str.trim(), None)
        };

        let in_clean = in_str.trim_matches(|c| c == '(' || c == ')' || c == ' ');
        for arg_tok in split_directive_tokens(in_clean) {
            if let Some(arg) = parse_tensor_arg(&arg_tok) {
                inputs.push(arg);
            }
        }

        if let Some(out_part) = out_str {
            let out_clean = out_part.trim_matches(|c| c == '(' || c == ')' || c == ' ' || c == '{');
            for arg_tok in split_directive_tokens(out_clean) {
                if let Some(arg) = parse_tensor_arg(&arg_tok) {
                    outputs.push(arg);
                }
            }
        }
    }

    Ok(ClKernelDef {
        name,
        inputs,
        outputs,
        attributes,
        body_statements: Vec::new(),
    })
}

/// Lower high-level tensor kernel AST into 16-character 3-address VLIW bundles
pub fn lower_kernel_to_vliw16(kernel: &ClKernelDef, start_cycle: usize) -> Vec<String> {
    let mut bundles = Vec::new();
    let mut cycle = start_cycle;
    let nop_slot = pack_slot_16_with_crc8('_', "NO", "00", '#', "00", "00", "0000", '>');

    let is_empty = kernel.body_statements.is_empty();
    let k_type = kernel.attributes.get("type").map(|s| s.as_str())
        .or_else(|| if is_empty { Some(kernel.name.as_str()) } else { None });

    if let Some(t) = k_type {
        match t {
            "flash-attn" | "flash_attn" => {
                let s0 = pack_slot_16_with_crc8('_', "QK", "01", '$', "01", "00", "0000", '>');
                let s1 = pack_slot_16_with_crc8('_', "QV", "02", '$', "02", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s0, s1, nop_slot, nop_slot));
                cycle += 1;
                let s2 = pack_slot_16_with_crc8('_', "QA", "04", '$', "01", "02", "0000", '>');
                let s3 = pack_slot_16_with_crc8('_', "SM", "06", '$', "04", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s2, s3, nop_slot, nop_slot));
                cycle += 1;
                let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
                return bundles;
            }
            "bitnet-gemm" | "bitnet_linear" => {
                let s0 = pack_slot_16_with_crc8('_', "SZ", "05", '$', "01", "02", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s0, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s1 = pack_slot_16_with_crc8('_', "MD", "08", '.', "05", "03", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s1, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s2 = pack_slot_16_with_crc8('_', "TM", "09", '*', "08", "04", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s2, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s3 = pack_slot_16_with_crc8('_', "PO", "0A", '+', "09", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s3, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
                return bundles;
            }
            "rmsnorm" | "rms_norm" => {
                let s0 = pack_slot_16_with_crc8('_', "RM", "04", '@', "01", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s0, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s1 = pack_slot_16_with_crc8('_', "FM", "04", '+', "04", "02", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s1, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
                return bundles;
            }
            "swiglu" => {
                let s0 = pack_slot_16_with_crc8('_', "SI", "07", '$', "05", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s0, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s1 = pack_slot_16_with_crc8('_', "PO", "08", '*', "07", "06", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s1, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s2 = pack_slot_16_with_crc8('_', "FM", "09", '+', "08", "02", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s2, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s3 = pack_slot_16_with_crc8('_', "MX", "0A", '>', "09", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s3, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
                return bundles;
            }
            "clifford-rotate4d" | "rotor4d_rotate" => {
                let s0 = pack_slot_16_with_crc8('_', "TP", "01", '#', "01", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s0, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s1 = pack_slot_16_with_crc8('_', "CR", "02", '$', "01", "02", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s1, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let s2 = pack_slot_16_with_crc8('_', "WF", "03", '$', "02", "00", "0000", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, s2, nop_slot, nop_slot, nop_slot));
                cycle += 1;
                let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
                bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
                return bundles;
            }
            _ => {}
        }
    }

    let mut env = KernelRegisterEnv::new();
    for (idx, inp) in kernel.inputs.iter().enumerate() {
        env.bind(&inp.name, (idx + 1).min(14));
    }
    for (idx, out) in kernel.outputs.iter().enumerate() {
        env.bind(&out.name, (idx + 4).min(15));
    }

    let parsed_stmts = parse_kernel_stmts(&kernel.body_statements);
    let mut label_map = std::collections::HashMap::new();
    let mut has_halt = false;

    lower_stmts_hierarchical(
        &parsed_stmts,
        &mut cycle,
        &mut bundles,
        &mut env,
        &mut label_map,
        &nop_slot,
        &mut has_halt,
    );

    if !has_halt {
        let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
        bundles.push(format!("B{:04}: {} {} {} {}", cycle, h0, nop_slot, nop_slot, nop_slot));
    }

    bundles
}

#[derive(Debug, Clone)]
enum KernelStmt {
    Simple(String),
    Loop {
        count: usize,
        body: Vec<KernelStmt>,
    },
    IfElse {
        cond_var: String,
        then_body: Vec<KernelStmt>,
        else_body: Vec<KernelStmt>,
    },
    Label(String),
    Jump(String),
    BranchZero { cond_var: String, target: String },
    BranchNonZero { cond_var: String, target: String },
}

fn parse_kernel_stmts(raw_lines: &[String]) -> Vec<KernelStmt> {
    let mut idx = 0;
    let (stmts, _) = parse_stmt_block(raw_lines, &mut idx, false);
    stmts
}

fn parse_stmt_block(raw_lines: &[String], idx: &mut usize, stop_on_closing_brace: bool) -> (Vec<KernelStmt>, bool) {
    let mut stmts = Vec::new();
    while *idx < raw_lines.len() {
        let line = raw_lines[*idx].trim();
        *idx += 1;
        if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
            continue;
        }
        if stop_on_closing_brace && (line == "}" || line.starts_with('}')) {
            let has_else = line.contains("else");
            return (stmts, has_else);
        }
        if line.starts_with("repeat ") || line.starts_with("loop ") {
            let count_part = line.split_whitespace().nth(1).unwrap_or("4").trim_end_matches('{').trim();
            let count = count_part.parse::<usize>().unwrap_or(4);
            let (body, _) = parse_stmt_block(raw_lines, idx, true);
            stmts.push(KernelStmt::Loop { count, body });
        } else if line.starts_with("if ") {
            let cond_var = line.strip_prefix("if ").unwrap_or("").trim().trim_end_matches('{').trim().to_string();
            let (then_body, trailing_else) = parse_stmt_block(raw_lines, idx, true);
            let mut else_body = Vec::new();
            if trailing_else || (*idx < raw_lines.len() && raw_lines[*idx].trim().starts_with("else")) {
                if !trailing_else {
                    *idx += 1;
                }
                let (eb, _) = parse_stmt_block(raw_lines, idx, true);
                else_body = eb;
            }
            stmts.push(KernelStmt::IfElse { cond_var, then_body, else_body });
        } else if line.starts_with('@') && line.ends_with(':') {
            let label = line.trim_start_matches('@').trim_end_matches(':').trim().to_string();
            stmts.push(KernelStmt::Label(label));
        } else if line.starts_with("jump ->") || line.starts_with("jump ") {
            let tgt = line.split("->").nth(1).unwrap_or(line).trim().trim_start_matches('@').trim().to_string();
            stmts.push(KernelStmt::Jump(tgt));
        } else if line.starts_with("branch_if_zero") || line.starts_with("branch_zero") {
            let parts: Vec<&str> = line.split("->").collect();
            let cond = parts[0].split_whitespace().nth(1).unwrap_or("%cond").to_string();
            let tgt = parts.get(1).unwrap_or(&"end").trim().trim_start_matches('@').trim().to_string();
            stmts.push(KernelStmt::BranchZero { cond_var: cond, target: tgt });
        } else if line.starts_with("branch_if_nonzero") || line.starts_with("branch_nonzero") {
            let parts: Vec<&str> = line.split("->").collect();
            let cond = parts[0].split_whitespace().nth(1).unwrap_or("%cond").to_string();
            let tgt = parts.get(1).unwrap_or(&"end").trim().trim_start_matches('@').trim().to_string();
            stmts.push(KernelStmt::BranchNonZero { cond_var: cond, target: tgt });
        } else {
            stmts.push(KernelStmt::Simple(line.to_string()));
        }
    }
    (stmts, false)
}

fn lower_stmts_hierarchical(
    stmts: &[KernelStmt],
    cycle: &mut usize,
    bundles: &mut Vec<String>,
    env: &mut KernelRegisterEnv,
    label_map: &mut std::collections::HashMap<String, usize>,
    nop_slot: &str,
    has_halt: &mut bool,
) {
    for s in stmts {
        match s {
            KernelStmt::Simple(line) => {
                lower_single_simple_stmt(line, cycle, bundles, env, nop_slot, has_halt);
            }
            KernelStmt::Label(name) => {
                label_map.insert(name.clone(), *cycle);
            }
            KernelStmt::Loop { count, body } => {
                let loop_reg = env.resolve_hex("%loop_cnt", 14);
                let s_init = pack_slot_16_with_crc8('\'', "==", &loop_reg, '#', "00", "00", &format!("{:04X}", count), '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_init, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
                let loop_start_cycle = *cycle;
                lower_stmts_hierarchical(body, cycle, bundles, env, label_map, nop_slot, has_halt);
                let s_dec = pack_slot_16_with_crc8('_', "PO", &loop_reg, '-', &loop_reg, "00", "0001", '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_dec, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
                let s_bn = pack_slot_16_with_crc8('_', "BN", &loop_reg, '#', &loop_reg, "00", &format!("{:04X}", loop_start_cycle), '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_bn, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
            }
            KernelStmt::IfElse { cond_var, then_body, else_body } => {
                let cond_reg = env.resolve_hex(cond_var, 1);
                let bz_idx = bundles.len();
                let bz_cycle = *cycle;
                bundles.push(String::new());
                *cycle += 1;

                lower_stmts_hierarchical(then_body, cycle, bundles, env, label_map, nop_slot, has_halt);

                if else_body.is_empty() {
                    let target_cycle = *cycle;
                    let s_bz = pack_slot_16_with_crc8('_', "BZ", &cond_reg, '#', &cond_reg, "00", &format!("{:04X}", target_cycle), '>');
                    bundles[bz_idx] = format!("B{:04}: {} {} {} {}", bz_cycle, s_bz, nop_slot, nop_slot, nop_slot);
                } else {
                    let jp_idx = bundles.len();
                    let jp_cycle = *cycle;
                    bundles.push(String::new());
                    *cycle += 1;

                    let else_start_cycle = *cycle;
                    let s_bz = pack_slot_16_with_crc8('_', "BZ", &cond_reg, '#', &cond_reg, "00", &format!("{:04X}", else_start_cycle), '>');
                    bundles[bz_idx] = format!("B{:04}: {} {} {} {}", bz_cycle, s_bz, nop_slot, nop_slot, nop_slot);

                    lower_stmts_hierarchical(else_body, cycle, bundles, env, label_map, nop_slot, has_halt);
                    let after_else_cycle = *cycle;
                    let s_jp = pack_slot_16_with_crc8('_', "JP", "00", '#', "00", "00", &format!("{:04X}", after_else_cycle), '>');
                    bundles[jp_idx] = format!("B{:04}: {} {} {} {}", jp_cycle, s_jp, nop_slot, nop_slot, nop_slot);
                }
            }
            KernelStmt::Jump(target) => {
                let tgt_cycle = label_map.get(target).copied().unwrap_or(0);
                let s_jp = pack_slot_16_with_crc8('_', "JP", "00", '#', "00", "00", &format!("{:04X}", tgt_cycle), '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_jp, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
            }
            KernelStmt::BranchZero { cond_var, target } => {
                let cond_reg = env.resolve_hex(cond_var, 1);
                let tgt_cycle = label_map.get(target).copied().unwrap_or(0);
                let s_bz = pack_slot_16_with_crc8('_', "BZ", &cond_reg, '#', &cond_reg, "00", &format!("{:04X}", tgt_cycle), '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_bz, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
            }
            KernelStmt::BranchNonZero { cond_var, target } => {
                let cond_reg = env.resolve_hex(cond_var, 1);
                let tgt_cycle = label_map.get(target).copied().unwrap_or(0);
                let s_bn = pack_slot_16_with_crc8('_', "BN", &cond_reg, '#', &cond_reg, "00", &format!("{:04X}", tgt_cycle), '>');
                bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s_bn, nop_slot, nop_slot, nop_slot));
                *cycle += 1;
            }
        }
    }
}

fn lower_single_simple_stmt(
    trimmed: &str,
    cycle: &mut usize,
    bundles: &mut Vec<String>,
    env: &mut KernelRegisterEnv,
    nop_slot: &str,
    has_halt: &mut bool,
) {
    // Direct bundle or slot passthrough
    if trimmed.starts_with('B') && trimmed.contains(':') {
        let (_, slots_part) = trimmed.split_once(':').unwrap();
        bundles.push(format!("B{:04}: {}", *cycle, slots_part.trim()));
        *cycle += 1;
        return;
    } else if trimmed.starts_with('_') || trimmed.starts_with('\'') {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let mut s = Vec::new();
        for p in &parts {
            if (p.starts_with('_') || p.starts_with('\'')) && p.len() >= 10 {
                s.push(*p);
            }
        }
        while s.len() < 4 {
            s.push(nop_slot);
        }
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s[0], s[1], s[2], s[3]));
        *cycle += 1;
        return;
    }

    let (lhs_dest, rhs_expr) = if let Some((lhs, rhs)) = trimmed.split_once('=') {
        (Some(lhs.trim()), rhs.trim())
    } else {
        (None, trimmed)
    };
    let p_args = extract_percent_args(rhs_expr);

    if rhs_expr.contains("rms_norm") || rhs_expr.contains("rmsnorm") {
        let d_reg = if let Some(dest) = lhs_dest {
            if dest.contains("norm") || dest == "%out" || dest.is_empty() {
                env.bind(dest, 4);
                "04".to_string()
            } else {
                env.resolve_hex(dest, 4)
            }
        } else {
            "04".to_string()
        };
        let s_reg = if let Some(arg) = p_args.first() {
            env.resolve_hex(arg, 1)
        } else {
            "01".to_string()
        };
        let s0 = pack_slot_16_with_crc8('_', "RM", &d_reg, '@', &s_reg, "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s1 = pack_slot_16_with_crc8('_', "FM", &d_reg, '+', &d_reg, "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s1, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("flash_attn") || rhs_expr.contains("attention") {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let s0 = pack_slot_16_with_crc8('_', "QK", "01", '$', "01", "00", "0000", '>');
        let s1 = pack_slot_16_with_crc8('_', "QV", "02", '$', "02", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, s1, nop_slot, nop_slot));
        *cycle += 1;
        let s2 = pack_slot_16_with_crc8('_', "QA", &d_reg, '$', "01", "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s2, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s3 = pack_slot_16_with_crc8('_', "SM", "06", '$', &d_reg, "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s3, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("swiglu") || rhs_expr.contains("mlp") {
        if let Some(dest) = lhs_dest {
            env.bind(dest, 5);
        }
        let s0 = pack_slot_16_with_crc8('_', "SI", "07", '$', "05", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s1 = pack_slot_16_with_crc8('_', "PO", "08", '*', "07", "06", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s1, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s2 = pack_slot_16_with_crc8('_', "FM", "09", '+', "08", "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s2, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s3 = pack_slot_16_with_crc8('_', "MX", "0A", '>', "09", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s3, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("bitnet") || rhs_expr.contains("ternary") {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let s0 = pack_slot_16_with_crc8('_', "SZ", "05", '$', "01", "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s1 = pack_slot_16_with_crc8('_', "MD", "08", '.', "05", "03", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s1, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s2 = pack_slot_16_with_crc8('_', "TM", "09", '*', "08", "04", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s2, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s3 = pack_slot_16_with_crc8('_', "PO", &d_reg, '+', "09", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s3, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("rotor4d") || rhs_expr.contains("clifford") {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 3);
        let s0 = pack_slot_16_with_crc8('_', "TP", "01", '#', "01", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s1 = pack_slot_16_with_crc8('_', "CR", "02", '$', "01", "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s1, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s2 = pack_slot_16_with_crc8('_', "WF", &d_reg, '$', "02", "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s2, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.starts_with("emit") || rhs_expr.contains("-> Core") || rhs_expr.starts_with("tx") {
        let s_reg = if let Some(arg) = p_args.first() {
            env.resolve_hex(arg, 5)
        } else {
            "05".to_string()
        };
        let s0 = pack_slot_16_with_crc8('_', "TX", &s_reg, '$', &s_reg, "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.starts_with("receive") || rhs_expr.starts_with("rx") {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%in"), 1);
        let s0 = pack_slot_16_with_crc8('_', "RX", &d_reg, '$', &d_reg, "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("embedding") || rhs_expr.contains("lookup") {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let tok_reg = if let Some(arg) = p_args.first() {
            env.resolve_hex(arg, 1)
        } else {
            "01".to_string()
        };
        let s0 = pack_slot_16_with_crc8('_', "LD", &d_reg, '@', &tok_reg, "00", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        let s1 = pack_slot_16_with_crc8('_', "FM", &d_reg, '+', &d_reg, "02", "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s1, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("residual") || rhs_expr.contains("add") || rhs_expr.contains('+') {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let a_reg = if p_args.len() >= 1 { env.resolve_hex(&p_args[0], 1) } else { "01".to_string() };
        let b_reg = if p_args.len() >= 2 { env.resolve_hex(&p_args[1], 2) } else { "02".to_string() };
        let s0 = pack_slot_16_with_crc8('_', "PO", &d_reg, '+', &a_reg, &b_reg, "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("sub") || rhs_expr.contains('-') {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let a_reg = if p_args.len() >= 1 { env.resolve_hex(&p_args[0], 1) } else { "01".to_string() };
        let b_reg = if p_args.len() >= 2 { env.resolve_hex(&p_args[1], 2) } else { "02".to_string() };
        let s0 = pack_slot_16_with_crc8('_', "PO", &d_reg, '-', &a_reg, &b_reg, "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("mul") || rhs_expr.contains('*') {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 4);
        let a_reg = if p_args.len() >= 1 { env.resolve_hex(&p_args[0], 1) } else { "01".to_string() };
        let b_reg = if p_args.len() >= 2 { env.resolve_hex(&p_args[1], 2) } else { "02".to_string() };
        let s0 = pack_slot_16_with_crc8('_', "PO", &d_reg, '*', &a_reg, &b_reg, "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if rhs_expr.contains("halt") {
        let h0 = pack_slot_16_with_crc8('_', "HL", "00", '#', "00", "00", "0000", '!');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, h0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
        *has_halt = true;
    } else if p_args.is_empty() && (rhs_expr.parse::<i64>().is_ok() || rhs_expr.starts_with("0x") || rhs_expr.starts_with("0X")) {
        let imm_val = if rhs_expr.starts_with("0x") || rhs_expr.starts_with("0X") {
            i64::from_str_radix(rhs_expr.trim_start_matches("0x").trim_start_matches("0X"), 16).unwrap_or(0)
        } else {
            rhs_expr.parse::<i64>().unwrap_or(0)
        };
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 1);
        let s0 = pack_slot_16_with_crc8('\'', "==", &d_reg, '#', "00", "00", &format!("{:04X}", imm_val as u16), '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    } else if p_args.is_empty() && (rhs_expr.contains('+') || rhs_expr.contains('-') || rhs_expr.contains('*')) {
        let (val, found) = evaluate_const_expr(rhs_expr);
        if found {
            let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 1);
            let s0 = pack_slot_16_with_crc8('\'', "==", &d_reg, '#', "00", "00", &format!("{:04X}", val as u16), '>');
            bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
            *cycle += 1;
        } else {
            let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 1);
            let s0 = pack_slot_16_with_crc8('_', "PO", &d_reg, '+', "01", "02", "0000", '>');
            bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
            *cycle += 1;
        }
    } else {
        let d_reg = env.resolve_hex(lhs_dest.unwrap_or("%out"), 1);
        let a_reg = if p_args.len() >= 1 { env.resolve_hex(&p_args[0], 1) } else { "01".to_string() };
        let b_reg = if p_args.len() >= 2 { env.resolve_hex(&p_args[1], 2) } else { "02".to_string() };
        let s0 = pack_slot_16_with_crc8('_', "PO", &d_reg, '+', &a_reg, &b_reg, "0000", '>');
        bundles.push(format!("B{:04}: {} {} {} {}", *cycle, s0, nop_slot, nop_slot, nop_slot));
        *cycle += 1;
    }
}

fn evaluate_const_expr(expr: &str) -> (i64, bool) {
    let clean = expr.trim();
    for op in ['+', '-', '*'] {
        if let Some((lhs, rhs)) = clean.split_once(op) {
            let l_trimmed = lhs.trim();
            let r_trimmed = rhs.trim();
            if let (Ok(l), Ok(r)) = (l_trimmed.parse::<i64>(), r_trimmed.parse::<i64>()) {
                let res = match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    _ => 0,
                };
                return (res, true);
            }
        }
    }
    (0, false)
}

struct KernelRegisterEnv {
    var_map: std::collections::HashMap<String, usize>,
    next_alloc: usize,
}

impl KernelRegisterEnv {
    fn new() -> Self {
        Self {
            var_map: std::collections::HashMap::new(),
            next_alloc: 1,
        }
    }

    fn bind(&mut self, var_name: &str, reg: usize) {
        let clean = var_name.trim().trim_start_matches('%').to_lowercase();
        self.var_map.insert(clean, reg);
        if reg >= self.next_alloc {
            self.next_alloc = (reg + 1).min(15);
        }
    }

    fn resolve_reg(&mut self, var_name: &str, default_reg: usize) -> usize {
        let clean = var_name.trim().trim_start_matches('%').to_lowercase();
        if clean.is_empty() {
            return default_reg;
        }
        if (clean.starts_with('r') || clean.starts_with('R')) && clean.len() >= 2 {
            if let Ok(idx) = usize::from_str_radix(&clean[1..], 16) {
                if idx < 16 {
                    return idx;
                }
            }
        }
        if let Some(&reg) = self.var_map.get(&clean) {
            return reg;
        }
        let reg = self.next_alloc;
        self.var_map.insert(clean, reg);
        if self.next_alloc < 15 {
            self.next_alloc += 1;
        }
        reg
    }

    fn resolve_hex(&mut self, var_name: &str, default_reg: usize) -> String {
        let reg = self.resolve_reg(var_name, default_reg);
        format!("{:02X}", reg)
    }
}

fn extract_percent_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices() {
        if c == '%' {
            start = Some(i);
        } else if let Some(st) = start {
            if !c.is_alphanumeric() && c != '_' {
                args.push(s[st..i].to_string());
                start = None;
            }
        }
    }
    if let Some(st) = start {
        args.push(s[st..].to_string());
    }
    args
}

/// Module metadata for .cl modular namespace system
#[derive(Debug, Clone, Default)]
pub struct ClModuleMeta {
    pub name: String,
    pub exports: std::collections::HashSet<String>,
    pub has_explicit_exports: bool,
}

/// Resolve module file path across standard search paths
pub fn resolve_cl_module_path(target: &str) -> Option<std::path::PathBuf> {
    let clean = target.trim().trim_matches('"').trim_matches('\'');
    let mut candidate_names = Vec::new();
    if clean.ends_with(".cl") {
        candidate_names.push(clean.to_string());
    } else {
        candidate_names.push(format!("{}.cl", clean));
        candidate_names.push(clean.to_string());
    }

    // Support dot-separated or colon-separated module syntax: "sovereign.math" -> "sovereign/math.cl"
    let slashes = clean.replace("::", "/").replace(".", "/");
    if slashes != clean {
        if !slashes.ends_with(".cl") {
            candidate_names.push(format!("{}.cl", slashes));
        }
        candidate_names.push(slashes);
    }

    let search_roots = [
        "",
        "../..",
        "libcl",
        "../../libcl",
        "models",
        "../../models",
        "models/sovereign_1b",
        "../../models/sovereign_1b",
        "models/sovereign_3b",
        "../../models/sovereign_3b",
        "examples/cl",
        "../../examples/cl",
    ];

    for name in &candidate_names {
        for root in &search_roots {
            let path = if root.is_empty() {
                std::path::PathBuf::from(name)
            } else {
                std::path::PathBuf::from(root).join(name)
            };
            if path.exists() && path.is_file() {
                if let Ok(canonical) = path.canonicalize() {
                    return Some(canonical);
                }
                return Some(path);
            }
        }
    }
    None
}

/// Lower a full .cl file containing .kernel blocks into pure 16-character machine code
pub fn lower_cl_with_kernels(content: &str) -> Result<String, String> {
    let mut cycle_counter = 0usize;
    let mut visited_paths = std::collections::HashSet::new();
    lower_cl_with_kernels_recursive(content, &mut cycle_counter, &mut visited_paths, None)
}

fn lower_cl_with_kernels_recursive(
    content: &str,
    cycle_counter: &mut usize,
    visited_paths: &mut std::collections::HashSet<std::path::PathBuf>,
    alias_prefix: Option<&str>,
) -> Result<String, String> {
    let mut output = String::new();
    let mut current_kernel: Option<ClKernelDef> = None;
    let mut kernel_brace_depth: usize = 0;
    let mut module_meta = ClModuleMeta::default();

    // Scan for module declarations and export list
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(".module") {
            let mod_name = trimmed.strip_prefix(".module")
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            module_meta.name = mod_name;
        } else if trimmed.starts_with(".export") {
            let exp_part = trimmed.strip_prefix(".export").unwrap_or("").trim();
            module_meta.has_explicit_exports = true;
            let stripped = exp_part.trim_start_matches('{').trim_end_matches('}').trim();
            for sym in stripped.split(',') {
                let s = sym.trim().trim_start_matches('@').trim();
                if !s.is_empty() {
                    module_meta.exports.insert(s.to_string());
                }
            }
        }
    }

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(ref mut k) = current_kernel {
            let mut depth = kernel_brace_depth;
            let mut closed_kernel = false;

            for c in trimmed.chars() {
                if c == '{' {
                    depth += 1;
                } else if c == '}' {
                    if depth > 0 {
                        depth -= 1;
                    }
                    if depth == 0 {
                        closed_kernel = true;
                        break;
                    }
                }
            }
            kernel_brace_depth = depth;

            if closed_kernel {
                let mut completed = current_kernel.take().unwrap();
                if trimmed.len() > 1 && trimmed != "}" {
                    let stmt = trimmed.trim_end_matches('}').trim();
                    if !stmt.is_empty() {
                        completed.body_statements.push(stmt.to_string());
                    }
                }

                let is_exported = !module_meta.has_explicit_exports || module_meta.exports.contains(&completed.name);
                let tag = if is_exported { "Public" } else { "Internal" };

                output.push_str(&format!("; === Lowered AI Kernel ({tag}): {} ===\n", completed.name));
                output.push_str(&format!("@{}:\n", completed.name));
                if let Some(alias) = alias_prefix {
                    output.push_str(&format!("@{}_{}:\n", alias, completed.name));
                    output.push_str(&format!("@{}::{}:\n", alias, completed.name));
                }

                let bundles = lower_kernel_to_vliw16(&completed, *cycle_counter);
                for b in bundles {
                    output.push_str(&b);
                    output.push('\n');
                    *cycle_counter += 1;
                }
            } else if !trimmed.is_empty() && trimmed != "{" {
                k.body_statements.push(trimmed.to_string());
            }
            continue;
        }

        if trimmed.starts_with(".module") || trimmed.starts_with(".export") {
            output.push_str(&format!("; [Directive: {}]\n", trimmed));
            continue;
        }

        if trimmed.starts_with(".use") || trimmed.starts_with(".include") {
            let (target_raw, alias_opt) = if trimmed.starts_with(".use") {
                let rest = trimmed.strip_prefix(".use").unwrap_or("").trim();
                if let Some((target_part, alias_part)) = rest.split_once(" as ") {
                    (target_part.trim(), Some(alias_part.trim()))
                } else {
                    (rest, None)
                }
            } else {
                (trimmed.strip_prefix(".include").unwrap_or("").trim(), None)
            };

            let clean_target = target_raw.trim_matches('"').trim_matches('\'');
            if let Some(resolved_path) = resolve_cl_module_path(clean_target) {
                if visited_paths.contains(&resolved_path) {
                    output.push_str(&format!("; --- Already included module '{}' (skipped duplicate) ---\n", clean_target));
                    continue;
                }
                visited_paths.insert(resolved_path.clone());

                if let Ok(c) = std::fs::read_to_string(&resolved_path) {
                    let lowered_inc = lower_cl_with_kernels_recursive(&c, cycle_counter, visited_paths, alias_opt)?;
                    output.push_str(&format!("; --- Module loaded from '{}' (alias: {:?}) ---\n", resolved_path.display(), alias_opt));
                    output.push_str(&lowered_inc);
                    output.push('\n');
                    continue;
                }
            } else {
                output.push_str(&format!("; [Warning: module '{}' could not be resolved from search paths]\n", clean_target));
                continue;
            }
        }

        if trimmed.starts_with(".kernel") {
            let k = parse_kernel_header(trimmed)?;
            if trimmed.contains('{') {
                current_kernel = Some(k);
                let opens = trimmed.chars().filter(|&c| c == '{').count();
                let closes = trimmed.chars().filter(|&c| c == '}').count();
                kernel_brace_depth = if opens > closes { opens - closes } else { 1 };
            } else {
                let is_exported = !module_meta.has_explicit_exports || module_meta.exports.contains(&k.name);
                let tag = if is_exported { "Public" } else { "Internal" };
                output.push_str(&format!("; === Lowered AI Kernel ({tag}): {} ===\n", k.name));
                output.push_str(&format!("@{}:\n", k.name));
                if let Some(alias) = alias_prefix {
                    output.push_str(&format!("@{}_{}:\n", alias, k.name));
                    output.push_str(&format!("@{}::{}:\n", alias, k.name));
                }
                let bundles = lower_kernel_to_vliw16(&k, *cycle_counter);
                for b in bundles {
                    output.push_str(&b);
                    output.push('\n');
                    *cycle_counter += 1;
                }
            }
            continue;
        }

        output.push_str(line);
        output.push('\n');

        if trimmed.starts_with('B') || trimmed.starts_with('b') {
            if let Some((c_str, _)) = trimmed.split_once(':') {
                let num_str = c_str.trim().trim_start_matches(|c: char| c == 'B' || c == 'b');
                if let Ok(c) = usize::from_str_radix(num_str, 16) {
                    *cycle_counter = (*cycle_counter).max(c + 1);
                } else if let Ok(c) = num_str.parse::<usize>() {
                    *cycle_counter = (*cycle_counter).max(c + 1);
                }
            }
        }
    }

    Ok(output)
}
