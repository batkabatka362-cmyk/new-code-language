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

    /// Creates a 100% lossless, bijective .dpdata package for arbitrary binary/UTF-8 data.
    pub fn create_lossless_package<P: AsRef<Path>>(data: &[u8], out_path: P) -> Result<usize, String> {
        let (packed_states, orig_len) = LosslessBijective94::encode_bytes(data);
        let token_count = packed_states.len() as u32;
        let orig_len_u64 = orig_len as u64;

        let mut file = File::create(out_path).map_err(|e| format!("Failed to create .dpdata: {}", e))?;
        file.write_all(DPDATA_MAGIC_LOSSLESS).map_err(|e| format!("Write failed: {}", e))?;
        file.write_all(&orig_len_u64.to_le_bytes()).map_err(|e| format!("Write failed: {}", e))?;
        file.write_all(&token_count.to_le_bytes()).map_err(|e| format!("Write failed: {}", e))?;
        file.write_all(&packed_states).map_err(|e| format!("Write failed: {}", e))?;

        Ok(16 + packed_states.len())
    }

    /// Reads a lossless .dpdata binary stream back into the exact original byte stream.
    pub fn load_lossless_package<P: AsRef<Path>>(in_path: P) -> Result<Vec<u8>, String> {
        let mut file = File::open(in_path).map_err(|e| format!("Failed to open .dpdata: {}", e))?;
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).map_err(|e| format!("Header read failed: {}", e))?;

        if &magic != DPDATA_MAGIC_LOSSLESS {
            return Err("Not a lossless DP95 format package".to_string());
        }

        let mut orig_len_buf = [0u8; 8];
        file.read_exact(&mut orig_len_buf).map_err(|e| format!("Read orig_len failed: {}", e))?;
        let orig_len = u64::from_le_bytes(orig_len_buf) as usize;

        let mut token_count_buf = [0u8; 4];
        file.read_exact(&mut token_count_buf).map_err(|e| format!("Read token_count failed: {}", e))?;
        let token_count = u32::from_le_bytes(token_count_buf) as usize;

        let mut states = vec![0u8; token_count];
        file.read_exact(&mut states).map_err(|e| format!("Read states failed: {}", e))?;

        Ok(LosslessBijective94::decode_bytes(&states, orig_len))
    }

    /// Lossless UTF-8 text pack and unpack
    pub fn create_lossless_text_package<P: AsRef<Path>>(text: &str, out_path: P) -> Result<usize, String> {
        Self::create_lossless_package(text.as_bytes(), out_path)
    }

    pub fn load_lossless_text_package<P: AsRef<Path>>(in_path: P) -> Result<String, String> {
        let bytes = Self::load_lossless_package(in_path)?;
        String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode failed: {}", e))
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

// ==============================================================================
// 3. LOSSLESS BIJECTIVE BASE-94 ENCODING & CAYLEY LATIN TRANSFORM
// ==============================================================================
pub const DPDATA_MAGIC_LOSSLESS: &[u8; 4] = b"DP95";

pub struct LosslessBijective94;

impl LosslessBijective94 {
    /// Encodes any byte slice into Base-94 state tokens [0..93].
    /// Every 4 bytes (32-bit uint) decomposes into 5 Base-94 digits (94^5 > 2^32).
    pub fn encode_bytes(data: &[u8]) -> (Vec<u8>, usize) {
        let orig_len = data.len();
        let num_blocks = (orig_len + 3) / 4;
        let mut out = Vec::with_capacity(num_blocks * 5);

        for chunk in data.chunks(4) {
            let mut buf = [0u8; 4];
            buf[..chunk.len()].copy_from_slice(chunk);
            let mut v = (buf[0] as u64)
                | ((buf[1] as u64) << 8)
                | ((buf[2] as u64) << 16)
                | ((buf[3] as u64) << 24);

            for _ in 0..5 {
                let digit = (v % 94) as u8;
                out.push(digit);
                v /= 94;
            }
        }

        (out, orig_len)
    }

    /// Decodes Base-94 state tokens [0..93] back into exact original bytes.
    pub fn decode_bytes(states: &[u8], orig_len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(orig_len);

        for chunk in states.chunks(5) {
            let mut v = 0u64;
            let mut mult = 1u64;
            for &d in chunk {
                v += (d as u64 % 94) * mult;
                mult *= 94;
            }

            let b = [
                (v & 0xFF) as u8,
                ((v >> 8) & 0xFF) as u8,
                ((v >> 16) & 0xFF) as u8,
                ((v >> 24) & 0xFF) as u8,
            ];

            for byte in b {
                if out.len() < orig_len {
                    out.push(byte);
                }
            }
        }

        out
    }

    /// Permutes state with Cayley Quasigroup Latin Square: C(s, key) = (3s + 5key + 7) mod 94
    #[inline(always)]
    pub fn cayley_forward(state: u8, key: u8) -> u8 {
        crate::cl_matrix94::cayley_cell(3, 5, 7, state, key)
    }

    /// Exact inverse of Cayley Quasigroup Latin Square using modular inverse 3^-1 mod 94 = 63.
    #[inline(always)]
    pub fn cayley_inverse(cell: u8, key: u8) -> u8 {
        let term = (cell as i64) - 5 * (key as i64) - 7;
        let pos_term = ((term % 94) + 94) % 94;
        ((63 * pos_term) % 94) as u8
    }
}

// ==============================================================================
// 4. STREAMING DPSTA READER (Bounded strictly to 35.3 KB L1 Cache footprint)
// ==============================================================================
pub struct StreamingDpstaReader<R: Read> {
    reader: R,
    pub token_count: usize,
    read_tokens: usize,
    chunk_buffer: Vec<u8>,
}

impl<R: Read> StreamingDpstaReader<R> {
    pub const BUFFER_SIZE: usize = crate::cl_matrix94::FOUR_TACT_SIZE; // 35,344 bytes

    pub fn new(mut reader: R) -> Result<Self, String> {
        let mut header = [0u8; 8];
        reader.read_exact(&mut header).map_err(|e| format!("Header read error: {}", e))?;
        if &header[0..4] != DPDATA_MAGIC && &header[0..4] != DPDATA_MAGIC_LOSSLESS {
            return Err("Invalid .dpdata magic signature".to_string());
        }
        let token_count = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        Ok(Self {
            reader,
            token_count,
            read_tokens: 0,
            chunk_buffer: vec![0u8; Self::BUFFER_SIZE],
        })
    }

    /// Reads the next chunk up to 35.3 KB into the internal L1 cache buffer.
    pub fn read_next_chunk(&mut self) -> std::io::Result<Option<&[u8]>> {
        if self.read_tokens >= self.token_count {
            return Ok(None);
        }
        let to_read = (self.token_count - self.read_tokens).min(Self::BUFFER_SIZE);
        self.reader.read_exact(&mut self.chunk_buffer[..to_read])?;
        self.read_tokens += to_read;
        Ok(Some(&self.chunk_buffer[..to_read]))
    }

    pub fn read_so_far(&self) -> usize {
        self.read_tokens
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
    fn test_lossless_bijective_base94_roundtrip() {
        let test_texts = [
            "Монгол хэл дээрх хиймэл оюуны DPSTA өгөгдөл шахалт ба сэргээлт! 100% алдагдалгүй.",
            "fn compute_clifford_4d(theta: f64) -> [f64; 4] {\n    let s = theta.sin();\n    [s, -s, 1.0, 0.0]\n}",
            "Arbitrary spaces, tabs \t\t, and\nnewlines with special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?~`",
            "",
            "A",
            "12345",
        ];

        for text in &test_texts {
            let (encoded, orig_len) = LosslessBijective94::encode_bytes(text.as_bytes());
            // Verify all encoded digits are valid discrete state IDs 0..93
            for &d in &encoded {
                assert!(d < 94, "Every state must be in 0..93 discrete range");
            }

            let decoded = LosslessBijective94::decode_bytes(&encoded, orig_len);
            let reconstructed = String::from_utf8(decoded).expect("Must be valid UTF-8");
            assert_eq!(*text, reconstructed, "Lossless bijective Base-94 roundtrip must match 100%");
        }
    }

    #[test]
    fn test_cayley_quasigroup_latin_square_invertibility() {
        // Test all states 0..93 and multiple salt keys
        for key in [0u8, 1, 7, 13, 42, 93] {
            for state in 0..94u8 {
                let permuted = LosslessBijective94::cayley_forward(state, key);
                assert!(permuted < 94);
                let inverted = LosslessBijective94::cayley_inverse(permuted, key);
                assert_eq!(state, inverted, "Cayley Latin Square must be 100% reversible with key {}", key);
            }
        }
    }

    #[test]
    fn test_streaming_dpsta_reader_l1_bounded() {
        let text = "CRON DPSTA Crystalline Stream L1 Cache Verification Test ".repeat(200);
        let tmp = std::env::temp_dir().join("test_stream_l1.dpdata");

        CompactDatasetEngine::create_lossless_text_package(&text, &tmp).unwrap();

        let file = fs::File::open(&tmp).unwrap();
        let mut stream_reader = StreamingDpstaReader::new(file).unwrap();

        let mut total_chunks = 0;
        let mut total_bytes_read = 0;

        while let Ok(Some(chunk)) = stream_reader.read_next_chunk() {
            total_chunks += 1;
            total_bytes_read += chunk.len();
            assert!(chunk.len() <= StreamingDpstaReader::<fs::File>::BUFFER_SIZE);
        }

        assert!(total_chunks >= 1);
        assert_eq!(total_bytes_read, stream_reader.token_count);

        // Verify content
        let recovered = CompactDatasetEngine::load_lossless_text_package(&tmp).unwrap();
        assert_eq!(recovered, text);

        let _ = fs::remove_file(tmp);
    }
}

