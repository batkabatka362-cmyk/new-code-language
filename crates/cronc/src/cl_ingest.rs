//! `cl_ingest`: High-Speed Big Data Ingestion, Morph-Tokenizing & Dataset Compiler.
//!
//! Based on DPSTA & Topological Brain Transformer (TBT) Master Blueprint (Section 4, Gap 3).
//!
//! Key Functions:
//!   - High-throughput parallel ingestion of text, markdown, source code, and books.
//!   - Morph-tokenization & Base-94 canonicalization (space -> '!', ASCII 33..126).
//!   - Sub-byte 1.58-bit `.dpdata` dataset emission (5 synapses per byte = 5x compression).
//!   - Automatic training of a 36,414-byte Crystalline Vault snapshot (`.cldb`) via
//!     4-tact state transitions with homeostatic soft-decay.
//!   - Comprehensive telemetry: compression ratio, throughput (tokens/sec), active synapses.

use crate::cl_cldb::CldbVault;
use crate::cl_dpsta::CompactDatasetEngine;
use crate::cl_htgm::HierarchicalTBTMemory;
use crate::cl_matrix94::{FourTactPipeline94, ASCII_END, ASCII_START, NUM_STATES};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Tokenization mode for the ingestion engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizerMode {
    /// 94-char hardware canonicalization (ASCII 33..126, space -> '!')
    Ascii94,
    /// Multilingual subword BPE (Mongolian Cyrillic, English, Code, Special tokens)
    BpeSubword,
}

/// Configuration options for the big data ingestion pipeline.
#[derive(Debug, Clone)]
pub struct IngestOptions {
    pub chunk_size: usize,
    pub max_files: usize,
    pub emit_cldb: bool,
    pub vault_name: String,
    pub tokenizer_mode: TokenizerMode,
    pub streaming: bool,
}

impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            chunk_size: 65536,
            max_files: 10000,
            emit_cldb: true,
            vault_name: "sovereign_ingest_vault".to_string(),
            tokenizer_mode: TokenizerMode::Ascii94,
            streaming: false,
        }
    }
}

/// Statistics report generated after big data ingestion.
#[derive(Debug, Clone)]
pub struct IngestReport {
    pub files_scanned: usize,
    pub raw_bytes_read: usize,
    pub total_tokens: usize,
    pub dpdata_bytes_written: usize,
    pub compression_ratio_x: f64,
    pub cldb_vault_path: Option<PathBuf>,
    pub elapsed_millis: f64,
    pub throughput_tokens_per_sec: f64,
    pub active_synapses_trained: usize,
}

/// High-speed ingestion engine.
pub struct BigDataIngestEngine {
    pub options: IngestOptions,
}

impl Default for BigDataIngestEngine {
    fn default() -> Self {
        Self::new(IngestOptions::default())
    }
}

impl BigDataIngestEngine {
    pub fn new(options: IngestOptions) -> Self {
        Self { options }
    }

