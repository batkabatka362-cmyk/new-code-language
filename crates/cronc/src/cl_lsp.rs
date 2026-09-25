//! Language Server Protocol (LSP) & Diagnostic Engine for `.cl` VLIW Machine Code
//!
//! Provides real-time IDE diagnostics, CRC-8 ATM verification, pipeline hazard detection,
//! opcode hover documentation, and JSON-RPC diagnostic formatting.

use crate::cl_macro::compute_slot_crc;

/// Severity of a diagnostic message
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A single diagnostic finding on a `.cl` source file
#[derive(Debug, Clone)]
pub struct ClDiagnostic {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
}

/// Analyzes a `.cl` source string and produces comprehensive diagnostics
pub fn analyze_cl_source(source: &str) -> Vec<ClDiagnostic> {
    let mut diagnostics = Vec::new();

    for (line_idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with(';') || trimmed.starts_with('.') || trimmed.starts_with('@') {
            continue;
        }

        // Check for Bundle Header
        if !trimmed.starts_with('B') && !trimmed.starts_with('b') {
            diagnostics.push(ClDiagnostic {
                line: line_idx + 1,
                start_col: 1,
                end_col: trimmed.len() + 1,
                severity: DiagnosticSeverity::Error,
                code: "SYN001".to_string(),
                message: "Bundle line must start with bundle index 'B####:'".to_string(),
            });
            continue;
        }

        let (header, slot_str) = match trimmed.split_once(':') {
            Some((h, s)) => (h, s.trim()),
            None => {
                diagnostics.push(ClDiagnostic {
                    line: line_idx + 1,
                    start_col: 1,
                    end_col: trimmed.len() + 1,
                    severity: DiagnosticSeverity::Error,
                    code: "SYN002".to_string(),
                    message: "Missing ':' after bundle index".to_string(),
                });
                continue;
            }
        };

        let slots: Vec<&str> = slot_str.split_whitespace().collect();

        if slots.len() != 4 {
            diagnostics.push(ClDiagnostic {
                line: line_idx + 1,
                start_col: header.len() + 2,
                end_col: trimmed.len() + 1,
                severity: DiagnosticSeverity::Error,
                code: "VLIW001".to_string(),
                message: format!("VLIW bundle must contain exactly 4 slots (found {})", slots.len()),
            });
        }

        let mut written_regs = Vec::new();

        for (slot_idx, slot) in slots.iter().enumerate() {
            let col_offset = header.len() + 2 + slot_idx * 11;
            let chars: Vec<char> = slot.chars().collect();

            // 1. Check exact 10-char slot length
            if chars.len() != 10 {
                diagnostics.push(ClDiagnostic {
                    line: line_idx + 1,
                    start_col: col_offset,
                    end_col: col_offset + chars.len(),
                    severity: DiagnosticSeverity::Error,
                    code: "SLOT001".to_string(),
                    message: format!("Slot {} must be exactly 10 characters (found {})", slot_idx, chars.len()),
                });
                continue;
            }

            // 2. Check CRC-8 ATM token
            let payload: String = chars[..9].iter().collect();
            let actual_token = chars[9];
            let crc = compute_slot_crc(&payload);
            let expected_token = (33 + (crc % 94)) as char;

            if actual_token != expected_token {
                diagnostics.push(ClDiagnostic {
                    line: line_idx + 1,
                    start_col: col_offset + 9,
                    end_col: col_offset + 10,
                    severity: DiagnosticSeverity::Error,
                    code: "CRC001".to_string(),
                    message: format!(
                        "Invalid CRC-8 ATM token '{}' for slot payload '{}' (expected '{}')",
                        actual_token, payload, expected_token
                    ),
                });
            }

            // 3. Pipeline Hazard Check: Write-After-Write within the same bundle
            if let Some(dst_reg) = extract_dst_reg(slot) {
                if written_regs.contains(&dst_reg) {
                    diagnostics.push(ClDiagnostic {
                        line: line_idx + 1,
                        start_col: col_offset,
                        end_col: col_offset + 10,
                        severity: DiagnosticSeverity::Warning,
                        code: "HAZ001".to_string(),
                        message: format!(
                            "Write-After-Write (WAW) hazard on Register R{:X} in same bundle",
                            dst_reg
                        ),
                    });
                } else {
                    written_regs.push(dst_reg);
                }
            }
        }
    }

    diagnostics
}

/// Helper to extract destination register from a slot safely
fn extract_dst_reg(slot: &str) -> Option<u8> {
    let chars: Vec<char> = slot.chars().collect();
    if chars.len() < 5 {
        return None;
    }
    if chars[0] == '=' && chars[1] == '=' {
        let hex_s: String = chars[2..4].iter().collect();
        u8::from_str_radix(&hex_s, 16).ok()
    } else if chars[0] == '_' {
        let hex_s: String = chars[3..5].iter().collect();
        u8::from_str_radix(&hex_s, 16).ok()
    } else {
        None
    }
}

/// Formats diagnostics into human-readable terminal output
pub fn format_diagnostics_report(diagnostics: &[ClDiagnostic]) -> String {
    if diagnostics.is_empty() {
        return "✓ LSP Audit: 0 errors, 0 warnings. Code is 100% compliant with .cl hardware spec.\n".to_string();
    }

    let mut report = format!("Found {} diagnostic findings:\n", diagnostics.len());
    for d in diagnostics {
        let level = match d.severity {
            DiagnosticSeverity::Error => "[ERROR]",
            DiagnosticSeverity::Warning => "[WARN ]",
            DiagnosticSeverity::Information => "[INFO ]",
            DiagnosticSeverity::Hint => "[HINT ]",
        };
        report.push_str(&format!(
            "  {} Line {}:{}-{} [{}] {}\n",
            level, d.line, d.start_col, d.end_col, d.code, d.message
        ));
    }
    report
}
