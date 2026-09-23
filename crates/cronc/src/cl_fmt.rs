// ============================================================================
// CRON Cognitive Low-Level (.cl) Source Formatter & Canonicalizer
// Module: cronc::cl_fmt
// Target: 256-Core 4D-Torus Neuromorphic Hardware
//
// Automatically formats, aligns, pads, and heals .cl machine code for:
//   - Canonical 10-character slot token width with exact CRC-8 ATM checksums
//   - Uniform VLIW 4-slot bundle alignment with optional zero-power NOP padding
//   - Semantic directive indentation & comment alignment
// ============================================================================

use crate::cl_heal::{heal_slot_crc, CANONICAL_NOP};

/// Formatting and canonicalization options for .cl source code
#[derive(Debug, Clone)]
pub struct ClFmtOptions {
    pub pad_nops: bool,
    pub heal_crc: bool,
    pub align_comments: bool,
    pub comment_column: usize,
}

impl Default for ClFmtOptions {
    fn default() -> Self {
        Self {
            pad_nops: true,
            heal_crc: true,
            align_comments: true,
            comment_column: 56,
        }
    }
}

/// Formats a full .cl machine program into canonical, beautiful VLIW layout
pub fn format_cl_program(source: &str, options: &ClFmtOptions) -> Result<String, String> {
    let mut out = String::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            out.push('\n');
            continue;
        }

        // Preserve full-line comments and section headers
        if trimmed.starts_with(';') || trimmed.starts_with("//") {
            out.push_str(trimmed);
            out.push('\n');
            continue;
        }

        // Labels: @label: or L00:
        if (trimmed.starts_with('@') && trimmed.ends_with(':'))
            || (trimmed.starts_with('L') && trimmed.ends_with(':') && trimmed.len() <= 20)
        {
            out.push_str(trimmed);
            out.push('\n');
            continue;
        }

        // Semantic directives: .stage, .clifford, .weights, .fuse, .flow, .layout, .core
        if trimmed.starts_with('.') {
            out.push_str(trimmed);
            out.push('\n');
            continue;
        }

        // Separate trailing inline comments if any
        let (instruction_part, comment_part) = if let Some(pos) = trimmed.find(';') {
            (trimmed[..pos].trim_end(), Some(&trimmed[pos..]))
        } else if let Some(pos) = trimmed.find("//") {
            (trimmed[..pos].trim_end(), Some(&trimmed[pos..]))
        } else {
            (trimmed, None)
        };

        // Parse VLIW bundle: B<cycle>: <slot0> ...
        if let Some((cycle_part, slots_part)) = instruction_part.split_once(':') {
            let cycle_num = cycle_part
                .trim()
                .trim_start_matches(|c: char| c == 'B' || c == 'b')
                .parse::<usize>()
                .map_err(|e| format!("Invalid cycle header '{}': {}", cycle_part, e))?;

            let raw_tokens: Vec<&str> = slots_part.split_whitespace().collect();
            let mut formatted_slots = Vec::new();

            for tok in &raw_tokens {
                if options.heal_crc {
                    let (healed, _) = heal_slot_crc(tok);
                    formatted_slots.push(healed);
                } else {
                    let mut s = tok.to_string();
                    while s.len() < 10 {
                        s.push('0');
                    }
                    if s.len() > 10 {
                        s.truncate(10);
                    }
                    formatted_slots.push(s);
                }
            }

            // Pad with zero-power NOP tokens if requested
            if options.pad_nops {
                let nop = if options.heal_crc {
                    heal_slot_crc(CANONICAL_NOP).0
                } else {
                    CANONICAL_NOP.to_string()
                };
                while formatted_slots.len() < 4 {
                    formatted_slots.push(nop.clone());
                }
            }

            let bundle_str = format!("B{:04}: {}", cycle_num, formatted_slots.join(" "));

            if let Some(comment) = comment_part {
                if options.align_comments {
                    let padding = if bundle_str.len() < options.comment_column {
                        " ".repeat(options.comment_column - bundle_str.len())
                    } else {
                        " ".to_string()
                    };
                    out.push_str(&format!("{}{}{}\n", bundle_str, padding, comment));
                } else {
                    out.push_str(&format!("{} {}\n", bundle_str, comment));
                }
            } else {
                out.push_str(&bundle_str);
                out.push('\n');
            }
        } else {
            // Unrecognized line format, preserve as-is
            out.push_str(trimmed);
            out.push('\n');
        }
    }

    Ok(out)
}
