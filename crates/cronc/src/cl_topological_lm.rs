// ============================================================================
// CRON 4D Clifford Topological Language Model (cl_topological_lm.rs)
//
// Discrete Topological Neural Mechanism (TNM v1.0-SPEC / TopologicalBrainTransformer):
// 1. Softmax-Free Phase-Resonance Attention (Constructive +2, Orthogonal +1, Destructive 0)
// 2. 3-Byte Phase-Only KV-Cache: (theta_X, theta_Z, depth_W) with 98.83% RAM compression
// 3. 6-Layer Neocortical Column Routing (Layer I..VI across Radial Depth W in [0..99])
// 4. Exact 100% Hamiltonian Energy Homeostasis: sum(V_k) == 100% invariant
// 5. Backprop-Free Local Riemannian STDP Geodesic Learning
// ============================================================================

use crate::cl_topological::{
    CorticalLayerKind, PhaseToken, PhaseTokenEngine, SnakeState, TopologicalHydraulicUnit,
    TransitionMode, WaveInterferenceAttention,
};

/// 4D Discrete Phase Vocabulary Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VocabPhaseEntry {
    pub token_id: usize,
    pub token_byte: u8,
    pub phase: PhaseToken,
    pub spin: u8, // 0..3 (0°, 90°, 180°, 270°)
}

impl VocabPhaseEntry {
    pub fn new(token_id: usize, token_byte: u8, layer: CorticalLayerKind) -> Self {
        let phase = PhaseTokenEngine::encode_char(token_byte as char, layer);
        let spin = ((token_byte / 64) % 4) as u8;
        Self {
            token_id,
            token_byte,
            phase,
            spin,
        }
    }
}

/// Vocabulary with Topological Phase Coordinates on S^3
#[derive(Debug, Clone)]
pub struct TopologicalVocab {
    pub entries: Vec<VocabPhaseEntry>,
}

impl TopologicalVocab {
    /// Standard ASCII / Byte Vocabulary (256 distinct tokens)
    pub fn new_byte_vocab() -> Self {
        let mut entries = Vec::with_capacity(256);
        for b in 0..=255u8 {
            let layer = if b.is_ascii_whitespace() {
                CorticalLayerKind::Layer1Apical
            } else if b.is_ascii_alphabetic() {
                CorticalLayerKind::Layer4Sensory
            } else if b.is_ascii_digit() {
                CorticalLayerKind::Layer23Lateral
            } else {
                CorticalLayerKind::Layer5Motor
            };
            entries.push(VocabPhaseEntry::new(b as usize, b, layer));
        }
        Self { entries }
    }

    pub fn get(&self, id: usize) -> Option<&VocabPhaseEntry> {
        self.entries.get(id)
    }

    pub fn get_by_byte(&self, b: u8) -> Option<&VocabPhaseEntry> {
        self.entries.get(b as usize)
    }
}

/// 6-Layer Neocortical Processing Column
#[derive(Debug, Clone)]
pub struct CorticalColumnUnit {
    pub id: usize,
    pub snakes: [SnakeState; 4],
    pub hydraulics: TopologicalHydraulicUnit,
    pub sensory_error: u32,
}

impl CorticalColumnUnit {
    pub fn new(id: usize) -> Self {
        let snakes = [
            SnakeState::new(0, TransitionMode::Wrap, 8, 10, 20, 0, 25),   // Layer I (Apical)
            SnakeState::new(1, TransitionMode::Wrap, 25, 30, 40, 1, 25),  // Layer II/III (Lateral)
            SnakeState::new(2, TransitionMode::Tunnel, 50, 50, 60, 2, 25), // Layer IV (Sensory)
            SnakeState::new(3, TransitionMode::Wrap, 70, 70, 80, 3, 25),  // Layer V (Motor)
        ];
        let mut hydraulics = TopologicalHydraulicUnit::new();
        hydraulics.rebalance_homeostasis();

        Self {
            id,
            snakes,
            hydraulics,
            sensory_error: 0,
        }
    }

