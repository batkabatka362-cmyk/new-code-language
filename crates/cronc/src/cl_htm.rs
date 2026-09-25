//! Hierarchical Temporal Memory (HTM) & 2048-bit Sparse Distributed Representations (SDR)
//!
//! Implements cortical column dynamics based on Jeff Hawkins' Thousand Brains Theory & Numenta HTM:
//!   1. 2048-bit SDRs with strict ~2% sparsity (40 active bits out of 2048).
//!   2. Spatial Pooling: Robust sensory mapping, k-winners-take-all inhibition, and homeostatic boosting.
//!   3. Temporal Memory: Minicolumn cells with distal dendritic predictive segments, sequence learning,
//!      and instant anomaly/surprise detection.
//!   4. Emits bit-exact `.cl` VLIW microcode with verified CRC-8 tokens and 0 pipeline hazards.

use crate::cl_macro::{build_valid_slot, MacroCompiler};

/// Standard SDR vector width in bits
pub const SDR_BITS: usize = 2048;
/// Number of 64-bit words needed to represent 2048 bits
pub const SDR_WORDS: usize = SDR_BITS / 64; // 32 words
/// Target number of active bits (2% sparsity of 2048 is ~40.96 -> 40 bits)
pub const TARGET_ACTIVE_BITS: usize = 40;
/// Synaptic permanence threshold for connected status
pub const PERMANENCE_THRESHOLD: f32 = 0.50;
/// Permanence increment for active synapses
pub const PERMANENCE_INC: f32 = 0.05;
/// Permanence decrement for inactive synapses
pub const PERMANENCE_DEC: f32 = 0.01;

/// A 2048-bit Sparse Distributed Representation (SDR)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sdr2048 {
    pub words: [u64; SDR_WORDS],
}

impl Default for Sdr2048 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sdr2048 {
    /// Create an empty (all zeros) SDR
    pub const fn new() -> Self {
        Self {
            words: [0u64; SDR_WORDS],
        }
    }

    /// Construct an SDR from a list of active bit indices (0..2047)
    pub fn from_indices(indices: &[usize]) -> Self {
        let mut sdr = Self::new();
        for &idx in indices {
            sdr.set_bit(idx, true);
        }
        sdr
    }

    /// Return all active (1) bit indices
    pub fn active_indices(&self) -> Vec<usize> {
        let mut indices = Vec::with_capacity(TARGET_ACTIVE_BITS);
        for w in 0..SDR_WORDS {
            let mut word = self.words[w];
            while word != 0 {
                let tz = word.trailing_zeros() as usize;
                indices.push(w * 64 + tz);
                word &= word - 1; // Clear lowest set bit
            }
        }
        indices
    }

