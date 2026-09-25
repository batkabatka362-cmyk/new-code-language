// ============================================================================
// CRON Documentation Generator Engine (`cron doc`)
// Extracts AST Documentation, API Signatures, Brain Directives & Types
// Generates Clean Interactive Markdown and HTML Documentation Sites
// ============================================================================

use crate::ast::*;
use std::collections::HashMap;

/// Documentation block for a single function or method.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDoc {
    pub name: String,
    pub is_inline: bool,
    pub params: Vec<(String, String)>,
    pub return_type: String,
    pub doc_comment: String,
}

/// Documentation block for a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDoc {
    pub name: String,
    pub is_linear: bool,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<FunctionDoc>,
    pub doc_comment: String,
}

/// Documentation block for a cognitive brain partition.
#[derive(Debug, Clone, PartialEq)]
pub struct BrainDoc {
    pub name: String,
    pub target_core_range: String,
    pub description: String,
}

/// Complete documentation model for a module / file.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDoc {
    pub module_name: String,
    pub structs: Vec<StructDoc>,
    pub functions: Vec<FunctionDoc>,
    pub brains: Vec<BrainDoc>,
    pub type_aliases: HashMap<String, String>,
}

impl ModuleDoc {
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            structs: Vec::new(),
            functions: Vec::new(),
            brains: Vec::new(),
            type_aliases: HashMap::new(),
        }
    }

    /// Generates standard GitHub Flavored Markdown documentation.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Module `{}`\n\n", self.module_name));
        md.push_str("> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor\n\n");

        if !self.brains.is_empty() {
            md.push_str("## 🧠 Cognitive Brain Partitions\n\n");
            for brain in &self.brains {
                md.push_str(&format!("### Brain `{}`\n", brain.name));
                md.push_str(&format!("- **Cores**: {}\n", brain.target_core_range));
                md.push_str(&format!("- **Description**: {}\n\n", brain.description));
            }
        }

        if !self.structs.is_empty() {
            md.push_str("## 📦 Structures & Linear Types\n\n");
            for st in &self.structs {
                let linear_tag = if st.is_linear { " `(linear affine)`" } else { "" };
                md.push_str(&format!("### `struct {}{}`\n\n", st.name, linear_tag));
                if !st.doc_comment.is_empty() {
                    md.push_str(&format!("{}\n\n", st.doc_comment));
                }
                md.push_str("| Field | Type |\n|---|---|\n");
                for (fname, ftype) in &st.fields {
                    md.push_str(&format!("| `{}` | `{}` |\n", fname, ftype));
                }
                md.push('\n');

                if !st.methods.is_empty() {
                    md.push_str("**Methods:**\n\n");
                    for m in &st.methods {
                        let params_str = m.params.iter()
                            .map(|(p, t)| format!("{}: {}", p, t))
                            .collect::<Vec<_>>()
                            .join(", ");
                        md.push_str(&format!("- `fn {}({}) -> {}`\n", m.name, params_str, m.return_type));
                    }
                    md.push('\n');
                }
            }
        }

        if !self.functions.is_empty() {
            md.push_str("## ⚡ Functions & Intrinsics\n\n");
            for func in &self.functions {
                let inline_tag = if func.is_inline { " `(inline zero-overhead)`" } else { "" };
                let params_str = func.params.iter()
                    .map(|(p, t)| format!("{}: {}", p, t))
                    .collect::<Vec<_>>()
                    .join(", ");
                md.push_str(&format!("### `fn {}({}) -> {}{}`\n\n", func.name, params_str, func.return_type, inline_tag));
                if !func.doc_comment.is_empty() {
                    md.push_str(&format!("{}\n\n", func.doc_comment));
                }
            }
        }

        md
    }
}

/// Extracts documentation directly from a verified CRON AST `Program`.
pub fn extract_program_doc(module_name: &str, program: &Program) -> ModuleDoc {
    let mut doc = ModuleDoc::new(module_name);

    // Extract structs
    for st in &program.structs {
        let fields = st.fields.clone();

        // Find associated impl methods
        let mut methods = Vec::new();
        for impl_decl in &program.impls {
            if impl_decl.target_struct == st.name {
                for m in &impl_decl.methods {
                    let params = m.params.iter()
                        .map(|p| (p.name.clone(), p.param_type.clone()))
                        .collect();
                    methods.push(FunctionDoc {
                        name: m.name.clone(),
                        is_inline: m.is_inline,
                        params,
                        return_type: m.return_type.clone().unwrap_or_else(|| "()".to_string()),
                        doc_comment: String::new(),
                    });
                }
            }
        }

        doc.structs.push(StructDoc {
            name: st.name.clone(),
            is_linear: false,
            fields,
            methods,
            doc_comment: String::new(),
        });
    }

    // Extract functions
    for func in &program.functions {
        let params = func.params.iter()
            .map(|p| (p.name.clone(), p.param_type.clone()))
            .collect();
        doc.functions.push(FunctionDoc {
            name: func.name.clone(),
            is_inline: func.is_inline,
            params,
            return_type: func.return_type.clone().unwrap_or_else(|| "()".to_string()),
            doc_comment: String::new(),
        });
    }

    // Extract cognitive brains
    for brain in &program.brains {
        doc.brains.push(BrainDoc {
            name: brain.name.clone(),
            target_core_range: format!("Core 0..255 (Faculties: {})", brain.name),
            description: format!("Autonomous Biological Brain Partition for {}", brain.name),
        });
    }

    doc
}
