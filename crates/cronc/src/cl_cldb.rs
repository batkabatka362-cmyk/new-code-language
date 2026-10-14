//! `cl_cldb`: Crystalline Vault Database (.cldb) for DPSTA & TBT.
//!
//! Stores zero-overhead crystallized cognitive state snapshots (~36.4 KB):
//!   - 46-byte Header (Magic 'CLDB', Version, Name, Updates)
//!   - 35,344 bytes: 4-Tact Transition Matrices (4 x 94 x 94)
//!   - 1,024 bytes: 16-Slot 512-bit Attractor Ring Buffer (16 x 64B)
//!   - Total exact file size: 36,414 bytes.

use crate::cl_htgm::{HierarchicalTBTMemory, RING_DEPTH, D_U64};
use crate::cl_matrix94::{FourTactPipeline94, FOUR_TACT_SIZE};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub const CLDB_MAGIC: &[u8; 4] = b"CLDB";
pub const CLDB_VERSION: u16 = 1;
pub const HEADER_SIZE: usize = 46;
pub const ATTRACTOR_BYTES_SIZE: usize = RING_DEPTH * (D_U64 * 8); // 16 * 64 = 1,024
pub const TOTAL_CLDB_SIZE: usize = HEADER_SIZE + FOUR_TACT_SIZE + ATTRACTOR_BYTES_SIZE; // 36,414

#[derive(Clone, Debug)]
pub struct CldbHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub name: String,
    pub updates: u64,
}

pub struct CldbVault;

impl CldbVault {
    /// Saves pipeline matrices and attractor ring to a crystallized .cldb file (exactly 36,414 bytes).
    pub fn save<P: AsRef<Path>>(
        path: P,
        name: &str,
        updates: u64,
        pipeline: &FourTactPipeline94,
        htgm: &HierarchicalTBTMemory,
    ) -> Result<usize, String> {
        let mut file = File::create(path).map_err(|e| format!("Failed to create .cldb: {}", e))?;

        // 1. Header (46B)
        let mut header = [0u8; HEADER_SIZE];
        header[0..4].copy_from_slice(CLDB_MAGIC);
        header[4..6].copy_from_slice(&CLDB_VERSION.to_le_bytes());

        let name_bytes = name.as_bytes();
        let copy_len = name_bytes.len().min(32);
        header[6..6 + copy_len].copy_from_slice(&name_bytes[..copy_len]);
        header[38..46].copy_from_slice(&updates.to_le_bytes());

        file.write_all(&header)
            .map_err(|e| format!("Failed to write header: {}", e))?;

        // 2. 4-Tact Matrices (35,344B)
        let matrix_bytes = pipeline.to_bytes();
        if matrix_bytes.len() != FOUR_TACT_SIZE {
            return Err("Corrupted pipeline matrix size".to_string());
        }
        file.write_all(&matrix_bytes)
            .map_err(|e| format!("Failed to write matrices: {}", e))?;

        // 3. Attractor Ring (1,024B)
        let mut ring_bytes = Vec::with_capacity(ATTRACTOR_BYTES_SIZE);
        for slot in htgm.ring.iter() {
            for word in slot.vector.iter() {
                ring_bytes.extend_from_slice(&word.to_le_bytes());
            }
        }
        file.write_all(&ring_bytes)
            .map_err(|e| format!("Failed to write attractor ring: {}", e))?;

        Ok(TOTAL_CLDB_SIZE)
    }

    /// Loads crystallized .cldb file back into active in-compute state in microseconds.
    pub fn load<P: AsRef<Path>>(
        path: P,
        pipeline: &mut FourTactPipeline94,
        htgm: &mut HierarchicalTBTMemory,
    ) -> Result<CldbHeader, String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open .cldb: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read .cldb: {}", e))?;

        if buffer.len() != TOTAL_CLDB_SIZE {
            return Err(format!(
                "Invalid .cldb file size: expected {}, got {}",
                TOTAL_CLDB_SIZE,
                buffer.len()
            ));
        }

        // 1. Verify Magic
        if &buffer[0..4] != CLDB_MAGIC {
            return Err("Invalid .cldb magic header".to_string());
        }

        let version = u16::from_le_bytes([buffer[4], buffer[5]]);
        let name_raw = &buffer[6..38];
        let name_end = name_raw.iter().position(|&b| b == 0).unwrap_or(32);
        let name = String::from_utf8_lossy(&name_raw[..name_end]).to_string();
        let updates = u64::from_le_bytes(buffer[38..46].try_into().unwrap());

        // 2. Load 4-Tact Matrices
        let matrix_slice = &buffer[HEADER_SIZE..HEADER_SIZE + FOUR_TACT_SIZE];
        pipeline.load_from_bytes(matrix_slice)?;

        // 3. Load Attractor Ring
        let ring_start = HEADER_SIZE + FOUR_TACT_SIZE;
        let mut offset = ring_start;
        for slot in htgm.ring.iter_mut() {
            for word in slot.vector.iter_mut() {
                *word = u64::from_le_bytes(buffer[offset..offset + 8].try_into().unwrap());
                offset += 8;
            }
            slot.valid = true;
        }

        Ok(CldbHeader {
            magic: *CLDB_MAGIC,
            version,
            name,
            updates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cldb_roundtrip_snapshot() {
        let mut pipe = FourTactPipeline94::new();
        let mut htgm = HierarchicalTBTMemory::new();

        pipe.train_sequence("fn quantum_vault() -> bool { true }");
        htgm.train_text("let sovereign = 1;");

        let tmp_path = std::env::temp_dir().join("test_quantum_vault.cldb");
        let size = CldbVault::save(&tmp_path, "quantum_sphere", 42, &pipe, &htgm).unwrap();
        assert_eq!(size, TOTAL_CLDB_SIZE);

        let mut pipe_loaded = FourTactPipeline94::new();
        let mut htgm_loaded = HierarchicalTBTMemory::new();
        let header = CldbVault::load(&tmp_path, &mut pipe_loaded, &mut htgm_loaded).unwrap();

        assert_eq!(header.name, "quantum_sphere");
        assert_eq!(header.updates, 42);
        assert_eq!(pipe_loaded.to_bytes(), pipe.to_bytes());

        let _ = fs::remove_file(tmp_path);
    }
}
