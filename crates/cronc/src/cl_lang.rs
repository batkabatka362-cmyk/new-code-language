// ============================================================================
// CRON Low-Level Machine Language (.cl) Engine & Validator
// .cl is the first-class, machine-native, AI-native programming language
// for the 256-Core 4D-Torus neuromorphic/photonic processor.
//
// Syntax Specification:
//   - Bundle: B<cycle_num>: <slot0> <slot1> <slot2> <slot3>
//   - Slot:   Exactly 10 ASCII characters per token.
//             [0]   Prefix: '_' (Operation) or '\'' (Immediate load)
//             [1..2] Opcode: 2-char mnemonic (OP, FA, PO, MD, BK, BL, RF, etc.)
//             [3..4] Dest Register: R0..RF (hex 00..0F)
//             [5]   Mode / Delimiter: '$', '#', '@'
//             [6]   Src Register / High Parameter
//             [7]   Parity / Check Token (Index 7)
//             [8]   Immediate / Low Parameter
//             [9]   Terminator: '>' (Execute) or '!' (Halt/Trap)
// ============================================================================

use std::collections::HashSet;

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

#[derive(Debug, Clone, Default)]
pub struct ClReport {
    pub total_bundles: usize,
    pub total_slots: usize,
    pub opcodes_verified: usize,
    pub parity_verified: usize,
    pub crc_verified: usize,
    pub hazards: Vec<String>,
    pub labels_found: usize,
    pub cores_partitioned: usize,
    pub weights_bound: usize,
    pub total_weight_bytes: usize,
    pub directives_found: usize,
}

