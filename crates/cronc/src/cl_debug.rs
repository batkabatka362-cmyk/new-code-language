// ============================================================================
// CRON Cognitive Low-Level (.cl) Source-Level Debugger Metadata Engine
// Module: cronc::cl_debug
// Target: 256-Core 4D-Torus Neuromorphic Hardware
//
// Provides comprehensive source-mapping, symbol tracking, and interactive
// stepping inspection between high-level .cr AST and low-level .cl VLIW bundles.
// ============================================================================

use std::collections::{HashMap, HashSet};
use crate::cl_lang::{parse_slot, ClBundle};

/// High-level source location mapped to machine-level instruction cycles
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub symbol: String,
}

/// Register semantic binding for hardware variable inspection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterSymbol {
    pub reg_index: usize,
    pub name: String,
    pub type_hint: String,
    pub last_updated_cycle: usize,
}

/// Debug metadata attached to a single VLIW cycle bundle
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleDebugMeta {
    pub cycle: usize,
    pub source_loc: Option<SourceLocation>,
    pub slot_annotations: [String; 4],
    pub written_regs: Vec<usize>,
    pub read_regs: Vec<usize>,
}

/// Comprehensive Debug Information for a .cl program
#[derive(Debug, Clone, Default)]
pub struct ClDebugInfo {
    pub program_name: String,
    pub bundle_meta: HashMap<usize, BundleDebugMeta>,
    pub symbols: HashMap<usize, RegisterSymbol>,
    pub breakpoints: HashSet<usize>,
    pub register_file: [u32; 16],
    pub current_cycle: usize,
    pub total_cycles_executed: usize,
    pub is_halted: bool,
}

impl ClDebugInfo {
    pub fn new(program_name: &str) -> Self {
        Self {
            program_name: program_name.to_string(),
            bundle_meta: HashMap::new(),
            symbols: HashMap::new(),
            breakpoints: HashSet::new(),
            register_file: [0u32; 16],
            current_cycle: 0,
            total_cycles_executed: 0,
            is_halted: false,
        }
    }

    /// Set a hardware execution breakpoint at a specific cycle
    pub fn set_breakpoint(&mut self, cycle: usize) {
        self.breakpoints.insert(cycle);
    }

    /// Remove a breakpoint
    pub fn clear_breakpoint(&mut self, cycle: usize) {
        self.breakpoints.remove(&cycle);
    }

    /// Check if the cycle triggers a breakpoint
    pub fn is_breakpoint(&self, cycle: usize) -> bool {
        self.breakpoints.contains(&cycle)
    }

    /// Register a variable symbol mapped to a physical register
    pub fn bind_symbol(&mut self, reg_index: usize, name: &str, type_hint: &str) {
        self.symbols.insert(
            reg_index,
            RegisterSymbol {
                reg_index,
                name: name.to_string(),
                type_hint: type_hint.to_string(),
                last_updated_cycle: self.current_cycle,
            },
        );
    }

    /// Lookup variable name bound to a physical register
    pub fn get_symbol_name(&self, reg_index: usize) -> Option<&str> {
        self.symbols.get(&reg_index).map(|s| s.name.as_str())
    }

    /// Read physical register value
    pub fn read_reg(&self, reg_index: usize) -> u32 {
        if reg_index < 16 {
            self.register_file[reg_index]
        } else {
            0
        }
    }

    /// Write physical register value
    pub fn write_reg(&mut self, reg_index: usize, val: u32) {
        if reg_index < 16 {
            self.register_file[reg_index] = val;
            if let Some(sym) = self.symbols.get_mut(&reg_index) {
                sym.last_updated_cycle = self.current_cycle;
            }
        }
    }

    /// Parse .cl source code and automatically generate debug metadata
    pub fn extract_from_cl(cl_code: &str) -> Self {
        let mut debug = Self::new("cl_program");
        let mut current_label = "entry".to_string();
        let mut line_num = 0;

        for line in cl_code.lines() {
            line_num += 1;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with(';') || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.starts_with('@') && trimmed.ends_with(':') {
                current_label = trimmed.trim_start_matches('@').trim_end_matches(':').to_string();
                continue;
            }

            if let Some((cycle_part, slots_part)) = trimmed.split_once(':') {
                if let Ok(cycle) = cycle_part.trim().trim_start_matches('B').parse::<usize>() {
                    let tokens: Vec<&str> = slots_part.split_whitespace().collect();
                    let mut annotations = [String::new(), String::new(), String::new(), String::new()];
                    let mut written = Vec::new();
                    let mut read = Vec::new();

                    for (i, tok) in tokens.iter().enumerate().take(4) {
                        if let Ok(slot) = parse_slot(tok) {
                            annotations[i] = format!("{}[{}]", slot.opcode, slot.mode);
                            if let Some(d) = slot.dest_reg {
                                written.push(d);
                            }
                            if let Some(s) = slot.src_reg {
                                read.push(s);
                            }
                        }
                    }

                    debug.bundle_meta.insert(
                        cycle,
                        BundleDebugMeta {
                            cycle,
                            source_loc: Some(SourceLocation {
                                file: "inline.cl".to_string(),
                                line: line_num,
                                column: 1,
                                symbol: current_label.clone(),
                            }),
                            slot_annotations: annotations,
                            written_regs: written,
                            read_regs: read,
                        },
                    );
                }
            }
        }

        debug
    }

    /// Step execution through one VLIW cycle bundle
    pub fn step_cycle(&mut self, bundle: &ClBundle) -> bool {
        self.current_cycle = bundle.cycle;
        self.total_cycles_executed += 1;

        for slot in &bundle.slots {
            if slot.opcode == "HL" {
                self.is_halted = true;
                return false;
            }
            if let Some(d) = slot.dest_reg {
                if d < 16 {
                    self.register_file[d] = self.register_file[d].wrapping_add(1);
                }
            }
        }

        !self.is_halted
    }

    /// Render human-readable register inspection table
    pub fn render_register_table(&self) -> String {
        let mut out = String::new();
        out.push_str("============================================================\n");
        out.push_str(&format!("  CRON .cl DEBUGGER INSPECTION | Cycle: B{:04}\n", self.current_cycle));
        out.push_str("============================================================\n");
        out.push_str("  Register | Value (Hex) | Value (Dec) | Bound Symbol\n");
        out.push_str("  ---------+-------------+-------------+--------------------\n");
        for r in 0..16 {
            let val = self.register_file[r];
            let sym = self.symbols.get(&r).map(|s| s.name.as_str()).unwrap_or("-");
            out.push_str(&format!(
                "  R{:02} (R{:1X}) | 0x{:08X}  | {: >11} | {}\n",
                r, r, val, val, sym
            ));
        }
        out.push_str("============================================================\n");
        out
    }
}
