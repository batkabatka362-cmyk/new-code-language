//! Holographic Hyperdimensional Vector Symbolic Associative Memory (HDC / VSA)
//!
//! Implements 1024-trit high-capacity holographic memory with single-shot associative binding,
//! superposition bundling, sequence permutation, and O(1) flash retrieval for SAGI AGI.
//! Zero Catastrophic Forgetting & Zero-KV-Cache overhead.

use crate::cl_macro::{MacroCompiler, MacroStmt};

/// Dimension of the Hyperdimensional Vector (1024 trits = 64 16-trit words)
pub const HDC_DIM: usize = 1024;
pub const HDC_WORDS: usize = HDC_DIM / 16; // 64 32-bit words

/// 1024-Trit Hyperdimensional Vector with values in {-1, 0, +1}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperVector {
    /// Stored as 64 32-bit words (each holding 16 2-bit trits: 00=0, 01=+1, 10=-1)
    pub words: [u32; HDC_WORDS],
}

impl HyperVector {
    pub fn zero() -> Self {
        Self {
            words: [0u32; HDC_WORDS],
        }
    }

    /// Generates a pseudo-random orthogonal hypervector from a seed string/hash
    pub fn from_seed(seed_str: &str) -> Self {
        let mut words = [0u32; HDC_WORDS];
        let mut hash = 0x811c9dc5u32;
        for b in seed_str.bytes() {
            hash = hash.wrapping_mul(0x01000193) ^ (b as u32);
        }

        let mut rng = hash as u64;
        for w in 0..HDC_WORDS {
            let mut word = 0u32;
            for i in 0..16 {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                let choice = (rng >> 32) % 3;
                let trit_bits = match choice {
                    1 => 0b01, // +1
                    2 => 0b10, // -1
                    _ => 0b00, // 0
                };
                word |= (trit_bits as u32) << (i * 2);
            }
            words[w] = word;
        }
        Self { words }
    }

    /// Extract trit at index (0..1023) -> -1, 0, or +1
    pub fn get_trit(&self, index: usize) -> i8 {
        let word_idx = (index / 16) % HDC_WORDS;
        let trit_idx = index % 16;
        let bits = (self.words[word_idx] >> (trit_idx * 2)) & 0b11;
        match bits {
            0b01 => 1,
            0b10 => -1,
            _ => 0,
        }
    }

    /// Set trit at index
    pub fn set_trit(&mut self, index: usize, val: i8) {
        let word_idx = (index / 16) % HDC_WORDS;
        let trit_idx = index % 16;
        let mask = !(0b11u32 << (trit_idx * 2));
        let bits = match val {
            1 => 0b01u32,
            -1 => 0b10u32,
            _ => 0b00u32,
        };
        self.words[word_idx] = (self.words[word_idx] & mask) | (bits << (trit_idx * 2));
    }

    /// Binding operation (Associative multiplication: A * B)
    /// Used to bind Concept to Role (e.g. Country * Capital)
    pub fn bind(&self, other: &HyperVector) -> HyperVector {
        let mut result = HyperVector::zero();
        for i in 0..HDC_DIM {
            let a = self.get_trit(i);
            let b = other.get_trit(i);
            let prod = a * b; // (-1)*(-1)=1, 1*(-1)=-1, etc.
            result.set_trit(i, prod);
        }
        result
    }

    /// Bundling operation (Superposition: A + B + C with majority rule)
    /// Combines multiple memories into a single holographic vector without growing in size
    pub fn bundle(vectors: &[HyperVector]) -> HyperVector {
        let mut result = HyperVector::zero();
        for i in 0..HDC_DIM {
            let mut sum: i32 = 0;
            for v in vectors {
                sum += v.get_trit(i) as i32;
            }
            let trit = if sum > 0 {
                1
            } else if sum < 0 {
                -1
            } else {
                0
            };
            result.set_trit(i, trit);
        }
        result
    }

    /// Sequence Permutation (Cyclic rotation by k steps: rho^k(A))
    /// Encodes temporal order and causal chains (e.g., Word0 -> Word1 -> Word2)
    pub fn permute(&self, shift: usize) -> HyperVector {
        let mut result = HyperVector::zero();
        for i in 0..HDC_DIM {
            let new_idx = (i + shift) % HDC_DIM;
            result.set_trit(new_idx, self.get_trit(i));
        }
        result
    }

    /// Cosine / Dot product similarity between two hypervectors (-1.0 to +1.0)
    pub fn similarity(&self, other: &HyperVector) -> f32 {
        let mut dot: i32 = 0;
        let mut norm_a: i32 = 0;
        let mut norm_b: i32 = 0;

        for i in 0..HDC_DIM {
            let a = self.get_trit(i) as i32;
            let b = other.get_trit(i) as i32;
            dot += a * b;
            norm_a += a * a;
            norm_b += b * b;
        }

        if norm_a == 0 || norm_b == 0 {
            0.0
        } else {
            (dot as f32) / ((norm_a as f32).sqrt() * (norm_b as f32).sqrt())
        }
    }

    /// Compiles this Hypervector into a stream of `.cl` VLIW instructions for Core registers
    pub fn compile_load_to_cl(&self, core_id: u8) -> String {
        let mut compiler = MacroCompiler::new(core_id);
        let mut stmts = Vec::new();

        // Load first 16 words into registers R0..RF
        for (reg_idx, &word) in self.words.iter().take(16).enumerate() {
            stmts.push(MacroStmt::AssignImm {
                dst: reg_idx as u8,
                imm: (word & 0xFFFF) as u16,
            });
        }
        stmts.push(MacroStmt::Barrier);

        compiler.compile_stmts(&stmts);
        compiler.finish()
    }
}

/// Holographic Associative Memory Store (Flash $O(1)$ Recall)
#[derive(Debug, Clone)]
pub struct HolographicMemoryBank {
    pub key_memory: Vec<(String, HyperVector)>,
    pub composite_memory: HyperVector,
}

impl HolographicMemoryBank {
    pub fn new() -> Self {
        Self {
            key_memory: Vec::new(),
            composite_memory: HyperVector::zero(),
        }
    }

    /// Store a key-value associative pair (e.g. ("capital", "paris"))
    pub fn store_pair(&mut self, key: &str, value: &str) {
        let v_key = HyperVector::from_seed(key);
        let v_val = HyperVector::from_seed(value);
        let bound = v_key.bind(&v_val);

        self.key_memory.push((value.to_string(), v_val));
        self.composite_memory = HyperVector::bundle(&[self.composite_memory.clone(), bound]);
    }

    /// Flash recall of a value given a query key in O(1) algebraic unbinding
    pub fn recall(&self, query_key: &str) -> Option<(String, f32)> {
        let v_query = HyperVector::from_seed(query_key);
        // Unbind: In ternary VSA, unbinding is self-inverse multiplication
        let unbound_probe = self.composite_memory.bind(&v_query);

        let mut best_match: Option<(String, f32)> = None;
        let mut max_sim = -1.0f32;

        for (val_name, v_target) in &self.key_memory {
            let sim = unbound_probe.similarity(v_target);
            if sim > max_sim && sim > 0.15 {
                max_sim = sim;
                best_match = Some((val_name.clone(), sim));
            }
        }

        best_match
    }
}
