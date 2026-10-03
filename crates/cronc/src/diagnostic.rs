// ============================================================================
// CRON Compiler Diagnostics & Error Formatter
// Rustc-grade diagnostic engine with visual carets, multi-line context,
// ANSI colors, "did you mean" fuzzy matching, auto-suggestions, and error catalog.
// ============================================================================

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
    Help,
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticSeverity::Error => write!(f, "error"),
            DiagnosticSeverity::Warning => write!(f, "warning"),
            DiagnosticSeverity::Note => write!(f, "note"),
            DiagnosticSeverity::Help => write!(f, "help"),
        }
    }
}

/// Represents a single suggestion for automatic code repair
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub replacement: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub line: usize,
    pub col: usize,
    pub span_len: usize,
    pub source_line: Option<String>,
    pub pre_line: Option<(usize, String)>,
    pub post_line: Option<(usize, String)>,
    pub label: Option<String>,
    pub suggestion: Option<Suggestion>,
    pub help: Option<String>,
    pub note: Option<String>,
    pub extra_notes: Vec<String>,
    pub extra_helps: Vec<String>,
    pub force_color: Option<bool>,
}

impl Diagnostic {
    /// Creates a new error diagnostic (backward compatible with original API)
    pub fn new(code: &'static str, message: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            code,
            message: message.into(),
            severity: DiagnosticSeverity::Error,
            line,
            col,
            span_len: 1,
            source_line: None,
            pre_line: None,
            post_line: None,
            label: None,
            suggestion: None,
            help: None,
            note: None,
            extra_notes: Vec::new(),
            extra_helps: Vec::new(),
            force_color: None,
        }
    }

    /// Creates a warning diagnostic
    pub fn warning(code: &'static str, message: impl Into<String>, line: usize, col: usize) -> Self {
        let mut d = Self::new(code, message, line, col);
        d.severity = DiagnosticSeverity::Warning;
        d
    }

    /// Set span character length (default 1)
    pub fn with_span(mut self, len: usize) -> Self {
        self.span_len = len.max(1);
        self
    }

    /// Set source code content to automatically extract error line and context lines
    pub fn with_source(mut self, src: &str) -> Self {
        if self.line > 0 {
            let lines: Vec<&str> = src.lines().collect();
            let idx = self.line - 1;
            if idx < lines.len() {
                self.source_line = Some(lines[idx].to_string());
                if idx > 0 {
                    self.pre_line = Some((self.line - 1, lines[idx - 1].to_string()));
                }
                if idx + 1 < lines.len() {
                    self.post_line = Some((self.line + 1, lines[idx + 1].to_string()));
                }
            }
        }
        self
    }

    /// Inline label directly underneath the caret (e.g. `^^^^ expected 'i64', found 'bool'`)
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add an automated code replacement suggestion
    pub fn with_suggestion(mut self, replacement: impl Into<String>, description: impl Into<String>) -> Self {
        self.suggestion = Some(Suggestion {
            replacement: replacement.into(),
            description: description.into(),
        });
        self
    }

    /// Add primary actionable help note
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        if self.help.is_none() {
            self.help = Some(help.into());
        } else {
            self.extra_helps.push(help.into());
        }
        self
    }

    /// Add primary architectural/hardware note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        if self.note.is_none() {
            self.note = Some(note.into());
        } else {
            self.extra_notes.push(note.into());
        }
        self
    }

    /// Force enable or disable ANSI color rendering
    pub fn with_color(mut self, enable: bool) -> Self {
        self.force_color = Some(enable);
        self
    }

    /// Determines if ANSI color output should be enabled
    fn should_use_color(&self) -> bool {
        if let Some(forced) = self.force_color {
            return forced;
        }
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }
        if std::env::var("CLICOLOR_FORCE").map(|v| v != "0").unwrap_or(false)
            || std::env::var("FORCE_COLOR").map(|v| v != "0").unwrap_or(false)
        {
            return true;
        }
        false
    }

    /// Render diagnostic in plain ASCII (no ANSI codes)
    pub fn render_plain(&self, file_label: Option<&str>) -> String {
        self.render_internal(file_label, false)
    }

    /// Render diagnostic with or without ANSI colors
    pub fn render_colored(&self, file_label: Option<&str>, use_color: bool) -> String {
        self.render_internal(file_label, use_color)
    }

    /// Render diagnostic following Rustc format with auto-color detection
    pub fn render(&self, file_label: Option<&str>) -> String {
        let color = self.should_use_color();
        self.render_internal(file_label, color)
    }

    fn render_internal(&self, file_label: Option<&str>, color: bool) -> String {
        let label = file_label.unwrap_or("<input>");
        let mut out = String::new();

        // ANSI Color sequences
        let (c_bold_red, c_bold_yellow, c_bold_cyan, c_bold_blue, c_bold_green, c_bold_white, c_reset) = if color {
            (
                "\x1b[1;31m",
                "\x1b[1;33m",
                "\x1b[1;36m",
                "\x1b[1;34m",
                "\x1b[1;32m",
                "\x1b[1;37m",
                "\x1b[0m",
            )
        } else {
            ("", "", "", "", "", "", "")
        };

        // Header: e.g. error[E0002]: Linear type leak...
        let (sev_str, sev_color) = match self.severity {
            DiagnosticSeverity::Error => ("error", c_bold_red),
            DiagnosticSeverity::Warning => ("warning", c_bold_yellow),
            DiagnosticSeverity::Note => ("note", c_bold_cyan),
            DiagnosticSeverity::Help => ("help", c_bold_green),
        };

        out.push_str(&format!(
            "{}{}[{}]{}: {}{}{}\n",
            sev_color, sev_str, self.code, c_reset,
            c_bold_white, self.message, c_reset
        ));

        // Location: --> file:line:col
        // Keep "--> file:line:col" as a clean substring without internal escape codes for compatibility
        if color {
            out.push_str(&format!(
                "{}  --> {}:{}:{}{}\n",
                c_bold_blue, label, self.line, self.col, c_reset
            ));
        } else {
            out.push_str(&format!("  --> {}:{}:{}\n", label, self.line, self.col));
        }

        if let Some(src) = &self.source_line {
            let max_line_num = self.post_line.as_ref().map(|(l, _)| *l).unwrap_or(self.line);
            let pad_len = max_line_num.to_string().len().max(2);
            let empty_pad = " ".repeat(pad_len);

            out.push_str(&format!("{}   {} |{}\n", c_bold_blue, empty_pad, c_reset));

            // Optional 1 preceding line
            if let Some((p_num, p_src)) = &self.pre_line {
                let p_pad = format!("{:>width$}", p_num, width = pad_len);
                out.push_str(&format!("{} {} |{} {}\n", c_bold_blue, p_pad, c_reset, p_src));
            }

            // Error line
            let l_pad = format!("{:>width$}", self.line, width = pad_len);
            out.push_str(&format!("{} {} |{} {}\n", c_bold_blue, l_pad, c_reset, src));

            // Caret underline with optional label
            let col_idx = if self.col > 0 { self.col - 1 } else { 0 };
            let caret_pad = " ".repeat(col_idx);
            let carets = "^".repeat(self.span_len);

            let label_part = if let Some(lbl) = &self.label {
                format!(" {}{}{}", sev_color, lbl, c_reset)
            } else {
                String::new()
            };

            out.push_str(&format!(
                "{}   {} |{} {}{}{}{}\n",
                c_bold_blue, empty_pad, c_reset,
                caret_pad, sev_color, carets, label_part
            ));

            // Optional 1 succeeding line
            if let Some((post_num, post_src)) = &self.post_line {
                let post_pad = format!("{:>width$}", post_num, width = pad_len);
                out.push_str(&format!("{} {} |{} {}\n", c_bold_blue, post_pad, c_reset, post_src));
            }

            out.push_str(&format!("{}   {} |{}\n", c_bold_blue, empty_pad, c_reset));
        }

        // Suggestions with replacement preview
        if let Some(sug) = &self.suggestion {
            out.push_str(&format!(
                "{}   = help:{} {}: `{}`\n",
                c_bold_green, c_reset, sug.description, sug.replacement
            ));
        }

        // Primary & extra notes
        if let Some(note) = &self.note {
            out.push_str(&format!("{}   = note:{} {}\n", c_bold_cyan, c_reset, note));
        }
        for n in &self.extra_notes {
            out.push_str(&format!("{}   = note:{} {}\n", c_bold_cyan, c_reset, n));
        }

        // Primary & extra helps
        if let Some(help) = &self.help {
            out.push_str(&format!("{}   = help:{} {}\n", c_bold_green, c_reset, help));
        }
        for h in &self.extra_helps {
            out.push_str(&format!("{}   = help:{} {}\n", c_bold_green, c_reset, h));
        }

        out
    }

    /// Serialize diagnostic to standard JSON for IDE and LSP integration
    pub fn to_json(&self, file_label: Option<&str>) -> String {
        let label = file_label.unwrap_or("<input>");
        format!(
            r#"{{"code":"{}","severity":"{}","message":"{}","file":"{}","line":{},"col":{},"span_len":{},"help":{},"note":{}}}"#,
            self.code,
            self.severity,
            self.message.replace('"', "\\\""),
            label.replace('\\', "\\\\"),
            self.line,
            self.col,
            self.span_len,
            self.help.as_ref().map(|h| format!("\"{}\"", h.replace('"', "\\\""))).unwrap_or_else(|| "null".to_string()),
            self.note.as_ref().map(|n| format!("\"{}\"", n.replace('"', "\\\""))).unwrap_or_else(|| "null".to_string())
        )
    }
}

