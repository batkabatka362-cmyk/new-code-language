// ============================================================================
// CRON Low-Level Machine Language (.cl) AI Self-Healing Engine
// Designed specifically for AI Vibe-Coding Co-Pilots.
// Automatically repairs, validates, and canonicalizes LLM-generated .cl code:
//   1. Recalculates exact CRC-8 ATM polynomial checksums for corrupted/placeholder slots.
//   2. Pads bundles with < 4 slots using zero-power NOP tokens (_NO00#000>).
//   3. Resolves intra-bundle RAW and WAW data hazards via cycle splitting.
//   4. Canonicalizes bundle indexing and formatting for 100% hardware compliance.
// ============================================================================

use crate::cl_lang::{compute_crc8_atm, parse_slot, verify_token_crc8};
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct ClHealReport {
    pub total_bundles_processed: usize,
    pub slots_repaired_crc: usize,
    pub bundles_padded_nops: usize,
    pub waw_hazards_resolved: usize,
    pub raw_hazards_resolved: usize,
    pub canonical_code: String,
}

pub const CANONICAL_NOP: &str = "_NO00#000>";

/// Formats and recalculates CRC-8 ATM for a single slot
pub fn heal_slot_crc(raw: &str) -> (String, bool) {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return (CANONICAL_NOP.to_string(), true);
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let prefix = if chars[0] == '_' || chars[0] == '\'' || chars[0] == '~' || chars[0] == '@'
        || chars[0] == '%' || chars[0] == '&' || chars[0] == '^' || chars[0] == '|'
        || chars[0] == '$' || chars[0] == '#' || chars[0] == ':' || chars[0] == '\\'
        || chars[0] == '*' || chars[0] == '+' || chars[0] == '-' || chars[0] == '/'
        || chars[0] == '?' || chars[0] == '!' || chars[0] == '=' || chars[0] == '<' || chars[0] == '>'
        || chars[0] == '.' || chars[0] == '(' || chars[0] == ')'
    {
        chars[0]
    } else {
        '_'
    };
    let term = if *chars.last().unwrap() == '>' || *chars.last().unwrap() == '!' || *chars.last().unwrap() == '?' || *chars.last().unwrap() == ';'
        || *chars.last().unwrap() == ']' || *chars.last().unwrap() == '}' || *chars.last().unwrap() == ')' || *chars.last().unwrap() == '|'
        || *chars.last().unwrap() == '~' || *chars.last().unwrap() == '$' || *chars.last().unwrap() == '#' || *chars.last().unwrap() == ','
    {
        *chars.last().unwrap()
    } else {
        '>'
    };

    if trimmed.len() == 10 && verify_token_crc8(trimmed) {
        return (trimmed.to_string(), false);
    }

    let op = if chars.len() >= 3 {
        format!("{}{}", chars[1], chars[2])
    } else {
        "NO".to_string()
    };
    let dest_hex = if chars.len() >= 5 {
        format!("{}{}", chars[3], chars[4])
    } else {
        "00".to_string()
    };
    let mode = if chars.len() >= 6 { chars[5] } else { '#' };
    let src_char = if chars.len() >= 7 { chars[6] } else { '0' };
    let imm_char = if chars.len() >= 9 {
        chars[chars.len() - 2]
    } else {
        '0'
    };

    let payload = format!("{}{}{}{}{}{}", prefix, op, dest_hex, mode, src_char, imm_char);
    let crc8 = compute_crc8_atm(payload.as_bytes()) & 0x0F;
    let crc_char = format!("{:1X}", crc8).chars().next().unwrap();

    let healed = format!(
        "{}{}{}{}{}{}{}{}",
        prefix, op, dest_hex, mode, src_char, crc_char, imm_char, term
    );
    (healed, true)
}