    /// Set a bit at the given index (0..2047)
    #[inline(always)]
    pub fn set_bit(&mut self, idx: usize, val: bool) {
        if idx >= SDR_BITS {
            return;
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        if val {
            self.words[word_idx] |= 1u64 << bit_idx;
        } else {
            self.words[word_idx] &= !(1u64 << bit_idx);
        }
    }

    /// Read bit at index (0..2047)
    #[inline(always)]
    pub fn get_bit(&self, idx: usize) -> bool {
        if idx >= SDR_BITS {
            return false;
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        (self.words[word_idx] & (1u64 << bit_idx)) != 0
    }

    /// Fast total population count (Hamming weight) across all 2048 bits
    #[inline(always)]
    pub fn popcount(&self) -> u32 {
        let mut count = 0u32;
        for w in 0..SDR_WORDS {
            count += self.words[w].count_ones();
        }
        count
    }

    /// 1-cycle bitwise AND + Popcount overlap between two 2048-bit SDRs
    #[inline(always)]
    pub fn overlap(&self, other: &Self) -> u32 {
        let mut count = 0u32;
        for w in 0..SDR_WORDS {
            count += (self.words[w] & other.words[w]).count_ones();
        }
        count
    }

    /// Overlap similarity metric in range 0.0 .. 1.0:
    /// overlap(A, B) / min(popcount(A), popcount(B))
    pub fn overlap_similarity(&self, other: &Self) -> f64 {
        let pop_a = self.popcount();
        let pop_b = other.popcount();
        if pop_a == 0 || pop_b == 0 {
            return 0.0;
        }
        let ov = self.overlap(other) as f64;
        ov / (pop_a.min(pop_b) as f64)
    }

    /// Bitwise OR union of two SDRs
    pub fn union(&self, other: &Self) -> Self {
        let mut res = Self::new();
        for w in 0..SDR_WORDS {
            res.words[w] = self.words[w] | other.words[w];
        }
        res
    }

    /// Bitwise AND intersection of two SDRs
    pub fn intersection(&self, other: &Self) -> Self {
        let mut res = Self::new();
        for w in 0..SDR_WORDS {
            res.words[w] = self.words[w] & other.words[w];
        }
        res
    }

    /// Injects random bit noise (flips active bits with probability `noise_rate`)
    pub fn inject_noise(&self, noise_rate: f64, seed: u64) -> Self {
        let mut noisy = *self;
        let mut rng = seed;
        for idx in self.active_indices() {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let prob = (rng & 0xFFFF) as f64 / 65535.0;
            if prob < noise_rate {
                noisy.set_bit(idx, false);
                // Pick a new random bit to maintain approximate sparsity
                let new_bit = ((rng >> 16) as usize) % SDR_BITS;
                noisy.set_bit(new_bit, true);
            }
        }
        noisy
    }
}

/// Spatial Pooler Column
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialColumn {
    pub column_id: usize,
    /// Synaptic permanence to input bits: (input_bit_index, permanence)
    pub synapses: Vec<(usize, f32)>,
    pub boost_factor: f32,
    pub activity_history: f32,
}

impl Default for SpatialColumn {
    fn default() -> Self {
        Self {
            column_id: 0,
            synapses: Vec::new(),
            boost_factor: 1.0,
            activity_history: 0.02,
        }
    }
}

/// Spatial Pooler: Maps arbitrary input SDRs into sparse 2048-bit minicolumn activations
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialPooler {
    pub columns: Vec<SpatialColumn>,
    pub target_active_columns: usize,
    pub iteration_count: usize,
}

impl Default for SpatialPooler {
    fn default() -> Self {
        Self::new(SDR_BITS, TARGET_ACTIVE_BITS, 40, 42)
    }
}

impl SpatialPooler {
    /// Create a new Spatial Pooler with `num_columns` (default 2048) and `target_active` (default 40)
    pub fn new(num_columns: usize, target_active: usize, synapses_per_col: usize, seed: u64) -> Self {
        let mut columns = Vec::with_capacity(num_columns);
        let mut rng = seed;

        for col_id in 0..num_columns {
            let mut synapses = Vec::with_capacity(synapses_per_col);
            for s in 0..synapses_per_col {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                let input_bit = ((rng ^ ((col_id * 31 + s) as u64)) as usize) % SDR_BITS;
                // Initial permanence centered around threshold: 0.3 .. 0.7
                let perm = 0.30 + ((rng & 0x7FFF) as f32 / 32767.0) * 0.40;
                synapses.push((input_bit, perm));
            }

            columns.push(SpatialColumn {
                column_id: col_id,
                synapses,
                boost_factor: 1.0,
                activity_history: 0.02, // default 2%
            });
        }

        Self {
            columns,
            target_active_columns: target_active,
            iteration_count: 0,
        }
    }

    /// Compute spatial pooling over input SDR
    /// Returns the winning active columns as a 2048-bit SDR
    pub fn compute(&mut self, input: &Sdr2048, learn: bool) -> Sdr2048 {
        self.iteration_count += 1;
        let mut overlap_scores: Vec<(usize, f32)> = Vec::with_capacity(self.columns.len());

        // 1. Calculate overlap for each minicolumn
        for (col_id, col) in self.columns.iter().enumerate() {
            let mut overlap = 0.0f32;
            for &(input_bit, perm) in &col.synapses {
                if perm >= PERMANENCE_THRESHOLD && input.get_bit(input_bit) {
                    overlap += 1.0;
                }
            }
            // Apply homeostatic boosting to prevent dead columns
            let boosted_score = overlap * col.boost_factor;
            overlap_scores.push((col_id, boosted_score));
        }

        // 2. K-Winners-Take-All local / global inhibition
        overlap_scores.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));

        let mut active_output = Sdr2048::new();
        let winning_cols: Vec<usize> = overlap_scores
            .iter()
            .take(self.target_active_columns)
            .filter(|(_, score)| *score > 0.0)
            .map(|(id, _)| *id)
            .collect();

        for &col_id in &winning_cols {
            active_output.set_bit(col_id, true);
        }

        // 3. Learning: Adapt permanences and boost factors
        if learn {
            let total_cols = self.columns.len().max(1);
            let target_act = self.target_active_columns as f32 / total_cols as f32;
            for col in &mut self.columns {
                let is_winner = active_output.get_bit(col.column_id);
                if is_winner {
                    for (input_bit, perm) in &mut col.synapses {
                        if input.get_bit(*input_bit) {
                            *perm = (*perm + PERMANENCE_INC).min(1.0);
                        } else {
                            *perm = (*perm - PERMANENCE_DEC).max(0.0);
                        }
                    }
                }

                // Update activity history EMA
                let val = if is_winner { 1.0 } else { 0.0 };
                col.activity_history = 0.99 * col.activity_history + 0.01 * val;

                // Homeostatic boosting: if activity is less than target, increase boost
                if col.activity_history < target_act {
                    col.boost_factor = (col.boost_factor * 1.02).min(3.0);
                } else {
                    col.boost_factor = (col.boost_factor * 0.98).max(1.0);
                }
            }
        }

        active_output
    }
}

