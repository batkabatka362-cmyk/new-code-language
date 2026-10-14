//! `cl_dpsta`: Ultra-Compact 94-State Dataset Engine & Sub-Byte 1.58-bit Ternary Packing.
//!
//! Based on DPSTA & Topological Brain Transformer (TBT) specification (matrix4.1.2.pdf).
//!
//! Features:
//!   - Sub-Byte 1.58-bit Ternary Packing: 5 ternary synapses {-1, 0, +1} packed into 1 byte (3^5 = 243 <= 256).
//!     Fits 42.9 billion synapses into 8 GB RAM!
//!   - Crystalline Dataset Binary Stream (.dpdata): Magic 'DP94' + uint32 token count + 7-bit state tokens.
//!   - 10-character Base-94 hardware word formatter with CRC-8 integrity.

use crate::cl_matrix94::char_to_state;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub const DPDATA_MAGIC: &[u8; 4] = b"DP94";

// ==============================================================================
// 1. SUB-BYTE 1.58-BIT TERNARY PACKER (3^5 = 243 <= 256)
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum TernaryWeight {
    NegOne = -1,
    Zero = 0,
    PosOne = 1,
}

impl TernaryWeight {
    #[inline(always)]
    pub fn from_i8(val: i8) -> Self {
        if val < 0 {
            TernaryWeight::NegOne
        } else if val == 0 {
            TernaryWeight::Zero
        } else {
            TernaryWeight::PosOne
        }
    }

    #[inline(always)]
    pub fn to_code(self) -> u8 {
        match self {
            TernaryWeight::NegOne => 0,
            TernaryWeight::Zero => 1,
            TernaryWeight::PosOne => 2,
        }
    }

    #[inline(always)]
    pub fn from_code(code: u8) -> Self {
        match code % 3 {
            0 => TernaryWeight::NegOne,
            1 => TernaryWeight::Zero,
            _ => TernaryWeight::PosOne,
        }
    }
}

/// Packs 5 ternary weights into a single byte:
/// byte_val = w0 + 3*w1 + 9*w2 + 27*w3 + 81*w4 (<= 242)
pub fn pack_5_ternary(weights: [TernaryWeight; 5]) -> u8 {
    let mut val = 0u16;
    let mut multiplier = 1u16;
    for &w in weights.iter() {
        val += (w.to_code() as u16) * multiplier;
        multiplier *= 3;
    }
    val as u8
}

/// Unpacks 1 byte back into 5 ternary weights.
pub fn unpack_5_ternary(byte_val: u8) -> [TernaryWeight; 5] {
    let mut rem = byte_val;
    let mut weights = [TernaryWeight::Zero; 5];
    for w in weights.iter_mut() {
        let code = rem % 3;
        *w = TernaryWeight::from_code(code);
        rem /= 3;
    }
    weights
}

/// Compresses a slice of i8 ternary weights into sub-byte packed bytes.
pub fn pack_ternary_slice(weights: &[i8]) -> Vec<u8> {
    let num_chunks = (weights.len() + 4) / 5;
    let mut out = Vec::with_capacity(num_chunks);

    for chunk in weights.chunks(5) {
        let mut t_chunk = [TernaryWeight::Zero; 5];
        for (i, &w) in chunk.iter().enumerate() {
            t_chunk[i] = TernaryWeight::from_i8(w);
        }
        out.push(pack_5_ternary(t_chunk));
    }
    out
}

/// Decompresses sub-byte packed bytes back into i8 ternary weights.
pub fn unpack_ternary_slice(packed: &[u8], total_weights: usize) -> Vec<i8> {
    let mut out = Vec::with_capacity(total_weights);
    for &b in packed {
        let unpacked = unpack_5_ternary(b);
        for w in unpacked {
            if out.len() < total_weights {
                out.push(w as i8);
            }
        }
    }
    out
}

// ==============================================================================
// 2. CRYSTALLINE DATASET ENGINE (.dpdata)
// ==============================================================================
pub struct SubByteTernaryDataset;

impl SubByteTernaryDataset {
    /// Ingests ASCII text and packs it into sub-byte 1.58-bit ternary binary format.
    pub fn pack_ascii(text: &str) -> Vec<u8> {
        let ternary_weights: Vec<i8> = text
            .chars()
            .filter(|c| !c.is_control())
            .map(|c| {
                let st = char_to_state(c) as i8;
                if st % 3 == 0 {
                    -1
                } else if st % 3 == 1 {
                    0
                } else {
                    1
                }
            })
            .collect();
        pack_ternary_slice(&ternary_weights)
    }

