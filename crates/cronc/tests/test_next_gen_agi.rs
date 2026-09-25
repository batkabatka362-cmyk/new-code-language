// ============================================================================
// CRON Next-Generation AGI Verification Test Suite
// Verifies:
// 1. Interactive 6-Brain SAGI Chat Engine (cl_chat)
// 2. Automated Universal AOT Compiler (cl_aot)
// 3. 65,536-Core Wafer-Scale Supercomputer 8D Topology (cl_wafer_scale)
// 4. Self-Adapting Neuromorphic JIT & Dynamic Plasticity (cl_plasticity)
// ============================================================================

use cronc::cl_chat::SagiChatSession;
use cronc::cl_aot::{compile_cl_aot, AotBuildConfig, AotBackend, detect_native_compiler};
use cronc::cl_wafer_scale::{Coord8D, WaferScaleConfig, generate_wafer_plan, render_ascii_wafer_map, TOTAL_WAFER_CORES};
use cronc::cl_plasticity::{SelfAdaptingJitEngine, PlasticityConfig};
use cronc::cl_jit::ClJitCore;

#[test]
fn test_sagi_interactive_chat_6brain_session() {
    let mut session = SagiChatSession::new();
    assert_eq!(session.history.len(), 0);

    let (response, telemetry) = session.send_message("What are your 6 brains?", 32, 0.7);
    assert!(!response.is_empty(), "Response must not be empty");
    assert!(session.history.len() >= 2, "History must record user and assistant messages");

    assert!(telemetry.brain1_causal_hops > 0);
    assert!(telemetry.brain2_optical_gemm_ops > 0);
    assert!(telemetry.brain3_reversible_steps > 0);
    assert!(telemetry.brain4_stdp_potentiations > 0);
    assert!(telemetry.brain5_cordic_rotations > 0);
    assert!(telemetry.brain6_sentry_checks > 0);
    assert!(telemetry.total_4d_torus_packets > 0);

    let hud = session.render_ascii_hud(&telemetry);
    assert!(hud.contains("SAGI 256-CORE 4D-TORUS NEOCORTICAL LIVE ACTIVITY HUD"));
    assert!(hud.contains("Brain 1 - Causal KG"));
}

#[test]
fn test_aot_universal_compiler_build() {
    if detect_native_compiler().is_none() {
        return; // Skip if no compiler on host
    }

    let cl_code = r#"
    B0000: '==01#00A> '==02#005> _NO00#000> _NO00#000>
    B0001: _PO03M102> _PO04S102> _PO05B100> _HL00#000!
    "#;

    let config = AotBuildConfig {
        backend: AotBackend::C23,
        opt_level: 3,
        enable_lto: false,
        strip_symbols: false,
        output_path: None,
        target_triple: None,
    };

    let report = compile_cl_aot(cl_code, "test_aot_kernel", &config).expect("AOT compilation must succeed");
    assert!(report.success);
    assert!(report.target_binary_bytes > 0);
    assert!(report.compilation_time_ms > 0.0);
}

#[test]
fn test_wafer_scale_65536_cores_8d_topology() {
    assert_eq!(TOTAL_WAFER_CORES, 65536);

    // Bijective mapping across 8D coordinate space
    let origin = Coord8D::from_global_id(0);
    assert_eq!(origin.to_global_id(), 0);
    assert_eq!(origin.core_x, 0);
    assert_eq!(origin.die_x, 0);

    let last_core = Coord8D::from_global_id(65535);
    assert_eq!(last_core.to_global_id(), 65535);
    assert_eq!(last_core.core_w, 3);
    assert_eq!(last_core.die_w, 3);

    let (intra, inter) = origin.distance_to(&last_core);
    assert!(intra <= 8, "Max intra-die 4D torus distance is 8 hops");
    assert!(inter <= 8, "Max inter-die 4D torus distance is 8 hops");

    let config = WaferScaleConfig::default();
    let plan = generate_wafer_plan(&config);
    assert_eq!(plan.total_active_cores, 65536);
    assert_eq!(plan.total_dies, 256);
    assert!(plan.peak_tops_int2 > 10000.0, "Wafer must achieve >10,000 TOPS");
    assert!(plan.bisection_bandwidth_tb_s > 50.0);
    assert!(plan.estimated_power_watts < 10000.0, "Wafer power must be <10 kW");

    let ascii_map = render_ascii_wafer_map(&plan);
    assert!(ascii_map.contains("CRON 65,536-CORE FULL WAFER-SCALE SUPERCOMPUTER"));
    assert!(ascii_map.contains("16x16 MONOLITHIC SILICON DIE TOPOLOGY"));
}

#[test]
fn test_self_adapting_plasticity_and_hot_patching() {
    let config = PlasticityConfig {
        hot_loop_threshold: 5,
        stdp_learning_rate: 0.1,
        tau_decay_us: 10.0,
        enable_auto_fma_fusion: true,
        energy_target_pj_per_op: 0.05,
    };

    let mut engine = SelfAdaptingJitEngine::new(config);
    let sample_bundle = "B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>";

    // Execute multiple iterations
    for _ in 0..10 {
        let _patch = engine.profile_and_adapt_bundle(sample_bundle);
    }

    let mut core = ClJitCore::new();
    engine.sync_to_core(&mut core);
    assert!(core.stdp_updates_count > 0);

    let report = engine.generate_report();
    assert!(report.total_executed_cycles >= 10);
    assert_eq!(report.active_synaptic_weights.len(), 16);
}