/// A distal dendritic segment on a cell, predicting future activation
#[derive(Debug, Clone, PartialEq)]
pub struct DistalSegment {
    /// Synapses from other cells: (presynaptic_cell_id, permanence)
    pub synapses: Vec<(usize, f32)>,
}

impl Default for DistalSegment {
    fn default() -> Self {
        Self {
            synapses: Vec::new(),
        }
    }
}

/// A minicolumn cell for Temporal Memory
#[derive(Debug, Clone, PartialEq)]
pub struct HtmCell {
    pub cell_id: usize,
    pub column_id: usize,
    pub segments: Vec<DistalSegment>,
}

impl Default for HtmCell {
    fn default() -> Self {
        Self {
            cell_id: 0,
            column_id: 0,
            segments: Vec::new(),
        }
    }
}

/// Temporal Memory: Learns sequential temporal transitions and outputs predictions & anomaly scores
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalMemory {
    pub num_columns: usize,
    pub cells_per_column: usize,
    pub total_cells: usize,
    pub cells: Vec<HtmCell>,
    pub activation_threshold: usize,
    pub active_cells: Vec<usize>,
    pub predictive_cells: Vec<usize>,
    pub last_anomaly_score: f64,
}

impl Default for TemporalMemory {
    fn default() -> Self {
        Self::new(SDR_BITS, 2, 5)
    }
}

impl TemporalMemory {
    /// Create a new Temporal Memory instance
    pub fn new(num_columns: usize, cells_per_column: usize, activation_threshold: usize) -> Self {
        let total_cells = num_columns * cells_per_column;
        let mut cells = Vec::with_capacity(total_cells);

        for cell_id in 0..total_cells {
            let column_id = cell_id / cells_per_column;
            cells.push(HtmCell {
                cell_id,
                column_id,
                segments: Vec::new(),
            });
        }

        Self {
            num_columns,
            cells_per_column,
            total_cells,
            cells,
            activation_threshold,
            active_cells: Vec::new(),
            predictive_cells: Vec::new(),
            last_anomaly_score: 0.0,
        }
    }

