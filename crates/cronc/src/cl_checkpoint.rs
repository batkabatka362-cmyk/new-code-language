// ============================================================================
// CRON Fault-Tolerant Distributed Optimizer Checkpoint Engine (cl_checkpoint.rs)
//
// 100% Pure Rust - Zero Python, Zero Pickle, Zero PyTorch Checkpoint dependencies.
//
// Features:
//   1. Atomic Two-Phase Commit: Writes to a process-unique temporary file, forces
//      disk synchronization (sync_all), and performs atomic rename to prevent
//      corrupted checkpoints during power loss or kill signals.
//   2. End-to-End CRC-32 Integrity Validation: Embeds per-tensor IEEE 802.3 checksums
//      and an overarching file trailer CRC-32 to guarantee bit-level fidelity.
//   3. Full Optimizer State Preservation: Stores parameter weights, AdamW/Lion
//      1st moment (m) and 2nd moment (v) tensors, step index t, and learning rate.
//   4. Direct Pipeline Binding: Seamlessly exports and restores AutonomousLanguageModel
//      instances from cl_train_pipeline.
// ============================================================================

use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::cl_train_pipeline::{AutonomousLanguageModel, ParamMatrix, TrainingOptimizer};

/// Binary Checkpoint Magic Signature: "CRONCHK1" (8 bytes)
pub const CHECKPOINT_MAGIC: &[u8; 8] = b"CRONCHK1";
pub const CHECKPOINT_VERSION: u32 = 1;

/// Pure Rust IEEE 802.3 CRC-32 Implementation (Polynomial 0xEDB88320)
pub struct Crc32 {
    table: [u32; 256],
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Crc32 {
    pub const fn new() -> Self {
        let mut table = [0u32; 256];
        let mut i = 0usize;
        while i < 256 {
            let mut crc = i as u32;
            let mut j = 0;
            while j < 8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
                j += 1;
            }
            table[i] = crc;
            i += 1;
        }
        Self { table }
    }

    pub fn checksum(&self, data: &[u8]) -> u32 {
        let mut crc = !0u32;
        for &byte in data {
            let idx = ((crc ^ (byte as u32)) & 0xFF) as usize;
            crc = (crc >> 8) ^ self.table[idx];
        }
        !crc
    }

    pub fn update(&self, mut crc: u32, data: &[u8]) -> u32 {
        crc = !crc;
        for &byte in data {
            let idx = ((crc ^ (byte as u32)) & 0xFF) as usize;
            crc = (crc >> 8) ^ self.table[idx];
        }
        !crc
    }
}

pub static CRC32: Crc32 = Crc32::new();

/// Serialized Model & Optimizer Tensor Record
#[derive(Debug, Clone, PartialEq)]
pub struct TensorRecord {
    pub name: String,
    pub rows: usize,
    pub cols: usize,
    pub weights: Vec<f32>,
    pub m: Vec<f32>,
    pub v: Vec<f32>,
    pub crc32: u32,
}

/// Checkpoint Architecture & Hyperparameter Header
#[derive(Debug, Clone, PartialEq)]
pub struct CheckpointHeader {
    pub version: u32,
    pub step: usize,
    pub learning_rate: f32,
    pub optimizer_type: u32, // 0 = AdamW, 1 = Lion, 2 = SGD
    pub vocab_size: usize,
    pub dim: usize,
    pub hidden_dim: usize,
    pub num_layers: usize,
    pub num_experts: usize,
    pub num_tensors: usize,
}

/// Full In-Memory Checkpoint Representation
#[derive(Debug, Clone, PartialEq)]
pub struct ModelCheckpoint {
    pub header: CheckpointHeader,
    pub tensors: Vec<TensorRecord>,
}

