//! Neural Model & Tensor Importer for `.cl` VLIW Machine Code
//!
//! Imports neural weight matrices, quantizes them into BitNet ternary weights {-1, 0, +1}
//! or packed 8-bit integers, and maps them across the 256-Core 4D-Torus or 65,536-Core 8D-Torus.

use crate::cl_macro::{MacroCompiler, MacroStmt};

/// Quantized BitNet Ternary Value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryVal {
    NegOne = -1,
    Zero = 0,
    PosOne = 1,
}

impl TernaryVal {
    pub fn from_f32(val: f32, threshold: f32) -> Self {
        if val > threshold {
            TernaryVal::PosOne
        } else if val < -threshold {
            TernaryVal::NegOne
        } else {
            TernaryVal::Zero
        }
    }

    pub fn to_trit_bits(self) -> u8 {
        match self {
            TernaryVal::Zero => 0b00,
            TernaryVal::PosOne => 0b01,
            TernaryVal::NegOne => 0b10,
        }
    }
}

/// A 2D Neural Layer Matrix
#[derive(Debug, Clone)]
pub struct NeuralLayer {
    pub name: String,
    pub in_features: usize,
    pub out_features: usize,
    pub weights: Vec<f32>,
}

impl NeuralLayer {
    pub fn new_random(name: &str, in_features: usize, out_features: usize, seed: u64) -> Self {
        let mut rng = seed;
        let mut weights = Vec::with_capacity(in_features * out_features);
        for _ in 0..(in_features * out_features) {
            // Simple deterministic LCG for reproducible weights
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let val = ((rng >> 32) as i32 % 1000) as f32 / 500.0 - 1.0;
            weights.push(val);
        }
        Self {
            name: name.to_string(),
            in_features,
            out_features,
            weights,
        }
    }

    /// Quantize layer into 1.58-bit ternary matrix
    pub fn quantize_ternary(&self, threshold: f32) -> Vec<TernaryVal> {
        self.weights.iter().map(|&w| TernaryVal::from_f32(w, threshold)).collect()
    }

    /// Packs 16 ternary values into a 32-bit register word
    pub fn pack_16_trits(trits: &[TernaryVal]) -> u32 {
        let mut word = 0u32;
        for (i, &t) in trits.iter().take(16).enumerate() {
            let bits = t.to_trit_bits() as u32;
            word |= bits << (i * 2);
        }
        word
    }

    /// Compiles this neural layer into `.cl` VLIW instructions for a specific Core
    pub fn compile_to_cl(&self, core_id: u8, total_cores: usize) -> String {
        let mut compiler = MacroCompiler::new(core_id);

        let rows_per_core = (self.out_features + total_cores - 1) / total_cores;
        let start_row = (core_id as usize) * rows_per_core;
        let end_row = (start_row + rows_per_core).min(self.out_features);

        let mut stmts = Vec::new();

        // 1. Initialize input activation in R1
        stmts.push(MacroStmt::AssignImm { dst: 1, imm: 0x01FF });
        // 2. Clear accumulator in R0
        stmts.push(MacroStmt::AssignImm { dst: 0, imm: 0x0000 });

        let ternary_weights = self.quantize_ternary(0.25);

        for row in start_row..end_row {
            let row_offset = row * self.in_features;
            let row_weights = &ternary_weights[row_offset..(row_offset + self.in_features).min(ternary_weights.len())];

            // Pack weights in chunks of 16 trits into R2
            for chunk in row_weights.chunks(16) {
                let packed = Self::pack_16_trits(chunk);
                stmts.push(MacroStmt::AssignImm { dst: 2, imm: (packed & 0xFFF) as u16 });
                // Execute ternary MAC into R0
                stmts.push(MacroStmt::TernaryMac { dst: 0, src: 1, weights: 2 });
            }

            // Route intermediate activation to neighbor core via 4D NoC
            let next_core = ((core_id as usize + 1) % total_cores) as u8;
            stmts.push(MacroStmt::AssignReg { dst: 15, src: 0 });
            stmts.push(MacroStmt::SendNoc { dest_core: next_core, data_reg: 15 });
            stmts.push(MacroStmt::Barrier);
        }

        compiler.compile_stmts(&stmts);
        compiler.finish()
    }
}
