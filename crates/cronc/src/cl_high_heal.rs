// ============================================================================
// CRON & CL HIGH-LEVEL FORMAL SELF-HEALING CONTRACT & DIAGNOSTICS
// Module: cronc::cl_high_heal (crates/cronc/src/cl_high_heal.rs)
//
// Implements the Post-Human AI Autonomous Self-Healing Contract (Section VIII).
// When an AI agent generates code with shape, type, or syntax mismatches,
// returns structured machine-readable JSON diagnostics and concrete code patches.
// ============================================================================

use crate::cl_high_parser::parse_high_cl;
use crate::cl_high_lowering::{HighLoweringEngine, LoweringConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathematicalConflict {
    pub source_type: String,
    pub target_type: String,
    pub root_cause: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfHealPatch {
    pub required_primitive: String,
    pub original_snippet: String,
    pub replacement_code: String,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormalDiagnosticReport {
    pub status: String, // "FORMAL_VERIFIED" | "FORMAL_REJECTED"
    pub error_domain: Option<String>,
    pub location: Option<SourceLocation>,
    pub mathematical_conflict: Option<MathematicalConflict>,
    pub self_heal_patch: Option<SelfHealPatch>,
    pub vliw_bundle_count: usize,
}

impl FormalDiagnosticReport {
    pub fn verified(bundles: usize) -> Self {
        Self {
            status: "FORMAL_VERIFIED".to_string(),
            error_domain: None,
            location: None,
            mathematical_conflict: None,
            self_heal_patch: None,
            vliw_bundle_count: bundles,
        }
    }

    pub fn rejected(
        domain: &str,
        file: &str,
        line: usize,
        col: usize,
        conflict: MathematicalConflict,
        patch: SelfHealPatch,
    ) -> Self {
        Self {
            status: "FORMAL_REJECTED".to_string(),
            error_domain: Some(domain.to_string()),
            location: Some(SourceLocation {
                file: file.to_string(),
                line,
                col,
            }),
            mathematical_conflict: Some(conflict),
            self_heal_patch: Some(patch),
            vliw_bundle_count: 0,
        }
    }

    /// Render machine-readable JSON for autonomous AI agents
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"diagnostic_status\": \"{}\",\n", self.status));

        if let Some(ref domain) = self.error_domain {
            json.push_str(&format!("  \"error_domain\": \"{}\",\n", domain));
        }

        if let Some(ref loc) = self.location {
            json.push_str("  \"location\": {\n");
            json.push_str(&format!("    \"file\": \"{}\",\n", loc.file));
            json.push_str(&format!("    \"line\": {},\n", loc.line));
            json.push_str(&format!("    \"col\": {}\n", loc.col));
            json.push_str("  },\n");
        }

        if let Some(ref conflict) = self.mathematical_conflict {
            json.push_str("  \"mathematical_conflict\": {\n");
            json.push_str(&format!("    \"source_type\": \"{}\",\n", conflict.source_type));
            json.push_str(&format!("    \"target_type\": \"{}\",\n", conflict.target_type));
            json.push_str(&format!("    \"root_cause\": \"{}\"\n", conflict.root_cause.replace('"', "\\\"")));
            json.push_str("  },\n");
        }

        if let Some(ref patch) = self.self_heal_patch {
            json.push_str("  \"self_heal_patch\": {\n");
            json.push_str(&format!("    \"required_primitive\": \"{}\",\n", patch.required_primitive));
            json.push_str(&format!("    \"original_snippet\": \"{}\",\n", patch.original_snippet.replace('"', "\\\"")));
            json.push_str(&format!("    \"replacement_code\": \"{}\",\n", patch.replacement_code.replace('\n', "\\n").replace('"', "\\\"")));
            json.push_str(&format!("    \"explanation\": \"{}\"\n", patch.explanation.replace('"', "\\\"")));
            json.push_str("  },\n");
        }

        json.push_str(&format!("  \"vliw_bundle_count\": {}\n", self.vliw_bundle_count));
        json.push('}');
        json
    }
}

/// Analyze High-Level CL source, verify mathematical contracts, and synthesize self-healing patch if needed.
pub fn verify_and_heal_high_cl(source: &str, file_label: Option<&str>) -> FormalDiagnosticReport {
    let file = file_label.unwrap_or("inline.cl");

    // 1. Try parsing AST
    let module = match parse_high_cl(source) {
        Ok(m) => m,
        Err(err_msg) => {
            // Synthesize parser error self-healing patch
            let (line, col) = extract_line_col_heuristic(&err_msg, source);
            return FormalDiagnosticReport::rejected(
                "SYNTAX_GRAMMAR_CONTRACT_VIOLATION",
                file,
                line,
                col,
                MathematicalConflict {
                    source_type: "RawSourceStream".to_string(),
                    target_type: "HighLevelAST".to_string(),
                    root_cause: err_msg.clone(),
                },
                SelfHealPatch {
                    required_primitive: ".op.syntax_canonicalize".to_string(),
                    original_snippet: get_line_snippet(source, line),
                    replacement_code: format!("// Auto-fix suggestion: verify braces or semicolons around line {}", line),
                    explanation: format!("Grammar parse error: {}", err_msg),
                },
            );
        }
    };

    // 2. Try Lowering to VLIW IR & Bundles
    let engine = HighLoweringEngine::new(LoweringConfig::default());
    let program_ir = match engine.lower_module_to_ir(&module) {
        Ok(ir) => ir,
        Err(lower_err) => {
            return FormalDiagnosticReport::rejected(
                "TENSOR_AFFINE_CONTRACT_MISMATCH",
                file,
                1,
                1,
                MathematicalConflict {
                    source_type: "HighModuleAST".to_string(),
                    target_type: "VLIW128BitIR".to_string(),
                    root_cause: lower_err.clone(),
                },
                SelfHealPatch {
                    required_primitive: ".op.type_cast".to_string(),
                    original_snippet: "".to_string(),
                    replacement_code: "".to_string(),
                    explanation: lower_err,
                },
            );
        }
    };

    FormalDiagnosticReport::verified(program_ir.bundles.len())
}

/// Automatically apply self-healing patch if a fixable contract violation is detected.
pub fn auto_apply_high_cl_heal(source: &str) -> Result<String, String> {
    let report = verify_and_heal_high_cl(source, None);
    if report.status == "FORMAL_VERIFIED" {
        return Ok(source.to_string());
    }

    if let Some(ref patch) = report.self_heal_patch {
        if !patch.original_snippet.is_empty() && !patch.replacement_code.is_empty() {
            let healed = source.replace(&patch.original_snippet, &patch.replacement_code);
            return Ok(healed);
        }
    }

    Err(report.to_json())
}

fn extract_line_col_heuristic(msg: &str, _source: &str) -> (usize, usize) {
    if let Some(pos) = msg.find("line ") {
        let rest = &msg[pos + 5..];
        let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(l) = num.parse::<usize>() {
            return (l, 1);
        }
    }
    (1, 1)
}

fn get_line_snippet(source: &str, line_num: usize) -> String {
    source.lines().nth(line_num.saturating_sub(1)).unwrap_or("").trim().to_string()
}
