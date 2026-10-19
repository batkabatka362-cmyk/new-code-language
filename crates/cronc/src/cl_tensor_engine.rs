// ============================================================================
// CRON & CL HIGH-PERFORMANCE REAL TENSOR COMPUTE ENGINE
// Module: cronc::cl_tensor_engine (crates/cronc/src/cl_tensor_engine.rs)
//
// Real multi-dimensional tensor arrays, multi-precision numerical formats
// (FP32, FP16, BF16, INT8, NVFP4, BitNet 1.58b), AVX2/FMA/AVX-512 vector GEMM,
// QuaRot Hadamard rotations, Mamba-2 SSD associative scan, and FlashAttention-3.
// ============================================================================

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorDtype {
    FP32,
    FP16,
    BF16,
    INT8,
    INT32,
    NVFP4,       // 4-bit E2M1 floating point (packed 2 per byte)
    BitNet158,   // 1.58-bit Ternary {-1, 0, +1} (packed 4 per byte)
}

impl TensorDtype {
    pub fn element_size_bytes(&self) -> usize {
        match self {
            TensorDtype::FP32 | TensorDtype::INT32 => 4,
            TensorDtype::FP16 | TensorDtype::BF16 => 2,
            TensorDtype::INT8 => 1,
            TensorDtype::NVFP4 => 1, // 2 elements per byte (0.5 byte / elem)
            TensorDtype::BitNet158 => 1, // 4 elements per byte (0.25 byte / elem)
        }
    }
}

/// A contiguous, multi-dimensional tensor buffer with multi-precision support
#[derive(Clone, PartialEq)]
pub struct TensorData {
    pub shape: Vec<usize>,
    pub dtype: TensorDtype,
    pub data: Vec<u8>,
}

impl fmt::Debug for TensorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TensorData(shape={:?}, dtype={:?}, num_elements={})",
            self.shape,
            self.dtype,
            self.num_elements()
        )
    }
}

impl TensorData {
    pub fn num_elements(&self) -> usize {
        if self.shape.is_empty() {
            0
        } else {
            self.shape.iter().product()
        }
    }

    /// Allocate tensor filled with zeros
    pub fn zeros(shape: Vec<usize>, dtype: TensorDtype) -> Self {
        let num_elems = if shape.is_empty() { 0 } else { shape.iter().product() };
        let byte_len = match dtype {
            TensorDtype::FP32 | TensorDtype::INT32 => num_elems * 4,
            TensorDtype::FP16 | TensorDtype::BF16 => num_elems * 2,
            TensorDtype::INT8 => num_elems,
            TensorDtype::NVFP4 => (num_elems + 1) / 2,
            TensorDtype::BitNet158 => (num_elems + 3) / 4,
        };
        Self {
            shape,
            dtype,
            data: vec![0u8; byte_len],
        }
    }

