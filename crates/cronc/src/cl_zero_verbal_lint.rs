// ============================================================================
// CRON Zero-Verbal Constitution Linter & Auto-Fixer (cl_zero_verbal_lint.rs)
//
// Enforces Level 1A Post-Human Zero-Verbal Calculus rules:
// 1. Prohibits procedural variable soup (H_0, H_norm, Attn_out, Q_proj, etc.)
// 2. Prohibits traditional keywords (let, var, fn, def, return, for, while, etc.)
// 3. Prohibits English function names (softmax, transpose, outer, sqrt, mean, silu)
// 4. Provides 1-click Auto-Fixer transpiling forbidden prose to canonical Unicode
// ============================================================================



/// Severity of a Zero-Verbal diagnostic issue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroVerbalSeverity {
    Violation, // Hard violation of Zero-Verbal Constitution
    Warning,   // Suspicious procedural prose pattern
    Suggestion,// Canonical mathematical enhancement
}

impl ZeroVerbalSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            ZeroVerbalSeverity::Violation => "VIOLATION",
            ZeroVerbalSeverity::Warning => "WARNING",
            ZeroVerbalSeverity::Suggestion => "SUGGEST",
        }
    }
}

/// A specific diagnostic item found during zero-verbal auditing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroVerbalDiagnostic {
    pub line_number: usize,
    pub token: String,
    pub severity: ZeroVerbalSeverity,
    pub message: String,
    pub canonical_replacement: Option<String>,
}

/// Comprehensive Zero-Verbal Audit Report
#[derive(Debug, Clone)]
pub struct ZeroVerbalAuditReport {
    pub file_path: String,
    pub total_lines: usize,
    pub violations_count: usize,
    pub warnings_count: usize,
    pub suggestions_count: usize,
    pub is_constitution_compliant: bool,
    pub diagnostics: Vec<ZeroVerbalDiagnostic>,
}

impl ZeroVerbalAuditReport {
    pub fn render_ascii_hud(&self) -> String {
        let mut hud = String::new();
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud.push_str("                 CRON ZERO-VERBAL CONSTITUTION AUDIT HUD (LEVEL 1A)                     \n");
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud.push_str(&format!(" • Target File:       {}\n", self.file_path));
        hud.push_str(&format!(" • Total Code Lines:  {}\n", self.total_lines));
        hud.push_str(&format!(
            " • Constitution:      {}\n",
            if self.is_constitution_compliant {
                "✓ 100% POST-HUMAN COMPLIANT (0 Words, Pure Calculus)"
            } else {
                "⚠ PROSE VIOLATIONS DETECTED (English words or procedural soup found)"
            }
        ));
        hud.push_str(&format!(
            " • Summary:           {} Violations | {} Warnings | {} Suggestions\n",
            self.violations_count, self.warnings_count, self.suggestions_count
        ));
        hud.push_str("────────────────────────────────────────────────────────────────────────────────────────\n");

        if self.diagnostics.is_empty() {
            hud.push_str("  ✓ Code conforms rigorously to the Zero-Verbal Mathematical Constitution.\n");
        } else {
            hud.push_str(&format!(
                "| {:<5} | {:<10} | {:<15} | {:<25} | {:<16} |\n",
                "Line", "Severity", "Prose Token", "Violation Cause", "Canonical Unicode"
            ));
            hud.push_str("+-------+------------+-----------------+---------------------------+------------------+\n");
            for diag in &self.diagnostics {
                let token_display = if diag.token.len() > 15 {
                    format!("{}...", &diag.token[..12])
                } else {
                    diag.token.clone()
                };
                let msg_display = if diag.message.len() > 25 {
                    format!("{}...", &diag.message[..22])
                } else {
                    diag.message.clone()
                };
                let repl_display = diag.canonical_replacement.as_deref().unwrap_or("None");

                hud.push_str(&format!(
                    "| {:<5} | {:<10} | {:<15} | {:<25} | {:<16} |\n",
                    diag.line_number,
                    diag.severity.as_str(),
                    token_display,
                    msg_display,
                    repl_display
                ));
            }
        }
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud
    }
}

/// The Zero-Verbal Constitution Engine
pub struct ZeroVerbalLinter;

