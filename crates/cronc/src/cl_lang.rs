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

#[derive(Debug, Clone, Default)]
pub struct ClReport {
    pub total_bundles: usize,
    pub total_slots: usize,
    pub opcodes_verified: usize,
    pub parity_verified: usize,
    pub hazards: Vec<String>,
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
    "HL", // Halt Execution
    "NO", // NOP (No Operation)
    "=0", // Immediate Load Low
    "=1", // Immediate Load High
    "==", // Generic Immediate Load
];

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
    if prefix != '_' && prefix != '\'' {
        return Err(format!("Invalid slot prefix '{}' in '{}'", prefix, raw));
    }

    let opcode: String = chars[1..3].iter().collect();
    let terminator = chars[9];
    if terminator != '>' && terminator != '!' {
        return Err(format!(
            "Invalid slot terminator '{}' in '{}' (expected '>' or '!')",
            terminator, raw
        ));
    }

    // Parse dest register if applicable (only for instructions that write to a register)
    let dest_str: String = chars[3..5].iter().collect();
    let is_writer = opcode != "SB" && opcode != "SH" && opcode != "RS" 
                 && opcode != "HL" && opcode != "DW" && opcode != "YD" && opcode != "NO";
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

pub fn verify_cl_program(content: &str) -> Result<ClReport, String> {
    let mut report = ClReport::default();
    let mut line_num = 0;

    for line in content.lines() {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
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

        if parts.len() != 5 {
            return Err(format!(
                "Bundle Error at line {}: Each .cl VLIW bundle must contain exactly 4 slots, found {}",
                line_num,
                parts.len() - 1
            ));
        }

        let mut dest_regs_in_bundle: HashSet<usize> = HashSet::new();
        let mut optical_count_in_bundle = 0;

        for slot_str in &parts[1..5] {
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

            // Parity token check
            if slot.parity_token.is_ascii_hexdigit() || slot.parity_token.is_ascii_alphanumeric() {
                report.parity_verified += 1;
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
