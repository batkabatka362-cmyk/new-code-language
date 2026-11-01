// ============================================================================
// CRON Language Server Protocol (LSP 3.17) Engine
// Target: Antigravity IDE, VS Code, Neovim, and Language Clients
// Capabilities:
//   - .cr High-level blueprints: Diagnostics (E0001..E0012), Hovers, Completions
//   - .cl Cognitive Low-Level Machine Code:
//       * Real-time 4-Way VLIW Slot & Port Diagnostics (CL001..CL005)
//       * Silicon Micro-Architecture Hovers (Port, Cycles, Energy pJ, FLOPs)
//       * Slot-Aware Smart Completions (ALU0, ALU1, MEM, NOC, R0..R15)
//       * Autonomous Vibe-Loop QuickFix Code Actions
//       * Document Auto-Formatting to 94-char Canonical Slots
// ============================================================================

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use serde::{Deserialize, Serialize};

pub mod dap;
pub use dap::{run_dap_server, DapServer};

// === JSON-RPC 2.0 Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// === LSP 3.17 Protocol Structures ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: u32, // 1 = Error, 2 = Warning, 3 = Info, 4 = Hint
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent {
    pub kind: String, // "markdown" or "plaintext"
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: MarkupContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    pub label: String,
    pub kind: u32, // 1=Text, 3=Function, 7=Class, 14=Keyword, 21=Constant, etc.
    pub detail: Option<String>,
    pub documentation: Option<MarkupContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<Diagnostic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<WorkspaceEdit>,
}

// === LSP 3.17 Extended Structures ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<MarkupContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInformation {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<MarkupContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_signature: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_parameter: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    pub name: String,
    pub kind: u32,  // 5=Class, 6=Method, 12=Function, 23=Struct
    pub range: Range,
    pub selection_range: Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocumentSymbol>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHint {
    pub position: Position,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<u32>, // 1 = Type, 2 = Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_left: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RawSemanticToken {
    pub line: u32,
    pub col: u32,
    pub len: u32,
    pub token_type: u32,
    pub token_modifiers: u32,
}

pub fn classify_cl_token(w: &str, line: u32, col: u32, out: &mut Vec<RawSemanticToken>) {
    if (w.starts_with('R') || w.starts_with('r') || w.starts_with('V') || w.starts_with('v') || w.starts_with('A') || w.starts_with('a'))
        && w.len() >= 2
        && w[1..].chars().all(|c| c.is_ascii_digit())
    {
        out.push(RawSemanticToken {
            line,
            col,
            len: w.len() as u32,
            token_type: 3, // variable
            token_modifiers: 2, // readonly
        });
        return;
    }

    match w {
        "ADD" | "SUB" | "MUL" | "DIV" | "FMA" | "FMS" | "ROPE" | "SWIZZLE" | "ROUTE_DOR"
        | "SEND" | "RECV" | "BROADCAST" | "COLLECTIVE" | "LOAD" | "STORE" | "PREFETCH"
        | "NOP" | "RMSNORM" | "SWIGLU" | "RELU" | "GELU" | "MAX" | "MIN" | "CMP"
        | "AND" | "OR" | "XOR" | "SHL" | "SHR" | "BARRIER" | "SYNC" | "JMP" | "BEQ" => {
            out.push(RawSemanticToken {
                line,
                col,
                len: w.len() as u32,
                token_type: 9, // macro
                token_modifiers: 8, // defaultLibrary
            });
            return;
        }
        _ => {}
    }

    if (w.starts_with('#') && w.len() > 1 && w[1..].chars().all(|c| c.is_ascii_digit()))
        || w.chars().all(|c| c.is_ascii_digit())
        || (w.starts_with("0x") && w.len() > 2)
    {
        out.push(RawSemanticToken {
            line,
            col,
            len: w.len() as u32,
            token_type: 12, // number
            token_modifiers: 0,
        });
    }
}

pub fn is_builtin_cognitive_fn(name: &str) -> bool {
    matches!(
        name,
        "pack_wave"
            | "compute_attention_head"
            | "step_synaptic_plasticity"
            | "ground_and_unify"
            | "init_kg_partition"
            | "assert_triple"
            | "batch_norm_quantize"
            | "dense_relu_step"
            | "spatial_broadcast"
            | "consume"
            | "simd_splat"
            | "simd_fma"
            | "simd_dot"
            | "simd_reduce_sum"
            | "spawn_at"
            | "torus_send"
            | "torus_recv"
            | "torus_broadcast"
            | "optical_gemm"
            | "optical_gemm_forward"
            | "predicated_op"
            | "photonic_self_attention"
            | "ssm_selective_scan_step"
            | "mamba2_transformer_hybrid_forward"
            | "step_cochlea_channel"
            | "init_cochlea_channel"
            | "process_audio_filterbank"
            | "accumulate_patch_dvs_event"
            | "init_vision_patch_tile"
            | "patch_to_core_id"
            | "project_patch_token_wave"
            | "fredkin_gate"
            | "toffoli_gate"
            | "reversible_entangle_states"
            | "step_forward_reversible"
            | "step_backward_time_inversion"
            | "evaluate_thermal_safety"
            | "sphere_geodesic"
            | "sphere_rk4"
            | "sphere_flow"
            | "dpsta_transition"
            | "dpsta_pack"
            | "dpsta_unpack"
            | "quarot_hadamard"
            | "sage_attention"
            | "rmsnorm"
            | "swiglu"
            | "bitnet_popcount"
            | "print"
            | "println"
            | "assert"
            | "panic"
    )
}

pub fn is_builtin_type_name(name: &str) -> bool {
    matches!(
        name,
        "wave_t"
            | "f16"
            | "f32"
            | "f64"
            | "bf16"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "tensor"
            | "bool"
            | "trit"
            | "trit_t"
            | "complex_t"
            | "void"
            | "str"
            | "string"
            | "char"
            | "ext_addr_t"
            | "spk_stamp"
            | "rev_t"
            | "TorusCoord"
            | "Option"
            | "Result"
            | "Tensor"
            | "DynamicVec"
    )
}

// === Language Identification ===

pub fn is_cl_document(uri: &str, text: &str) -> bool {
    if uri.ends_with(".cl") || uri.ends_with(".clm") || uri.ends_with(".clpack") || uri.ends_with(".cksl") {
        return true;
    }
    text.lines().any(|l| {
        let t = l.trim();
        // Bundle header pattern: B#### or b#### (letter + 2 digits minimum)
        let is_bundle_header = t.len() >= 3
            && (t.starts_with('B') || t.starts_with('b'))
            && t.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
            && t.chars().nth(2).is_some_and(|c| c.is_ascii_digit());
        t.starts_with("@CORE")
            || t.starts_with(".core")
            || t.starts_with("@kernel")
            || t.starts_with('§')
            || t.contains('↦')
            || t.contains('⟳')
            || t.contains("!Δ")
            || is_bundle_header
            || (t.contains('|') && (t.contains('_') || t.contains("ALU") || t.contains("NOP") || t.contains("ADD")))
    })
}

// === Machine Code (.cl) Real-Time Diagnostics ===

pub fn check_cl_diagnostics(source: &str) -> Vec<Diagnostic> {
    // CRON Mathematical Calculus (.cl) support
    if cronc::is_math_calculus_source(source) {
        let mut diagnostics = Vec::new();
        match cronc::parse_math_code(source) {
            Ok(prog) => {
                if let Err(shape_errors) = prog.verify_shapes() {
                    for err in shape_errors {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 20 },
                            },
                            severity: 1, // Error
                            code: Some("CRON_MATH_SHAPE".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!("Shape Mismatch: {}", err),
                        });
                    }
                }
                if let Err(sym_errors) = prog.verify_symbolic_shapes() {
                    for err in sym_errors {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 20 },
                            },
                            severity: 1, // Error
                            code: Some("CRON_SYMBOLIC_SHAPE".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!("Symbolic Dimension Mismatch: {}", err),
                        });
                    }
                }
            }
            Err(parse_err) => {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                    severity: 1, // Error
                    code: Some("CRON_MATH_SYNTAX".to_string()),
                    source: Some("cron-lsp".to_string()),
                    message: format!("Mathematical Calculus Syntax Error: {}", parse_err),
                });
            }
        }
        return diagnostics;
    }
    // Sovereign CL v2.0 Support: Pure Mathematical Morphisms and Silicon Control Flow
    if source.contains('§') || source.contains('↦') || source.contains('⟳') || source.contains("!Δ") {
        let mut diagnostics = Vec::new();
        match cronc::parse_cl_v2(source) {
            Ok(doc) => {
                let mut engine = cronc::V2LoweringEngine::new();
                match engine.lower_document(&doc) {
                    Ok(bundles) => {
                        let mut doc_vliw = doc.clone();
                        doc_vliw.vliw_stream = bundles;
                        let report = engine.co_verify(&doc_vliw);
                        if !report.passed {
                            for diag_msg in report.diagnostics {
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position { line: 0, character: 0 },
                                        end: Position { line: 0, character: 10 },
                                    },
                                    severity: 2, // Warning
                                    code: Some("CL_V2_COSIM".to_string()),
                                    source: Some("cron-lsp".to_string()),
                                    message: format!("Formal Co-Verification: {}", diag_msg),
                                });
                            }
                        }
                    }
                    Err(lower_err) => {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 10 },
                            },
                            severity: 1, // Error
                            code: Some("CL_V2_LOWER".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!("VLIW Lowering Error: {}", lower_err),
                        });
                    }
                }
            }
            Err(parse_err) => {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                    severity: 1, // Error
                    code: Some("CL_V2_SYNTAX".to_string()),
                    source: Some("cron-lsp".to_string()),
                    message: format!("Sovereign CL v2 syntax error: {}", parse_err),
                });
            }
        }
        return diagnostics;
    }

    let mut diagnostics = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let has_extended_registers = source.contains(".clifford") || source.contains("4d_torus") || source.contains("Rotor");

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with(';') || trimmed.starts_with('#') {
            continue;
        }

        // Strip inline comments if preceded by whitespace or standard comment marker
        let code_line = if let Some(pos) = trimmed.find(" //") {
            trimmed[..pos].trim()
        } else if let Some(pos) = trimmed.find(" ;") {
            trimmed[..pos].trim()
        } else {
            trimmed
        };
        if code_line.is_empty() {
            continue;
        }

        let line_u32 = line_idx as u32;

        // 1. Check @CORE directive syntax
        if code_line.starts_with("@CORE") {
            if !code_line.starts_with("@CORE(") || !code_line.ends_with(')') {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_u32, character: 0 },
                        end: Position { line: line_u32, character: line.len() as u32 },
                    },
                    severity: 1, // Error
                    code: Some("CL001".to_string()),
                    source: Some("cron-lsp".to_string()),
                    message: "Malformed @CORE directive. Expected syntax: @CORE(x,y,z,w)".to_string(),
                });
            }
            continue;
        }

        // 2. Directives starting with '@' or '.' (e.g. @kernel, .target, .core, .stage, .clifford, .ipc_target)
        if code_line.starts_with('@') || code_line.starts_with('.') {
            continue;
        }

        // 3. Closing brace of topological bundles
        if code_line == "}" {
            continue;
        }

        // 4. Labels (e.g. "LOOP:", "INIT:", "MAIN:", "DONE:")
        if code_line.ends_with(':') && !code_line.contains(' ') {
            continue;
        }

        // 5. Raw 40-character continuous Base-94 microcode bundle (e.g. cortex_4snake_output.cl, sovereign_mind_vliw.cl)
        if code_line.len() == 40 && !code_line.contains(' ') {
            continue;
        }

        // 6. Space-separated 10-char Base-94 microcode slots without prefix
        let space_tokens: Vec<&str> = code_line.split_whitespace().collect();
        if space_tokens.len() >= 3 && space_tokens.iter().all(|t| t.len() == 10) {
            continue;
        }

        // 7. Check if line starts with bundle header B####: / b####: / L####: / C####:
        let first_word = code_line.split_whitespace().next().unwrap_or("");
        let is_bundle_line = if first_word.ends_with(':') && first_word.len() >= 3 {
            let p = first_word.chars().next().unwrap();
            (p == 'B' || p == 'b' || p == 'L' || p == 'l' || p == 'C' || p == 'c')
                && first_word[1..first_word.len() - 1].chars().all(|c| c.is_ascii_hexdigit())
        } else {
            false
        };

        if is_bundle_line {
            if let Some((_hdr, rest)) = code_line.split_once(':') {
                let rest_trim = rest.trim();
                // Check if it's an explicit pipe-delimited VLIW line (e.g. "LOAD | NOP | NOP | NOP")
                if rest_trim.contains(" | ") {
                    let parts: Vec<&str> = rest_trim.split('|').collect();
                    if parts.len() != 4 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_u32, character: 0 },
                                end: Position { line: line_u32, character: line.len() as u32 },
                            },
                            severity: 1,
                            code: Some("CL001".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!(
                                "Invalid VLIW bundle slot count (found {} slots, expected exactly 4: ALU0 | ALU1 | MEM | NOC)",
                                parts.len()
                            ),
                        });
                    } else {
                        validate_vliw_pipe_slots(&parts, line_u32, line.len() as u32, has_extended_registers, &mut diagnostics);
                    }
                    continue;
                }

                // Otherwise it's space-separated Base-94 microcode slots
                let slots: Vec<&str> = rest_trim.split_whitespace().collect();
                for (s_idx, s) in slots.iter().enumerate() {
                    if s.starts_with('_') && s.ends_with('>') && s.len() != 10 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_u32, character: 0 },
                                end: Position { line: line_u32, character: line.len() as u32 },
                            },
                            severity: 2, // Warning
                            code: Some("CL002".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!(
                                "Raw slot #{} '{}' length is {} chars. Canonical fixed slot width is 10 chars (_XXYY#ZZZ>)",
                                s_idx + 1, s, s.len()
                            ),
                        });
                    }
                }
            }
            continue;
        }

        // 8. Pipe-delimited VLIW without bundle header (e.g. "ADD R1, R2 | NOP | LOAD [R3] | SEND (1,0,0,0)")
        if code_line.contains(" | ") {
            let parts: Vec<&str> = code_line.split('|').collect();
            if parts.len() != 4 {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_u32, character: 0 },
                        end: Position { line: line_u32, character: line.len() as u32 },
                    },
                    severity: 1,
                    code: Some("CL001".to_string()),
                    source: Some("cron-lsp".to_string()),
                    message: format!(
                        "Invalid VLIW bundle slot count (found {} slots, expected exactly 4: ALU0 | ALU1 | MEM | NOC)",
                        parts.len()
                    ),
                });
            } else {
                validate_vliw_pipe_slots(&parts, line_u32, line.len() as u32, has_extended_registers, &mut diagnostics);
            }
            continue;
        }

        // 9. Valid standalone microcode slot prefixes:
        let first_char = code_line.chars().next().unwrap();
        let is_slot_prefix = matches!(first_char, '_' | '\'' | '~' | '!' | '`' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '^' | '$' | '#' | '?' | '<' | '>' | '(' | ')');
        if is_slot_prefix {
            continue;
        }

        // 10. Topological bundle & keyword statements
        let is_topo = code_line.starts_with("bundle ")
            || code_line.starts_with("flow ")
            || code_line.starts_with("step ")
            || code_line.starts_with("transfer")
            || code_line.starts_with("attend")
            || code_line.starts_with("resonate")
            || code_line.starts_with("spin")
            || code_line.starts_with("vol")
            || code_line.starts_with("WRAP")
            || code_line.starts_with("TUNNEL")
            || code_line.starts_with("FLUID")
            || code_line.starts_with("ANCHOR")
            || code_line.starts_with("let ")
            || code_line.starts_with("barrier")
            || code_line.starts_with("return")
            || code_line.starts_with("S0:") || code_line.starts_with("S1:") || code_line.starts_with("S2:") || code_line.starts_with("S3:")
            || code_line.starts_with("s0:") || code_line.starts_with("s1:") || code_line.starts_with("s2:") || code_line.starts_with("s3:");
        if is_topo {
            continue;
        }

        // 11. Recognized assembly mnemonics (single instruction or macro line)
        let first_ident = code_line.split(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or("").to_ascii_uppercase();
        let is_known_mnemonic = matches!(
            first_ident.as_str(),
            "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "AND" | "OR" | "XOR" | "NOT" | "SHL" | "SHR"
            | "MOV" | "LOAD" | "STORE" | "NOP" | "HALT" | "RET" | "JMP" | "BRA" | "CALL" | "SWIZZLE"
            | "SEND" | "RECV" | "ROUTE_DOR" | "BROADCAST" | "COLLECTIVE"
            | "ATTN_STEP" | "LIF_STEP" | "STDP_STEP" | "DOT_I2" | "DOT_I4" | "SWIGLU_STEP"
            | "ROPE_STEP" | "RMSNORM_STEP" | "ROT_SANDWICH" | "CLIFFORD_ADD" | "TROPIC_MAX"
            | "LAYER_SHIFT" | "STDP_LEARN" | "WAVE_SPREAD" | "EMIT_CHAR"
            | "PACK_TRITS" | "OP_GEMM" | "TERNARY_MAC"
        );
        if is_known_mnemonic {
            continue;
        }

        // 12. Truly unrecognized statement -> Error CL001
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position { line: line_u32, character: 0 },
                end: Position { line: line_u32, character: line.len() as u32 },
            },
            severity: 1,
            code: Some("CL001".to_string()),
            source: Some("cron-lsp".to_string()),
            message: format!("Unrecognized statement in .cl microcode: '{}'. Expected 4-slot VLIW bundle", code_line),
        });
    }

    // If source contains topological bundles, perform full semantic invariant check
    if source.contains("bundle ") {
        if let Err(err_msg) = cronc::cl_topological::ClTopologicalCompiler::compile_source(source) {
            let err_line = lines.iter().position(|l| l.contains("bundle ")).unwrap_or(0) as u32;
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: err_line, character: 0 },
                    end: Position { line: err_line, character: lines.get(err_line as usize).map(|l| l.len()).unwrap_or(10) as u32 },
                },
                severity: 1,
                code: Some("TOPO_INV".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!("Topological Invariant Error: {}", err_msg),
            });
        }
    }

    diagnostics
}

