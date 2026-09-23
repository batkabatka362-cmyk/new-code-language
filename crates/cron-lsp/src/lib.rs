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
pub struct CompletionItem {
    pub label: String,
    pub kind: u32, // 1=Text, 3=Function, 7=Class, 14=Keyword, 21=Constant, etc.
    pub detail: Option<String>,
    pub documentation: Option<MarkupContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// === Language Identification ===

pub fn is_cl_document(uri: &str, text: &str) -> bool {
    if uri.ends_with(".cl") {
        return true;
    }
    text.lines().any(|l| {
        let t = l.trim();
        t.starts_with("@CORE")
            || t.starts_with("B00")
            || (t.contains('|') && (t.contains('_') || t.contains("ALU") || t.contains("NOP") || t.contains("ADD")))
    })
}

// === Machine Code (.cl) Real-Time Diagnostics ===

pub fn check_cl_diagnostics(source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        let line_u32 = line_idx as u32;

        // Check @CORE directive
        if trimmed.starts_with("@CORE") {
            if !trimmed.starts_with("@CORE(") || !trimmed.ends_with(')') {
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

        // Check bundle lines with slot separators '|'
        if trimmed.contains('|') {
            let parts: Vec<&str> = trimmed.split('|').collect();
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
                let slot_names = ["ALU0", "ALU1", "MEM", "NOC"];
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

                    // Check for invalid registers like R16..R99
                    for token in p_trim.split(|c: char| !c.is_alphanumeric()) {
                        if token.starts_with('R') || token.starts_with('r') {
                            if let Ok(reg_num) = token[1..].parse::<u32>() {
                                if reg_num > 15 {
                                    diagnostics.push(Diagnostic {
                                        range: Range {
                                            start: Position { line: line_u32, character: 0 },
                                            end: Position { line: line_u32, character: line.len() as u32 },
                                        },
                                        severity: 1,
                                        code: Some("CL004".to_string()),
                                        source: Some("cron-lsp".to_string()),
                                        message: format!(
                                            "Register 'R{}' in {} slot exceeds 4-bit hardware limit (valid: R0..R15)",
                                            reg_num, slot_names[slot_idx]
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
                                end: Position { line: line_u32, character: line.len() as u32 },
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
                                end: Position { line: line_u32, character: line.len() as u32 },
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
                                end: Position { line: line_u32, character: line.len() as u32 },
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

                    // Check slot width warning (CL002) for raw slots (e.g. '_AD00$000>')
                    if p_trim.starts_with('_') && p_trim.ends_with('>') && p_trim.len() != 10 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_u32, character: 0 },
                                end: Position { line: line_u32, character: line.len() as u32 },
                            },
                            severity: 2, // Warning
                            code: Some("CL002".to_string()),
                            source: Some("cron-lsp".to_string()),
                            message: format!(
                                "Raw slot '{}' length is {} chars. Canonical fixed slot width is 10 chars (_XXYY#ZZZ>)",
                                p_trim, p_trim.len()
                            ),
                        });
                    }
                }
            }
        } else if !trimmed.starts_with("B0") && !trimmed.starts_with('_') && !trimmed.starts_with('@') && !trimmed.starts_with('.') {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line_u32, character: 0 },
                    end: Position { line: line_u32, character: line.len() as u32 },
                },
                severity: 1,
                code: Some("CL001".to_string()),
                source: Some("cron-lsp".to_string()),
                message: format!("Unrecognized statement in .cl microcode: '{}'. Expected 4-slot VLIW bundle", trimmed),
            });
        }
    }

    diagnostics
}

// === Silicon Micro-Architecture Hover Tooltips (.cl & .cr) ===

pub fn get_cl_hover_documentation(word: &str) -> Option<String> {
    let upper = word.to_ascii_uppercase();

    // Check register R0..R15
    if (upper.starts_with('R') || upper.starts_with("REG")) && upper.len() >= 2 {
        let num_part = upper.strip_prefix("REG").unwrap_or_else(|| &upper[1..]);
        if let Ok(reg_idx) = num_part.parse::<u32>() {
            if reg_idx <= 15 {
                return Some(format!(
                    "### Register `R{}` (CRON 256-Core 4D-Torus)\n\n\
                    - **Register Bank**: 32-bit General-Purpose Architectural Register File\n\
                    - **Addressable Width**: 4-bit Index (0x{:X})\n\
                    - **Read Ports**: 2 read ports per execution ALU\n\
                    - **Write Ports**: 1 dedicated write port per cycle\n\
                    - **Silicon Hazard**: 0-cycle forwarding bypass network enabled",
                    reg_idx, reg_idx
                ));
            }
        }
    }

    // Check bundle header B0000..B9999
    if upper.starts_with('B') && upper.len() == 5 && upper[1..].chars().all(|c| c.is_ascii_digit()) {
        let cycle: u32 = upper[1..].parse().unwrap_or(0);
        return Some(format!(
            "### VLIW Microcode Bundle `{}`\n\n\
            - **Target Execution Cycle**: Cycle {}\n\
            - **Parallel Issue**: 4 independent execution slots `[ALU0 | ALU1 | MEM | NOC]`\n\
            - **Theoretical Peak**: 4.0 Instructions Per Cycle (IPC)\n\
            - **Instruction Width**: 94 characters (canonical 4-slot fixed alignment)",
            upper, cycle
        ));
    }

    match upper.as_str() {
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
        _ => None,
    }
}

// === Slot-Aware Autocompletion for .cl ===

pub fn get_cl_completion_items(line_str: &str, character: usize) -> Vec<CompletionItem> {
    let mut items = Vec::new();

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

// === High-Level Language (.cr) Completions ===

pub fn get_completion_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Keywords
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
        ("import", "import { Symbol } from \"path.cr\"", "Import module symbol"),
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
                        "textDocumentSync": 1, // Full document sync
                        "hoverProvider": true,
                        "completionProvider": {
                            "resolveProvider": false,
                            "triggerCharacters": [".", ":", "$", "@", " ", "_", "R"]
                        },
                        "codeActionProvider": true,
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {
                        "name": "cron-lsp",
                        "version": "3.17.0"
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
            "textDocument/codeAction" => {
                let actions = self.compute_code_actions(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(actions).unwrap_or_default()), None))
            }
            "textDocument/formatting" => {
                let edits = self.compute_formatting(&req.params);
                Some(self.format_response(req.id, Some(serde_json::to_value(edits).unwrap_or(serde_json::Value::Array(vec![]))), None))
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
                            get_cl_hover_documentation(&word).or_else(|| get_hover_documentation(&word))
                        } else {
                            get_hover_documentation(&word).or_else(|| get_cl_hover_documentation(&word))
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
                }
            }
        }
        actions
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

pub fn extract_word_at_pos(line: &str, character: usize) -> Option<(String, usize, usize)> {
    if character > line.len() {
        return None;
    }

    let bytes = line.as_bytes();
    let is_ident_char = |c: u8| c.is_ascii_alphanumeric() || c == b'_';

    let mut start = character;
    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }

    let mut end = character;
    while end < bytes.len() && is_ident_char(bytes[end]) {
        end += 1;
    }

    if start < end {
        let word = std::str::from_utf8(&bytes[start..end]).ok()?.to_string();
        Some((word, start, end))
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
        let new_text = edits[0]["new_text"].as_str().unwrap();
        assert!(new_text.contains("B0000:"));
    }
}
