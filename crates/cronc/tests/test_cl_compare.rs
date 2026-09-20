// ============================================================================
// Unit Tests for CRON Real-World Competitive Benchmark Validation Engine
// Module: cronc::cl_compare
// ============================================================================

use cronc::cl_compare::{
    comparison_to_json, evaluate_workload, generate_whitepaper_markdown,
    get_system_specs, render_ascii_comparison_scoreboard, run_comparison_suite,
    BaselineFilter, WorkloadKind, BOLTZMANN_CONSTANT, LANDAUER_LIMIT_PER_BIT_JOULES,
};

#[test]
fn test_thermodynamic_constants() {
    assert!(BOLTZMANN_CONSTANT > 1.38e-23 && BOLTZMANN_CONSTANT < 1.39e-23);
    assert!(LANDAUER_LIMIT_PER_BIT_JOULES > 2.8e-21 && LANDAUER_LIMIT_PER_BIT_JOULES < 2.9e-21);
}

#[test]
fn test_workload_and_baseline_parsing() {
    assert_eq!(WorkloadKind::parse("bitnet"), Some(WorkloadKind::BitNetGemm));
    assert_eq!(WorkloadKind::parse("gemm"), Some(WorkloadKind::BitNetGemm));
    assert_eq!(WorkloadKind::parse("ringtape"), Some(WorkloadKind::RingTapeAttention));
    assert_eq!(WorkloadKind::parse("systolic"), Some(WorkloadKind::SystolicWavefront));
    assert_eq!(WorkloadKind::parse("125m"), Some(WorkloadKind::MiniLlm125M));
    assert_eq!(WorkloadKind::parse("optic"), Some(WorkloadKind::ThermodynamicOptical));
    assert_eq!(WorkloadKind::parse("unknown_nonexistent"), None);

    assert_eq!(BaselineFilter::parse("all"), Some(BaselineFilter::All));
    assert_eq!(BaselineFilter::parse("cuda"), Some(BaselineFilter::PyTorchCuda));
    assert_eq!(BaselineFilter::parse("h100"), Some(BaselineFilter::PyTorchCuda));
    assert_eq!(BaselineFilter::parse("mojo"), Some(BaselineFilter::MojoAvx512));
    assert_eq!(BaselineFilter::parse("invalid"), None);
}

#[test]
fn test_system_specifications() {
    let (cron, h100, mojo) = get_system_specs();
    assert_eq!(cron.tdp_watts, 28.0);
    assert_eq!(h100.tdp_watts, 700.0);
    assert_eq!(mojo.tdp_watts, 350.0);
    assert!(cron.peak_bandwidth_gbps > h100.peak_bandwidth_gbps);
}

#[test]
fn test_workload_evaluation_metrics() {
    let bitnet = evaluate_workload(WorkloadKind::BitNetGemm);
    assert!(bitnet.speedup_vs_pytorch > 1.0, "CRON BitNet must be faster than PyTorch H100");
    assert!(bitnet.speedup_vs_mojo > 5.0, "CRON BitNet must be significantly faster than CPU");
    assert!(bitnet.memory_reduction_vs_pytorch > 5.0, "Ternary weights must be >5x smaller than FP16");
    assert!(bitnet.energy_savings_pct_vs_pytorch > 90.0, "Energy savings must exceed 90%");

    let tape = evaluate_workload(WorkloadKind::RingTapeAttention);
    assert!(tape.speedup_vs_pytorch > 1.0);
    assert_eq!(tape.cron_silicon.dram_bandwidth_util_gbps, 0.0, "RingTape should consume 0 DRAM BW during attention");

    let systolic = evaluate_workload(WorkloadKind::SystolicWavefront);
    assert!(systolic.speedup_vs_pytorch > 1.0);

    let llm = evaluate_workload(WorkloadKind::MiniLlm125M);
    assert!(llm.cron_silicon.tokens_per_sec.unwrap() > llm.pytorch_cuda.tokens_per_sec.unwrap());
    assert!(llm.memory_reduction_vs_pytorch > 8.0);

    let optic = evaluate_workload(WorkloadKind::ThermodynamicOptical);
    assert!(optic.energy_savings_pct_vs_pytorch > 95.0);
}

#[test]
fn test_comparison_suite_report() {
    let suite = run_comparison_suite();
    assert_eq!(suite.results.len(), 5);
    assert!(suite.summary_speedup_mean > 1.5, "Average speedup must exceed 1.5x");
    assert!(suite.summary_memory_reduction_mean > 3.0, "Average memory compression must exceed 3.0x");
    assert!(suite.summary_energy_savings_mean > 90.0, "Average energy savings must exceed 90%");
}

#[test]
fn test_ascii_scoreboard_rendering() {
    let suite = run_comparison_suite();
    let ascii = render_ascii_comparison_scoreboard(&suite);
    assert!(ascii.contains("REAL-WORLD PERFORMANCE & EFFICIENCY SCOREBOARD"));
    assert!(ascii.contains("CRON 256-Core 4D-Torus"));
    assert!(ascii.contains("NVIDIA H100 SXM5 80GB"));
    assert!(ascii.contains("BitNet b1.58 Ternary GEMM"));
    assert!(ascii.contains("RingTape Streaming FlashAttn"));
    assert!(ascii.contains("GEOMETRIC MEAN ADVANTAGE"));
    assert!(ascii.contains("Zero Floating Multipliers"));
}

#[test]
fn test_json_export_validity() {
    let suite = run_comparison_suite();
    let json = comparison_to_json(&suite);
    assert!(json.starts_with('{'));
    assert!(json.ends_with("}\n"));
    assert!(json.contains("\"mean_speedup_vs_pytorch\""));
    assert!(json.contains("\"mean_memory_reduction_vs_pytorch\""));
    assert!(json.contains("\"cron_spec\""));
    assert!(json.contains("\"h100_spec\""));
    assert!(json.contains("\"workloads\""));
}

#[test]
fn test_whitepaper_markdown_generation() {
    let suite = run_comparison_suite();
    let md = generate_whitepaper_markdown(&suite);
    assert!(md.contains("# CRON: A Spatial-Cognitive Architecture & Language for Extreme-Efficiency AI"));
    assert!(md.contains("## 1. Executive Summary"));
    assert!(md.contains("## 2. Hardware Architecture & System Specifications"));
    assert!(md.contains("## 3. Mathematical Foundations of CRON Superiority"));
    assert!(md.contains("### 3.1. Zero-Multiplier BitNet b1.58 Ternary Contraction"));
    assert!(md.contains("### 3.2. RingTape $O(1)$ KV-Streaming Attention"));
    assert!(md.contains("### 3.3. Landauer Thermodynamic Limit & Optical Gating"));
    assert!(md.contains("## 4. Workload Benchmark Details"));
    assert!(md.contains("## 5. Reproduction & Verification"));
}