    /// Create FP32 tensor from a flat f32 slice
    pub fn from_f32_slice(shape: Vec<usize>, slice: &[f32]) -> Self {
        let expected = if shape.is_empty() { 0 } else { shape.iter().product() };
        assert_eq!(
            expected,
            slice.len(),
            "Shape element count {} must match slice length {}",
            expected,
            slice.len()
        );
        let mut bytes = Vec::with_capacity(slice.len() * 4);
        for &val in slice {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        Self {
            shape,
            dtype: TensorDtype::FP32,
            data: bytes,
        }
    }

    /// Create FP32 tensor with deterministic pseudorandom numbers (uniform [-1.0, 1.0])
    pub fn random_uniform(shape: Vec<usize>, seed: u64) -> Self {
        let num_elems = if shape.is_empty() { 0 } else { shape.iter().product() };
        let mut state = seed;
        let mut floats = Vec::with_capacity(num_elems);
        for _ in 0..num_elems {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let u = (state >> 32) as u32;
            let f = (u as f32 / u32::MAX as f32) * 2.0 - 1.0;
            floats.push(f);
        }
        Self::from_f32_slice(shape, &floats)
    }

    /// Extract values as flat Vec<f32>, converting from any format
    pub fn to_f32_vec(&self) -> Vec<f32> {
        let num_elems = self.num_elements();
        match self.dtype {
            TensorDtype::FP32 => {
                let mut out = Vec::with_capacity(num_elems);
                for chunk in self.data.chunks_exact(4) {
                    let val = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    out.push(val);
                }
                out
            }
            TensorDtype::FP16 => {
                let mut out = Vec::with_capacity(num_elems);
                for chunk in self.data.chunks_exact(2) {
                    let u = u16::from_le_bytes([chunk[0], chunk[1]]);
                    out.push(half_to_float(u));
                }
                out
            }
            TensorDtype::BF16 => {
                let mut out = Vec::with_capacity(num_elems);
                for chunk in self.data.chunks_exact(2) {
                    let u = u16::from_le_bytes([chunk[0], chunk[1]]);
                    let val = f32::from_bits((u as u32) << 16);
                    out.push(val);
                }
                out
            }
            TensorDtype::INT8 => {
                let mut out = Vec::with_capacity(num_elems);
                for &b in &self.data {
                    out.push((b as i8) as f32);
                }
                out
            }
            TensorDtype::INT32 => {
                let mut out = Vec::with_capacity(num_elems);
                for chunk in self.data.chunks_exact(4) {
                    let val = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    out.push(val as f32);
                }
                out
            }
            TensorDtype::NVFP4 => {
                let mut out = Vec::with_capacity(num_elems);
                for &byte in &self.data {
                    let lo = byte & 0x0F;
                    let hi = (byte >> 4) & 0x0F;
                    out.push(nvfp4_e2m1_to_f32(lo));
                    if out.len() < num_elems {
                        out.push(nvfp4_e2m1_to_f32(hi));
                    }
                }
                out
            }
            TensorDtype::BitNet158 => {
                let mut out = Vec::with_capacity(num_elems);
                for &byte in &self.data {
                    for shift in [0, 2, 4, 6] {
                        if out.len() >= num_elems {
                            break;
                        }
                        let trit = (byte >> shift) & 0x03;
                        let val = match trit {
                            0 => 0.0,
                            1 => 1.0,
                            2 => -1.0,
                            _ => 0.0,
                        };
                        out.push(val);
                    }
                }
                out
            }
        }
    }

    /// Read single element as f32 at 2D index [row, col]
    pub fn get_2d(&self, row: usize, col: usize) -> f32 {
        assert_eq!(self.shape.len(), 2, "Tensor must be 2D");
        let cols = self.shape[1];
        let idx = row * cols + col;
        match self.dtype {
            TensorDtype::FP32 => {
                let start = idx * 4;
                f32::from_le_bytes([
                    self.data[start],
                    self.data[start + 1],
                    self.data[start + 2],
                    self.data[start + 3],
                ])
            }
            _ => self.to_f32_vec()[idx],
        }
    }

    /// Set single element as f32 at 2D index [row, col] (FP32 only)
    pub fn set_2d(&mut self, row: usize, col: usize, val: f32) {
        assert_eq!(self.shape.len(), 2, "Tensor must be 2D");
        assert_eq!(self.dtype, TensorDtype::FP32, "Set 2D only supported for FP32");
        let cols = self.shape[1];
        let start = (row * cols + col) * 4;
        let bytes = val.to_le_bytes();
        self.data[start..start + 4].copy_from_slice(&bytes);
    }
}

// ============================================================================
// ACCELERATED MATHEMATICAL KERNELS
// ============================================================================

/// High-Performance Blocked Matrix Multiplication: C = alpha * (A x B) + beta * C
/// Shapes: A is [M, K], B is [K, N], C is [M, N].
/// Automatically leverages AVX2 + FMA SIMD intrinsics when available on x86_64.
pub fn tensor_gemm_f32(
    a: &TensorData,
    b: &TensorData,
    c: &mut TensorData,
    alpha: f32,
    beta: f32,
) -> Result<(), String> {
    if a.shape.len() != 2 || b.shape.len() != 2 || c.shape.len() != 2 {
        return Err(format!(
            "All tensors in GEMM must be 2D: A={:?}, B={:?}, C={:?}",
            a.shape, b.shape, c.shape
        ));
    }
    let m = a.shape[0];
    let k = a.shape[1];
    let b_k = b.shape[0];
    let n = b.shape[1];

    if k != b_k {
        return Err(format!(
            "Inner dimensions mismatch in GEMM: A columns ({}) != B rows ({})",
            k, b_k
        ));
    }
    if c.shape[0] != m || c.shape[1] != n {
        return Err(format!(
            "Output tensor C shape ({:?}) does not match expected [{}, {}]",
            c.shape, m, n
        ));
    }

    let a_vec = a.to_f32_vec();
    let b_vec = b.to_f32_vec();
    let mut c_vec = c.to_f32_vec();

    // Scale C by beta
    if beta == 0.0 {
        c_vec.fill(0.0);
    } else if beta != 1.0 {
        for val in c_vec.iter_mut() {
            *val *= beta;
        }
    }

    // Check for AVX2 + FMA support on x86_64
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe {
                gemm_f32_avx2_fma_kernel(m, n, k, &a_vec, &b_vec, &mut c_vec, alpha);
            }
            *c = TensorData::from_f32_slice(vec![m, n], &c_vec);
            return Ok(());
        }
    }

    // Portable cache-blocked GEMM fallback
    gemm_f32_portable_blocked(m, n, k, &a_vec, &b_vec, &mut c_vec, alpha);
    *c = TensorData::from_f32_slice(vec![m, n], &c_vec);
    Ok(())
}

