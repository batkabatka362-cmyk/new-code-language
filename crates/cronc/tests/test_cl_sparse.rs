// ============================================================================
// CRON 2:4 Structural Sparsity & INT1.58 Pruning Test Suite
// ============================================================================

use cronc::cl_sparse::{
    analyze_matrix_sparsity, compress_2_4_matrix, decompress_2_4_matrix,
    prune_to_2_4, synthesize_sparse_2_4_gemm,
};
use cronc::{run_cl_jit, verify_cl_memory_access, verify_cl_program, MemcheckOptions};

#[test]
fn test_prune_to_2_4_pattern() {
    // Dense 4x4 matrix
    let dense = vec![
        1, 5, 2, 8, // row 0: magnitudes 8, 5 are top 2 -> [0, 5, 0, 8]
        -7, 3, -1, 0, // row 1: magnitudes 7, 3 are top 2 -> [-7, 3, 0, 0]
        0, 0, 0, 0, // row 2: all zeros -> [0, 0, 0, 0]
        4, -9, 2, 1, // row 3: magnitudes 9, 4 are top 2 -> [4, -9, 0, 0]
    ];

    let pruned = prune_to_2_4(&dense, 4, 4);

    let rep = analyze_matrix_sparsity(&pruned, 4, 4);
    assert!(rep.is_2_4_compliant, "Pruned matrix must be 100% 2:4 compliant");
    assert!(rep.overall_sparsity_pct >= 50.0);

    // Verify row 0 kept 5 and 8
    assert_eq!(pruned[0], 0);
    assert_eq!(pruned[1], 5);
    assert_eq!(pruned[2], 0);
    assert_eq!(pruned[3], 8);

    // Verify row 1 kept -7 and 3
    assert_eq!(pruned[4], -7);
    assert_eq!(pruned[5], 3);
    assert_eq!(pruned[6], 0);
    assert_eq!(pruned[7], 0);
}

#[test]
fn test_compress_and_decompress_2_4() {
    let original = vec![
        0, 4, 0, 9,
        -5, 0, 1, 0,
        0, 0, 3, 7,
        2, -8, 0, 0,
    ];

    let compressed = compress_2_4_matrix(&original, 4, 4)
        .expect("Compression of 2:4 matrix must succeed");

    assert_eq!(compressed.values.len(), 8); // 16 elements / 2 = 8 non-zero elements
    assert_eq!(compressed.metadata_indices.len(), 4); // 4 blocks = 4 metadata bytes

    let decompressed = decompress_2_4_matrix(&compressed);
    assert_eq!(original, decompressed, "Decompression must achieve 100% bit-exact parity");
}

#[test]
fn test_synthesize_sparse_2_4_gemm_validation() {
    let cl_code = synthesize_sparse_2_4_gemm(64, 64, 64)
        .expect("Sparse GEMM microcode synthesis must succeed");

    // 1. Verify VLIW syntax
    let rep = verify_cl_program(&cl_code).expect("Synthesized microcode must pass .cl parser");
    assert!(rep.total_bundles >= 4);

    // 2. Verify 16-bank conflict freedom
    let mem_opts = MemcheckOptions {
        enable_swizzling: true,
        ..Default::default()
    };
    let mem_rep = verify_cl_memory_access(&cl_code, &mem_opts)
        .expect("Memory access verification must succeed");
    assert!(mem_rep.is_provably_conflict_free, "Sparse microcode must be 100% bank-conflict-free");
}

#[test]
fn test_sparse_jit_execution() {
    let cl_code = synthesize_sparse_2_4_gemm(32, 32, 32).unwrap();
    let stats = run_cl_jit(&cl_code).expect("JIT execution of sparse GEMM must succeed");
    assert!(stats.is_halted, "Core must halt cleanly");
}

#[test]
fn test_sparsity_ascii_and_json_reports() {
    let matrix = vec![0, 3, 0, -4, 2, 0, 0, 1];
    let rep = analyze_matrix_sparsity(&matrix, 2, 4);

    let ascii = rep.render_ascii_report();
    assert!(ascii.contains("CRON 2:4 STRUCTURAL SPARSITY"));
    assert!(ascii.contains("2:4 Structural Compliance"));

    let json = rep.to_json();
    assert!(json.contains("\"is_2_4_compliant\": true"));
    assert!(json.contains("\"theoretical_speedup\": 2.00"));
}