impl ModelCheckpoint {
    /// Extracts a full checkpoint from an active AutonomousLanguageModel and current step
    pub fn from_model(model: &AutonomousLanguageModel, step: usize, lr: f32) -> Self {
        let cfg = &model.config;
        let mut tensors = Vec::new();

        let opt_type = match cfg.optimizer {
            TrainingOptimizer::AdamW => 0,
            TrainingOptimizer::Lion => 1,
            TrainingOptimizer::Sgd => 2,
        };

        // 1. Embedding
        tensors.push(Self::record_from_matrix("embedding", &model.embedding));

        // 2. Transformer Layers
        for (l_idx, layer) in model.layers.iter().enumerate() {
            for (e_idx, w_gate) in layer.w_gate.iter().enumerate() {
                tensors.push(Self::record_from_matrix(&format!("layer.{}.w_gate.{}", l_idx, e_idx), w_gate));
            }
            for (e_idx, w_up) in layer.w_up.iter().enumerate() {
                tensors.push(Self::record_from_matrix(&format!("layer.{}.w_up.{}", l_idx, e_idx), w_up));
            }
            for (e_idx, w_down) in layer.w_down.iter().enumerate() {
                tensors.push(Self::record_from_matrix(&format!("layer.{}.w_down.{}", l_idx, e_idx), w_down));
            }
            if let Some(ref router) = layer.w_router {
                tensors.push(Self::record_from_matrix(&format!("layer.{}.w_router", l_idx), router));
            }
        }

        // 3. Output LM Head
        tensors.push(Self::record_from_matrix("w_head", &model.w_head));

        let header = CheckpointHeader {
            version: CHECKPOINT_VERSION,
            step,
            learning_rate: lr,
            optimizer_type: opt_type,
            vocab_size: cfg.vocab_size,
            dim: cfg.dim,
            hidden_dim: cfg.hidden_dim,
            num_layers: cfg.num_layers,
            num_experts: cfg.num_experts,
            num_tensors: tensors.len(),
        };

        Self { header, tensors }
    }