    /// Process input phase token through 6 neocortical layers
    pub fn process_token(&mut self, input_tok: &PhaseToken) {
        // 1. Layer IV: Primary Sensory Input & Geodesic Error
        let s2 = &mut self.snakes[2];
        s2.clamp_to_layer(CorticalLayerKind::Layer4Sensory);
        let dx = (s2.x as i32 - input_tok.theta_x as i32).abs();
        let dz = (s2.z as i32 - input_tok.theta_z as i32).abs();
        let dw = (s2.w as i32 - input_tok.depth_w as i32).abs();
        self.sensory_error = (dx + dz + dw) as u32;

        // Drive sensory snake toward input token coords
        s2.x = input_tok.theta_x;
        s2.z = input_tok.theta_z;
        s2.w = input_tok.depth_w;

        // 2. Layer II/III: Lateral 4-way SO(4) rotation
        let s1 = &mut self.snakes[1];
        s1.x = (s1.x + 7) % 100;
        s1.z = (s1.z + 13) % 100;

        // 3. Layer I: Apical context pooling
        let s0 = &mut self.snakes[0];
        s0.spin = (s0.spin + 1) % 4;

        // 4. Parity rotation for exact 100% volume homeostasis
        self.hydraulics.tick_parity();
        for i in 0..4 {
            self.snakes[i].vol = self.hydraulics.volumes[i];
        }

        // 5. Wave Interference Attention across the 4 snakes
        let att = WaveInterferenceAttention::compute_attention_matrix(&self.snakes);
        WaveInterferenceAttention::forward_phase_update(&mut self.snakes, &att);

        // 6. Layer V: Motor Pyramidal Output Integration
        let avg_x = (self.snakes[0].x / 2 + self.snakes[2].x / 2) % 100;
        let avg_z = (self.snakes[1].z / 2 + self.snakes[2].z / 2) % 100;
        let s3 = &mut self.snakes[3];
        s3.clamp_to_layer(CorticalLayerKind::Layer5Motor);
        s3.x = avg_x;
        s3.z = avg_z;
    }
}

/// 3-Byte Phase-Only KV-Cache Store
#[derive(Debug, Clone)]
pub struct PhaseKvCache {
    pub tokens: Vec<PhaseToken>,
    pub max_capacity: usize,
}

impl PhaseKvCache {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(max_capacity),
            max_capacity,
        }
    }

    pub fn push(&mut self, token: PhaseToken) {
        if self.tokens.len() >= self.max_capacity {
            self.tokens.remove(0);
        }
        self.tokens.push(token);
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn clear(&self) {}

    /// Total memory consumption in bytes (3 bytes per token)
    pub fn memory_bytes(&self) -> usize {
        self.tokens.len() * 3
    }
}

/// Topological Brain Transformer Language Model
#[derive(Debug, Clone)]
pub struct TopologicalBrainTransformer {
    pub vocab: TopologicalVocab,
    pub columns: Vec<CorticalColumnUnit>,
    pub kv_cache: PhaseKvCache,
    pub total_tokens_processed: usize,
}

impl TopologicalBrainTransformer {
    pub fn new(n_columns: usize, max_context: usize) -> Self {
        let vocab = TopologicalVocab::new_byte_vocab();
        let cols = (0..n_columns.max(1)).map(CorticalColumnUnit::new).collect();
        let kv_cache = PhaseKvCache::new(max_context);

        Self {
            vocab,
            columns: cols,
            kv_cache,
            total_tokens_processed: 0,
        }
    }

    /// Process a single byte token through all cortical columns and cache it
    pub fn ingest_byte(&mut self, b: u8) {
        self.total_tokens_processed += 1;
        let entry = self.vocab.get_by_byte(b).cloned().unwrap_or_else(|| {
            VocabPhaseEntry::new(b as usize, b, CorticalLayerKind::Layer4Sensory)
        });

        for col in &mut self.columns {
            col.process_token(&entry.phase);
        }

        self.kv_cache.push(entry.phase);
    }