pub fn heal_cl_program(source: &str) -> Result<ClHealReport, String> {
    let mut report = ClHealReport::default();
    let mut raw_bundles: Vec<Vec<String>> = Vec::new();

    // 1. Initial parse, line cleaning, and slot healing
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") || trimmed.starts_with('.') {
            continue;
        }

        let slots_raw = if let Some((_header, slots_part)) = trimmed.split_once(':') {
            slots_part
        } else {
            trimmed
        };

        let slots_code = if let Some((code, _)) = slots_raw.split_once("//") {
            code
        } else if let Some((code, _)) = slots_raw.split_once(';') {
            code
        } else {
            slots_raw
        };

        let slots: Vec<String> = slots_code.split_whitespace().map(|s| s.to_string()).collect();

        if slots.is_empty() {
            continue;
        }

        let mut healed_slots: Vec<String> = Vec::with_capacity(4);
        for slot in &slots {
            let (healed, was_repaired) = heal_slot_crc(slot);
            if was_repaired {
                report.slots_repaired_crc += 1;
            }
            healed_slots.push(healed);
        }

        // Pad with NOPs if fewer than 4 slots
        if healed_slots.len() < 4 {
            let missing = 4 - healed_slots.len();
            for _ in 0..missing {
                healed_slots.push(CANONICAL_NOP.to_string());
            }
            report.bundles_padded_nops += 1;
        }

        raw_bundles.push(healed_slots);
    }

    if raw_bundles.is_empty() {
        return Err("Cannot heal empty .cl source (no valid instructions found)".to_string());
    }

    // 2. Hazard Resolution via Cycle Splitting
    let mut resolved_bundles: Vec<Vec<String>> = Vec::new();

    for bundle in raw_bundles {
        let mut current_bundle: Vec<String> = Vec::with_capacity(4);
        let mut overflow_bundle: Vec<String> = Vec::with_capacity(4);
        let mut dest_regs: HashSet<usize> = HashSet::new();

        for slot_str in bundle {
            if slot_str == CANONICAL_NOP {
                current_bundle.push(slot_str);
                continue;
            }

            if let Ok(slot) = parse_slot(&slot_str) {
                let mut has_hazard = false;

                // WAW Hazard check
                if let Some(d) = slot.dest_reg {
                    if d > 0 && dest_regs.contains(&d) {
                        has_hazard = true;
                        report.waw_hazards_resolved += 1;
                    }
                }

                // RAW Hazard check
                if let Some(s) = slot.src_reg {
                    if s > 0 && dest_regs.contains(&s) {
                        has_hazard = true;
                        report.raw_hazards_resolved += 1;
                    }
                }

                if has_hazard {
                    overflow_bundle.push(slot_str);
                } else {
                    if let Some(d) = slot.dest_reg {
                        if d > 0 {
                            dest_regs.insert(d);
                        }
                    }
                    current_bundle.push(slot_str);
                }
            } else {
                current_bundle.push(slot_str);
            }
        }

        while current_bundle.len() < 4 {
            current_bundle.push(CANONICAL_NOP.to_string());
        }
        resolved_bundles.push(current_bundle);

        if !overflow_bundle.is_empty() {
            while overflow_bundle.len() < 4 {
                overflow_bundle.push(CANONICAL_NOP.to_string());
            }
            resolved_bundles.push(overflow_bundle);
        }
    }

    // 3. Emit canonical .cl representation with monotonically increasing B0000: indices
    let mut canonical = String::with_capacity(source.len() + 1024);
    canonical.push_str("; ============================================================\n");
    canonical.push_str("; CRON Canonical AI Vibe-Coded .cl Machine Bundle Stream\n");
    canonical.push_str("; Formatted & Self-Healed by cronc SSS+ Engine\n");
    canonical.push_str("; ============================================================\n\n");

    for (cycle_idx, bundle) in resolved_bundles.iter().enumerate() {
        let line = format!(
            "B{:04}: {} {} {} {}\n",
            cycle_idx, bundle[0], bundle[1], bundle[2], bundle[3]
        );
        canonical.push_str(&line);
    }

    report.total_bundles_processed = resolved_bundles.len();
    report.canonical_code = canonical;

    Ok(report)
}