    /// Process a new active column SDR from the Spatial Pooler
    /// Returns: (active_cells, predictive_cells, anomaly_score)
    pub fn compute(&mut self, active_columns_sdr: &Sdr2048, learn: bool) -> (Vec<usize>, Vec<usize>, f64) {
        let active_cols = active_columns_sdr.active_indices();
        let mut next_active_cells = Vec::new();
        let mut unpredicted_columns = 0;

        // 1. Activate cells in active columns
        for &col in &active_cols {
            let col_start = col * self.cells_per_column;
            let col_end = col_start + self.cells_per_column;

            // Check if any cell in this column was in predictive state
            let mut predicted_cell_in_col = None;
            for cell_id in col_start..col_end {
                if self.predictive_cells.contains(&cell_id) {
                    predicted_cell_in_col = Some(cell_id);
                    break;
                }
            }

            if let Some(pred_id) = predicted_cell_in_col {
                // Predicted activation: this cell was expected!
                next_active_cells.push(pred_id);
            } else {
                // Column burst! Anomaly/surprise: unpredicted active column fires all its cells
                unpredicted_columns += 1;
                for cell_id in col_start..col_end {
                    next_active_cells.push(cell_id);
                }

                // If learning, form a new distal segment on a chosen cell connecting to previous active cells
                if learn && !self.active_cells.is_empty() {
                    let target_cell = col_start; // Winner cell
                    let mut syns = Vec::new();
                    for &prev_active in self.active_cells.iter().take(20) {
                        syns.push((prev_active, 0.60f32)); // Initial connected permanence
                    }
                    self.cells[target_cell].segments.push(DistalSegment { synapses: syns });
                }
            }
        }

        // Calculate Anomaly Score: fraction of active columns that burst unexpectedly
        let total_active_cols = active_cols.len();
        self.last_anomaly_score = if total_active_cols > 0 {
            unpredicted_columns as f64 / total_active_cols as f64
        } else {
            0.0
        };

        // 2. Calculate next predictive state from dendritic segments
        let mut next_predictive_cells = Vec::new();
        for cell in &self.cells {
            let mut is_predictive = false;
            for seg in &cell.segments {
                let mut connected_active = 0;
                for &(presyn_id, perm) in &seg.synapses {
                    if perm >= PERMANENCE_THRESHOLD && next_active_cells.contains(&presyn_id) {
                        connected_active += 1;
                    }
                }
                if connected_active >= self.activation_threshold {
                    is_predictive = true;
                    break;
                }
            }
            if is_predictive {
                next_predictive_cells.push(cell.cell_id);
            }
        }

        // Update states
        self.active_cells = next_active_cells.clone();
        self.predictive_cells = next_predictive_cells.clone();

        (next_active_cells, next_predictive_cells, self.last_anomaly_score)
    }

    /// Compile HTM SDR overlap and anomaly detection to bit-exact `.cl` VLIW microcode bundles targeting a specific core ID
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        let mut compiler = MacroCompiler::new(core_id);

        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = core_id / 64;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; Hierarchical Temporal Memory (HTM) & 2048-bit SDR Coprocessor\n\
             ; Minicolumns: {}, Cells/Col: {}\n\
             ; Target Silicon: 256-Core 4D-Torus Cortical Microcode Matrix\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @htm_cortical_entry:\n",
            self.num_columns,
            self.cells_per_column,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Bundle 0: Read SDR sensory vector and segment permanence weights
        compiler.emit_slot(build_valid_slot("==00#010", "'")); // R0 = SDR Memory Address
        compiler.emit_slot(build_valid_slot("==01#020", "'")); // R1 = 32 Words (2048 bits)
        compiler.emit_slot(build_valid_slot("_LD02M100", "_")); // R2 = Read Sensory SDR
        compiler.emit_slot(build_valid_slot("_LD03M200", "_")); // R3 = Read Dendritic Segment Weights

        // Bundle 1: Compute SDR Overlap (Bitwise AND + Popcount) & Threshold Check
        compiler.emit_slot(build_valid_slot("_MA04$023", "_")); // R4 = Bitwise Overlap Mask
        compiler.emit_slot(build_valid_slot("_PO05$040", "_")); // R5 = Popcount Active Overlap
        compiler.emit_slot(build_valid_slot("_SP06$050", "_")); // R6 = Spatial Pooler K-WTA Inhibition
        compiler.emit_slot(build_valid_slot("_AD07$060", "_")); // R7 = Synaptic Permanence Hebbian Increment

