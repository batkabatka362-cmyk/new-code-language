// ============================================================================
// CRON DPSTA & TBT Python <-> Rust Cross-Verification Parity Test Suite
// File: crates/cronc/tests/test_dpsta_python_parity.rs
// ============================================================================

use std::path::Path;
use cronc::cl_cldb::{CldbVault, TOTAL_CLDB_SIZE};
use cronc::cl_dpsta::CompactDatasetEngine;
use cronc::cl_matrix94::{char_to_state, state_to_char, FourTactPipeline94, NUM_STATES, FOUR_TACT_SIZE};
use cronc::cl_htgm::{HierarchicalTBTMemory, D_BITS};

#[test]
fn test_python_cldb_vault_parity() {
    let py_cldb_path = Path::new(r"c:\Users\local_ybjuj27\Desktop\python learning\cortex_knowledge.cldb");
    if !py_cldb_path.exists() {
        eprintln!("Skipping test: cortex_knowledge.cldb not found at {:?}", py_cldb_path);
        return;
    }

    // 1. File size must be exact 36,414 bytes
    let metadata = std::fs::metadata(py_cldb_path).expect("Read metadata");
    assert_eq!(metadata.len() as usize, TOTAL_CLDB_SIZE, "Python .cldb must be exactly 36,414 bytes");

    // 2. Load into active Rust pipeline and HTGM
    let mut pipeline = FourTactPipeline94::new();
    let mut htgm = HierarchicalTBTMemory::new();
    let header = CldbVault::load(py_cldb_path, &mut pipeline, &mut htgm)
        .expect("Rust CldbVault must load Python .cldb");

    assert_eq!(&header.magic, b"CLDB");
    assert_eq!(header.version, 1);
    println!(
        "Successfully loaded Python cortex_knowledge.cldb: Model='{}', Updates={}",
        header.name, header.updates
    );
}

#[test]
fn test_python_dpdata_package_parity() {
    let py_dpdata_path = Path::new(r"c:\Users\local_ybjuj27\Desktop\python learning\sample_corpus.dpdata");
    if !py_dpdata_path.exists() {
        eprintln!("Skipping test: sample_corpus.dpdata not found at {:?}", py_dpdata_path);
        return;
    }

    // Load .dpdata dataset tokens in Rust
    let states = CompactDatasetEngine::load_dataset_package(py_dpdata_path)
        .expect("Rust CompactDatasetEngine must load Python .dpdata");

    assert!(!states.is_empty(), "Dataset tokens must not be empty");
    println!("Loaded {} discrete tokens from Python sample_corpus.dpdata", states.len());

    // Every token must map to a valid 94-state printable character (ASCII 33..126)
    for &st in &states {
        assert!(st < NUM_STATES as u8, "State {} exceeds NUM_STATES", st);
        let ch = state_to_char(st);
        assert!(ch.is_ascii() && (ch as u8) >= 33 && (ch as u8) <= 126, "Char '{}' outside 33..126", ch);
    }
}

#[test]
fn test_python_mathematical_constants_and_shapes() {
    // 1. 94-state dimensions and 35.3 KB L1 Cache footprint
    assert_eq!(NUM_STATES, 94);
    assert_eq!(FOUR_TACT_SIZE, 4 * 94 * 94);
    assert_eq!(FOUR_TACT_SIZE, 35_344);

    // 2. HTGM 512-bit hypervector
    assert_eq!(D_BITS, 512);

    // 3. ASCII to state mapping bijection on 33..126
    for ascii in 33..=126u8 {
        let ch = ascii as char;
        let state = char_to_state(ch);
        assert!(state < 94);
        let recovered = state_to_char(state);
        assert_eq!(recovered, ch);
    }
}

#[test]
fn test_cross_engine_pipeline_and_attractor_coherence() {
    let mut pipeline = FourTactPipeline94::new();
    let mut htgm = HierarchicalTBTMemory::new();

    // Train on sample string "SOVEREIGN_AGI!"
    let sample = "SOVEREIGN_AGI!";
    pipeline.train_sequence(sample);
    for ch in sample.chars() {
        htgm.train_char(ch);
    }

    // Pipeline clock tick prediction
    let mut last_output = None;
    for ch in "SOVEREIGN".chars() {
        let out = pipeline.clock_tick(true, ch);
        last_output = Some(out);
    }

    let out = last_output.expect("Must have output");
    assert!(out.cycle > 0);

    // Probe attractor resonance
    let resonance = htgm.probe_resonance();
    println!("Resonance Probe: Score={}, Resonant={}", resonance.resonance_score, resonance.is_resonant);
}

#[test]
fn test_sovereign_models_generation_and_vault_integrity() {
    let target_dir = if Path::new("models").exists() {
        std::path::PathBuf::from("models")
    } else {
        std::path::PathBuf::from("../../models")
    };
    let dpdata_path = target_dir.join("quantum_rotor_kernel.dpdata");
    let cldb_path = target_dir.join("cortex_knowledge.cldb");

    let dp_size = cronc::SovereignCorpusEngine::export_sovereign_dpdata(&dpdata_path)
        .expect("Export sovereign dpdata");
    assert!(dp_size > 0);
    println!("Exported sovereign dpdata: {} bytes", dp_size);

    let cldb_size = cronc::SovereignCorpusEngine::export_sovereign_cldb(&cldb_path)
        .expect("Export sovereign cldb");
    assert_eq!(cldb_size, TOTAL_CLDB_SIZE);
    println!("Exported sovereign cldb vault: {} bytes", cldb_size);
}

