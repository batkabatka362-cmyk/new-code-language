//! Interactive Microcode Playground & Live Simulator (`cron cl-playground`)
//!
//! Provides cycle-by-cycle VLIW bundle stepping, live register delta tracking (R0..RF),
//! slot CRC-8 ATM verification badges, and real-time coprocessor execution telemetry.

use crate::cl_jit::{ClJitCore, execute_cl_on_core};
use crate::cl_macro::compute_slot_crc;

/// Snapshot of a single cycle in the playground
#[derive(Debug, Clone)]
pub struct PlaygroundFrame {
    pub cycle: usize,
    pub bundle_raw: String,
    pub slots: Vec<String>,
    pub slots_valid: Vec<bool>,
    pub registers: [u32; 16],
    pub changed_registers: Vec<usize>,
    pub optical_ops: usize,
    pub reversible_ops: usize,
    pub stdp_ops: usize,
    pub ai_isa_ops: usize,
    pub is_halted: bool,
}

/// Interactive Microcode Playground Session
pub struct ClPlaygroundSession {
    pub cl_source: String,
    pub bundles: Vec<String>,
    pub current_cycle: usize,
    pub core: ClJitCore,
    pub prev_registers: [u32; 16],
}

impl ClPlaygroundSession {
    pub fn new(cl_source: &str) -> Self {
        let bundles: Vec<String> = cl_source
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| l.starts_with('B') && l.contains(':'))
            .collect();

        Self {
            cl_source: cl_source.to_string(),
            bundles,
            current_cycle: 0,
            core: ClJitCore::new(),
            prev_registers: [0; 16],
        }
    }

    /// Step exactly one VLIW bundle forward
    pub fn step(&mut self) -> Option<PlaygroundFrame> {
        if self.current_cycle >= self.bundles.len() || self.core.is_halted {
            return None;
        }

        let bundle_line = self.bundles[self.current_cycle].clone();
        self.prev_registers = self.core.r;

        // Execute this single bundle line on the core
        let _ = execute_cl_on_core(&bundle_line, &mut self.core);

        let mut slots = Vec::new();
        let mut slots_valid = Vec::new();

        if let Some((_, slots_part)) = bundle_line.split_once(':') {
            for slot_str in slots_part.split_whitespace() {
                slots.push(slot_str.to_string());
                if slot_str.len() == 10 {
                    let payload = &slot_str[0..9];
                    let expected = slot_str.chars().nth(9).unwrap();
                    let computed = (33 + (compute_slot_crc(payload) % 94)) as char;
                    slots_valid.push(expected == computed);
                } else {
                    slots_valid.push(false);
                }
            }
        }

        let mut changed_registers = Vec::new();
        for i in 0..16 {
            if self.core.r[i] != self.prev_registers[i] {
                changed_registers.push(i);
            }
        }

        let frame = PlaygroundFrame {
            cycle: self.current_cycle,
            bundle_raw: bundle_line,
            slots,
            slots_valid,
            registers: self.core.r,
            changed_registers,
            optical_ops: self.core.optical_gemm_count,
            reversible_ops: self.core.reversible_ops_count,
            stdp_ops: self.core.stdp_updates_count,
            ai_isa_ops: self.core.ai_isa_ops_count,
            is_halted: self.core.is_halted,
        };

        self.current_cycle += 1;
        Some(frame)
    }

    /// Run full simulation and collect all cycle frames
    pub fn run_all(&mut self, max_cycles: usize) -> Vec<PlaygroundFrame> {
        let mut frames = Vec::new();
        let limit = max_cycles.min(self.bundles.len());
        for _ in 0..limit {
            if let Some(f) = self.step() {
                frames.push(f);
                if self.core.is_halted {
                    break;
                }
            } else {
                break;
            }
        }
        frames
    }

    /// Format a playground frame into high-tech ASCII terminal HUD
    pub fn render_frame(frame: &PlaygroundFrame) -> String {
        let mut out = String::new();
        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");
        out.push_str(&format!(
            "| CRON .cl MICROCODE PLAYGROUND — CYCLE {:04} | STATUS: {:<52} |\n",
            frame.cycle,
            if frame.is_halted { "HALTED [HL00]" } else { "EXECUTING (NOMINAL 4.0 IPC)" }
        ));
        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");
        out.push_str(&format!("| VLIW BUNDLE: {:<89} |\n", frame.bundle_raw));
        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");
        
        // Slot indicators
        let mut slot_line = String::new();
        for (i, (s, valid)) in frame.slots.iter().zip(&frame.slots_valid).enumerate() {
            let status_badge = if *valid { "✓ CRC-OK" } else { "✗ CRC-ERR" };
            slot_line.push_str(&format!("Slot {}: {:<10} [{}] | ", i, s, status_badge));
        }
        out.push_str(&format!("| {:<102} |\n", slot_line.trim_end_matches(" | ")));
        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");

        // 16 Core Registers Grid
        out.push_str("| HARDWARE REGISTERS (R0..RF):                                                                           |\n");
        for row in 0..4 {
            let mut row_str = String::new();
            for col in 0..4 {
                let reg_idx = row * 4 + col;
                let val = frame.registers[reg_idx];
                let is_changed = frame.changed_registers.contains(&reg_idx);
                let mark = if is_changed { "*" } else { " " };
                row_str.push_str(&format!("R{:X}{:1}: 0x{:08X} ({:<5}) | ", reg_idx, mark, val, val));
            }
            out.push_str(&format!("|   {:<100} |\n", row_str.trim_end_matches(" | ")));
        }

        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");
        out.push_str(&format!(
            "| COPROCESSORS: Optical GEMMs: {:<4} | Reversible Ops: {:<4} | STDP Updates: {:<4} | AI ISA Ops: {:<6}   |\n",
            frame.optical_ops, frame.reversible_ops, frame.stdp_ops, frame.ai_isa_ops
        ));
        out.push_str("+--------------------------------------------------------------------------------------------------------+\n");

        out
    }
}