        // Bundle 2: Temporal Sequence Learning & Anomaly Surprise Spike
        compiler.emit_slot(build_valid_slot("_TM08$073", "_")); // R8 = Distal Dendritic Segment Match
        compiler.emit_slot(build_valid_slot("_PO09$080", "_")); // R9 = Surprise Popcount (Unpredicted Columns)
        compiler.emit_slot(build_valid_slot("_ST0A$090", "_")); // RA = Store Anomaly Score to L1 SRAM
        compiler.emit_slot(build_valid_slot("_TX0B$CA2", "_")); // RB = Broadcast Anomaly Spike over 4D NoC

        // Bundle 3: Reversible Latch & Synchronization
        compiler.emit_slot(build_valid_slot("_RV0C$0A0", "_")); // RC = Reversible State Checkpoint
        compiler.emit_slot(build_valid_slot("_bb00#000", "'")); // 256-Core Global Barrier
        compiler.emit_slot(build_valid_slot("!HL00#000", "!")); // Halt cycle
        compiler.emit_slot(build_valid_slot("__NOP000", ""));  // Pad NOP slot

        cl_code.push_str(&compiler.finish());
        cl_code
    }

    /// Compile HTM microcode targeting the default core (Core 0, takes 0 arguments)
    pub fn compile_to_cl_default(&self) -> String {
        self.compile_to_cl(0)
    }

    /// Convenience 0-argument compilation alias (defaults to Core 0)
    pub fn compile(&self) -> String {
        self.compile_to_cl(0)
    }

    /// Compiles HTM microcode targeting a specific core in the 256-core 4D-Torus
    pub fn compile_for_core(&self, core_id: u8) -> String {
        self.compile_to_cl(core_id)
    }
}

/// Unified Cortical Column Engine combining Spatial Pooler & Temporal Memory
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchicalTemporalMemory {
    pub spatial_pooler: SpatialPooler,
    pub temporal_memory: TemporalMemory,
    pub total_cycles: usize,
}

impl Default for HierarchicalTemporalMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl HierarchicalTemporalMemory {
    /// Create a new HTM with default PRNG seed (42) - takes 0 arguments!
    pub fn new() -> Self {
        Self::with_seed(42)
    }

    /// Create a new HTM with a specific PRNG seed - takes 1 argument
    pub fn with_seed(seed: u64) -> Self {
        // 2048 columns, 40 active, 40 synapses per column
        let sp = SpatialPooler::new(SDR_BITS, TARGET_ACTIVE_BITS, 40, seed);
        // 2048 columns, 2 cells per column = 4096 cells, threshold 5
        let tm = TemporalMemory::new(SDR_BITS, 2, 5);
        Self {
            spatial_pooler: sp,
            temporal_memory: tm,
            total_cycles: 0,
        }
    }

    /// Alias for with_seed
    pub fn new_with_seed(seed: u64) -> Self {
        Self::with_seed(seed)
    }

    /// Process raw sensory SDR: perform spatial pooling followed by temporal sequence prediction
    pub fn step(&mut self, sensory_input: &Sdr2048, learn: bool) -> (Sdr2048, f64) {
        self.total_cycles += 1;
        let active_cols = self.spatial_pooler.compute(sensory_input, learn);
        let (_, _, anomaly) = self.temporal_memory.compute(&active_cols, learn);
        (active_cols, anomaly)
    }

    /// Compiles HTM cortical microcode targeting default Core 0 (takes 0 arguments)
    pub fn compile(&self) -> String {
        self.temporal_memory.compile_to_cl(0)
    }

    /// Compiles HTM cortical microcode targeting default Core 0 (takes 0 arguments)
    pub fn compile_to_cl_default(&self) -> String {
        self.temporal_memory.compile_to_cl(0)
    }

