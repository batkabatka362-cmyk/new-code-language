// ============================================================================
// Unit Tests: CRON 4D-Torus Workload Balancer & Spatial Partitioning (test_cl_balance.rs)
// ============================================================================

use cronc::cl_balance::{
    balance_workload, derive_optimal_strategy, render_ascii_mesh_heatmap,
    synthesize_multicore_cl_bundle, BalanceConfig, ParallelismStrategy,
};
use cronc::cl_link::Coord4D;

#[test]
fn test_torus_coord_and_manhattan_distance() {
    let c0 = Coord4D::new(0, 0, 0, 0).unwrap();
    let c1 = Coord4D::new(3, 0, 0, 0).unwrap();

    // With wrap-around on 4-wide torus: min(|0-3|, 4 - 3) = 1 hop
    assert_eq!(c0.torus_manhattan_distance(&c1), 1);

    let c2 = Coord4D::new(2, 2, 2, 2).unwrap();
    // dx=2, dy=2, dz=2, dw=2 -> 2 + 2 + 2 + 2 = 8 hops
    assert_eq!(c0.torus_manhattan_distance(&c2), 8);
}

#[test]
fn test_strategy_validation_and_optimal_derivation() {
    let strat256 = derive_optimal_strategy(256, 32);
    assert_eq!(strat256.tp, 8);
    assert_eq!(strat256.pp, 4);
    assert_eq!(strat256.dp, 8);
    assert_eq!(strat256.total_cores(), 256);
    assert!(strat256.is_valid_for(256));
    assert!(!strat256.is_valid_for(128));

    let invalid = ParallelismStrategy::new(8, 8, 8);
    assert_eq!(invalid.total_cores(), 512);
    assert!(!invalid.is_valid_for(256));
}

#[test]
fn test_workload_balance_plan() {
    let config = BalanceConfig {
        total_cores: 256,
        tensor_m: 2048,
        tensor_k: 4096,
        tensor_n: 11008,
        layers: 32,
        micro_batches: 8,
        target_strategy: None,
    };

    let plan = balance_workload(&config).expect("Workload balance should succeed");
    assert_eq!(plan.total_cores, 256);
    assert_eq!(plan.core_assignments.len(), 256);
    assert!(plan.imbalance_ratio > 0.80);
    assert!(plan.avg_torus_hop_count > 0.0 && plan.avg_torus_hop_count < 4.0);
    assert!(plan.pipeline_bubble_fraction > 0.0 && plan.pipeline_bubble_fraction < 0.50);
    assert!(plan.estimated_speedup > 100.0);
    assert!(plan.total_gflops > 0.0);
    assert!(plan.total_communication_mb > 0.0);
}

#[test]
fn test_ascii_mesh_heatmap_rendering() {
    let config = BalanceConfig::default();
    let plan = balance_workload(&config).unwrap();
    let heatmap = render_ascii_mesh_heatmap(&plan);

    assert!(heatmap.contains("CRON 256-CORE 4D-TORUS WORKLOAD HEATMAP"));
    assert!(heatmap.contains("Legend: [██]"));
    assert!(heatmap.contains("W=0: [Input / Embeddings / Head]"));
    assert!(heatmap.contains("W=1: [Early Transformer Layers]"));
    assert!(heatmap.contains("W=2: [Middle / Deep Representations]"));
    assert!(heatmap.contains("W=3: [Late Transformer / LM Output]"));
    assert!(heatmap.contains("FLOP Imbalance Ratio:"));
    assert!(heatmap.contains("Average Torus Hop Count:"));
}

#[test]
fn test_multicore_cl_bundle_synthesis() {
    let config = BalanceConfig::default();
    let plan = balance_workload(&config).unwrap();
    let bundle = synthesize_multicore_cl_bundle(&plan);

    assert!(bundle.contains("CRON 4D-Torus Multi-Core Coordinated Execution Bundle"));
    assert!(bundle.contains("CORE_0000"));
    assert!(bundle.contains("_SB00#000!"));
    assert!(bundle.contains("_HL00#000!"));
}

#[test]
fn test_balance_json_serialization() {
    let config = BalanceConfig::default();
    let plan = balance_workload(&config).unwrap();
    let json = plan.to_json();

    assert!(json.contains("\"total_cores\": 256"));
    assert!(json.contains("\"strategy\""));
    assert!(json.contains("\"imbalance_ratio\""));
    assert!(json.contains("\"avg_torus_hop_count\""));
    assert!(json.contains("\"cores\""));
    assert!(json.contains("\"utilization_ratio\""));
}