/// Helper function to validate port compatibility and registers in a 4-slot pipe VLIW bundle
fn validate_vliw_pipe_slots(
    parts: &[&str],
    line_u32: u32,
    line_len: u32,
    has_extended_registers: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let slot_names = ["ALU0", "ALU1", "MEM", "NOC"];
    let max_reg = if has_extended_registers { 63 } else { 15 };

    for (slot_idx, part) in parts.iter().enumerate() {
        let p_raw = if let Some((_hdr, content)) = part.split_once(':') {
            content.trim()
        } else {
            part.trim()
        };
        let p_trim = p_raw;
        if p_trim.is_empty() {
            continue;
        }

        // Check for invalid registers
        for token in p_trim.split(|c: char| !c.is_alphanumeric()) {
            if token.starts_with('R') || token.starts_with('r') {
                if let Ok(reg_num) = token[1..].parse::<u32>() {
                    if reg_num > max_reg {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_u32, character: 0 },
                                end: Position { line: line_u32, character: line_len },
                            },
                            severity: 1,
                            code: Some("CL004".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!(
                                "Register 'R{}' in {} slot exceeds hardware limit (valid: R0..R{})",
                                reg_num, slot_names[slot_idx], max_reg
                            ),
                        });
                    }
                }
            }
        }

        // Check opcode port compatibility
        let upper = p_trim.to_ascii_uppercase();
        let op = upper.split_whitespace().next().unwrap_or("");
        if (slot_idx == 0 || slot_idx == 1) && (op == "LOAD" || op == "STORE" || op == "SWIZZLE") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line_u32, character: 0 },
                    end: Position { line: line_u32, character: line_len },
                },
                severity: 1,
                code: Some("CL003".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!(
                    "Memory opcode '{}' cannot execute on {}. Must be placed in MEM slot (slot #3)",
                    op, slot_names[slot_idx]
                ),
            });
        }
        if (slot_idx < 3) && (op == "ROUTE_DOR" || op == "SEND" || op == "RECV" || op == "BROADCAST" || op == "COLLECTIVE") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line_u32, character: 0 },
                    end: Position { line: line_u32, character: line_len },
                },
                severity: 1,
                code: Some("CL003".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!(
                    "NoC opcode '{}' cannot execute on {}. Must be placed in NOC slot (slot #4)",
                    op, slot_names[slot_idx]
                ),
            });
        }
        if (slot_idx > 0) && (op == "ATTN_STEP" || op == "LIF_STEP" || op == "STDP_STEP" || op == "DOT_I2" || op == "DOT_I4" || op == "SWIGLU_STEP" || op == "ROPE_STEP" || op == "RMSNORM_STEP") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line_u32, character: 0 },
                    end: Position { line: line_u32, character: line_len },
                },
                severity: 1,
                code: Some("CL003".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!(
                    "Advanced neural/photonic opcode '{}' requires primary ALU0 execution lane",
                    op
                ),
            });
        }
        if (slot_idx >= 2) && (op == "ROT_SANDWICH" || op == "CLIFFORD_ADD" || op == "TROPIC_MAX" || op == "LAYER_SHIFT") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line_u32, character: 0 },
                    end: Position { line: line_u32, character: line_len },
                },
                severity: 1,
                code: Some("CL003".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!(
                    "Clifford arithmetic opcode '{}' cannot execute on {}. Must be placed in ALU0 or ALU1 slot",
                    op, slot_names[slot_idx]
                ),
            });
        }
    }
}

// === Silicon Micro-Architecture Hover Tooltips (.cl & .cr) ===


/// Computes rich LSP Markdown Hover documentation for CRON Mathematical Calculus variables,
/// functions, tensor shapes, memory footprints, and built-in operators.
pub fn get_math_hover_documentation(doc: &str, word: &str) -> Option<String> {
    // 1. Built-in Mathematical Operators and Functions (Zero-Verbal & Unicode Parity)
    match word {
        "Ψ" | "attention" | "attn" | "flash_attn" => return Some(
            "### CRON Operator: `Ψ(Q, K, V, [scale])` (FlashAttention)\n\n            - **Operation**: Causal Masked Scaled Dot-Product Attention $\\rho(Q K^T / \\sqrt{d}) V$\n            - **Zero-Verbal Constitution**: Native single-character attention morphism\n            - **Silicon Implementation**: Fused online softmax register tile (0 intermediate DRAM writes)".to_string()
        ),
                "Ψ_c" | "kv_cache_attn" => return Some(
            "### CRON Operator: `Ψ_c(q, k, v, K_cache, V_cache, [scale])` (Streaming KV-Attention)\n\n            - **Signature**: `(out_t, K_next, V_next) = Ψ_c(...)`\n            - **Streaming Attention**: Appends step token into history ring-buffer cache and evaluates attention\n            - **Complexity**: $O(T \\cdot D)$ per autoregressive step instead of $O(T^2)$\n            - **Silicon Target**: Direct SRAM-tiled KV cache bypass".to_string()
        ),
        "Ψ_g" | "gqa_attn" => return Some(
            "### CRON Operator: `Ψ_g(Q, K, V, n_heads, n_kv_heads, [scale])` (Grouped-Query Attention)\n\n            - **Architecture**: LLaMA-3 / Qwen-2.5 multi-query head group projection\n            - **Silicon Implementation**: Zero-overhead broadcast head indexing".to_string()
        ),
        "Φ" | "rope" => return Some(
            "### CRON Operator: `Φ(X, [base_pos])` (Rotary Position Embedding)\n\n            - **Operation**: 2D complex plane phase rotation $\\Phi(X)_i = X_i e^{j \\theta_i}$\n            - **Silicon Target**: Direct SIMD complex multiply-add lane".to_string()
        ),
        "ρ" | "softmax" => return Some(
            "### CRON Operator: `ρ(Z)` (Row-Wise Softmax)\n\n            - **Operation**: Numerically stable row-wise probability distribution $\\rho(Z)_i = \\frac{e^{Z_i - \\max(Z)}}{\\sum e^{Z_j - \\max(Z)}}$\n            - **Silicon Target**: Online 3-pass fused register reduction".to_string()
        ),
        "Λ" | "cross_entropy" => return Some(
            "### CRON Functional: `Λ(Z, y)` (Cross-Entropy Loss)\n\n            - **Operation**: $\\Lambda(Z, y) = -\\frac{1}{T} \\sum_t \\log \\rho(Z)_{t, y_t}$\n            - **Autodiff Adjoint**: Exact fused analytical pullback $\\nabla_Z \\Lambda = \\frac{1}{T}(\\rho(Z) - \\mathbf{1}_y)$".to_string()
        ),
        "∇" | "grad" | "diff" => return Some(
            "### CRON Calculus: `∇(L, W)` (Analytical Matrix Derivative)\n\n            - **Operation**: Reverse-mode analytical automatic differentiation $\\nabla_W \\mathcal{L}$\n            - **Silicon Target**: Ahead-of-Time static adjoint tape (0 finite-difference overhead)".to_string()
        ),
        "ι" | "encode" => return Some(
            "### CRON Tokenizer: `ι(text)`\n\n            - **Operation**: Encodes UTF-8 string literal into 1D integer token vector\n            - **Zero-Verbal Constitution**: In-silicon native subword/byte tokenizer".to_string()
        ),
        "δ" | "decode" => return Some(
            "### CRON Tokenizer: `δ(tokens)`\n\n            - **Operation**: Decodes 1D integer token vector back into UTF-8 text string".to_string()
        ),
        "χ" | "sample" => return Some(
            "### CRON Operator: `χ(logits, temp, top_p, seed)` (Stochastic Sampler)\n\n            - **Operation**: Zero-verbal in-silicon autoregressive token sampling with temperature and nucleus gating\n            - **Determinism**: 100% reproducible across CPU, C23, and GPU PTX".to_string()
        ),
        "κ_k" | "top_k" => return Some(
            "### CRON Operator: `κ_k(logits, k)` (Top-K Filter)\n\n            - **Operation**: In-register partial bitonic sorting keeping only top-$k$ logits".to_string()
        ),
        "κ_p" | "top_p" => return Some(
            "### CRON Operator: `κ_p(logits, p)` (Nucleus Threshold)\n\n            - **Operation**: Cumulative sum probability threshold filter over $(0, 1]$".to_string()
        ),
        "κ" | "clip_grad" => return Some(
            "### CRON Operator: `κ(G, max_norm)` (Gradient Norm Clipping)\n\n            - **Operation**: $G \\cdot \\min(1.0, \\frac{\\text{max_norm}}{\\|G\\|_F})$ preventing exploding gradients".to_string()
        ),
        "O" | "zeros" => return Some(
            "### CRON Constructor: `O(M, N)` (Zero Tensor)\n\n            - **Operation**: Zero-initializes an $M \\times N$ matrix $\\mathbf{O}$\n            - **Silicon Target**: 64-byte aligned SIMD zero-memset".to_string()
        ),
        "I" | "eye" => return Some(
            "### CRON Constructor: `I(N)` (Identity Matrix)\n\n            - **Operation**: $N \\times N$ identity matrix $\\mathbf{I}_N$ with diagonal 1.0f".to_string()
        ),
        "τ" | "τ_2" | "bitlinear" | "quantize_t2" => return Some(
            "### CRON Operator: `τ(W)` (BitNet b1.58 Quantizer)\n\n            - **Operation**: Ternary $\\{-1, 0, +1\\}$ quantization with scale $\\gamma = \\text{mean}(|W|)$\n            - **Silicon Target**: Zero-Multiply integer addition/subtraction engine".to_string()
        ),
        "τ_8" | "quantize_i8" => return Some(
            "### CRON Operator: `τ_8(W)` (INT8 Quantizer)\n\n            - **Operation**: Row-wise symmetric INT8 quantization for $4\\times$ memory bandwidth compression".to_string()
        ),
        "Π_q" | "qmatmul" => return Some(
            "### CRON Operator: `Π_q(X, W_q, scales)` (Quantized MatMul)\n\n            - **Operation**: INT8 integer matrix multiplication with row scale vector".to_string()
        ),
        "∑" | "sum" => return Some(
            "### CRON Operator: `∑(X)` (Sum Reduction)\n\n            - **Operation**: Full parallel reduction sum $\\sum X_i$".to_string()
        ),
        "∑_k" | "sum_k" | "kahan_sum" | "neumaier_sum" => return Some(
            "### CRON Operator: `∑_k(X)` (Compensated Neumaier Summation)\n\n            - **Operation**: High-precision summation maintaining running low-order error compensation $c$\n            - **Precision**: 100% mathematically exact across millions of accumulating floats (Zero-Loss)\n            - **Numerical Stability**: Eliminates catastrophic floating-point cancellation in deep neural layers".to_string()
        ),
        "μ" | "mean" => return Some(
            "### CRON Operator: `μ(X)` (Mean Reduction)\n\n            - **Operation**: Arithmetic mean $\\mu(X) = \\frac{1}{N}\\sum X_i$".to_string()
        ),
        "σ" | "silu" | "sigmoid" | "swish" => return Some(
            "### CRON Operator: `σ(X)` (Activation)\n\n            - **Operation**: Sigmoid $\\frac{1}{1 + e^{-X}}$ or SiLU $X \\cdot \\sigma(X)$".to_string()
        ),
        "√" | "sqrt" => return Some(
            "### CRON Operator: `√(X)` (Square Root)\n\n            - **Operation**: Vectorized SIMD square root $\\sqrt{X}$".to_string()
        ),
        "∥" | "norm" => return Some(
            "### CRON Operator: `∥X∥` (Frobenius Norm)\n\n            - **Operation**: L2 Frobenius norm $\\|X\\|_2 = \\sqrt{\\sum X_i^2}$".to_string()
        ),
        "⊗" | "outer" => return Some(
            "### CRON Operator: `u ⊗ v` (Tensor Outer Product)\n\n            - **Operation**: Outer product between two 1D vectors $M_{i, j} = u_i \\cdot v_j$\n            - **Output**: 2D Matrix $[|u|, |v|]$".to_string()
        ),
        "N" | "rmsnorm" => return Some(
            "### CRON Module: `N(X, γ)` (RMSNorm)\n\n            - **Operation**: $N(X, \\gamma) = \\frac{X}{\\sqrt{\\mu(X^2) + \\epsilon}} \\cdot \\gamma$\n            - **Standard Library**: `<< \"libcl/math/rms_norm.cl\"`".to_string()
        ),
        "S" | "swiglu" | "fused_swiglu" => return Some(
            "### CRON Module: `S(X, W_1, W_2, W_3)` (SwiGLU MLP)\n\n            - **Operation**: $S(X) = (\\sigma(X W_1) \\odot (X W_2)) W_3$\n            - **Standard Library**: `<< \"libcl/math/swiglu_mlp.cl\"`".to_string()
        ),
        "B" | "bitnet" => return Some(
            "### CRON Module: `B(X, W, γ)` (BitNet b1.58 Layer)\n\n            - **Operation**: $B(X) = (X @ \\tau(W)^T) \\cdot \\gamma$\n            - **Standard Library**: `<< \"libcl/math/bitnet_layer.cl\"`".to_string()
        ),
        "M" | "Ξ" | "ssd_scan" => return Some(
            "### CRON Module: `M(X, A, B, C, Δ)` (Mamba-2 SSD Block)\n\n            - **Operation**: Associative linear recurrent state space scan\n            - **Standard Library**: `<< \"libcl/math/mamba2_ssd.cl\"`".to_string()
        ),
        "ω" | "argmax" => return Some(
            "### CRON Operator: `ω(v)` (Argmax)\n\n            - **Operation**: Returns the integer index of the maximum element in vector $v$".to_string()
        ),
        "ξ" | "concat" => return Some(
            "### CRON Operator: `ξ(a, b)` (Concat)\n\n            - **Operation**: Concatenates vectors or matrices along leading dimension".to_string()
        ),
        _ => {}
    }

    // 2. Program-level inspection (Constants, Inferred Shapes, Functions)
    if let Ok(prog) = cronc::parse_math_code(doc) {
        // Check functions
        for item in &prog.items {
            if let cronc::MathItem::Function { name, params, ret_type: _, body } = item {
                if name == word {
                    let param_strs: Vec<String> = params.iter().map(|p| {
                        let dim_str = if !p.symbolic_dims.is_empty() {
                            format!("[{}]", p.symbolic_dims.iter().map(|d| match d {
                                cronc::cl_math::MathDim::Fixed(n) => n.to_string(),
                                cronc::cl_math::MathDim::Symbolic(s) => s.clone(),
                            }).collect::<Vec<_>>().join(", "))
                        } else if !p.dims.is_empty() {
                            format!("[{}]", p.dims.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(", "))
                        } else {
                            String::new()
                        };
                        format!("{}: {}{}", p.name, p.dtype.as_str(), dim_str)
                    }).collect();

                    let has_sym = params.iter().any(|p| !p.symbolic_dims.is_empty());
                    let sym_note = if has_sym {
                        "\n- **Symbolic Contract**: Verified algebraic contraction polymorphism"
                    } else {
                        ""
                    };

                    return Some(format!(
                        "### CRON Mathematical Function: `{}`\n\n\
                        - **Signature**: `{}({})`{}\n\
                        - **Definition**: `{}`\n\
                        - **Lowered C23**: `void cron_{}(...);`",
                        name, name, param_strs.join(", "), sym_note, body, name
                    ));
                }
            }
        }

        // Check constants
        for item in &prog.items {
            if let cronc::MathItem::Constant { name, val } = item {
                if name == word {
                    return Some(format!(
                        "### CRON Scalar Constant: `{}`\n\n\
                        - **Type**: `Scalar (f32)`\n\
                        - **Value**: `{}`\n\
                        - **Memory Footprint**: 4 bytes\n\
                        - **Silicon Storage**: Immediate Constant / Dedicated Register",
                        name, val
                    ));
                }
            }
        }

        // Check inferred tensor shapes
        let shapes = prog.infer_shapes_map();
        if let Some(shape) = shapes.get(word) {
            let total_elems: usize = if shape.is_empty() { 1 } else { shape.iter().product() };
            let bytes = total_elems * 4;
            let mem_str = if bytes < 1024 {
                format!("{} bytes", bytes)
            } else if bytes < 1024 * 1024 {
                format!("{:.2} KB ({} bytes)", bytes as f64 / 1024.0, bytes)
            } else {
                format!("{:.2} MB ({} bytes)", bytes as f64 / (1024.0 * 1024.0), bytes)
            };

            return Some(format!(
                "### CRON Tensor Variable: `{}`\n\n\
                - **Tensor Shape**: `{:?}`\n\
                - **Total Elements**: `{}`\n\
                - **Memory Footprint (FP32)**: {}\n\
                - **Silicon Layout**: Contiguous Row-Major C-order Array Buffer\n\
                - **Hardware Alignment**: 64-byte AVX-512 / Tensor Core cache-line aligned",
                word, shape, total_elems, mem_str
            ));
        }
    }

    None
}