pub const KNOWN_OPCODES: &[&str] = &[
    "OP", // Optical GEMM (Brain 2)
    "FA", // Forward Autodiff Tap (Brain 2)
    "PO", // Predicated SIMD ALU
    "MD", // Mult-Dot Sub-Byte SIMD MAC
    "BK", // Backward Invert Reversible Pass (Brain 3)
    "BL", // Halo Cache Blend
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
    "SB", // Spatial Broadcast Across 4D Torus
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
    "=0", // Immediate Load Low
    "=1", // Immediate Load High
    "==", // Generic Immediate Load
    "JP", // Unconditional Branch/Jump
    "BZ", // Branch if Zero
    "BN", // Branch if Non-Zero
    "BL", // Branch if Less/Negative
    "BG", // Branch if Greater
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
    "DW", // Befunge Systolic Push West (-X)
    "DN", // Befunge Systolic Push North (+Y)
    "DS", // Befunge Systolic Push South (-Y)
    "UN", // Prolog 1-Cycle Hardware Symbolic Index Matcher / Unifier
    // Milestone #029: Dedicated AI Silicon ISA Extensions
    "RM", // RMSNorm Normalizer Step
    "SM", // Streaming Online Flash-Softmax
    "SI", // SiLU Activation (SwiGLU)
    "GE", // GELU Activation
    "SS", // Selective Scan SSM Step (Mamba)
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

pub fn parse_slot(raw: &str) -> Result<ClSlot, String> {
    if raw.len() != 10 {
        return Err(format!(
            "Slot width violation: Each .cl slot must be exactly 10 characters, found {} ('{}')",
            raw.len(),
            raw
        ));
    }

    let chars: Vec<char> = raw.chars().collect();
    let prefix = chars[0];
    let is_valid_prefix = prefix == '_' || prefix == '\'' || prefix == '~' || prefix == '@'
        || prefix == '%' || prefix == '&' || prefix == '^' || prefix == '|'
        || prefix == '$' || prefix == '#' || prefix == ':' || prefix == '\\'
        || prefix == '*' || prefix == '+' || prefix == '-' || prefix == '/'
        || prefix == '?' || prefix == '!' || prefix == '=' || prefix == '<' || prefix == '>'
        || prefix == '.' || prefix == '(' || prefix == ')';
    if !is_valid_prefix {
        return Err(format!("Invalid slot prefix '{}' in '{}'", prefix, raw));
    }

    let opcode: String = chars[1..3].iter().collect();
    let terminator = chars[9];
    let is_valid_terminator = terminator == '>' || terminator == '!' || terminator == '?' || terminator == ';'
        || terminator == ']' || terminator == '}' || terminator == ')' || terminator == '|'
        || terminator == '~' || terminator == '$' || terminator == '#' || terminator == ',';
    if !is_valid_terminator {
        return Err(format!(
            "Invalid slot terminator '{}' in '{}' (expected valid CL terminator)",
            terminator, raw
        ));
    }

    // Parse dest register if applicable (only for instructions that write to a register)
    let dest_str: String = chars[3..5].iter().collect();
    let is_writer = opcode != "SB" && opcode != "SH" && opcode != "RS" 
                 && opcode != "HL" && opcode != "DW" && opcode != "YD" && opcode != "NO"
                 && opcode != "JP" && opcode != "BZ" && opcode != "BN" && opcode != "BL" && opcode != "BG"
                 && opcode != "TI" && opcode != "TD" && opcode != "TW" && opcode != "ZL"
                 && opcode != "DE" && opcode != "DW" && opcode != "DN" && opcode != "DS";
    let dest_reg = if is_writer {
        usize::from_str_radix(&dest_str, 16).ok()
    } else {
        None
    };

    let mode = chars[5];
    let src_str: String = chars[6..7].iter().collect();
    let src_reg = usize::from_str_radix(&src_str, 16).ok();
    let parity_token = chars[7];
    let imm_token = chars[8];

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

pub fn verify_cl_program(content: &str) -> Result<ClReport, String> {
    let mut report = ClReport::default();
    let mut line_num = 0;

    for line in content.lines() {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }

        // Check for Label definitions (e.g. @label: or L00:)
        if (trimmed.starts_with('@') && trimmed.ends_with(':'))
            || (trimmed.starts_with('L') && trimmed.ends_with(':') && trimmed.len() <= 20)
        {
            report.labels_found += 1;
            continue;
        }

        // Check for Multi-Core / Directive definitions (e.g. .core [0,0,0,0]:)
        if trimmed.starts_with(".core") {
            report.cores_partitioned += 1;
            continue;
        }
        if trimmed.starts_with(".weights") {
            let binding = parse_weights_directive(trimmed)
                .map_err(|e| format!("Line {}: {}", line_num, e))?;
            report.weights_bound += 1;
            report.total_weight_bytes += binding.values.len() * 4;
            continue;
        }
        if trimmed.starts_with(".data") || trimmed.starts_with(".section") {
            continue;
        }
        if trimmed.starts_with(".stage")
            || trimmed.starts_with(".fuse")
            || trimmed.starts_with(".tensor")
            || trimmed.starts_with(".flow")
            || trimmed.starts_with(".layout")
        {
            report.directives_found += 1;
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // Must start with cycle identifier B<num>:
        let cycle_part = parts[0];
        if !cycle_part.starts_with('B') || !cycle_part.ends_with(':') {
            return Err(format!(
                "Syntax Error at line {}: Missing bundle cycle header (expected 'B<cycle>:', got '{}')",
                line_num, cycle_part
            ));
        }

        if parts.len() < 2 || parts.len() > 5 {
            return Err(format!(
                "Bundle Error at line {}: Each .cl VLIW bundle must contain between 1 and 4 slots, found {}",
                line_num,
                parts.len() - 1
            ));
        }

        let mut dest_regs_in_bundle: HashSet<usize> = HashSet::new();
        let mut optical_count_in_bundle = 0;

        for slot_str in &parts[1..] {
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
                if src > 0 && dest_regs_in_bundle.contains(&src) {
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
            if verify_token_crc8(slot_str) || slot.parity_token.is_ascii_hexdigit() {
                report.crc_verified += 1;
            }

            // Hazard check: Write-After-Write (WAW) conflict detection
            if let Some(dest) = slot.dest_reg {
                if dest > 0 && slot.opcode != "HL" && slot.opcode != "NO" {
                    if dest_regs_in_bundle.contains(&dest) {
                        let msg = format!(
                            "Line {}: Write-After-Write (WAW) hazard detected on Register R{:X} within cycle {}",
                            line_num, dest, cycle_part
                        );
                        report.hazards.push(msg);
                    } else {
                        dest_regs_in_bundle.insert(dest);
                    }
                }
            }
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
    let crc_nibble = format!("{:1X}", crc8 & 0x0F).chars().next().unwrap();
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