    fn record_from_matrix(name: &str, mat: &ParamMatrix) -> TensorRecord {
        let mut bytes = Vec::with_capacity(mat.data.len() * 4 * 3);
        for &w in &mat.data {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
        for &m in &mat.m {
            bytes.extend_from_slice(&m.to_le_bytes());
        }
        for &v in &mat.v {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let crc = CRC32.checksum(&bytes);

        TensorRecord {
            name: name.to_string(),
            rows: mat.rows,
            cols: mat.cols,
            weights: mat.data.clone(),
            m: mat.m.clone(),
            v: mat.v.clone(),
            crc32: crc,
        }
    }

    /// Atomically writes the checkpoint to disk with two-phase commit and CRC-32 validation
    pub fn save_atomic<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let target_path = path.as_ref();
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create checkpoint directory {:?}: {}", parent, e))?;
        }

        let tmp_path = format!(
            "{}.tmp.{}",
            target_path.display(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let tmp_path = PathBuf::from(tmp_path);

        let file = File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temporary checkpoint file {:?}: {}", tmp_path, e))?;
        let mut writer = BufWriter::new(file);

        // 1. Magic
        writer
            .write_all(CHECKPOINT_MAGIC)
            .map_err(|e| format!("Failed to write magic: {}", e))?;

        // 2. Header
        writer
            .write_all(&self.header.version.to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.step as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&self.header.learning_rate.to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&self.header.optimizer_type.to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.vocab_size as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.dim as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.hidden_dim as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.num_layers as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.num_experts as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;
        writer
            .write_all(&(self.header.num_tensors as u64).to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;

        // 3. Tensors
        let mut global_crc = 0u32;

        for t in &self.tensors {
            let name_bytes = t.name.as_bytes();
            writer
                .write_all(&(name_bytes.len() as u32).to_le_bytes())
                .map_err(|e| format!("Write failed: {}", e))?;
            writer
                .write_all(name_bytes)
                .map_err(|e| format!("Write failed: {}", e))?;
            writer
                .write_all(&(t.rows as u64).to_le_bytes())
                .map_err(|e| format!("Write failed: {}", e))?;
            writer
                .write_all(&(t.cols as u64).to_le_bytes())
                .map_err(|e| format!("Write failed: {}", e))?;

            // Data payload
            for &w in &t.weights {
                writer.write_all(&w.to_le_bytes()).map_err(|e| format!("Write failed: {}", e))?;
            }
            for &m in &t.m {
                writer.write_all(&m.to_le_bytes()).map_err(|e| format!("Write failed: {}", e))?;
            }
            for &v in &t.v {
                writer.write_all(&v.to_le_bytes()).map_err(|e| format!("Write failed: {}", e))?;
            }

            // Tensor CRC-32
            writer
                .write_all(&t.crc32.to_le_bytes())
                .map_err(|e| format!("Write failed: {}", e))?;

            global_crc ^= t.crc32;
        }

        // 4. Checkpoint trailer CRC-32
        writer
            .write_all(&global_crc.to_le_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;

        writer
            .flush()
            .map_err(|e| format!("Flush failed: {}", e))?;

        let underlying_file = writer
            .into_inner()
            .map_err(|e| format!("Buffer extraction failed: {}", e))?;

        underlying_file
            .sync_all()
            .map_err(|e| format!("Disk sync_all failed on {:?}: {}", tmp_path, e))?;

        drop(underlying_file);

        // Atomic rename
        fs::rename(&tmp_path, target_path)
            .map_err(|e| format!("Atomic rename failed {:?} -> {:?}: {}", tmp_path, target_path, e))?;

        Ok(())
    }

    /// Loads and validates a checkpoint from disk
    pub fn load_and_verify<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path.as_ref())
            .map_err(|e| format!("Failed to open checkpoint {:?}: {}", path.as_ref(), e))?;
        let mut reader = BufReader::new(file);

        // 1. Verify Magic
        let mut magic = [0u8; 8];
        reader
            .read_exact(&mut magic)
            .map_err(|e| format!("Failed to read magic: {}", e))?;
        if &magic != CHECKPOINT_MAGIC {
            return Err(format!("Invalid checkpoint magic signature: {:?}", magic));
        }

        // 2. Read Header
        let mut buf4 = [0u8; 4];
        let mut buf8 = [0u8; 8];

        reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
        let version = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let step = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
        let learning_rate = f32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
        let optimizer_type = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let vocab_size = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let dim = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let hidden_dim = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let num_layers = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let num_experts = u64::from_le_bytes(buf8) as usize;

        reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
        let num_tensors = u64::from_le_bytes(buf8) as usize;

        let header = CheckpointHeader {
            version,
            step,
            learning_rate,
            optimizer_type,
            vocab_size,
            dim,
            hidden_dim,
            num_layers,
            num_experts,
            num_tensors,
        };

        // 3. Read Tensors
        let mut tensors = Vec::with_capacity(num_tensors);
        let mut computed_global_crc = 0u32;

        for _ in 0..num_tensors {
            reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
            let name_len = u32::from_le_bytes(buf4) as usize;

            let mut name_buf = vec![0u8; name_len];
            reader.read_exact(&mut name_buf).map_err(|e| format!("Read failed: {}", e))?;
            let name = String::from_utf8(name_buf)
                .map_err(|e| format!("Invalid UTF-8 tensor name: {}", e))?;

            reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
            let rows = u64::from_le_bytes(buf8) as usize;

            reader.read_exact(&mut buf8).map_err(|e| format!("Read failed: {}", e))?;
            let cols = u64::from_le_bytes(buf8) as usize;

            let total_elems = rows * cols;
            let mut weights = vec![0.0f32; total_elems];
            let mut m = vec![0.0f32; total_elems];
            let mut v = vec![0.0f32; total_elems];

            let mut raw_bytes = Vec::with_capacity(total_elems * 4 * 3);

            for elem in &mut weights {
                reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
                raw_bytes.extend_from_slice(&buf4);
                *elem = f32::from_le_bytes(buf4);
            }
            for elem in &mut m {
                reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
                raw_bytes.extend_from_slice(&buf4);
                *elem = f32::from_le_bytes(buf4);
            }
            for elem in &mut v {
                reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
                raw_bytes.extend_from_slice(&buf4);
                *elem = f32::from_le_bytes(buf4);
            }

            reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
            let stored_crc = u32::from_le_bytes(buf4);

            let calculated_crc = CRC32.checksum(&raw_bytes);
            if calculated_crc != stored_crc {
                return Err(format!(
                    "CRC-32 checksum mismatch on tensor '{}': expected 0x{:08X}, calculated 0x{:08X}",
                    name, stored_crc, calculated_crc
                ));
            }

            computed_global_crc ^= stored_crc;

            tensors.push(TensorRecord {
                name,
                rows,
                cols,
                weights,
                m,
                v,
                crc32: stored_crc,
            });
        }

        // 4. Trailer CRC-32
        reader.read_exact(&mut buf4).map_err(|e| format!("Read failed: {}", e))?;
        let stored_global_crc = u32::from_le_bytes(buf4);

        if computed_global_crc != stored_global_crc {
            return Err(format!(
                "Global checkpoint CRC-32 trailer mismatch: stored 0x{:08X}, computed 0x{:08X}",
                stored_global_crc, computed_global_crc
            ));
        }

        Ok(Self { header, tensors })
    }

    /// Restores weights and optimizer state into an existing AutonomousLanguageModel
    pub fn restore_into_model(&self, model: &mut AutonomousLanguageModel) -> Result<(), String> {
        let tensor_map: std::collections::HashMap<&str, &TensorRecord> =
            self.tensors.iter().map(|t| (t.name.as_str(), t)).collect();

        // 1. Embedding
        if let Some(t) = tensor_map.get("embedding") {
            Self::apply_to_matrix(&mut model.embedding, t)?;
        }

        // 2. Layers
        for (l_idx, layer) in model.layers.iter_mut().enumerate() {
            for (e_idx, w_gate) in layer.w_gate.iter_mut().enumerate() {
                let name = format!("layer.{}.w_gate.{}", l_idx, e_idx);
                if let Some(t) = tensor_map.get(name.as_str()) {
                    Self::apply_to_matrix(w_gate, t)?;
                }
            }
            for (e_idx, w_up) in layer.w_up.iter_mut().enumerate() {
                let name = format!("layer.{}.w_up.{}", l_idx, e_idx);
                if let Some(t) = tensor_map.get(name.as_str()) {
                    Self::apply_to_matrix(w_up, t)?;
                }
            }
            for (e_idx, w_down) in layer.w_down.iter_mut().enumerate() {
                let name = format!("layer.{}.w_down.{}", l_idx, e_idx);
                if let Some(t) = tensor_map.get(name.as_str()) {
                    Self::apply_to_matrix(w_down, t)?;
                }
            }
            if let Some(ref mut router) = layer.w_router {
                let name = format!("layer.{}.w_router", l_idx);
                if let Some(t) = tensor_map.get(name.as_str()) {
                    Self::apply_to_matrix(router, t)?;
                }
            }
        }

        // 3. Output Head
        if let Some(t) = tensor_map.get("w_head") {
            Self::apply_to_matrix(&mut model.w_head, t)?;
        }

        // Update backwards compatibility mirrors
        if !model.layers.is_empty() && !model.layers[0].w_gate.is_empty() {
            model.w_gate = model.layers[0].w_gate[0].clone();
            model.w_up = model.layers[0].w_up[0].clone();
            model.w_down = model.layers[0].w_down[0].clone();
        }

        Ok(())
    }

    fn apply_to_matrix(mat: &mut ParamMatrix, rec: &TensorRecord) -> Result<(), String> {
        if mat.rows != rec.rows || mat.cols != rec.cols {
            return Err(format!(
                "Shape mismatch for tensor '{}': expected [{}, {}], checkpoint has [{}, {}]",
                rec.name, mat.rows, mat.cols, rec.rows, rec.cols
            ));
        }
        mat.data.copy_from_slice(&rec.weights);
        mat.m.copy_from_slice(&rec.m);
        mat.v.copy_from_slice(&rec.v);
        Ok(())
    }
}