pub fn get_cl_hover_documentation(word: &str) -> Option<String> {
    let upper = word.to_ascii_uppercase();

    // Check register R0..R63
    if (upper.starts_with('R') || upper.starts_with("REG")) && upper.len() >= 2 {
        let num_part = upper.strip_prefix("REG").unwrap_or_else(|| &upper[1..]);
        if let Ok(reg_idx) = num_part.parse::<u32>() {
            if reg_idx <= 63 {
                return Some(format!(
                    "### Register `R{}` (CRON 256-Core 4D-Torus & Clifford ISA)\n\n\
                    - **Register Bank**: 32-bit General-Purpose Architectural Register File (R0..R63 extended)\n\
                    - **Addressable Width**: 6-bit Physical Index (0x{:02X})\n\
                    - **Read Ports**: 2 read ports per execution ALU\n\
                    - **Write Ports**: 1 dedicated write port per cycle\n\
                    - **Silicon Hazard**: 0-cycle forwarding bypass network enabled",
                    reg_idx, reg_idx
                ));
            }
        }
    }

    // Check bundle header B0000..B9999999 or L0000.. or C0000..
    if (upper.starts_with('B') || upper.starts_with('L') || upper.starts_with('C'))
        && upper.len() >= 3
        && upper[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        let cycle = u64::from_str_radix(&upper[1..], 16).unwrap_or(0);
        return Some(format!(
            "### VLIW Microcode Bundle `{}`\n\n\
            - **Target Execution Cycle**: Cycle {}\n\
            - **Parallel Issue**: 4 independent execution slots `[ALU0 | ALU1 | MEM | NOC]`\n\
            - **Theoretical Peak**: 4.0 Instructions Per Cycle (IPC)\n\
            - **Instruction Width**: 40 bytes / 94 characters (canonical 4-slot fixed alignment)",
            upper, cycle
        ));
    }

    let lookup = upper.strip_prefix('.').unwrap_or(&upper);
    match lookup {
        "NOP" => Some(
            "### Micro-Op: `NOP` (No Operation)\n\n\
            - **Silicon Port**: ALU0 / ALU1 / MEM / NOC\n\
            - **Execution Latency**: 0 Cycles\n\
            - **Dynamic Energy**: 0.00 pJ (Clock-gated silicon way)\n\
            - **Canonical Encoding**: `_NO00$000>`".to_string()
        ),
        "ADD" => Some(
            "### Micro-Op: `ADD` (32-bit Integer / Fixed-Point Addition)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.42 pJ\n\
            - **Throughput**: 1 op / cycle / port\n\
            - **Silicon Way**: Dual-issue integer execution lane\n\
            - **Canonical Encoding**: `_AD00$000>`".to_string()
        ),
        "SUB" => Some(
            "### Micro-Op: `SUB` (32-bit Integer Subtraction)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.42 pJ\n\
            - **Throughput**: 1 op / cycle\n\
            - **Canonical Encoding**: `_SU00$000>`".to_string()
        ),
        "MUL" => Some(
            "### Micro-Op: `MUL` (32-bit Integer Multiplier)\n\n\
            - **Silicon Port**: ALU0 (Primary Multiplier Pipeline)\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.85 pJ\n\
            - **Throughput**: 1 op / cycle\n\
            - **Canonical Encoding**: `_MU00$000>`".to_string()
        ),
        "DOT_I2" => Some(
            "### Micro-Op: `DOT_I2` (BitNet 1.58b Ternary Dot Product)\n\n\
            - **Silicon Port**: ALU0 (Photonic & Ternary Sub-Byte Unit)\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.08 pJ (Zero-MAC bypass enabled)\n\
            - **Vector Parallelism**: 16 Ternary MACs {-1, 0, +1} in a single cycle\n\
            - **Attainable Peak**: 32 INT-ops / cycle\n\
            - **Canonical Encoding**: `_D200$000>`".to_string()
        ),
        "DOT_I4" => Some(
            "### Micro-Op: `DOT_I4` (Sub-Byte 4-bit Quantized Dot Product)\n\n\
            - **Silicon Port**: ALU0\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.18 pJ\n\
            - **Vector Parallelism**: 8 INT4 MACs per cycle\n\
            - **Canonical Encoding**: `_D400$000>`".to_string()
        ),
        "ATTN_STEP" => Some(
            "### Micro-Op: `ATTN_STEP` (Photonic FlashAttention-2 Head Step)\n\n\
            - **Silicon Port**: ALU0 / Photonic MZI Mesh (Brain 2)\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Energy Efficiency**: 128.4 TOPS/Watt (0.15 pJ)\n\
            - **Kernel Operation**: Fused $Q \\times K^T \\odot \\text{Softmax} \\times V$\n\
            - **Canonical Encoding**: `_AT00$000>`".to_string()
        ),
        "LIF_STEP" => Some(
            "### Micro-Op: `LIF_STEP` (Leaky Integrate-and-Fire Spiking Step)\n\n\
            - **Silicon Port**: ALU0 / Brain 4 Neuromorphic Crossbar\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Event Dynamic Energy**: 0.12 pJ / spike event\n\
            - **Neuron Dynamics**: $V_{mem}[t+1] = V_{mem}[t] \\cdot \\lambda + I_{syn}[t] - V_{th} \\cdot S[t]$\n\
            - **Canonical Encoding**: `_LI00$000>`".to_string()
        ),
        "STDP_STEP" => Some(
            "### Micro-Op: `STDP_STEP` (Spike-Timing-Dependent Plasticity Adaptation)\n\n\
            - **Silicon Port**: ALU0 / Brain 4 AER Synaptic Crossbar\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.14 pJ\n\
            - **Learning Rule**: Bi-directional exponential synaptic trace $\\Delta w = \\eta \\cdot e^{-\\Delta t / \\tau}$\n\
            - **Canonical Encoding**: `_ST00$000>`".to_string()
        ),
        "SWIGLU_STEP" => Some(
            "### Micro-Op: `SWIGLU_STEP` (Fused Gated SwiGLU Step)\n\n\
            - **Silicon Port**: ALU0\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.52 pJ\n\
            - **Mathematical Semantics**: $(\\text{SiLU}(x W_{\\text{gate}}) \\odot (x W_{\\text{up}}))$\n\
            - **Canonical Encoding**: `_SW00$000>`".to_string()
        ),
        "ROPE_STEP" => Some(
            "### Micro-Op: `ROPE_STEP` (Rotary Position Embedding Step)\n\n\
            - **Silicon Port**: ALU0\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.38 pJ\n\
            - **Rotation**: Complex Givens 2D subspace rotation $R_{\\Theta, m}^d$\n\
            - **Canonical Encoding**: `_RO00$000>`".to_string()
        ),
        "RMSNORM_STEP" => Some(
            "### Micro-Op: `RMSNORM_STEP` (Root Mean Square Layer Normalization)\n\n\
            - **Silicon Port**: ALU0\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.32 pJ\n\
            - **Formula**: $x / \\sqrt{\\frac{1}{d}\\sum x_i^2 + \\epsilon} \\cdot \\gamma$\n\
            - **Canonical Encoding**: `_RM00$000>`".to_string()
        ),
        "LOAD" => Some(
            "### Micro-Op: `LOAD` (Local Scratchpad SRAM Read)\n\n\
            - **Silicon Port**: MEM\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.65 pJ\n\
            - **Bandwidth**: 160 GB/s per core\n\
            - **Bank Architecture**: 16-Bank PGAS Scratchpad with GF(2^4) Swizzling\n\
            - **Canonical Encoding**: `_LD00#000>`".to_string()
        ),
        "STORE" => Some(
            "### Micro-Op: `STORE` (Local Scratchpad SRAM Write)\n\n\
            - **Silicon Port**: MEM\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.68 pJ\n\
            - **Bandwidth**: 160 GB/s per core\n\
            - **Canonical Encoding**: `_ST00#000>`".to_string()
        ),
        "SWIZZLE" => Some(
            "### Micro-Op: `SWIZZLE` (Galois Field Bank Permutation)\n\n\
            - **Silicon Port**: MEM\n\
            - **Execution Latency**: 0 Cycles (Combinational XOR crossbar)\n\
            - **Stall Penalty**: Provably 0 Bank Conflicts across 16 banks\n\
            - **Galois Polynomial**: $x^4 + x + 1$ over $GF(2^4)$\n\
            - **Canonical Encoding**: `_SW00#000>`".to_string()
        ),
        "PREFETCH" => Some(
            "### Micro-Op: `PREFETCH` (Asynchronous SRAM Line Prefetch)\n\n\
            - **Silicon Port**: MEM\n\
            - **Execution Latency**: 1 Cycle non-blocking\n\
            - **Dynamic Energy**: 0.20 pJ\n\
            - **Canonical Encoding**: `_PR00#000>`".to_string()
        ),
        "ROUTE_DOR" => Some(
            "### Micro-Op: `ROUTE_DOR` (Dimension-Ordered Torus Routing)\n\n\
            - **Silicon Port**: NOC (4D-Torus Network Router)\n\
            - **Deadlock Guarantee**: Provably deadlock-free via Dally-Seitz order ($X \\to Y \\to Z \\to W$)\n\
            - **Routing Latency**: 2 Cycles per hop\n\
            - **Link Bandwidth**: 5.82 Gbps inter-core channel\n\
            - **Canonical Encoding**: `_RO00@000>`".to_string()
        ),
        "SEND" => Some(
            "### Micro-Op: `SEND` (Torus Asynchronous Packet Send)\n\n\
            - **Silicon Port**: NOC\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.85 pJ\n\
            - **Canonical Encoding**: `_SE00@000>`".to_string()
        ),
        "RECV" => Some(
            "### Micro-Op: `RECV` (Torus Channel Packet Receive)\n\n\
            - **Silicon Port**: NOC\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.85 pJ\n\
            - **Canonical Encoding**: `_RE00@000>`".to_string()
        ),
        "BROADCAST" => Some(
            "### Micro-Op: `BROADCAST` (Spatial Multicast across 4D Torus)\n\n\
            - **Silicon Port**: NOC\n\
            - **Execution Latency**: 2 Cycles\n\
            - **Dynamic Energy**: 2.10 pJ\n\
            - **Canonical Encoding**: `_BC00@000>`".to_string()
        ),
        "COLLECTIVE" => Some(
            "### Micro-Op: `COLLECTIVE` (Hardware Multi-Die AllReduce/AllGather)\n\n\
            - **Silicon Port**: NOC\n\
            - **Execution Latency**: 3 Cycles\n\
            - **Scaling**: Up to 16 Dies, 4,096 cores\n\
            - **Canonical Encoding**: `_CO00@000>`".to_string()
        ),
        "ROT_SANDWICH" => Some(
            "### Micro-Op: `ROT_SANDWICH` (Clifford Cl(4,0) Space-Time Spindle Sandwich)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Formula**: $v' = R \\cdot v \\cdot \\tilde{R}$\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.28 pJ (Curvature Geometric Transformation)\n\
            - **Canonical Encoding**: `_RS00$000>`".to_string()
        ),
        "CLIFFORD_ADD" => Some(
            "### Micro-Op: `CLIFFORD_ADD` (Geometric Multivector Grade Addition)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Parallelism**: 16-element Cl(4,0) blade superposition in 1 cycle\n\
            - **Dynamic Energy**: 0.35 pJ\n\
            - **Canonical Encoding**: `_CA00$000>`".to_string()
        ),
        "TROPIC_MAX" => Some(
            "### Micro-Op: `TROPIC_MAX` (Tropical Semiring (max, +) Unit)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Semantics**: $c_k = \\max_j (a_{kj} + b_j)$ (Multiplier-Free Geodesic)\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.05 pJ\n\
            - **Canonical Encoding**: `_TM00$000>`".to_string()
        ),
        "LAYER_SHIFT" => Some(
            "### Micro-Op: `LAYER_SHIFT` (4D Hyper-Sphere Layer IV Sensory Clamping)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.10 pJ\n\
            - **Canonical Encoding**: `_LY00$000>`".to_string()
        ),
        "STDP_LEARN" => Some(
            "### Micro-Op: `STDP_LEARN` (Spike-Timing-Dependent Plasticity Online Weight Patch)\n\n\
            - **Silicon Port**: ALU0 / Brain 4 AER Synapse Crossbar\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.12 pJ\n\
            - **Canonical Encoding**: `_SL00$000>`".to_string()
        ),
        "WAVE_SPREAD" => Some(
            "### Micro-Op: `WAVE_SPREAD` (Astrocytic Glial Wave Diffusion & Interference)\n\n\
            - **Silicon Port**: ALU0 / Brain 5 Glial Workspace\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Dynamic Energy**: 0.15 pJ\n\
            - **Canonical Encoding**: `_WF00$000>`".to_string()
        ),
        "EMIT_CHAR" => Some(
            "### Micro-Op: `EMIT_CHAR` (Sovereign Mind Autonomous Character Emission)\n\n\
            - **Silicon Port**: NOC / Terminal Host Port\n\
            - **Execution Latency**: 1 Cycle\n\
            - **Canonical Encoding**: `_EC00@000>`".to_string()
        ),
        "MLA" | "MLA_ATTN" | "MLA_STEP" => Some(
            "### Micro-Op: `MLA` (DeepSeek-V3 Multi-Head Latent Attention)\n\n\
            - **Silicon Port**: ALU0 / Photonic MZI Mesh (Brain 2)\n\
            - **KV Compression**: 16x low-rank latent compression ($d_c = 512$)\n\
            - **Hardware Architecture**: Zero DSP multipliers; dual RoPE + decoupled key projection\n\
            - **Attainable Efficiency**: 128.4 TOPS/Watt (0.09 pJ / token)\n\
            - **Canonical Encoding**: `_ML00$000>`".to_string()
        ),
        "REASON" | "COT_STEP" | "THINK" => Some(
            "### Micro-Op: `REASON` (DeepSeek-R1 Test-Time Reasoning Engine)\n\n\
            - **Silicon Subsystem**: Brain 1 Symbolic Reasoner + GRPO Advantage Normalizer\n\
            - **Execution Mode**: Monologue internal token generation within `<think>` bounds\n\
            - **Dynamic Energy**: 0.17 pJ (Reversible Landauer floor)\n\
            - **Formal Proof**: Verified convergence under Z3/Bitwuzla formal invariants\n\
            - **Canonical Encoding**: `_RS00$000>`".to_string()
        ),
        "TITANS" | "TITANS_MEM" | "RWKV7" => Some(
            "### Micro-Op: `TITANS` (Google Titans Neural Long-Term Memory & RWKV-7)\n\n\
            - **Silicon Port**: ALU0 / Brain 4 Synaptic Crossbar\n\
            - **Complexity**: Flat $O(1)$ constant time & constant memory per token\n\
            - **Mechanism**: Surprise-metric associative memory matrix with adaptive decay\n\
            - **Context Window**: Unbounded (1M+ tokens without KV cache growth)\n\
            - **Canonical Encoding**: `_TT00$000>`".to_string()
        ),
        "LIQUID" | "LTC" | "TIME_DILATE" => Some(
            "### Micro-Op: `LIQUID` (Continuous-Time Liquid Neural Network / LTC)\n\n\
            - **Silicon Subsystem**: Brain 5 Dynamical Differential State Solvers\n\
            - **Equation**: $\\frac{dx}{dt} = -[\\frac{1}{\\tau} + f(x, I)] x + A f(x, I)$\n\
            - **Reflex Latency**: Sub-microsecond shock reflex under time dilation\n\
            - **Memory Bound**: <1 KB total internal state footprint\n\
            - **Canonical Encoding**: `_LQ00$000>`".to_string()
        ),
        "LORA" | "CL_LORA" => Some(
            "### Directive / Micro-Op: `CL_LORA` (Sub-Byte Discrete Low-Rank Adapter)\n\n\
            - **Adapter Rank**: $r \\in [1..8]$ with sub-byte 5-trits-per-byte packing\n\
            - **Footprint**: < 2 KB binary (100% fits within <= 4.1 KB L1 CPU/silicon cache)\n\
            - **Backpropagation**: 0-FLOP integer sign updates; zero floating-point noise\n\
            - **Hardware Multipliers**: 0 DSPs (pure integer add/sub projection)\n\
            - **Canonical Encoding**: `_LR00$000>`".to_string()
        ),
        "MONGOL" | "MONGOL_MORPH" => Some(
            "### Directive: `MONGOL` (Agglutinative Mongolian Morphological NLP)\n\n\
            - **Grammar Subsystem**: Finite-state morphological suffix decomposer\n\
            - **Token Efficiency**: 8.57x token sequence compression over subword BPE\n\
            - **Script Bridge**: Bidirectional Cyrillic <-> Traditional Hudum Mongolian script\n\
            - **Vowel Harmony**: Validates masculine/feminine vowel congruence in hardware".to_string()
        ),
        "STAGE" => Some(
            "### Directive: `.stage <name>` (VLIW Pipeline Stage Declaration)\n\n\
            Declares an isolated computational pipeline stage with bounded hardware resource usage,\n\
            register allocation constraints, and target precision (`f32`, `i8`, `i2` ternary).".to_string()
        ),
        "WEIGHTS" => Some(
            "### Directive: `.weights <file.safetensors|file.cll>` (Model Weights Ingestion)\n\n\
            Binds external model parameters (SafeTensors, GGUF, or sub-byte CL-LoRA .cll)\n\
            directly into local core SRAM scratchpad addresses without host DRAM latency.".to_string()
        ),
        "SPHERE_GEODESIC" | "_MD" => Some(
            "### Micro-Op: `SPHERE_GEODESIC` (4D Curvature Geodesic ALU)\n\n\
            - **Silicon Port**: ALU0 / ALU1\n\
            - **Mathematical Space**: $S^3 \\times \\mathbb{Z}_{100}$ Riemannian 4D Hypersphere\n\
            - **Geodesic Distance**: $d_g(p, q) = \\arccos(\\langle p, q \\rangle)$\n\
            - **Zero Multipliers**: Multiplier-free phase curvature computation in 1 cycle\n\
            - **Dynamic Energy**: 0.08 pJ / projection\n\
            - **Canonical Encoding**: `_MD00$000>`".to_string()
        ),
        "SPHERE_RK4" | "SPHERE_FLOW" | "_OD" => Some(
            "### Micro-Op: `SPHERE_RK4` (Continuous-Time Manifold Runge-Kutta Flow)\n\n\
            - **Silicon Subsystem**: Brain 5 Dynamical State Solvers\n\
            - **Equation**: Continuous-time flow on $S^3$: $x_{t+h} = \\exp_{x_t}(h \\cdot f(x_t))$\n\
            - **Norm Preservation**: 100% strict preservation of $\\|x\\|=1.0000$ (drift = 0.0000)\n\
            - **Hopf Fibration**: Surjective projection $\\pi: S^3 \\to S^2$ for 3D state visualization\n\
            - **Canonical Encoding**: `_OD00$000>`".to_string()
        ),
        "DPSTA_TRANS" | "_DP" => Some(
            "### Micro-Op: `DPSTA_TRANS` (Deterministic 4-Tact 94-State Coordinate Phase Transition)\n\n\
            - **Silicon Subsystem**: Brain 1 DPSTA Finite-State Transducer\n\
            - **States**: 94 printable ASCII alphabet states ($S_{33}..S_{126}$)\n\
            - **Cycle Precision**: Deterministic 4-tact microcode state machine\n\
            - **Compression**: Phase-Quantized KV-Cache compression with 4x memory savings\n\
            - **Canonical Encoding**: `_DP00$000>`".to_string()
        ),
        "QUAROT_HADAMARD" | "_TT" => Some(
            "### Micro-Op: `QUAROT_HADAMARD` (Fast Walsh-Hadamard Orthogonal Rotation)\n\n\
            - **Silicon Port**: ALU0 (Photonic / Vector Mesh)\n\
            - **Outlier Suppression**: >141x peak activation outlier suppression\n\
            - **Quantization**: Enables lossless W4A4 sub-byte tensor execution\n\
            - **Hardware Complexity**: $O(N \\log N)$ zero-multiplier adder/subtractor tree\n\
            - **Canonical Encoding**: `_TT00$000>`".to_string()
        ),
        "SAGE_ATTENTION" | "_QA" => Some(
            "### Micro-Op: `SAGE_ATTENTION` (Sub-byte Streaming Quantized FlashAttention)\n\n\
            - **Silicon Subsystem**: Brain 2 Streaming Attention Engine\n\
            - **Feature**: INT4/INT8 streaming attention with hardware accumulator clipping\n\
            - **Memory Efficiency**: Eliminates softmax overflow and intermediate DRAM traffic\n\
            - **Canonical Encoding**: `_QA00$000>`".to_string()
        ),
        "BITNET_POPCOUNT" | "_PO" => Some(
            "### Micro-Op: `BITNET_POPCOUNT` (0-FLOP Ternary Dot Product Adder-Tree)\n\n\
            - **Silicon Port**: ALU0 (Ternary Adder Pipeline)\n\
            - **Encoding**: 2-bit sub-byte packed ternary weights {-1, 0, +1}\n\
            - **Execution**: Pure bitwise XOR / POPCOUNT adder-tree (0 DSP multipliers, 0 FLOPs)\n\
            - **Throughput**: 32 ternary weights evaluated per 64-bit cycle\n\
            - **Canonical Encoding**: `_PO00$000>`".to_string()
        ),
        _ => None,
    }
}

