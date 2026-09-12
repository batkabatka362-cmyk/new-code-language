// ============================================================================
// CRON Compiler Diagnostics & Error Formatter
// Rustc-grade diagnostic engine with visual carets, error codes, and help notes.
// ============================================================================

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub span_len: usize,
    pub source_line: Option<String>,
    pub help: Option<String>,
    pub note: Option<String>,
}

impl Diagnostic {
    pub fn new(code: &'static str, message: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            code,
            message: message.into(),
            line,
            col,
            span_len: 1,
            source_line: None,
            help: None,
            note: None,
        }
    }

    pub fn with_span(mut self, len: usize) -> Self {
        self.span_len = len.max(1);
        self
    }

    pub fn with_source(mut self, src: &str) -> Self {
        if self.line > 0 {
            if let Some(line_str) = src.lines().nth(self.line - 1) {
                self.source_line = Some(line_str.to_string());
            }
        }
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn render(&self, file_label: Option<&str>) -> String {
        let label = file_label.unwrap_or("<input>");
        let mut out = String::new();

        out.push_str(&format!("error[{}]: {}\n", self.code, self.message));
        out.push_str(&format!("  --> {}:{}:{}\n", label, self.line, self.col));

        if let Some(src) = &self.source_line {
            let line_num_str = self.line.to_string();
            let pad = " ".repeat(line_num_str.len());

            out.push_str(&format!("   {} |\n", pad));
            out.push_str(&format!("{} | {}\n", line_num_str, src));

            let col_idx = if self.col > 0 { self.col - 1 } else { 0 };
            let caret_pad = " ".repeat(col_idx);
            let carets = "^".repeat(self.span_len);

            out.push_str(&format!("   {} | {}{}\n", pad, caret_pad, carets));
        }

        if let Some(note) = &self.note {
            out.push_str(&format!("   = note: {}\n", note));
        }

        if let Some(help) = &self.help {
            out.push_str(&format!("   = help: {}\n", help));
        }

        out
    }
}
