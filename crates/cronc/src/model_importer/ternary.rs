// ============================================================================
// CRON BitNet 1.58-Bit Ternary Quantization & Kernel Engine (ternary.rs)
// Module: cronc::model_importer::ternary
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

/// Quantization statistics and parameters for a ternary tensor
#[derive(Debug, Clone, PartialEq)]
pub struct TernaryQuantParams {
    pub scale: f32,
    pub zero_point: f32,
    pub original_elements: usize,
    pub nonzero_elements: usize,
    pub sparsity_ratio: f32,
}

/// BitNet 1.58-bit ternary quantization:
/// Quantizes f32 weights into {-1, 0, +1} using the absmean scaling factor:
///   gamma = (1 / N) * sum(|W_i|)
///   W_tilde = clamp(round(W / gamma), -1, +1)
pub fn quantize_to_ternary(weights: &[f32]) -> (Vec<i8>, TernaryQuantParams) {
    if weights.is_empty() {
        return (
            Vec::new(),
            TernaryQuantParams {
                scale: 1.0,
                zero_point: 0.0,
                original_elements: 0,
                nonzero_elements: 0,
                sparsity_ratio: 0.0,
            },
        );
    }

    // 1. Calculate absmean scale gamma
    let sum_abs: f64 = weights.iter().map(|&w| (w as f64).abs()).sum();
    let gamma = (sum_abs / weights.len() as f64).max(1e-7) as f32;

    // 2. Quantize to {-1, 0, 1}
    let mut quantized = Vec::with_capacity(weights.len());
    let mut nonzeros = 0usize;

    for &w in weights {
        let scaled = w / gamma;
        let rounded = scaled.round() as i32;
        let clamped = rounded.clamp(-1, 1) as i8;
        if clamped != 0 {
            nonzeros += 1;
        }
        quantized.push(clamped);
    }

    let sparsity = 1.0 - (nonzeros as f32 / weights.len() as f32);

    let params = TernaryQuantParams {
        scale: gamma,
        zero_point: 0.0,
        original_elements: weights.len(),
        nonzero_elements: nonzeros,
        sparsity_ratio: sparsity,
    };

    (quantized, params)
}

/// Packs ternary values {-1, 0, +1} into 2-bit representations.
/// 4 ternary weights are packed per byte:
///   00: 0
///   01: +1
///   11: -1
///   10: Reserved (treated as 0)
pub fn pack_ternary_2bit(ternary: &[i8]) -> Vec<u8> {
    let num_bytes = (ternary.len() + 3) / 4;
    let mut packed = Vec::with_capacity(num_bytes);

    for chunk in ternary.chunks(4) {
        let mut byte = 0u8;
        for (i, &val) in chunk.iter().enumerate() {
            let code = match val {
                1 => 0b01,
                -1 => 0b11,
                _ => 0b00,
            };
            byte |= code << (i * 2);
        }
        packed.push(byte);
    }

    packed
}

/// Unpacks 2-bit packed bytes back into {-1, 0, +1} values.
pub fn unpack_ternary_2bit(packed: &[u8], total_elements: usize) -> Vec<i8> {
    let mut unpacked = Vec::with_capacity(total_elements);
    for &byte in packed {
        for slot in 0..4 {
            if unpacked.len() >= total_elements {
                break;
            }
            let code = (byte >> (slot * 2)) & 0b11;
            let val = match code {
                0b01 => 1i8,
                0b11 => -1i8,
                _ => 0i8,
            };
            unpacked.push(val);
        }
    }
    unpacked
}

/// Multiplication-Free Dot Product:
/// Computes:
///   y = gamma * ( sum_{w=+1} x_i - sum_{w=-1} x_i )
///
/// Features:
///   1. ZERO floating-point multiplications per weight!
///   2. Only 1 final scalar scaling multiplication by gamma.
///   3. Unpacked on-the-fly directly from 2-bit packed stream.
pub fn ternary_dot_product(packed_weights: &[u8], activations: &[f32], scale: f32) -> f32 {
    let mut pos_sum = 0.0f32;
    let mut neg_sum = 0.0f32;
    let n = activations.len();

    let mut elem_idx = 0usize;
    for &byte in packed_weights {
        for slot in 0..4 {
            if elem_idx >= n {
                break;
            }
            let code = (byte >> (slot * 2)) & 0b11;
            let x = activations[elem_idx];
            match code {
                0b01 => pos_sum += x,
                0b11 => neg_sum += x,
                _ => {}
            }
            elem_idx += 1;
        }
    }

    scale * (pos_sum - neg_sum)
}

/// Matrix-Vector Multiplication with Ternary Weights (GEMV)
/// Computes y = W * x for packed ternary matrix W [out_features x in_features]
pub fn ternary_gemv(
    packed_matrix: &[u8],
    activations: &[f32],
    scales: &[f32],
    out: &mut [f32],
    in_features: usize,
    out_features: usize,
) {
    let row_bytes = (in_features + 3) / 4;
    assert_eq!(scales.len(), out_features);
    assert!(packed_matrix.len() >= out_features * row_bytes);
    assert_eq!(out.len(), out_features);

    for r in 0..out_features {
        let row_slice = &packed_matrix[r * row_bytes..(r + 1) * row_bytes];
        out[r] = ternary_dot_product(row_slice, activations, scales[r]);
    }
}
