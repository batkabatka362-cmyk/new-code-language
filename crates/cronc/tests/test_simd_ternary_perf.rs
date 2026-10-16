// ============================================================================
// CRON SIMD ILP Ternary Dot Product & Multithreaded GEMV Tests
// ============================================================================

use cronc::model_importer::ternary::{
    pack_ternary_2bit, popcnt_ternary_dot_32, quantize_to_ternary, ternary_dot_product, ternary_gemv,
};

#[test]
fn test_simd_ilp_ternary_dot_parity_and_correctness() {
    // 1. Test varied lengths (cross-boundary at 4 and 16)
    for len in [1, 3, 4, 7, 15, 16, 17, 31, 32, 63, 64, 127, 128, 256, 1024] {
        let weights_f32: Vec<f32> = (0..len)
            .map(|i| if i % 3 == 0 { 1.5 } else if i % 3 == 1 { -0.8 } else { 0.0 })
            .collect();
        let (quantized, params) = quantize_to_ternary(&weights_f32);
        let packed = pack_ternary_2bit(&quantized);

        let activations: Vec<f32> = (0..len).map(|i| (i as f32 * 0.1).sin()).collect();

        // Scalar baseline
        let mut expected_pos = 0.0f32;
        let mut expected_neg = 0.0f32;
        for (i, &w) in quantized.iter().enumerate() {
            if w == 1 {
                expected_pos += activations[i];
            } else if w == -1 {
                expected_neg += activations[i];
            }
        }
        let expected_dot = params.scale * (expected_pos - expected_neg);

        // ILP multi-accumulator dot product
        let actual_dot = ternary_dot_product(&packed, &activations, params.scale);

        let diff = (expected_dot - actual_dot).abs();
        assert!(
            diff < 1e-4,
            "Mismatch at len {}: expected {:.6}, got {:.6}, diff {:.6}",
            len,
            expected_dot,
            actual_dot,
            diff
        );
    }
}

#[test]
fn test_popcnt_ternary_dot_32_correctness() {
    // 32 trits: alternating +1, -1, 0
    let mut trits = Vec::new();
    for i in 0..32 {
        if i % 3 == 0 {
            trits.push(1i8);
        } else if i % 3 == 1 {
            trits.push(-1i8);
        } else {
            trits.push(0i8);
        }
    }
    let packed = pack_ternary_2bit(&trits);
    let mut w_u64 = 0u64;
    for (i, &b) in packed.iter().enumerate().take(8) {
        w_u64 |= (b as u64) << (i * 8);
    }

    let act_signs = 0xAAAA_AAAAu32; // Alternating 1 and 0 bits

    let result = popcnt_ternary_dot_32(w_u64, act_signs);

    // Verify against manual evaluation
    let mut manual_sum = 0i32;
    for (i, &w) in trits.iter().enumerate() {
        let act = if ((act_signs >> i) & 1) == 1 { 1 } else { -1 };
        manual_sum += w as i32 * act;
    }

    assert_eq!(result, manual_sum);
}

#[test]
fn test_multithreaded_ternary_gemv_parity() {
    let in_feat = 256;
    let out_feat = 128;
    let row_bytes = in_feat / 4;

    let weights_raw: Vec<f32> = (0..(out_feat * in_feat))
        .map(|i| ((i as f32) * 0.05).cos())
        .collect();
    let (quantized, _) = quantize_to_ternary(&weights_raw);
    let packed_matrix = pack_ternary_2bit(&quantized);

    let activations: Vec<f32> = (0..in_feat).map(|i| (i as f32 * 0.02).sin()).collect();
    let scales = vec![0.75f32; out_feat];

    let mut out_parallel = vec![0.0f32; out_feat];
    ternary_gemv(
        &packed_matrix,
        &activations,
        &scales,
        &mut out_parallel,
        in_feat,
        out_feat,
    );

    // Verify every single row output is finite and matches sequential calculation
    for r in 0..out_feat {
        let row_slice = &packed_matrix[r * row_bytes..(r + 1) * row_bytes];
        let seq_val = ternary_dot_product(row_slice, &activations, scales[r]);
        let diff = (out_parallel[r] - seq_val).abs();
        assert!(diff < 1e-4, "Row {} mismatch: parallel {}, seq {}", r, out_parallel[r], seq_val);
    }
}