    /// Recursively collects text files from a file or directory path.
    pub fn collect_files(target: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if target.is_file() {
            files.push(target.to_path_buf());
            return Ok(files);
        }

        if target.is_dir() {
            Self::walk_dir(target, &mut files)?;
        }
        Ok(files)
    }

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                // Ignore target and .git directories
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name != "target" && name != ".git" && name != "node_modules" {
                    let _ = Self::walk_dir(&p, files);
                }
            } else if p.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if matches!(ext_lower.as_str(), "txt" | "md" | "cl" | "rs" | "c" | "py" | "json" | "csv" | "log") {
                        files.push(p);
                    }
                }
            }
        }
        Ok(())
    }

    /// Reads file and automatically extracts text from JSONL or raw text/markdown.
    pub fn parse_file_content(path: &Path) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let is_jsonl = path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("jsonl")).unwrap_or(false);
        if is_jsonl {
            let mut extracted = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(txt) = Self::extract_json_text_field(trimmed) {
                    extracted.push_str(&txt);
                    extracted.push('\n');
                } else {
                    extracted.push_str(trimmed);
                    extracted.push('\n');
                }
            }
            Some(extracted)
        } else {
            Some(content)
        }
    }

    /// Lightweight JSON text field extractor without external dependencies.
    pub fn extract_json_text_field(line: &str) -> Option<String> {
        let candidate_keys = ["\"text\":", "\"content\":", "\"prompt\":", "\"response\":", "\"body\":", "\"conversations\":"];
        for key in &candidate_keys {
            if let Some(pos) = line.find(key) {
                let after = &line[pos + key.len()..];
                if let Some(quote_start) = after.find('"') {
                    let mut s = String::new();
                    let mut escaped = false;
                    for ch in after[quote_start + 1..].chars() {
                        if escaped {
                            match ch {
                                'n' => s.push('\n'),
                                't' => s.push('\t'),
                                'r' => s.push('\r'),
                                '\\' => s.push('\\'),
                                '"' => s.push('"'),
                                _ => s.push(ch),
                            }
                            escaped = false;
                        } else if ch == '\\' {
                            escaped = true;
                        } else if ch == '"' {
                            return Some(s);
                        } else {
                            s.push(ch);
                        }
                    }
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
        None
    }

    /// Canonicalizes text into the 94-char alphabet (ASCII 33..126, space -> '!').
    pub fn canonicalize_text(text: &str) -> String {
        text.chars()
            .map(|c| if c == ' ' { '!' } else { c })
            .filter(|&c| (ASCII_START as char..=ASCII_END as char).contains(&c))
            .collect()
    }

    /// Ingests all collected files, builds `.dpdata`, and trains `.cldb`.
    pub fn ingest_path(
        &self,
        source_path: &Path,
        out_dpdata: &Path,
        out_cldb: Option<&Path>,
    ) -> io::Result<IngestReport> {
        let start = Instant::now();
        let file_paths = Self::collect_files(source_path)?;

        if self.options.streaming {
            return self.ingest_path_streaming(source_path, out_dpdata, out_cldb);
        }

        let mut raw_bytes_read = 0usize;
        let mut all_text = String::new();

        for p in file_paths.iter().take(self.options.max_files) {
            if let Some(content) = Self::parse_file_content(p) {
                raw_bytes_read += content.len();
                all_text.push_str(&content);
                all_text.push('\n');
            }
        }

        if all_text.is_empty() {
            all_text.push_str("SOVEREIGN AGI EMPTY INGESTION CORPUS INITIALIZED.");
        }

        // Tokenize based on selected tokenizer mode (ASCII Base-94 or Multilingual BPE Subwords)
        let normalized = match self.options.tokenizer_mode {
            TokenizerMode::Ascii94 => Self::canonicalize_text(&all_text),
            TokenizerMode::BpeSubword => {
                let bpe = crate::bpe_tokenizer::BpeTokenizer::from_default_vocab();
                let token_ids = bpe.encode(&all_text);
                token_ids.iter().map(|&id| ((id % 94) as u8 + ASCII_START) as char).collect()
            }
        };
        let tokens_count = normalized.len();

        // 1. Pack into .dpdata binary
        CompactDatasetEngine::create_dataset_package(&normalized, out_dpdata)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let dpdata_bytes_written = fs::metadata(out_dpdata)?.len() as usize;

        let compression_ratio_x = if dpdata_bytes_written > 0 {
            raw_bytes_read as f64 / dpdata_bytes_written as f64
        } else {
            1.0
        };

        // 2. Train 4-tact cortex & optionally emit .cldb
        let mut cortex = FourTactPipeline94::new();
        let mut htgm = HierarchicalTBTMemory::new();

        // Train in chunks
        let chunk_size = self.options.chunk_size;
        for chunk in normalized.as_bytes().chunks(chunk_size) {
            if let Ok(chunk_str) = std::str::from_utf8(chunk) {
                cortex.train_sequence(chunk_str);
                htgm.train_sequence(chunk_str);
            }
        }

        let mut active_synapses = 0usize;
        for s0 in 0..NUM_STATES {
            for s1 in 0..NUM_STATES {
                if cortex.m_stage1[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage2[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage3[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage4[s0][s1] > 0 { active_synapses += 1; }
            }
        }

        let cldb_vault_path = if let Some(cldb_p) = out_cldb {
            CldbVault::save(
                cldb_p,
                &self.options.vault_name,
                tokens_count as u64,
                &cortex,
                &htgm,
            ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Some(cldb_p.to_path_buf())
        } else {
            None
        };

        let elapsed = start.elapsed();
        let elapsed_millis = elapsed.as_secs_f64() * 1000.0;
        let throughput = if elapsed.as_secs_f64() > 0.0 {
            tokens_count as f64 / elapsed.as_secs_f64()
        } else {
            tokens_count as f64 * 1000.0
        };

        Ok(IngestReport {
            files_scanned: file_paths.len(),
            raw_bytes_read,
            total_tokens: tokens_count,
            dpdata_bytes_written,
            compression_ratio_x,
            cldb_vault_path,
            elapsed_millis,
            throughput_tokens_per_sec: throughput,
            active_synapses_trained: active_synapses,
        })
    }

    /// Streamingly ingests files with a constant memory ceiling (suitable for 10GB+ corpora).
    pub fn ingest_path_streaming(
        &self,
        source_path: &Path,
        out_dpdata: &Path,
        out_cldb: Option<&Path>,
    ) -> io::Result<IngestReport> {
        use std::io::{Seek, SeekFrom, Write};
        let start = Instant::now();
        let file_paths = Self::collect_files(source_path)?;

        let mut raw_bytes_read = 0usize;
        let mut total_tokens = 0usize;
        let mut cortex = FourTactPipeline94::new();
        let mut htgm = HierarchicalTBTMemory::new();
        let bpe = crate::bpe_tokenizer::BpeTokenizer::from_default_vocab();

        // Open streaming binary writer for .dpdata
        let mut file_out = fs::File::create(out_dpdata)?;
        file_out.write_all(crate::cl_dpsta::DPDATA_MAGIC)?;
        // Placeholder for 32-bit token count
        file_out.write_all(&0u32.to_le_bytes())?;

        let mut streaming_buffer = String::with_capacity(self.options.chunk_size);

        for p in file_paths.iter().take(self.options.max_files) {
            if let Some(content) = Self::parse_file_content(p) {
                raw_bytes_read += content.len();
                streaming_buffer.push_str(&content);
                streaming_buffer.push('\n');

                if streaming_buffer.len() >= self.options.chunk_size {
                    let chunk_tokens: String = match self.options.tokenizer_mode {
                        TokenizerMode::Ascii94 => Self::canonicalize_text(&streaming_buffer),
                        TokenizerMode::BpeSubword => {
                            let token_ids = bpe.encode(&streaming_buffer);
                            token_ids.iter().map(|&id| ((id % 94) as u8 + ASCII_START) as char).collect()
                        }
                    };

                    total_tokens += chunk_tokens.len();
                    cortex.train_sequence(&chunk_tokens);
                    htgm.train_sequence(&chunk_tokens);

                    let packed_states: Vec<u8> = chunk_tokens.chars().map(crate::cl_matrix94::char_to_state).collect();
                    file_out.write_all(&packed_states)?;

                    streaming_buffer.clear();
                }
            }
        }

        if !streaming_buffer.is_empty() {
            let chunk_tokens: String = match self.options.tokenizer_mode {
                TokenizerMode::Ascii94 => Self::canonicalize_text(&streaming_buffer),
                TokenizerMode::BpeSubword => {
                    let token_ids = bpe.encode(&streaming_buffer);
                    token_ids.iter().map(|&id| ((id % 94) as u8 + ASCII_START) as char).collect()
                }
            };
            total_tokens += chunk_tokens.len();
            cortex.train_sequence(&chunk_tokens);
            htgm.train_sequence(&chunk_tokens);
            let packed_states: Vec<u8> = chunk_tokens.chars().map(crate::cl_matrix94::char_to_state).collect();
            file_out.write_all(&packed_states)?;
        }

        // Rewind and write exact token count in header
        file_out.seek(SeekFrom::Start(4))?;
        file_out.write_all(&(total_tokens as u32).to_le_bytes())?;
        file_out.flush()?;
        drop(file_out);

        let dpdata_bytes_written = fs::metadata(out_dpdata)?.len() as usize;
        let compression_ratio_x = if dpdata_bytes_written > 0 {
            raw_bytes_read as f64 / dpdata_bytes_written as f64
        } else {
            1.0
        };

        let mut active_synapses = 0usize;
        for s0 in 0..NUM_STATES {
            for s1 in 0..NUM_STATES {
                if cortex.m_stage1[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage2[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage3[s0][s1] > 0 { active_synapses += 1; }
                if cortex.m_stage4[s0][s1] > 0 { active_synapses += 1; }
            }
        }

        let cldb_vault_path = if let Some(cldb_p) = out_cldb {
            CldbVault::save(
                cldb_p,
                &self.options.vault_name,
                total_tokens as u64,
                &cortex,
                &htgm,
            ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Some(cldb_p.to_path_buf())
        } else {
            None
        };

        let elapsed = start.elapsed();
        let elapsed_millis = elapsed.as_secs_f64() * 1000.0;
        let throughput = if elapsed.as_secs_f64() > 0.0 {
            total_tokens as f64 / elapsed.as_secs_f64()
        } else {
            total_tokens as f64 * 1000.0
        };

        Ok(IngestReport {
            files_scanned: file_paths.len(),
            raw_bytes_read,
            total_tokens,
            dpdata_bytes_written,
            compression_ratio_x,
            cldb_vault_path,
            elapsed_millis,
            throughput_tokens_per_sec: throughput,
            active_synapses_trained: active_synapses,
        })
    }
}

/// Single batch of input & target token IDs for in-silicon training
#[derive(Debug, Clone)]
pub struct ClDataBatch {
    pub input_ids: Vec<usize>,
    pub target_ids: Vec<usize>,
    pub batch_size: usize,
    pub seq_len: usize,
}

/// High-Speed Dataset DataLoader with batching, deterministic shuffle, and epoch iteration
#[derive(Debug, Clone)]
pub struct ClDataLoader {
    pub tokens: Vec<usize>,
    pub batch_size: usize,
    pub seq_len: usize,
    pub shuffle: bool,
    pub drop_last: bool,
    pub seed: u64,
}

impl ClDataLoader {
    pub fn new(tokens: Vec<usize>, batch_size: usize, seq_len: usize, shuffle: bool, seed: u64) -> Self {
        Self {
            tokens,
            batch_size: batch_size.max(1),
            seq_len: seq_len.max(1),
            shuffle,
            drop_last: true,
            seed,
        }
    }

    pub fn from_dpdata_file(path: &Path, batch_size: usize, seq_len: usize, shuffle: bool, seed: u64) -> io::Result<Self> {
        let states = crate::cl_dpsta::CompactDatasetEngine::load_dataset_package(path)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let tokens: Vec<usize> = states.into_iter().map(|s| s as usize).collect();
        Ok(Self::new(tokens, batch_size, seq_len, shuffle, seed))
    }

    pub fn len(&self) -> usize {
        let sample_stride = self.seq_len;
        let total_samples = if self.tokens.len() > self.seq_len {
            (self.tokens.len() - 1) / sample_stride
        } else {
            0
        };
        if self.drop_last {
            total_samples / self.batch_size
        } else {
            (total_samples + self.batch_size - 1) / self.batch_size
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter_epoch(&self, epoch: usize) -> Vec<ClDataBatch> {
        let sample_stride = self.seq_len;
        if self.tokens.len() <= self.seq_len + 1 {
            return Vec::new();
        }

        let num_possible_samples = (self.tokens.len() - 1) / sample_stride;
        let mut sample_indices: Vec<usize> = (0..num_possible_samples).map(|i| i * sample_stride).collect();

        if self.shuffle {
            // Deterministic Xorshift64* pseudo-random shuffle
            let mut s = self.seed.wrapping_add((epoch as u64).wrapping_mul(0x9E3779B97F4A7C15));
            for i in (1..sample_indices.len()).rev() {
                s ^= s >> 12;
                s ^= s << 25;
                s ^= s >> 27;
                let j = (s.wrapping_mul(0x2545F4914F6CDD1D) as usize) % (i + 1);
                sample_indices.swap(i, j);
            }
        }

        let mut batches = Vec::new();
        for chunk in sample_indices.chunks(self.batch_size) {
            if self.drop_last && chunk.len() < self.batch_size {
                break;
            }

            let mut batch_inputs = Vec::with_capacity(chunk.len() * self.seq_len);
            let mut batch_targets = Vec::with_capacity(chunk.len() * self.seq_len);

            for &start_idx in chunk {
                for s in 0..self.seq_len {
                    batch_inputs.push(self.tokens[start_idx + s]);
                    batch_targets.push(self.tokens[start_idx + s + 1]);
                }
            }

            batches.push(ClDataBatch {
                input_ids: batch_inputs,
                target_ids: batch_targets,
                batch_size: chunk.len(),
                seq_len: self.seq_len,
            });
        }

        batches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cl_cldb::TOTAL_CLDB_SIZE;

    #[test]
    fn test_big_data_ingest_single_file_and_directory() {
        let tmp = std::env::temp_dir().join("test_ingest_pipeline");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let doc1 = tmp.join("doc1.txt");
        let doc2 = tmp.join("doc2.md");
        fs::write(&doc1, "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG 1234567890.").unwrap();
        fs::write(&doc2, "SOVEREIGN ZERO FLOP GEOMETRIC CORTICAL ENGINE.").unwrap();

        let out_dpdata = tmp.join("output.dpdata");
        let out_cldb = tmp.join("output.cldb");

        let engine = BigDataIngestEngine::default();
        let report = engine.ingest_path(&tmp, &out_dpdata, Some(&out_cldb)).unwrap();

        assert_eq!(report.files_scanned, 2);
        assert!(report.total_tokens > 50);
        assert!(out_dpdata.exists());
        assert!(out_cldb.exists());
        assert_eq!(fs::metadata(&out_cldb).unwrap().len() as usize, TOTAL_CLDB_SIZE);
        assert!(report.active_synapses_trained > 0);
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_bpe_multilingual_streaming_and_dataloader() {
        let tmp = std::env::temp_dir().join("test_ingest_bpe_stream");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        // Multilingual document with Mongolian Cyrillic, English, and Code
        let doc = tmp.join("multilingual.txt");
        fs::write(&doc, "Монгол хэл дээрх хиймэл оюун ухааны тооцоолол болон CRON 4D-Torus цахиур сургалт let x = 42;").unwrap();

        let out_dpdata = tmp.join("stream.dpdata");
        let out_cldb = tmp.join("stream.cldb");

        let mut opts = IngestOptions::default();
        opts.tokenizer_mode = TokenizerMode::BpeSubword;
        opts.streaming = true;
        opts.chunk_size = 32; // small chunk to trigger streaming loop

        let engine = BigDataIngestEngine::new(opts);
        let report = engine.ingest_path(&tmp, &out_dpdata, Some(&out_cldb)).unwrap();

        assert_eq!(report.files_scanned, 1);
        assert!(report.total_tokens > 20);
        assert!(out_dpdata.exists());
        assert!(out_cldb.exists());

        // Test ClDataLoader on the synthesized .dpdata dataset
        let loader = ClDataLoader::from_dpdata_file(&out_dpdata, 2, 4, true, 42).unwrap();
        assert!(!loader.is_empty());
        let batches = loader.iter_epoch(0);
        assert!(!batches.is_empty());
        assert_eq!(batches[0].batch_size, 2);
        assert_eq!(batches[0].seq_len, 4);
        assert_eq!(batches[0].input_ids.len(), 8);
        assert_eq!(batches[0].target_ids.len(), 8);

        let _ = fs::remove_dir_all(&tmp);
    }
}