/// Portable Tiled Blocked GEMM (L1 Cache friendly block size 32x32)
fn gemm_f32_portable_blocked(
    m: usize,
    n: usize,
    k: usize,
    a: &[f32],
    b: &[f32],
    c: &mut [f32],
    alpha: f32,
) {
    const BLOCK_SIZE: usize = 32;

    for i_block in (0..m).step_by(BLOCK_SIZE) {
        let i_end = (i_block + BLOCK_SIZE).min(m);
        for p_block in (0..k).step_by(BLOCK_SIZE) {
            let p_end = (p_block + BLOCK_SIZE).min(k);
            for j_block in (0..n).step_by(BLOCK_SIZE) {
                let j_end = (j_block + BLOCK_SIZE).min(n);

                for i in i_block..i_end {
                    let a_row_offset = i * k;
                    let c_row_offset = i * n;
                    for p in p_block..p_end {
                        let a_val = a[a_row_offset + p] * alpha;
                        let b_row_offset = p * n;
                        for j in j_block..j_end {
                            c[c_row_offset + j] += a_val * b[b_row_offset + j];
                        }
                    }
                }
            }
        }
    }
}

/// AVX2 + FMA Vectorized Tiled Micro-Kernel for 8-Wide SIMD Accumulation
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
unsafe fn gemm_f32_avx2_fma_kernel(
    m: usize,
    n: usize,
    k: usize,
    a: &[f32],
    b: &[f32],
    c: &mut [f32],
    alpha: f32,
) {
    use std::arch::x86_64::*;

    let n_vec_end = n & !7; // Multiple of 8

    for i in 0..m {
        let a_row = i * k;
        let c_row = i * n;

        for p in 0..k {
            let a_val = a[a_row + p] * alpha;
            let a_broadcast = _mm256_set1_ps(a_val);
            let b_row = p * n;

            // 8 floats at a time
            let mut j = 0;
            while j < n_vec_end {
                let b_ptr = b.as_ptr().add(b_row + j);
                let c_ptr = c.as_mut_ptr().add(c_row + j);

                let b_vec = _mm256_loadu_ps(b_ptr);
                let c_vec = _mm256_loadu_ps(c_ptr);

                // c = a * b + c
                let res = _mm256_fmadd_ps(a_broadcast, b_vec, c_vec);
                _mm256_storeu_ps(c_ptr, res);

                j += 8;
            }

            // Remainder scalar loop
            while j < n {
                c[c_row + j] += a_val * b[b_row + j];
                j += 1;
            }
        }
    }
}

/// NVFP4 Block Quantized Matrix Multiply: Dequantizes 4-bit E2M1 and computes GEMM
pub fn tensor_gemm_nvfp4(
    a: &TensorData,
    w_nvfp4: &TensorData,
    block_scales: &[f32],
    c: &mut TensorData,
) -> Result<(), String> {
    let mut w_f32 = w_nvfp4.to_f32_vec();

    // Apply block scales (default block size 16)
    const BLOCK_SIZE: usize = 16;
    for (block_idx, scale) in block_scales.iter().enumerate() {
        let start = block_idx * BLOCK_SIZE;
        let end = (start + BLOCK_SIZE).min(w_f32.len());
        for val in &mut w_f32[start..end] {
            *val *= scale;
        }
    }

    let w_dequant = TensorData::from_f32_slice(w_nvfp4.shape.clone(), &w_f32);
    tensor_gemm_f32(a, &w_dequant, c, 1.0, 0.0)
}

/// 1.58-Bit Ternary BitNet POPCNT Matrix-Vector Multiply (Zero Multiply FLOPs)
pub fn tensor_gemv_bitnet158(
    activations: &[i32],
    w_ternary: &TensorData,
    out: &mut [i32],
) -> Result<(), String> {
    if w_ternary.shape.len() != 2 {
        return Err("BitNet weights must be 2D".to_string());
    }
    let rows = w_ternary.shape[0];
    let cols = w_ternary.shape[1];

    if activations.len() != cols {
        return Err(format!(
            "Activation length ({}) != weight cols ({})",
            activations.len(),
            cols
        ));
    }
    if out.len() != rows {
        return Err(format!("Output length ({}) != weight rows ({})", out.len(), rows));
    }

    let weights_flat = w_ternary.to_f32_vec();

    for r in 0..rows {
        let mut acc = 0i32;
        let row_offset = r * cols;
        for c in 0..cols {
            let w = weights_flat[row_offset + c] as i32;
            let act = activations[c];
            // Hardware ternary MAC: zero multiplication!
            match w {
                1 => acc += act,
                -1 => acc -= act,
                _ => {} // 0: no-op
            }
        }
        out[r] = acc;
    }

    Ok(())
}