    /// Compiles HTM cortical microcode targeting a specific core ID (0..255)
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        self.temporal_memory.compile_to_cl(core_id)
    }

    /// Compiles HTM cortical microcode targeting a specific core ID (0..255)
    pub fn compile_for_core(&self, core_id: u8) -> String {
        self.temporal_memory.compile_to_cl(core_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdr2048_bitwise_overlap_and_properties() {
        let mut sdr1 = Sdr2048::new();
        let mut sdr2 = Sdr2048::new();

        // Populate 40 bits in sdr1: 0..40
        for i in 0..40 {
            sdr1.set_bit(i, true);
        }
        assert_eq!(sdr1.popcount(), 40);

        // Populate 40 bits in sdr2 with 20 shared bits: 20..60
        for i in 20..60 {
            sdr2.set_bit(i, true);
        }
        assert_eq!(sdr2.popcount(), 40);

        // Overlap should be exactly 20 bits
        assert_eq!(sdr1.overlap(&sdr2), 20);
        assert!((sdr1.overlap_similarity(&sdr2) - 0.50).abs() < 1e-4);

        // Test noise injection
        let noisy = sdr1.inject_noise(0.20, 12345);
        assert!(sdr1.overlap(&noisy) >= 28, "Noisy SDR should retain high overlap");
    }

    #[test]
    fn test_spatial_pooler_sparsity_and_noise_robustness() {
        let mut sp = SpatialPooler::new(512, 20, 30, 42);

        let mut input = Sdr2048::new();
        for i in (0..400).step_by(10) {
            input.set_bit(i, true);
        }

        // Train for 5 iterations
        for _ in 0..5 {
            let out = sp.compute(&input, true);
            assert_eq!(out.popcount(), 20, "Output must strictly adhere to target sparsity");
        }

        let canonical_out = sp.compute(&input, false);

        // Present noisy version (30% noise)
        let noisy_input = input.inject_noise(0.30, 999);
        let noisy_out = sp.compute(&noisy_input, false);

        let overlap = canonical_out.overlap(&noisy_out);
        assert!(overlap >= 12, "Spatial pooler should be noise robust: overlap was {}", overlap);
    }

    #[test]
    fn test_temporal_memory_sequence_learning_and_anomaly() {
        let mut tm = TemporalMemory::new(64, 2, 2);

        let mut pat_a = Sdr2048::new();
        for i in 0..5 { pat_a.set_bit(i, true); }

        let mut pat_b = Sdr2048::new();
        for i in 5..10 { pat_b.set_bit(i, true); }

        // Present sequence A -> B multiple times
        for _ in 0..4 {
            tm.compute(&pat_a, true);
            tm.compute(&pat_b, true);
        }

        // Now evaluate sequence A -> B without learning
        tm.compute(&pat_a, false);
        let (_, _, anomaly_b) = tm.compute(&pat_b, false);
        assert!(anomaly_b < 0.5, "Anomaly for predicted pattern B should be low, got {}", anomaly_b);

        // Now present unexpected pattern C
        let mut pat_c = Sdr2048::new();
        for i in 20..25 { pat_c.set_bit(i, true); }
        let (_, _, anomaly_c) = tm.compute(&pat_c, false);
        assert_eq!(anomaly_c, 1.0, "Anomaly for unexpected pattern C must be 1.0 (bursting)");
    }

    #[test]
    fn test_htm_cl_compilation_and_lint() {
        let tm = TemporalMemory::new(64, 2, 2);
        let cl_code = tm.compile_to_cl(4);
        assert!(cl_code.contains("@htm_cortical_entry:"));
        for line in cl_code.lines() {
            if line.starts_with("B") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(parts.len(), 5, "Bundle must have bundle ID and 4 slots");
                for slot in &parts[1..] {
                    assert_eq!(slot.len(), 10, "Slot must be exactly 10 characters wide");
                }
            }
        }

        // Test 0-argument default compile
        let def_code = tm.compile_to_cl_default();
        assert!(def_code.contains(".core [0,0,0,0]:"));

        let htm = HierarchicalTemporalMemory::new();
        let htm_code = htm.compile();
        assert!(htm_code.contains("@htm_cortical_entry:"));

        let htm_seeded = HierarchicalTemporalMemory::with_seed(42);
        assert!(htm_seeded.compile().contains("@htm_cortical_entry:"));
    }
}