impl ZeroVerbalLinter {
    /// List of prohibited traditional procedural keywords
    const FORBIDDEN_KEYWORDS: &'static [&'static str] = &[
        "let", "var", "fn", "def", "function", "return", "for", "while", "if",
        "else", "class", "import", "struct", "mut", "const",
    ];

    /// List of prohibited procedural variable soup words
    const FORBIDDEN_VARIABLE_SOUP: &'static [&'static str] = &[
        "H_0", "H_norm", "Attn_out", "Q_proj", "H_attn", "H_mlp", "H_bit", "H_layer",
        "Z_logits", "Z_last", "tok", "out", "result", "output", "input", "hidden",
        "tensor", "layer", "weights", "bias", "activation", "logits",
    ];

    /// Prohibited English mathematical words mapped to canonical Unicode equivalents
    const WORD_TO_CANONICAL: &'static [(&'static str, &'static str)] = &[
        ("softmax", "ρ"),
        ("transpose", "^T"),
        ("outer", "⊗"),
        ("sqrt", "√"),
        ("mean", "μ"),
        ("sum", "∑"),
        ("norm", "N"),
        ("silu", "σ"),
        ("sigmoid", "σ"),
        ("grad", "∇"),
        ("attention", "Ψ"),
        ("gqa", "Ψ_g"),
    ];

    /// Audits a string of .cl source code against the Zero-Verbal Constitution
    pub fn audit_source(source: &str, file_name: &str) -> ZeroVerbalAuditReport {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            // Ignore pure comments or import/binding lines
            if trimmed.starts_with("//") || trimmed.starts_with("<<") {
                continue;
            }

            // Strip out string literals so URI paths ("path.safetensors:key") are not flagged
            let mut code_without_strings = String::new();
            let mut in_str = false;
            for ch in trimmed.chars() {
                if ch == '"' {
                    in_str = !in_str;
                    continue;
                }
                if !in_str {
                    code_without_strings.push(ch);
                }
            }

            // Check for traditional keywords
            for &kw in Self::FORBIDDEN_KEYWORDS {
                if Self::contains_word(&code_without_strings, kw) {
                    diagnostics.push(ZeroVerbalDiagnostic {
                        line_number: line_num,
                        token: kw.to_string(),
                        severity: ZeroVerbalSeverity::Violation,
                        message: "Forbidden procedural keyword".to_string(),
                        canonical_replacement: None,
                    });
                }
            }

            // Check for procedural variable soup
            for &soup in Self::FORBIDDEN_VARIABLE_SOUP {
                if Self::contains_word(&code_without_strings, soup) {
                    diagnostics.push(ZeroVerbalDiagnostic {
                        line_number: line_num,
                        token: soup.to_string(),
                        severity: ZeroVerbalSeverity::Violation,
                        message: "Forbidden procedural variable soup".to_string(),
                        canonical_replacement: Some("Mathematical tensor symbol (X, H, Y)".to_string()),
                    });
                }
            }

            // Check for English math functions
            for &(eng, canon) in Self::WORD_TO_CANONICAL {
                if Self::contains_word(&code_without_strings, eng) {
                    diagnostics.push(ZeroVerbalDiagnostic {
                        line_number: line_num,
                        token: eng.to_string(),
                        severity: ZeroVerbalSeverity::Violation,
                        message: format!("English word '{}' prohibited in Level 1A", eng),
                        canonical_replacement: Some(canon.to_string()),
                    });
                }
            }
        }

        let violations = diagnostics.iter().filter(|d| d.severity == ZeroVerbalSeverity::Violation).count();
        let warnings = diagnostics.iter().filter(|d| d.severity == ZeroVerbalSeverity::Warning).count();
        let suggestions = diagnostics.iter().filter(|d| d.severity == ZeroVerbalSeverity::Suggestion).count();

        ZeroVerbalAuditReport {
            file_path: file_name.to_string(),
            total_lines: lines.len(),
            violations_count: violations,
            warnings_count: warnings,
            suggestions_count: suggestions,
            is_constitution_compliant: violations == 0,
            diagnostics,
        }
    }

    /// Helper to test if a word exists as an isolated token
    fn contains_word(haystack: &str, needle: &str) -> bool {
        let bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let n = needle_bytes.len();
        if bytes.len() < n {
            return false;
        }

        for i in 0..=bytes.len() - n {
            if &bytes[i..i + n] == needle_bytes {
                let prev_ok = if i == 0 {
                    true
                } else {
                    let c = bytes[i - 1];
                    !c.is_ascii_alphanumeric() && c != b'_'
                };
                let next_ok = if i + n == bytes.len() {
                    true
                } else {
                    let c = bytes[i + n];
                    !c.is_ascii_alphanumeric() && c != b'_'
                };
                if prev_ok && next_ok {
                    return true;
                }
            }
        }
        false
    }

    /// Automatically fixes English prose and transforms code into canonical Zero-Verbal calculus
    pub fn auto_fix_source(source: &str) -> (String, usize) {
        let mut fixed = String::new();
        let mut fix_count = 0;

        for line in source.lines() {
            let mut line_str = line.to_string();

            // 1. Fix softmax(...) -> ρ(...)
            if line_str.contains("softmax(") {
                line_str = line_str.replace("softmax(", "ρ(");
                fix_count += 1;
            }

            // 2. Fix transpose(X) -> X^T
            while let Some(pos) = line_str.find("transpose(") {
                let rest = &line_str[pos + "transpose(".len()..];
                if let Some(close_pos) = rest.find(')') {
                    let inner = rest[..close_pos].trim();
                    let full_match = format!("transpose({})", inner);
                    let replacement = if inner.contains('@') || inner.contains('+') || inner.contains('*') {
                        format!("({})^T", inner)
                    } else {
                        format!("{}^T", inner)
                    };
                    line_str = line_str.replace(&full_match, &replacement);
                    fix_count += 1;
                } else {
                    break;
                }
            }

            // 3. Fix outer(A, B) -> (A ⊗ B)
            while let Some(pos) = line_str.find("outer(") {
                let rest = &line_str[pos + "outer(".len()..];
                if let Some(close_pos) = rest.find(')') {
                    let inner = &rest[..close_pos];
                    if let Some(comma_pos) = inner.find(',') {
                        let arg1 = inner[..comma_pos].trim();
                        let arg2 = inner[comma_pos + 1..].trim();
                        let full_match = format!("outer({})", inner);
                        let replacement = format!("({} ⊗ {})", arg1, arg2);
                        line_str = line_str.replace(&full_match, &replacement);
                        fix_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // 4. Fix sqrt(...) -> √(…)
            if line_str.contains("sqrt(") {
                line_str = line_str.replace("sqrt(", "√(");
                fix_count += 1;
            }

            // 5. Fix mean(...) -> μ(…)
            if line_str.contains("mean(") {
                line_str = line_str.replace("mean(", "μ(");
                fix_count += 1;
            }

            // 6. Fix silu(X) -> (X * σ(X))
            while let Some(pos) = line_str.find("silu(") {
                let rest = &line_str[pos + "silu(".len()..];
                if let Some(close_pos) = rest.find(')') {
                    let inner = rest[..close_pos].trim();
                    let full_match = format!("silu({})", inner);
                    let replacement = format!("(({}) * σ({}))", inner, inner);
                    line_str = line_str.replace(&full_match, &replacement);
                    fix_count += 1;
                } else {
                    break;
                }
            }

            // 7. Fix sigmoid(...) -> σ(…)
            if line_str.contains("sigmoid(") {
                line_str = line_str.replace("sigmoid(", "σ(");
                fix_count += 1;
            }

            fixed.push_str(&line_str);
            fixed.push('\n');
        }

        // Pop last newline if original didn't have one
        if !source.ends_with('\n') && fixed.ends_with('\n') {
            fixed.pop();
        }

        (fixed, fix_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_verbal_audit_clean_code() {
        let clean = r#"
c_q(x, W_dq) = x @ W_dq
q(c, W_uq)   = c @ W_uq
c_kv(x, W_dk) = x @ W_dk
k(c, W_uk)    = c @ W_uk
v(c, W_uv)    = c @ W_uv
Ψ(q, k, v) = ρ((q @ k^T) * 0.125) @ v
"#;
        let report = ZeroVerbalLinter::audit_source(clean, "deepseek_mla.cl");
        assert!(report.is_constitution_compliant);
        assert_eq!(report.violations_count, 0);
    }

    #[test]
    fn test_zero_verbal_audit_flags_prose_words() {
        let dirty = r#"
// Bad function with prose
attn(q, k, v) = softmax((q @ transpose(k)) * 0.125) @ v
H_norm = sqrt(mean(x^2) + 1e-5)
val = outer(k, v)
res = silu(x)
let mut x = 5
"#;
        let report = ZeroVerbalLinter::audit_source(dirty, "dirty.cl");
        assert!(!report.is_constitution_compliant);
        assert!(report.violations_count >= 6);
        let tokens: Vec<String> = report.diagnostics.iter().map(|d| d.token.clone()).collect();
        assert!(tokens.contains(&"softmax".to_string()));
        assert!(tokens.contains(&"transpose".to_string()));
        assert!(tokens.contains(&"sqrt".to_string()));
        assert!(tokens.contains(&"mean".to_string()));
        assert!(tokens.contains(&"outer".to_string()));
        assert!(tokens.contains(&"let".to_string()));
    }

    #[test]
    fn test_zero_verbal_autofix_transpilation() {
        let code = r#"attn(q, k, v) = softmax((q @ transpose(k)) * 0.125) @ v
M(k, v) = outer(k, v)
N(x) = sqrt(mean(x^2) + 1e-5)
"#;
        let (fixed, count) = ZeroVerbalLinter::auto_fix_source(code);
        assert!(count >= 5);
        assert!(fixed.contains("ρ((q @ k^T) * 0.125) @ v"));
        assert!(fixed.contains("(k ⊗ v)"));
        assert!(fixed.contains("√(μ(x^2) + 1e-5)"));

        // Verify the fixed code passes audit with 0 violations!
        let report = ZeroVerbalLinter::audit_source(&fixed, "fixed.cl");
        assert!(report.is_constitution_compliant);
    }
}