pub fn get_hover_documentation(word: &str) -> Option<String> {
    match word {
        // Keywords
        "lin" => Some(
            "### `lin` (Linear Type Qualifier)\n\n\
            Declares an **affine linear resource** that must be consumed **exactly once** via `consume(...)`.\n\
            Prevents hardware resource leaks, optical wavefront decoherence, and uncollected buffer pins.\n\n\
            ```cron\n\
            let lin photon_bundle: wave_t = pack_wave(amp=[1, 2], phase=[0, 0])\n\
            let out = consume(photon_bundle) // Must be consumed!\n\
            ```".to_string()
        ),
        "let" => Some(
            "### `let` Binding\n\n\
            Binds an immutable variable. Reassignment without `mut` triggers compiler error `E0005`.\n\n\
            ```cron\n\
            let max_thermal_thresh: u32 = 180\n\
            let mut dynamic_counter: u32 = 0\n\
            ```".to_string()
        ),
        "mut" => Some(
            "### `mut` (Mutable Qualifier)\n\n\
            Permits variable reassignment. Required if the identifier is modified later in scope.".to_string()
        ),
        "brain" => Some(
            "### `brain` (Cognitive Subsystem Block)\n\n\
            Declares a dedicated neuromorphic partition on the 4D-Torus mesh.\n\
            - **Brain 1**: Symbolic Reasoner\n\
            - **Brain 2**: Photonic GEMM & Attention\n\
            - **Brain 3**: Quantum Phase & Fredkin/Toffoli\n\
            - **Brain 4**: Neuromorphic STDP Plasticity\n\
            - **Brain 5**: Chaos Diffusion & Lorenz Attractor\n\
            - **Brain 6**: Metacognitive Sentry Arbiter\n\n\
            ```cron\n\
            brain AttentionEngine [cores=64, thermal_max=180] {\n\
                // High-performance photonic pipeline\n\
            }\n\
            ```".to_string()
        ),
        "resilient_compute" => Some(
            "### `resilient_compute` (Hardware Sentry Block)\n\n\
            Activates Brain 6 self-healing watchdog. Automatically deflects packets to neighboring\n\
            cores across 4D axes (X+, Y-, Z+, W-) if local thermal threshold is exceeded.".to_string()
        ),
        "consume" => Some(
            "### `consume(resource)`\n\n\
            Consumes an affine linear resource (`lin`). Once consumed, the variable cannot be used\n\
            or consumed again (`E0003`).".to_string()
        ),
        "spawn" => Some(
            "### `spawn expr`\n\n\
            Spawns a hardware fiber task on a neighboring torus core. Returns an asynchronous handle.".to_string()
        ),
        "await" => Some(
            "### `await handle`\n\n\
            Awaits the completion of a hardware fiber task and retrieves the computed result.".to_string()
        ),
        "region" => Some(
            "### `region ArenaName [target=SELF] { ... }`\n\n\
            0-cycle hardware arena memory region. All allocations inside are discarded upon scope exit\n\
            unless explicitly transferred via `export`.".to_string()
        ),
        "export" => Some(
            "### `export symbol as alias`\n\n\
            Exports a symbol from a local region or module into the parent scope.".to_string()
        ),
        // Types
        "wave_t" => Some(
            "### `wave_t` (Photonic Wavefront Type)\n\n\
            Represents an optical phase and amplitude state (4 amplitudes, 4 phases) modulated\n\
            through Mach-Zehnder Interferometer (MZI) meshes.".to_string()
        ),
        "ext_addr_t" => Some(
            "### `ext_addr_t` (External HBM Address)\n\n\
            32-bit/64-bit physical memory address for High Bandwidth Memory (HBM3e) DMA channels.".to_string()
        ),
        "spk_stamp" => Some(
            "### `spk_stamp` (Neuromorphic Spike Timestamp)\n\n\
            Sub-nanosecond hardware timestamp used for STDP synaptic weight updates.".to_string()
        ),
        // Builtins
        "pack_wave" => Some(
            "### `pack_wave(amp=[...], phase=[...]) -> wave_t`\n\n\
            Constructs an optical wavefront packet for Brain 2 Photonic Matrix Multipliers.".to_string()
        ),
        "compute_attention_head" => Some(
            "### `compute_attention_head(q, k, v, mask) -> (attn_out, therm)`\n\n\
            Executes single-cycle photonic tensor attention head in optical domain at 128.4 TOPS/W.".to_string()
        ),
        "step_synaptic_plasticity" => Some(
            "### `step_synaptic_plasticity(pre, post, rate)`\n\n\
            Applies biological Spike-Timing-Dependent Plasticity (STDP) across local synaptic crossbars.".to_string()
        ),
        "ground_and_unify" => Some(
            "### `ground_and_unify(fact, rules) -> match_t`\n\n\
            Performs branchless hyper-edge unification on Brain 1 Symbolic Knowledge Graph.".to_string()
        ),
        "sphere_geodesic" => Some(
            "### `sphere_geodesic(p, q) -> distance`\n\n\
            Computes the Riemannian geodesic distance $d_g(p, q) = \\arccos(\\langle p, q \\rangle)$ on the 4D hypersphere $S^3$ without floating-point DSP multipliers.".to_string()
        ),
        "sphere_rk4" | "sphere_flow" => Some(
            "### `sphere_rk4(x, h, f) -> x_next`\n\n\
            Executes a 4th-Order Manifold Runge-Kutta continuous dynamical flow step on $S^3$ with 100% strict unit norm preservation (drift = 0.0000).".to_string()
        ),
        "dpsta_transition" => Some(
            "### `dpsta_transition(state, input, tact) -> next_state`\n\n\
            Executes a deterministic 4-tact state transition across the 94 printable ASCII coordinate alphabet ($S_{33}..S_{126}$).".to_string()
        ),
        "dpsta_pack" => Some(
            "### `dpsta_pack(kv_floats) -> dpdata_block`\n\n\
            Compresses attention key-value states into compact Base-94 phase coordinates (4x memory compression vs FP32).".to_string()
        ),
        "dpsta_unpack" => Some(
            "### `dpsta_unpack(dpdata_block) -> kv_floats`\n\n\
            Decompresses Base-94 phase coordinates back into continuous attention tensors with >0.98 cosine fidelity.".to_string()
        ),
        "quarot_hadamard" => Some(
            "### `quarot_hadamard(tensor) -> rotated_tensor`\n\n\
            Applies randomized orthogonal Walsh-Hadamard Transform rotation, suppressing peak outliers by >141x for lossless W4A4 execution.".to_string()
        ),
        "sage_attention" => Some(
            "### `sage_attention(q, k, v) -> attn_out`\n\n\
            Sub-byte streaming quantized FlashAttention with INT4/INT8 accumulator clamping, eliminating softmax overflow.".to_string()
        ),
        "rmsnorm" => Some(
            "### `rmsnorm(x, gamma, eps) -> normalized_x`\n\n\
            Hardware Root Mean Square layer normalization in 1 cycle.".to_string()
        ),
        "swiglu" => Some(
            "### `swiglu(x, w_gate, w_up) -> activated_x`\n\n\
            Fused Gated SwiGLU non-linear activation unit $(\\text{SiLU}(x W_{\\text{gate}}) \\odot (x W_{\\text{up}}))$.".to_string()
        ),
        "bitnet_popcount" => Some(
            "### `bitnet_popcount(activations, ternary_weights) -> acc`\n\n\
            Universal 0-FLOP sub-byte ternary dot product using 2-bit popcount adder-trees without DSP multipliers.".to_string()
        ),
        _ => None,
    }
}

// === Slot-Aware Autocompletion for .cl ===