// ============================================================================
// Levenshtein & "Did You Mean" Fuzzy Matching Engine
// ============================================================================

/// Computes Damerau-Levenshtein distance (accounts for insertions, deletions, substitutions, and transpositions)
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }

    let mut d = vec![vec![0usize; len2 + 1]; len1 + 1];

    for i in 0..=len1 { d[i][0] = i; }
    for j in 0..=len2 { d[0][j] = j; }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);

            if i > 1 && j > 1 && s1_chars[i - 1] == s2_chars[j - 2] && s1_chars[i - 2] == s2_chars[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }

    d[len1][len2]
}

/// Finds the best matching candidate from a candidate list if within threshold distance
pub fn did_you_mean<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    if query.is_empty() || candidates.is_empty() {
        return None;
    }

    let mut best_match: Option<&'a str> = None;
    let mut best_dist = usize::MAX;

    // Allowed threshold: 1 for short words (<=3), 2 for medium (<=6), 3 for longer words
    let max_allowed = if query.len() <= 3 {
        1
    } else if query.len() <= 6 {
        2
    } else {
        3
    };

    for candidate in candidates {
        let dist = levenshtein_distance(query, candidate);
        if dist <= max_allowed && dist < best_dist {
            best_dist = dist;
            best_match = Some(*candidate);
        }
    }

    best_match
}