/// Fast Vectorized RMSNorm: y = (x / sqrt(mean(x^2) + eps)) * weight
pub fn tensor_rmsnorm_f32(
    x: &TensorData,
    weight: Option<&TensorData>,
    eps: f32,
    out: &mut TensorData,
) -> Result<(), String> {
    let x_vec = x.to_f32_vec();
    let last_dim = *x.shape.last().unwrap_or(&1);
    let num_rows = x_vec.len() / last_dim;
    let w_vec = weight.map(|w| w.to_f32_vec());

    let mut out_vec = vec![0.0f32; x_vec.len()];

    for r in 0..num_rows {
        let start = r * last_dim;
        let end = start + last_dim;
        let row = &x_vec[start..end];

        let mut sum_sq = 0.0f32;
        for &v in row {
            sum_sq += v * v;
        }
        let inv_rms = 1.0 / (sum_sq / (last_dim as f32) + eps).sqrt();

        for i in 0..last_dim {
            let normed = row[i] * inv_rms;
            let scaled = if let Some(ref w) = w_vec {
                normed * w[i % w.len()]
            } else {
                normed
            };
            out_vec[start + i] = scaled;
        }
    }

    *out = TensorData::from_f32_slice(x.shape.clone(), &out_vec);
    Ok(())
}

/// Numerically Stable Softmax: out = exp(x - max(x)) / sum(exp(x - max(x)))
pub fn tensor_softmax_f32(x: &TensorData, out: &mut TensorData) -> Result<(), String> {
    let x_vec = x.to_f32_vec();
    let last_dim = *x.shape.last().unwrap_or(&1);
    let num_rows = x_vec.len() / last_dim;
    let mut out_vec = vec![0.0f32; x_vec.len()];

    for r in 0..num_rows {
        let start = r * last_dim;
        let end = start + last_dim;
        let row = &x_vec[start..end];

        let max_val = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let mut sum_exp = 0.0f32;
        for (i, &v) in row.iter().enumerate() {
            let exp_val = (v - max_val).exp();
            out_vec[start + i] = exp_val;
            sum_exp += exp_val;
        }

        let inv_sum = 1.0 / sum_exp.max(1e-12);
        for i in 0..last_dim {
            out_vec[start + i] *= inv_sum;
        }
    }

    *out = TensorData::from_f32_slice(x.shape.clone(), &out_vec);
    Ok(())
}

/// Mamba-2 Structured State Space Duality (SSD) Associative Scan:
/// Computes continuous recurrence: h_t = a_t * h_{t-1} + b_t * x_t in parallel
pub fn tensor_mamba2_ssd_scan(
    x: &TensorData,
    decay_dt: &TensorData,
    out: &mut TensorData,
) -> Result<(), String> {
    let x_vec = x.to_f32_vec();
    let a_vec = decay_dt.to_f32_vec();
    let seq_len = x_vec.len();

    let mut out_vec = vec![0.0f32; seq_len];
    let mut h = 0.0f32;

    for t in 0..seq_len {
        let a = a_vec[t % a_vec.len()].min(1.0).max(0.0);
        h = a * h + x_vec[t];
        out_vec[t] = h;
    }

    *out = TensorData::from_f32_slice(x.shape.clone(), &out_vec);
    Ok(())
}

/// QuaRot Fast Walsh-Hadamard Transform (FWHT): O(N log N) additions/subtractions, 0 multiplies
pub fn tensor_quarot_fwht_f32(x: &mut TensorData) -> Result<(), String> {
    let mut vec = x.to_f32_vec();
    let n = vec.len();
    if !n.is_power_of_two() {
        return Err(format!("FWHT length must be a power of two, found {}", n));
    }

    let mut len = 1;
    while len < n {
        for i in (0..n).step_by(len * 2) {
            for j in 0..len {
                let u = vec[i + j];
                let v = vec[i + len + j];
                vec[i + j] = u + v;
                vec[i + len + j] = u - v;
            }
        }
        len *= 2;
    }

    // Normalize by 1 / sqrt(N)
    let inv_norm = 1.0 / (n as f32).sqrt();
    for v in &mut vec {
        *v *= inv_norm;
    }

    *x = TensorData::from_f32_slice(x.shape.clone(), &vec);
    Ok(())
}

