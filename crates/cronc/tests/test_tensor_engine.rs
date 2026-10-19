// ============================================================================
// CRON & CL TEST SUITE: Real Tensor Compute Engine
// Tests AVX2/FMA GEMM, NVFP4, BitNet 1.58b, QuaRot FWHT, FlashAttention-3, Mamba-2
// ============================================================================

use cronc::cl_tensor_engine::*;

#[test]
fn test_gemm_f32_analytical_correctness() {
    // 2x2 matrix multiplication:
    // [1, 2] x [5, 6] = [1*5 + 2*7, 1*6 + 2*8] = [19, 22]
    // [3, 4]   [7, 8]   [3*5 + 4*7, 3*6 + 4*8] = [43, 50]
    let a = TensorData::from_f32_slice(vec![2, 2], &[1.0, 2.0, 3.0, 4.0]);
    let b = TensorData::from_f32_slice(vec![2, 2], &[5.0, 6.0, 7.0, 8.0]);
    let mut c = TensorData::zeros(vec![2, 2], TensorDtype::FP32);

    tensor_gemm_f32(&a, &b, &mut c, 1.0, 0.0).expect("GEMM failed");

    let out = c.to_f32_vec();
    assert_eq!(out, vec![19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn test_gemm_f32_large_tiled_simd() {
    // 64x64 matrix multiplication (exercises AVX2 8-wide vector lanes)
    let m = 64;
    let k = 64;
    let n = 64;

    let a = TensorData::random_uniform(vec![m, k], 12345);
    let b = TensorData::random_uniform(vec![k, n], 67890);
    let mut c = TensorData::zeros(vec![m, n], TensorDtype::FP32);

    tensor_gemm_f32(&a, &b, &mut c, 1.0, 0.0).expect("Large GEMM failed");

    // Verify first row and column analytically against naive dot product
    let a_vec = a.to_f32_vec();
    let b_vec = b.to_f32_vec();
    let c_vec = c.to_f32_vec();

    for test_row in [0, 15, 31, 63] {
        for test_col in [0, 7, 23, 63] {
            let mut expected = 0.0f32;
            for p in 0..k {
                expected += a_vec[test_row * k + p] * b_vec[p * n + test_col];
            }
            let actual = c_vec[test_row * n + test_col];
            let diff = (expected - actual).abs();
            assert!(
                diff < 1e-4,
                "Mismatch at [{}, {}]: expected {}, got {}, diff {}",
                test_row, test_col, expected, actual, diff
            );
        }
    }
}

#[test]
fn test_bitnet_158_gemv_zero_multiply_popcnt() {
    // 4 weights: [+1, -1, 0, +1]
    // 4 activations: [10, 20, 30, 40]
    // Expected result = 10*(+1) + 20*(-1) + 30*(0) + 40*(+1) = 10 - 20 + 0 + 40 = 30
    let weights_trit = TensorData::from_f32_slice(vec![1, 4], &[1.0, -1.0, 0.0, 1.0]);
    let activations = vec![10i32, 20i32, 30i32, 40i32];
    let mut out = vec![0i32; 1];

    tensor_gemv_bitnet158(&activations, &weights_trit, &mut out).expect("BitNet GEMV failed");
    assert_eq!(out[0], 30);
}

#[test]
fn test_rmsnorm_f32() {
    // Vector with 4 elements [2, 2, 2, 2] -> RMS = sqrt(mean(4, 4, 4, 4)) = 2.
    // Normalized = [2/2, 2/2, 2/2, 2/2] = [1, 1, 1, 1]
    let x = TensorData::from_f32_slice(vec![1, 4], &[2.0, 2.0, 2.0, 2.0]);
    let mut out = TensorData::zeros(vec![1, 4], TensorDtype::FP32);

    tensor_rmsnorm_f32(&x, None, 0.0, &mut out).expect("RMSNorm failed");
    let out_vec = out.to_f32_vec();
    for &val in &out_vec {
        assert!((val - 1.0).abs() < 1e-5);
    }
}

#[test]
fn test_softmax_f32() {
    let x = TensorData::from_f32_slice(vec![1, 4], &[1.0, 2.0, 3.0, 4.0]);
    let mut out = TensorData::zeros(vec![1, 4], TensorDtype::FP32);

    tensor_softmax_f32(&x, &mut out).expect("Softmax failed");
    let out_vec = out.to_f32_vec();

    // Sum of softmax must equal 1.0
    let sum: f32 = out_vec.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
    // Probabilities must be strictly monotonically increasing
    assert!(out_vec[0] < out_vec[1]);
    assert!(out_vec[1] < out_vec[2]);
    assert!(out_vec[2] < out_vec[3]);
}

#[test]
fn test_quarot_fwht_orthogonality() {
    // Applying Hadamard transform twice must yield identity: H(H(x)) = x
    let orig = vec![1.0, 3.0, 5.0, 7.0, -2.0, 4.0, -6.0, 8.0];
    let mut tensor = TensorData::from_f32_slice(vec![8], &orig);

    tensor_quarot_fwht_f32(&mut tensor).expect("First FWHT failed");
    tensor_quarot_fwht_f32(&mut tensor).expect("Second FWHT failed");

    let reconstructed = tensor.to_f32_vec();
    for i in 0..orig.len() {
        assert!(
            (orig[i] - reconstructed[i]).abs() < 1e-4,
            "Reconstruction error at index {}: orig {}, got {}",
            i, orig[i], reconstructed[i]
        );
    }
}

#[test]
fn test_flash_attention_3_matches_standard_attention() {
    let seq_len = 8;
    let d = 4;

    let q = TensorData::random_uniform(vec![seq_len, d], 1111);
    let k = TensorData::random_uniform(vec![seq_len, d], 2222);
    let v = TensorData::random_uniform(vec![seq_len, d], 3333);

    let mut out_flash = TensorData::zeros(vec![seq_len, d], TensorDtype::FP32);
    tensor_flash_attention_3(&q, &k, &v, &mut out_flash).expect("FlashAttention-3 failed");

    // Compute standard attention: Softmax(Q * K^T / sqrt(d)) * V
    let q_vec = q.to_f32_vec();
    let k_vec = k.to_f32_vec();
    let v_vec = v.to_f32_vec();
    let scale = 1.0 / (d as f32).sqrt();

    let mut expected_out = vec![0.0f32; seq_len * d];

    for i in 0..seq_len {
        let q_row = &q_vec[i * d..(i + 1) * d];
        let mut scores = vec![0.0f32; seq_len];
        for j in 0..seq_len {
            let k_row = &k_vec[j * d..(j + 1) * d];
            let mut dot = 0.0f32;
            for p in 0..d {
                dot += q_row[p] * k_row[p];
            }
            scores[j] = dot * scale;
        }

        // Softmax
        let max_s = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let mut sum_exp = 0.0f32;
        let mut exp_scores = vec![0.0f32; seq_len];
        for j in 0..seq_len {
            let e = (scores[j] - max_s).exp();
            exp_scores[j] = e;
            sum_exp += e;
        }
        for j in 0..seq_len {
            exp_scores[j] /= sum_exp;
        }

        // Multiply by V
        for j in 0..seq_len {
            let weight = exp_scores[j];
            let v_row = &v_vec[j * d..(j + 1) * d];
            for p in 0..d {
                expected_out[i * d + p] += weight * v_row[p];
            }
        }
    }

    let actual_out = out_flash.to_f32_vec();
    for idx in 0..expected_out.len() {
        assert!(
            (expected_out[idx] - actual_out[idx]).abs() < 1e-4,
            "FlashAttention-3 mismatch at {}: expected {}, got {}",
            idx, expected_out[idx], actual_out[idx]
        );
    }
}

#[test]
fn test_mamba2_ssd_scan() {
    let x = TensorData::from_f32_slice(vec![4], &[1.0, 2.0, 3.0, 4.0]);
    let a_dt = TensorData::from_f32_slice(vec![4], &[0.5, 0.5, 0.5, 0.5]);
    let mut out = TensorData::zeros(vec![4], TensorDtype::FP32);

    tensor_mamba2_ssd_scan(&x, &a_dt, &mut out).expect("Mamba2 SSD scan failed");
    let out_vec = out.to_f32_vec();

    // h_0 = 0*0.5 + 1.0 = 1.0
    // h_1 = 1.0*0.5 + 2.0 = 2.5
    // h_2 = 2.5*0.5 + 3.0 = 4.25
    // h_3 = 4.25*0.5 + 4.0 = 6.125
    assert_eq!(out_vec, vec![1.0, 2.5, 4.25, 6.125]);
}

#[test]
fn test_sphere_geodesic_flow_4d() {
    let coords = [1.0f32, 0.0, 0.0, 0.0];
    let velocity = [0.1f32, 0.2, 0.3, 0.4];
    let next = sphere_geodesic_flow_4d(&coords, &velocity, 0.05);

    // norm_sq = 1.0, decay = 1 / (1 + 0.05 * 1.0) = 1 / 1.05 = 0.95238
    assert!((next[0] - 1.1 / 1.05).abs() < 1e-4);
    assert!((next[1] - 0.2 / 1.05).abs() < 1e-4);
}
