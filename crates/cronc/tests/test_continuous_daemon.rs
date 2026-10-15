use cronc::{
    ContinuousLearningDaemon, DaemonConfig, TOTAL_CLDB_SIZE,
};
use std::fs;

#[test]
fn test_daemon_incremental_file_ingestion() {
    let tmp_dir = std::env::temp_dir().join("cron_test_daemon_ingest");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).expect("Create tmp dir");

    let watch_dir = tmp_dir.join("watch_docs");
    fs::create_dir_all(&watch_dir).expect("Create watch dir");

    let cldb_path = tmp_dir.join("cortex.cldb");

    let config = DaemonConfig {
        watch_paths: vec![watch_dir.clone()],
        cldb_path: cldb_path.clone(),
        dpdata_path: None,
        sleep_idle_secs: 10,
        novelty_threshold: 0.05,
        prune_threshold: 3,
        max_files_per_cycle: 10,
    };

    let mut daemon = ContinuousLearningDaemon::new(config);

    // Initial cycle: no files yet
    let report0 = daemon.step_cycle();
    assert_eq!(report0.files_ingested, 0);

    // Add a new file
    let doc1 = watch_dir.join("theory.txt");
    fs::write(&doc1, "CRON 4D Torus Neuromorphic Accelerator 0-FLOPs Multiplier Free Computing")
        .expect("Write doc1");

    let report1 = daemon.step_cycle();
    assert_eq!(report1.files_ingested, 1);
    assert!(report1.tokens_processed > 0);
    assert_eq!(daemon.total_files_ingested, 1);
    assert!(!daemon.sleep_engine.short_term_buffer.is_empty(), "Episodic traces must be recorded in Hippocampal buffer");

    // Scanning again without changes should not re-ingest
    let report2 = daemon.step_cycle();
    assert_eq!(report2.files_ingested, 0);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_daemon_idle_triggers_hippocampal_swr_and_cldb_crystallization() {
    let tmp_dir = std::env::temp_dir().join("cron_test_daemon_swr");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).expect("Create tmp dir");

    let watch_dir = tmp_dir.join("watch_swr");
    fs::create_dir_all(&watch_dir).expect("Create watch dir");

    let cldb_path = tmp_dir.join("crystallized_vault.cldb");

    let config = DaemonConfig {
        watch_paths: vec![watch_dir.clone()],
        cldb_path: cldb_path.clone(),
        dpdata_path: None,
        sleep_idle_secs: 0, // Instant sleep trigger when idle
        novelty_threshold: 0.05,
        prune_threshold: 4,
        max_files_per_cycle: 10,
    };

    let mut daemon = ContinuousLearningDaemon::new(config);

    // Ingest sample data
    let doc = watch_dir.join("sample.md");
    fs::write(&doc, "# Quantum Rotor and Clifford Algebra S^3 Sphere Matrix")
        .expect("Write doc");

    let report_ingest = daemon.step_cycle();
    assert_eq!(report_ingest.files_ingested, 1);

    // Next cycle: Idle timeout triggers Hippocampal SWR consolidation and .cldb lock
    let report_sleep = daemon.step_cycle();
    assert!(report_sleep.swr_ripples_executed > 0, "SWR ripples must be executed");
    assert!(report_sleep.cldb_persisted, ".cldb vault snapshot must be persisted");
    assert_eq!(report_sleep.cldb_bytes, TOTAL_CLDB_SIZE, "Persisted .cldb must be exactly 36,414 bytes");

    assert!(cldb_path.exists());
    let meta = fs::metadata(&cldb_path).expect("Read metadata");
    assert_eq!(meta.len() as usize, TOTAL_CLDB_SIZE);

    // Short-term buffer should be cleared after neocortical consolidation
    assert_eq!(daemon.sleep_engine.short_term_buffer.len(), 0);

    // Test reload from second daemon instance (zero amnesia)
    let config2 = DaemonConfig {
        watch_paths: vec![watch_dir],
        cldb_path: cldb_path.clone(),
        ..Default::default()
    };
    let mut daemon2 = ContinuousLearningDaemon::new(config2);
    let restored = daemon2.load_or_init_cldb().expect("Load cldb");
    assert!(restored, "Second daemon must restore crystallized 36,414B cognitive vault");

    // Test ASCII HUD rendering
    let hud = daemon.render_ascii_brainwave_hud();
    assert!(hud.contains("CRON AUTONOMOUS CONTINUOUS LEARNING"));
    assert!(hud.contains("36,414"));

    let _ = fs::remove_dir_all(&tmp_dir);
}
