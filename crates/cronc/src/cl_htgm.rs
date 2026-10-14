//! `cl_htgm`: 3-Tier Hierarchical Geometric Rotor Memory (HTGM) for DPSTA & TBT.
//!
//! Based on DPSTA & Topological Brain Transformer (TBT) specification (matrix4.1.2.pdf).
//!
//! Tiers:
//!   - Tier 1 (Micro-scale): Char-level 512-bit rolling n-gram with wire rotation rho^0, rho^1, rho^2.
//!   - Tier 2 (Meso-scale):  Word-level Clifford rotor binding across 4 resonant channels s in {17, 34, 68, 136}.
//!   - Tier 3 (Macro-scale): 16-slot Attractor Ring Buffer with Tropical Phase Resonance (max, +)
//!                          and Next-Token Carrier Registry (resolving the Verilog/Python divergence).

use std::fmt;

pub const D_BITS: usize = 512;
pub const D_U64: usize = D_BITS / 64; // 8
pub const NUM_ANCHORS: usize = 94;
pub const RING_DEPTH: usize = 16;
pub const CHANNEL_SHIFTS: [usize; 4] = [17, 34, 68, 136];
pub const RESONANCE_THRESHOLD: u32 = 272; // > 53% bit correlation

pub type HyperVec512 = [u64; D_U64];

/// Deterministically generates pseudo-orthogonal 512-bit hypervectors for the 94 discrete ASCII states.
pub fn generate_anchors() -> Box<[HyperVec512; NUM_ANCHORS]> {
    let mut anchors = Box::new([[0u64; D_U64]; NUM_ANCHORS]);
    // Deterministic linear congruential pseudo-random generator
    let mut seed: u64 = 0x5EED_4200_1337_7777;
    for state_idx in 0..NUM_ANCHORS {
        for word_idx in 0..D_U64 {
            // Mix 64-bit splitmix
            seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = seed;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            anchors[state_idx][word_idx] = z ^ (z >> 31);
        }
    }
    anchors
}

/// Bitwise XOR binding of two 512-bit vectors.
#[inline(always)]
pub fn xor_bind(a: &HyperVec512, b: &HyperVec512) -> HyperVec512 {
    let mut out = [0u64; D_U64];
    for i in 0..D_U64 {
        out[i] = a[i] ^ b[i];
    }
    out
}

/// Popcount of a 512-bit vector.
#[inline(always)]
pub fn popcount512(vec: &HyperVec512) -> u32 {
    let mut count = 0u32;
    for &w in vec.iter() {
        count += w.count_ones();
    }
    count
}

/// Computes Hamming similarity (512 - HammingDistance).
#[inline(always)]
pub fn hamming_similarity(a: &HyperVec512, b: &HyperVec512) -> u32 {
    let diff = xor_bind(a, b);
    (D_BITS as u32) - popcount512(&diff)
}

/// Cyclic bit-rotation of a 512-bit hypervector across word boundaries.
pub fn wire_rotate(vec: &HyperVec512, shift: usize) -> HyperVec512 {
    let s = shift % D_BITS;
    if s == 0 {
        return *vec;
    }

    let word_shift = s / 64;
    let bit_shift = s % 64;
    let mut out = [0u64; D_U64];

    for i in 0..D_U64 {
        let src_idx = (i + D_U64 - word_shift) % D_U64;
        let prev_idx = (src_idx + D_U64 - 1) % D_U64;

        if bit_shift == 0 {
            out[i] = vec[src_idx];
        } else {
            out[i] = (vec[src_idx] << bit_shift) | (vec[prev_idx] >> (64 - bit_shift));
        }
    }
    out
}

/// An entry in the Tier 3 Attractor Ring Buffer.
#[derive(Clone, Debug, Default)]
pub struct AttractorSlot {
    pub vector: HyperVec512,
    pub next_token: u8,
    pub raw_text: String,
    pub valid: bool,
}

/// Telemetry output from Tier 3 Attractor Resonance.
#[derive(Clone, Debug, Default)]
pub struct AttractorResonanceOutput {
    pub best_slot_index: usize,
    pub resonance_score: u32,
    pub target_token: u8,
    pub tropical_boost: i32,
    pub is_resonant: bool,
}

/// 3-Tier Hierarchical Geometric Rotor Memory (HTGM).
pub struct HierarchicalTBTMemory {
    pub anchors: Box<[HyperVec512; NUM_ANCHORS]>,

    // Tier 1: Micro-scale 3-gram history
    pub char_hist_0: HyperVec512,
    pub char_hist_1: HyperVec512,
    pub char_hist_2: HyperVec512,

    // Tier 2: Meso-scale Word & Sentence Rotor Accumulator
    pub word_acc: HyperVec512,
    pub sentence_acc: HyperVec512,
    pub channel_idx: usize,
    pub current_word_chars: Vec<u8>,
    pub current_sentence_words: Vec<HyperVec512>,
    pub current_sentence_str: String,

    // Tier 3: Macro-scale Attractor Ring Buffer
    pub ring: [AttractorSlot; RING_DEPTH],
    pub ring_head: usize,
    pub ring_occupancy: usize,
}

impl fmt::Debug for HierarchicalTBTMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HierarchicalTBTMemory")
            .field("ring_head", &self.ring_head)
            .field("ring_occupancy", &self.ring_occupancy)
            .finish()
    }
}