pub fn get_cl_completion_items(line_str: &str, character: usize) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // High-Level Sovereign CL v2.0 (Non-VLIW lines: @kernel, §Block, :=, ↦, ⟳, !Δ)
    if !line_str.contains('|') && !line_str.trim_start().starts_with("B0") && !line_str.trim_start().starts_with("W0") {
        for (sym, insert, doc) in sovereign_primitives_list() {
            items.push(CompletionItem {
                label: sym.to_string(),
                kind: 14, // Keyword / Operator
                detail: Some(format!("Sovereign CL v2: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", sym, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
        for (sym, insert, doc) in zero_word_math_primitives_list() {
            items.push(CompletionItem {
                label: sym.to_string(),
                kind: 3, // Function / Math Operator
                detail: Some(format!("CRON Math Calculus: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", sym, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
        return items;
    }

    // Determine slot index from cursor character position
    let slot_idx = if line_str.contains('|') {
        let prefix = if character <= line_str.len() {
            &line_str[..character]
        } else {
            line_str
        };
        prefix.matches('|').count()
    } else {
        0
    };

    // Slot 0: ALU0 (Arithmetic, Logical, Photonic, Neuromorphic)
    if slot_idx == 0 {
        let alu0_ops = [
            ("ADD", "ADD R0, R1, R2", "32-bit Integer Add (1 cyc, 0.42 pJ)"),
            ("SUB", "SUB R0, R1, R2", "32-bit Integer Subtract (1 cyc, 0.42 pJ)"),
            ("MUL", "MUL R0, R1, R2", "32-bit Integer Multiply (1 cyc, 0.85 pJ)"),
            ("DOT_I2", "DOT_I2 R0, R1, R2", "BitNet 1.58b Ternary Dot Product (16 MACs, 0.08 pJ)"),
            ("DOT_I4", "DOT_I4 R0, R1, R2", "INT4 Sub-Byte Dot Product (8 MACs, 0.18 pJ)"),
            ("ATTN_STEP", "ATTN_STEP R0, R1, R2", "Photonic FlashAttention Step (128.4 TOPS/W, 0.15 pJ)"),
            ("LIF_STEP", "LIF_STEP R0, R1, R2", "Neuromorphic LIF Spike Update (0.12 pJ/spike)"),
            ("STDP_STEP", "STDP_STEP R0, R1, R2", "STDP Synaptic Weight Plasticity Step (0.14 pJ)"),
            ("SWIGLU_STEP", "SWIGLU_STEP R0, R1, R2", "Fused Gated SwiGLU Activation (1 cyc, 0.52 pJ)"),
            ("ROPE_STEP", "ROPE_STEP R0, R1, R2", "Rotary Position Embedding Step (1 cyc, 0.38 pJ)"),
            ("RMSNORM_STEP", "RMSNORM_STEP R0, R1, R2", "RMS Layer Normalization Step (1 cyc, 0.32 pJ)"),
            ("SPARSE_MAC", "SPARSE_MAC R0, R1, R2", "2:4 Structural Sparse MAC (2.0x Speedup)"),
            ("NOP", "NOP", "No-Operation (0 cycles, 0 pJ)"),
        ];
        for (op, insert, doc) in alu0_ops {
            items.push(CompletionItem {
                label: op.to_string(),
                kind: 3, // Function
                detail: Some(format!("ALU0: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", op, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
    } else if slot_idx == 1 {
        // Slot 1: ALU1 (Auxiliary arithmetic, comparisons, bitwise)
        let alu1_ops = [
            ("ADD", "ADD R3, R4, R5", "Auxiliary 32-bit Add (1 cyc, 0.42 pJ)"),
            ("SUB", "SUB R3, R4, R5", "Auxiliary 32-bit Subtract (1 cyc, 0.42 pJ)"),
            ("AND", "AND R3, R4, R5", "Bitwise AND (1 cyc, 0.35 pJ)"),
            ("OR", "OR R3, R4, R5", "Bitwise OR (1 cyc, 0.35 pJ)"),
            ("XOR", "XOR R3, R4, R5", "Bitwise XOR (1 cyc, 0.35 pJ)"),
            ("SHL", "SHL R3, R4, 2", "Logical Shift Left (1 cyc, 0.35 pJ)"),
            ("SHR", "SHR R3, R4, 2", "Logical Shift Right (1 cyc, 0.35 pJ)"),
            ("CMP", "CMP R3, R4, R5", "Comparison flags update (1 cyc, 0.35 pJ)"),
            ("MOV", "MOV R3, R4", "Register move (1 cyc, 0.30 pJ)"),
            ("NOP", "NOP", "No-Operation (0 cycles, 0 pJ)"),
        ];
        for (op, insert, doc) in alu1_ops {
            items.push(CompletionItem {
                label: op.to_string(),
                kind: 3,
                detail: Some(format!("ALU1: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", op, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
    } else if slot_idx == 2 {
        // Slot 2: MEM (Memory hierarchy)
        let mem_ops = [
            ("LOAD", "LOAD [R6+0]", "Scratchpad L1 SRAM Read (1 cyc, 0.65 pJ, 160 GB/s)"),
            ("STORE", "STORE [R6+0], R7", "Scratchpad L1 SRAM Write (1 cyc, 0.68 pJ, 160 GB/s)"),
            ("SWIZZLE", "SWIZZLE GF2_4", "Galois Field GF(2^4) 16-Bank Conflict-Free Permutation (0 cyc)"),
            ("PREFETCH", "PREFETCH [R6+64]", "Non-blocking SRAM line prefetch (1 cyc, 0.20 pJ)"),
            ("SPARSE_LOAD", "SPARSE_LOAD [R6], 2_4", "Fetch 2:4 compressed weight tile (1 cyc, 0.45 pJ)"),
            ("BARRIER", "BARRIER", "Execution pipeline barrier (1 cyc, 0.10 pJ)"),
            ("NOP", "NOP", "No-Operation (0 cycles, 0 pJ)"),
        ];
        for (op, insert, doc) in mem_ops {
            items.push(CompletionItem {
                label: op.to_string(),
                kind: 3,
                detail: Some(format!("MEM: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", op, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
    } else if slot_idx == 3 {
        // Slot 3: NOC (4D-Torus routing)
        let noc_ops = [
            ("ROUTE_DOR", "ROUTE_DOR +X", "Dimension-Ordered Torus Routing (2 cyc, Dally-Seitz deadlock-free)"),
            ("ROUTE_DOR_Y", "ROUTE_DOR +Y", "Torus Y-axis Dimension-Ordered Routing (2 cyc)"),
            ("SEND", "SEND (1,0,0,0), R8", "Asynchronous Direct Packet Send (1 cyc, 0.85 pJ)"),
            ("RECV", "RECV R8", "Torus Channel Packet Receive (1 cyc, 0.85 pJ)"),
            ("BROADCAST", "BROADCAST +X, R8", "Spatial Torus Multicast (2 cyc, 2.10 pJ)"),
            ("COLLECTIVE", "COLLECTIVE ALLREDUCE", "Hardware 4D Mesh AllReduce Collective (3 cyc)"),
            ("NOP", "NOP", "No-Operation (0 cycles, 0 pJ)"),
        ];
        for (op, insert, doc) in noc_ops {
            items.push(CompletionItem {
                label: op.to_string(),
                kind: 3,
                detail: Some(format!("NOC: {}", doc)),
                documentation: Some(MarkupContent {
                    kind: "markdown".to_string(),
                    value: format!("**`{}`**\n\n{}", op, doc),
                }),
                insert_text: Some(insert.to_string()),
            });
        }
    }

    // Always include hardware registers R0..R15
    for r in 0..=15 {
        items.push(CompletionItem {
            label: format!("R{}", r),
            kind: 21, // Constant / Variable
            detail: Some(format!("Hardware Register R{} (32-bit architectural)", r)),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("Register `R{}`: 32-bit general-purpose register file location.", r),
            }),
            insert_text: Some(format!("R{}", r)),
        });
    }

    items
}

pub fn sovereign_primitives_list() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("@kernel", "@kernel ${1:kernel_name}\n.target silicon.generic\n.ipc_target 4.0\n.cores 256\n@mesh [4×4×4×4] : ⮑ (x+1, y, z, w)", "Pillar 1: Silicon Target Contract"),
        ("@mesh", "@mesh [4×4×4×4] : ⮑ (x+1, y, z, w)", "Pillar 1: 4D Torus NoC Mesh Routing"),
        ("@io", "@io {\n    in:  ${1:In}[16×16:FP32]  @sram[0x0000]\n    out: ${2:Out}[16×16:FP32]  @sram[0x0400]\n}", "Pillar 2: Memory Tier Binding (@sram/@dram/@hbm)"),
        ("§Block", "§${1:ComputeBlock}[16×16:FP32] {\n    ${2:X} ↦ ${3:Y} :\n        ${3:Y} := silu(${2:X})\n}", "Pillar 3: Mathematical Tensor Morphism Block"),
        ("↦", "${1:X} ↦ ${2:Y} :", "Pillar 3: Tensor Morphism Mapping"),
        (":=", "${1:Dest} := ${2:Expr}", "Pure Functional Assignment (no verbal 'let')"),
        ("←", "#R${1:1} ← @sram[${2:0x0000}]", "Silicon Register Memory Transfer (Load/Store)"),
        ("⟳", "⟳ (${1:256}) {\n    #R1 ← @sram[0x0000]\n    @sram[0x0400] ← #R1\n}", "Pillar 4: Zero-Overhead Hardware Loop"),
        ("?", "? (${1:cond}) ⮞ {\n    #R1 ← 1\n} : {\n    #R1 ← 0\n}", "Pillar 4: Branchless Hardware Conditional"),
        ("⮌", "⮌ ${1:result}", "Silicon Yield / Morphism Return"),
        ("!Δ", "!Δ { |${1:Y}| < 1e4, latency < 10ns }", "Pillar 4: Invariant & Watchdog Sentry"),
        ("<~>", "${1:a} <~> ${2:b}", "Photonic MZI Phase Coupling / Unitary Multiply"),
        (">>=", "${1:a} >>= ${2:b}", "Continuous Associative State-Space Scan (Mamba-2 SSD)"),
        ("<|>", "${1:a} <|> ${2:b}", "Log-Space Entropy Gating / Softmax"),
        ("<%>", "${1:a} <%> ${2:b}", "Polynomial Crypto-Ring Modulo Reduction"),
        ("/@", "${1:a} /@ ${2:b}", "Radix Tree Subtree Graft (Zero-Copy KV Cache)"),
        ("~*~", "${1:a} ~*~ ${2:b}", "Periodic Harmonic Wave Resonance"),
        ("^", "${1:a} ^ ${2:b}", "Clifford Geometric Algebra Exterior Wedge Product"),
        ("~", "~${1:a}", "Clifford Rotor Reversion"),
    ]
}

pub fn zero_word_math_primitives_list() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("Ψ_g", "Ψ_g(${1:Q}, ${2:K}, ${3:V}, ${4:H_q}, ${5:H_kv}, ${6:τ})", "Grouped-Query Causal FlashAttention (GQA)"),
        ("Ψ", "Ψ(${1:Q}, ${2:K}, ${3:V}, ${4:τ})", "Causal Multi-Head FlashAttention"),
        ("Ψ_c", "Ψ_c(${1:q}, ${2:k}, ${3:v}, ${4:K_cache}, ${5:V_cache}, ${6:τ})", "Streaming KV-Cache Causal Attention"),
        ("Ψ_r", "Ψ_r(${1:q}, ${2:k_new}, ${3:v_new}, ${4:K_ring}, ${5:V_ring}, ${6:pos}, ${7:max_len})", "Hardware-Level Cyclic Ring-Buffer KV Cache Attention"),
        ("N", "N(${1:X}, ${2:γ})", "Fused Root Mean Square Normalization (RMSNorm)"),
        ("N_ε", "N_ε(${1:X}, ${2:γ}, ${3:ε})", "RMSNorm with custom epsilon tolerance"),
        ("S", "S(${1:X}, ${2:W_g}, ${3:W_u}, ${4:W_d})", "Fused SwiGLU Gated Feed-Forward Network"),
        ("B", "B(${1:X}, ${2:W}, ${3:γ})", "BitNet 1.03b / 1.58b Sub-Byte Quantized Linear Layer"),
        ("Ξ_m", "Ξ_m(${1:X}, ${2:W_gate}, ${3:experts}, ${4:2})", "Sparse Mixture of Experts (MoE) Top-K Gating with Flash Softmax"),
        ("M", "M(${1:X}, ${2:decay_dt})", "Mamba-2 State Space Duality (SSD) Linear Scan"),
        ("ρ", "ρ(${1:Z})", "Row-wise Softmax Probability Distribution"),
        ("Φ", "Φ(${1:X})", "Continuous Rotary Position Embedding (RoPE)"),
        ("Λ", "Λ(${1:Z}, ${2:y})", "Cross-Entropy Loss with Exact Pullback"),
        ("Λ_g", "Λ_g(${1:logp}, ${2:ref}, ${3:adv}, ${4:0.2}, ${5:0.04})", "Group Relative Policy Optimization (GRPO) Reasoning Loss"),
        ("∇", "∇_${1:W}(${2:L})", "Analytic Reverse-Mode Backpropagation Gradient"),
        ("Δ", "Δ(${1:W}, ${2:G}, ${3:η})", "Discrete Weight Optimizer Step (W - η * G)"),
        ("Ω", "Ω(${1:W}, ${2:G}, ${3:M}, ${4:V}, ${5:t}, ${6:lr})", "Pure Functional Analytical AdamW Step"),
        ("ι", "ι(\"${1:text}\")", "Zero-verbal Subword/Byte Token Embedding Lookup"),
        ("δ", "δ(${1:tokens})", "UTF-8 Token Detokenizer to Text"),
        ("χ", "χ(${1:logits}, ${2:temp}, ${3:top_p}, ${4:seed})", "Deterministic Autoregressive In-Silicon Token Sampler"),
        ("κ_k", "κ_k(${1:logits}, ${2:k})", "Bitonic Top-K Logit Selection Filter"),
        ("κ_p", "κ_p(${1:logits}, ${2:p})", "Nucleus Cumulative Probability Filter"),
        ("κ", "κ(${1:G}, ${2:max_norm})", "Global Frobenius Gradient Norm Clipping"),
        ("∑", "∑(${1:X})", "Parallel Reduction Sum"),
        ("∑_k", "∑_k(${1:X})", "High-Precision Neumaier Compensated Summation (Zero-Loss)"),
        ("μ", "μ(${1:X})", "Parallel Mean Reduction"),
        ("σ", "σ(${1:X})", "Fused SiLU / Sigmoid Activation Function"),
        ("√", "√(${1:X})", "Vectorized SIMD Square Root"),
        ("∥", "∥${1:X}∥", "L2 Frobenius Norm"),
        ("⊗", "${1:u} ⊗ ${2:v}", "Tensor Outer Product Matrix Construction"),
        ("<<", "<< \"${1:libcl/math/module.cl}\"", "Pure AST Zero-Verbal Module Import"),
        ("@", "@\"${1:model.safetensors}:${2:tensor_name}\"", "Zero-Copy Direct SafeTensors Weight Binding"),
    ]
}

// === High-Level Language & Sovereign CL Completions ===

pub fn get_completion_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // 1. Sovereign CL v2.0 5-Pillar Non-Verbal Primitives (Primary)
    for (sym, insert, doc) in sovereign_primitives_list() {
        items.push(CompletionItem {
            label: sym.to_string(),
            kind: 14, // Keyword / Operator
            detail: Some(format!("Sovereign CL v2: {}", doc)),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("**`{}`**\n\n{}", sym, doc),
            }),
            insert_text: Some(insert.to_string()),
        });
    }

    // 1b. Zero-Word Mathematical Calculus Operators (Primary)
    for (sym, insert, doc) in zero_word_math_primitives_list() {
        items.push(CompletionItem {
            label: sym.to_string(),
            kind: 3, // Function / Math Operator
            detail: Some(format!("CRON Math Calculus: {}", doc)),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("**`{}`**\n\n{}", sym, doc),
            }),
            insert_text: Some(insert.to_string()),
        });
    }

    // 2. Legacy Keywords (Secondary)
    let keywords = [
        ("let", "let var = value", "Bind immutable variable"),
        ("lin", "let lin res = value", "Declare affine linear variable (must consume once)"),
        ("mut", "let mut counter = 0", "Declare mutable variable"),
        ("grad", "let grad weight = 0.5", "Declare autodiff gradient-tracked variable"),
        ("def", "def func(param: type) -> type { }", "Define function"),
        ("return", "return expr", "Return from function"),
        ("if", "if condition { } else { }", "Conditional branch"),
        ("else", "else { }", "Else block"),
        ("while", "while cond { }", "While loop"),
        ("for", "for item in iter { }", "For in loop"),
        ("brain", "brain Name [cores=64] { }", "Cognitive subsystem partition"),
        ("resilient_compute", "resilient_compute [fallback_target=X+] { }", "Self-healing sentry block"),
        ("region", "region Arena [target=SELF] { }", "Zero-cycle hardware arena region"),
        ("consume", "consume(linear_var)", "Consume linear resource"),
        ("spawn", "spawn background_task()", "Spawn hardware fiber on torus"),
        ("await", "await handle", "Await async fiber handle"),
        ("export", "export item as alias", "Export symbol from region or module"),
        ("import", "import { Symbol } from \"path.cl\"", "Import module symbol"),
        ("struct", "struct Name { field: type }", "Declare struct data type"),
        ("trait", "trait Name { def method(self); }", "Declare trait interface"),
        ("impl", "impl Trait for Struct { }", "Implement trait for struct"),
    ];

    for (kw, insert, doc) in keywords {
        items.push(CompletionItem {
            label: kw.to_string(),
            kind: 14, // Keyword
            detail: Some(doc.to_string()),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("**{}**\n\n{}", kw, doc),
            }),
            insert_text: Some(insert.to_string()),
        });
    }

    // Types
    let types = [
        ("u32", "32-bit unsigned integer"),
        ("i32", "32-bit signed integer"),
        ("u64", "64-bit unsigned integer"),
        ("i64", "64-bit signed integer"),
        ("f32", "32-bit IEEE-754 float"),
        ("f64", "64-bit IEEE-754 double"),
        ("bf16", "16-bit brain floating point"),
        ("c64", "64-bit complex float (2x32-bit real/imag pair)"),
        ("bool", "Boolean (true / false)"),
        ("wave_t", "Photonic MZI Wavefront (4 amplitudes, 4 phases)"),
        ("ext_addr_t", "High-Bandwidth Memory (HBM) physical address"),
        ("spk_stamp", "Neuromorphic spike timestamp"),
        ("vec4_i8", "Packed 4-element 8-bit integer vector"),
        ("tensor", "Multi-dimensional tensor handle"),
    ];

    for (t, doc) in types {
        items.push(CompletionItem {
            label: t.to_string(),
            kind: 7, // Class / Type
            detail: Some(doc.to_string()),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("Type `{}`: {}", t, doc),
            }),
            insert_text: Some(t.to_string()),
        });
    }

    // Builtins
    let builtins = [
        ("pack_wave", "pack_wave(amp=[...], phase=[...])", "Construct optical wavefront"),
        ("compute_attention_head", "compute_attention_head(q, k, v, mask)", "Single-cycle photonic attention"),
        ("step_synaptic_plasticity", "step_synaptic_plasticity(pre, post, rate)", "STDP synaptic weight update"),
        ("ground_and_unify", "ground_and_unify(fact, rules)", "Symbolic knowledge graph unification"),
        ("init_kg_partition", "init_kg_partition(partition_id=0)", "Initialize knowledge graph memory partition"),
        ("assert_triple", "assert_triple(subj, pred, obj)", "Store semantic RDF knowledge triple"),
        ("batch_norm_quantize", "batch_norm_quantize(data, mean, scale)", "Fixed-point batch normalization"),
        ("dense_relu_step", "dense_relu_step(input, weight, bias)", "Quantized matrix ReLU step"),
        ("spatial_broadcast", "spatial_broadcast(axis=0, packet)", "Broadcast packet across 4D torus"),
        ("sphere_geodesic", "sphere_geodesic(p, q)", "4D Riemannian geodesic distance on S^3"),
        ("sphere_rk4", "sphere_rk4(x, h, f)", "Continuous-time manifold Runge-Kutta flow on S^3"),
        ("dpsta_transition", "dpsta_transition(state, input, tact)", "4-tact 94-state deterministic transition"),
        ("dpsta_pack", "dpsta_pack(kv_floats)", "Base-94 Phase-Quantized KV-cache compression"),
        ("dpsta_unpack", "dpsta_unpack(dpdata_block)", "Base-94 Phase-Quantized KV-cache decompression"),
        ("quarot_hadamard", "quarot_hadamard(tensor)", "Fast Walsh-Hadamard outlier suppression rotation"),
        ("sage_attention", "sage_attention(q, k, v)", "Sub-byte streaming quantized FlashAttention"),
        ("rmsnorm", "rmsnorm(x, gamma, eps)", "Hardware RMSNorm normalizer"),
        ("swiglu", "swiglu(x, w_gate, w_up)", "Fused Gated SwiGLU activation"),
        ("bitnet_popcount", "bitnet_popcount(activations, ternary_weights)", "0-FLOP ternary dot product adder-tree"),
    ];

    for (b, insert, doc) in builtins {
        items.push(CompletionItem {
            label: b.to_string(),
            kind: 3, // Function
            detail: Some(doc.to_string()),
            documentation: Some(MarkupContent {
                kind: "markdown".to_string(),
                value: format!("Function `{}`\n\n{}", insert, doc),
            }),
            insert_text: Some(insert.to_string()),
        });
    }

    items
}

// === CRON Language Server State & Handler ===

pub struct LspServer {
    pub documents: HashMap<String, String>,
    pub is_shutdown: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            is_shutdown: false,
        }
    }

    pub fn handle_message(&mut self, json_str: &str) -> Option<String> {
        let req: JsonRpcRequest = match serde_json::from_str(json_str) {
            Ok(r) => r,
            Err(_) => return None,
        };

        match req.method.as_str() {
            "initialize" => {
                let result = serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": 1,
                        "hoverProvider": true,
                        "completionProvider": {
                            "resolveProvider": false,
                            "triggerCharacters": [".", ":", "$", "@", " ", "_", "R"]
                        },
                        "signatureHelpProvider": {
                            "triggerCharacters": ["(", ","],
                            "retriggerCharacters": [","]
                        },
                        "definitionProvider": true,
                        "documentSymbolProvider": true,
                        "codeActionProvider": true,
                        "documentFormattingProvider": true,
                        "inlayHintProvider": true,
                        "semanticTokensProvider": {
                            "legend": {
                                "tokenTypes": [
                                    "keyword", "type", "function", "variable", "parameter",
                                    "class", "struct", "interface", "label", "macro",
                                    "comment", "string", "number", "operator"
                                ],
                                "tokenModifiers": [
                                    "declaration", "readonly", "modification", "defaultLibrary"
                                ]
                            },
                            "full": true
                        }
                    },
                    "serverInfo": {
                        "name": "cron-lsp",
                        "version": "3.17.1"
                    }
                });
                Some(self.format_response(req.id, Some(result), None))
            }
            "initialized" => None,
            "textDocument/didOpen" => {
                if let Some(params) = req.params {
                    if let Some(text_doc) = params.get("textDocument") {
                        let uri = text_doc.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                        let text = text_doc.get("text").and_then(|t| t.as_str()).unwrap_or("");
                        self.documents.insert(uri.to_string(), text.to_string());
                        return Some(self.compute_and_publish_diagnostics(uri, text));
                    }
                }
                None
            }
            "textDocument/didChange" => {
                if let Some(params) = req.params {
                    let uri = params.get("textDocument")
                        .and_then(|td| td.get("uri"))
                        .and_then(|u| u.as_str())
                        .unwrap_or("");
                    if let Some(content_changes) = params.get("contentChanges").and_then(|c| c.as_array()) {
                        if let Some(last_change) = content_changes.last() {
                            if let Some(text) = last_change.get("text").and_then(|t| t.as_str()) {
                                self.documents.insert(uri.to_string(), text.to_string());
                                return Some(self.compute_and_publish_diagnostics(uri, text));
                            }
                        }
                    }
                }
                None
            }
            "textDocument/didClose" => {
                if let Some(params) = req.params {
                    if let Some(uri) = params.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()) {
                        self.documents.remove(uri);
                    }
                }
                None
            }
            "textDocument/hover" => {
                let hover_result = self.compute_hover(&req.params);
                Some(self.format_response(req.id, Some(hover_result), None))
            }
            "textDocument/completion" => {
                let completions = self.compute_completions(&req.params);
                let result = serde_json::json!({
                    "isIncomplete": false,
                    "items": completions
                });
                Some(self.format_response(req.id, Some(result), None))
            }
            "textDocument/signatureHelp" => {
                let sig = self.compute_signature_help(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(sig).unwrap_or(serde_json::Value::Null)), None))
            }
            "textDocument/definition" => {
                let locs = self.compute_definition(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(locs).unwrap_or(serde_json::Value::Array(vec![]))), None))
            }
            "textDocument/documentSymbol" => {
                let syms = self.compute_document_symbols(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(syms).unwrap_or(serde_json::Value::Array(vec![]))), None))
            }
            "textDocument/codeAction" => {
                let actions = self.compute_code_actions(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(actions).unwrap_or_default()), None))
            }
            "textDocument/formatting" => {
                let edits = self.compute_formatting(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(edits).unwrap_or(serde_json::Value::Array(vec![]))), None))
            }
            "textDocument/inlayHint" => {
                let hints = self.compute_inlay_hints(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(hints).unwrap_or(serde_json::Value::Array(vec![]))), None))
            }
            "textDocument/semanticTokens/full" => {
                let data = self.compute_semantic_tokens(&req.params);
                let result = serde_json::json!({
                    "data": data
                });
                Some(self.format_response(req.id, Some(result), None))
            }
            "shutdown" => {
                self.is_shutdown = true;
                Some(self.format_response(req.id, Some(serde_json::Value::Null), None))
            }
            "exit" => None,
            _ => {
                if req.id.is_some() {
                    Some(self.format_response(
                        req.id,
                        None,
                        Some(JsonRpcError {
                            code: -32601,
                            message: format!("Method '{}' not found", req.method),
                            data: None,
                        }),
                    ))
                } else {
                    None
                }
            }
        }
    }

    pub fn compute_diagnostics_for_source(&self, uri: &str, source: &str) -> Vec<Diagnostic> {
        if is_cl_document(uri, source) {
            check_cl_diagnostics(source)
        } else {
            let diags = cronc::check_source_diagnostics(source);
            diags.into_iter().map(|d| {
                let line_0 = if d.line > 0 { (d.line - 1) as u32 } else { 0 };
                let col_0 = if d.col > 0 { (d.col - 1) as u32 } else { 0 };
                let end_col = col_0 + (d.span_len.max(1) as u32);

                let mut msg = d.message;
                if let Some(help) = d.help {
                    msg.push_str(&format!("\nhelp: {}", help));
                }
                if let Some(note) = d.note {
                    msg.push_str(&format!("\nnote: {}", note));
                }

                Diagnostic {
                    range: Range {
                        start: Position { line: line_0, character: col_0 },
                        end: Position { line: line_0, character: end_col },
                    },
                    severity: 1, // Error
                    code: Some(d.code.to_string()),
                    source: Some("cronc".to_string()),
                    message: msg,
                }
            }).collect()
        }
    }

    pub fn compute_and_publish_diagnostics(&self, uri: &str, source: &str) -> String {
        let diagnostics = self.compute_diagnostics_for_source(uri, source);
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/publishDiagnostics".to_string(),
            params: serde_json::json!({
                "uri": uri,
                "diagnostics": diagnostics
            }),
        };
        serde_json::to_string(&notification).unwrap_or_default()
    }

    fn compute_hover(&self, params: &Option<serde_json::Value>) -> serde_json::Value {
        if let Some(p) = params {
            let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
            let pos = p.get("position");
            let line = pos.and_then(|p| p.get("line")).and_then(|l| l.as_u64()).unwrap_or(0) as usize;
            let character = pos.and_then(|p| p.get("character")).and_then(|c| c.as_u64()).unwrap_or(0) as usize;

            if let Some(doc) = self.documents.get(uri) {
                let is_cl = is_cl_document(uri, doc);
                if let Some(line_str) = doc.lines().nth(line) {
                    if let Some((word, word_start, word_end)) = extract_word_at_pos(line_str, character) {
                        let doc_md = if is_cl {
                            get_math_hover_documentation(doc, &word)
                                .or_else(|| get_cl_hover_documentation(&word))
                                .or_else(|| get_hover_documentation(&word))
                        } else {
                            get_hover_documentation(&word)
                                .or_else(|| get_math_hover_documentation(doc, &word))
                                .or_else(|| get_cl_hover_documentation(&word))
                        };

                        if let Some(md) = doc_md {
                            return serde_json::json!({
                                "contents": {
                                    "kind": "markdown",
                                    "value": md
                                },
                                "range": {
                                    "start": { "line": line, "character": word_start },
                                    "end": { "line": line, "character": word_end }
                                }
                            });
                        }
                    }
                }
            }
        }

        serde_json::Value::Null
    }

    fn compute_completions(&self, params: &Option<serde_json::Value>) -> Vec<CompletionItem> {
        if let Some(p) = params {
            let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
            let pos = p.get("position");
            let line = pos.and_then(|p| p.get("line")).and_then(|l| l.as_u64()).unwrap_or(0) as usize;
            let character = pos.and_then(|p| p.get("character")).and_then(|c| c.as_u64()).unwrap_or(0) as usize;

            if let Some(doc) = self.documents.get(uri) {
                if is_cl_document(uri, doc) {
                    let line_str = doc.lines().nth(line).unwrap_or("");
                    return get_cl_completion_items(line_str, character);
                }
            }
        }
        get_completion_items()
    }

    // === SignatureHelp — shows parameter hints when user types '(' or ',' in .cr ===
    fn compute_signature_help(&self, params: &Option<serde_json::Value>) -> Option<SignatureHelp> {
        let p = params.as_ref()?;
        let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
        let pos = p.get("position");
        let line_idx = pos.and_then(|p| p.get("line")).and_then(|l| l.as_u64()).unwrap_or(0) as usize;
        let character = pos.and_then(|p| p.get("character")).and_then(|c| c.as_u64()).unwrap_or(0) as usize;

        let doc = self.documents.get(uri)?;
        let line_str = doc.lines().nth(line_idx)?;
        let prefix = &line_str[..character.min(line_str.len())];

        // Find the function name before the open paren
        let before_paren = prefix.rfind('(')?;
        let fn_prefix = prefix[..before_paren].trim();
        // Count commas to get active parameter index
        let after_paren = &prefix[before_paren + 1..];
        let active_param = after_paren.chars().filter(|&c| c == ',').count() as u32;

        // Find the identifier immediately before '('
        let fn_name = fn_prefix.split(|c: char| !c.is_alphanumeric() && c != '_').next_back().unwrap_or("");

        // .cr builtin signature table
        type SigEntry = (&'static str, &'static str, &'static str, &'static [(&'static str, &'static str)]);
        let sigs: &[SigEntry] = &[
            ("pack_wave",
             "pack_wave(amp: [f32; 4], phase: [f32; 4]) -> wave_t",
             "Construct a photonic MZI wavefront packet for Brain 2 processing.",
             &[("amp", "Array of 4 amplitude values [0.0 .. 1.0]"),
               ("phase", "Array of 4 phase values in radians [0 .. 2π]")]),
            ("compute_attention_head",
             "compute_attention_head(q: tensor, k: tensor, v: tensor, mask: u32) -> (tensor, f32)",
             "Single-cycle photonic FlashAttention-2 head at 128.4 TOPS/W.",
             &[("q", "Query tensor"), ("k", "Key tensor"), ("v", "Value tensor"), ("mask", "Causal mask bits")]),
            ("step_synaptic_plasticity",
             "step_synaptic_plasticity(pre: spk_stamp, post: spk_stamp, rate: f32)",
             "Apply STDP synaptic plasticity across neuromorphic crossbars.",
             &[("pre", "Pre-synaptic spike timestamp"), ("post", "Post-synaptic spike timestamp"), ("rate", "Learning rate η")]),
            ("ground_and_unify",
             "ground_and_unify(fact: u64, rules: u64) -> match_t",
             "Branchless hyper-edge unification on Brain 1 Knowledge Graph.",
             &[("fact", "Encoded RDF fact triple ID"), ("rules", "Rule set pointer")]),
            ("init_kg_partition",
             "init_kg_partition(partition_id: u32)",
             "Initialize a knowledge graph memory partition.",
             &[("partition_id", "Partition index (0 .. 15)")]),
            ("assert_triple",
             "assert_triple(subj: u64, pred: u64, obj: u64)",
             "Store a semantic RDF knowledge triple in the graph store.",
             &[("subj", "Subject entity ID"), ("pred", "Predicate relation ID"), ("obj", "Object entity ID")]),
            ("batch_norm_quantize",
             "batch_norm_quantize(data: tensor, mean: f32, scale: f32) -> tensor",
             "Fixed-point batch normalization for quantized inference.",
             &[("data", "Input tensor"), ("mean", "Batch mean"), ("scale", "Quantization scale factor")]),
            ("dense_relu_step",
             "dense_relu_step(input: tensor, weight: tensor, bias: f32) -> tensor",
             "INT4 quantized dense matrix multiply + ReLU activation.",
             &[("input", "Input activation tensor"), ("weight", "Weight matrix"), ("bias", "Bias scalar")]),
            ("spatial_broadcast",
             "spatial_broadcast(axis: u32, packet: u64)",
             "Broadcast a NoC packet across 4D-Torus axis.",
             &[("axis", "Torus axis 0..3 (X,Y,Z,W)"), ("packet", "Encoded packet payload")]),
            ("consume",
             "consume(resource: lin T) -> T",
             "Consume a linear resource exactly once. Prevents memory leaks.",
             &[("resource", "A `lin`-qualified variable to consume")]),
        ];

        for (name, label, doc_text, params_list) in sigs {
            if *name == fn_name {
                let parameters: Vec<ParameterInformation> = params_list.iter().map(|(pname, pdoc)| {
                    ParameterInformation {
                        label: pname.to_string(),
                        documentation: Some(MarkupContent {
                            kind: "markdown".to_string(),
                            value: pdoc.to_string(),
                        }),
                    }
                }).collect();
                return Some(SignatureHelp {
                    signatures: vec![SignatureInformation {
                        label: label.to_string(),
                        documentation: Some(MarkupContent {
                            kind: "markdown".to_string(),
                            value: doc_text.to_string(),
                        }),
                        parameters: Some(parameters),
                    }],
                    active_signature: Some(0),
                    active_parameter: Some(active_param),
                });
            }
        }
        None
    }

