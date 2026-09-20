// ============================================================================
// CRON Compiler: Sub-Byte Quantization Engine (BitNet 1.58b & INT4)
// Module: cronc::model_importer::quantize
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantizationMode {
    None,
    Ternary158b,
    Int4,
}

#[derive(Debug, Clone)]
pub struct QuantizationReport {
    pub mode: QuantizationMode,
    pub original_bytes: usize,
    pub quantized_bytes: usize,
    pub compression_ratio: f32, // e.g. 0.0625 for 16x reduction
    pub memory_reduction_percent: f32, // e.g. 93.75%
    pub scale_factor: f32,
    pub mean_squared_error: f32,
}

#[derive(Debug, Clone)]
pub struct QuantizedTensor {
    pub name: String,
    pub shape: Vec<usize>,
    pub mode: QuantizationMode,
    pub packed_words: Vec<u32>, // Packed 32-bit registers for SIMD
    pub scales: Vec<f32>,       // Per-channel or per-tensor scale
    pub report: QuantizationReport,
}

/// Quantize an array of f32 weights into BitNet 1.58b Ternary representation ({-1, 0, +1}).
///
/// Mathematical formulation:
///   gamma = (1 / N) * sum(|W_i|)
///   W_q   = round(clamp(W_i / gamma, -1.0, 1.0)) in {-1, 0, 1}
///
/// In CRON 32-bit hardware registers:
///   Each 2-bit slot encodes:
///     00_2 = 0
///     01_2 = +1
///     11_2 = -1 (represented in signed 2-bit or twin-bit format)
///   16 weights are packed into one 32-bit u32 word, matching `simd_ternary_dot()`.
pub fn quantize_ternary_158b(name: &str, shape: &[usize], values: &[f32]) -> QuantizedTensor {
    if values.is_empty() {
        return QuantizedTensor {
            name: name.to_string(),
            shape: shape.to_vec(),
            mode: QuantizationMode::Ternary158b,
            packed_words: Vec::new(),
            scales: vec![1.0],
            report: QuantizationReport {
                mode: QuantizationMode::Ternary158b,
                original_bytes: 0,
                quantized_bytes: 0,
                compression_ratio: 1.0,
                memory_reduction_percent: 0.0,
                scale_factor: 1.0,
                mean_squared_error: 0.0,
            },
        };
    }

    // 1. Calculate average absolute scale gamma
    let sum_abs: f64 = values.iter().map(|&v| v.abs() as f64).sum();
    let gamma = (sum_abs / (values.len() as f64)).max(1e-8) as f32;

    // 2. Compute ternary weights and mean squared error
    let mut ternary_vals = Vec::with_capacity(values.len());
    let mut total_se = 0.0f64;

    for &w in values {
        let scaled = w / gamma;
        let clamped = scaled.max(-1.0).min(1.0);
        let q = clamped.round() as i8; // -1, 0, or 1
        ternary_vals.push(q);

        let reconstructed = (q as f32) * gamma;
        let diff = (w - reconstructed) as f64;
        total_se += diff * diff;
    }

    let mse = (total_se / (values.len() as f64)) as f32;

    // 3. Pack 16 ternary values per 32-bit word
    // Bit mapping:
    //  0 -> 0b00
    // +1 -> 0b01
    // -1 -> 0b11 (2's complement 2-bit)
    let mut packed_words = Vec::new();
    let chunk_size = 16;
    for chunk in ternary_vals.chunks(chunk_size) {
        let mut word: u32 = 0;
        for (idx, &q) in chunk.iter().enumerate() {
            let bits = match q {
                1 => 0b01u32,
                -1 => 0b11u32,
                _ => 0b00u32,
            };
            word |= bits << (idx * 2);
        }
        packed_words.push(word);
    }

    let original_bytes = values.len() * 4;
    let quantized_bytes = packed_words.len() * 4;
    let compression_ratio = quantized_bytes as f32 / original_bytes.max(1) as f32;
    let memory_reduction_percent = (1.0 - compression_ratio) * 100.0;

    QuantizedTensor {
        name: name.to_string(),
        shape: shape.to_vec(),
        mode: QuantizationMode::Ternary158b,
        packed_words,
        scales: vec![gamma],
        report: QuantizationReport {
            mode: QuantizationMode::Ternary158b,
            original_bytes,
            quantized_bytes,
            compression_ratio,
            memory_reduction_percent,
            scale_factor: gamma,
            mean_squared_error: mse,
        },
    }
}

/// Quantize an array of f32 weights into INT4 signed nibble representation ([-8, 7]).
///
/// Mathematical formulation:
///   alpha = max(|W_i|) / 7.0
///   W_q   = round(clamp(W_i / alpha, -8.0, 7.0))
///
/// In CRON 32-bit hardware registers:
///   8 nibbles (4-bit signed) packed into one 32-bit u32 word.
pub fn quantize_int4(name: &str, shape: &[usize], values: &[f32]) -> QuantizedTensor {
    if values.is_empty() {
        return QuantizedTensor {
            name: name.to_string(),
            shape: shape.to_vec(),
            mode: QuantizationMode::Int4,
            packed_words: Vec::new(),
            scales: vec![1.0],
            report: QuantizationReport {
                mode: QuantizationMode::Int4,
                original_bytes: 0,
                quantized_bytes: 0,
                compression_ratio: 1.0,
                memory_reduction_percent: 0.0,
                scale_factor: 1.0,
                mean_squared_error: 0.0,
            },
        };
    }

    let max_abs = values
        .iter()
        .fold(0.0f32, |acc, &v| acc.max(v.abs()))
        .max(1e-8);
    let alpha = max_abs / 7.0;

    let mut int4_vals = Vec::with_capacity(values.len());
    let mut total_se = 0.0f64;

    for &w in values {
        let scaled = w / alpha;
        let clamped = scaled.max(-8.0).min(7.0);
        let q = clamped.round() as i8;
        int4_vals.push(q);

        let reconstructed = (q as f32) * alpha;
        let diff = (w - reconstructed) as f64;
        total_se += diff * diff;
    }

    let mse = (total_se / (values.len() as f64)) as f32;

    // Pack 8 nibbles per 32-bit word
    let mut packed_words = Vec::new();
    let chunk_size = 8;
    for chunk in int4_vals.chunks(chunk_size) {
        let mut word: u32 = 0;
        for (idx, &q) in chunk.iter().enumerate() {
            let nibble = (q as u8) & 0x0F;
            word |= (nibble as u32) << (idx * 4);
        }
        packed_words.push(word);
    }

    let original_bytes = values.len() * 4;
    let quantized_bytes = packed_words.len() * 4;
    let compression_ratio = quantized_bytes as f32 / original_bytes.max(1) as f32;
    let memory_reduction_percent = (1.0 - compression_ratio) * 100.0;

    QuantizedTensor {
        name: name.to_string(),
        shape: shape.to_vec(),
        mode: QuantizationMode::Int4,
        packed_words,
        scales: vec![alpha],
        report: QuantizationReport {
            mode: QuantizationMode::Int4,
            original_bytes,
            quantized_bytes,
            compression_ratio,
            memory_reduction_percent,
            scale_factor: alpha,
            mean_squared_error: mse,
        },
    }
}