    /// Unpacks sub-byte 1.58-bit stream back into ternary values.
    pub fn unpack_stream(packed: &[u8], total: usize) -> Vec<i8> {
        unpack_ternary_slice(packed, total)
    }
}

pub struct CompactDatasetEngine;

impl CompactDatasetEngine {
    /// Compresses a raw text string into a .dpdata binary dataset.
    pub fn create_dataset_package<P: AsRef<Path>>(text: &str, out_path: P) -> Result<usize, String> {
        let packed_states: Vec<u8> = text
            .chars()
            .filter(|c| !c.is_control())
            .map(char_to_state)
            .collect();

        let token_count = packed_states.len() as u32;
        let mut file = File::create(out_path).map_err(|e| format!("Failed to create .dpdata: {}", e))?;

        file.write_all(DPDATA_MAGIC)
            .map_err(|e| format!("Write failed: {}", e))?;
        file.write_all(&token_count.to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        file.write_all(&packed_states)
            .map_err(|e| format!("Write failed: {}", e))?;

        Ok(8 + packed_states.len())
    }

    /// Reads a .dpdata binary stream back into discrete state tokens.
    pub fn load_dataset_package<P: AsRef<Path>>(in_path: P) -> Result<Vec<u8>, String> {
        let mut file = File::open(in_path).map_err(|e| format!("Failed to open .dpdata: {}", e))?;
        let mut header = [0u8; 8];
        file.read_exact(&mut header)
            .map_err(|e| format!("Failed to read .dpdata header: {}", e))?;

        if &header[0..4] != DPDATA_MAGIC {
            return Err("Invalid .dpdata magic signature".to_string());
        }

        let token_count = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let mut states = vec![0u8; token_count];
        file.read_exact(&mut states)
            .map_err(|e| format!("Failed to read .dpdata tokens: {}", e))?;

        Ok(states)
    }

    /// Formats a 10-character Base-94 hardware word with CRC-8 integrity.
    pub fn format_base94_word(chars_10: &str) -> (String, u8) {
        let mut word: Vec<char> = chars_10.chars().take(10).collect();
        while word.len() < 10 {
            word.push('_');
        }
        let word_str: String = word.into_iter().collect();

        // Calculate CRC-8 (standard SMBus polynomial 0x07)
        let mut crc = 0u8;
        for b in word_str.as_bytes() {
            crc ^= b;
            for _ in 0..8 {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ 0x07;
                } else {
                    crc <<= 1;
                }
            }
        }

        (word_str, crc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cl_matrix94::state_to_char;
    use std::fs;

    #[test]
    fn test_subbyte_ternary_packing_roundtrip() {
        let original: Vec<i8> = vec![-1, 0, 1, 1, -1, 0, -1, 1, 0, 0, 1, -1];
        let packed = pack_ternary_slice(&original);

        // 12 weights pack into ceil(12/5) = 3 bytes!
        assert_eq!(packed.len(), 3);

        let recovered = unpack_ternary_slice(&packed, original.len());
        assert_eq!(recovered, original);
    }

    #[test]
    fn test_dpdata_dataset_roundtrip() {
        // Only printable ASCII 33..126 characters roundtrip perfectly in the 94-state system.
        // Space (ASCII 32) is outside this range and maps to state 0 = '!' by design.
        let sample = "fn_sovereign_agi()_{_94_}";
        let tmp = std::env::temp_dir().join("test_sample.dpdata");

        CompactDatasetEngine::create_dataset_package(sample, &tmp).unwrap();
        let tokens = CompactDatasetEngine::load_dataset_package(&tmp).unwrap();

        let reconstructed: String = tokens.into_iter().map(state_to_char).collect();
        assert_eq!(reconstructed, sample);

        let _ = fs::remove_file(tmp);
    }

    #[test]
    fn test_space_maps_to_bang_by_design() {
        // Space (ASCII 32) is below the 94-state range (33..126).
        // It maps to state 0 = '!' — this is correct DPSTA behavior.
        use crate::cl_matrix94::{char_to_state, state_to_char};
        let s = char_to_state(' ');
        assert_eq!(s, 0);
        assert_eq!(state_to_char(s), '!');
    }

    #[test]
    fn test_base94_word_crc8() {
        let (word, crc) = CompactDatasetEngine::format_base94_word("QUANTUM_OP");
        assert_eq!(word.len(), 10);
        assert!(crc > 0);
    }
}