pub fn uri_to_path(uri: &str) -> Option<std::path::PathBuf> {
    if let Some(stripped) = uri.strip_prefix("file:///") {
        let decoded = stripped.replace("%20", " ");
        Some(std::path::PathBuf::from(decoded))
    } else if let Some(stripped) = uri.strip_prefix("file://") {
        let decoded = stripped.replace("%20", " ");
        Some(std::path::PathBuf::from(decoded))
    } else {
        None
    }
}

    // === DocumentSymbol — file outline (functions, structs, brains, .cl labels) ===
    fn compute_document_symbols(&self, params: &Option<serde_json::Value>) -> Vec<DocumentSymbol> {
        let mut symbols: Vec<DocumentSymbol> = Vec::new();
        let p = match params { Some(p) => p, None => return symbols };
        let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
        
        let doc_content = if let Some(d) = self.documents.get(uri) {
            d.clone()
        } else if let Some((_, d)) = self.documents.iter().find(|(k, _)| k.eq_ignore_ascii_case(uri)) {
            d.clone()
        } else if let Some(path) = Self::uri_to_path(uri) {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            return symbols;
        };

        if doc_content.is_empty() {
            return symbols;
        }

        if is_cl_document(uri, &doc_content) {
            // .cl: emit @kernel, @entry, bundle, @CORE, .core, labels, and bundle headers B#### as symbols
            for (i, line) in doc_content.lines().enumerate() {
                let t = line.trim();
                let line_u32 = i as u32;
                let is_bundle = t.len() >= 3
                    && (t.starts_with('B') || t.starts_with('b') || t.starts_with('L') || t.starts_with('C'))
                    && t.chars().nth(1).map(|c| c.is_ascii_hexdigit()).unwrap_or(false)
                    && t.contains(':');
                let is_core = t.starts_with("@CORE") || t.starts_with(".core");
                let is_kernel = t.starts_with("@kernel") || t.starts_with("@entry") || t.starts_with("@macro");
                let is_topo_bundle = t.starts_with("bundle ");
                let is_label = t.ends_with(':') && !t.contains(' ') && !t.starts_with(';') && !t.starts_with("//");

                if is_bundle || is_core || is_kernel || is_topo_bundle || is_label {
                    let (name, kind, detail) = if is_kernel {
                        let k_name = t.split_whitespace().nth(1).unwrap_or(t).trim_end_matches(':').to_string();
                        (k_name, 12, "Kernel Function") // 12=Function
                    } else if is_topo_bundle {
                        let b_name = t.strip_prefix("bundle ").unwrap_or(t).split(['{', ' ']).next().unwrap_or(t).trim().to_string();
                        (b_name, 5, "Topological Bundle") // 5=Class
                    } else if is_core {
                        (t.split([':', ' ']).next().unwrap_or(t).to_string(), 5, "CORE Directive")
                    } else if is_label {
                        (t.trim_end_matches(':').to_string(), 12, "Label")
                    } else {
                        (t.split([':', ' ']).next().unwrap_or(t).to_string(), 12, "VLIW Bundle")
                    };

                    let r = Range {
                        start: Position { line: line_u32, character: 0 },
                        end: Position { line: line_u32, character: line.len() as u32 },
                    };
                    symbols.push(DocumentSymbol {
                        name,
                        kind,
                        range: r.clone(),
                        selection_range: r,
                        detail: Some(detail.to_string()),
                        children: None,
                    });
                }
            }
        } else {
            // .cr: simple line-based scan for def, struct, trait, brain, region
            for (i, line) in doc_content.lines().enumerate() {
                let t = line.trim();
                let line_u32 = i as u32;
                let make_range = |l: u32, len: u32| Range {
                    start: Position { line: l, character: 0 },
                    end: Position { line: l, character: len },
                };
                let len = line.len() as u32;

                if let Some(rest) = t.strip_prefix("def ") {
                    // def funcname(...
                    let name = rest.split(['(', ' ']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 12, // Function
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("function".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("fn ") {
                    let name = rest.split(['(', ' ']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 12, // Function
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("function".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("struct ") {
                    let name = rest.split(['{', ' ', '<']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 23, // Struct
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("struct".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("trait ") {
                    let name = rest.split(['{', ' ', '<']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 11, // Interface
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("trait".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("brain ") {
                    let name = rest.split(['[', ' ', '{']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 5, // Class
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("brain partition".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("region ") {
                    let name = rest.split(['[', ' ', '{']).next().unwrap_or("?").to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 14, // Keyword / Module
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("region".to_string()),
                        children: None,
                    });
                } else if let Some(rest) = t.strip_prefix("impl ") {
                    let name = rest.split('{').next().unwrap_or("?").trim().to_string();
                    symbols.push(DocumentSymbol {
                        name,
                        kind: 11, // Interface / Impl
                        range: make_range(line_u32, len),
                        selection_range: make_range(line_u32, len),
                        detail: Some("impl block".to_string()),
                        children: None,
                    });
                }
            }
        }
        symbols
    }

    // === GoToDefinition — text-searches all open documents for the definition ===
    fn compute_definition(&self, params: &Option<serde_json::Value>) -> Vec<Location> {
        let mut locations: Vec<Location> = Vec::new();
        let p = match params { Some(p) => p, None => return locations };
        let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
        let pos = p.get("position");
        let line_idx = pos.and_then(|p| p.get("line")).and_then(|l| l.as_u64()).unwrap_or(0) as usize;
        let character = pos.and_then(|p| p.get("character")).and_then(|c| c.as_u64()).unwrap_or(0) as usize;

        let doc = match self.documents.get(uri) { Some(d) => d, None => return locations };
        let line_str = match doc.lines().nth(line_idx) { Some(l) => l, None => return locations };
        let (word, _, _) = match extract_word_at_pos(line_str, character) { Some(w) => w, None => return locations };

        // For .cl: look for labels like the bundle header or @CORE directive
        // For .cr: look for `def <word>`, `fn <word>`, `struct <word>`, `trait <word>`, `brain <word>`, `impl <word>`
        let def_patterns: Vec<String> = if is_cl_document(uri, doc) {
            // Bundle label: B0000, B0001, ...
            vec![format!("{}:", word), format!("{} ", word)]
        } else {
            vec![
                format!("def {}", word),
                format!("fn {}", word),
                format!("struct {}", word),
                format!("trait {}", word),
                format!("brain {}", word),
                format!("impl {}", word),
                format!("enum {}", word),
                format!("type {}", word),
            ]
        };

        // Search all open documents
        for (doc_uri, doc_text) in &self.documents {
            for (i, doc_line) in doc_text.lines().enumerate() {
                for pat in &def_patterns {
                    if doc_line.contains(pat.as_str()) {
                        let col = doc_line.find(pat.as_str()).unwrap_or(0) as u32;
                        let line_u32 = i as u32;
                        locations.push(Location {
                            uri: doc_uri.clone(),
                            range: Range {
                                start: Position { line: line_u32, character: col },
                                end: Position { line: line_u32, character: col + word.len() as u32 },
                            },
                        });
                        break;
                    }
                }
            }
        }
        locations
    }

    // === Semantic Tokens Provider (LSP 3.17) ===
    fn compute_semantic_tokens(&self, params: &Option<serde_json::Value>) -> Vec<u32> {
        let p = match params { Some(p) => p, None => return Vec::new() };
        let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
        let doc = match self.documents.get(uri) { Some(d) => d, None => return Vec::new() };

        let mut raw_tokens: Vec<RawSemanticToken> = Vec::new();

        if is_cl_document(uri, doc) {
            for (line_idx, line) in doc.lines().enumerate() {
                let l_u32 = line_idx as u32;
                let trimmed = line.trim_start();
                let leading_spaces = (line.len() - trimmed.len()) as u32;

                if let Some(pos) = line.find(';') {
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: pos as u32,
                        len: (line.len() - pos) as u32,
                        token_type: 10,
                        token_modifiers: 0,
                    });
                } else if let Some(pos) = line.find("//") {
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: pos as u32,
                        len: (line.len() - pos) as u32,
                        token_type: 10,
                        token_modifiers: 0,
                    });
                }

                if trimmed.starts_with("@CORE") {
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: leading_spaces,
                        len: 5,
                        token_type: 5, // class
                        token_modifiers: 1, // declaration
                    });
                }

                if (trimmed.starts_with('B') || trimmed.starts_with('b')) && trimmed.contains(':') {
                    let lbl_len = trimmed.find(':').unwrap_or(5);
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: leading_spaces,
                        len: lbl_len as u32,
                        token_type: 8, // label
                        token_modifiers: 1, // declaration
                    });
                }

                let mut current_word = String::new();
                let mut word_start = 0;
                for (char_idx, ch) in line.char_indices() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '#' {
                        if current_word.is_empty() {
                            word_start = char_idx;
                        }
                        current_word.push(ch);
                    } else {
                        if !current_word.is_empty() {
                            classify_cl_token(&current_word, l_u32, word_start as u32, &mut raw_tokens);
                            current_word.clear();
                        }
                    }
                }
                if !current_word.is_empty() {
                    classify_cl_token(&current_word, l_u32, word_start as u32, &mut raw_tokens);
                }
            }
        } else {
            for (line_idx, line) in doc.lines().enumerate() {
                let l_u32 = line_idx as u32;
                if let Some(pos) = line.find("//") {
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: pos as u32,
                        len: (line.len() - pos) as u32,
                        token_type: 10, // comment
                        token_modifiers: 0,
                    });
                } else if let Some(pos) = line.find('#') {
                    raw_tokens.push(RawSemanticToken {
                        line: l_u32,
                        col: pos as u32,
                        len: (line.len() - pos) as u32,
                        token_type: 10, // comment
                        token_modifiers: 0,
                    });
                }
            }

            let mut lexer = cronc::lexer::Lexer::new(doc);
            if let Ok(tokens) = lexer.tokenize() {
                let mut prev_kw: Option<&'static str> = None;
                for sp in &tokens {
                    let line_0 = if sp.span.line > 0 { (sp.span.line - 1) as u32 } else { 0 };
                    let col_0 = if sp.span.col > 0 { (sp.span.col - 1) as u32 } else { 0 };
                    let len = sp.span.len as u32;

                    match &sp.value {
                        cronc::token::Token::Lin => {
                            raw_tokens.push(RawSemanticToken {
                                line: line_0,
                                col: col_0,
                                len,
                                token_type: 0, // keyword
                                token_modifiers: 2, // readonly
                            });
                            prev_kw = Some("lin");
                        }
                        cronc::token::Token::Def => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = Some("def");
                        }
                        cronc::token::Token::Struct => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = Some("struct");
                        }
                        cronc::token::Token::Trait => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = Some("trait");
                        }
                        cronc::token::Token::Brain => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = Some("brain");
                        }
                        cronc::token::Token::Region => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = Some("region");
                        }
                        cronc::token::Token::Let | cronc::token::Token::Mut | cronc::token::Token::Consume
                        | cronc::token::Token::Spawn | cronc::token::Token::Await | cronc::token::Token::ResilientCompute
                        | cronc::token::Token::If | cronc::token::Token::Else | cronc::token::Token::While
                        | cronc::token::Token::For | cronc::token::Token::Return | cronc::token::Token::Export
                        | cronc::token::Token::Impl | cronc::token::Token::Schedule | cronc::token::Token::Fuse => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 0, token_modifiers: 0 });
                            prev_kw = None;
                        }
                        cronc::token::Token::Ident(name) => {
                            let (token_type, token_modifiers) = match prev_kw {
                                Some("def") => (2, 1),       // function, declaration
                                Some("struct") => (6, 1),    // struct, declaration
                                Some("trait") => (7, 1),     // interface, declaration
                                Some("brain") => (5, 1),     // class, declaration
                                _ => {
                                    if is_builtin_cognitive_fn(name) {
                                        (2, 8) // function, defaultLibrary
                                    } else if is_builtin_type_name(name) {
                                        (1, 8) // type, defaultLibrary
                                    } else if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                                        (1, 0) // type, user-defined struct/trait/enum
                                    } else {
                                        (3, 0) // variable
                                    }
                                }
                            };
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type, token_modifiers });
                            prev_kw = None;
                        }
                        cronc::token::Token::StringLit(_) => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 11, token_modifiers: 0 });
                            prev_kw = None;
                        }
                        cronc::token::Token::IntLit(_) | cronc::token::Token::HexLit(_) | cronc::token::Token::FloatLit(_) => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 12, token_modifiers: 0 });
                            prev_kw = None;
                        }
                        cronc::token::Token::Axis(_) => {
                            raw_tokens.push(RawSemanticToken { line: line_0, col: col_0, len, token_type: 8, token_modifiers: 0 });
                            prev_kw = None;
                        }
                        _ => {
                            prev_kw = None;
                        }
                    }
                }
            }
        }

        raw_tokens.sort_by_key(|t| (t.line, t.col));
        raw_tokens.dedup_by_key(|t| (t.line, t.col));

        let mut data: Vec<u32> = Vec::new();
        let mut prev_line = 0;
        let mut prev_col = 0;

        for tok in raw_tokens {
            let delta_line = tok.line.saturating_sub(prev_line);
            let delta_col = if delta_line == 0 {
                tok.col.saturating_sub(prev_col)
            } else {
                tok.col
            };
            data.push(delta_line);
            data.push(delta_col);
            data.push(tok.len);
            data.push(tok.token_type);
            data.push(tok.token_modifiers);

            prev_line = tok.line;
            prev_col = tok.col;
        }

        data
    }

    fn compute_code_actions(&self, params: &Option<serde_json::Value>) -> Vec<CodeAction> {
        let mut actions = Vec::new();
        if let Some(p) = params {
            let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
            if let Some(doc) = self.documents.get(uri) {
                if is_cl_document(uri, doc) {
                    let total_lines = doc.lines().count() as u32;
                    let last_len = doc.lines().last().map(|l| l.len()).unwrap_or(0) as u32;
                    let full_range = Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: total_lines, character: last_len },
                    };

                    // Action 1: Autonomous Vibe-Loop Heal
                    if let Ok(healed_report) = cronc::heal_cl_program(doc) {
                        let mut changes = HashMap::new();
                        changes.insert(uri.to_string(), vec![TextEdit {
                            range: full_range.clone(),
                            new_text: healed_report.canonical_code,
                        }]);

                        actions.push(CodeAction {
                            title: "Heal .cl Microcode with Autonomous Vibe Loop".to_string(),
                            kind: Some("quickfix".to_string()),
                            diagnostics: None,
                            is_preferred: Some(true),
                            edit: Some(WorkspaceEdit { changes: Some(changes) }),
                        });
                    }

                    // Action 2: DAG Critical-Path Super-Optimizer
                    let config = cronc::ClOptConfig {
                        level: cronc::ClOptLevel::Level2,
                        ..Default::default()
                    };
                    if let Ok(opt_res) = cronc::optimize_cl_program_advanced(doc, &config) {
                        let mut opt_changes = HashMap::new();
                        opt_changes.insert(uri.to_string(), vec![TextEdit {
                            range: full_range,
                            new_text: opt_res.optimized_code,
                        }]);

                        actions.push(CodeAction {
                            title: "Super-Optimize VLIW Microcode (DAG Level 2)".to_string(),
                            kind: Some("refactor.rewrite".to_string()),
                            diagnostics: None,
                            is_preferred: Some(false),
                            edit: Some(WorkspaceEdit { changes: Some(opt_changes) }),
                        });
                    }
                } else {
                    // Action for .cr: Auto-Fix / QuickFix for linear type leaks (E0002/E0003)
                    if let Some(ctx) = p.get("context") {
                        if let Some(diags) = ctx.get("diagnostics").and_then(|d| d.as_array()) {
                            for diag in diags {
                                let code = diag.get("code").and_then(|c| c.as_str()).unwrap_or("");
                                let msg = diag.get("message").and_then(|m| m.as_str()).unwrap_or("");
                                if code == "E0002" || code == "E0003" || msg.contains("Linear variable") {
                                    if let Some(start_quote) = msg.find('\'') {
                                        if let Some(end_quote) = msg[start_quote + 1..].find('\'') {
                                            let var_name = &msg[start_quote + 1..start_quote + 1 + end_quote];
                                            let mut insert_line = doc.lines().count() as u32;
                                            for (idx, line) in doc.lines().enumerate() {
                                                if line.trim().starts_with(".END") {
                                                    insert_line = idx as u32;
                                                    break;
                                                }
                                            }
                                            let mut changes = HashMap::new();
                                            changes.insert(uri.to_string(), vec![TextEdit {
                                                range: Range {
                                                    start: Position { line: insert_line, character: 0 },
                                                    end: Position { line: insert_line, character: 0 },
                                                },
                                                new_text: format!("    consume({})\n", var_name),
                                            }]);

                                            actions.push(CodeAction {
                                                title: format!("Insert 'consume({})' to resolve linear type leak", var_name),
                                                kind: Some("quickfix".to_string()),
                                                diagnostics: None,
                                                is_preferred: Some(true),
                                                edit: Some(WorkspaceEdit { changes: Some(changes) }),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        actions
    }

    fn compute_inlay_hints(&self, params: &Option<serde_json::Value>) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let p = match params {
            Some(p) => p,
            None => return hints,
        };
        let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
        let doc = match self.documents.get(uri) {
            Some(d) => d,
            None => return hints,
        };

        let range_filter = p.get("range").and_then(|r| {
            let start_line = r.get("start")?.get("line")?.as_u64()? as u32;
            let end_line = r.get("end")?.get("line")?.as_u64()? as u32;
            Some((start_line, end_line))
        });

        if is_cl_document(uri, doc) {
            // Inlay hints for .cl (Clean Assembly / VLIW)
            for (line_idx, line) in doc.lines().enumerate() {
                let line_u32 = line_idx as u32;
                if let Some((start_l, end_l)) = range_filter {
                    if line_u32 < start_l || line_u32 > end_l {
                        continue;
                    }
                }
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with(';') || trimmed.starts_with('#') {
                    continue;
                }

                // Check for bundle header: B####:
                if trimmed.len() >= 6
                    && (trimmed.starts_with('B') || trimmed.starts_with('b'))
                    && trimmed[1..5].chars().all(|c| c.is_ascii_digit())
                    && trimmed.chars().nth(5) == Some(':')
                {
                    if let Ok(bundle_num) = trimmed[1..5].parse::<u32>() {
                        if let Some(colon_pos) = line.find(':') {
                            hints.push(InlayHint {
                                position: Position { line: line_u32, character: (colon_pos + 1) as u32 },
                                label: format!(" [C#{}]", bundle_num),
                                kind: Some(2),
                                padding_left: Some(true),
                                padding_right: Some(true),
                                tooltip: Some(format!("VLIW Execution Cycle #{}", bundle_num)),
                            });
                        }
                    }

                    let payload = trimmed[6..].trim();
                    if payload.contains('|') {
                        let slots: Vec<&str> = payload.split('|').collect();
                        let slot_labels = ["ALU", "FMA", "MEM", "NOC"];
                        let mut search_from = 6;
                        for (idx, slot) in slots.iter().enumerate() {
                            let slot_trimmed = slot.trim();
                            if !slot_trimmed.is_empty() {
                                if let Some(found_idx) = line[search_from..].find(slot_trimmed) {
                                    let col = search_from + found_idx;
                                    let label_text = if idx < slot_labels.len() { slot_labels[idx] } else { "SLOT" };
                                    hints.push(InlayHint {
                                        position: Position { line: line_u32, character: col as u32 },
                                        label: format!("[{}] ", label_text),
                                        kind: Some(2),
                                        padding_left: Some(false),
                                        padding_right: Some(true),
                                        tooltip: Some(format!("Port Slot {}: {}", idx, label_text)),
                                    });
                                    search_from = col + slot_trimmed.len();
                                }
                            }
                        }
                    } else {
                        let tokens: Vec<&str> = payload.split_whitespace().collect();
                        let slot_labels = ["ALU", "FMA", "OPT", "NOC"];
                        let mut search_from = 6;
                        for (idx, tok) in tokens.iter().enumerate() {
                            if let Some(found_idx) = line[search_from..].find(tok) {
                                let col = search_from + found_idx;
                                let label_text = if idx < slot_labels.len() { slot_labels[idx] } else { "SLOT" };
                                hints.push(InlayHint {
                                    position: Position { line: line_u32, character: col as u32 },
                                    label: format!("[{}] ", label_text),
                                    kind: Some(2),
                                    padding_left: Some(false),
                                    padding_right: Some(true),
                                    tooltip: Some(format!("Port Slot {}: {}", idx, label_text)),
                                });
                                search_from = col + tok.len();
                            }
                        }
                    }
                }
            }
        } else {
            // Inlay hints for .cr (CRON High-Level Language)
            let collected = cronc::collect_inlay_hints(doc);
            if collected.is_empty() {
                // Line-level fallback for live typing resilience
                for (line_idx, line) in doc.lines().enumerate() {
                    let trimmed = line.trim();
                    if let Some(stripped) = trimmed.strip_prefix("let ") {
                        let rest = stripped.trim();
                        let (is_lin, var_part) = if let Some(s) = rest.strip_prefix("lin ") {
                            (true, s.trim())
                        } else if let Some(s) = rest.strip_prefix("mut ") {
                            (false, s.trim())
                        } else {
                            (false, rest)
                        };

                        if let Some((lhs, rhs)) = var_part.split_once('=') {
                            let var_name = lhs.trim();
                            if !var_name.contains(':') && !var_name.is_empty() {
                                let rhs_trim = rhs.trim();
                                let inferred = if rhs_trim.starts_with('"') {
                                    Some("string")
                                } else if rhs_trim == "true" || rhs_trim == "false" {
                                    Some("bool")
                                } else if rhs_trim.contains('.') && rhs_trim.chars().any(|c| c.is_ascii_digit()) {
                                    Some("f64")
                                } else if rhs_trim.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                                    Some("i64")
                                } else if rhs_trim.starts_with("pack_wave") {
                                    Some("wave_t")
                                } else {
                                    None
                                };

                                let label = match (inferred, is_lin) {
                                    (Some(t), true) => format!(": {} [linear]", t),
                                    (Some(t), false) => format!(": {}", t),
                                    (None, true) => ": [linear]".to_string(),
                                    (None, false) => String::new(),
                                };

                                if !label.is_empty() {
                                    if let Some(var_pos) = line.find(var_name) {
                                        let col = var_pos + var_name.len();
                                        hints.push(InlayHint {
                                            position: Position { line: line_idx as u32, character: col as u32 },
                                            label,
                                            kind: Some(1),
                                            padding_left: Some(true),
                                            padding_right: Some(false),
                                            tooltip: if is_lin {
                                                Some("Linear Affine Resource: Must be consumed exactly once".to_string())
                                            } else {
                                                None
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                for h in collected {
                    let line_u32 = h.line.saturating_sub(1) as u32;
                    let col_u32 = h.col.saturating_sub(1) as u32;
                    if let Some((start_l, end_l)) = range_filter {
                        if line_u32 < start_l || line_u32 > end_l {
                            continue;
                        }
                    }
                    hints.push(InlayHint {
                        position: Position { line: line_u32, character: col_u32 },
                        label: h.label,
                        kind: if h.is_type { Some(1) } else { Some(2) },
                        padding_left: Some(true),
                        padding_right: Some(false),
                        tooltip: h.tooltip,
                    });
                }
            }
        }

        hints
    }

    fn compute_formatting(&self, params: &Option<serde_json::Value>) -> Vec<TextEdit> {
        let mut edits = Vec::new();
        if let Some(p) = params {
            let uri = p.get("textDocument").and_then(|td| td.get("uri")).and_then(|u| u.as_str()).unwrap_or("");
            if let Some(doc) = self.documents.get(uri) {
                if is_cl_document(uri, doc) {
                    let total_lines = doc.lines().count() as u32;
                    let last_len = doc.lines().last().map(|l| l.len()).unwrap_or(0) as u32;
                    if let Ok(healed_report) = cronc::heal_cl_program(doc) {
                        edits.push(TextEdit {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: total_lines, character: last_len },
                            },
                            new_text: healed_report.canonical_code,
                        });
                    }
                }
            }
        }
        edits
    }

    fn format_response(&self, id: Option<serde_json::Value>, result: Option<serde_json::Value>, error: Option<JsonRpcError>) -> String {
        let resp = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result,
            error,
        };
        serde_json::to_string(&resp).unwrap_or_default()
    }
}

pub fn is_math_ident_char(c: char) -> bool {
    c.is_alphanumeric()
        || c == '_'
        || matches!(
            c,
            'α'..='ω'
                | 'Α'..='Ω'
                | '∇'
                | '∑'
                | '√'
                | '∥'
                | '⊗'
        )
}

pub fn extract_word_at_pos(line: &str, character: usize) -> Option<(String, usize, usize)> {
    if line.is_empty() {
        return None;
    }
    let char_indices: Vec<(usize, char)> = line.char_indices().collect();
    if char_indices.is_empty() {
        return None;
    }

    let mut target_idx = None;
    for (i, &(byte_offset, c)) in char_indices.iter().enumerate() {
        let char_len = c.len_utf8();
        if character >= byte_offset && character < byte_offset + char_len {
            target_idx = Some(i);
            break;
        }
    }

    let pos = match target_idx {
        Some(i) => {
            if is_math_ident_char(char_indices[i].1) {
                i
            } else if i > 0 && is_math_ident_char(char_indices[i - 1].1) && character == char_indices[i].0 {
                i - 1
            } else {
                return None;
            }
        }
        None => {
            if character >= line.len() && !char_indices.is_empty() && is_math_ident_char(char_indices.last().unwrap().1) {
                char_indices.len() - 1
            } else {
                return None;
            }
        }
    };

    let mut start = pos;
    while start > 0 && is_math_ident_char(char_indices[start - 1].1) {
        start -= 1;
    }

    let mut end = pos + 1;
    while end < char_indices.len() && is_math_ident_char(char_indices[end].1) {
        end += 1;
    }

    let start_byte = char_indices[start].0;
    let end_byte = if end < char_indices.len() {
        char_indices[end].0
    } else {
        line.len()
    };

    if start_byte < end_byte {
        Some((line[start_byte..end_byte].to_string(), start_byte, end_byte))
    } else {
        None
    }
}

// === Standard IO Server Loop ===

pub fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = std::io::stdout();
    let mut server = LspServer::new();

    loop {
        if server.is_shutdown {
            break;
        }

        let mut content_length: usize = 0;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Ok(()); // EOF
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }
            if let Some((k, v)) = trimmed.split_once(':') {
                if k.trim().eq_ignore_ascii_case("content-length") {
                    content_length = v.trim().parse().unwrap_or(0);
                }
            }
        }

        if content_length == 0 {
            continue;
        }

        let mut body_bytes = vec![0u8; content_length];
        reader.read_exact(&mut body_bytes)?;
        let body_str = String::from_utf8_lossy(&body_bytes);

        if let Some(resp_json) = server.handle_message(&body_str) {
            let header = format!("Content-Length: {}\r\n\r\n", resp_json.len());
            stdout.write_all(header.as_bytes())?;
            stdout.write_all(resp_json.as_bytes())?;
            stdout.flush()?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_initialize_and_capabilities() {
        let mut server = LspServer::new();
        let init_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });

        let resp_str = server.handle_message(&init_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(resp["id"], 1);
        assert_eq!(resp["result"]["capabilities"]["textDocumentSync"], 1);
        assert_eq!(resp["result"]["capabilities"]["hoverProvider"], true);
        assert_eq!(resp["result"]["capabilities"]["codeActionProvider"], true);
        assert_eq!(resp["result"]["capabilities"]["documentFormattingProvider"], true);
        assert_eq!(resp["result"]["capabilities"]["inlayHintProvider"], true);
    }

    #[test]
    fn test_lsp_cr_diagnostics() {
        let mut server = LspServer::new();
        let code = r#"
        .MODULE LeakModule
        _main:
            let lin unconsumed_photon = 100
        .END
        "#;

        let open_req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///workspace/test.cr",
                    "text": code
                }
            }
        });

        let resp_str = server.handle_message(&open_req.to_string()).unwrap();
        let notify: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(notify["method"], "textDocument/publishDiagnostics");
        let diags = notify["params"]["diagnostics"].as_array().unwrap();
        assert!(!diags.is_empty());
        assert_eq!(diags[0]["code"], "E0002");
        assert!(diags[0]["message"].as_str().unwrap().contains("Linear variable 'unconsumed_photon' was allocated but never consumed"));
    }

    #[test]
    fn test_lsp_cr_multi_error_diagnostics() {
        let mut server = LspServer::new();
        let code = r#"
        .MODULE MultiLeak
        _main:
            let a = 1
            a = 2
            let lin leak1 = 10
            let lin leak2 = 20
        .END
        "#;

        let open_req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///workspace/multi_err.cr",
                    "text": code
                }
            }
        });

        let resp_str = server.handle_message(&open_req.to_string()).unwrap();
        let notify: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(notify["method"], "textDocument/publishDiagnostics");
        let diags = notify["params"]["diagnostics"].as_array().unwrap();
        // Multi-error diagnostics: all errors in the file are published in a single pass!
        assert!(diags.len() >= 2, "Expected multiple diagnostics, got: {}", diags.len());
        assert!(diags.iter().any(|d| d["code"] == "E0005"));
        assert!(diags.iter().any(|d| d["code"] == "E0002"));
    }

    #[test]
    fn test_lsp_cl_diagnostics() {
        let mut server = LspServer::new();
        let bad_cl = r#"
        @CORE(0,0,0
        B0000: LOAD [R1+0] | NOP | NOP | NOP
        B0001: ADD R20, R1, R2 | NOP | LOAD [R1] | ROUTE_DOR +X
        B0002: NOP | NOP
        "#;

        let open_req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///workspace/kernel.cl",
                    "text": bad_cl
                }
            }
        });

        let resp_str = server.handle_message(&open_req.to_string()).unwrap();
        let notify: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(notify["method"], "textDocument/publishDiagnostics");
        let diags = notify["params"]["diagnostics"].as_array().unwrap();

        // Should flag malformed @CORE
        assert!(diags.iter().any(|d| d["code"] == "CL001" && d["message"].as_str().unwrap().contains("@CORE")));
        // Should flag memory opcode in ALU0 slot
        assert!(diags.iter().any(|d| d["code"] == "CL003" && d["message"].as_str().unwrap().contains("Memory opcode 'LOAD'")));
        // Should flag R20 exceeding R15
        assert!(diags.iter().any(|d| d["code"] == "CL004" && d["message"].as_str().unwrap().contains("R20")));
        // Should flag invalid slot count (2 slots instead of 4)
        assert!(diags.iter().any(|d| d["code"] == "CL001" && d["message"].as_str().unwrap().contains("found 2 slots")));
    }

    #[test]
    fn test_lsp_cl_topological_diagnostics() {
        let mut server = LspServer::new();
        let valid_topo_cl = r#"
        bundle InitManifold {
            S0: WRAP W=0 X=0 Z=0 spin=0 vol=25;
            S1: WRAP W=1 X=25 Z=50 spin=1 vol=25;
            S2: WRAP W=2 X=50 Z=25 spin=2 vol=25;
            S3: WRAP W=3 X=75 Z=75 spin=3 vol=25;
        }
        "#;

        let open_req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///workspace/topological_rotor.cl",
                    "text": valid_topo_cl
                }
            }
        });

        let resp_str = server.handle_message(&open_req.to_string()).unwrap();
        let notify: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(notify["method"], "textDocument/publishDiagnostics");
        let diags = notify["params"]["diagnostics"].as_array().unwrap();
        assert_eq!(diags.len(), 0, "Valid topological CL should produce zero diagnostics, got: {:?}", diags);

        // Test invalid invariant (Volume sum != 100)
        let invalid_topo_cl = r#"
        bundle BadVolume {
            S0: WRAP W=0 X=0 Z=0 spin=0 vol=20;
            S1: WRAP W=1 X=25 Z=50 spin=1 vol=20;
            S2: WRAP W=2 X=50 Z=25 spin=2 vol=20;
            S3: WRAP W=3 X=75 Z=75 spin=3 vol=20;
        }
        "#;
        let open_bad = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///workspace/bad_rotor.cl",
                    "text": invalid_topo_cl
                }
            }
        });
        let resp_bad = server.handle_message(&open_bad.to_string()).unwrap();
        let notify_bad: serde_json::Value = serde_json::from_str(&resp_bad).unwrap();
        let bad_diags = notify_bad["params"]["diagnostics"].as_array().unwrap();
        assert!(bad_diags.iter().any(|d| d["code"] == "TOPO_INV"));
    }

    #[test]
    fn test_lsp_cl_hover_silicon_metrics() {
        let mut server = LspServer::new();
        let cl_code = "B0000: DOT_I2 R0, R1, R2 | NOP | LOAD [R3+0] | ROUTE_DOR +X\n";
        server.documents.insert("file:///kernel.cl".to_string(), cl_code.to_string());

        // Hover over DOT_I2
        let hover_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": "file:///kernel.cl" },
                "position": { "line": 0, "character": 9 }
            }
        });

        let resp_str = server.handle_message(&hover_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let val = resp["result"]["contents"]["value"].as_str().unwrap();
        assert!(val.contains("DOT_I2"));
        assert!(val.contains("BitNet 1.58b Ternary Dot Product"));
        assert!(val.contains("0.08 pJ"));

        // Hover over R3
        let hover_r3 = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 11,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": "file:///kernel.cl" },
                "position": { "line": 0, "character": 40 }
            }
        });
        let resp_r3_str = server.handle_message(&hover_r3.to_string()).unwrap();
        let resp_r3: serde_json::Value = serde_json::from_str(&resp_r3_str).unwrap();
        let r3_val = resp_r3["result"]["contents"]["value"].as_str().unwrap();
        assert!(r3_val.contains("Register `R3`"));
        assert!(r3_val.contains("32-bit General-Purpose Architectural Register File"));
    }

    #[test]
    fn test_lsp_cl_slot_aware_completions() {
        let mut server = LspServer::new();
        let line = "B0000: ADD R0, R1, R2 | NOP | LOAD [R3] | ROUTE_DOR +X";
        server.documents.insert("file:///test.cl".to_string(), line.to_string());

        // Cursor at column 8 (in ALU0 slot)
        let comp_alu0 = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 20,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": "file:///test.cl" },
                "position": { "line": 0, "character": 8 }
            }
        });
        let resp_str = server.handle_message(&comp_alu0.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let items = resp["result"]["items"].as_array().unwrap();
        assert!(items.iter().any(|i| i["label"] == "DOT_I2"));
        assert!(items.iter().any(|i| i["label"] == "ATTN_STEP"));
        assert!(items.iter().any(|i| i["label"] == "LIF_STEP"));

        // Cursor at column 35 (in MEM slot)
        let comp_mem = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 21,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": "file:///test.cl" },
                "position": { "line": 0, "character": 35 }
            }
        });
        let resp_mem_str = server.handle_message(&comp_mem.to_string()).unwrap();
        let resp_mem: serde_json::Value = serde_json::from_str(&resp_mem_str).unwrap();
        let mem_items = resp_mem["result"]["items"].as_array().unwrap();
        assert!(mem_items.iter().any(|i| i["label"] == "LOAD"));
        assert!(mem_items.iter().any(|i| i["label"] == "SWIZZLE"));
        assert!(mem_items.iter().any(|i| i["label"] == "PREFETCH"));

        // Cursor at column 48 (in NOC slot)
        let comp_noc = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 22,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": "file:///test.cl" },
                "position": { "line": 0, "character": 48 }
            }
        });
        let resp_noc_str = server.handle_message(&comp_noc.to_string()).unwrap();
        let resp_noc: serde_json::Value = serde_json::from_str(&resp_noc_str).unwrap();
        let noc_items = resp_noc["result"]["items"].as_array().unwrap();
        assert!(noc_items.iter().any(|i| i["label"] == "ROUTE_DOR"));
        assert!(noc_items.iter().any(|i| i["label"] == "COLLECTIVE"));
    }

    #[test]
    fn test_lsp_cl_zero_word_math_completions() {
        let mut server = LspServer::new();
        let math_line = "F(X[B, D]) = ";
        server.documents.insert("file:///math_model.cl".to_string(), math_line.to_string());

        let comp_math = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 25,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": "file:///math_model.cl" },
                "position": { "line": 0, "character": 13 }
            }
        });
        let resp_str = server.handle_message(&comp_math.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let items = resp["result"]["items"].as_array().unwrap();

        // Verify Zero-Word Canonical Math Operators are present
        assert!(items.iter().any(|i| i["label"] == "Ψ_g"));
        assert!(items.iter().any(|i| i["label"] == "Ψ"));
        assert!(items.iter().any(|i| i["label"] == "N"));
        assert!(items.iter().any(|i| i["label"] == "S"));
        assert!(items.iter().any(|i| i["label"] == "B"));
        assert!(items.iter().any(|i| i["label"] == "ρ"));
        assert!(items.iter().any(|i| i["label"] == "Φ"));
        assert!(items.iter().any(|i| i["label"] == "∇"));
        assert!(items.iter().any(|i| i["label"] == "Δ"));
        assert!(items.iter().any(|i| i["label"] == "ι"));
        assert!(items.iter().any(|i| i["label"] == "<<"));
        assert!(items.iter().any(|i| i["label"] == "@"));
    }

    #[test]
    fn test_lsp_cl_code_action_and_formatting() {
        let mut server = LspServer::new();
        let raw_code = "B0000: _AD00$000> _NO00$000> _LD00#010> _RO00@000>\n";
        server.documents.insert("file:///prog.cl".to_string(), raw_code.to_string());

        // Request Code Actions
        let action_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 30,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": { "uri": "file:///prog.cl" },
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 0, "character": 10 }
                },
                "context": { "diagnostics": [] }
            }
        });
        let resp_str = server.handle_message(&action_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let actions = resp["result"].as_array().unwrap();
        assert!(actions.iter().any(|a| a["title"].as_str().unwrap().contains("Heal .cl Microcode")));
        assert!(actions.iter().any(|a| a["title"].as_str().unwrap().contains("Super-Optimize VLIW")));

        // Request Formatting
        let format_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 31,
            "method": "textDocument/formatting",
            "params": {
                "textDocument": { "uri": "file:///prog.cl" },
                "options": { "tabSize": 4, "insertSpaces": true }
            }
        });
        let f_resp_str = server.handle_message(&format_req.to_string()).unwrap();
        let f_resp: serde_json::Value = serde_json::from_str(&f_resp_str).unwrap();
        let edits = f_resp["result"].as_array().unwrap();
        assert_eq!(edits.len(), 1);
        let new_text = edits[0].get("newText").or_else(|| edits[0].get("new_text")).and_then(|v| v.as_str()).unwrap();
        assert!(new_text.contains("B0000:"));
    }

    #[test]
    fn test_lsp_initialize_new_capabilities() {
        // Verify the 3 newly declared capabilities are present
        let mut server = LspServer::new();
        let init_req = serde_json::json!({
            "jsonrpc": "2.0", "id": 1,
            "method": "initialize", "params": {}
        });
        let resp_str = server.handle_message(&init_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let caps = &resp["result"]["capabilities"];
        // signatureHelp trigger chars
        let triggers = caps["signatureHelpProvider"]["triggerCharacters"].as_array().unwrap();
        assert!(triggers.iter().any(|t| t == "("));
        assert!(triggers.iter().any(|t| t == ","));
        // definition, documentSymbol, and semanticTokens
        assert_eq!(caps["definitionProvider"], true);
        assert_eq!(caps["documentSymbolProvider"], true);
        assert_eq!(caps["inlayHintProvider"], true);
        assert_eq!(caps["semanticTokensProvider"]["full"], true);
        // version updated
        assert_eq!(resp["result"]["serverInfo"]["version"], "3.17.1");
    }

    #[test]
    fn test_lsp_signature_help_pack_wave() {
        let mut server = LspServer::new();
        let cr_code = "let w = pack_wave(amp=[1.0, 0.5], phase=[0.0, 1.57])\n";
        server.documents.insert("file:///test.cr".to_string(), cr_code.to_string());

        // Cursor after '(' — active parameter = 0 (amp)
        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 40,
            "method": "textDocument/signatureHelp",
            "params": {
                "textDocument": { "uri": "file:///test.cr" },
                "position": { "line": 0, "character": 19 }
            }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let sigs = resp["result"]["signatures"].as_array().unwrap();
        assert!(!sigs.is_empty());
        assert!(sigs[0]["label"].as_str().unwrap().contains("pack_wave"));
        assert!(sigs[0]["label"].as_str().unwrap().contains("wave_t"));
        assert_eq!(resp["result"]["activeSignature"], 0);
        assert_eq!(resp["result"]["activeParameter"], 0);
        // Verify parameter documentation present
        let params = sigs[0]["parameters"].as_array().unwrap();
        assert!(params.iter().any(|p| p["label"] == "amp"));
        assert!(params.iter().any(|p| p["label"] == "phase"));
    }

    #[test]
    fn test_lsp_signature_help_active_parameter() {
        // After one comma, activeParameter should advance to 1
        let mut server = LspServer::new();
        // Cursor is after comma: "compute_attention_head(q, "
        let cr_code = "let x = compute_attention_head(q, k)\n";
        server.documents.insert("file:///attn.cr".to_string(), cr_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 41,
            "method": "textDocument/signatureHelp",
            "params": {
                "textDocument": { "uri": "file:///attn.cr" },
                "position": { "line": 0, "character": 33 }  // after 'q, '
            }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(resp["result"]["activeParameter"], 1);
        let sigs = resp["result"]["signatures"].as_array().unwrap();
        assert!(sigs[0]["label"].as_str().unwrap().contains("compute_attention_head"));
    }

    #[test]
    fn test_lsp_document_symbols_cr() {
        let mut server = LspServer::new();
        let cr_code = concat!(
            "def compute_loss(x: f32, y: f32) -> f32 { }\n",
            "struct WeightMatrix { data: tensor }\n",
            "trait Trainable { def step(self); }\n",
            "brain AttentionEngine [cores=64] { }\n",
            "region Workspace [target=SELF] { }\n",
            "impl Trainable for WeightMatrix { }\n",
        );
        server.documents.insert("file:///model.cr".to_string(), cr_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 50,
            "method": "textDocument/documentSymbol",
            "params": { "textDocument": { "uri": "file:///model.cr" } }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let syms = resp["result"].as_array().unwrap();

        // Check function symbol
        assert!(syms.iter().any(|s| s["name"] == "compute_loss" && s["kind"] == 12));
        // Check struct symbol
        assert!(syms.iter().any(|s| s["name"] == "WeightMatrix" && s["kind"] == 23));
        // Check trait symbol
        assert!(syms.iter().any(|s| s["name"] == "Trainable" && s["kind"] == 11));
        // Check brain symbol (kind=5 Class)
        assert!(syms.iter().any(|s| s["name"] == "AttentionEngine" && s["kind"] == 5));
        // Check region symbol
        assert!(syms.iter().any(|s| s["name"] == "Workspace" && s["kind"] == 14));
        // Check impl symbol
        assert!(syms.iter().any(|s| s["name"].as_str().unwrap().contains("Trainable for WeightMatrix") && s["kind"] == 11));
    }

    #[test]
    fn test_lsp_document_symbols_cl() {
        let mut server = LspServer::new();
        let cl_code = concat!(
            "@CORE(0,0,0,0)\n",
            "B0000: ADD R0, R1, R2 | NOP | LOAD [R3] | NOP\n",
            "B0001: MUL R4, R0, R2 | NOP | NOP | SEND R7\n",
        );
        server.documents.insert("file:///kernel.cl".to_string(), cl_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 51,
            "method": "textDocument/documentSymbol",
            "params": { "textDocument": { "uri": "file:///kernel.cl" } }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let syms = resp["result"].as_array().unwrap();

        // @CORE should appear as Class (kind=5)
        assert!(syms.iter().any(|s| s["name"].as_str().unwrap().starts_with("@CORE") && s["kind"] == 5));
        // B0000 and B0001 as Function (kind=12)
        assert!(syms.iter().any(|s| s["name"] == "B0000" && s["kind"] == 12));
        assert!(syms.iter().any(|s| s["name"] == "B0001" && s["kind"] == 12));
    }

    #[test]
    fn test_lsp_goto_definition_function() {
        let mut server = LspServer::new();
        let cr_code = concat!(
            "def forward_pass(x: tensor) -> tensor { }\n",
            "let result = forward_pass(input)\n",
        );
        server.documents.insert("file:///net.cr".to_string(), cr_code.to_string());

        // Hover cursor over 'forward_pass' on line 1 (the call site)
        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 60,
            "method": "textDocument/definition",
            "params": {
                "textDocument": { "uri": "file:///net.cr" },
                "position": { "line": 1, "character": 15 }  // inside 'forward_pass'
            }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let locs = resp["result"].as_array().unwrap();

        // Should find definition on line 0
        assert!(!locs.is_empty());
        assert_eq!(locs[0]["uri"], "file:///net.cr");
        assert_eq!(locs[0]["range"]["start"]["line"], 0);
    }

    #[test]
    fn test_lsp_semantic_tokens_cr() {
        let mut server = LspServer::new();
        let cr_code = "let lin photon = pack_wave(amp=[1.0, 0.5])\n";
        server.documents.insert("file:///wave.cr".to_string(), cr_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 70,
            "method": "textDocument/semanticTokens/full",
            "params": {
                "textDocument": { "uri": "file:///wave.cr" }
            }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let data = resp["result"]["data"].as_array().unwrap();
        // Semantic tokens are 5-tuples [deltaLine, deltaCol, len, tokenType, tokenModifiers]
        assert_eq!(data.len() % 5, 0);
        assert!(!data.is_empty());

        // Find the 'lin' token: tokenType == 0 (keyword), tokenModifiers == 2 (readonly)
        let mut found_lin = false;
        let mut found_pack_wave = false;
        for chunk in data.chunks(5) {
            let len = chunk[2].as_u64().unwrap();
            let token_type = chunk[3].as_u64().unwrap();
            let modifiers = chunk[4].as_u64().unwrap();
            if len == 3 && token_type == 0 && modifiers == 2 {
                found_lin = true;
            }
            // 'pack_wave' has len 9, token_type 2 (function), modifiers 8 (defaultLibrary)
            if len == 9 && token_type == 2 && modifiers == 8 {
                found_pack_wave = true;
            }
        }
        assert!(found_lin, "lin token not properly identified with readonly modifier");
        assert!(found_pack_wave, "pack_wave builtin function not properly identified");
    }

    #[test]
    fn test_lsp_semantic_tokens_cl() {
        let mut server = LspServer::new();
        let cl_code = concat!(
            "@CORE(0,0,0,0)\n",
            "B0000: ADD R0, R1, R2 | NOP | LOAD [R3] | NOP ; test comment\n"
        );
        server.documents.insert("file:///core.cl".to_string(), cl_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0", "id": 71,
            "method": "textDocument/semanticTokens/full",
            "params": {
                "textDocument": { "uri": "file:///core.cl" }
            }
        });
        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let data = resp["result"]["data"].as_array().unwrap();
        assert_eq!(data.len() % 5, 0);
        assert!(!data.is_empty());

        let mut found_core = false;
        let mut found_bundle = false;
        let mut found_add = false;
        let mut found_reg = false;
        for chunk in data.chunks(5) {
            let len = chunk[2].as_u64().unwrap();
            let token_type = chunk[3].as_u64().unwrap();
            let modifiers = chunk[4].as_u64().unwrap();
            if len == 5 && token_type == 5 {
                found_core = true; // @CORE
            }
            if len == 5 && token_type == 8 {
                found_bundle = true; // B0000
            }
            if len == 3 && token_type == 9 {
                found_add = true; // ADD (macro)
            }
            if len == 2 && token_type == 3 && modifiers == 2 {
                found_reg = true; // R0..R2 (variable, readonly)
            }
        }
        assert!(found_core, "@CORE directive should be classified as class");
        assert!(found_bundle, "B0000 should be classified as label");
        assert!(found_add, "ADD should be classified as macro");
        assert!(found_reg, "R0..R2 registers should be classified as readonly variables");
    }

    #[test]
    fn test_lsp_cr_inlay_hints() {
        let mut server = LspServer::new();
        let code = r#"
        .MODULE InlayDemo
        _main:
            let a = 10
            let lin photon = pack_wave()
            consume(photon)
        .END
        "#;
        server.documents.insert("file:///workspace/demo.cr".to_string(), code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 99,
            "method": "textDocument/inlayHint",
            "params": {
                "textDocument": { "uri": "file:///workspace/demo.cr" },
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 10, "character": 0 }
                }
            }
        });

        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let hints = resp["result"].as_array().unwrap();
        assert!(!hints.is_empty(), "Inlay hints should be computed for .cr");
        assert!(hints.iter().any(|h| h["label"] == ": i64"), "Expected : i64 inlay hint for 'a'");
        assert!(hints.iter().any(|h| h["label"].as_str().unwrap().contains("wave_t") && h["label"].as_str().unwrap().contains("[linear]")), "Expected wave_t [linear] hint");
    }

    #[test]
    fn test_lsp_cl_inlay_hints() {
        let mut server = LspServer::new();
        let cl_code = "B0001: ADD R1, R2, R3 | FMA V0, V1, V2 | LOAD R4, [R5] | SEND R6\n";
        server.documents.insert("file:///workspace/test.cl".to_string(), cl_code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 100,
            "method": "textDocument/inlayHint",
            "params": {
                "textDocument": { "uri": "file:///workspace/test.cl" }
            }
        });

        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let hints = resp["result"].as_array().unwrap();
        assert!(!hints.is_empty(), "Inlay hints should be computed for .cl");
        assert!(hints.iter().any(|h| h["label"].as_str().unwrap().contains("[C#1]")), "Expected cycle hint [C#1]");
        assert!(hints.iter().any(|h| h["label"].as_str().unwrap().contains("[ALU]")), "Expected [ALU] slot hint");
        assert!(hints.iter().any(|h| h["label"].as_str().unwrap().contains("[FMA]")), "Expected [FMA] slot hint");
    }

    #[test]
    fn test_lsp_cr_code_action_consume_linear() {
        let mut server = LspServer::new();
        let code = r#"
        .MODULE ActionDemo
        _main:
            let lin orphan_photon = 42
        .END
        "#;
        server.documents.insert("file:///workspace/action.cr".to_string(), code.to_string());

        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 101,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": { "uri": "file:///workspace/action.cr" },
                "range": { "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 30 } },
                "context": {
                    "diagnostics": [
                        {
                            "code": "E0002",
                            "message": "Linear type leak: Linear variable 'orphan_photon' was allocated but never consumed or exported",
                            "range": { "start": { "line": 3, "character": 12 }, "end": { "line": 3, "character": 25 } }
                        }
                    ]
                }
            }
        });

        let resp_str = server.handle_message(&req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let actions = resp["result"].as_array().unwrap();
        assert!(!actions.is_empty(), "Expected code action for unconsumed linear resource");
        assert!(actions[0]["title"].as_str().unwrap().contains("consume(orphan_photon)"));
        assert_eq!(actions[0]["kind"], "quickfix");
    }

    #[test]
    fn test_lsp_sovereign_cl_v2_clean_diagnostics() {
        let server = LspServer::new();
        let v2_source = r#"
@kernel test_v2_clean
.target silicon.frontier_models
.ipc_target 4.0
.cores 256
@mesh [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  X[16×16:FP32]  @sram[0x0000]
    out: Y[16×16:FP32]  @sram[0x0400]
}

§CleanBlock[16×16:FP32] {
    W: T[16×16:FP32]
    X ↦ Y :
        Norm := rmsnorm(X)
        Y := silu(Norm)
}

§ControlFlow {
    ⟳ (16) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0040]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Y| < 1000, latency < 15ns }
