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
    )
}

pub fn parse_slot(raw: &str) -> Result<ClSlot, String> {
    let bytes = raw.as_bytes();
    if bytes.len() != 10 {
        return Err(format!(
            "Slot width violation: Each .cl slot must be exactly 10 characters, found {} ('{}')",
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

    let term_b = bytes[9];
    if !term_b.is_ascii_graphic() {
        return Err(format!(
            "Invalid slot terminator '{}' in '{}' (expected valid CL terminator)",
            term_b as char, raw
        ));
    }
    let terminator = term_b as char;

    let is_writer = !is_non_writer_opcode(&op_bytes);
    let dest_reg = if is_writer {
        parse_hex2_bytes(bytes[3], bytes[4])
    } else {
        None
    };

    let mode = bytes[5] as char;
    let src_reg = hex_digit_val(bytes[6]);
    let parity_token = bytes[7] as char;
    let imm_token = bytes[8] as char;

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
    })
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

    let name = trimmed.split_whitespace().next().unwrap_or(trimmed).to_string();
    Ok(ClDirective::Custom { name, content: trimmed.to_string() })
}

pub fn verify_cl_program(content: &str) -> Result<ClReport, String> {
    let mut report = ClReport::default();
    let mut line_num = 0;

    for line in content.lines() {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed == "{" || trimmed == "}" {
            continue;
        }

        // Check for Label or Kernel definitions (e.g. @kernel foo or @label: or L00:)
        if trimmed.starts_with('@')
            || (trimmed.starts_with('L') && trimmed.ends_with(':') && trimmed.len() <= 20)
        {
            report.labels_found += 1;
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
                if src > 0 && (dest_regs_mask & (1 << src)) != 0 {
                    let msg = format!(
                        "Line {}: Read-After-Write (RAW) latency hazard: Register R{:X} read in same cycle before write commits in {}",
                        line_num, src, cycle_part
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
                if dest > 0 && slot.opcode != "HL" && slot.opcode != "NO" {
                    let mask = 1 << dest;
                    if (dest_regs_mask & mask) != 0 {
                        let msg = format!(
                            "Line {}: Write-After-Write (WAW) hazard detected on Register R{:X} within cycle {}",
                            line_num, dest, cycle_part
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

    if report.total_bundles == 0 {
        return Err("Empty .cl program: No valid VLIW instruction bundles found".to_string());
    }

    Ok(report)
}

/// CRC-8 ATM Polynomial: x^8 + x^2 + x + 1 (0x07) (Pages 70-80)
/// Zero-cost instruction token integrity calculation for 128-bit VLIW instructions.
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

/// Verify CRC-8 integrity of a 10-character .cl instruction token
pub fn verify_token_crc8(raw: &str) -> bool {
    if raw.len() != 10 {
        return false;
    }
    let chars: Vec<char> = raw.chars().collect();
    let payload = format!(
        "{}{}{}{}{}{}{}{}",
        chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[8]
    );
    let actual_crc = compute_crc8_atm(payload.as_bytes()) & 0x0F;
    let expected_nibble = chars[7].to_digit(16).unwrap_or(0xFF) as u8;
    actual_crc == expected_nibble
}

/// Unified canonical parser for immediate values and branch targets in CL instruction tokens.
/// Standard 10-character slots store 12-bit immediate in chars [6..9] and parity/term in [9].
/// Branch targets store full 16-bit cycle target in chars [6..10].
pub fn parse_slot_immediate(slot: &str) -> Option<u32> {
    let bytes = slot.as_bytes();
    if bytes.len() == 10 {
        if let Some(hash_pos) = bytes.iter().position(|&b| b == b'#') {
            let is_branch = bytes[0] == b'!'
                || &bytes[1..3] == b"JP" || &bytes[1..3] == b"BZ"
                || &bytes[1..3] == b"BN" || &bytes[1..3] == b"BL"
                || &bytes[1..3] == b"BG" || &bytes[1..3] == b"JL"
                || &bytes[1..3] == b"JS";

            let rest = &bytes[hash_pos + 1..];
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
            return None;
        } else {
            return hex_digit_val(bytes[8]).map(|d| d as u32);
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
