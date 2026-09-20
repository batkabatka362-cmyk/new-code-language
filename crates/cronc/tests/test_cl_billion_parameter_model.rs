// ============================================================================
// CRON Multi-Billion Parameter Foundation Model Verification Test
// Tests examples/cl/cron_bitnet_7b_foundation.cl (7.544B Parameters)
// ============================================================================

use cronc::cl_c23::compile_cl_to_c23;
use cronc::cl_heal::heal_cl_program;
use cronc::cl_link::ClLinker;
use cronc::cl_perf::{profile_cl_kernel, ClPerfConfig};
use std::fs;
use std::path::Path;

#[test]
fn test_cron_bitnet_7b_foundation_model_integrity_and_ppa() {
    let cl_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
        .join("cl")
        .join("cron_bitnet_7b_foundation.cl");

    assert!(
        cl_path.exists(),
        "cron_bitnet_7b_foundation.cl must exist at {:?}",
        cl_path
    );

    let cl_source = fs::read_to_string(&cl_path)
        .expect("Failed to read cron_bitnet_7b_foundation.cl");

    // 1. Verify Self-Healing / Hazard-Free Check
    let healed = heal_cl_program(&cl_source).expect("Healer failed on 7.5B model");
    assert_eq!(
        healed.raw_hazards_resolved, 0,
        "7.5B model must have 0 RAW data hazards"
    );
    assert_eq!(
        healed.waw_hazards_resolved, 0,
        "7.5B model must have 0 WAW write hazards"
    );

    // 2. Spatial Linker: Multi-Core 4D-Torus placement & NoC DOR proof
    let mut linker = ClLinker::new();
    let linked = linker
        .parse_and_link(&cl_source)
        .expect("Multi-core spatial linker failed on 7.5B model");

    assert!(
        linked.total_cores >= 7,
        "7.5B model must utilize distributed 4D-Torus cores"
    );
    assert!(
        linked.inter_core_channels.len() >= 20,
        "7.5B model must have inter-core NoC wormhole communication channels"
    );
    assert!(
        linked.dor_deadlock_free,
        "DOR routing must be provably deadlock-free (Dally-Seitz theorem)"
    );

    // 3. PPA Performance Profiling
    let ppa = profile_cl_kernel(&cl_source, "cron_bitnet_7b_foundation.cl", &ClPerfConfig::default());

    assert!(
        ppa.chip_tflops > 10.0,
        "Throughput must exceed 10 TFLOPs"
    );
    assert!(
        ppa.tops_per_watt > 2.0,
        "Energy efficiency must exceed 2.0 TOPS/Watt"
    );
    assert!(
        ppa.junction_temp_c < 50.0,
        "Junction temperature must remain within safe thermal operating limits"
    );

    // 4. Native C23 Transpilation Verification
    let c23_code = compile_cl_to_c23(&cl_source, "cron_bitnet_7b")
        .expect("Failed to transpile 7.5B .cl model to C23");
    assert!(
        c23_code.contains("CRON Native C23 Silicon Machine Translation"),
        "Emitted code must contain valid C23 header"
    );

    println!(
        "CRON-BitNet-7B Foundation Model verified: {} cores linked, {} NoC channels, {:.2} TOPS/Watt",
        linked.total_cores,
        linked.inter_core_channels.len(),
        ppa.tops_per_watt
    );
}