"#;
        let diags = server.compute_diagnostics_for_source("file:///frontier.cl", v2_source);
        assert!(diags.is_empty(), "Sovereign CL v2 should produce 0 false diagnostic errors, found: {:?}", diags);
    }

    #[test]
    fn test_phase11_lsp_math_hover_and_diagnostics() {
        let math_code = r#"
            W = [[1.0, 2.0], [3.0, 4.0]]
            forward(X: f32[B, D], W: f32[D, M]) = X @ W
        "#;

        // 1. Test Hover on Tensor Variable W
        let w_hover = get_math_hover_documentation(math_code, "W").expect("Hover for W");
        assert!(w_hover.contains("CRON Tensor Variable: `W`"));
        assert!(w_hover.contains("[2, 2]"));
        assert!(w_hover.contains("16 bytes"));

        // 2. Test Hover on Function forward
        let fn_hover = get_math_hover_documentation(math_code, "forward").expect("Hover for forward");
        assert!(fn_hover.contains("CRON Mathematical Function: `forward`"));
        assert!(fn_hover.contains("X: f32[B, D]"));
        assert!(fn_hover.contains("Symbolic Contract"));
        assert!(fn_hover.contains("void cron_forward"));

        // 3. Test Hover on Built-in Operators
        let kv_hover = get_math_hover_documentation(math_code, "kv_cache_attn").expect("Hover for kv_cache_attn");
        assert!(kv_hover.contains("Streaming Attention"));
        assert!(kv_hover.contains("O(T"));

        let norm_hover = get_math_hover_documentation(math_code, "norm").expect("Hover for norm");
        assert!(norm_hover.contains("Frobenius Norm"));

        // 4. Test Valid Diagnostics
        let valid_diags = check_cl_diagnostics(math_code);
        assert!(valid_diags.is_empty(), "Valid math code should produce 0 diagnostics, found: {:?}", valid_diags);

        // 5. Test Invalid Symbolic Dimension Mismatch Diagnostics
        let invalid_sym_code = r#"
            bad_forward(X: f32[B, D1], W: f32[D2, M]) = X @ W
        "#;
        let sym_diags = check_cl_diagnostics(invalid_sym_code);
        assert!(!sym_diags.is_empty(), "Expected symbolic mismatch diagnostic");
        assert!(sym_diags[0].message.contains("Symbolic Dimension Mismatch"));
        assert!(sym_diags[0].message.contains("D1"));
        assert!(sym_diags[0].message.contains("D2"));
    }

    #[test]
    fn test_phase19_greek_unicode_math_hover_and_word_extraction() {
        let code = r#"
            s0 = "AI"
            t = ι(s0)
            H0 = E[t]
            γ_1 = [1.0, 1.0]
            H_norm = N(H0, γ_1)
            H_rope = Φ(H_norm, 0.0)
            H_attn = Ψ(Q, K, V)
            (y_s, K_s, V_s) = Ψ_c(q, k, v, K, V, 0.5)
            L = Λ(H_attn, y)
            gW = ∇(L, W)
            M_out = a ⊗ b
            s1 = δ(toks)
            tok = χ(logits, 0.8, 0.95, 42)
            W0 = O(4, 4)
            Id = I(4)
        "#;
        // Verify word extraction on Greek/Unicode characters
        let line_attn = "            H_attn = Ψ(Q, K, V)";
        let char_pos = line_attn.find('Ψ').expect("Find Psi");
        let (word, _, _) = extract_word_at_pos(line_attn, char_pos).expect("Extract Psi");
        assert_eq!(word, "Ψ");

        let line_kv = "            (y_s, K_s, V_s) = Ψ_c(q, k, v, K, V, 0.5)";
        let char_pos_kv = line_kv.find('Ψ').expect("Find Psi_c");
        let (word_kv, _, _) = extract_word_at_pos(line_kv, char_pos_kv).expect("Extract Psi_c");
        assert_eq!(word_kv, "Ψ_c");

        let line_grad = "            gW = ∇(L, W)";
        let char_pos_grad = line_grad.find('∇').expect("Find Nabla");
        let (word_grad, _, _) = extract_word_at_pos(line_grad, char_pos_grad).expect("Extract Nabla");
        assert_eq!(word_grad, "∇");

        // Verify Hover documentation
        let hover_psi = get_math_hover_documentation(code, "Ψ").expect("Hover for Ψ");
        assert!(hover_psi.contains("FlashAttention"));

        let hover_psic = get_math_hover_documentation(code, "Ψ_c").expect("Hover for Ψ_c");
        assert!(hover_psic.contains("Streaming KV-Attention"));

        let hover_grad = get_math_hover_documentation(code, "∇").expect("Hover for ∇");
        assert!(hover_grad.contains("Analytical Matrix Derivative"));

        let hover_otimes = get_math_hover_documentation(code, "⊗").expect("Hover for ⊗");
        assert!(hover_otimes.contains("Tensor Outer Product"));

        let hover_lambda = get_math_hover_documentation(code, "Λ").expect("Hover for Λ");
        assert!(hover_lambda.contains("Cross-Entropy Loss"));
    }
}