// Standard CRON language keywords for did-you-mean suggestions
pub const CRON_KEYWORDS: &[&str] = &[
    "def", "let", "lin", "grad", "region", "mut", "return", "export", "as",
    "await", "spawn", "consume", "import", "from", "if", "else", "while",
    "for", "in", "true", "false", "and", "or", "not", "struct", "enum",
    "match", "type", "trait", "impl", "schedule", "fuse", "comptime",
    "brain", "fork", "simulate", "abort", "then", "with", "resilient_compute",
    "fallback", "invariant", "ensures", "proof_contract",
];

// Standard scalar & tensor types for did-you-mean suggestions
pub const CRON_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64",
    "f16", "f32", "f64", "b1.58", "trit", "bool", "String",
    "Tensor", "Tensor4D", "Channel", "Fiber", "Rotor4D", "Bivector",
];

// ============================================================================
// Comprehensive CRON Error Catalog (like `rustc --explain E0002`)
// ============================================================================

pub struct ErrorInfo {
    pub code: &'static str,
    pub title: &'static str,
    pub explanation: &'static str,
    pub bad_example: &'static str,
    pub good_example: &'static str,
}

pub fn explain_error(code: &str) -> Option<ErrorInfo> {
    let clean_code = code.trim().to_uppercase();
    match clean_code.as_str() {
        "E0001" => Some(ErrorInfo {
            code: "E0001",
            title: "Syntax or Grammatical Parse Error",
            explanation: "The CRON compiler encountered unexpected tokens or invalid syntactic structure that violates the CRON EBNF grammar.",
            bad_example: "fn calculate_weights() {\n    // 'fn' is Rust syntax, not CRON\n}",
            good_example: "def calculate_weights() {\n    // in CRON, functions use 'def'\n}",
        }),
        "E0002" => Some(ErrorInfo {
            code: "E0002",
            title: "Linear Affine Type Leak",
            explanation: "A linear variable (`lin x = ...`) was allocated but never consumed or exported before its enclosing scope closed. On 256-Core 4D-Torus silicon, unconsumed linear variables induce physical SRAM register leaks.",
            bad_example: "def process() {\n    lin buf = allocate_buffer();\n    // buf is abandoned here\n}",
            good_example: "def process() {\n    lin buf = allocate_buffer();\n    consume(buf);\n}",
        }),
        "E0003" => Some(ErrorInfo {
            code: "E0003",
            title: "Use-After-Consume / Double Linear Consumption",
            explanation: "A linear variable (`lin x = ...`) was already consumed and cannot be reused or duplicated. Linear values enforce strict affine uniqueness.",
            bad_example: "def send_twice(lin msg) {\n    send(consume(msg));\n    send(consume(msg)); // Error: already consumed\n}",
            good_example: "def send_once(lin msg) {\n    send(consume(msg));\n}",
        }),
        "E0004" => Some(ErrorInfo {
            code: "E0004",
            title: "Region Arena Lifetime Escape Violation",
            explanation: "A variable allocated inside a `region { ... }` arena was referenced outside without being promoted via `export var as exported_var`. Scoped region memory is wiped in 0 cycles upon region exit.",
            bad_example: "let outside = region {\n    let local = [1, 2, 3];\n    local // Error: escapes region arena\n};",
            good_example: "let outside = region {\n    let local = [1, 2, 3];\n    export local as saved;\n};",
        }),
        "E0005" => Some(ErrorInfo {
            code: "E0005",
            title: "Reassignment to Immutable Variable",
            explanation: "An assignment was made to a variable not declared with `mut`. By default, all variables in CRON are immutable to guarantee thread-safe spatial dataflow.",
            bad_example: "let x = 10;\nx = 20; // Error: x is immutable",
            good_example: "let mut x = 10;\nx = 20;",
        }),
        "E0006" => Some(ErrorInfo {
            code: "E0006",
            title: "Type Mismatch",
            explanation: "An expression was assigned to a target with an incompatible type. CRON performs strict compile-time type verification with zero implicit lossy casting.",
            bad_example: "let count: i64 = \"42\"; // Error: expected i64, found String",
            good_example: "let count: i64 = 42;",
        }),
        "E0007" => Some(ErrorInfo {
            code: "E0007",
            title: "Undefined Identifier / Variable Out of Scope",
            explanation: "The variable, function, or symbol name referenced in the expression does not exist in the current lexical scope.",
            bad_example: "def compute() {\n    return unknow_tensor + 1;\n}",
            good_example: "def compute() {\n    let known_tensor = 42;\n    return known_tensor + 1;\n}",
        }),
        "E0008" => Some(ErrorInfo {
            code: "E0008",
            title: "Trait Contract Monomorphization Violation",
            explanation: "A `impl Trait for Struct` block does not implement all required methods defined in the trait. All trait dispatch in CRON is statically monomorphized (zero virtual method table overhead).",
            bad_example: "trait Printable {\n    def print_self();\n}\nimpl Printable for Neuron {\n    // missing def print_self()\n}",
            good_example: "trait Printable {\n    def print_self();\n}\nimpl Printable for Neuron {\n    def print_self() { println(\"Neuron\"); }\n}",
        }),
        "E0009" => Some(ErrorInfo {
            code: "E0009",
            title: "Spatial Hardware Memory Domain Crossing Violation",
            explanation: "Direct assignment across hardware memory tiers (@sram, @hbm, @noc) is forbidden without explicit DMA synchronization (`dma_sync`). This guarantees zero pipeline stalls across the 4D-Torus interconnect.",
            bad_example: "let hbm_tensor: @hbm Tensor = sram_tensor; // Error",
            good_example: "let hbm_tensor: @hbm Tensor = dma_sync(sram_tensor);",
        }),
        "E0010" => Some(ErrorInfo {
            code: "E0010",
            title: "Dependent Tensor Shape Dimension Mismatch",
            explanation: "Symbolic shape inference detected that tensor matrix contraction dimensions do not match at compile-time (e.g. contracting [B, M, K] with [B, J, N] where K != J).",
            bad_example: "let a: Tensor[1024, 768] = ...;\nlet b: Tensor[512, 768] = ...;\nlet c = matmul(a, b); // Error: 768 != 512",
            good_example: "let a: Tensor[1024, 768] = ...;\nlet b: Tensor[768, 512] = ...;\nlet c = matmul(a, b);",
        }),
        "E0011" => Some(ErrorInfo {
            code: "E0011",
            title: "Decoupled Silicon Schedule Target Function Not Found",
            explanation: "A `schedule target_fn { ... }` directive referenced a function or kernel name that does not exist in the program.",
            bad_example: "schedule flash_attention {\n    tile_size(32, 32);\n} // Error: def flash_attention not defined",
            good_example: "def flash_attention(...) { ... }\nschedule flash_attention {\n    tile_size(32, 32);\n}",
        }),
        "E0012" => Some(ErrorInfo {
            code: "E0012",
            title: "Invalid Hardware Schedule Directive Parameters",
            explanation: "A schedule directive contains invalid parameters (e.g. zero tile dimension, non-power-of-two vector width, or invalid 4D-Torus axis).",
            bad_example: "schedule gemm {\n    tile_size(0, 32); // Error: zero dimension\n    vectorize(3);     // Error: not power of 2\n}",
            good_example: "schedule gemm {\n    tile_size(32, 32);\n    vectorize(4);\n}",
        }),
        "E0013" => Some(ErrorInfo {
            code: "E0013",
            title: "Kernel Fusion Dependency Cycle",
            explanation: "A `fuse { ... }` block contains an illegal cyclic dependency between merged kernel operations, preventing valid topological scheduling.",
            bad_example: "fuse {\n    let a = b + 1;\n    let b = a * 2; // Cyclic dependency\n}",
            good_example: "fuse {\n    let b = initial_val * 2;\n    let a = b + 1;\n}",
        }),
        "E0014" => Some(ErrorInfo {
            code: "E0014",
            title: "Static Invariant or Postcondition Contract Violation",
            explanation: "An `invariant` or `ensures` postcondition contract could not be proven true at compile time for all execution paths.",
            bad_example: "def abs(x: i32) -> i32 ensures result >= 0 {\n    return x; // May be negative\n}",
            good_example: "def abs(x: i32) -> i32 ensures result >= 0 {\n    return if x < 0 { -x } else { x };\n}",
        }),
        "E0015" => Some(ErrorInfo {
            code: "E0015",
            title: "Formal Proof Contract Unsatisfiable",
            explanation: "The formal proof checker could not prove that the bounded function invariants hold across all state transitions.",
            bad_example: "proof_contract safety {\n    invariant 2 + 2 == 5; // Unprovable falsehood\n}",
            good_example: "proof_contract safety {\n    invariant buffer_len <= MAX_CAPACITY;\n}",
        }),
        "E0016" => Some(ErrorInfo {
            code: "E0016",
            title: "Tainted Data Sink / Security Boundary Violation",
            explanation: "An external or tainted input was passed to a privileged hardware intrinsic without first passing through `sanitize()`.",
            bad_example: "def handler(tainted_input: String) {\n    secure_patch_icache(tainted_input); // Error: tainted input\n}",
            good_example: "def handler(tainted_input: String) {\n    let clean = sanitize(tainted_input);\n    secure_patch_icache(clean);\n}",
        }),
        "E0017" => Some(ErrorInfo {
            code: "E0017",
            title: "Missing Capability Permission for Privileged Operation",
            explanation: "The current execution scope lacks the required security capability (e.g. `caps::DMA_DIRECT`, `caps::OPTICAL_WRITE`) to invoke a restricted hardware instruction.",
            bad_example: "def reset_laser() {\n    laser_reg_write(0x10, 0); // Error: lacks laser capability\n}",
            good_example: "def reset_laser() with [caps::OPTICAL_WRITE] {\n    laser_reg_write(0x10, 0);\n}",
        }),
        "E0018" => Some(ErrorInfo {
            code: "E0018",
            title: "PGAS Bank Conflict / Uneven Memory Stride",
            explanation: "A memory access pattern accesses multiple addresses mapping to the identical SRAM bank in the same clock cycle, causing pipeline stall serialization.",
            bad_example: "let val = sram_read(bank_index * 16); // All threads hit bank 0",
            good_example: "let val = sram_swizzle_read(bank_index); // Swizzled uniform stride",
        }),
        "E0019" => Some(ErrorInfo {
            code: "E0019",
            title: "Quantization Dynamic Range Overflow",
            explanation: "A floating-point weight or activation exceeds the representable range of the target quantized type (e.g. BitNet b1.58 ternary trits {-1, 0, +1}).",
            bad_example: "let t: trit = 2.5; // Error: trit only represents -1, 0, or +1",
            good_example: "let t: trit = quantize_ternary(2.5);",
        }),
        "E0020" => Some(ErrorInfo {
            code: "E0020",
            title: "4D Torus Routing Deadlock Cycle Detected",
            explanation: "Static dimension-order analysis detected an unregulated channel cycle along the X/Y/Z/W axes, risking network-on-chip deadlock.",
            bad_example: "pipe.send_west(pipe.recv_east()); // Bidirectional ring deadlock hazard",
            good_example: "pipe.route_dor_xyzw(packet); // Deadlock-free dimension-order routing",
        }),
        "E0021" => Some(ErrorInfo {
            code: "E0021",
            title: "Landauer Thermodynamic Dissipation Limit Exceeded",
            explanation: "The scheduled microcode operations exceed the Landauer thermodynamic thermal budget for the allocated core tile at current ambient operating temperature.",
            bad_example: "schedule heavy_burst {\n    unroll(64);\n} // Exceeds thermal dissipation limit of 28W",
            good_example: "schedule heavy_burst {\n    tile_size(16, 16);\n    distribute_4d(\"X+\", 4);\n}",
        }),
        "E0022" => Some(ErrorInfo {
            code: "E0022",
            title: "Clifford Rotor Degenerate Geometric Metric",
            explanation: "A 4D Clifford rotor multivector has a zero norm or undefined metric signature under Cl(4,0) spatial algebra.",
            bad_example: "let r = Rotor4D::new(0.0, 0.0, 0.0, 0.0); // Degenerate rotor",
            good_example: "let r = Rotor4D::identity();",
        }),
        "E0023" => Some(ErrorInfo {
            code: "E0023",
            title: "Metaplasticity Trace Saturation",
            explanation: "Spike-timing-dependent plasticity (STDP) synaptic weight adjustments exceed the biological saturation threshold, requiring homeostatic normalization.",
            bad_example: "synapse.weight += 100.0; // Saturation runaway",
            good_example: "synapse.apply_stdp_with_homeostasis(pre, post);",
        }),
        "E0024" => Some(ErrorInfo {
            code: "E0024",
            title: "Photonic MZI Phase Calibration Drift",
            explanation: "The Mach-Zehnder Interferometer phase angle theta/phi drifted outside calibrated optical interference bounds [0, 2π).",
            bad_example: "mzi.set_phase(10.5 * PI); // Out-of-bounds phase",
            good_example: "mzi.set_phase(fmod(phase, 2.0 * PI));",
        }),
        "E0025" => Some(ErrorInfo {
            code: "E0025",
            title: "Quantum Superposition Amplitude Normalization Violation",
            explanation: "The sum of squared quantum state amplitudes does not equal 1.0 (within epsilon tolerance), violating unitarity.",
            bad_example: "let state = QuantumState::new([0.8, 0.8]); // Sum of squares = 1.28 != 1.0",
            good_example: "let state = QuantumState::normalize([0.8, 0.6]); // 0.64 + 0.36 = 1.0",
        }),
        _ => None,
    }
}