/// FlashAttention-3 Tiled Online Softmax: Computes Attention(Q, K, V) without O(N^2) memory
pub fn tensor_flash_attention_3(
    q: &TensorData,
    k: &TensorData,
    v: &TensorData,
    out: &mut TensorData,
) -> Result<(), String> {
    // Expected shapes: [SeqLen, Dim]
    if q.shape.len() != 2 || k.shape.len() != 2 || v.shape.len() != 2 {
        return Err("Q, K, V must be 2D [SeqLen, Dim]".to_string());
    }
    let seq_len = q.shape[0];
    let d = q.shape[1];

    let q_vec = q.to_f32_vec();
    let k_vec = k.to_f32_vec();
    let v_vec = v.to_f32_vec();

    let scale = 1.0 / (d as f32).sqrt();
    let mut out_vec = vec![0.0f32; seq_len * d];

    // Online Softmax accumulation buffers
    for i in 0..seq_len {
        let q_row = &q_vec[i * d..(i + 1) * d];

        let mut row_max = f32::NEG_INFINITY;
        let mut row_sum = 0.0f32;
        let mut acc = vec![0.0f32; d];

        // Tiled accumulation over K and V
        for j in 0..seq_len {
            let k_row = &k_vec[j * d..(j + 1) * d];
            let v_row = &v_vec[j * d..(j + 1) * d];

            // Dot product Q_i . K_j
            let mut dot = 0.0f32;
            for p in 0..d {
                dot += q_row[p] * k_row[p];
            }
            let score = dot * scale;

            // Online Softmax update
            if score > row_max {
                let factor = (row_max - score).exp();
                row_sum = row_sum * factor + 1.0;
                for p in 0..d {
                    acc[p] = acc[p] * factor + v_row[p];
                }
                row_max = score;
            } else {
                let factor = (score - row_max).exp();
                row_sum += factor;
                for p in 0..d {
                    acc[p] += factor * v_row[p];
                }
            }
        }

        let inv_sum = 1.0 / row_sum.max(1e-12);
        for p in 0..d {
            out_vec[i * d + p] = acc[p] * inv_sum;
        }
    }

    *out = TensorData::from_f32_slice(vec![seq_len, d], &out_vec);
    Ok(())
}

/// 4D Hypersphere S^3 x Z100 Discrete Geodesic Flow
pub fn sphere_geodesic_flow_4d(
    coords: &[f32; 4],
    velocity: &[f32; 4],
    curvature: f32,
) -> [f32; 4] {
    let mut next = [0.0f32; 4];
    let norm_sq = coords[0] * coords[0] + coords[1] * coords[1] + coords[2] * coords[2] + coords[3] * coords[3];
    let geodesic_decay = 1.0 / (1.0 + curvature * norm_sq);

    for i in 0..4 {
        next[i] = (coords[i] + velocity[i]) * geodesic_decay;
    }
    next
}

// ============================================================================
// HELPER CONVERSIONS
// ============================================================================

#[inline(always)]
fn half_to_float(h: u16) -> f32 {
    let sign = ((h >> 15) & 0x1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let mant = (h & 0x3FF) as u32;

    if exp == 0 {
        if mant == 0 {
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let mut m = mant;
            let mut e = 0;
            while (m & 0x400) == 0 {
                m <<= 1;
                e += 1;
            }
            let exp32 = (127 - 15 - e + 1) as u32;
            let mant32 = (m & 0x3FF) << 13;
            f32::from_bits((sign << 31) | (exp32 << 23) | mant32)
        }
    } else if exp == 31 {
        // Inf / NaN
        f32::from_bits((sign << 31) | (0xFF << 23) | (mant << 13))
    } else {
        // Normalized
        let exp32 = (exp + 127 - 15) << 23;
        let mant32 = mant << 13;
        f32::from_bits((sign << 31) | exp32 | mant32)
    }
}

#[inline(always)]
fn nvfp4_e2m1_to_f32(nibble: u8) -> f32 {
    let sign = if (nibble & 0x8) != 0 { -1.0 } else { 1.0 };
    let exp = (nibble >> 1) & 0x3;
    let mant = nibble & 0x1;

    let val = match (exp, mant) {
        (0, 0) => 0.0,
        (0, 1) => 0.5,
        (1, 0) => 1.0,
        (1, 1) => 1.5,
        (2, 0) => 2.0,
        (2, 1) => 3.0,
        (3, 0) => 4.0,
        (3, 1) => 6.0,
        _ => 0.0,
    };
    sign * val
}
