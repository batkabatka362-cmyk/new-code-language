// ============================================================================
// CRON Pure-Rust Subword Byte-Pair Encoding (BPE) Tokenizer
// Module: cronc::bpe_tokenizer
//
// 100% Pure Rust - Zero External Dependencies (No tiktoken, No HuggingFace)
// Features:
//   1. Canonical Byte-Fallback: 0..256 mapped to raw byte values (0% UNK drops).
//   2. High-speed iterative BPE pair merge ranking.
//   3. Foundation model special tokens (<|begin_of_text|>, <|im_start|>, <|im_end|>).
//   4. Direct ingestion from GGUF metadata (tokenizer.ggml.tokens & merges).
//   5. Lossless roundtrip UTF-8 decoding.
// ============================================================================

use std::collections::HashMap;

/// Special Token Constants for CRON Foundation Models
pub const BOS_TOKEN: &str = "<|begin_of_text|>";
pub const EOS_TOKEN: &str = "<|end_of_text|>";
pub const IM_START_TOKEN: &str = "<|im_start|>";
pub const IM_END_TOKEN: &str = "<|im_end|>";
pub const TOOL_CALL_TOKEN: &str = "<|tool_call|>";

/// Subword Byte-Pair Encoding (BPE) Tokenizer
#[derive(Debug, Clone)]
pub struct BpeTokenizer {
    token_to_id: HashMap<Vec<u8>, u32>,
    id_to_token: Vec<Vec<u8>>,
    merges: HashMap<(u32, u32), (u32, u32)>, // (id_a, id_b) -> (merged_id, rank)
    special_tokens: HashMap<String, u32>,
    id_to_special: HashMap<u32, String>,
    pub bos_id: u32,
    pub eos_id: u32,
}

impl Default for BpeTokenizer {
    fn default() -> Self {
        Self::from_default_vocab()
    }
}

impl BpeTokenizer {
    /// Creates an empty tokenizer with base 256 byte-fallback tokens
    pub fn new_empty() -> Self {
        let mut token_to_id = HashMap::with_capacity(512);
        let mut id_to_token = Vec::with_capacity(512);

        // Map 0..256 directly to raw byte values
        for b in 0..=255u8 {
            let byte_slice = vec![b];
            token_to_id.insert(byte_slice.clone(), b as u32);
            id_to_token.push(byte_slice);
        }

        let mut tok = Self {
            token_to_id,
            id_to_token,
            merges: HashMap::new(),
            special_tokens: HashMap::new(),
            id_to_special: HashMap::new(),
            bos_id: 256,
            eos_id: 257,
        };

        tok.register_special_token(BOS_TOKEN, 256);
        tok.register_special_token(EOS_TOKEN, 257);
        tok.register_special_token(IM_START_TOKEN, 258);
        tok.register_special_token(IM_END_TOKEN, 259);
        tok.register_special_token(TOOL_CALL_TOKEN, 260);

        tok
    }

    /// Registers a special token with a unique ID
    pub fn register_special_token(&mut self, token_str: &str, id: u32) {
        self.special_tokens.insert(token_str.to_string(), id);
        self.id_to_special.insert(id, token_str.to_string());
        let bytes = token_str.as_bytes().to_vec();
        self.token_to_id.insert(bytes.clone(), id);
        if (id as usize) >= self.id_to_token.len() {
            self.id_to_token.resize((id as usize) + 1, Vec::new());
        }
        self.id_to_token[id as usize] = bytes;
    }

    /// Ingests vocabulary and merges from GGUF metadata
    pub fn from_gguf_metadata(
        tokens: &[String],
        _scores: Option<&[f32]>,
        merges: Option<&[String]>,
    ) -> Result<Self, String> {
        let mut tokenizer = Self::new_empty();

        for (idx, token_str) in tokens.iter().enumerate() {
            let id = idx as u32;
            let bytes = token_str.as_bytes().to_vec();
            tokenizer.token_to_id.insert(bytes.clone(), id);
            if (id as usize) >= tokenizer.id_to_token.len() {
                tokenizer.id_to_token.resize((id as usize) + 1, Vec::new());
            }
            tokenizer.id_to_token[id as usize] = bytes;
        }

        if let Some(merge_list) = merges {
            for (rank, merge_rule) in merge_list.iter().enumerate() {
                let parts: Vec<&str> = merge_rule.split_whitespace().collect();
                if parts.len() == 2 {
                    let a_bytes = parts[0].as_bytes();
                    let b_bytes = parts[1].as_bytes();
                    if let (Some(&id_a), Some(&id_b)) = (
                        tokenizer.token_to_id.get(a_bytes),
                        tokenizer.token_to_id.get(b_bytes),
                    ) {
                        let mut combined = parts[0].as_bytes().to_vec();
                        combined.extend_from_slice(parts[1].as_bytes());
                        let merged_id = if let Some(&existing_id) = tokenizer.token_to_id.get(&combined) {
                            existing_id
                        } else {
                            let new_id = tokenizer.id_to_token.len() as u32;
                            tokenizer.token_to_id.insert(combined.clone(), new_id);
                            tokenizer.id_to_token.push(combined);
                            new_id
                        };
                        tokenizer.merges.insert((id_a, id_b), (merged_id, rank as u32));
                    }
                }
            }
        }

        Ok(tokenizer)
    }

