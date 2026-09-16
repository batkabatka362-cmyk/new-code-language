// ============================================================================
// CRON Language Server Protocol (LSP 3.17) Engine
// Target: Antigravity IDE, VS Code, Neovim, and Language Clients
// Capabilities: Diagnostics (E0001..E0010), Hover Tooltips, Auto-Completion
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
    pub kind: u32, // 1=Text, 3=Function, 7=Class, 14=Keyword, etc.
    pub detail: Option<String>,
    pub documentation: Option<MarkupContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
}

// === CRON Documentation & Symbol Knowledge Base ===

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
            32-bit/64-bit physical memory address for High Bandwidth Memory (HBM3e)DMA channels.".to_string()
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
                            "triggerCharacters": [".", ":", "$", "@"]
                        }
                    },
                    "serverInfo": {
                        "name": "cron-lsp",
                        "version": "0.1.0"
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
                let completions = get_completion_items();
                let result = serde_json::json!({
                    "isIncomplete": false,
                    "items": completions
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

    pub fn compute_diagnostics_for_source(&self, source: &str) -> Vec<Diagnostic> {
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

    pub fn compute_and_publish_diagnostics(&self, uri: &str, source: &str) -> String {
        let diagnostics = self.compute_diagnostics_for_source(source);
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
                if let Some(line_str) = doc.lines().nth(line) {
                    if let Some((word, word_start, word_end)) = extract_word_at_pos(line_str, character) {
                        if let Some(doc_md) = get_hover_documentation(&word) {
                            return serde_json::json!({
                                "contents": {
                                    "kind": "markdown",
                                    "value": doc_md
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

        // Read headers (Content-Length: <n>\r\n\r\n)
        let mut content_length: usize = 0;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Ok(()); // EOF
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break; // Header section ended
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

        // Read exact content bytes
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
    }

    #[test]
    fn test_lsp_linear_leak_diagnostics() {
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
    fn test_lsp_hover_tooltip() {
        let mut server = LspServer::new();
        let code = ".MODULE HoverTest\n_main:\nlet lin wave = pack_wave(amp=[1, 2], phase=[3, 4])\nconsume(wave)\n.END\n";
        server.documents.insert("file:///test.cr".to_string(), code.to_string());

        // Hover over "pack_wave" on line 2 (0-indexed)
        let hover_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 42,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": "file:///test.cr" },
                "position": { "line": 2, "character": 18 }
            }
        });

        let resp_str = server.handle_message(&hover_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        assert_eq!(resp["id"], 42);
        let val = resp["result"]["contents"]["value"].as_str().unwrap();
        assert!(val.contains("pack_wave"));
        assert!(val.contains("wave_t"));
    }

    #[test]
    fn test_lsp_completions() {
        let mut server = LspServer::new();
        let comp_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 100,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": "file:///test.cr" },
                "position": { "line": 0, "character": 0 }
            }
        });

        let resp_str = server.handle_message(&comp_req.to_string()).unwrap();
        let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
        let items = resp["result"]["items"].as_array().unwrap();
        assert!(items.iter().any(|i| i["label"] == "resilient_compute"));
        assert!(items.iter().any(|i| i["label"] == "compute_attention_head"));
        assert!(items.iter().any(|i| i["label"] == "wave_t"));
    }
}
