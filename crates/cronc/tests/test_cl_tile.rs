// ============================================================================
// CRON Polyhedral VLIW Loop Tiler & Tensor Contraction Test Suite
// ============================================================================

use cronc::cl_tile::{
    derive_optimal_gemm_tiles, tile_conv2d, tile_gemm, TileOptions,
    CORE_SRAM_CAPACITY_BYTES,
};
use cronc::{run_cl_jit, verify_cl_memory_access, verify_cl_program, MemcheckOptions};

#[test]
fn test_derive_optimal_gemm_tiles() {
    // 128x128x128 GEMM with 4-byte FP32 / INT32 elements
    let (tm, tn, tk) = derive_optimal_gemm_tiles(128, 128, 128, CORE_SRAM_CAPACITY_BYTES, 4);

    let bytes = tm * tk * 4 + tk * tn * 4 + tm * tn * 4;
    assert!(bytes <= CORE_SRAM_CAPACITY_BYTES);
    assert!(tm >= 4 && tn >= 4 && tk >= 4);
    assert!(tm <= 128 && tn <= 128 && tk <= 128);
}

#[test]
fn test_tile_gemm_synthesis_and_validation() {
    let opts = TileOptions {
        unroll_factor: 4,
        enable_swizzling: true,
        target_subbyte_mac: false,
        ..Default::default()
    };

    let result = tile_gemm(64, 64, 64, &opts).expect("GEMM tiling must succeed");

    assert_eq!(result.original_dims, (64, 64, 64));
    assert!(result.total_tiles >= 1);
    assert!(result.sram_footprint_bytes <= CORE_SRAM_CAPACITY_BYTES);
    assert!(result.sram_utilization_pct <= 100.0);
    assert!(result.estimated_ipc >= 3.0);
    assert!(result.is_conflict_free);

    // 1. Verify VLIW grammar and slot validity
    let cl_report = verify_cl_program(&result.canonical_cl_code)
        .expect(".cl code must pass syntax validation");
    assert!(cl_report.total_bundles >= 4);
    assert_eq!(cl_report.total_slots, cl_report.total_bundles * 4);

    // 2. Formally verify 16-bank conflict freedom
    let mem_opts = MemcheckOptions {
        enable_swizzling: true,
        ..Default::default()
    };
    let mem_report = verify_cl_memory_access(&result.canonical_cl_code, &mem_opts)
        .expect("Memory access check must succeed");
    assert!(
        mem_report.is_provably_conflict_free,
        "Synthesized polyhedral tiled kernel must be 100% bank-conflict-free"
    );
    assert_eq!(mem_report.conflicted_cycles, 0);
}

#[test]
fn test_tile_conv2d_synthesis() {
    let opts = TileOptions {
        unroll_factor: 4,
        enable_swizzling: true,
        target_subbyte_mac: true, // Sub-byte INT2 weights
        ..Default::default()
    };

    let result = tile_conv2d(32, 64, 16, 3, &opts).expect("Conv2D tiling must succeed");

    assert!(result.operation_type.contains("Conv2D"));
    assert!(result.total_tiles >= 1);
    assert!(result.is_conflict_free);

    let cl_report = verify_cl_program(&result.canonical_cl_code)
        .expect("Conv2D .cl code must pass syntax validation");
    assert!(cl_report.total_bundles >= 4);
}

#[test]
fn test_tile_gemm_jit_execution() {
    let opts = TileOptions::default();
    let result = tile_gemm(32, 32, 32, &opts).expect("Tiling should succeed");

    let core_stats = run_cl_jit(&result.canonical_cl_code).expect("JIT execution must succeed");
    assert!(core_stats.is_halted, "Kernel must halt cleanly");
    assert!(core_stats.optical_gemm_count > 0, "Optical GEMM operations must be executed");
}

#[test]
fn test_tile_ascii_and_json_reports() {
    let opts = TileOptions::default();
    let result = tile_gemm(64, 64, 64, &opts).expect("Tiling should succeed");

    let ascii = result.render_ascii_schedule();
    assert!(ascii.contains("CRON POLYHEDRAL VLIW LOOP TILER"));
    assert!(ascii.contains("SRAM Memory Footprint"));
    assert!(ascii.contains("4-Way VLIW Slot Allocation Heatmap"));

    let json = result.to_json();
    assert!(json.contains("\"operation_type\":"));
    assert!(json.contains("\"tile_dims\":"));
    assert!(json.contains("\"is_conflict_free\": true"));
}

#[test]
fn test_tile_gemm_systolic_wavefront_and_bare_metal_16x16_wgmma() {
    let opts = TileOptions {
        unroll_factor: 4,
        enable_swizzling: true,
        target_subbyte_mac: true, // BitNet ternary MACs
        systolic_wavefront: true, // Befunge 2D systolic flow
    };

    let result = tile_gemm(32, 32, 32, &opts).expect("Systolic tiling should succeed");

    assert!(result.operation_type.contains("Systolic Wavefront"));
    assert_eq!(result.estimated_ipc, 4.0);

    // Verify directional Befunge routing opcodes in emitted VLIW
    assert!(result.canonical_cl_code.contains("_DE"), "Expected East routing _DE");
    assert!(result.canonical_cl_code.contains("_DS"), "Expected South routing _DS");
    assert!(result.canonical_cl_code.contains("_MD"), "Expected sub-byte MAC _MD");

    // Verify ASCII visualizer shows systolic status
    let ascii = result.render_ascii_schedule();
    assert!(ascii.contains("Systolic Wavefront"));
    assert!(ascii.contains("Befunge 2D NoC Torus Dataflow"));

    // Verify ThunderKittens-style 16x16 Bare-Metal WGMMA Tile primitive
    let wgmma_res = cronc::cl_tile::tile_16x16_wgmma(true).expect("16x16 WGMMA should succeed");
    assert_eq!(wgmma_res.tile_dims, (16, 16, 16));
    assert_eq!(wgmma_res.total_bundles, 16);
    assert!(wgmma_res.canonical_cl_code.contains("_MD"));
    assert!(wgmma_res.canonical_cl_code.contains("_LD"));
    assert!(wgmma_res.canonical_cl_code.contains("_TT"));

    // Verify pure .cl microcode parses cleanly through validator
    let report = verify_cl_program(&wgmma_res.canonical_cl_code).expect("Validation should pass");
    assert_eq!(report.hazards.len(), 0, "16x16 WGMMA must be hazard-free");
}