    /// Builds a rich foundation vocabulary out of the box (English subwords, code syntax, numbers)
    pub fn from_default_vocab() -> Self {
        let mut tok = Self::new_empty();

        let common_subwords = [
            "the", "The", " in", " to", " of", " and", " a", " is", " for", " that",
            " on", " with", " as", " by", " at", " from", " this", " be", " are", " or",
            " an", " was", " we", " will", " can", " all", " not", " your", " you", "CRON",
            "BitNet", "AI", "tensor", "core", "layer", "model", "return", "let", "fn", "struct",
            "if", "else", "for", "while", "true", "false", "null", "none", "import", "export",
            "1.58", "7B", "4D", "Torus", "VLIW", "NoC", "RAM", "MB", "GB", "ms",
            " {\"", "\": \"", "\", \"", "\"}", "{}", "[]", "->", "=>", "==", "!=",
            " (", ")", " [", "]", " {", "}", ": ", ", ", ".\"", "\".",
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
            "\\n", "\\t", "\\\"", "\\\\", "...", "/*", "*/", "//",
            " cognitive", " neural", " hardware", " silicon", " paged", " streaming",
        ];

        let mut current_id = 300u32;
        let mut rank = 0u32;

        for subword in common_subwords {
            let bytes = subword.as_bytes().to_vec();
            if !tok.token_to_id.contains_key(&bytes) {
                tok.token_to_id.insert(bytes.clone(), current_id);
                if (current_id as usize) >= tok.id_to_token.len() {
                    tok.id_to_token.resize((current_id as usize) + 1, Vec::new());
                }
                tok.id_to_token[current_id as usize] = bytes.clone();

                // Add synthetic merge pairs for subword
                if bytes.len() >= 2 {
                    let id_a = bytes[0] as u32;
                    let id_b = if bytes.len() == 2 {
                        bytes[1] as u32
                    } else {
                        // Intermediate subword
                        current_id
                    };
                    tok.merges.insert((id_a, id_b), (current_id, rank));
                    rank += 1;
                }
                current_id += 1;
            }
        }

        tok
    }

    /// Encodes arbitrary text into subword token IDs using BPE
    pub fn encode(&self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return Vec::new();
        }

        // 1. Check for special tokens prefix/match
        let mut tokens = Vec::new();
        let mut remaining = text;

        while !remaining.is_empty() {
            // Check if remaining starts with a special token
            let mut matched_special = false;
            for (special_str, &id) in &self.special_tokens {
                if remaining.starts_with(special_str) {
                    tokens.push(id);
                    remaining = &remaining[special_str.len()..];
                    matched_special = true;
                    break;
                }
            }

            if matched_special {
                continue;
            }

            // Find next special token boundary or take the whole chunk
            let mut next_boundary = remaining.len();
            for special_str in self.special_tokens.keys() {
                if let Some(pos) = remaining.find(special_str) {
                    if pos > 0 && pos < next_boundary {
                        next_boundary = pos;
                    }
                }
            }

            let chunk = &remaining[..next_boundary];
            remaining = &remaining[next_boundary..];

            // 2. Initial byte-level tokenization of the chunk
            let mut chunk_tokens: Vec<u32> = chunk.as_bytes().iter().map(|&b| b as u32).collect();

            // 3. Iterative BPE Merge resolution
            loop {
                if chunk_tokens.len() < 2 {
                    break;
                }

                // Find candidate pair with minimum rank (highest priority)
                let mut best_pair: Option<(usize, (u32, u32))> = None;
                let mut min_rank = u32::MAX;

                for i in 0..chunk_tokens.len() - 1 {
                    let pair = (chunk_tokens[i], chunk_tokens[i + 1]);
                    if let Some(&(merged_id, rank)) = self.merges.get(&pair) {
                        if rank < min_rank {
                            min_rank = rank;
                            best_pair = Some((i, (merged_id, rank)));
                        }
                    }
                }

                match best_pair {
                    Some((idx, (merged_id, _))) => {
                        chunk_tokens[idx] = merged_id;
                        chunk_tokens.remove(idx + 1);
                    }
                    None => break, // No more merges possible
                }
            }

            tokens.extend_from_slice(&chunk_tokens);
        }

        tokens
    }

    /// Decodes a sequence of token IDs back into a UTF-8 String
    pub fn decode(&self, tokens: &[u32]) -> String {
        let mut raw_bytes = Vec::new();

        for &id in tokens {
            if let Some(special) = self.id_to_special.get(&id) {
                raw_bytes.extend_from_slice(special.as_bytes());
            } else if (id as usize) < self.id_to_token.len() {
                raw_bytes.extend_from_slice(&self.id_to_token[id as usize]);
            } else if id <= 255 {
                raw_bytes.push(id as u8);
            }
        }

        String::from_utf8_lossy(&raw_bytes).to_string()
    }

    /// Total vocabulary size
    #[inline]
    pub fn vocab_size(&self) -> usize {
        self.id_to_token.len().max(256)
    }

    /// BOS Token ID
    #[inline]
    pub fn bos_token_id(&self) -> u32 {
        self.bos_id
    }

    /// EOS Token ID
    #[inline]
    pub fn eos_token_id(&self) -> u32 {
        self.eos_id
    }
}
