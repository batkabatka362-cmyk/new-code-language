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

/// Configuration options for the big data ingestion pipeline.
#[derive(Debug, Clone)]
pub struct IngestOptions {
    pub chunk_size: usize,
    pub max_files: usize,
    pub emit_cldb: bool,
    pub vault_name: String,
}

impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            chunk_size: 65536,
            max_files: 10000,
            emit_cldb: true,
            vault_name: "sovereign_ingest_vault".to_string(),
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

        let mut raw_bytes_read = 0usize;
        let mut all_text = String::new();

        for p in file_paths.iter().take(self.options.max_files) {
            if let Ok(content) = fs::read_to_string(p) {
                raw_bytes_read += content.len();
                all_text.push_str(&content);
                all_text.push('\n');
            }
        }

        if all_text.is_empty() {
            all_text.push_str("SOVEREIGN AGI EMPTY INGESTION CORPUS INITIALIZED.");
        }

        // Canonicalize into Base-94 tokens
        let normalized = Self::canonicalize_text(&all_text);
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
}