impl Default for HierarchicalTBTMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl HierarchicalTBTMemory {
    pub fn new() -> Self {
        Self {
            anchors: generate_anchors(),
            char_hist_0: [0u64; D_U64],
            char_hist_1: [0u64; D_U64],
            char_hist_2: [0u64; D_U64],
            word_acc: [0u64; D_U64],
            sentence_acc: [0u64; D_U64],
            channel_idx: 0,
            current_word_chars: Vec::new(),
            current_sentence_words: Vec::new(),
            current_sentence_str: String::new(),
            ring: std::array::from_fn(|_| AttractorSlot::default()),
            ring_head: 0,
            ring_occupancy: 0,
        }
    }

    /// Computes the Tier 1 rolling 3-gram vector:
    /// rot_0 ^ rot_1 ^ rot_2
    pub fn get_local_ngram_vector(&self) -> HyperVec512 {
        let rot_0 = self.char_hist_0;
        let rot_1 = wire_rotate(&self.char_hist_1, 19);
        let rot_2 = wire_rotate(&self.char_hist_2, 38);
        xor_bind(&xor_bind(&rot_0, &rot_1), &rot_2)
    }

    /// Feeds a character into the 3-Tier memory hierarchy.
    pub fn train_char(&mut self, ch: char) {
        let cid = crate::cl_matrix94::char_to_state(ch) as usize;
        let char_vec = self.anchors[cid];

        // 1. Tier 1: Shift char history
        self.char_hist_0 = self.char_hist_1;
        self.char_hist_1 = self.char_hist_2;
        self.char_hist_2 = char_vec;
        self.current_sentence_str.push(ch);

        let is_word_delim = ch == ' ' || ch == '\t' || ch == '\n' || ch == '_' || ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == ';' || ch == ',';
        let is_sent_delim = ch == '.' || ch == ';' || ch == '!' || ch == '?';

        // 2. Tier 2: Word & Sentence Clifford Rotor Accumulation
        if is_word_delim {
            if !self.current_word_chars.is_empty() {
                // Shift word accumulator through current resonant channel
                let shift = CHANNEL_SHIFTS[self.channel_idx % 4];
                let shifted_word = wire_rotate(&self.word_acc, shift);
                self.sentence_acc = xor_bind(&self.sentence_acc, &shifted_word);
                self.current_sentence_words.push(self.word_acc);
                self.channel_idx = (self.channel_idx + 1) % 4;

                self.word_acc = [0u64; D_U64];
                self.current_word_chars.clear();
            }
        } else {
            self.word_acc = xor_bind(&self.word_acc, &char_vec);
            self.current_word_chars.push(cid as u8);
        }

        // 3. Tier 3: Sentence Attractor Ring Storage
        if is_sent_delim && !self.current_sentence_words.is_empty() {
            let next_token_hint = cid as u8;
            self.ring[self.ring_head] = AttractorSlot {
                vector: self.sentence_acc,
                next_token: next_token_hint,
                raw_text: self.current_sentence_str.clone(),
                valid: true,
            };

            self.ring_head = (self.ring_head + 1) % RING_DEPTH;
            if self.ring_occupancy < RING_DEPTH {
                self.ring_occupancy += 1;
            }

            // Reset accumulators for next sentence
            self.sentence_acc = [0u64; D_U64];
            self.current_sentence_words.clear();
            self.current_sentence_str.clear();
            self.channel_idx = 0;
        }
    }

    /// Feeds an entire string of characters sequentially into the 3-Tier memory hierarchy.
    pub fn train_sequence(&mut self, text: &str) {
        for ch in text.chars() {
            self.train_char(ch);
        }
    }

    /// Probes the 16-slot Attractor Ring using parallel Tropical Phase Resonance.
    pub fn probe_resonance(&self) -> AttractorResonanceOutput {
        let probe_vec = self.get_local_ngram_vector();
        let mut best_score = 0u32;
        let mut best_idx = 0usize;
        let mut target_token = 0u8;

        for (idx, slot) in self.ring.iter().enumerate() {
            if slot.valid {
                let sim = hamming_similarity(&probe_vec, &slot.vector);
                if sim > best_score {
                    best_score = sim;
                    best_idx = idx;
                    target_token = slot.next_token;
                }
            }
        }

        if best_score > RESONANCE_THRESHOLD && self.ring_occupancy > 0 {
            let tropical_boost = ((best_score - 256) as i32) << 4;
            AttractorResonanceOutput {
                best_slot_index: best_idx,
                resonance_score: best_score,
                target_token,
                tropical_boost,
                is_resonant: true,
            }
        } else {
            AttractorResonanceOutput {
                best_slot_index: best_idx,
                resonance_score: best_score,
                target_token: 0,
                tropical_boost: 0,
                is_resonant: false,
            }
        }
    }

    /// Trains on a string segment.
    pub fn train_text(&mut self, text: &str) {
        for ch in text.chars() {
            self.train_char(ch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypervector_properties() {
        let anchors = generate_anchors();
        assert_eq!(anchors.len(), NUM_ANCHORS);

        // Check pseudo-orthogonality (approx 50% bit overlap = ~256 bits)
        let sim = hamming_similarity(&anchors[0], &anchors[1]);
        assert!(sim > 200 && sim < 312, "Similarity {} not near 256", sim);
    }

    #[test]
    fn test_wire_rotate_invariance() {
        let v = [0x0123_4567_89AB_CDEF; D_U64];
        let rot_full = wire_rotate(&v, 512);
        assert_eq!(v, rot_full);
    }

    #[test]
    fn test_attractor_resonance_trigger() {
        let mut htgm = HierarchicalTBTMemory::new();
        htgm.train_text("fn sovereign_matrix() -> u64;");
        htgm.train_text("let val = 100;");

        let probe = htgm.probe_resonance();
        // Since ring has entries, resonance engine computes score
        assert!(probe.resonance_score > 0);
    }
}