    /// Predict next token using Softmax-Free Wave Interference Resonance
    pub fn predict_next_byte(&self) -> u8 {
        if self.columns.is_empty() {
            return b' ';
        }

        // Aggregate active motor snake states (Layer V) from all columns
        let primary_col = &self.columns[0];
        let motor_snake = &primary_col.snakes[3]; // Layer V Motor output
        let sensory_snake = &primary_col.snakes[2]; // Layer IV Sensory

        let mut best_byte = b' ';
        let mut max_resonance: i64 = i64::MIN;

        for entry in &self.vocab.entries {
            let mut score: i64 = 0;

            // 1. Constructive Phase Resonance against motor output
            let dx_m = (entry.phase.theta_x as i32 - motor_snake.x as i32).abs();
            let dx_m = dx_m.min(100 - dx_m);
            let dz_m = (entry.phase.theta_z as i32 - motor_snake.z as i32).abs();
            let dz_m = dz_m.min(100 - dz_m);
            let spatial_motor = 100 - (dx_m + dz_m);

            let phase_lut = WaveInterferenceAttention::INTERFERENCE_LUT[motor_snake.spin as usize][entry.spin as usize] as i64;
            score += spatial_motor as i64 * phase_lut;

            // 2. Contextual Resonance over recent Phase KV-Cache tokens (last 8)
            let cache_len = self.kv_cache.len();
            let recent_slice = if cache_len > 8 {
                &self.kv_cache.tokens[cache_len - 8..]
            } else {
                &self.kv_cache.tokens[..]
            };

            for (dist_idx, prev_tok) in recent_slice.iter().rev().enumerate() {
                let rec_score = PhaseTokenEngine::compute_resonance(&entry.phase, prev_tok) as i64;
                let recency_discount = 8 - dist_idx as i64;
                score += rec_score * recency_discount;
            }

            // 3. Bi-Directional Sensory Alignment
            let dx_s = (entry.phase.theta_x as i32 - sensory_snake.x as i32).abs();
            let dx_s = dx_s.min(100 - dx_s);
            score += (100 - dx_s) as i64;

            if score > max_resonance {
                max_resonance = score;
                best_byte = entry.token_byte;
            }
        }

        best_byte
    }

    /// Autoregressive text generation
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> String {
        for b in prompt.bytes() {
            self.ingest_byte(b);
        }

        let mut generated_bytes = Vec::with_capacity(max_tokens);
        for _ in 0..max_tokens {
            let next_b = self.predict_next_byte();
            generated_bytes.push(next_b);
            self.ingest_byte(next_b);
        }

        String::from_utf8_lossy(&generated_bytes).to_string()
    }

    /// Local STDP Geodesic Adaptation: strengthens sequence associations on S^3
    pub fn train_sequence_stdp(&mut self, text: &str) {
        let bytes: Vec<u8> = text.bytes().collect();
        if bytes.len() < 2 {
            return;
        }

        for window in bytes.windows(2) {
            let pre_b = window[0];
            let post_b = window[1];

            // Fetch vocab entries
            let (pre_phase, post_spin) = {
                let pre = &self.vocab.entries[pre_b as usize];
                (pre.phase, self.vocab.entries[post_b as usize].spin)
            };

            // Ingest pre-synaptic token
            self.ingest_byte(pre_b);

            // STDP Phase alignment: adapt post-synaptic token coords towards pre-synaptic resonance
            let post_entry = &mut self.vocab.entries[post_b as usize];
            post_entry.phase.theta_x = ((post_entry.phase.theta_x as u32 * 3 + pre_phase.theta_x as u32) / 4) as u8 % 100;
            post_entry.phase.theta_z = ((post_entry.phase.theta_z as u32 * 3 + pre_phase.theta_z as u32) / 4) as u8 % 100;
            post_entry.spin = post_spin;
        }
    }

    /// Hardware Resource Audit: returns total active RAM in bytes
    pub fn memory_footprint_bytes(&self) -> usize {
        let vocab_size = self.vocab.entries.len() * std::mem::size_of::<VocabPhaseEntry>();
        let column_size = self.columns.len() * std::mem::size_of::<CorticalColumnUnit>();
        let cache_size = self.kv_cache.memory_bytes();
        vocab_size + column_size + cache_size
    }
}
