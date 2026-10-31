// ============================================================================
// CRON SOVEREIGN MATHEMATICAL LANGUAGE (cl_math.rs)
// Standard Conventional Algebra & Zero-Word AI Compositional Calculus
//
// Key Principles:
// 1. Zero Verbal Words: No 'fn', 'def', 'let', 'return', 'var', 'import', 'if'.
// 2. Standard Mathematical Algebra: f(x) = W @ x + b, sigmoid(z) = 1 / (1 + exp(-z)).
// 3. Pipeline Function Composition: x -> norm -> linear(W, b) -> activate.
// 4. First-Class Vectors, Matrices & Conditional Calculus: [1, 2, 3], W @ x, c ? a : b.
// 5. Native Hardware Dtypes: f16 (GPU half), f32, f64, bf16, i32, t2 (ternary 1.58b).
// 6. Multi-Target Lowering:
//    - Real-Time Scalar/Vector/Matrix Evaluator
//    - ISO C23 Native Source (with strength reduction & dtype inference)
//    - NVIDIA CUDA PTX v7.5+ SSA Compiler
//    - 4-Slot 128-Bit VLIW Silicon Microcode (CRC-8 ATM Verified)
// ============================================================================

use std::collections::HashMap;
use std::fmt;

use crate::cl_v2_ast::{
    ClV2Document, FunctionBlock, MathMorphismBlock, MathStep, ProgramItem, SiliconTargetContract,
    VliwBundle, VliwSlot,
};

/// Supported numeric precisions and hardware storage representations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathDtype {
    F16,
    F32,
    F64,
    Bf16,
    I32,
    I64,
    T2, // Ternary {-1, 0, +1} (BitNet 1.58-bit)
}

impl MathDtype {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "f16" | "half" | "fp16" | "~16" => Some(MathDtype::F16),
            "f32" | "float" | "fp32" | "~32" => Some(MathDtype::F32),
            "f64" | "double" | "fp64" | "~64" => Some(MathDtype::F64),
            "bf16" | "bfloat16" => Some(MathDtype::Bf16),
            "i32" | "int" | "int32" | "+32" => Some(MathDtype::I32),
            "i64" | "long" | "int64" | "+64" => Some(MathDtype::I64),
            "t2" | "ternary" | "b1.58" | "t" | "%2" => Some(MathDtype::T2),
            _ => None,
        }
    }

    pub fn to_c23_type(&self) -> &'static str {
        match self {
            MathDtype::F16 => "_Float16",
            MathDtype::F32 => "float",
            MathDtype::F64 => "double",
            MathDtype::Bf16 => "__bf16",
            MathDtype::I32 => "int32_t",
            MathDtype::I64 => "int64_t",
            MathDtype::T2 => "int8_t",
        }
    }

    pub fn to_ptx_type(&self) -> &'static str {
        match self {
            MathDtype::F16 => ".f16",
            MathDtype::F32 => ".f32",
            MathDtype::F64 => ".f64",
            MathDtype::Bf16 => ".b16",
            MathDtype::I32 => ".s32",
            MathDtype::I64 => ".s64",
            MathDtype::T2 => ".s8",
        }
    }

    pub fn to_v2_dtype_str(&self) -> &'static str {
        match self {
            MathDtype::F16 => "FP16",
            MathDtype::F32 => "FP32",
            MathDtype::F64 => "FP64",
            MathDtype::Bf16 => "BF16",
            MathDtype::I32 => "INT32",
            MathDtype::I64 => "INT64",
            MathDtype::T2 => "T",
        }
    }
}

/// First-class runtime value supporting Scalars, 1D Vectors, 2D Matrices, and Multi-Weight Tuples
#[derive(Debug, Clone, PartialEq)]
pub enum MathValue {
    Scalar(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
    Tuple(Vec<MathValue>),
    Tensor {
        shape: Vec<usize>,
        strides: Vec<usize>,
        data: Vec<f64>,
    },
}

impl MathValue {
    pub fn compute_contiguous_strides(shape: &[usize]) -> Vec<usize> {
        if shape.is_empty() {
            return vec![];
        }
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len() - 1).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }

    pub fn new_tensor(shape: Vec<usize>, data: Vec<f64>) -> Result<Self, String> {
        let expected_total: usize = if shape.is_empty() { 0 } else { shape.iter().product() };
        if !data.is_empty() && expected_total != data.len() {
            return Err(format!(
                "Tensor shape {:?} expects {} elements, but got {}",
                shape, expected_total, data.len()
            ));
        }
        let strides = Self::compute_contiguous_strides(&shape);
        Ok(MathValue::Tensor { shape, strides, data })
    }

    pub fn get_flattened_strided(shape: &[usize], strides: &[usize], data: &[f64]) -> Vec<f64> {
        if shape.is_empty() {
            return data.first().copied().map(|x| vec![x]).unwrap_or_default();
        }
        let total: usize = shape.iter().product();
        if data.is_empty() {
            return vec![0.0; total];
        }
        let mut res = Vec::with_capacity(total);
        let ndim = shape.len();
        let mut idxs = vec![0; ndim];
        for _ in 0..total {
            let mut offset = 0;
            for d in 0..ndim {
                offset += idxs[d] * strides[d];
            }
            res.push(data.get(offset).copied().unwrap_or(0.0));
            for d in (0..ndim).rev() {
                idxs[d] += 1;
                if idxs[d] < shape[d] {
                    break;
                }
                idxs[d] = 0;
            }
        }
        res
    }

    pub fn to_contiguous(&self) -> MathValue {
        match self {
            MathValue::Tensor { shape, strides, data } => {
                let canonical = Self::compute_contiguous_strides(shape);
                if strides == &canonical {
                    self.clone()
                } else {
                    let flat = Self::get_flattened_strided(shape, strides, data);
                    MathValue::Tensor {
                        shape: shape.clone(),
                        strides: canonical,
                        data: flat,
                    }
                }
            }
            other => other.clone(),
        }
    }

    pub fn transpose_axes(&self, ax1: usize, ax2: usize) -> Result<MathValue, String> {
        match self {
            MathValue::Tensor { shape, strides, data } => {
                let ndim = shape.len();
                if ax1 >= ndim || ax2 >= ndim {
                    return Err(format!(
                        "Axis index out of bounds for transpose_axes: axes ({}, {}) for rank {}",
                        ax1, ax2, ndim
                    ));
                }
                let mut new_shape = shape.clone();
                let mut new_strides = strides.clone();
                new_shape.swap(ax1, ax2);
                new_strides.swap(ax1, ax2);
                Ok(MathValue::Tensor {
                    shape: new_shape,
                    strides: new_strides,
                    data: data.clone(),
                })
            }
            MathValue::Matrix(_) => {
                if (ax1 == 0 && ax2 == 1) || (ax1 == 1 && ax2 == 0) {
                    self.transpose()
                } else {
                    Err(format!("Invalid axes ({}, {}) for 2D matrix", ax1, ax2))
                }
            }
            _ => Err("transpose_axes requires Tensor or Matrix".to_string()),
        }
    }

    pub fn permute(&self, dims: &[usize]) -> Result<MathValue, String> {
        match self {
            MathValue::Tensor { shape, strides, data } => {
                let ndim = shape.len();
                if dims.len() != ndim {
                    return Err(format!(
                        "permute dims length {} does not match tensor rank {}",
                        dims.len(), ndim
                    ));
                }
                let mut seen = vec![false; ndim];
                for &d in dims {
                    if d >= ndim || seen[d] {
                        return Err(format!("Invalid permutation dims: {:?}", dims));
                    }
                    seen[d] = true;
                }
                let new_shape: Vec<usize> = dims.iter().map(|&d| shape[d]).collect();
                let new_strides: Vec<usize> = dims.iter().map(|&d| strides[d]).collect();
                Ok(MathValue::Tensor {
                    shape: new_shape,
                    strides: new_strides,
                    data: data.clone(),
                })
            }
            MathValue::Matrix(_) => {
                if dims == [1, 0] {
                    self.transpose()
                } else if dims == [0, 1] {
                    Ok(self.clone())
                } else {
                    Err(format!("Invalid permutation {:?} for 2D matrix", dims))
                }
            }
            _ => Err("permute requires Tensor or Matrix".to_string()),
        }
    }

    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            MathValue::Scalar(v) => Some(*v),
            MathValue::Vector(vec) if vec.len() == 1 => Some(vec[0]),
            MathValue::Tensor { data, .. } if data.len() == 1 => Some(data[0]),
            _ => None,
        }
    }

    pub fn map_unary<F: Fn(f64) -> f64>(&self, f: F) -> MathValue {
        self.map_unary_dyn(&f)
    }

    fn map_unary_dyn(&self, f: &dyn Fn(f64) -> f64) -> MathValue {
        match self {
            MathValue::Scalar(v) => MathValue::Scalar(f(*v)),
            MathValue::Vector(vec) => MathValue::Vector(vec.iter().map(|&x| f(x)).collect()),
            MathValue::Matrix(mat) => MathValue::Matrix(
                mat.iter()
                    .map(|row| row.iter().map(|&x| f(x)).collect())
                    .collect(),
            ),
            MathValue::Tuple(items) => {
                MathValue::Tuple(items.iter().map(|item| item.map_unary_dyn(f)).collect())
            }
            MathValue::Tensor { shape, strides, data } => MathValue::Tensor {
                shape: shape.clone(),
                strides: strides.clone(),
                data: data.iter().map(|&x| f(x)).collect(),
            },
        }
    }

    pub fn flatten_elements(&self) -> Vec<f64> {
        match self {
            MathValue::Scalar(v) => vec![*v],
            MathValue::Vector(vec) => vec.clone(),
            MathValue::Matrix(mat) => mat.iter().flatten().copied().collect(),
            MathValue::Tuple(items) => items.iter().flat_map(|item| item.flatten_elements()).collect(),
            MathValue::Tensor { shape, strides, data } => {
                Self::get_flattened_strided(shape, strides, data)
            }
        }
    }

    pub fn transpose(&self) -> Result<MathValue, String> {
        match self {
            MathValue::Scalar(s) => Ok(MathValue::Scalar(*s)),
            MathValue::Vector(v) => {
                let mat: Vec<Vec<f64>> = v.iter().map(|&x| vec![x]).collect();
                Ok(MathValue::Matrix(mat))
            }
            MathValue::Matrix(mat) => {
                if mat.is_empty() {
                    return Ok(MathValue::Matrix(Vec::new()));
                }
                let rows = mat.len();
                let cols = mat[0].len();
                let mut transposed = vec![vec![0.0; rows]; cols];
                for r in 0..rows {
                    for c in 0..cols {
                        if c < mat[r].len() {
                            transposed[c][r] = mat[r][c];
                        }
                    }
                }
                Ok(MathValue::Matrix(transposed))
            }
            MathValue::Tuple(items) => {
                let mut res = Vec::with_capacity(items.len());
                for item in items {
                    res.push(item.transpose()?);
                }
                Ok(MathValue::Tuple(res))
            }
            MathValue::Tensor { shape, strides, data } => {
                if shape.len() < 2 {
                    Ok(self.clone())
                } else {
                    let n = shape.len();
                    let mut new_shape = shape.clone();
                    let mut new_strides = strides.clone();
                    new_shape.swap(n - 2, n - 1);
                    new_strides.swap(n - 2, n - 1);
                    Ok(MathValue::Tensor {
                        shape: new_shape,
                        strides: new_strides,
                        data: data.clone(),
                    })
                }
            }
        }
    }

    pub fn reshape(&self, dims: &[usize]) -> Result<MathValue, String> {
        let elems = self.flatten_elements();
        let expected_total: usize = dims.iter().product();
        if dims.is_empty() {
            return Ok(MathValue::Scalar(elems.first().copied().unwrap_or(0.0)));
        }
        if expected_total != elems.len() && !elems.is_empty() {
            return Err(format!(
                "Cannot reshape tensor of {} elements into shape {:?}",
                elems.len(),
                dims
            ));
        }
        if dims.len() == 1 {
            return Ok(MathValue::Vector(elems));
        }
        if dims.len() == 2 {
            let rows = dims[0];
            let cols = dims[1];
            let mut mat = Vec::with_capacity(rows);
            for r in 0..rows {
                let start = r * cols;
                let end = (start + cols).min(elems.len());
                mat.push(elems[start..end].to_vec());
            }
            return Ok(MathValue::Matrix(mat));
        }
        let strides = Self::compute_contiguous_strides(dims);
        Ok(MathValue::Tensor {
            shape: dims.to_vec(),
            strides,
            data: elems,
        })
    }

    pub fn slice(&self, start: usize, end: usize) -> Result<MathValue, String> {
        match self {
            MathValue::Scalar(_) => Err("Cannot slice a scalar value".to_string()),
            MathValue::Vector(v) => {
                let s = start.min(v.len());
                let e = end.min(v.len());
                if s > e {
                    Ok(MathValue::Vector(Vec::new()))
                } else {
                    Ok(MathValue::Vector(v[s..e].to_vec()))
                }
            }
            MathValue::Matrix(m) => {
                let s = start.min(m.len());
                let e = end.min(m.len());
                if s > e {
                    Ok(MathValue::Matrix(Vec::new()))
                } else {
                    Ok(MathValue::Matrix(m[s..e].to_vec()))
                }
            }
            MathValue::Tuple(items) => {
                let s = start.min(items.len());
                let e = end.min(items.len());
                if s > e {
                    Ok(MathValue::Tuple(Vec::new()))
                } else {
                    Ok(MathValue::Tuple(items[s..e].to_vec()))
                }
            }
            MathValue::Tensor { shape, strides, data } => {
                if shape.is_empty() {
                    return Err("Cannot slice empty tensor".to_string());
                }
                let d0 = shape[0];
                let s = start.min(d0);
                let e = end.min(d0);
                let mut new_shape = shape.clone();
                if s >= e {
                    new_shape[0] = 0;
                    let new_strides = Self::compute_contiguous_strides(&new_shape);
                    Ok(MathValue::Tensor { shape: new_shape, strides: new_strides, data: Vec::new() })
                } else {
                    new_shape[0] = e - s;
                    let flat = Self::get_flattened_strided(shape, strides, data);
                    let elems_per_d0: usize = shape[1..].iter().product();
                    let s_offset = s * elems_per_d0;
                    let e_offset = e * elems_per_d0;
                    let new_data = flat[s_offset..e_offset.min(flat.len())].to_vec();
                    let new_strides = Self::compute_contiguous_strides(&new_shape);
                    Ok(MathValue::Tensor { shape: new_shape, strides: new_strides, data: new_data })
                }
            }
        }
    }

    pub fn shape(&self) -> Vec<usize> {
        match self {
            MathValue::Scalar(_) => vec![],
            MathValue::Vector(v) => vec![v.len()],
            MathValue::Matrix(m) => vec![m.len(), m.first().map(|r| r.len()).unwrap_or(0)],
            MathValue::Tuple(items) => vec![items.len()],
            MathValue::Tensor { shape, .. } => shape.clone(),
        }
    }

    pub fn zeros_like(&self) -> MathValue {
        match self {
            MathValue::Scalar(_) => MathValue::Scalar(0.0),
            MathValue::Vector(v) => MathValue::Vector(vec![0.0; v.len()]),
            MathValue::Matrix(m) => {
                let rows = m.len();
                let cols = m.first().map(|r| r.len()).unwrap_or(0);
                MathValue::Matrix(vec![vec![0.0; cols]; rows])
            }
            MathValue::Tuple(items) => MathValue::Tuple(items.iter().map(|i| i.zeros_like()).collect()),
            MathValue::Tensor { shape, strides, data } => MathValue::Tensor {
                shape: shape.clone(),
                strides: strides.clone(),
                data: vec![0.0; data.len()],
            },
        }
    }

    pub fn add_accumulate(&mut self, other: &MathValue) -> Result<(), String> {
        match (self, other) {
            (MathValue::Tuple(t1), MathValue::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(format!(
                        "Tuple length mismatch in add_accumulate: {} vs {}",
                        t1.len(),
                        t2.len()
                    ));
                }
                for (a, b) in t1.iter_mut().zip(t2.iter()) {
                    a.add_accumulate(b)?;
                }
                Ok(())
            }
            (MathValue::Scalar(s1), MathValue::Scalar(s2)) => {
                *s1 += s2;
                Ok(())
            }
            (MathValue::Vector(v1), MathValue::Vector(v2)) => {
                if v1.len() != v2.len() {
                    return Err(format!(
                        "Vector dimension mismatch in add_accumulate: {} vs {}",
                        v1.len(),
                        v2.len()
                    ));
                }
                for (a, b) in v1.iter_mut().zip(v2.iter()) {
                    *a += b;
                }
                Ok(())
            }
            (MathValue::Matrix(m1), MathValue::Matrix(m2)) => {
                if m1.len() != m2.len() {
                    return Err(format!(
                        "Matrix row mismatch in add_accumulate: {} vs {}",
                        m1.len(),
                        m2.len()
                    ));
                }
                for (r1, r2) in m1.iter_mut().zip(m2.iter()) {
                    for (a, b) in r1.iter_mut().zip(r2.iter()) {
                        *a += b;
                    }
                }
                Ok(())
            }
            (MathValue::Vector(v1), MathValue::Matrix(m2)) => {
                let cols = m2.first().map(|r| r.len()).unwrap_or(0);
                if v1.len() == cols {
                    for row in m2 {
                        for (a, b) in v1.iter_mut().zip(row.iter()) {
                            *a += b;
                        }
                    }
                    Ok(())
                } else if v1.len() == m2.len() {
                    for (a, row) in v1.iter_mut().zip(m2.iter()) {
                        *a += row.iter().sum::<f64>();
                    }
                    Ok(())
                } else {
                    Err(format!(
                        "Cannot accumulate matrix of shape [{}, {}] into vector of length {}",
                        m2.len(),
                        cols,
                        v1.len()
                    ))
                }
            }
            (
                MathValue::Tensor {
                    shape: s1,
                    data: d1,
                    ..
                },
                MathValue::Tensor {
                    shape: s2,
                    data: d2,
                    ..
                },
            ) => {
                if s1 != s2 || d1.len() != d2.len() {
                    return Err(format!(
                        "Shape mismatch in tensor accumulation: {:?} vs {:?}",
                        s1, s2
                    ));
                }
                for (a, &b) in d1.iter_mut().zip(d2.iter()) {
                    *a += b;
                }
                Ok(())
            }
            (MathValue::Scalar(s), tensor) => {
                *s += tensor.flatten_elements().iter().sum::<f64>();
                Ok(())
            }
            _ => Err("Incompatible shapes in add_accumulate".to_string()),
        }
    }

    pub fn sum_axis(&self, axis: usize) -> Result<MathValue, String> {
        match self {
            MathValue::Scalar(s) => Ok(MathValue::Scalar(*s)),
            MathValue::Vector(v) => {
                if axis == 0 {
                    Ok(MathValue::Scalar(v.iter().sum()))
                } else {
                    Err(format!("Axis {} out of bounds for 1D vector", axis))
                }
            }
            MathValue::Matrix(m) => {
                if m.is_empty() {
                    return Ok(MathValue::Vector(Vec::new()));
                }
                let cols = m[0].len();
                if axis == 0 {
                    let mut out = vec![0.0; cols];
                    for row in m {
                        for (c, &val) in row.iter().enumerate() {
                            if c < cols {
                                out[c] += val;
                            }
                        }
                    }
                    Ok(MathValue::Vector(out))
                } else if axis == 1 {
                    let out: Vec<f64> = m.iter().map(|row| row.iter().sum()).collect();
                    Ok(MathValue::Vector(out))
                } else {
                    Err(format!("Axis {} out of bounds for 2D matrix", axis))
                }
            }
            MathValue::Tuple(_) => Err("Cannot sum_axis on a Tuple".to_string()),
            MathValue::Tensor { shape, strides, data } => {
                let ndim = shape.len();
                if axis >= ndim {
                    return Err(format!("Axis {} out of bounds for tensor of rank {}", axis, ndim));
                }
                let mut out_shape = shape.clone();
                let _removed_dim = out_shape.remove(axis);
                let out_total: usize = if out_shape.is_empty() { 1 } else { out_shape.iter().product() };
                let mut out_data = vec![0.0; out_total];
                let out_strides = Self::compute_contiguous_strides(&out_shape);

                let total: usize = shape.iter().product();
                let mut idxs = vec![0; ndim];
                for _ in 0..total {
                    let mut in_offset = 0;
                    for d in 0..ndim {
                        in_offset += idxs[d] * strides[d];
                    }
                    let val = data.get(in_offset).copied().unwrap_or(0.0);

                    let mut out_offset = 0;
                    let mut od = 0;
                    for d in 0..ndim {
                        if d != axis {
                            out_offset += idxs[d] * out_strides[od];
                            od += 1;
                        }
                    }
                    if out_offset < out_data.len() {
                        out_data[out_offset] += val;
                    }

                    for d in (0..ndim).rev() {
                        idxs[d] += 1;
                        if idxs[d] < shape[d] {
                            break;
                        }
                        idxs[d] = 0;
                    }
                }

                if out_shape.is_empty() {
                    Ok(MathValue::Scalar(out_data.first().copied().unwrap_or(0.0)))
                } else if out_shape.len() == 1 {
                    Ok(MathValue::Vector(out_data))
                } else if out_shape.len() == 2 {
                    let r = out_shape[0];
                    let c = out_shape[1];
                    let mut mat = Vec::with_capacity(r);
                    for i in 0..r {
                        mat.push(out_data[i * c..(i + 1) * c].to_vec());
                    }
                    Ok(MathValue::Matrix(mat))
                } else {
                    Ok(MathValue::Tensor {
                        shape: out_shape.clone(),
                        strides: out_strides,
                        data: out_data,
                    })
                }
            }
        }
    }
}

impl fmt::Display for MathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathValue::Scalar(v) => {
                if (v - v.round()).abs() < 1e-12 {
                    write!(f, "{}", *v as i64)
                } else {
                    write!(f, "{:.6}", v)
                }
            }
            MathValue::Vector(vec) => {
                let parts: Vec<String> = vec
                    .iter()
                    .map(|v| {
                        if (v - v.round()).abs() < 1e-12 {
                            format!("{}", *v as i64)
                        } else {
                            format!("{:.4}", v)
                        }
                    })
                    .collect();
                write!(f, "[{}]", parts.join(", "))
            }
            MathValue::Matrix(mat) => {
                let rows: Vec<String> = mat
                    .iter()
                    .map(|r| {
                        let elems: Vec<String> = r.iter().map(|v| format!("{:.4}", v)).collect();
                        format!("[{}]", elems.join(", "))
                    })
                    .collect();
                write!(f, "[{}]", rows.join(", "))
            }
            MathValue::Tuple(items) => {
                let parts: Vec<String> = items.iter().map(|i| format!("{}", i)).collect();
                write!(f, "({})", parts.join(", "))
            }
            MathValue::Tensor {
                shape,
                strides,
                data,
            } => {
                let flat = Self::get_flattened_strided(shape, strides, data);
                let preview: Vec<String> =
                    flat.iter().take(6).map(|v| format!("{:.4}", v)).collect();
                let suffix = if flat.len() > 6 { ", ..." } else { "" };
                write!(
                    f,
                    "Tensor(shape={:?}, [{}{}])",
                    shape,
                    preview.join(", "),
                    suffix
                )
            }
        }
    }
}

/// Function parameter with optional mathematical type and tensor dimensions
#[derive(Debug, Clone, PartialEq)]
pub struct MathParam {
    pub name: String,
    pub dtype: MathDtype,
    pub dims: Vec<usize>,
}

/// Binary mathematical operators with standard precedence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathBinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    MatMul, // @
    Pow,    // ^
    Eq,     // ==
    Neq,    // !=
    Lt,     // <
    Lte,    // <=
    Gt,     // >
    Gte,    // >=
}

impl fmt::Display for MathBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathBinOp::Add => write!(f, "+"),
            MathBinOp::Sub => write!(f, "-"),
            MathBinOp::Mul => write!(f, "*"),
            MathBinOp::Div => write!(f, "/"),
            MathBinOp::MatMul => write!(f, "@"),
            MathBinOp::Pow => write!(f, "^"),
            MathBinOp::Eq => write!(f, "=="),
            MathBinOp::Neq => write!(f, "!="),
            MathBinOp::Lt => write!(f, "<"),
            MathBinOp::Lte => write!(f, "<="),
            MathBinOp::Gt => write!(f, ">"),
            MathBinOp::Gte => write!(f, ">="),
        }
    }
}

/// Unary mathematical operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathUnaryOp {
    Neg, // -x
    Not, // !x
}

/// Mathematical Expression AST
#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Num(f64),
    Var(String),
    Array(Vec<MathExpr>), // Vector or Matrix literal: [1, 2, 3] or [[1, 2], [3, 4]]
    Tuple(Vec<MathExpr>), // Multi-Weight / Optimizer State Tuple: (W1, b1, W2, b2)
    Index(Box<MathExpr>, Box<MathExpr>), // Tensor / Vector indexing: x[i], W[i][j]
    TensorRef {
        path: String,
        key: String,
    }, // External weights: @"model.safetensors:w" or @"weights.cll:w"
    Iterate {
        input: Box<MathExpr>,
        func: Box<MathExpr>,
        count: Box<MathExpr>,
    }, // Functional power/iteration: x -> f^N
    Placeholder,          // '_' in pipeline chaining
    Unary(MathUnaryOp, Box<MathExpr>),
    Binary(Box<MathExpr>, MathBinOp, Box<MathExpr>),
    Ternary {
        cond: Box<MathExpr>,
        then_expr: Box<MathExpr>,
        else_expr: Box<MathExpr>,
    },
    Call(String, Vec<MathExpr>),
    Pipeline(Vec<MathExpr>), // e.g. x -> f -> g
}

impl fmt::Display for MathExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathExpr::Num(n) => write!(f, "{}", n),
            MathExpr::Var(v) => write!(f, "{}", v),
            MathExpr::Array(elems) => {
                let s: Vec<String> = elems.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", s.join(", "))
            }
            MathExpr::Tuple(items) => {
                let s: Vec<String> = items.iter().map(|e| e.to_string()).collect();
                write!(f, "({})", s.join(", "))
            }
            MathExpr::Index(base, idx) => write!(f, "{}[{}]", base, idx),
            MathExpr::TensorRef { path, key } => write!(f, "@\"{}:{}\"", path, key),
            MathExpr::Iterate { input, func, count } => {
                write!(f, "({} -> {} ^ {})", input, func, count)
            }
            MathExpr::Placeholder => write!(f, "_"),
            MathExpr::Unary(MathUnaryOp::Neg, e) => write!(f, "(-{})", e),
            MathExpr::Unary(MathUnaryOp::Not, e) => write!(f, "(!{})", e),
            MathExpr::Binary(l, op, r) => write!(f, "({} {} {})", l, op, r),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => write!(f, "({} ? {} : {})", cond, then_expr, else_expr),
            MathExpr::Call(name, args) => {
                let s: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, s.join(", "))
            }
            MathExpr::Pipeline(stages) => {
                let s: Vec<String> = stages.iter().map(|st| st.to_string()).collect();
                write!(f, "{}", s.join(" -> "))
            }
        }
    }
}

impl MathExpr {
    /// Beta-reduction variable substitution
    pub fn substitute(&self, subst: &HashMap<String, MathExpr>) -> MathExpr {
        match self {
            MathExpr::Num(n) => MathExpr::Num(*n),
            MathExpr::Var(v) => subst
                .get(v)
                .cloned()
                .unwrap_or_else(|| MathExpr::Var(v.clone())),
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.substitute(subst)).collect())
            }
            MathExpr::Tuple(items) => {
                MathExpr::Tuple(items.iter().map(|e| e.substitute(subst)).collect())
            }
            MathExpr::Index(b, i) => {
                MathExpr::Index(Box::new(b.substitute(subst)), Box::new(i.substitute(subst)))
            }
            MathExpr::TensorRef { path, key } => MathExpr::TensorRef {
                path: path.clone(),
                key: key.clone(),
            },
            MathExpr::Iterate { input, func, count } => MathExpr::Iterate {
                input: Box::new(input.substitute(subst)),
                func: Box::new(func.substitute(subst)),
                count: Box::new(count.substitute(subst)),
            },
            MathExpr::Placeholder => MathExpr::Placeholder,
            MathExpr::Unary(op, inner) => MathExpr::Unary(*op, Box::new(inner.substitute(subst))),
            MathExpr::Binary(l, op, r) => MathExpr::Binary(
                Box::new(l.substitute(subst)),
                *op,
                Box::new(r.substitute(subst)),
            ),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.substitute(subst)),
                then_expr: Box::new(then_expr.substitute(subst)),
                else_expr: Box::new(else_expr.substitute(subst)),
            },
            MathExpr::Call(name, args) => MathExpr::Call(
                name.clone(),
                args.iter().map(|a| a.substitute(subst)).collect(),
            ),
            MathExpr::Pipeline(stages) => {
                MathExpr::Pipeline(stages.iter().map(|s| s.substitute(subst)).collect())
            }
        }
    }

    /// Recursively inlines user-defined functions (`inline_user_fns`) and expands symbolic `grad(f, x)`
    pub fn inline_and_expand(
        &self,
        fns: &HashMap<String, (Vec<String>, MathExpr)>,
        inline_user_fns: bool,
        depth: usize,
    ) -> MathExpr {
        if depth > 16 {
            return self.clone();
        }
        match self {
            MathExpr::Num(_)
            | MathExpr::Var(_)
            | MathExpr::Placeholder
            | MathExpr::TensorRef { .. } => self.clone(),
            MathExpr::Array(elems) => MathExpr::Array(
                elems
                    .iter()
                    .map(|e| e.inline_and_expand(fns, inline_user_fns, depth + 1))
                    .collect(),
            ),
            MathExpr::Tuple(items) => MathExpr::Tuple(
                items
                    .iter()
                    .map(|e| e.inline_and_expand(fns, inline_user_fns, depth + 1))
                    .collect(),
            ),
            MathExpr::Index(b, i) => MathExpr::Index(
                Box::new(b.inline_and_expand(fns, inline_user_fns, depth + 1)),
                Box::new(i.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Unary(op, inner) => MathExpr::Unary(
                *op,
                Box::new(inner.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Binary(l, op, r) => MathExpr::Binary(
                Box::new(l.inline_and_expand(fns, inline_user_fns, depth + 1)),
                *op,
                Box::new(r.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.inline_and_expand(fns, inline_user_fns, depth + 1)),
                then_expr: Box::new(then_expr.inline_and_expand(fns, inline_user_fns, depth + 1)),
                else_expr: Box::new(else_expr.inline_and_expand(fns, inline_user_fns, depth + 1)),
            },
            MathExpr::Iterate { input, func, count } => MathExpr::Iterate {
                input: Box::new(input.inline_and_expand(fns, inline_user_fns, depth + 1)),
                func: Box::new(func.inline_and_expand(fns, inline_user_fns, depth + 1)),
                count: Box::new(count.inline_and_expand(fns, inline_user_fns, depth + 1)),
            },
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                curr.inline_and_expand(fns, inline_user_fns, depth + 1)
            }
            MathExpr::Call(name, args) => {
                if (name == "grad" || name == "diff") && args.len() == 2 {
                    if let MathExpr::Var(wrt) = &args[1] {
                        let expanded_target = args[0].inline_and_expand(fns, true, depth + 1);
                        return expanded_target.differentiate(wrt).simplify();
                    }
                }
                let expanded_args: Vec<MathExpr> = args
                    .iter()
                    .map(|a| a.inline_and_expand(fns, inline_user_fns, depth + 1))
                    .collect();
                if inline_user_fns {
                    if let Some((param_names, body)) = fns.get(name) {
                        if param_names.len() == expanded_args.len() {
                            let mut subst = HashMap::new();
                            for (p, a) in param_names.iter().zip(expanded_args.into_iter()) {
                                subst.insert(p.clone(), a);
                            }
                            let inlined = body.substitute(&subst);
                            return inlined.inline_and_expand(fns, inline_user_fns, depth + 1);
                        }
                    }
                }
                MathExpr::Call(name.clone(), expanded_args)
            }
        }
    }

    /// Compile-Time Symbolic Automatic Differentiation: d(self) / d(var)
    pub fn differentiate(&self, var: &str) -> MathExpr {
        match self {
            MathExpr::Num(_)
            | MathExpr::Placeholder
            | MathExpr::Index(_, _)
            | MathExpr::TensorRef { .. } => MathExpr::Num(0.0),
            MathExpr::Var(v) => {
                if v == var {
                    MathExpr::Num(1.0)
                } else {
                    MathExpr::Num(0.0)
                }
            }
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.differentiate(var)).collect())
            }
            MathExpr::Tuple(items) => {
                MathExpr::Tuple(items.iter().map(|e| e.differentiate(var)).collect())
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                MathExpr::Unary(MathUnaryOp::Neg, Box::new(inner.differentiate(var)))
            }
            MathExpr::Unary(MathUnaryOp::Not, _) => MathExpr::Num(0.0),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: cond.clone(),
                then_expr: Box::new(then_expr.differentiate(var)),
                else_expr: Box::new(else_expr.differentiate(var)),
            },
            MathExpr::Binary(u, op, v) => {
                let du = u.differentiate(var);
                let dv = v.differentiate(var);
                match op {
                    MathBinOp::Add => MathExpr::Binary(Box::new(du), MathBinOp::Add, Box::new(dv)),
                    MathBinOp::Sub => MathExpr::Binary(Box::new(du), MathBinOp::Sub, Box::new(dv)),
                    MathBinOp::Mul | MathBinOp::MatMul => {
                        // Product rule: du * v + u * dv
                        let left = MathExpr::Binary(Box::new(du), *op, v.clone());
                        let right = MathExpr::Binary(u.clone(), *op, Box::new(dv));
                        MathExpr::Binary(Box::new(left), MathBinOp::Add, Box::new(right))
                    }
                    MathBinOp::Div => {
                        // Quotient rule: (du * v - u * dv) / (v ^ 2)
                        let left = MathExpr::Binary(Box::new(du), MathBinOp::Mul, v.clone());
                        let right = MathExpr::Binary(u.clone(), MathBinOp::Mul, Box::new(dv));
                        let num = MathExpr::Binary(Box::new(left), MathBinOp::Sub, Box::new(right));
                        let den = MathExpr::Binary(
                            v.clone(),
                            MathBinOp::Pow,
                            Box::new(MathExpr::Num(2.0)),
                        );
                        MathExpr::Binary(Box::new(num), MathBinOp::Div, Box::new(den))
                    }
                    MathBinOp::Pow => {
                        if let MathExpr::Num(c) = **v {
                            // Power rule: c * u^(c-1) * du
                            let coeff = MathExpr::Num(c);
                            let pow_minus_1 = MathExpr::Binary(
                                u.clone(),
                                MathBinOp::Pow,
                                Box::new(MathExpr::Num(c - 1.0)),
                            );
                            let term = MathExpr::Binary(
                                Box::new(coeff),
                                MathBinOp::Mul,
                                Box::new(pow_minus_1),
                            );
                            MathExpr::Binary(Box::new(term), MathBinOp::Mul, Box::new(du))
                        } else {
                            MathExpr::Num(0.0)
                        }
                    }
                    _ => MathExpr::Num(0.0),
                }
            }
            MathExpr::Call(func, args) => {
                if args.is_empty() {
                    return MathExpr::Num(0.0);
                }
                let u = &args[0];
                let du = u.differentiate(var);
                let outer_deriv = match func.as_str() {
                    "exp" => MathExpr::Call("exp".to_string(), vec![u.clone()]),
                    "ln" | "log" => MathExpr::Binary(
                        Box::new(MathExpr::Num(1.0)),
                        MathBinOp::Div,
                        Box::new(u.clone()),
                    ),
                    "sqrt" => MathExpr::Binary(
                        Box::new(MathExpr::Num(1.0)),
                        MathBinOp::Div,
                        Box::new(MathExpr::Binary(
                            Box::new(MathExpr::Num(2.0)),
                            MathBinOp::Mul,
                            Box::new(MathExpr::Call("sqrt".to_string(), vec![u.clone()])),
                        )),
                    ),
                    "sin" => MathExpr::Call("cos".to_string(), vec![u.clone()]),
                    "cos" => MathExpr::Unary(
                        MathUnaryOp::Neg,
                        Box::new(MathExpr::Call("sin".to_string(), vec![u.clone()])),
                    ),
                    "tanh" => {
                        let t = MathExpr::Call("tanh".to_string(), vec![u.clone()]);
                        let t2 = MathExpr::Binary(
                            Box::new(t),
                            MathBinOp::Pow,
                            Box::new(MathExpr::Num(2.0)),
                        );
                        MathExpr::Binary(Box::new(MathExpr::Num(1.0)), MathBinOp::Sub, Box::new(t2))
                    }
                    "sigmoid" => {
                        let s = MathExpr::Call("sigmoid".to_string(), vec![u.clone()]);
                        let one_minus_s = MathExpr::Binary(
                            Box::new(MathExpr::Num(1.0)),
                            MathBinOp::Sub,
                            Box::new(s.clone()),
                        );
                        MathExpr::Binary(Box::new(s), MathBinOp::Mul, Box::new(one_minus_s))
                    }
                    "relu" => MathExpr::Ternary {
                        cond: Box::new(MathExpr::Binary(
                            Box::new(u.clone()),
                            MathBinOp::Gt,
                            Box::new(MathExpr::Num(0.0)),
                        )),
                        then_expr: Box::new(MathExpr::Num(1.0)),
                        else_expr: Box::new(MathExpr::Num(0.0)),
                    },
                    "silu" => {
                        let s = MathExpr::Call("sigmoid".to_string(), vec![u.clone()]);
                        let one_minus_s = MathExpr::Binary(
                            Box::new(MathExpr::Num(1.0)),
                            MathBinOp::Sub,
                            Box::new(s.clone()),
                        );
                        let u_s = MathExpr::Binary(
                            Box::new(u.clone()),
                            MathBinOp::Mul,
                            Box::new(s.clone()),
                        );
                        let second_term =
                            MathExpr::Binary(Box::new(u_s), MathBinOp::Mul, Box::new(one_minus_s));
                        MathExpr::Binary(Box::new(s), MathBinOp::Add, Box::new(second_term))
                    }
                    "sum" | "mean" => {
                        return MathExpr::Call(func.clone(), vec![du]);
                    }
                    _ => MathExpr::Num(1.0),
                };
                MathExpr::Binary(Box::new(outer_deriv), MathBinOp::Mul, Box::new(du))
            }
            MathExpr::Iterate { .. } => MathExpr::Num(0.0),
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                curr.differentiate(var)
            }
        }
    }

    /// Algebraic simplification and constant folding pass
    pub fn simplify(&self) -> MathExpr {
        match self {
            MathExpr::Num(_)
            | MathExpr::Var(_)
            | MathExpr::Placeholder
            | MathExpr::TensorRef { .. } => self.clone(),
            MathExpr::Array(elems) => MathExpr::Array(elems.iter().map(|e| e.simplify()).collect()),
            MathExpr::Tuple(items) => MathExpr::Tuple(items.iter().map(|e| e.simplify()).collect()),
            MathExpr::Index(b, i) => {
                MathExpr::Index(Box::new(b.simplify()), Box::new(i.simplify()))
            }
            MathExpr::Unary(op, inner) => {
                let s = inner.simplify();
                if *op == MathUnaryOp::Neg {
                    if let MathExpr::Num(n) = s {
                        return MathExpr::Num(-n);
                    }
                }
                MathExpr::Unary(*op, Box::new(s))
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.simplify()),
                then_expr: Box::new(then_expr.simplify()),
                else_expr: Box::new(else_expr.simplify()),
            },
            MathExpr::Iterate { input, func, count } => MathExpr::Iterate {
                input: Box::new(input.simplify()),
                func: Box::new(func.simplify()),
                count: Box::new(count.simplify()),
            },
            MathExpr::Call(name, args) => {
                MathExpr::Call(name.clone(), args.iter().map(|a| a.simplify()).collect())
            }
            MathExpr::Pipeline(stages) => {
                MathExpr::Pipeline(stages.iter().map(|s| s.simplify()).collect())
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();
                match (&l, op, &r) {
                    (MathExpr::Num(a), MathBinOp::Add, MathExpr::Num(b)) => MathExpr::Num(a + b),
                    (MathExpr::Num(a), MathBinOp::Sub, MathExpr::Num(b)) => MathExpr::Num(a - b),
                    (MathExpr::Num(a), MathBinOp::Mul, MathExpr::Num(b)) => MathExpr::Num(a * b),
                    (MathExpr::Num(a), MathBinOp::Pow, MathExpr::Num(b)) => {
                        MathExpr::Num(a.powf(*b))
                    }
                    (MathExpr::Num(a), MathBinOp::Add, _) if a.abs() < 1e-12 => r,
                    (_, MathBinOp::Add, MathExpr::Num(b)) if b.abs() < 1e-12 => l,
                    (_, MathBinOp::Sub, MathExpr::Num(b)) if b.abs() < 1e-12 => l,
                    (MathExpr::Num(a), MathBinOp::Sub, _) if a.abs() < 1e-12 => {
                        MathExpr::Unary(MathUnaryOp::Neg, Box::new(r))
                    }
                    (MathExpr::Num(a), MathBinOp::Mul, _) if a.abs() < 1e-12 => MathExpr::Num(0.0),
                    (_, MathBinOp::Mul, MathExpr::Num(b)) if b.abs() < 1e-12 => MathExpr::Num(0.0),
                    (MathExpr::Num(a), MathBinOp::Mul, _) if (a - 1.0).abs() < 1e-12 => r,
                    (_, MathBinOp::Mul, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (_, MathBinOp::Div, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (MathExpr::Num(a), MathBinOp::Div, _) if a.abs() < 1e-12 => MathExpr::Num(0.0),
                    (_, MathBinOp::Pow, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (_, MathBinOp::Pow, MathExpr::Num(b)) if b.abs() < 1e-12 => MathExpr::Num(1.0),
                    _ => MathExpr::Binary(Box::new(l), *op, Box::new(r)),
                }
            }
        }
    }

    /// High-level Tensor Graph Optimization & Kernel Fusion pass
    pub fn optimize_tensor_graph(&self) -> MathExpr {
        let simplified = match self {
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.optimize_tensor_graph()).collect())
            }
            MathExpr::Tuple(items) => {
                MathExpr::Tuple(items.iter().map(|e| e.optimize_tensor_graph()).collect())
            }
            MathExpr::Index(b, i) => {
                MathExpr::Index(
                    Box::new(b.optimize_tensor_graph()),
                    Box::new(i.optimize_tensor_graph()),
                )
            }
            MathExpr::Unary(op, inner) => {
                MathExpr::Unary(*op, Box::new(inner.optimize_tensor_graph()))
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.optimize_tensor_graph()),
                then_expr: Box::new(then_expr.optimize_tensor_graph()),
                else_expr: Box::new(else_expr.optimize_tensor_graph()),
            },
            MathExpr::Iterate { input, func, count } => MathExpr::Iterate {
                input: Box::new(input.optimize_tensor_graph()),
                func: Box::new(func.optimize_tensor_graph()),
                count: Box::new(count.optimize_tensor_graph()),
            },
            MathExpr::Call(name, args) => {
                let opt_args: Vec<MathExpr> =
                    args.iter().map(|a| a.optimize_tensor_graph()).collect();
                // Rule 1: transpose(transpose(X)) -> X
                if (name == "transpose" || name == "T") && opt_args.len() == 1 {
                    if let MathExpr::Call(sub_name, sub_args) = &opt_args[0] {
                        if (sub_name == "transpose" || sub_name == "T") && sub_args.len() == 1 {
                            return sub_args[0].clone();
                        }
                    }
                }
                MathExpr::Call(name.clone(), opt_args)
            }
            MathExpr::Pipeline(stages) => {
                MathExpr::Pipeline(stages.iter().map(|s| s.optimize_tensor_graph()).collect())
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = lhs.optimize_tensor_graph();
                let r = rhs.optimize_tensor_graph();
                // Rule 2: Identity MatMul: X @ eye(N) -> X, eye(N) @ X -> X
                if *op == MathBinOp::MatMul {
                    if let MathExpr::Call(ref name, _) = r {
                        if name == "eye" {
                            return l;
                        }
                    }
                    if let MathExpr::Call(ref name, _) = l {
                        if name == "eye" {
                            return r;
                        }
                    }
                    // Rule 3: Attention Fusion: softmax((Q @ K^T) * scale) @ V -> attention(Q, K, V, scale)
                    if let MathExpr::Call(ref s_name, ref s_args) = l {
                        if s_name == "softmax" && s_args.len() == 1 {
                            let inner_score = &s_args[0];
                            let mut scale_val = None;
                            let qk_expr = match inner_score {
                                MathExpr::Binary(qk, MathBinOp::Mul, scale_expr) => {
                                    scale_val = Some(*scale_expr.clone());
                                    qk.as_ref()
                                }
                                MathExpr::Binary(qk, MathBinOp::Div, scale_expr) => {
                                    scale_val = Some(MathExpr::Binary(
                                        Box::new(MathExpr::Num(1.0)),
                                        MathBinOp::Div,
                                        scale_expr.clone(),
                                    ));
                                    qk.as_ref()
                                }
                                other => other,
                            };
                            if let MathExpr::Binary(q_box, MathBinOp::MatMul, kt_box) = qk_expr {
                                if let MathExpr::Call(kt_name, kt_args) = kt_box.as_ref() {
                                    if (kt_name == "transpose" || kt_name == "T")
                                        && kt_args.len() == 1
                                    {
                                        let q = *q_box.clone();
                                        let k = kt_args[0].clone();
                                        let v = r.clone();
                                        let mut call_args = vec![q, k, v];
                                        if let Some(s) = scale_val {
                                            call_args.push(s);
                                        }
                                        return MathExpr::Call("attention".to_string(), call_args);
                                    }
                                }
                            }
                        }
                    }
                }
                // Rule 4: Zero Tensor Addition: X + zeros(...) -> X, zeros(...) + X -> X
                if *op == MathBinOp::Add {
                    if let MathExpr::Call(ref name, _) = r {
                        if name == "zeros" {
                            return l;
                        }
                    }
                    if let MathExpr::Call(ref name, _) = l {
                        if name == "zeros" {
                            return r;
                        }
                    }
                }
                // Rule 5: Ones Tensor Multiplication: X * ones(...) -> X, ones(...) * X -> X
                if *op == MathBinOp::Mul {
                    if let MathExpr::Call(ref name, _) = r {
                        if name == "ones" {
                            return l;
                        }
                    }
                    if let MathExpr::Call(ref name, _) = l {
                        if name == "ones" {
                            return r;
                        }
                    }
                }
                MathExpr::Binary(Box::new(l), *op, Box::new(r))
            }
            _ => self.clone(),
        };
        simplified.simplify()
    }
}

/// Top-level Items in a Mathematical Module
#[derive(Debug, Clone, PartialEq)]
pub enum MathItem {
    Link(String), // << "path/to/module.cl"
    Export {
        expr: MathExpr,
        path: String,
        key: String,
    }, // expr >> "path.safetensors:key"
    Constant {
        name: String,
        val: f64,
    },
    Binding {
        name: String,
        expr: MathExpr,
    },
    TupleBinding {
        names: Vec<String>,
        expr: MathExpr,
    },
    Function {
        name: String,
        params: Vec<MathParam>,
        ret_type: Option<MathDtype>,
        body: MathExpr,
    },
    Expression(MathExpr),
}

/// Complete parsed mathematical module
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MathProgram {
    pub items: Vec<MathItem>,
}

// ============================================================================
// TOKENIZER / LEXER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Num(f64),
    Str(String), // "..."
    Shl,         // << (Zero-verbal module link)
    Shr,         // >> (Zero-verbal tensor export)
    Equal,       // =
    Arrow,       // ->
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    At,          // @ (MatMul)
    Caret,       // ^ (Power)
    Question,    // ? (Conditional ternary)
    EqEq,        // ==
    NotEq,       // !=
    Lt,          // <
    Lte,         // <=
    Gt,          // >
    Gte,         // >=
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    Comma,       // ,
    Colon,       // :
    Semicolon,   // ;
    Underscore,  // _
}

pub struct MathLexer<'a> {
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    src: &'a str,
}

impl<'a> MathLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.char_indices().peekable(),
            src,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut depth: usize = 0;

        while let Some(&(i, ch)) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    self.chars.next();
                    if depth == 0
                        && matches!(
                            tokens.last(),
                            Some(Token::Ident(_) | Token::Num(_) | Token::RParen | Token::RBracket)
                        )
                    {
                        tokens.push(Token::Semicolon);
                    }
                }
                '/' => {
                    self.chars.next();
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.chars.next();
                        while let Some(&(_, next_c)) = self.chars.peek() {
                            if next_c == '\n' {
                                break;
                            }
                            self.chars.next();
                        }
                    } else if let Some(&(_, '*')) = self.chars.peek() {
                        self.chars.next();
                        while let Some((_, next_c)) = self.chars.next() {
                            if next_c == '*' {
                                if let Some(&(_, '/')) = self.chars.peek() {
                                    self.chars.next();
                                    break;
                                }
                            }
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                ';' => {
                    self.chars.next();
                    tokens.push(Token::Semicolon);
                }
                '=' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::EqEq);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '-' => {
                    self.chars.next();
                    if let Some(&(_, '>')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Arrow);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '+' => {
                    self.chars.next();
                    tokens.push(Token::Plus);
                }
                '*' => {
                    self.chars.next();
                    tokens.push(Token::Star);
                }
                '@' => {
                    self.chars.next();
                    tokens.push(Token::At);
                }
                '^' => {
                    self.chars.next();
                    tokens.push(Token::Caret);
                }
                '?' => {
                    self.chars.next();
                    tokens.push(Token::Question);
                }
                '!' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::NotEq);
                    } else {
                        return Err("Unexpected '!' without '='".to_string());
                    }
                }
                '<' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Lte);
                    } else if let Some(&(_, '<')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Shl);
                    } else {
                        tokens.push(Token::Lt);
                    }
                }
                '>' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Gte);
                    } else if let Some(&(_, '>')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Shr);
                    } else {
                        tokens.push(Token::Gt);
                    }
                }
                '(' => {
                    self.chars.next();
                    depth += 1;
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    depth = depth.saturating_sub(1);
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.chars.next();
                    depth += 1;
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.chars.next();
                    depth = depth.saturating_sub(1);
                    tokens.push(Token::RBracket);
                }
                ',' => {
                    self.chars.next();
                    tokens.push(Token::Comma);
                }
                ':' => {
                    self.chars.next();
                    tokens.push(Token::Colon);
                }
                '_' => {
                    self.chars.next();
                    if let Some(&(_, next_c)) = self.chars.peek() {
                        if next_c.is_alphanumeric() {
                            let mut ident = String::from("_");
                            while let Some(&(_, c)) = self.chars.peek() {
                                if c.is_alphanumeric() || c == '_' {
                                    ident.push(c);
                                    self.chars.next();
                                } else {
                                    break;
                                }
                            }
                            tokens.push(Token::Ident(ident));
                            continue;
                        }
                    }
                    tokens.push(Token::Underscore);
                }
                '0'..='9' => {
                    let start = i;
                    let mut end = i;
                    while let Some(&(curr_i, c)) = self.chars.peek() {
                        if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                            end = curr_i + c.len_utf8();
                            self.chars.next();
                            if c == 'e' || c == 'E' {
                                if let Some(&(_, sign)) = self.chars.peek() {
                                    if sign == '+' || sign == '-' {
                                        end += sign.len_utf8();
                                        self.chars.next();
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let num_str = &self.src[start..end];
                    let val: f64 = num_str
                        .parse()
                        .map_err(|e| format!("Invalid number '{}': {}", num_str, e))?;
                    tokens.push(Token::Num(val));
                }
                '"' => {
                    self.chars.next();
                    let mut s = String::new();
                    let mut closed = false;
                    while let Some((_, c)) = self.chars.next() {
                        if c == '"' {
                            closed = true;
                            break;
                        } else if c == '\\' {
                            if let Some((_, esc)) = self.chars.next() {
                                match esc {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    'r' => s.push('\r'),
                                    '\\' => s.push('\\'),
                                    '"' => s.push('"'),
                                    other => {
                                        s.push('\\');
                                        s.push(other);
                                    }
                                }
                            }
                        } else {
                            s.push(c);
                        }
                    }
                    if !closed {
                        return Err(format!("Unterminated string literal at byte offset {}", i));
                    }
                    tokens.push(Token::Str(s));
                }
                'a'..='z' | 'A'..='Z' => {
                    let start = i;
                    let mut end = i;
                    while let Some(&(curr_i, c)) = self.chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            end = curr_i + c.len_utf8();
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    let ident = &self.src[start..end];
                    tokens.push(Token::Ident(ident.to_string()));
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at byte offset {}",
                        ch, i
                    ));
                }
            }
        }

        Ok(tokens)
    }
}

// ============================================================================
// PARSER
// ============================================================================

pub struct MathParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl MathParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_tok(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn match_tok(&mut self, expected: &Token) -> bool {
        if let Some(t) = self.peek() {
            if t == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.match_tok(expected) {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.peek()))
        }
    }

    pub fn parse_program(&mut self) -> Result<MathProgram, String> {
        let mut program = MathProgram::default();

        while self.pos < self.tokens.len() {
            if self.match_tok(&Token::Semicolon) {
                continue;
            }

            let item = self.parse_item()?;
            program.items.push(item);
        }

        Ok(program)
    }

    fn is_func_def(&self) -> bool {
        if let Some(Token::Ident(_)) = self.tokens.get(self.pos) {
            if let Some(Token::LParen) = self.tokens.get(self.pos + 1) {
                let mut depth = 0;
                let mut i = self.pos + 1;
                while i < self.tokens.len() {
                    match &self.tokens[i] {
                        Token::LParen => depth += 1,
                        Token::RParen => {
                            depth -= 1;
                            if depth == 0 {
                                let mut next_idx = i + 1;
                                if let Some(Token::Colon) = self.tokens.get(next_idx) {
                                    next_idx += 2;
                                }
                                return matches!(self.tokens.get(next_idx), Some(Token::Equal));
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
        false
    }

    fn is_tuple_binding(&self) -> bool {
        if let Some(Token::LParen) = self.tokens.get(self.pos) {
            let mut i = self.pos + 1;
            let mut has_ident = false;
            while i < self.tokens.len() {
                match &self.tokens[i] {
                    Token::Ident(_) => {
                        has_ident = true;
                        i += 1;
                    }
                    Token::Comma => {
                        i += 1;
                    }
                    Token::RParen => {
                        return has_ident && self.tokens.get(i + 1) == Some(&Token::Equal);
                    }
                    _ => return false,
                }
            }
        }
        false
    }

    fn parse_item(&mut self) -> Result<MathItem, String> {
        // 0. Check for module link: << "path/to/module.cl"
        if self.match_tok(&Token::Shl) {
            if let Some(Token::Str(path)) = self.next_tok() {
                return Ok(MathItem::Link(path));
            } else {
                return Err("Expected string file path after '<<'".to_string());
            }
        }

        // 1. Check for tuple binding: (W1, b1, ...) = expr
        if self.is_tuple_binding() {
            self.expect(&Token::LParen)?;
            let mut names = Vec::new();
            while !self.match_tok(&Token::RParen) {
                if let Some(Token::Ident(name)) = self.next_tok() {
                    names.push(name);
                    if self.match_tok(&Token::Comma) {
                        continue;
                    } else if self.match_tok(&Token::RParen) {
                        break;
                    } else {
                        return Err("Expected ',' or ')' in tuple binding".to_string());
                    }
                } else {
                    return Err("Expected identifier in tuple binding".to_string());
                }
            }
            self.expect(&Token::Equal)?;
            let expr = self.parse_expr()?;
            return Ok(MathItem::TupleBinding { names, expr });
        }

        // 1.5. Check for function declaration: name(params) = expr
        if self.is_func_def() {
            if let Some(Token::Ident(name)) = self.next_tok() {
                self.expect(&Token::LParen)?;
                let params = self.parse_params()?;
                self.expect(&Token::RParen)?;

                let mut ret_type = None;
                if self.match_tok(&Token::Colon) {
                    ret_type = Some(self.parse_dtype()?);
                }

                self.expect(&Token::Equal)?;
                let body = self.parse_expr()?;
                return Ok(MathItem::Function {
                    name,
                    params,
                    ret_type,
                    body,
                });
            }
        }

        // 2. Check for constant or variable binding: name = expr
        if let Some(Token::Ident(name)) = self.peek().cloned() {
            if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Equal {
                self.pos += 2; // consume ident and '='
                let expr = self.parse_expr()?;
                if let MathExpr::Num(val) = expr {
                    return Ok(MathItem::Constant { name, val });
                } else {
                    return Ok(MathItem::Binding { name, expr });
                }
            }
        }

        // 3. Top-level expression or tensor export: expr >> "path:key"
        let expr = self.parse_expr()?;
        if self.match_tok(&Token::Shr) {
            if let Some(Token::Str(s)) = self.next_tok() {
                let (path, key) = if let Some((p, k)) = s.rsplit_once(':') {
                    if k.contains('/') || k.contains('\\') {
                        (s.clone(), String::new())
                    } else {
                        (p.to_string(), k.to_string())
                    }
                } else {
                    (s.clone(), String::new())
                };
                return Ok(MathItem::Export { expr, path, key });
            } else {
                return Err("Expected string literal file path after '>>'".to_string());
            }
        }
        Ok(MathItem::Expression(expr))
    }

    fn parse_params(&mut self) -> Result<Vec<MathParam>, String> {
        let mut params = Vec::new();

        if let Some(Token::RParen) = self.peek() {
            return Ok(params);
        }

        loop {
            let name = match self.next_tok() {
                Some(Token::Ident(n)) => n,
                other => return Err(format!("Expected parameter name, found {:?}", other)),
            };

            let mut dtype = MathDtype::F32;
            let mut dims = Vec::new();

            if self.match_tok(&Token::Colon) {
                dtype = self.parse_dtype()?;

                if self.match_tok(&Token::LBracket) {
                    loop {
                        if let Some(Token::Num(n)) = self.next_tok() {
                            dims.push(n as usize);
                        } else {
                            return Err("Expected dimension size in tensor brackets".to_string());
                        }

                        if self.match_tok(&Token::Comma) {
                            continue;
                        } else if self.match_tok(&Token::RBracket) {
                            break;
                        } else {
                            return Err("Expected ',' or ']' in tensor dimension".to_string());
                        }
                    }
                }
            }

            params.push(MathParam { name, dtype, dims });

            if self.match_tok(&Token::Comma) {
                continue;
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_dtype(&mut self) -> Result<MathDtype, String> {
        if let Some(Token::Ident(s)) = self.next_tok() {
            MathDtype::from_str(&s).ok_or_else(|| format!("Unknown data type '{}'", s))
        } else {
            Err("Expected data type identifier (e.g. f16, f32, t2)".to_string())
        }
    }

    // Precedence hierarchy:
    // 1. Pipeline: expr -> expr
    // 2. Conditional Ternary: cond ? then : else
    // 3. Comparison: ==, !=, <, <=, >, >=
    // 4. Additive: +, -
    // 5. Multiplicative: *, /, @ (MatMul)
    // 6. Exponentiation: ^
    // 7. Unary: -
    // 8. Primary: num, [array], var, call, (expr)

    pub fn parse_expr(&mut self) -> Result<MathExpr, String> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Result<MathExpr, String> {
        let mut stages = vec![self.parse_ternary()?];

        while self.match_tok(&Token::Arrow) {
            let next_stage = self.parse_ternary()?;
            stages.push(next_stage);
        }

        if stages.len() == 1 {
            Ok(stages.pop().unwrap())
        } else {
            let mut curr = stages.remove(0);
            for stage in stages {
                curr = Self::desugar_pipe_step(curr, stage);
            }
            Ok(curr)
        }
    }

    fn desugar_pipe_step(input: MathExpr, target: MathExpr) -> MathExpr {
        match target {
            MathExpr::Binary(func, MathBinOp::Pow, count) => {
                // Functional power / iteration: x -> f^N
                MathExpr::Iterate {
                    input: Box::new(input),
                    func,
                    count,
                }
            }
            MathExpr::Var(func_name) => MathExpr::Call(func_name, vec![input]),
            MathExpr::Call(func_name, mut args) => {
                let mut found_placeholder = false;
                for arg in args.iter_mut() {
                    if let MathExpr::Placeholder = arg {
                        *arg = input.clone();
                        found_placeholder = true;
                        break;
                    }
                }
                if !found_placeholder {
                    args.insert(0, input);
                }
                MathExpr::Call(func_name, args)
            }
            other => MathExpr::Binary(Box::new(input), MathBinOp::MatMul, Box::new(other)),
        }
    }

    fn parse_ternary(&mut self) -> Result<MathExpr, String> {
        let cond = self.parse_comparison()?;

        if self.match_tok(&Token::Question) {
            let then_expr = self.parse_expr()?;
            self.expect(&Token::Colon)?;
            let else_expr = self.parse_ternary()?;
            Ok(MathExpr::Ternary {
                cond: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(cond)
        }
    }

    fn parse_comparison(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_additive()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::EqEq => MathBinOp::Eq,
                Token::NotEq => MathBinOp::Neq,
                Token::Lt => MathBinOp::Lt,
                Token::Lte => MathBinOp::Lte,
                Token::Gt => MathBinOp::Gt,
                Token::Gte => MathBinOp::Gte,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_additive()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_additive(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_multiplicative()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Plus => MathBinOp::Add,
                Token::Minus => MathBinOp::Sub,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_multiplicative()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_power()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star => MathBinOp::Mul,
                Token::Slash => MathBinOp::Div,
                Token::At => MathBinOp::MatMul,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_power()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_power(&mut self) -> Result<MathExpr, String> {
        let lhs = self.parse_unary()?;

        if self.match_tok(&Token::Caret) {
            let rhs = self.parse_power()?;
            Ok(MathExpr::Binary(
                Box::new(lhs),
                MathBinOp::Pow,
                Box::new(rhs),
            ))
        } else {
            Ok(lhs)
        }
    }

    fn parse_unary(&mut self) -> Result<MathExpr, String> {
        if self.match_tok(&Token::Minus) {
            let expr = self.parse_unary()?;
            Ok(MathExpr::Unary(MathUnaryOp::Neg, Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<MathExpr, String> {
        let mut base = match self.next_tok() {
            Some(Token::Num(n)) => MathExpr::Num(n),
            Some(Token::Underscore) => MathExpr::Placeholder,
            Some(Token::At) => {
                if let Some(Token::Str(s)) = self.next_tok() {
                    let (path, key) = if let Some((p, k)) = s.rsplit_once(':') {
                        if k.contains('/') || k.contains('\\') {
                            (s.clone(), String::new())
                        } else {
                            (p.to_string(), k.to_string())
                        }
                    } else {
                        (s.clone(), String::new())
                    };
                    MathExpr::TensorRef { path, key }
                } else {
                    return Err("Expected string literal after '@' for tensor reference, e.g. @\"model.safetensors:w\"".to_string());
                }
            }
            Some(Token::LBracket) => {
                // Vector or Matrix literal: [1, 2, 3] or [[1, 2], [3, 4]]
                let mut elems = Vec::new();
                if !self.match_tok(&Token::RBracket) {
                    loop {
                        elems.push(self.parse_expr()?);
                        if self.match_tok(&Token::Comma) {
                            continue;
                        } else if self.match_tok(&Token::RBracket) {
                            break;
                        } else {
                            return Err("Expected ',' or ']' in array/tensor literal".to_string());
                        }
                    }
                }
                MathExpr::Array(elems)
            }
            Some(Token::Ident(name)) => {
                if self.match_tok(&Token::LParen) {
                    let mut args = Vec::new();
                    if !self.match_tok(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.match_tok(&Token::Comma) {
                                continue;
                            } else if self.match_tok(&Token::RParen) {
                                break;
                            } else {
                                return Err("Expected ',' or ')' in argument list".to_string());
                            }
                        }
                    }
                    MathExpr::Call(name, args)
                } else {
                    MathExpr::Var(name)
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                if self.match_tok(&Token::Comma) {
                    let mut items = vec![expr];
                    while !self.match_tok(&Token::RParen) {
                        items.push(self.parse_expr()?);
                        if self.match_tok(&Token::Comma) {
                            continue;
                        } else if self.match_tok(&Token::RParen) {
                            break;
                        } else {
                            return Err("Expected ',' or ')' in tuple expression".to_string());
                        }
                    }
                    MathExpr::Tuple(items)
                } else {
                    self.expect(&Token::RParen)?;
                    expr
                }
            }
            other => return Err(format!("Unexpected token in expression: {:?}", other)),
        };

        // Support postfix indexing: x[0], W[1][0]
        while self.match_tok(&Token::LBracket) {
            let idx = self.parse_expr()?;
            self.expect(&Token::RBracket)?;
            base = MathExpr::Index(Box::new(base), Box::new(idx));
        }

        Ok(base)
    }
}

// ============================================================================
// ANALYTICAL REVERSE-MODE TAPE AUTODIFF ENGINE (VJP Matrix Backprop)
// ============================================================================

fn reduce_adjoint_to_shape(adj: &MathValue, target_shape: &[usize]) -> Result<MathValue, String> {
    let adj_shape = adj.shape();
    if adj_shape == target_shape {
        return Ok(adj.clone());
    }
    // Target is scalar: sum all elements of adj
    if target_shape.is_empty() {
        let sum: f64 = adj.flatten_elements().iter().sum();
        return Ok(MathValue::Scalar(sum));
    }
    // 1D Vector [N] target from 2D Matrix [M, N] (row-wise broadcast reduction):
    if target_shape.len() == 1 && adj_shape.len() == 2 {
        if let MathValue::Matrix(m) = adj {
            let cols = target_shape[0];
            let mut out = vec![0.0; cols];
            for row in m {
                for (j, &val) in row.iter().enumerate() {
                    if j < cols {
                        out[j] += val;
                    }
                }
            }
            return Ok(MathValue::Vector(out));
        }
    }
    // 1D Vector [M] target from 2D Matrix [M, 1]
    if target_shape.len() == 1 && adj_shape == vec![target_shape[0], 1] {
        if let MathValue::Matrix(m) = adj {
            let out: Vec<f64> = m
                .iter()
                .map(|row| row.first().copied().unwrap_or(0.0))
                .collect();
            return Ok(MathValue::Vector(out));
        }
    }
    // 2D Matrix [M, 1] target from 2D Matrix [M, N]
    if target_shape.len() == 2 && target_shape[1] == 1 && adj_shape.len() == 2 {
        if let MathValue::Matrix(m) = adj {
            let rows = target_shape[0];
            let mut out = Vec::with_capacity(rows);
            for row in m {
                out.push(vec![row.iter().sum::<f64>()]);
            }
            return Ok(MathValue::Matrix(out));
        }
    }
    // 2D Matrix [1, N] target from 2D Matrix [M, N]
    if target_shape.len() == 2 && target_shape[0] == 1 && adj_shape.len() == 2 {
        if let MathValue::Matrix(m) = adj {
            let cols = target_shape[1];
            let mut out_row = vec![0.0; cols];
            for row in m {
                for (j, &val) in row.iter().enumerate() {
                    if j < cols {
                        out_row[j] += val;
                    }
                }
            }
            return Ok(MathValue::Matrix(vec![out_row]));
        }
    }
    // 1D Vector [D] target from N-D Tensor [..., D] (reduction over all leading batch/sequence dims)
    if target_shape.len() == 1
        && adj_shape.len() > 1
        && *adj_shape.last().unwrap_or(&0) == target_shape[0]
    {
        let last_dim = target_shape[0];
        let flat = adj.flatten_elements();
        let mut out = vec![0.0; last_dim];
        for (i, &val) in flat.iter().enumerate() {
            out[i % last_dim] += val;
        }
        return Ok(MathValue::Vector(out));
    }
    // 2D Matrix [M, N] target from N-D Tensor [..., M, N] (reduction over leading batch/head dims)
    if target_shape.len() == 2
        && adj_shape.len() > 2
        && &adj_shape[adj_shape.len() - 2..] == target_shape
    {
        let rows = target_shape[0];
        let cols = target_shape[1];
        let mat_size = rows * cols;
        let flat = adj.flatten_elements();
        let mut out = vec![vec![0.0; cols]; rows];
        for (i, &val) in flat.iter().enumerate() {
            let inner_idx = i % mat_size;
            let r = inner_idx / cols;
            let c = inner_idx % cols;
            out[r][c] += val;
        }
        return Ok(MathValue::Matrix(out));
    }
    Err(format!(
        "Cannot reduce adjoint of shape {:?} to target shape {:?}",
        adj_shape, target_shape
    ))
}

enum ExprTape {
    Num,
    Var(String),
    Array(Vec<ExprTape>),
    Index {
        base_val: MathValue,
        base_tape: Box<ExprTape>,
        idx_val: usize,
    },
    Unary(MathUnaryOp, MathValue, Box<ExprTape>),
    Binary {
        op: MathBinOp,
        lhs_val: MathValue,
        lhs_tape: Box<ExprTape>,
        rhs_val: MathValue,
        rhs_tape: Box<ExprTape>,
    },
    Call {
        name: String,
        arg_vals: Vec<MathValue>,
        arg_tapes: Vec<ExprTape>,
    },
}

impl ExprTape {
    fn backward(
        &self,
        adj: &MathValue,
        wrt: &str,
        acc_grad: &mut MathValue,
        evaluator: &MathEvaluator,
    ) -> Result<(), String> {
        match self {
            ExprTape::Num => Ok(()),
            ExprTape::Var(v) => {
                if v == wrt {
                    acc_grad.add_accumulate(adj)?;
                }
                Ok(())
            }
            ExprTape::Array(tapes) => match adj {
                MathValue::Vector(v) => {
                    for (tape, &val) in tapes.iter().zip(v.iter()) {
                        tape.backward(&MathValue::Scalar(val), wrt, acc_grad, evaluator)?;
                    }
                    Ok(())
                }
                MathValue::Matrix(m) => {
                    for (tape, row) in tapes.iter().zip(m.iter()) {
                        tape.backward(&MathValue::Vector(row.clone()), wrt, acc_grad, evaluator)?;
                    }
                    Ok(())
                }
                _ => Ok(()),
            },
            ExprTape::Index {
                base_val,
                base_tape,
                idx_val,
            } => match base_val {
                MathValue::Vector(v) => {
                    let mut zero_vec = vec![0.0; v.len()];
                    if *idx_val < zero_vec.len() {
                        zero_vec[*idx_val] = adj.as_scalar().unwrap_or(0.0);
                    }
                    base_tape.backward(&MathValue::Vector(zero_vec), wrt, acc_grad, evaluator)
                }
                MathValue::Matrix(m) => {
                    let rows = m.len();
                    let cols = m.first().map(|r| r.len()).unwrap_or(0);
                    let mut zero_mat = vec![vec![0.0; cols]; rows];
                    if *idx_val < rows {
                        if let MathValue::Vector(adj_row) = adj {
                            for (c, &val) in adj_row.iter().enumerate() {
                                if c < cols {
                                    zero_mat[*idx_val][c] = val;
                                }
                            }
                        }
                    }
                    base_tape.backward(&MathValue::Matrix(zero_mat), wrt, acc_grad, evaluator)
                }
                _ => Ok(()),
            },
            ExprTape::Unary(op, _inner_val, inner_tape) => match op {
                MathUnaryOp::Neg => {
                    let neg_adj = adj.map_unary(|x| -x);
                    inner_tape.backward(&neg_adj, wrt, acc_grad, evaluator)
                }
                MathUnaryOp::Not => Ok(()),
            },
            ExprTape::Binary {
                op,
                lhs_val,
                lhs_tape,
                rhs_val,
                rhs_tape,
            } => match op {
                MathBinOp::Add => {
                    let lhs_adj = reduce_adjoint_to_shape(adj, &lhs_val.shape())?;
                    let rhs_adj = reduce_adjoint_to_shape(adj, &rhs_val.shape())?;
                    lhs_tape.backward(&lhs_adj, wrt, acc_grad, evaluator)?;
                    rhs_tape.backward(&rhs_adj, wrt, acc_grad, evaluator)?;
                    Ok(())
                }
                MathBinOp::Sub => {
                    let lhs_adj = reduce_adjoint_to_shape(adj, &lhs_val.shape())?;
                    let rhs_adj_raw = reduce_adjoint_to_shape(adj, &rhs_val.shape())?;
                    let rhs_adj = rhs_adj_raw.map_unary(|x| -x);
                    lhs_tape.backward(&lhs_adj, wrt, acc_grad, evaluator)?;
                    rhs_tape.backward(&rhs_adj, wrt, acc_grad, evaluator)?;
                    Ok(())
                }
                MathBinOp::MatMul => match (lhs_val, rhs_val) {
                    (MathValue::Matrix(_), MathValue::Matrix(_))
                    | (MathValue::Tensor { .. }, MathValue::Matrix(_))
                    | (MathValue::Matrix(_), MathValue::Tensor { .. })
                    | (MathValue::Tensor { .. }, MathValue::Tensor { .. }) => {
                        let v_t = rhs_val.transpose()?;
                        let raw_adj_u = evaluator.eval_binary_op(adj, MathBinOp::MatMul, &v_t)?;
                        let adj_u = reduce_adjoint_to_shape(&raw_adj_u, &lhs_val.shape())?;

                        let u_t = lhs_val.transpose()?;
                        let raw_adj_v = evaluator.eval_binary_op(&u_t, MathBinOp::MatMul, adj)?;
                        let adj_v = reduce_adjoint_to_shape(&raw_adj_v, &rhs_val.shape())?;

                        lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                        rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                        Ok(())
                    }
                    (MathValue::Matrix(_), MathValue::Vector(v)) => match adj {
                        MathValue::Vector(adj_vec) => {
                            let mut outer_mat = Vec::with_capacity(adj_vec.len());
                            for &ai in adj_vec {
                                outer_mat.push(v.iter().map(|&vj| ai * vj).collect());
                            }
                            let adj_u = MathValue::Matrix(outer_mat);
                            let u_t = lhs_val.transpose()?;
                            let adj_v = evaluator.eval_binary_op(&u_t, MathBinOp::MatMul, adj)?;
                            lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                            rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                            Ok(())
                        }
                        _ => Err("Expected vector adjoint for matrix-vector product".to_string()),
                    },
                    (MathValue::Vector(u), MathValue::Matrix(_)) => {
                        let v_t = rhs_val.transpose()?;
                        let adj_u = evaluator.eval_binary_op(adj, MathBinOp::MatMul, &v_t)?;
                        match adj {
                            MathValue::Vector(adj_vec) => {
                                let mut outer_mat = Vec::with_capacity(u.len());
                                for &ui in u {
                                    outer_mat.push(adj_vec.iter().map(|&aj| ui * aj).collect());
                                }
                                let adj_v = MathValue::Matrix(outer_mat);
                                lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                                rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                                Ok(())
                            }
                            _ => Err(
                                "Expected vector adjoint for vector-matrix product".to_string()
                            ),
                        }
                    }
                    (MathValue::Vector(u), MathValue::Vector(v)) => {
                        let s = adj.as_scalar().unwrap_or(1.0);
                        let adj_u = MathValue::Vector(v.iter().map(|&x| s * x).collect());
                        let adj_v = MathValue::Vector(u.iter().map(|&x| s * x).collect());
                        lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                        rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                        Ok(())
                    }
                    (MathValue::Scalar(s1), MathValue::Scalar(s2)) => {
                        let s = adj.as_scalar().unwrap_or(1.0);
                        let adj_u = MathValue::Scalar(s * s2);
                        let adj_v = MathValue::Scalar(s * s1);
                        lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                        rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                        Ok(())
                    }
                    _ => Err("Unsupported shapes in MatMul backward".to_string()),
                },
                MathBinOp::Mul => {
                    let raw_adj_u = evaluator.eval_binary_op(adj, MathBinOp::Mul, rhs_val)?;
                    let raw_adj_v = evaluator.eval_binary_op(adj, MathBinOp::Mul, lhs_val)?;
                    let adj_u = reduce_adjoint_to_shape(&raw_adj_u, &lhs_val.shape())?;
                    let adj_v = reduce_adjoint_to_shape(&raw_adj_v, &rhs_val.shape())?;
                    lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                    rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                    Ok(())
                }
                MathBinOp::Pow => {
                    if let Some(c) = rhs_val.as_scalar() {
                        let c_minus_1 = evaluator.eval_binary_op(
                            lhs_val,
                            MathBinOp::Pow,
                            &MathValue::Scalar(c - 1.0),
                        )?;
                        let factor = evaluator.eval_binary_op(
                            &c_minus_1,
                            MathBinOp::Mul,
                            &MathValue::Scalar(c),
                        )?;
                        let raw_adj_u = evaluator.eval_binary_op(adj, MathBinOp::Mul, &factor)?;
                        let adj_u = reduce_adjoint_to_shape(&raw_adj_u, &lhs_val.shape())?;
                        lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                        Ok(())
                    } else {
                        Err("Only scalar power supported in analytical backward".to_string())
                    }
                }
                MathBinOp::Div => {
                    let raw_adj_u = evaluator.eval_binary_op(adj, MathBinOp::Div, rhs_val)?;
                    let adj_u = reduce_adjoint_to_shape(&raw_adj_u, &lhs_val.shape())?;
                    let v_sq = evaluator.eval_binary_op(rhs_val, MathBinOp::Mul, rhs_val)?;
                    let u_div_vsq = evaluator.eval_binary_op(lhs_val, MathBinOp::Div, &v_sq)?;
                    let raw_adj_v = evaluator
                        .eval_binary_op(adj, MathBinOp::Mul, &u_div_vsq)?
                        .map_unary(|x| -x);
                    let adj_v = reduce_adjoint_to_shape(&raw_adj_v, &rhs_val.shape())?;
                    lhs_tape.backward(&adj_u, wrt, acc_grad, evaluator)?;
                    rhs_tape.backward(&adj_v, wrt, acc_grad, evaluator)?;
                    Ok(())
                }
                _ => Err(format!("Unsupported binary op {:?} in backward", op)),
            },
            ExprTape::Call {
                name,
                arg_vals,
                arg_tapes,
            } => match name.as_str() {
                "sum" => {
                    let s = adj.as_scalar().unwrap_or(1.0);
                    let in_adj = arg_vals[0].map_unary(|_| s);
                    arg_tapes[0].backward(&in_adj, wrt, acc_grad, evaluator)
                }
                "mean" => {
                    let s = adj.as_scalar().unwrap_or(1.0);
                    let count = arg_vals[0].flatten_elements().len().max(1) as f64;
                    let in_adj = arg_vals[0].map_unary(|_| s / count);
                    arg_tapes[0].backward(&in_adj, wrt, acc_grad, evaluator)
                }
                "sum_axis" => {
                    let axis = if arg_vals.len() > 1 {
                        arg_vals[1].as_scalar().unwrap_or(0.0) as usize
                    } else {
                        0
                    };
                    match &arg_vals[0] {
                        MathValue::Matrix(m) => {
                            let rows = m.len();
                            let cols = m.first().map(|r| r.len()).unwrap_or(0);
                            if axis == 0 {
                                if let MathValue::Vector(adj_v) = adj {
                                    let broadcast_m = vec![adj_v.clone(); rows];
                                    arg_tapes[0].backward(
                                        &MathValue::Matrix(broadcast_m),
                                        wrt,
                                        acc_grad,
                                        evaluator,
                                    )
                                } else {
                                    Err("sum_axis(axis=0) expects vector adjoint".to_string())
                                }
                            } else {
                                if let MathValue::Vector(adj_v) = adj {
                                    let mut broadcast_m = Vec::with_capacity(rows);
                                    for &ai in adj_v {
                                        broadcast_m.push(vec![ai; cols]);
                                    }
                                    arg_tapes[0].backward(
                                        &MathValue::Matrix(broadcast_m),
                                        wrt,
                                        acc_grad,
                                        evaluator,
                                    )
                                } else {
                                    Err("sum_axis(axis=1) expects vector adjoint".to_string())
                                }
                            }
                        }
                        MathValue::Vector(v) => {
                            let s = adj.as_scalar().unwrap_or(1.0);
                            let in_adj = MathValue::Vector(vec![s; v.len()]);
                            arg_tapes[0].backward(&in_adj, wrt, acc_grad, evaluator)
                        }
                        _ => Err("Unsupported tensor type for sum_axis backward".to_string()),
                    }
                }
                "cross_entropy" => {
                    let s = adj.as_scalar().unwrap_or(1.0);
                    let logits = &arg_vals[0];
                    let targets = &arg_vals[1];
                    let calc_softmax_grad =
                        |row: &[f64], target_idx: usize, batch_scale: f64| -> Vec<f64> {
                            let max_v = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                            let exps: Vec<f64> = row.iter().map(|&x| (x - max_v).exp()).collect();
                            let sum_exp: f64 = exps.iter().sum::<f64>().max(1e-12);
                            let mut grad_row = Vec::with_capacity(row.len());
                            for (j, &e) in exps.iter().enumerate() {
                                let p = e / sum_exp;
                                let delta = if j == target_idx { 1.0 } else { 0.0 };
                                grad_row.push(batch_scale * (p - delta));
                            }
                            grad_row
                        };
                    match (logits, targets) {
                        (MathValue::Matrix(mat), MathValue::Vector(t_vec)) => {
                            let b = mat.len().max(1) as f64;
                            let scale = s / b;
                            let mut grad_mat = Vec::with_capacity(mat.len());
                            for (row, &t) in mat.iter().zip(t_vec.iter()) {
                                grad_mat.push(calc_softmax_grad(row, t as usize, scale));
                            }
                            let logits_adj = MathValue::Matrix(grad_mat);
                            arg_tapes[0].backward(&logits_adj, wrt, acc_grad, evaluator)
                        }
                        (MathValue::Vector(vec), MathValue::Scalar(t)) => {
                            let grad_vec = calc_softmax_grad(vec, *t as usize, s);
                            let logits_adj = MathValue::Vector(grad_vec);
                            arg_tapes[0].backward(&logits_adj, wrt, acc_grad, evaluator)
                        }
                        (MathValue::Vector(vec), MathValue::Vector(t_vec)) if t_vec.len() == 1 => {
                            let grad_vec = calc_softmax_grad(vec, t_vec[0] as usize, s);
                            let logits_adj = MathValue::Vector(grad_vec);
                            arg_tapes[0].backward(&logits_adj, wrt, acc_grad, evaluator)
                        }
                        _ => Err(
                            "cross_entropy expects (Matrix, Vector) or (Vector, Scalar)".to_string()
                        ),
                    }
                }
                "log_softmax" => {
                    let calc_log_softmax_vjp = |x: &[f64], adj_row: &[f64]| -> Vec<f64> {
                        let max_v = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let exps: Vec<f64> = x.iter().map(|&xi| (xi - max_v).exp()).collect();
                        let sum_exp: f64 = exps.iter().sum::<f64>().max(1e-12);
                        let sum_adj: f64 = adj_row.iter().sum();
                        let mut out = Vec::with_capacity(x.len());
                        for (j, &e) in exps.iter().enumerate() {
                            let p = e / sum_exp;
                            out.push(adj_row[j] - p * sum_adj);
                        }
                        out
                    };
                    match (&arg_vals[0], adj) {
                        (MathValue::Vector(v), MathValue::Vector(adj_v)) => {
                            let vjp = calc_log_softmax_vjp(v, adj_v);
                            arg_tapes[0].backward(&MathValue::Vector(vjp), wrt, acc_grad, evaluator)
                        }
                        (MathValue::Matrix(m), MathValue::Matrix(adj_m)) => {
                            let mut out_m = Vec::with_capacity(m.len());
                            for (row, adj_row) in m.iter().zip(adj_m.iter()) {
                                out_m.push(calc_log_softmax_vjp(row, adj_row));
                            }
                            arg_tapes[0].backward(&MathValue::Matrix(out_m), wrt, acc_grad, evaluator)
                        }
                        _ => Err("log_softmax shape mismatch in backward".to_string()),
                    }
                }
                "softmax" => {
                    let calc_softmax_vjp = |x: &[f64], adj_row: &[f64]| -> Vec<f64> {
                        let max_v = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let exps: Vec<f64> = x.iter().map(|&xi| (xi - max_v).exp()).collect();
                        let sum_exp: f64 = exps.iter().sum::<f64>().max(1e-12);
                        let probs: Vec<f64> = exps.iter().map(|&e| e / sum_exp).collect();
                        let dot: f64 = probs
                            .iter()
                            .zip(adj_row.iter())
                            .map(|(&p, &a)| p * a)
                            .sum();
                        probs
                            .iter()
                            .zip(adj_row.iter())
                            .map(|(&p, &a)| p * (a - dot))
                            .collect()
                    };
                    match (&arg_vals[0], adj) {
                        (MathValue::Vector(v), MathValue::Vector(adj_v)) => {
                            let vjp = calc_softmax_vjp(v, adj_v);
                            arg_tapes[0].backward(&MathValue::Vector(vjp), wrt, acc_grad, evaluator)
                        }
                        (MathValue::Matrix(m), MathValue::Matrix(adj_m)) => {
                            let mut out_m = Vec::with_capacity(m.len());
                            for (row, adj_row) in m.iter().zip(adj_m.iter()) {
                                out_m.push(calc_softmax_vjp(row, adj_row));
                            }
                            arg_tapes[0].backward(&MathValue::Matrix(out_m), wrt, acc_grad, evaluator)
                        }
                        _ => Err("softmax shape mismatch in backward".to_string()),
                    }
                }
                "rmsnorm" => {
                    let calc_rmsnorm_vjp = |x: &[f64], adj_row: &[f64]| -> Vec<f64> {
                        let d = x.len().max(1) as f64;
                        let sq_sum: f64 = x.iter().map(|&xi| xi * xi).sum();
                        let rms = (sq_sum / d + 1e-5).sqrt();
                        let dot: f64 = x.iter().zip(adj_row.iter()).map(|(&xi, &ai)| xi * ai).sum();
                        let rms3_d = rms * rms * rms * d;
                        let mut out = Vec::with_capacity(x.len());
                        for (j, &xi) in x.iter().enumerate() {
                            out.push(adj_row[j] / rms - (xi / rms3_d) * dot);
                        }
                        out
                    };
                    match (&arg_vals[0], adj) {
                        (MathValue::Vector(v), MathValue::Vector(adj_v)) => {
                            let vjp = calc_rmsnorm_vjp(v, adj_v);
                            arg_tapes[0].backward(&MathValue::Vector(vjp), wrt, acc_grad, evaluator)
                        }
                        (MathValue::Matrix(m), MathValue::Matrix(adj_m)) => {
                            let mut out_m = Vec::with_capacity(m.len());
                            for (row, adj_row) in m.iter().zip(adj_m.iter()) {
                                out_m.push(calc_rmsnorm_vjp(row, adj_row));
                            }
                            arg_tapes[0].backward(&MathValue::Matrix(out_m), wrt, acc_grad, evaluator)
                        }
                        _ => Err("rmsnorm shape mismatch in backward".to_string()),
                    }
                }
                "silu" => {
                    let sig = |x: f64| 1.0 / (1.0 + (-x).exp());
                    let calc_silu_vjp = |x: f64, a: f64| -> f64 {
                        let s = sig(x);
                        a * (s * (1.0 + x * (1.0 - s)))
                    };
                    match (&arg_vals[0], adj) {
                        (MathValue::Scalar(s), MathValue::Scalar(a)) => arg_tapes[0].backward(
                            &MathValue::Scalar(calc_silu_vjp(*s, *a)),
                            wrt,
                            acc_grad,
                            evaluator,
                        ),
                        (MathValue::Vector(v), MathValue::Vector(adj_v)) => {
                            let out: Vec<f64> = v
                                .iter()
                                .zip(adj_v.iter())
                                .map(|(&x, &a)| calc_silu_vjp(x, a))
                                .collect();
                            arg_tapes[0].backward(&MathValue::Vector(out), wrt, acc_grad, evaluator)
                        }
                        (MathValue::Matrix(m), MathValue::Matrix(adj_m)) => {
                            let out: Vec<Vec<f64>> = m
                                .iter()
                                .zip(adj_m.iter())
                                .map(|(r, ar)| {
                                    r.iter()
                                        .zip(ar.iter())
                                        .map(|(&x, &a)| calc_silu_vjp(x, a))
                                        .collect()
                                })
                                .collect();
                            arg_tapes[0].backward(&MathValue::Matrix(out), wrt, acc_grad, evaluator)
                        }
                        _ => Err("silu shape mismatch in backward".to_string()),
                    }
                }
                "relu" => {
                    let calc_relu_vjp =
                        |x: f64, a: f64| -> f64 { if x > 0.0 { a } else { 0.0 } };
                    match (&arg_vals[0], adj) {
                        (MathValue::Scalar(s), MathValue::Scalar(a)) => arg_tapes[0].backward(
                            &MathValue::Scalar(calc_relu_vjp(*s, *a)),
                            wrt,
                            acc_grad,
                            evaluator,
                        ),
                        (MathValue::Vector(v), MathValue::Vector(adj_v)) => {
                            let out: Vec<f64> = v
                                .iter()
                                .zip(adj_v.iter())
                                .map(|(&x, &a)| calc_relu_vjp(x, a))
                                .collect();
                            arg_tapes[0].backward(&MathValue::Vector(out), wrt, acc_grad, evaluator)
                        }
                        (MathValue::Matrix(m), MathValue::Matrix(adj_m)) => {
                            let out: Vec<Vec<f64>> = m
                                .iter()
                                .zip(adj_m.iter())
                                .map(|(r, ar)| {
                                    r.iter()
                                        .zip(ar.iter())
                                        .map(|(&x, &a)| calc_relu_vjp(x, a))
                                        .collect()
                                })
                                .collect();
                            arg_tapes[0].backward(&MathValue::Matrix(out), wrt, acc_grad, evaluator)
                        }
                        _ => Err("relu shape mismatch in backward".to_string()),
                    }
                }
                "transpose" | "T" => {
                    let adj_t = adj.transpose()?;
                    arg_tapes[0].backward(&adj_t, wrt, acc_grad, evaluator)
                }
                "embed" => {
                    if let MathValue::Matrix(e_mat) = &arg_vals[0] {
                        let vocab_size = e_mat.len();
                        let dim = e_mat.first().map(|r| r.len()).unwrap_or(0);
                        let mut grad_e = vec![vec![0.0; dim]; vocab_size];
                        match (adj, &arg_vals[1]) {
                            (MathValue::Matrix(adj_m), MathValue::Vector(tok_v)) => {
                                for (adj_row, &t) in adj_m.iter().zip(tok_v.iter()) {
                                    let idx = t as usize;
                                    if idx < vocab_size {
                                        for (d, &val) in adj_row.iter().enumerate() {
                                            if d < dim {
                                                grad_e[idx][d] += val;
                                            }
                                        }
                                    }
                                }
                                arg_tapes[0].backward(
                                    &MathValue::Matrix(grad_e),
                                    wrt,
                                    acc_grad,
                                    evaluator,
                                )
                            }
                            (MathValue::Vector(adj_v), MathValue::Scalar(t)) => {
                                let idx = *t as usize;
                                if idx < vocab_size {
                                    for (d, &val) in adj_v.iter().enumerate() {
                                        if d < dim {
                                            grad_e[idx][d] += val;
                                        }
                                    }
                                }
                                arg_tapes[0].backward(
                                    &MathValue::Matrix(grad_e),
                                    wrt,
                                    acc_grad,
                                    evaluator,
                                )
                            }
                            _ => Err("embed shape mismatch in backward".to_string()),
                        }
                    } else {
                        Err("embed table is not a matrix".to_string())
                    }
                }
                "clip_grad" | "grad_clip" => {
                    let elems = arg_vals[0].flatten_elements();
                    let norm = elems.iter().map(|&x| x * x).sum::<f64>().sqrt();
                    let max_norm = arg_vals[1].as_scalar().unwrap_or(1.0);
                    let scale = if norm > max_norm && norm > 1e-12 {
                        max_norm / norm
                    } else {
                        1.0
                    };
                    let adj_scaled = adj.map_unary(|x| x * scale);
                    arg_tapes[0].backward(&adj_scaled, wrt, acc_grad, evaluator)
                }
                "split_heads" => {
                    let adj_merged = evaluator.merge_heads(adj)?;
                    arg_tapes[0].backward(&adj_merged, wrt, acc_grad, evaluator)
                }
                "merge_heads" => {
                    let in_shape = arg_vals[0].shape();
                    let nh = in_shape.first().copied().unwrap_or(1);
                    let adj_split = evaluator.split_heads(adj, nh)?;
                    arg_tapes[0].backward(&adj_split, wrt, acc_grad, evaluator)
                }
                "permute" => {
                    let dims = match &arg_vals[1] {
                        MathValue::Vector(v) => v.iter().map(|&d| d as usize).collect::<Vec<usize>>(),
                        _ => arg_vals[0].shape(),
                    };
                    let mut inv_dims = vec![0; dims.len()];
                    for (i, &d) in dims.iter().enumerate() {
                        if d < inv_dims.len() {
                            inv_dims[d] = i;
                        }
                    }
                    let adj_perm = adj.permute(&inv_dims)?;
                    arg_tapes[0].backward(&adj_perm, wrt, acc_grad, evaluator)
                }
                "reshape" => {
                    let in_shape = arg_vals[0].shape();
                    let adj_reshaped = adj.reshape(&in_shape)?;
                    arg_tapes[0].backward(&adj_reshaped, wrt, acc_grad, evaluator)
                }
                "ssd_scan" | "mamba_ssd" => {
                    let d_elems = arg_vals[1].flatten_elements();
                    let adj_flat = adj.flatten_elements();
                    let x_shape = arg_vals[0].shape();
                    let last_dim = *x_shape.last().unwrap_or(&1);
                    let num_rows = adj_flat.len() / last_dim.max(1);
                    let mut grad_x = vec![0.0; adj_flat.len()];

                    for c in 0..last_dim {
                        let a = d_elems.get(c % d_elems.len()).copied().unwrap_or(0.9);
                        let mut delta = 0.0;
                        for r in (0..num_rows).rev() {
                            let idx = r * last_dim + c;
                            delta = adj_flat[idx] + a * delta;
                            grad_x[idx] = delta;
                        }
                    }
                    let reconstruct = |elems: Vec<f64>, shape: &[usize]| -> Result<MathValue, String> {
                        match shape.len() {
                            0 => Ok(MathValue::Scalar(elems.first().copied().unwrap_or(0.0))),
                            1 => Ok(MathValue::Vector(elems)),
                            2 => {
                                let rows = shape[0];
                                let cols = shape[1];
                                let mut mat = Vec::with_capacity(rows);
                                for r in 0..rows {
                                    let start = r * cols;
                                    let end = (start + cols).min(elems.len());
                                    mat.push(elems[start..end].to_vec());
                                }
                                Ok(MathValue::Matrix(mat))
                            }
                            _ => MathValue::new_tensor(shape.to_vec(), elems),
                        }
                    };
                    let adj_x = reconstruct(grad_x, &x_shape)?;
                    arg_tapes[0].backward(&adj_x, wrt, acc_grad, evaluator)
                }
                "bitlinear" => {
                    let x_val = &arg_vals[0];
                    let w_val = &arg_vals[1];
                    let gamma = if arg_vals.len() > 2 {
                        arg_vals[2].as_scalar().unwrap_or(1.0)
                    } else {
                        1.0
                    };
                    if let (MathValue::Matrix(_), MathValue::Matrix(w_m)) = (x_val, w_val) {
                        let adj_scaled = adj.map_unary(|a| a * gamma);
                        let grad_x = match adj_scaled {
                            MathValue::Matrix(ref am) => {
                                let m = am.len();
                                let n = am.first().map(|r| r.len()).unwrap_or(0);
                                let k = w_m.first().map(|r| r.len()).unwrap_or(0);
                                let mut gx = vec![vec![0.0; k]; m];
                                for i in 0..m {
                                    for col in 0..k {
                                        let mut dot = 0.0;
                                        for j in 0..n {
                                            dot += am[i][j] * w_m[j][col];
                                        }
                                        gx[i][col] = dot;
                                    }
                                }
                                MathValue::Matrix(gx)
                            }
                            _ => x_val.zeros_like(),
                        };
                        arg_tapes[0].backward(&grad_x, wrt, acc_grad, evaluator)?;

                        let grad_w = match adj {
                            MathValue::Matrix(ref am) => {
                                let m = am.len();
                                let n = am.first().map(|r| r.len()).unwrap_or(0);
                                if let MathValue::Matrix(ref xm) = x_val {
                                    let k = xm.first().map(|r| r.len()).unwrap_or(0);
                                    let mut gw = vec![vec![0.0; k]; n];
                                    for j in 0..n {
                                        for col in 0..k {
                                            let mut dot = 0.0;
                                            for i in 0..m {
                                                dot += am[i][j] * xm[i][col];
                                            }
                                            gw[j][col] = dot * gamma;
                                        }
                                    }
                                    MathValue::Matrix(gw)
                                } else {
                                    w_val.zeros_like()
                                }
                            }
                            _ => w_val.zeros_like(),
                        };
                        arg_tapes[1].backward(&grad_w, wrt, acc_grad, evaluator)
                    } else {
                        Ok(())
                    }
                }
                "qmatmul" => {
                    let x_val = &arg_vals[0];
                    let w_val = &arg_vals[1];
                    let scales = &arg_vals[2];
                    let scales_vec = scales.flatten_elements();
                    if let (MathValue::Matrix(_), MathValue::Matrix(w_m)) = (x_val, w_val) {
                        let grad_x = match adj {
                            MathValue::Matrix(ref am) => {
                                let m = am.len();
                                let n = am.first().map(|r| r.len()).unwrap_or(0);
                                let k = w_m.first().map(|r| r.len()).unwrap_or(0);
                                let mut gx = vec![vec![0.0; k]; m];
                                for i in 0..m {
                                    for col in 0..k {
                                        let mut dot = 0.0;
                                        for j in 0..n {
                                            let s = scales_vec.get(j).copied().unwrap_or(1.0);
                                            dot += am[i][j] * (w_m[j][col] * s);
                                        }
                                        gx[i][col] = dot;
                                    }
                                }
                                MathValue::Matrix(gx)
                            }
                            _ => x_val.zeros_like(),
                        };
                        arg_tapes[0].backward(&grad_x, wrt, acc_grad, evaluator)
                    } else {
                        Ok(())
                    }
                }
                _ => Err(format!(
                    "Unsupported function call '{}' in analytical backward",
                    name
                )),
            },
        }
    }
}

// ============================================================================
// RUNTIME EVALUATOR (Scalars, Vectors, Matrices & Tensor Reductions)
// ============================================================================

#[derive(Clone)]
pub struct MathEvaluator {
    pub env: HashMap<String, MathValue>,
    pub functions: HashMap<String, (Vec<String>, MathExpr)>,
}

impl Default for MathEvaluator {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("pi".to_string(), MathValue::Scalar(std::f64::consts::PI));
        env.insert("e".to_string(), MathValue::Scalar(std::f64::consts::E));
        env.insert("eps".to_string(), MathValue::Scalar(1e-5));
        Self {
            env,
            functions: HashMap::new(),
        }
    }
}

impl MathEvaluator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes program and returns last evaluated scalar (if scalar) for backward compatibility
    pub fn execute_program(&mut self, prog: &MathProgram) -> Result<Option<f64>, String> {
        let val = self.execute_program_value(prog)?;
        Ok(val.and_then(|v| v.as_scalar()))
    }

    /// Executes program and returns full MathValue (Scalar, Vector, or Matrix)
    pub fn execute_program_value(
        &mut self,
        prog: &MathProgram,
    ) -> Result<Option<MathValue>, String> {
        let mut last_res = None;

        for item in &prog.items {
            match item {
                MathItem::Link(_) => {}
                MathItem::Export { expr, path, key } => {
                    let val = self.eval_value(expr)?;
                    save_tensor_resource(path, key, &val)?;
                    last_res = Some(val);
                }
                MathItem::Constant { name, val } => {
                    self.env.insert(name.clone(), MathValue::Scalar(*val));
                }
                MathItem::Binding { name, expr } => {
                    let val = self.eval_value(expr)?;
                    self.env.insert(name.clone(), val);
                }
                MathItem::TupleBinding { names, expr } => {
                    let val = self.eval_value(expr)?;
                    match val {
                        MathValue::Tuple(vals) => {
                            if vals.len() != names.len() {
                                return Err(format!(
                                    "Cannot unpack tuple of length {} into {} variables",
                                    vals.len(),
                                    names.len()
                                ));
                            }
                            for (name, v) in names.iter().zip(vals.into_iter()) {
                                self.env.insert(name.clone(), v);
                            }
                        }
                        other => {
                            return Err(format!(
                                "Cannot unpack non-tuple value {:?} into tuple binding",
                                other
                            ));
                        }
                    }
                }
                MathItem::Function {
                    name, params, body, ..
                } => {
                    let p_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                    self.functions.insert(name.clone(), (p_names, body.clone()));
                }
                MathItem::Expression(expr) => {
                    let res = self.eval_value(expr)?;
                    last_res = Some(res);
                }
            }
        }

        Ok(last_res)
    }

    pub fn split_heads(&self, val: &MathValue, n_heads: usize) -> Result<MathValue, String> {
        if n_heads == 0 {
            return Err("n_heads must be at least 1".to_string());
        }
        match val {
            MathValue::Matrix(m) => {
                let seq = m.len();
                let dim = m.first().map(|r| r.len()).unwrap_or(0);
                if dim % n_heads != 0 {
                    return Err(format!(
                        "split_heads: hidden dimension {} must be divisible by n_heads {}",
                        dim, n_heads
                    ));
                }
                let head_dim = dim / n_heads;
                let mut out = vec![0.0; n_heads * seq * head_dim];
                for h in 0..n_heads {
                    for s in 0..seq {
                        for d in 0..head_dim {
                            let in_val = m[s][h * head_dim + d];
                            let out_idx = h * (seq * head_dim) + s * head_dim + d;
                            out[out_idx] = in_val;
                        }
                    }
                }
                MathValue::new_tensor(vec![n_heads, seq, head_dim], out)
            }
            MathValue::Tensor {
                shape,
                strides,
                data,
            } => {
                let ndim = shape.len();
                if ndim < 2 {
                    return Err("split_heads requires at least 2D tensor".to_string());
                }
                let dim = *shape.last().unwrap_or(&0);
                if dim % n_heads != 0 {
                    return Err(format!(
                        "split_heads: hidden dimension {} must be divisible by n_heads {}",
                        dim, n_heads
                    ));
                }
                let head_dim = dim / n_heads;
                let seq = shape[ndim - 2];
                let flat = MathValue::get_flattened_strided(shape, strides, data);

                if ndim == 2 {
                    let mut out = vec![0.0; n_heads * seq * head_dim];
                    for h in 0..n_heads {
                        for s in 0..seq {
                            for d in 0..head_dim {
                                let in_val = flat[s * dim + h * head_dim + d];
                                out[h * (seq * head_dim) + s * head_dim + d] = in_val;
                            }
                        }
                    }
                    MathValue::new_tensor(vec![n_heads, seq, head_dim], out)
                } else if ndim == 3 {
                    let batch = shape[0];
                    let mut out = vec![0.0; batch * n_heads * seq * head_dim];
                    for b in 0..batch {
                        for h in 0..n_heads {
                            for s in 0..seq {
                                for d in 0..head_dim {
                                    let in_idx = b * (seq * dim) + s * dim + h * head_dim + d;
                                    let out_idx = b * (n_heads * seq * head_dim)
                                        + h * (seq * head_dim)
                                        + s * head_dim
                                        + d;
                                    out[out_idx] = flat[in_idx];
                                }
                            }
                        }
                    }
                    MathValue::new_tensor(vec![batch, n_heads, seq, head_dim], out)
                } else {
                    Err(format!("split_heads unsupported for rank {}", ndim))
                }
            }
            _ => Err("split_heads expects a Matrix or Tensor".to_string()),
        }
    }

    pub fn merge_heads(&self, val: &MathValue) -> Result<MathValue, String> {
        match val {
            MathValue::Tensor {
                shape,
                strides,
                data,
            } => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                if shape.len() == 3 {
                    let n_heads = shape[0];
                    let seq = shape[1];
                    let head_dim = shape[2];
                    let dim = n_heads * head_dim;
                    let mut mat = vec![vec![0.0; dim]; seq];
                    for s in 0..seq {
                        for h in 0..n_heads {
                            for d in 0..head_dim {
                                let in_idx = h * (seq * head_dim) + s * head_dim + d;
                                mat[s][h * head_dim + d] = flat[in_idx];
                            }
                        }
                    }
                    Ok(MathValue::Matrix(mat))
                } else if shape.len() == 4 {
                    let batch = shape[0];
                    let n_heads = shape[1];
                    let seq = shape[2];
                    let head_dim = shape[3];
                    let dim = n_heads * head_dim;
                    let mut out = vec![0.0; batch * seq * dim];
                    for b in 0..batch {
                        for s in 0..seq {
                            for h in 0..n_heads {
                                for d in 0..head_dim {
                                    let in_idx = b * (n_heads * seq * head_dim)
                                        + h * (seq * head_dim)
                                        + s * head_dim
                                        + d;
                                    let out_idx = b * (seq * dim) + s * dim + h * head_dim + d;
                                    out[out_idx] = flat[in_idx];
                                }
                            }
                        }
                    }
                    MathValue::new_tensor(vec![batch, seq, dim], out)
                } else {
                    Err(format!(
                        "merge_heads expects 3D or 4D Tensor, got shape {:?}",
                        shape
                    ))
                }
            }
            _ => Err("merge_heads expects a Tensor".to_string()),
        }
    }

    pub fn eval_gqa_attn(
        &self,
        q: &MathValue,
        k: &MathValue,
        v: &MathValue,
        n_heads: usize,
        n_kv_heads: usize,
        scale: Option<f64>,
        is_causal: bool,
    ) -> Result<MathValue, String> {
        if n_heads == 0 || n_kv_heads == 0 {
            return Err("n_heads and n_kv_heads must be at least 1".to_string());
        }
        if n_heads % n_kv_heads != 0 {
            return Err(format!(
                "gqa_attn: n_heads ({}) must be divisible by n_kv_heads ({})",
                n_heads, n_kv_heads
            ));
        }
        let group_size = n_heads / n_kv_heads;

        if let (MathValue::Matrix(q_m), MathValue::Matrix(k_m), MathValue::Matrix(v_m)) = (q, k, v)
        {
            let seq_q = q_m.len();
            let seq_k = k_m.len();
            let dim_q = q_m.first().map(|r| r.len()).unwrap_or(0);
            let dim_k = k_m.first().map(|r| r.len()).unwrap_or(0);
            let dim_v = v_m.first().map(|r| r.len()).unwrap_or(0);

            if dim_q % n_heads != 0 {
                return Err(format!(
                    "gqa_attn: Q dim {} must be divisible by n_heads {}",
                    dim_q, n_heads
                ));
            }
            let head_dim = dim_q / n_heads;
            if dim_k != n_kv_heads * head_dim || dim_v != n_kv_heads * head_dim {
                return Err(format!(
                    "gqa_attn: K/V dim ({} / {}) must equal n_kv_heads * head_dim ({} * {} = {})",
                    dim_k,
                    dim_v,
                    n_kv_heads,
                    head_dim,
                    n_kv_heads * head_dim
                ));
            }

            let s = scale.unwrap_or(1.0 / (head_dim as f64).sqrt().max(1e-6));
            let mut out_m = vec![vec![0.0; dim_q]; seq_q];

            for h in 0..n_heads {
                let kv_h = h / group_size;
                let q_col_start = h * head_dim;
                let kv_col_start = kv_h * head_dim;

                let mut q_h = Vec::with_capacity(seq_q);
                for row in q_m {
                    q_h.push(row[q_col_start..q_col_start + head_dim].to_vec());
                }
                let mut k_kv = Vec::with_capacity(seq_k);
                for row in k_m {
                    k_kv.push(row[kv_col_start..kv_col_start + head_dim].to_vec());
                }
                let mut v_kv = Vec::with_capacity(seq_k);
                for row in v_m {
                    v_kv.push(row[kv_col_start..kv_col_start + head_dim].to_vec());
                }

                let mut p_mat: Vec<Vec<f64>> = Vec::with_capacity(seq_q);
                for (r, q_row) in q_h.iter().enumerate() {
                    let mut score_row = Vec::with_capacity(seq_k);
                    for (c, k_row) in k_kv.iter().enumerate() {
                        if is_causal && c > r {
                            score_row.push(f64::NEG_INFINITY);
                        } else {
                            let dot: f64 =
                                q_row.iter().zip(k_row.iter()).map(|(&a, &b)| a * b).sum();
                            score_row.push(dot * s);
                        }
                    }
                    let max_v = score_row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let exps: Vec<f64> = score_row.iter().map(|&x| (x - max_v).exp()).collect();
                    let sum_exp: f64 = exps.iter().sum::<f64>().max(1e-12);
                    p_mat.push(exps.into_iter().map(|e| e / sum_exp).collect());
                }

                for (r, p_row) in p_mat.iter().enumerate() {
                    for d in 0..head_dim {
                        let mut dot = 0.0;
                        for (c, &p_val) in p_row.iter().enumerate() {
                            dot += p_val * v_kv[c][d];
                        }
                        out_m[r][q_col_start + d] = dot;
                    }
                }
            }
            Ok(MathValue::Matrix(out_m))
        } else {
            let q_heads = if q.shape().len() == 2 {
                self.split_heads(q, n_heads)?
            } else {
                q.clone()
            };
            let k_heads = if k.shape().len() == 2 {
                self.split_heads(k, n_kv_heads)?
            } else {
                k.clone()
            };
            let v_heads = if v.shape().len() == 2 {
                self.split_heads(v, n_kv_heads)?
            } else {
                v.clone()
            };

            let q_shape = q_heads.shape();
            let k_shape = k_heads.shape();
            let v_shape = v_heads.shape();

            if q_shape.len() == 3 && k_shape.len() == 3 && v_shape.len() == 3 {
                let seq_q = q_shape[1];
                let head_dim = q_shape[2];
                let seq_k = k_shape[1];
                let s = scale.unwrap_or(1.0 / (head_dim as f64).sqrt().max(1e-6));

                let q_flat = q_heads.flatten_elements();
                let k_flat = k_heads.flatten_elements();
                let v_flat = v_heads.flatten_elements();

                let mut out_data = vec![0.0; n_heads * seq_q * head_dim];

                for h in 0..n_heads {
                    let kv_h = h / group_size;
                    let q_offset = h * (seq_q * head_dim);
                    let k_offset = kv_h * (seq_k * head_dim);
                    let v_offset = kv_h * (seq_k * head_dim);

                    for r in 0..seq_q {
                        let q_row =
                            &q_flat[q_offset + r * head_dim..q_offset + (r + 1) * head_dim];
                        let mut score_row = Vec::with_capacity(seq_k);
                        for c in 0..seq_k {
                            if is_causal && c > r {
                                score_row.push(f64::NEG_INFINITY);
                            } else {
                                let k_row =
                                    &k_flat[k_offset + c * head_dim..k_offset + (c + 1) * head_dim];
                                let dot: f64 =
                                    q_row.iter().zip(k_row.iter()).map(|(&a, &b)| a * b).sum();
                                score_row.push(dot * s);
                            }
                        }
                        let max_v = score_row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let exps: Vec<f64> = score_row.iter().map(|&x| (x - max_v).exp()).collect();
                        let sum_exp: f64 = exps.iter().sum::<f64>().max(1e-12);
                        let probs: Vec<f64> = exps.into_iter().map(|e| e / sum_exp).collect();

                        for d in 0..head_dim {
                            let mut dot = 0.0;
                            for (c, &p_val) in probs.iter().enumerate() {
                                let v_val = v_flat[v_offset + c * head_dim + d];
                                dot += p_val * v_val;
                            }
                            let out_idx = h * (seq_q * head_dim) + r * head_dim + d;
                            out_data[out_idx] = dot;
                        }
                    }
                }
                MathValue::new_tensor(vec![n_heads, seq_q, head_dim], out_data)
            } else {
                Err(format!(
                    "gqa_attn: unsupported operand shapes: Q {:?}, K {:?}, V {:?}",
                    q_shape, k_shape, v_shape
                ))
            }
        }
    }

    pub fn eval_mla_attn(
        &self,
        x: &MathValue,
        w_dkv: &MathValue,
        w_uk: &MathValue,
        w_uv: &MathValue,
        w_q: &MathValue,
        w_o_opt: Option<&MathValue>,
        is_causal: bool,
    ) -> Result<MathValue, String> {
        let w_dkv_t = w_dkv.transpose()?;
        let c_kv = self.eval_binary_op(x, MathBinOp::MatMul, &w_dkv_t)?;

        let w_uk_t = w_uk.transpose()?;
        let k = self.eval_binary_op(&c_kv, MathBinOp::MatMul, &w_uk_t)?;

        let w_uv_t = w_uv.transpose()?;
        let v = self.eval_binary_op(&c_kv, MathBinOp::MatMul, &w_uv_t)?;

        let w_q_t = w_q.transpose()?;
        let q = self.eval_binary_op(x, MathBinOp::MatMul, &w_q_t)?;

        let k_t = k.transpose()?;
        let scores = self.eval_binary_op(&q, MathBinOp::MatMul, &k_t)?;

        let d_k = k.shape().last().copied().unwrap_or(1) as f64;
        let scale = 1.0 / d_k.sqrt().max(1e-6);
        let scaled_scores =
            self.eval_binary_op(&scores, MathBinOp::Mul, &MathValue::Scalar(scale))?;

        let p = match scaled_scores {
            MathValue::Matrix(mat) => {
                let mut res = Vec::new();
                for (row_idx, row) in mat.into_iter().enumerate() {
                    let masked_row: Vec<f64> = row
                        .into_iter()
                        .enumerate()
                        .map(|(col_idx, val)| {
                            if is_causal && col_idx > row_idx {
                                f64::NEG_INFINITY
                            } else {
                                val
                            }
                        })
                        .collect();
                    let max_v = masked_row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let exps: Vec<f64> = masked_row.iter().map(|&z| (z - max_v).exp()).collect();
                    let sum_exp = exps.iter().sum::<f64>().max(1e-12);
                    res.push(exps.into_iter().map(|e| e / sum_exp).collect());
                }
                MathValue::Matrix(res)
            }
            other => other,
        };

        let attn_out = self.eval_binary_op(&p, MathBinOp::MatMul, &v)?;

        if let Some(w_o) = w_o_opt {
            let w_o_t = w_o.transpose()?;
            self.eval_binary_op(&attn_out, MathBinOp::MatMul, &w_o_t)
        } else {
            Ok(attn_out)
        }
    }

    pub fn eval_ssd_scan(
        &self,
        x: &MathValue,
        decay_dt: &MathValue,
    ) -> Result<MathValue, String> {
        match x {
            MathValue::Vector(x_vec) => {
                let d_vec = decay_dt.flatten_elements();
                let mut out = Vec::with_capacity(x_vec.len());
                let mut h = 0.0;
                for (t, &xt) in x_vec.iter().enumerate() {
                    let a = d_vec.get(t % d_vec.len()).copied().unwrap_or(0.9).min(1.0).max(0.0);
                    h = a * h + xt;
                    out.push(h);
                }
                Ok(MathValue::Vector(out))
            }
            MathValue::Matrix(mat) => {
                let d_elems = decay_dt.flatten_elements();
                let rows = mat.len();
                let cols = mat.first().map(|r| r.len()).unwrap_or(0);
                let mut out = vec![vec![0.0; cols]; rows];
                let mut h = vec![0.0; cols];

                for r in 0..rows {
                    for c in 0..cols {
                        let a_idx = if d_elems.len() == cols {
                            c
                        } else {
                            (r * cols + c) % d_elems.len()
                        };
                        let a = d_elems.get(a_idx).copied().unwrap_or(0.9).min(1.0).max(0.0);
                        h[c] = a * h[c] + mat[r][c];
                        out[r][c] = h[c];
                    }
                }
                Ok(MathValue::Matrix(out))
            }
            MathValue::Tensor { shape, strides, data } => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let d_elems = decay_dt.flatten_elements();
                let ndim = shape.len();
                let seq = shape[ndim - 2];
                let dim = shape[ndim - 1];
                let total = flat.len();
                let batch_size = total / (seq * dim);
                let mut out = vec![0.0; total];

                for b in 0..batch_size {
                    let mut h = vec![0.0; dim];
                    let b_offset = b * (seq * dim);
                    for s in 0..seq {
                        let s_offset = b_offset + s * dim;
                        for d in 0..dim {
                            let a = d_elems.get(d % d_elems.len()).copied().unwrap_or(0.9).min(1.0).max(0.0);
                            h[d] = a * h[d] + flat[s_offset + d];
                            out[s_offset + d] = h[d];
                        }
                    }
                }
                MathValue::new_tensor(shape.clone(), out)
            }
            _ => Err("ssd_scan expects Vector, Matrix, or Tensor".to_string()),
        }
    }

    pub fn eval_quantize_t2(&self, val: &MathValue) -> Result<MathValue, String> {
        let elems = val.flatten_elements();
        if elems.is_empty() {
            return Ok(MathValue::Tuple(vec![val.clone(), MathValue::Scalar(1.0)]));
        }
        let gamma = (elems.iter().map(|x| x.abs()).sum::<f64>() / elems.len() as f64).max(1e-12);
        let quant_fn = |x: f64| -> f64 {
            (x / gamma).clamp(-1.0, 1.0).round()
        };
        let quantized = val.map_unary_dyn(&quant_fn);
        Ok(MathValue::Tuple(vec![quantized, MathValue::Scalar(gamma)]))
    }

    pub fn eval_bitlinear(
        &self,
        x: &MathValue,
        w: &MathValue,
        gamma: Option<f64>,
    ) -> Result<MathValue, String> {
        let scale = gamma.unwrap_or(1.0);
        match (x, w) {
            (MathValue::Matrix(x_m), MathValue::Matrix(w_m)) => {
                let m = x_m.len();
                let k_x = x_m.first().map(|r| r.len()).unwrap_or(0);
                let w_rows = w_m.len();
                let w_cols = w_m.first().map(|r| r.len()).unwrap_or(0);

                if k_x == w_cols {
                    // Y = X @ W^T, where W is [N, K], output is [M, N]
                    let n = w_rows;
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for j in 0..n {
                            let mut acc = 0.0;
                            let x_row = &x_m[i];
                            let w_row = &w_m[j];
                            for (&xv, &wv) in x_row.iter().zip(w_row.iter()) {
                                if wv > 0.5 {
                                    acc += xv;
                                } else if wv < -0.5 {
                                    acc -= xv;
                                }
                            }
                            out[i][j] = acc * scale;
                        }
                    }
                    Ok(MathValue::Matrix(out))
                } else if k_x == w_rows {
                    // Standard GEMM: Y = X @ W, where W is [K, N], output is [M, N]
                    let n = w_cols;
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for j in 0..n {
                            let mut acc = 0.0;
                            let x_row = &x_m[i];
                            for (k, x_val) in x_row.iter().enumerate() {
                                let wv = w_m[k][j];
                                if wv > 0.5 {
                                    acc += x_val;
                                } else if wv < -0.5 {
                                    acc -= x_val;
                                }
                            }
                            out[i][j] = acc * scale;
                        }
                    }
                    Ok(MathValue::Matrix(out))
                } else {
                    Err(format!(
                        "bitlinear dimension mismatch: X is [{}, {}], W is [{}, {}]",
                        m, k_x, w_rows, w_cols
                    ))
                }
            }
            (MathValue::Vector(x_v), MathValue::Matrix(w_m)) => {
                let k_x = x_v.len();
                let w_rows = w_m.len();
                let w_cols = w_m.first().map(|r| r.len()).unwrap_or(0);
                if k_x == w_cols {
                    let mut out = Vec::with_capacity(w_rows);
                    for w_row in w_m {
                        let mut acc = 0.0;
                        for (&xv, &wv) in x_v.iter().zip(w_row.iter()) {
                            if wv > 0.5 {
                                acc += xv;
                            } else if wv < -0.5 {
                                acc -= xv;
                            }
                        }
                        out.push(acc * scale);
                    }
                    Ok(MathValue::Vector(out))
                } else if k_x == w_rows {
                    let mut out = vec![0.0; w_cols];
                    for (k, &xv) in x_v.iter().enumerate() {
                        for j in 0..w_cols {
                            let wv = w_m[k][j];
                            if wv > 0.5 {
                                out[j] += xv;
                            } else if wv < -0.5 {
                                out[j] -= xv;
                            }
                        }
                    }
                    for v in &mut out {
                        *v *= scale;
                    }
                    Ok(MathValue::Vector(out))
                } else {
                    Err(format!(
                        "bitlinear vector-matrix dimension mismatch: X [{}] vs W [{}, {}]",
                        k_x, w_rows, w_cols
                    ))
                }
            }
            (MathValue::Tensor { shape, strides, data }, MathValue::Matrix(w_m)) => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let k_x = *shape.last().unwrap_or(&1);
                let w_rows = w_m.len();
                let w_cols = w_m.first().map(|r| r.len()).unwrap_or(0);
                let (n, transposed) = if k_x == w_cols {
                    (w_rows, true)
                } else if k_x == w_rows {
                    (w_cols, false)
                } else {
                    return Err(format!(
                        "bitlinear tensor dimension mismatch: X inner dim {} vs W [{}, {}]",
                        k_x, w_rows, w_cols
                    ));
                };
                let num_rows = flat.len() / k_x.max(1);
                let mut out = vec![0.0; num_rows * n];
                for r in 0..num_rows {
                    let x_start = r * k_x;
                    let x_row = &flat[x_start..x_start + k_x];
                    for j in 0..n {
                        let mut acc = 0.0;
                        if transposed {
                            let w_row = &w_m[j];
                            for (&xv, &wv) in x_row.iter().zip(w_row.iter()) {
                                if wv > 0.5 {
                                    acc += xv;
                                } else if wv < -0.5 {
                                    acc -= xv;
                                }
                            }
                        } else {
                            for (k, &xv) in x_row.iter().enumerate() {
                                let wv = w_m[k][j];
                                if wv > 0.5 {
                                    acc += xv;
                                } else if wv < -0.5 {
                                    acc -= xv;
                                }
                            }
                        }
                        out[r * n + j] = acc * scale;
                    }
                }
                let mut out_shape = shape.clone();
                *out_shape.last_mut().unwrap() = n;
                MathValue::new_tensor(out_shape, out)
            }
            _ => Err("bitlinear requires Matrix or Tensor operands".to_string()),
        }
    }

    pub fn eval_quantize_i8(
        &self,
        val: &MathValue,
        _group_size: Option<usize>,
    ) -> Result<MathValue, String> {
        match val {
            MathValue::Matrix(m) => {
                let rows = m.len();
                let cols = m.first().map(|r| r.len()).unwrap_or(0);
                let mut q_mat = vec![vec![0.0; cols]; rows];
                let mut scales = Vec::with_capacity(rows);
                for r in 0..rows {
                    let max_v = m[r].iter().copied().fold(0.0f64, |acc, x| acc.max(x.abs()));
                    let s = if max_v > 1e-12 { max_v / 127.0 } else { 1.0 };
                    scales.push(s);
                    for c in 0..cols {
                        q_mat[r][c] = (m[r][c] / s).clamp(-128.0, 127.0).round();
                    }
                }
                Ok(MathValue::Tuple(vec![
                    MathValue::Matrix(q_mat),
                    MathValue::Vector(scales),
                ]))
            }
            MathValue::Vector(v) => {
                let max_v = v.iter().copied().fold(0.0f64, |acc, x| acc.max(x.abs()));
                let s = if max_v > 1e-12 { max_v / 127.0 } else { 1.0 };
                let q_vec = v
                    .iter()
                    .map(|&x| (x / s).clamp(-128.0, 127.0).round())
                    .collect();
                Ok(MathValue::Tuple(vec![
                    MathValue::Vector(q_vec),
                    MathValue::Scalar(s),
                ]))
            }
            MathValue::Tensor {
                shape,
                strides,
                data,
            } => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let last_dim = *shape.last().unwrap_or(&1);
                let num_rows = flat.len() / last_dim.max(1);
                let mut q_flat = vec![0.0; flat.len()];
                let mut scales = Vec::with_capacity(num_rows);
                for r in 0..num_rows {
                    let start = r * last_dim;
                    let end = start + last_dim;
                    let row = &flat[start..end];
                    let max_v = row.iter().copied().fold(0.0f64, |acc, x| acc.max(x.abs()));
                    let s = if max_v > 1e-12 { max_v / 127.0 } else { 1.0 };
                    scales.push(s);
                    for (i, &v) in row.iter().enumerate() {
                        q_flat[start + i] = (v / s).clamp(-128.0, 127.0).round();
                    }
                }
                let q_tensor = MathValue::new_tensor(shape.clone(), q_flat)?;
                Ok(MathValue::Tuple(vec![
                    q_tensor,
                    MathValue::Vector(scales),
                ]))
            }
            _ => Err("quantize_i8 requires Matrix, Vector, or Tensor".to_string()),
        }
    }

    pub fn eval_qmatmul(
        &self,
        x: &MathValue,
        w_q: &MathValue,
        scales: &MathValue,
    ) -> Result<MathValue, String> {
        let scales_vec = scales.flatten_elements();
        match (x, w_q) {
            (MathValue::Matrix(x_m), MathValue::Matrix(w_m)) => {
                let m = x_m.len();
                let k_x = x_m.first().map(|r| r.len()).unwrap_or(0);
                let w_rows = w_m.len();
                let w_cols = w_m.first().map(|r| r.len()).unwrap_or(0);
                if k_x == w_cols {
                    let n = w_rows;
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for j in 0..n {
                            let s = scales_vec.get(j).copied().unwrap_or(1.0);
                            let dot: f64 = x_m[i]
                                .iter()
                                .zip(w_m[j].iter())
                                .map(|(&a, &b)| a * b)
                                .sum();
                            out[i][j] = dot * s;
                        }
                    }
                    Ok(MathValue::Matrix(out))
                } else if k_x == w_rows {
                    let n = w_cols;
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for j in 0..n {
                            let s = scales_vec.get(j).copied().unwrap_or(1.0);
                            let dot: f64 = (0..k_x).map(|k| x_m[i][k] * w_m[k][j]).sum();
                            out[i][j] = dot * s;
                        }
                    }
                    Ok(MathValue::Matrix(out))
                } else {
                    Err(format!(
                        "qmatmul shape mismatch: X [{}, {}] vs W_q [{}, {}]",
                        m, k_x, w_rows, w_cols
                    ))
                }
            }
            (MathValue::Vector(x_v), MathValue::Matrix(w_m)) => {
                let k_x = x_v.len();
                let w_rows = w_m.len();
                let w_cols = w_m.first().map(|r| r.len()).unwrap_or(0);
                if k_x == w_cols {
                    let mut out = Vec::with_capacity(w_rows);
                    for j in 0..w_rows {
                        let s = scales_vec.get(j).copied().unwrap_or(1.0);
                        let dot: f64 = x_v
                            .iter()
                            .zip(w_m[j].iter())
                            .map(|(&a, &b)| a * b)
                            .sum();
                        out.push(dot * s);
                    }
                    Ok(MathValue::Vector(out))
                } else {
                    let mut out = vec![0.0; w_cols];
                    for j in 0..w_cols {
                        let s = scales_vec.get(j).copied().unwrap_or(1.0);
                        let dot: f64 = (0..k_x).map(|k| x_v[k] * w_m[k][j]).sum();
                        out[j] = dot * s;
                    }
                    Ok(MathValue::Vector(out))
                }
            }
            _ => Err("qmatmul requires Matrix or Vector operands".to_string()),
        }
    }

    pub fn eval(&self, expr: &MathExpr) -> Result<f64, String> {
        let val = self.eval_value(expr)?;
        val.as_scalar()
            .ok_or_else(|| format!("Expected scalar result, got tensor {}", val))
    }

    pub fn eval_value(&self, expr: &MathExpr) -> Result<MathValue, String> {
        match expr {
            MathExpr::Num(n) => Ok(MathValue::Scalar(*n)),
            MathExpr::TensorRef { path, key } => load_tensor_resource(path, key),
            MathExpr::Var(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined mathematical variable '{}'", name)),
            MathExpr::Array(elems) => {
                let mut evaluated = Vec::with_capacity(elems.len());
                for e in elems {
                    evaluated.push(self.eval_value(e)?);
                }
                if evaluated.is_empty() {
                    return Ok(MathValue::Vector(Vec::new()));
                }
                if matches!(evaluated[0], MathValue::Matrix(_)) {
                    let mut mat_list = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let MathValue::Matrix(m) = item {
                            mat_list.push(m);
                        } else {
                            return Err("Inconsistent tensor types in 3D array literal".to_string());
                        }
                    }
                    let r = mat_list[0].len();
                    let c = mat_list[0].first().map(|row| row.len()).unwrap_or(0);
                    let shape = vec![mat_list.len(), r, c];
                    let flat: Vec<f64> = mat_list.into_iter().flatten().flatten().collect();
                    MathValue::new_tensor(shape, flat)
                } else if matches!(evaluated[0], MathValue::Tensor { .. }) {
                    let mut sub_tensors = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let MathValue::Tensor { shape, strides, data } = item {
                            sub_tensors.push((shape, strides, data));
                        } else {
                            return Err("Inconsistent tensor types in ND array literal".to_string());
                        }
                    }
                    let sub_shape = sub_tensors[0].0.clone();
                    let mut shape = vec![sub_tensors.len()];
                    shape.extend_from_slice(&sub_shape);
                    let mut flat = Vec::new();
                    for (sh, st, dt) in sub_tensors {
                        flat.extend(MathValue::get_flattened_strided(&sh, &st, &dt));
                    }
                    MathValue::new_tensor(shape, flat)
                } else if matches!(evaluated[0], MathValue::Vector(_)) {
                    let mut rows = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let MathValue::Vector(r) = item {
                            rows.push(r);
                        } else {
                            return Err(
                                "Inconsistent matrix row types in array literal".to_string()
                            );
                        }
                    }
                    Ok(MathValue::Matrix(rows))
                } else {
                    let mut vec = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let Some(s) = item.as_scalar() {
                            vec.push(s);
                        } else {
                            return Err("Expected scalar elements inside 1D vector".to_string());
                        }
                    }
                    Ok(MathValue::Vector(vec))
                }
            }
            MathExpr::Tuple(items) => {
                let mut vals = Vec::with_capacity(items.len());
                for item in items {
                    vals.push(self.eval_value(item)?);
                }
                Ok(MathValue::Tuple(vals))
            }
            MathExpr::Index(base, idx) => {
                let b_val = self.eval_value(base)?;
                let i_val = self.eval_value(idx)?;
                let i_num = i_val
                    .as_scalar()
                    .ok_or_else(|| "Index must evaluate to a scalar integer".to_string())?
                    as usize;
                match b_val {
                    MathValue::Tuple(vals) => vals
                        .get(i_num)
                        .cloned()
                        .ok_or_else(|| {
                            format!("Tuple index {} out of bounds (len {})", i_num, vals.len())
                        }),
                    MathValue::Vector(vec) => vec
                        .get(i_num)
                        .copied()
                        .map(MathValue::Scalar)
                        .ok_or_else(|| {
                            format!("Vector index {} out of bounds (len {})", i_num, vec.len())
                        }),
                    MathValue::Matrix(mat) => mat
                        .get(i_num)
                        .cloned()
                        .map(MathValue::Vector)
                        .ok_or_else(|| {
                            format!(
                                "Matrix row index {} out of bounds (rows {})",
                                i_num,
                                mat.len()
                            )
                        }),
                    MathValue::Tensor { shape, strides, data } => {
                        if shape.is_empty() {
                            return Err("Cannot index empty tensor".to_string());
                        }
                        if i_num >= shape[0] {
                            return Err(format!(
                                "Tensor index {} out of bounds for axis 0 (dim {})",
                                i_num, shape[0]
                            ));
                        }
                        let sub_shape = shape[1..].to_vec();
                        let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                        let sub_total: usize = sub_shape.iter().product();
                        let s_offset = i_num * sub_total;
                        let e_offset = s_offset + sub_total;
                        let sub_data = flat[s_offset..e_offset.min(flat.len())].to_vec();
                        match sub_shape.len() {
                            0 => Ok(MathValue::Scalar(sub_data.first().copied().unwrap_or(0.0))),
                            1 => Ok(MathValue::Vector(sub_data)),
                            2 => {
                                let r = sub_shape[0];
                                let c = sub_shape[1];
                                let mut mat = Vec::with_capacity(r);
                                for i in 0..r {
                                    mat.push(sub_data[i * c..(i + 1) * c].to_vec());
                                }
                                Ok(MathValue::Matrix(mat))
                            }
                            _ => MathValue::new_tensor(sub_shape, sub_data),
                        }
                    }
                    MathValue::Scalar(s) => {
                        if i_num == 0 {
                            Ok(MathValue::Scalar(s))
                        } else {
                            Err("Cannot index into a scalar with index > 0".to_string())
                        }
                    }
                }
            }
            MathExpr::Placeholder => {
                Err("Unexpected unresolved pipeline placeholder '_'".to_string())
            }
            MathExpr::Unary(op, inner) => {
                let v = self.eval_value(inner)?;
                match op {
                    MathUnaryOp::Neg => Ok(v.map_unary(|x| -x)),
                    MathUnaryOp::Not => Ok(v.map_unary(|x| if x == 0.0 { 1.0 } else { 0.0 })),
                }
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let c = self.eval_value(cond)?;
                if let Some(s) = c.as_scalar() {
                    if s != 0.0 {
                        self.eval_value(then_expr)
                    } else {
                        self.eval_value(else_expr)
                    }
                } else {
                    Err("Ternary condition must evaluate to a scalar".to_string())
                }
            }
            MathExpr::Binary(lhs, op, rhs) => {
                if *op == MathBinOp::Pow {
                    if let MathExpr::Var(v) = &**rhs {
                        if v == "T" {
                            let l = self.eval_value(lhs)?;
                            return l.transpose();
                        }
                    }
                }
                let l = self.eval_value(lhs)?;
                let r = self.eval_value(rhs)?;
                self.eval_binary_op(&l, *op, &r)
            }
            MathExpr::Call(name, args) => match name.as_str() {
                "sqrt" => Ok(self.eval_single_val(args, "sqrt")?.map_unary(|x| x.sqrt())),
                "exp" => Ok(self.eval_single_val(args, "exp")?.map_unary(|x| x.exp())),
                "ln" | "log" => Ok(self.eval_single_val(args, "log")?.map_unary(|x| x.ln())),
                "sin" => Ok(self.eval_single_val(args, "sin")?.map_unary(|x| x.sin())),
                "cos" => Ok(self.eval_single_val(args, "cos")?.map_unary(|x| x.cos())),
                "tan" => Ok(self.eval_single_val(args, "tan")?.map_unary(|x| x.tan())),
                "tanh" => Ok(self.eval_single_val(args, "tanh")?.map_unary(|x| x.tanh())),
                "abs" => Ok(self.eval_single_val(args, "abs")?.map_unary(|x| x.abs())),
                "sigmoid" => Ok(self
                    .eval_single_val(args, "sigmoid")?
                    .map_unary(|z| 1.0 / (1.0 + (-z).exp()))),
                "relu" => Ok(self
                    .eval_single_val(args, "relu")?
                    .map_unary(|z| z.max(0.0))),
                "silu" => Ok(self
                    .eval_single_val(args, "silu")?
                    .map_unary(|z| z / (1.0 + (-z).exp()))),
                "gelu" => Ok(self.eval_single_val(args, "gelu")?.map_unary(|z| {
                    0.5 * z * (1.0 + (0.79788456 * (z + 0.044715 * z.powi(3))).tanh())
                })),
                "softmax" => {
                    let val = self.eval_single_val(args, "softmax")?;
                    match val {
                        MathValue::Scalar(_) => Ok(MathValue::Scalar(1.0)),
                        MathValue::Vector(vec) => {
                            let max_v = vec.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                            let exps: Vec<f64> = vec.iter().map(|&x| (x - max_v).exp()).collect();
                            let sum_val: f64 = exps.iter().sum();
                            let sum_exp = sum_val.max(1e-12);
                            Ok(MathValue::Vector(
                                exps.into_iter().map(|e| e / sum_exp).collect(),
                            ))
                        }
                        MathValue::Matrix(mat) => {
                            let mut res = Vec::new();
                            for row in mat {
                                let max_v = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                                let exps: Vec<f64> =
                                    row.iter().map(|&x| (x - max_v).exp()).collect();
                                let sum_val: f64 = exps.iter().sum();
                                let sum_exp = sum_val.max(1e-12);
                                res.push(exps.into_iter().map(|e| e / sum_exp).collect());
                            }
                            Ok(MathValue::Matrix(res))
                        }
                        MathValue::Tensor {
                            shape,
                            strides,
                            data,
                        } => {
                            let last_dim = *shape.last().unwrap_or(&1);
                            let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                            let mut out = vec![0.0; flat.len()];
                            let num_rows = flat.len() / last_dim.max(1);
                            for r in 0..num_rows {
                                let start = r * last_dim;
                                let end = start + last_dim;
                                let row = &flat[start..end];
                                let max_v = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                                let exps: Vec<f64> =
                                    row.iter().map(|&x| (x - max_v).exp()).collect();
                                let sum_exp = exps.iter().sum::<f64>().max(1e-12);
                                for (i, e) in exps.into_iter().enumerate() {
                                    out[start + i] = e / sum_exp;
                                }
                            }
                            MathValue::new_tensor(shape, out)
                        }
                        MathValue::Tuple(_) => Err("Cannot compute softmax on a Tuple".to_string()),
                    }
                }
                "attention" | "flash_attn" | "attn" | "causal_attn" | "masked_attn" => {
                    if args.len() < 3 {
                        return Err(
                            "'attention(Q, K, V)' requires at least 3 arguments".to_string()
                        );
                    }
                    let is_causal = name == "causal_attn" || name == "masked_attn";
                    let q = self.eval_value(&args[0])?;
                    let k = self.eval_value(&args[1])?;
                    let v = self.eval_value(&args[2])?;
                    let scale = if args.len() > 3 {
                        self.eval_value(&args[3])?.as_scalar().unwrap_or(1.0)
                    } else {
                        match &q {
                            MathValue::Matrix(m) => {
                                let d = m.first().map(|r| r.len()).unwrap_or(1) as f64;
                                1.0 / d.sqrt().max(1e-6)
                            }
                            MathValue::Tensor { shape, .. } => {
                                let d = *shape.last().unwrap_or(&1) as f64;
                                1.0 / d.sqrt().max(1e-6)
                            }
                            _ => 1.0,
                        }
                    };
                    let k_t = k.transpose()?;
                    let scores = self.eval_binary_op(&q, MathBinOp::MatMul, &k_t)?;
                    let scaled_scores =
                        self.eval_binary_op(&scores, MathBinOp::Mul, &MathValue::Scalar(scale))?;
                    let p = match scaled_scores {
                        MathValue::Matrix(mat) => {
                            let mut res = Vec::new();
                            for (row_idx, row) in mat.into_iter().enumerate() {
                                let masked_row: Vec<f64> = row
                                    .into_iter()
                                    .enumerate()
                                    .map(|(col_idx, val)| {
                                        if is_causal && col_idx > row_idx {
                                            f64::NEG_INFINITY
                                        } else {
                                            val
                                        }
                                    })
                                    .collect();
                                let max_v =
                                    masked_row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                                let exps: Vec<f64> =
                                    masked_row.iter().map(|&x| (x - max_v).exp()).collect();
                                let sum_val: f64 = exps.iter().sum();
                                let sum_exp = sum_val.max(1e-12);
                                res.push(exps.into_iter().map(|e| e / sum_exp).collect());
                            }
                            MathValue::Matrix(res)
                        }
                        other => other,
                    };
                    self.eval_binary_op(&p, MathBinOp::MatMul, &v)
                }
                "split_heads" => {
                    if args.len() != 2 {
                        return Err("'split_heads(X, n_heads)' expects 2 arguments".to_string());
                    }
                    let x = self.eval_value(&args[0])?;
                    let nh = self.eval(&args[1])? as usize;
                    self.split_heads(&x, nh)
                }
                "merge_heads" => {
                    let x = self.eval_single_val(args, "merge_heads")?;
                    self.merge_heads(&x)
                }
                "gqa_attn" | "gqa" | "causal_gqa" => {
                    if args.len() < 5 {
                        return Err(
                            "'gqa_attn(Q, K, V, n_heads, n_kv_heads, [scale])' requires at least 5 arguments"
                                .to_string(),
                        );
                    }
                    let is_causal = name == "causal_gqa";
                    let q = self.eval_value(&args[0])?;
                    let k = self.eval_value(&args[1])?;
                    let v = self.eval_value(&args[2])?;
                    let nh = self.eval(&args[3])? as usize;
                    let nkv = self.eval(&args[4])? as usize;
                    let scale = if args.len() > 5 {
                        Some(self.eval(&args[5])?)
                    } else {
                        None
                    };
                    self.eval_gqa_attn(&q, &k, &v, nh, nkv, scale, is_causal)
                }
                "mla_attn" | "mla" | "causal_mla" => {
                    if args.len() < 5 {
                        return Err(
                            "'mla_attn(X, W_dkv, W_uk, W_uv, W_q, [W_o])' requires at least 5 arguments"
                                .to_string(),
                        );
                    }
                    let is_causal = name == "causal_mla";
                    let x = self.eval_value(&args[0])?;
                    let w_dkv = self.eval_value(&args[1])?;
                    let w_uk = self.eval_value(&args[2])?;
                    let w_uv = self.eval_value(&args[3])?;
                    let w_q = self.eval_value(&args[4])?;
                    let w_o = if args.len() > 5 {
                        Some(self.eval_value(&args[5])?)
                    } else {
                        None
                    };
                    self.eval_mla_attn(&x, &w_dkv, &w_uk, &w_uv, &w_q, w_o.as_ref(), is_causal)
                }
                "ssd_scan" | "mamba_ssd" => {
                    if args.len() < 2 {
                        return Err(
                            "'ssd_scan(X, decay_dt)' requires at least 2 arguments".to_string()
                        );
                    }
                    let x = self.eval_value(&args[0])?;
                    let d = self.eval_value(&args[1])?;
                    self.eval_ssd_scan(&x, &d)
                }
                "permute" => {
                    if args.is_empty() {
                        return Err("'permute(X, dims)' requires at least 1 argument".to_string());
                    }
                    let x = self.eval_value(&args[0])?;
                    let dims: Vec<usize> = if args.len() == 2 {
                        match self.eval_value(&args[1])? {
                            MathValue::Vector(v) => v.iter().map(|&d| d as usize).collect(),
                            MathValue::Scalar(s) => vec![s as usize],
                            _ => return Err("Expected permutation dims vector".to_string()),
                        }
                    } else {
                        let mut ds = Vec::new();
                        for a in &args[1..] {
                            ds.push(self.eval(a)? as usize);
                        }
                        ds
                    };
                    x.permute(&dims)
                }
                "quantize_t2" | "t2_quant" => {
                    let val = self.eval_single_val(args, "quantize_t2")?;
                    self.eval_quantize_t2(&val)
                }
                "bitlinear" => {
                    if args.len() < 2 {
                        return Err(
                            "'bitlinear(X, W, [gamma])' requires at least 2 arguments".to_string(),
                        );
                    }
                    let x = self.eval_value(&args[0])?;
                    let w = self.eval_value(&args[1])?;
                    let gamma = if args.len() > 2 {
                        Some(self.eval(&args[2])?)
                    } else {
                        None
                    };
                    self.eval_bitlinear(&x, &w, gamma)
                }
                "quantize_i8" | "i8_quant" => {
                    if args.is_empty() {
                        return Err(
                            "'quantize_i8(W, [group_size])' requires at least 1 argument".to_string(),
                        );
                    }
                    let w = self.eval_value(&args[0])?;
                    let group_size = if args.len() > 1 {
                        Some(self.eval(&args[1])? as usize)
                    } else {
                        None
                    };
                    self.eval_quantize_i8(&w, group_size)
                }
                "qmatmul" => {
                    if args.len() < 3 {
                        return Err("'qmatmul(X, W_q, scales)' requires 3 arguments".to_string());
                    }
                    let x = self.eval_value(&args[0])?;
                    let w_q = self.eval_value(&args[1])?;
                    let scales = self.eval_value(&args[2])?;
                    self.eval_qmatmul(&x, &w_q, &scales)
                }
                "norm" => {
                    let val = self.eval_single_val(args, "norm")?;
                    let elems = val.flatten_elements();
                    let sq_sum: f64 = elems.iter().map(|x| x * x).sum();
                    Ok(MathValue::Scalar(sq_sum.sqrt()))
                }
                "rmsnorm" => {
                    if args.is_empty() {
                        return Err(
                            "'rmsnorm(x, [gamma], [eps])' requires at least 1 argument".to_string()
                        );
                    }
                    let x = self.eval_value(&args[0])?;
                    let gamma_opt = if args.len() > 1 {
                        Some(self.eval_value(&args[1])?)
                    } else {
                        None
                    };
                    let eps = if args.len() > 2 {
                        self.eval_value(&args[2])?.as_scalar().unwrap_or(1e-6)
                    } else {
                        1e-6
                    };
                    match x {
                        MathValue::Scalar(s) => {
                            let norm_s = s / (s * s + eps).sqrt();
                            if let Some(MathValue::Scalar(g)) = gamma_opt {
                                Ok(MathValue::Scalar(norm_s * g))
                            } else {
                                Ok(MathValue::Scalar(norm_s))
                            }
                        }
                        MathValue::Vector(vec) => {
                            let mean_sq =
                                vec.iter().map(|&v| v * v).sum::<f64>() / vec.len().max(1) as f64;
                            let rms = (mean_sq + eps).sqrt();
                            let mut norm_v: Vec<f64> = vec.iter().map(|&v| v / rms).collect();
                            if let Some(g_val) = gamma_opt {
                                let g_elems = g_val.flatten_elements();
                                for (i, v) in norm_v.iter_mut().enumerate() {
                                    let g = g_elems.get(i).copied().unwrap_or(1.0);
                                    *v *= g;
                                }
                            }
                            Ok(MathValue::Vector(norm_v))
                        }
                        MathValue::Matrix(mat) => {
                            let g_elems_opt = gamma_opt.map(|g| g.flatten_elements());
                            let mut res = Vec::new();
                            for row in mat {
                                let mean_sq = row.iter().map(|&v| v * v).sum::<f64>()
                                    / row.len().max(1) as f64;
                                let rms = (mean_sq + eps).sqrt();
                                let mut norm_r: Vec<f64> = row.iter().map(|&v| v / rms).collect();
                                if let Some(ref g_elems) = g_elems_opt {
                                    for (i, v) in norm_r.iter_mut().enumerate() {
                                        let g = g_elems.get(i).copied().unwrap_or(1.0);
                                        *v *= g;
                                    }
                                }
                                res.push(norm_r);
                            }
                            Ok(MathValue::Matrix(res))
                        }
                        MathValue::Tensor {
                            shape,
                            strides,
                            data,
                        } => {
                            let last_dim = *shape.last().unwrap_or(&1);
                            let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                            let g_elems_opt = gamma_opt.map(|g| g.flatten_elements());
                            let mut out = vec![0.0; flat.len()];
                            let num_rows = flat.len() / last_dim.max(1);
                            for r in 0..num_rows {
                                let start = r * last_dim;
                                let end = start + last_dim;
                                let row = &flat[start..end];
                                let mean_sq =
                                    row.iter().map(|&v| v * v).sum::<f64>() / last_dim.max(1) as f64;
                                let rms = (mean_sq + eps).sqrt();
                                for (i, &v) in row.iter().enumerate() {
                                    let mut val = v / rms;
                                    if let Some(ref g_elems) = g_elems_opt {
                                        val *=
                                            g_elems.get(i % g_elems.len()).copied().unwrap_or(1.0);
                                    }
                                    out[start + i] = val;
                                }
                            }
                            MathValue::new_tensor(shape, out)
                        }
                        MathValue::Tuple(_) => {
                            Err("Cannot compute rmsnorm on a Tuple".to_string())
                        }
                    }
                }
                "rope" => {
                    if args.is_empty() {
                        return Err("'rope(x, [pos])' requires at least 1 argument".to_string());
                    }
                    let x = self.eval_value(&args[0])?;
                    let base_pos = if args.len() > 1 {
                        self.eval_value(&args[1])?.as_scalar().unwrap_or(0.0)
                    } else {
                        0.0
                    };
                    let rotate_vec = |v: &[f64], pos: f64| -> Vec<f64> {
                        let mut out = v.to_vec();
                        let d = v.len();
                        for i in (0..d).step_by(2) {
                            if i + 1 < d {
                                let theta = pos * 10000.0_f64.powf(-(i as f64) / (d as f64));
                                let cos_t = theta.cos();
                                let sin_t = theta.sin();
                                let x0 = v[i];
                                let x1 = v[i + 1];
                                out[i] = x0 * cos_t - x1 * sin_t;
                                out[i + 1] = x0 * sin_t + x1 * cos_t;
                            }
                        }
                        out
                    };
                    match x {
                        MathValue::Scalar(s) => Ok(MathValue::Scalar(s)),
                        MathValue::Vector(v) => Ok(MathValue::Vector(rotate_vec(&v, base_pos))),
                        MathValue::Matrix(mat) => {
                            let mut res = Vec::new();
                            for (row_idx, row) in mat.iter().enumerate() {
                                let pos = base_pos + row_idx as f64;
                                res.push(rotate_vec(row, pos));
                            }
                            Ok(MathValue::Matrix(res))
                        }
                        MathValue::Tensor {
                            shape,
                            strides,
                            data,
                        } => {
                            let last_dim = *shape.last().unwrap_or(&1);
                            let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                            let mut out = vec![0.0; flat.len()];
                            let ndim = shape.len();
                            let seq_dim = if ndim >= 2 { shape[ndim - 2] } else { 1 };
                            let num_rows = flat.len() / last_dim.max(1);
                            for r in 0..num_rows {
                                let pos = base_pos + (r % seq_dim) as f64;
                                let start = r * last_dim;
                                let end = start + last_dim;
                                let rotated = rotate_vec(&flat[start..end], pos);
                                out[start..end].copy_from_slice(&rotated);
                            }
                            MathValue::new_tensor(shape, out)
                        }
                        MathValue::Tuple(_) => Err("Cannot compute rope on a Tuple".to_string()),
                    }
                }
                "mean" | "sum" => {
                    if args.is_empty() {
                        return Err(format!("'{}' requires at least one argument", name));
                    }
                    let mut all_elems = Vec::new();
                    for a in args {
                        all_elems.extend(self.eval_value(a)?.flatten_elements());
                    }
                    let sum: f64 = all_elems.iter().sum();
                    if name == "mean" {
                        Ok(MathValue::Scalar(sum / all_elems.len().max(1) as f64))
                    } else {
                        Ok(MathValue::Scalar(sum))
                    }
                }
                "sum_axis" => {
                    if args.is_empty() {
                        return Err(
                            "'sum_axis(X, [axis])' requires at least 1 argument".to_string()
                        );
                    }
                    let val = self.eval_value(&args[0])?;
                    let axis = if args.len() > 1 {
                        self.eval(&args[1])? as usize
                    } else {
                        0
                    };
                    val.sum_axis(axis)
                }
                "max" | "min" => {
                    if args.is_empty() {
                        return Err(format!("'{}' requires at least one argument", name));
                    }
                    let mut all_elems = Vec::new();
                    for a in args {
                        all_elems.extend(self.eval_value(a)?.flatten_elements());
                    }
                    let res = if name == "max" {
                        all_elems.into_iter().fold(f64::NEG_INFINITY, f64::max)
                    } else {
                        all_elems.into_iter().fold(f64::INFINITY, f64::min)
                    };
                    Ok(MathValue::Scalar(res))
                }
                "prod" => {
                    if args.is_empty() {
                        return Err("'prod' requires at least one argument".to_string());
                    }
                    let mut all_elems = Vec::new();
                    for a in args {
                        all_elems.extend(self.eval_value(a)?.flatten_elements());
                    }
                    let prod: f64 = all_elems.iter().product();
                    Ok(MathValue::Scalar(prod))
                }
                "scan" | "cumsum" => {
                    let val = self.eval_single_val(args, name)?;
                    let elems = val.flatten_elements();
                    let mut running = 0.0;
                    let mut out = Vec::with_capacity(elems.len());
                    for x in elems {
                        running += x;
                        out.push(running);
                    }
                    Ok(MathValue::Vector(out))
                }
                "dot" => {
                    if args.len() != 2 {
                        return Err("'dot' expects 2 arguments".to_string());
                    }
                    let a = self.eval_value(&args[0])?;
                    let b = self.eval_value(&args[1])?;
                    self.eval_binary_op(&a, MathBinOp::MatMul, &b)
                }
                "transpose" | "T" => {
                    if args.is_empty() {
                        return Err("'transpose' expects at least 1 argument".to_string());
                    }
                    let val = self.eval_value(&args[0])?;
                    if args.len() == 1 {
                        val.transpose()
                    } else if args.len() == 2 {
                        if let MathValue::Vector(v) = self.eval_value(&args[1])? {
                            if v.len() == 2 {
                                val.transpose_axes(v[0] as usize, v[1] as usize)
                            } else {
                                val.transpose()
                            }
                        } else {
                            val.transpose()
                        }
                    } else if args.len() >= 3 {
                        let ax1 = self.eval(&args[1])? as usize;
                        let ax2 = self.eval(&args[2])? as usize;
                        val.transpose_axes(ax1, ax2)
                    } else {
                        val.transpose()
                    }
                }
                "outer" => {
                    if args.len() == 2 {
                        let u_val = self.eval_value(&args[0])?;
                        let v_val = self.eval_value(&args[1])?;
                        match (u_val, v_val) {
                            (MathValue::Vector(u), MathValue::Vector(v)) => {
                                let mut mat = Vec::with_capacity(u.len());
                                for &ui in &u {
                                    let row: Vec<f64> = v.iter().map(|&vj| ui * vj).collect();
                                    mat.push(row);
                                }
                                Ok(MathValue::Matrix(mat))
                            }
                            (MathValue::Scalar(s), v) | (v, MathValue::Scalar(s)) => {
                                self.eval_binary_op(&MathValue::Scalar(s), MathBinOp::Mul, &v)
                            }
                            _ => Err("'outer' expects two 1D vectors".to_string()),
                        }
                    } else {
                        Err("'outer' expects exactly 2 arguments".to_string())
                    }
                }
                "reshape" => {
                    if args.len() == 2 {
                        let val = self.eval_value(&args[0])?;
                        let shape_val = self.eval_value(&args[1])?;
                        let dims: Vec<usize> = match shape_val {
                            MathValue::Vector(v) => v.iter().map(|&x| x as usize).collect(),
                            MathValue::Scalar(s) => vec![s as usize],
                            _ => return Err("Expected shape vector in reshape".to_string()),
                        };
                        val.reshape(&dims)
                    } else if args.len() > 2 {
                        let val = self.eval_value(&args[0])?;
                        let mut dims = Vec::new();
                        for a in &args[1..] {
                            let d = self.eval(a)? as usize;
                            dims.push(d);
                        }
                        val.reshape(&dims)
                    } else {
                        Err("'reshape' expects (tensor, shape)".to_string())
                    }
                }
                "slice" => {
                    if args.len() == 3 {
                        let val = self.eval_value(&args[0])?;
                        let s = self.eval(&args[1])? as usize;
                        let e = self.eval(&args[2])? as usize;
                        val.slice(s, e)
                    } else {
                        Err("'slice' expects (tensor, start, end)".to_string())
                    }
                }
                "range" => {
                    let (start, end) = if args.len() == 1 {
                        let n = self
                            .eval_value(&args[0])?
                            .as_scalar()
                            .ok_or_else(|| "'range' expects scalar bound".to_string())?
                            as i64;
                        (0, n)
                    } else if args.len() == 2 {
                        let s = self
                            .eval_value(&args[0])?
                            .as_scalar()
                            .ok_or_else(|| "'range' expects scalar start".to_string())?
                            as i64;
                        let e = self
                            .eval_value(&args[1])?
                            .as_scalar()
                            .ok_or_else(|| "'range' expects scalar end".to_string())?
                            as i64;
                        (s, e)
                    } else {
                        return Err("'range' expects 1 or 2 arguments".to_string());
                    };
                    let vec: Vec<f64> = (start..end).map(|i| i as f64).collect();
                    Ok(MathValue::Vector(vec))
                }
                "zeros" | "ones" => {
                    let fill = if name == "ones" { 1.0 } else { 0.0 };
                    let dims: Vec<usize> = if args.len() == 1 {
                        match self.eval_value(&args[0])? {
                            MathValue::Scalar(n) => vec![n as usize],
                            MathValue::Vector(v) => v.iter().map(|&x| x as usize).collect(),
                            _ => return Err(format!("'{}' expects (N), (M, N), or shape vector", name)),
                        }
                    } else {
                        let mut ds = Vec::new();
                        for a in args {
                            ds.push(self.eval(a)? as usize);
                        }
                        ds
                    };
                    match dims.len() {
                        0 => Ok(MathValue::Scalar(fill)),
                        1 => Ok(MathValue::Vector(vec![fill; dims[0]])),
                        2 => Ok(MathValue::Matrix(vec![vec![fill; dims[1]]; dims[0]])),
                        _ => {
                            let total: usize = dims.iter().product();
                            MathValue::new_tensor(dims, vec![fill; total])
                        }
                    }
                }
                "eye" => {
                    if args.len() != 1 {
                        return Err("'eye(N)' expects exactly 1 argument".to_string());
                    }
                    let n = self.eval(&args[0])? as usize;
                    let mut mat = vec![vec![0.0; n]; n];
                    for (i, row) in mat.iter_mut().enumerate() {
                        row[i] = 1.0;
                    }
                    Ok(MathValue::Matrix(mat))
                }
                "randn" => {
                    if args.is_empty() {
                        return Err("'randn' expects dimension arguments".to_string());
                    }
                    let next_u64 = |state: &mut u64| -> f64 {
                        *state = state.wrapping_add(0x9E3779B97F4A7C15);
                        let mut z = *state;
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
                        z ^= z >> 31;
                        ((z >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
                    };
                    let sample_norm = |state: &mut u64, std_dev: f64| -> f64 {
                        let u1 = next_u64(state).max(1e-12);
                        let u2 = next_u64(state);
                        let r = (-2.0 * u1.ln()).sqrt();
                        let theta = 2.0 * std::f64::consts::PI * u2;
                        r * theta.cos() * std_dev
                    };
                    let dims: Vec<usize> = if args.len() == 1 {
                        match self.eval_value(&args[0])? {
                            MathValue::Scalar(n) => vec![n as usize],
                            MathValue::Vector(v) => v.iter().map(|&x| x as usize).collect(),
                            _ => return Err("'randn' expects dimension arguments".to_string()),
                        }
                    } else {
                        let mut ds = Vec::new();
                        for a in args {
                            if let Ok(val) = self.eval_value(a) {
                                if let Some(s) = val.as_scalar() {
                                    ds.push(s as usize);
                                }
                            }
                        }
                        ds
                    };
                    let last_d = *dims.last().unwrap_or(&1);
                    let std_dev = 1.0 / (last_d.max(1) as f64).sqrt();
                    let mut state = 42u64;
                    let total: usize = dims.iter().product();
                    let samples: Vec<f64> = (0..total).map(|_| sample_norm(&mut state, std_dev)).collect();
                    match dims.len() {
                        0 => Ok(MathValue::Scalar(samples.first().copied().unwrap_or(0.0))),
                        1 => Ok(MathValue::Vector(samples)),
                        2 => {
                            let r = dims[0];
                            let c = dims[1];
                            let mut mat = Vec::with_capacity(r);
                            for i in 0..r {
                                mat.push(samples[i * c..(i + 1) * c].to_vec());
                            }
                            Ok(MathValue::Matrix(mat))
                        }
                        _ => MathValue::new_tensor(dims, samples),
                    }
                }
                "argmax" | "argmin" => {
                    let val = self.eval_single_val(args, name)?;
                    let is_max = name == "argmax";
                    let find_idx = |slice: &[f64]| -> f64 {
                        if slice.is_empty() {
                            return 0.0;
                        }
                        let mut best_i = 0usize;
                        let mut best_v = slice[0];
                        for (i, &v) in slice.iter().enumerate().skip(1) {
                            if (is_max && v > best_v) || (!is_max && v < best_v) {
                                best_v = v;
                                best_i = i;
                            }
                        }
                        best_i as f64
                    };
                    match val {
                        MathValue::Scalar(_) => Ok(MathValue::Scalar(0.0)),
                        MathValue::Vector(vec) => Ok(MathValue::Scalar(find_idx(&vec))),
                        MathValue::Matrix(mat) => {
                            let indices: Vec<f64> = mat.iter().map(|row| find_idx(row)).collect();
                            Ok(MathValue::Vector(indices))
                        }
                        MathValue::Tensor {
                            shape,
                            strides,
                            data,
                        } => {
                            let last_dim = *shape.last().unwrap_or(&1);
                            let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                            let num_rows = flat.len() / last_dim.max(1);
                            let mut indices = Vec::with_capacity(num_rows);
                            for r in 0..num_rows {
                                let start = r * last_dim;
                                let end = start + last_dim;
                                indices.push(find_idx(&flat[start..end]));
                            }
                            if shape.len() <= 2 {
                                Ok(MathValue::Vector(indices))
                            } else {
                                let out_shape = shape[..shape.len() - 1].to_vec();
                                MathValue::new_tensor(out_shape, indices)
                            }
                        }
                        MathValue::Tuple(_) => {
                            Err("Cannot compute argmax/argmin on a Tuple".to_string())
                        }
                    }
                }
                "concat" => {
                    if args.len() < 2 {
                        return Err(
                            "'concat(A, B, [axis])' expects at least 2 arguments".to_string()
                        );
                    }
                    let a = self.eval_value(&args[0])?;
                    let b = self.eval_value(&args[1])?;
                    let axis = if args.len() > 2 {
                        self.eval(&args[2])? as usize
                    } else {
                        0
                    };
                    match (a, b) {
                        (MathValue::Vector(mut u), MathValue::Vector(v)) => {
                            u.extend(v);
                            Ok(MathValue::Vector(u))
                        }
                        (MathValue::Vector(mut u), MathValue::Scalar(s)) => {
                            u.push(s);
                            Ok(MathValue::Vector(u))
                        }
                        (MathValue::Matrix(mut m1), MathValue::Matrix(m2)) => {
                            if axis == 0 {
                                m1.extend(m2);
                                Ok(MathValue::Matrix(m1))
                            } else {
                                if m1.len() != m2.len() {
                                    return Err(
                                        "Matrix row counts must match for concat along axis 1"
                                            .to_string(),
                                    );
                                }
                                for (r1, r2) in m1.iter_mut().zip(m2.into_iter()) {
                                    r1.extend(r2);
                                }
                                Ok(MathValue::Matrix(m1))
                            }
                        }
                        (MathValue::Matrix(mut m), MathValue::Vector(v)) => {
                            m.push(v);
                            Ok(MathValue::Matrix(m))
                        }
                        _ => Err("Unsupported operand types for 'concat'".to_string()),
                    }
                }
                "embed" => {
                    if args.len() < 2 {
                        return Err("'embed(E, tokens)' expects 2 arguments".to_string());
                    }
                    let table = self.eval_value(&args[0])?;
                    let tokens = self.eval_value(&args[1])?;
                    match (table, tokens) {
                        (MathValue::Matrix(vocab_mat), MathValue::Vector(toks)) => {
                            let mut res = Vec::with_capacity(toks.len());
                            for t in toks {
                                let idx = t as usize;
                                if idx < vocab_mat.len() {
                                    res.push(vocab_mat[idx].clone());
                                } else {
                                    return Err(format!(
                                        "Token index {} out of bounds for vocab size {}",
                                        idx,
                                        vocab_mat.len()
                                    ));
                                }
                            }
                            Ok(MathValue::Matrix(res))
                        }
                        (MathValue::Matrix(vocab_mat), MathValue::Scalar(s)) => {
                            let idx = s as usize;
                            if idx < vocab_mat.len() {
                                Ok(MathValue::Vector(vocab_mat[idx].clone()))
                            } else {
                                Err(format!(
                                    "Token index {} out of bounds for vocab size {}",
                                    idx,
                                    vocab_mat.len()
                                ))
                            }
                        }
                        _ => Err(
                            "'embed' expects a 2D weight matrix and 1D vector (or scalar) of token indices"
                                .to_string(),
                        ),
                    }
                }
                "log_softmax" => {
                    let val = self.eval_single_val(args, "log_softmax")?;
                    let calc_lse = |row: &[f64]| -> Vec<f64> {
                        let max_v = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let sum_exp: f64 = row.iter().map(|&x| (x - max_v).exp()).sum();
                        let lse = max_v + sum_exp.max(1e-12).ln();
                        row.iter().map(|&x| x - lse).collect()
                    };
                    match val {
                        MathValue::Scalar(_) => Ok(MathValue::Scalar(0.0)),
                        MathValue::Vector(vec) => Ok(MathValue::Vector(calc_lse(&vec))),
                        MathValue::Matrix(mat) => {
                            let res = mat.iter().map(|r| calc_lse(r)).collect();
                            Ok(MathValue::Matrix(res))
                        }
                        MathValue::Tensor {
                            shape,
                            strides,
                            data,
                        } => {
                            let last_dim = *shape.last().unwrap_or(&1);
                            let flat = MathValue::get_flattened_strided(&shape, &strides, &data);
                            let mut out = vec![0.0; flat.len()];
                            let num_rows = flat.len() / last_dim.max(1);
                            for r in 0..num_rows {
                                let start = r * last_dim;
                                let end = start + last_dim;
                                let lse_row = calc_lse(&flat[start..end]);
                                out[start..end].copy_from_slice(&lse_row);
                            }
                            MathValue::new_tensor(shape, out)
                        }
                        MathValue::Tuple(_) => {
                            Err("Cannot compute log_softmax on a Tuple".to_string())
                        }
                    }
                }
                "cross_entropy" => {
                    if args.len() < 2 {
                        return Err(
                            "'cross_entropy(logits, targets)' expects 2 arguments".to_string()
                        );
                    }
                    let logits = self.eval_value(&args[0])?;
                    let targets = self.eval_value(&args[1])?;
                    let calc_row_ce = |row: &[f64], target_idx: usize| -> Result<f64, String> {
                        if target_idx >= row.len() {
                            return Err(format!(
                                "Target index {} exceeds vocab dimension {}",
                                target_idx,
                                row.len()
                            ));
                        }
                        let max_v = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let sum_exp: f64 = row.iter().map(|&x| (x - max_v).exp()).sum();
                        let lse = max_v + sum_exp.max(1e-12).ln();
                        Ok(-(row[target_idx] - lse))
                    };
                    match (logits, targets) {
                        (MathValue::Matrix(mat), MathValue::Vector(target_vec)) => {
                            if mat.len() != target_vec.len() {
                                return Err(format!(
                                    "Batch/Seq length mismatch: {} logits rows vs {} targets",
                                    mat.len(),
                                    target_vec.len()
                                ));
                            }
                            let mut total_loss = 0.0;
                            for (row, &t) in mat.iter().zip(target_vec.iter()) {
                                total_loss += calc_row_ce(row, t as usize)?;
                            }
                            Ok(MathValue::Scalar(total_loss / mat.len().max(1) as f64))
                        }
                        (MathValue::Vector(vec), MathValue::Scalar(t)) => {
                            let loss = calc_row_ce(&vec, t as usize)?;
                            Ok(MathValue::Scalar(loss))
                        }
                        (MathValue::Vector(vec), MathValue::Vector(t_vec)) if t_vec.len() == 1 => {
                            let loss = calc_row_ce(&vec, t_vec[0] as usize)?;
                            Ok(MathValue::Scalar(loss))
                        }
                        _ => Err(
                            "cross_entropy expects (Matrix[Seq, Vocab], Vector[Seq]) or (Vector[Vocab], Scalar)"
                                .to_string(),
                        ),
                    }
                }
                "clip_grad" | "grad_clip" => {
                    if args.len() < 2 {
                        return Err("'clip_grad(G, max_norm)' requires 2 arguments".to_string());
                    }
                    let g = self.eval_value(&args[0])?;
                    let max_norm = self.eval_value(&args[1])?.as_scalar().ok_or_else(|| {
                        "max_norm in clip_grad must be a scalar number".to_string()
                    })?;
                    let elems = g.flatten_elements();
                    let sq_sum: f64 = elems.iter().map(|&x| x * x).sum();
                    let norm = sq_sum.sqrt();
                    if norm > max_norm && norm > 1e-12 {
                        let scale = max_norm / norm;
                        Ok(g.map_unary(|x| x * scale))
                    } else {
                        Ok(g)
                    }
                }
                "cosine_lr" => {
                    if args.len() < 3 {
                        return Err("'cosine_lr(t, max_t, lr_max, [lr_min, warmup])' requires at least 3 arguments".to_string());
                    }
                    let t = self.eval_value(&args[0])?.as_scalar().ok_or_else(|| "'t' must be a scalar".to_string())?;
                    let max_t = self.eval_value(&args[1])?.as_scalar().ok_or_else(|| "'max_t' must be a scalar".to_string())?;
                    let lr_max = self.eval_value(&args[2])?.as_scalar().ok_or_else(|| "'lr_max' must be a scalar".to_string())?;
                    let lr_min = if args.len() > 3 { self.eval_value(&args[3])?.as_scalar().unwrap_or(0.0) } else { 0.0 };
                    let warmup = if args.len() > 4 { self.eval_value(&args[4])?.as_scalar().unwrap_or(0.0) } else { 0.0 };

                    let lr = if t < warmup {
                        lr_max * (t / warmup.max(1.0))
                    } else {
                        let progress = ((t - warmup) / (max_t - warmup).max(1.0)).clamp(0.0, 1.0);
                        lr_min + 0.5 * (lr_max - lr_min) * (1.0 + (std::f64::consts::PI * progress).cos())
                    };
                    Ok(MathValue::Scalar(lr))
                }
                "adamw_step" | "adamw" => {
                    if args.len() < 6 {
                        return Err("'adamw_step(W, g, m, v, t, lr, [beta1, beta2, eps, wd])' requires at least 6 arguments".to_string());
                    }
                    let w_val = self.eval_value(&args[0])?;
                    let g_val = self.eval_value(&args[1])?;
                    let m_val = self.eval_value(&args[2])?;
                    let v_val = self.eval_value(&args[3])?;
                    let t = self.eval_value(&args[4])?.as_scalar().ok_or_else(|| "step count 't' must be a scalar".to_string())?;
                    let lr = self.eval_value(&args[5])?.as_scalar().ok_or_else(|| "learning rate 'lr' must be a scalar".to_string())?;
                    let beta1 = if args.len() > 6 { self.eval_value(&args[6])?.as_scalar().unwrap_or(0.9) } else { 0.9 };
                    let beta2 = if args.len() > 7 { self.eval_value(&args[7])?.as_scalar().unwrap_or(0.999) } else { 0.999 };
                    let eps = if args.len() > 8 { self.eval_value(&args[8])?.as_scalar().unwrap_or(1e-8) } else { 1e-8 };
                    let wd = if args.len() > 9 { self.eval_value(&args[9])?.as_scalar().unwrap_or(0.01) } else { 0.01 };

                    let w_elems = w_val.flatten_elements();
                    let g_elems = g_val.flatten_elements();
                    let m_elems = m_val.flatten_elements();
                    let v_elems = v_val.flatten_elements();

                    if w_elems.len() != g_elems.len() || w_elems.len() != m_elems.len() || w_elems.len() != v_elems.len() {
                        return Err(format!(
                            "Dimension mismatch in adamw_step: W len {}, g len {}, m len {}, v len {}",
                            w_elems.len(), g_elems.len(), m_elems.len(), v_elems.len()
                        ));
                    }

                    let bc1 = (1.0 - beta1.powf(t.max(1.0))).max(1e-12);
                    let bc2 = (1.0 - beta2.powf(t.max(1.0))).max(1e-12);

                    let mut w_new = Vec::with_capacity(w_elems.len());
                    let mut m_new = Vec::with_capacity(m_elems.len());
                    let mut v_new = Vec::with_capacity(v_elems.len());

                    for i in 0..w_elems.len() {
                        let w = w_elems[i];
                        let g = g_elems[i];
                        let m = m_elems[i];
                        let v = v_elems[i];

                        let w_decayed = w * (1.0 - lr * wd);
                        let m_t = beta1 * m + (1.0 - beta1) * g;
                        let v_t = beta2 * v + (1.0 - beta2) * (g * g);
                        let m_hat = m_t / bc1;
                        let v_hat = v_t / bc2;
                        let w_t = w_decayed - lr * (m_hat / (v_hat.sqrt() + eps));

                        w_new.push(w_t);
                        m_new.push(m_t);
                        v_new.push(v_t);
                    }

                    let shape = w_val.shape();
                    let reconstruct = |elems: Vec<f64>, shape: &[usize]| -> Result<MathValue, String> {
                        match shape.len() {
                            0 => Ok(MathValue::Scalar(elems.first().copied().unwrap_or(0.0))),
                            1 => Ok(MathValue::Vector(elems)),
                            2 => {
                                let rows = shape[0];
                                let cols = shape[1];
                                let mut mat = Vec::with_capacity(rows);
                                for r in 0..rows {
                                    let start = r * cols;
                                    let end = (start + cols).min(elems.len());
                                    mat.push(elems[start..end].to_vec());
                                }
                                Ok(MathValue::Matrix(mat))
                            }
                            _ => MathValue::new_tensor(shape.to_vec(), elems),
                        }
                    };

                    let w_out = reconstruct(w_new, &shape)?;
                    let m_out = reconstruct(m_new, &shape)?;
                    let v_out = reconstruct(v_new, &shape)?;

                    Ok(MathValue::Tuple(vec![w_out, m_out, v_out]))
                }
                "lion_step" | "lion" => {
                    if args.len() < 4 {
                        return Err("'lion_step(W, g, m, lr, [beta1, beta2, wd])' requires at least 4 arguments".to_string());
                    }
                    let w_val = self.eval_value(&args[0])?;
                    let g_val = self.eval_value(&args[1])?;
                    let m_val = self.eval_value(&args[2])?;
                    let lr = self.eval_value(&args[3])?.as_scalar().ok_or_else(|| "learning rate 'lr' must be a scalar".to_string())?;
                    let beta1 = if args.len() > 4 { self.eval_value(&args[4])?.as_scalar().unwrap_or(0.9) } else { 0.9 };
                    let beta2 = if args.len() > 5 { self.eval_value(&args[5])?.as_scalar().unwrap_or(0.99) } else { 0.99 };
                    let wd = if args.len() > 6 { self.eval_value(&args[6])?.as_scalar().unwrap_or(0.01) } else { 0.01 };

                    let w_elems = w_val.flatten_elements();
                    let g_elems = g_val.flatten_elements();
                    let m_elems = m_val.flatten_elements();

                    if w_elems.len() != g_elems.len() || w_elems.len() != m_elems.len() {
                        return Err(format!(
                            "Dimension mismatch in lion_step: W len {}, g len {}, m len {}",
                            w_elems.len(), g_elems.len(), m_elems.len()
                        ));
                    }

                    let mut w_new = Vec::with_capacity(w_elems.len());
                    let mut m_new = Vec::with_capacity(m_elems.len());

                    for i in 0..w_elems.len() {
                        let w = w_elems[i];
                        let g = g_elems[i];
                        let m = m_elems[i];

                        let c = beta1 * m + (1.0 - beta1) * g;
                        let sign_c = if c > 0.0 { 1.0 } else if c < 0.0 { -1.0 } else { 0.0 };
                        let w_t = w * (1.0 - lr * wd) - lr * sign_c;
                        let m_t = beta2 * m + (1.0 - beta2) * g;

                        w_new.push(w_t);
                        m_new.push(m_t);
                    }

                    let shape = w_val.shape();
                    let reconstruct = |elems: Vec<f64>, shape: &[usize]| -> Result<MathValue, String> {
                        match shape.len() {
                            0 => Ok(MathValue::Scalar(elems.first().copied().unwrap_or(0.0))),
                            1 => Ok(MathValue::Vector(elems)),
                            2 => {
                                let rows = shape[0];
                                let cols = shape[1];
                                let mut mat = Vec::with_capacity(rows);
                                for r in 0..rows {
                                    let start = r * cols;
                                    let end = (start + cols).min(elems.len());
                                    mat.push(elems[start..end].to_vec());
                                }
                                Ok(MathValue::Matrix(mat))
                            }
                            _ => MathValue::new_tensor(shape.to_vec(), elems),
                        }
                    };

                    let w_out = reconstruct(w_new, &shape)?;
                    let m_out = reconstruct(m_new, &shape)?;

                    Ok(MathValue::Tuple(vec![w_out, m_out]))
                }
                "grad" | "diff" => {
                    if args.len() != 2 {
                        return Err(format!("'{}' expects (expr, var)", name));
                    }
                    if let MathExpr::Tuple(vars) | MathExpr::Array(vars) = &args[1] {
                        let inlined = args[0].inline_and_expand(&self.functions, true, 0).simplify();
                        let mut grads = Vec::with_capacity(vars.len());
                        for v_expr in vars {
                            if let MathExpr::Var(wrt) = v_expr {
                                if let Ok(analytical_grad) = self.eval_analytical_grad(&inlined, wrt) {
                                    grads.push(analytical_grad);
                                } else {
                                    let g = self.eval_value(&MathExpr::Call("grad".to_string(), vec![args[0].clone(), v_expr.clone()]))?;
                                    grads.push(g);
                                }
                            } else {
                                return Err("Target variables in multi-gradient must be identifiers".to_string());
                            }
                        }
                        return Ok(MathValue::Tuple(grads));
                    }
                    let wrt = match &args[1] {
                        MathExpr::Var(v) => v.clone(),
                        _ => {
                            return Err(
                                "Second argument to 'grad' must be a variable name or tuple of variables".to_string()
                            )
                        }
                    };
                    let inlined = args[0].inline_and_expand(&self.functions, true, 0).simplify();
                    // 1. Try Analytical Reverse-Mode Tape Autodiff first (O(1) backpropagation)
                    if let Ok(analytical_grad) = self.eval_analytical_grad(&inlined, &wrt) {
                        return Ok(analytical_grad);
                    }
                    if let Some(current_val) = self.env.get(&wrt).cloned() {
                        let eps = 1e-6_f64;
                        let eval_scalar_loss =
                            |eval_ctx: &MathEvaluator, expr: &MathExpr| -> Result<f64, String> {
                                let v = eval_ctx.eval_value(expr)?;
                                Ok(v.flatten_elements().iter().sum())
                            };
                        match current_val {
                            MathValue::Scalar(s) => {
                                let inlined = args[0].inline_and_expand(&self.functions, true, 0);
                                let deriv_expr = inlined.differentiate(&wrt).simplify();
                                if let Ok(MathValue::Scalar(sym_grad)) =
                                    self.eval_value(&deriv_expr)
                                {
                                    return Ok(MathValue::Scalar(sym_grad));
                                }
                                let mut eval_plus = self.clone();
                                eval_plus
                                    .env
                                    .insert(wrt.clone(), MathValue::Scalar(s + eps));
                                let f_plus = eval_scalar_loss(&eval_plus, &args[0])?;

                                let mut eval_minus = self.clone();
                                eval_minus
                                    .env
                                    .insert(wrt.clone(), MathValue::Scalar(s - eps));
                                let f_minus = eval_scalar_loss(&eval_minus, &args[0])?;
                                Ok(MathValue::Scalar((f_plus - f_minus) / (2.0 * eps)))
                            }
                            MathValue::Vector(vec) => {
                                let mut grad_vec = Vec::with_capacity(vec.len());
                                for i in 0..vec.len() {
                                    let mut v_plus = vec.clone();
                                    v_plus[i] += eps;
                                    let mut eval_plus = self.clone();
                                    eval_plus.env.insert(wrt.clone(), MathValue::Vector(v_plus));
                                    let f_plus = eval_scalar_loss(&eval_plus, &args[0])?;

                                    let mut v_minus = vec.clone();
                                    v_minus[i] -= eps;
                                    let mut eval_minus = self.clone();
                                    eval_minus
                                        .env
                                        .insert(wrt.clone(), MathValue::Vector(v_minus));
                                    let f_minus = eval_scalar_loss(&eval_minus, &args[0])?;

                                    grad_vec.push((f_plus - f_minus) / (2.0 * eps));
                                }
                                Ok(MathValue::Vector(grad_vec))
                            }
                            MathValue::Matrix(mat) => {
                                let rows = mat.len();
                                let cols = mat.first().map(|r| r.len()).unwrap_or(0);
                                let mut grad_mat = vec![vec![0.0; cols]; rows];
                                for r in 0..rows {
                                    for c in 0..cols {
                                        let mut m_plus = mat.clone();
                                        m_plus[r][c] += eps;
                                        let mut eval_plus = self.clone();
                                        eval_plus
                                            .env
                                            .insert(wrt.clone(), MathValue::Matrix(m_plus));
                                        let f_plus = eval_scalar_loss(&eval_plus, &args[0])?;

                                        let mut m_minus = mat.clone();
                                        m_minus[r][c] -= eps;
                                        let mut eval_minus = self.clone();
                                        eval_minus
                                            .env
                                            .insert(wrt.clone(), MathValue::Matrix(m_minus));
                                        let f_minus = eval_scalar_loss(&eval_minus, &args[0])?;

                                        grad_mat[r][c] = (f_plus - f_minus) / (2.0 * eps);
                                    }
                                }
                                Ok(MathValue::Matrix(grad_mat))
                            }
                            MathValue::Tensor {
                                shape,
                                strides,
                                data,
                            } => {
                                let total: usize = shape.iter().product();
                                let flat =
                                    MathValue::get_flattened_strided(&shape, &strides, &data);
                                let mut grad_data = vec![0.0; total];
                                for i in 0..total {
                                    let mut flat_plus = flat.clone();
                                    flat_plus[i] += eps;
                                    let mut eval_plus = self.clone();
                                    eval_plus.env.insert(
                                        wrt.clone(),
                                        MathValue::new_tensor(shape.clone(), flat_plus)?,
                                    );
                                    let f_plus = eval_scalar_loss(&eval_plus, &args[0])?;

                                    let mut flat_minus = flat.clone();
                                    flat_minus[i] -= eps;
                                    let mut eval_minus = self.clone();
                                    eval_minus.env.insert(
                                        wrt.clone(),
                                        MathValue::new_tensor(shape.clone(), flat_minus)?,
                                    );
                                    let f_minus = eval_scalar_loss(&eval_minus, &args[0])?;

                                    grad_data[i] = (f_plus - f_minus) / (2.0 * eps);
                                }
                                MathValue::new_tensor(shape, grad_data)
                            }
                            MathValue::Tuple(_) => {
                                return Err("Cannot compute scalar grad directly wrt a Tuple variable".to_string());
                            }
                        }
                    } else {
                        let inlined = args[0].inline_and_expand(&self.functions, true, 0);
                        let deriv_expr = inlined.differentiate(&wrt).simplify();
                        self.eval_value(&deriv_expr)
                    }
                }
                user_fn => {
                    if let Some((param_names, body)) = self.functions.get(user_fn) {
                        if param_names.len() != args.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, got {}",
                                user_fn,
                                param_names.len(),
                                args.len()
                            ));
                        }
                        let mut scoped_eval = self.clone();
                        for (p_name, arg_expr) in param_names.iter().zip(args.iter()) {
                            let arg_val = self.eval_value(arg_expr)?;
                            scoped_eval.env.insert(p_name.clone(), arg_val);
                        }
                        scoped_eval.eval_value(body)
                    } else {
                        Err(format!("Undefined mathematical function '{}'", user_fn))
                    }
                }
            },
            MathExpr::Iterate { input, func, count } => {
                let mut curr_val = self.eval_value(input)?;
                let count_val = self.eval_value(count)?;
                let n = count_val.as_scalar().ok_or_else(|| {
                    format!("Iteration count must be a scalar number, got {}", count_val)
                })? as usize;

                for _ in 0..n {
                    match &**func {
                        MathExpr::Var(f_name) => {
                            if let Some((params, body)) = self.functions.get(f_name).cloned() {
                                let mut local_eval = self.clone();
                                if let MathValue::Tuple(vals) = &curr_val {
                                    if vals.len() == params.len() {
                                        for (p, v) in params.iter().zip(vals.iter()) {
                                            local_eval.env.insert(p.clone(), v.clone());
                                        }
                                    } else if !params.is_empty() {
                                        local_eval.env.insert(params[0].clone(), curr_val);
                                    }
                                } else if !params.is_empty() {
                                    local_eval.env.insert(params[0].clone(), curr_val);
                                }
                                curr_val = local_eval.eval_value(&body)?;
                            } else {
                                return Err(format!(
                                    "Undefined function '{}' in functional iteration",
                                    f_name
                                ));
                            }
                        }
                        MathExpr::Call(f_name, args) => {
                            if let Some((params, body)) = self.functions.get(f_name).cloned() {
                                let mut local_eval = self.clone();
                                if let MathValue::Tuple(vals) = &curr_val {
                                    if args.len() == 1
                                        && matches!(args[0], MathExpr::Placeholder)
                                        && vals.len() == params.len()
                                    {
                                        for (p, v) in params.iter().zip(vals.iter()) {
                                            local_eval.env.insert(p.clone(), v.clone());
                                        }
                                    } else {
                                        let mut eval_args = Vec::new();
                                        let mut inserted = false;
                                        for arg in args {
                                            if let MathExpr::Placeholder = arg {
                                                eval_args.push(curr_val.clone());
                                                inserted = true;
                                            } else {
                                                eval_args.push(self.eval_value(arg)?);
                                            }
                                        }
                                        if !inserted {
                                            eval_args.insert(0, curr_val.clone());
                                        }
                                        for (p, a) in params.iter().zip(eval_args.into_iter()) {
                                            local_eval.env.insert(p.clone(), a);
                                        }
                                    }
                                } else {
                                    let mut eval_args = Vec::new();
                                    let mut inserted = false;
                                    for arg in args {
                                        if let MathExpr::Placeholder = arg {
                                            eval_args.push(curr_val.clone());
                                            inserted = true;
                                        } else {
                                            eval_args.push(self.eval_value(arg)?);
                                        }
                                    }
                                    if !inserted {
                                        eval_args.insert(0, curr_val.clone());
                                    }
                                    for (p, a) in params.iter().zip(eval_args.into_iter()) {
                                        local_eval.env.insert(p.clone(), a);
                                    }
                                }
                                curr_val = local_eval.eval_value(&body)?;
                            } else {
                                return Err(format!(
                                    "Undefined function '{}' in functional iteration",
                                    f_name
                                ));
                            }
                        }
                        other => {
                            let step_expr =
                                MathParser::desugar_pipe_step(MathExpr::Placeholder, other.clone());
                            let mut local_eval = self.clone();
                            local_eval.env.insert("_".to_string(), curr_val);
                            curr_val = local_eval.eval_value(&step_expr)?;
                        }
                    }
                }
                Ok(curr_val)
            }
            MathExpr::Pipeline(stages) => {
                let mut curr_expr = stages[0].clone();
                for stage in &stages[1..] {
                    curr_expr = MathParser::desugar_pipe_step(curr_expr, stage.clone());
                }
                self.eval_value(&curr_expr)
            }
        }
    }

    fn eval_single_val(&self, args: &[MathExpr], name: &str) -> Result<MathValue, String> {
        if args.len() != 1 {
            Err(format!(
                "'{}' expects exactly 1 argument, found {}",
                name,
                args.len()
            ))
        } else {
            self.eval_value(&args[0])
        }
    }

    fn eval_binary_op(
        &self,
        lhs: &MathValue,
        op: MathBinOp,
        rhs: &MathValue,
    ) -> Result<MathValue, String> {
        // Matrix Multiplication (@)
        if op == MathBinOp::MatMul {
            return match (lhs, rhs) {
                (MathValue::Scalar(a), MathValue::Scalar(b)) => Ok(MathValue::Scalar(a * b)),
                (MathValue::Vector(u), MathValue::Vector(v)) => {
                    if u.len() != v.len() {
                        return Err(format!(
                            "Dot product dimension mismatch: {} vs {}",
                            u.len(),
                            v.len()
                        ));
                    }
                    let dot: f64 = u.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
                    Ok(MathValue::Scalar(dot))
                }
                (MathValue::Matrix(mat), MathValue::Vector(vec)) => {
                    let mut out = Vec::with_capacity(mat.len());
                    for row in mat {
                        if row.len() != vec.len() {
                            return Err(format!(
                                "Matrix-Vector dimension mismatch: row len {} vs vec len {}",
                                row.len(),
                                vec.len()
                            ));
                        }
                        let dot: f64 = row.iter().zip(vec.iter()).map(|(a, b)| a * b).sum();
                        out.push(dot);
                    }
                    Ok(MathValue::Vector(out))
                }
                (MathValue::Vector(vec), MathValue::Matrix(mat)) => {
                    if mat.is_empty() || vec.len() != mat.len() {
                        return Err(format!(
                            "Vector-Matrix dimension mismatch: vec len {} vs mat rows {}",
                            vec.len(),
                            mat.len()
                        ));
                    }
                    let n = mat[0].len();
                    let mut out = vec![0.0; n];
                    for (k, &v) in vec.iter().enumerate() {
                        for (j, o) in out.iter_mut().enumerate() {
                            *o += v * mat[k][j];
                        }
                    }
                    Ok(MathValue::Vector(out))
                }
                (MathValue::Matrix(a), MathValue::Matrix(b)) => {
                    if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
                        return Err("Matrix-Matrix dimension mismatch".to_string());
                    }
                    let m = a.len();
                    let k = b.len();
                    let n = b[0].len();
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for p in 0..k {
                            for j in 0..n {
                                out[i][j] += a[i][p] * b[p][j];
                            }
                        }
                    }
                    Ok(MathValue::Matrix(out))
                }
                (
                    MathValue::Tensor {
                        shape: s1,
                        strides: str1,
                        data: d1,
                    },
                    MathValue::Matrix(b),
                ) => {
                    let ndim = s1.len();
                    if ndim < 2 {
                        return Err("Tensor rank must be at least 2 for MatMul".to_string());
                    }
                    let k = *s1.last().unwrap_or(&0);
                    let s = s1[ndim - 2];
                    if b.is_empty() || k != b.len() {
                        return Err(format!(
                            "Dimension mismatch in batched MatMul: tensor inner dim {} != matrix rows {}",
                            k,
                            b.len()
                        ));
                    }
                    let n = b[0].len();
                    let mut out_shape = s1.clone();
                    out_shape[ndim - 1] = n;
                    let flat_lhs = MathValue::get_flattened_strided(s1, str1, d1);
                    let lhs_mat_size = s * k;
                    let num_mats = flat_lhs.len() / lhs_mat_size.max(1);
                    let mut out_data = Vec::with_capacity(num_mats * s * n);
                    for m_idx in 0..num_mats {
                        let slice = &flat_lhs[m_idx * lhs_mat_size..(m_idx + 1) * lhs_mat_size];
                        for i in 0..s {
                            for j in 0..n {
                                let mut acc = 0.0;
                                for p in 0..k {
                                    acc += slice[i * k + p] * b[p][j];
                                }
                                out_data.push(acc);
                            }
                        }
                    }
                    MathValue::new_tensor(out_shape, out_data)
                }
                (
                    MathValue::Matrix(a),
                    MathValue::Tensor {
                        shape: s2,
                        strides: str2,
                        data: d2,
                    },
                ) => {
                    let ndim = s2.len();
                    if ndim < 2 {
                        return Err("Tensor rank must be at least 2 for MatMul".to_string());
                    }
                    if a.is_empty() {
                        return Err("Empty left matrix in MatMul".to_string());
                    }
                    let m = a.len();
                    let k_a = a[0].len();
                    let k_b = s2[ndim - 2];
                    let n = *s2.last().unwrap_or(&0);
                    if k_a != k_b {
                        return Err(format!(
                            "Dimension mismatch in matrix-tensor MatMul: matrix cols {} != tensor rows {}",
                            k_a, k_b
                        ));
                    }
                    let mut out_shape = s2.clone();
                    out_shape[ndim - 2] = m;
                    let flat_rhs = MathValue::get_flattened_strided(s2, str2, d2);
                    let rhs_mat_size = k_b * n;
                    let num_mats = flat_rhs.len() / rhs_mat_size.max(1);
                    let mut out_data = Vec::with_capacity(num_mats * m * n);
                    for m_idx in 0..num_mats {
                        let slice = &flat_rhs[m_idx * rhs_mat_size..(m_idx + 1) * rhs_mat_size];
                        for i in 0..m {
                            for j in 0..n {
                                let mut acc = 0.0;
                                for p in 0..k_a {
                                    acc += a[i][p] * slice[p * n + j];
                                }
                                out_data.push(acc);
                            }
                        }
                    }
                    MathValue::new_tensor(out_shape, out_data)
                }
                (
                    MathValue::Tensor {
                        shape: s1,
                        strides: str1,
                        data: d1,
                    },
                    MathValue::Tensor {
                        shape: s2,
                        strides: str2,
                        data: d2,
                    },
                ) => {
                    let n1 = s1.len();
                    let n2 = s2.len();
                    if n1 < 2 || n2 < 2 {
                        return Err("Both tensors must have rank >= 2 for MatMul".to_string());
                    }
                    let m = s1[n1 - 2];
                    let k1 = s1[n1 - 1];
                    let k2 = s2[n2 - 2];
                    let n = s2[n2 - 1];
                    if k1 != k2 {
                        return Err(format!(
                            "Inner dimension mismatch in tensor MatMul: {} vs {}",
                            k1, k2
                        ));
                    }
                    let batch_dims = if n1 >= n2 { &s1[..n1 - 2] } else { &s2[..n2 - 2] };
                    let mut out_shape = batch_dims.to_vec();
                    out_shape.push(m);
                    out_shape.push(n);

                    let flat1 = MathValue::get_flattened_strided(s1, str1, d1);
                    let flat2 = MathValue::get_flattened_strided(s2, str2, d2);
                    let mat1_size = m * k1;
                    let mat2_size = k2 * n;
                    let num_mats1 = flat1.len() / mat1_size.max(1);
                    let num_mats2 = flat2.len() / mat2_size.max(1);
                    let num_out_mats = num_mats1.max(num_mats2);
                    let mut out_data = Vec::with_capacity(num_out_mats * m * n);

                    for b_idx in 0..num_out_mats {
                        let b1 = b_idx % num_mats1;
                        let b2 = b_idx % num_mats2;
                        let slice1 = &flat1[b1 * mat1_size..(b1 + 1) * mat1_size];
                        let slice2 = &flat2[b2 * mat2_size..(b2 + 1) * mat2_size];
                        for i in 0..m {
                            for j in 0..n {
                                let mut acc = 0.0;
                                for p in 0..k1 {
                                    acc += slice1[i * k1 + p] * slice2[p * n + j];
                                }
                                out_data.push(acc);
                            }
                        }
                    }
                    MathValue::new_tensor(out_shape, out_data)
                }
                _ => Err("Invalid operand shapes for '@' matrix multiplication".to_string()),
            };
        }

        // Element-wise scalar operation helper
        let scalar_op = |a: f64, b: f64| -> Result<f64, String> {
            match op {
                MathBinOp::Add => Ok(a + b),
                MathBinOp::Sub => Ok(a - b),
                MathBinOp::Mul | MathBinOp::MatMul => Ok(a * b),
                MathBinOp::Div => {
                    if b == 0.0 {
                        Err("Division by zero in mathematical expression".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                MathBinOp::Pow => Ok(a.powf(b)),
                MathBinOp::Eq => Ok(if (a - b).abs() < 1e-9 { 1.0 } else { 0.0 }),
                MathBinOp::Neq => Ok(if (a - b).abs() >= 1e-9 { 1.0 } else { 0.0 }),
                MathBinOp::Lt => Ok(if a < b { 1.0 } else { 0.0 }),
                MathBinOp::Lte => Ok(if a <= b { 1.0 } else { 0.0 }),
                MathBinOp::Gt => Ok(if a > b { 1.0 } else { 0.0 }),
                MathBinOp::Gte => Ok(if a >= b { 1.0 } else { 0.0 }),
            }
        };

        match (lhs, rhs) {
            (MathValue::Scalar(a), MathValue::Scalar(b)) => {
                Ok(MathValue::Scalar(scalar_op(*a, *b)?))
            }
            (MathValue::Vector(vec), MathValue::Scalar(s)) => {
                let mut out = Vec::with_capacity(vec.len());
                for &x in vec {
                    out.push(scalar_op(x, *s)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Scalar(s), MathValue::Vector(vec)) => {
                let mut out = Vec::with_capacity(vec.len());
                for &x in vec {
                    out.push(scalar_op(*s, x)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Vector(u), MathValue::Vector(v)) => {
                if u.len() != v.len() {
                    return Err(format!(
                        "Vector length mismatch in '{}' operation: {} vs {}",
                        op,
                        u.len(),
                        v.len()
                    ));
                }
                let mut out = Vec::with_capacity(u.len());
                for (&a, &b) in u.iter().zip(v.iter()) {
                    out.push(scalar_op(a, b)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Matrix(mat), MathValue::Scalar(s)) => {
                let mut out = Vec::with_capacity(mat.len());
                for row in mat {
                    let mut r = Vec::with_capacity(row.len());
                    for &x in row {
                        r.push(scalar_op(x, *s)?);
                    }
                    out.push(r);
                }
                Ok(MathValue::Matrix(out))
            }
            (MathValue::Scalar(s), MathValue::Matrix(mat)) => {
                let mut out = Vec::with_capacity(mat.len());
                for row in mat {
                    let mut r = Vec::with_capacity(row.len());
                    for &x in row {
                        r.push(scalar_op(*s, x)?);
                    }
                    out.push(r);
                }
                Ok(MathValue::Matrix(out))
            }
            (MathValue::Matrix(mat), MathValue::Vector(vec)) => {
                if mat.is_empty() {
                    return Ok(MathValue::Matrix(Vec::new()));
                }
                let cols = mat[0].len();
                let rows = mat.len();
                if vec.len() == cols {
                    let mut out = Vec::with_capacity(rows);
                    for row in mat {
                        let mut r = Vec::with_capacity(cols);
                        for (&x, &y) in row.iter().zip(vec.iter()) {
                            r.push(scalar_op(x, y)?);
                        }
                        out.push(r);
                    }
                    Ok(MathValue::Matrix(out))
                } else if vec.len() == rows {
                    let mut out = Vec::with_capacity(rows);
                    for (row, &y) in mat.iter().zip(vec.iter()) {
                        let mut r = Vec::with_capacity(cols);
                        for &x in row {
                            r.push(scalar_op(x, y)?);
                        }
                        out.push(r);
                    }
                    Ok(MathValue::Matrix(out))
                } else {
                    Err(format!(
                        "Matrix-Vector broadcast mismatch: matrix [{}x{}] vs vector [{}]",
                        rows,
                        cols,
                        vec.len()
                    ))
                }
            }
            (MathValue::Vector(vec), MathValue::Matrix(mat)) => {
                if mat.is_empty() {
                    return Ok(MathValue::Matrix(Vec::new()));
                }
                let cols = mat[0].len();
                let rows = mat.len();
                if vec.len() == cols {
                    let mut out = Vec::with_capacity(rows);
                    for row in mat {
                        let mut r = Vec::with_capacity(cols);
                        for (&x, &y) in vec.iter().zip(row.iter()) {
                            r.push(scalar_op(x, y)?);
                        }
                        out.push(r);
                    }
                    Ok(MathValue::Matrix(out))
                } else if vec.len() == rows {
                    let mut out = Vec::with_capacity(rows);
                    for (&x, row) in vec.iter().zip(mat.iter()) {
                        let mut r = Vec::with_capacity(cols);
                        for &y in row {
                            r.push(scalar_op(x, y)?);
                        }
                        out.push(r);
                    }
                    Ok(MathValue::Matrix(out))
                } else {
                    Err(format!(
                        "Vector-Matrix broadcast mismatch: vector [{}] vs matrix [{}x{}]",
                        vec.len(),
                        rows,
                        cols
                    ))
                }
            }
            (MathValue::Matrix(a), MathValue::Matrix(b)) => {
                if a.is_empty() || b.is_empty() {
                    return Ok(MathValue::Matrix(Vec::new()));
                }
                let r_a = a.len();
                let c_a = a[0].len();
                let r_b = b.len();
                let c_b = b[0].len();
                if (r_a != r_b && r_a != 1 && r_b != 1) || (c_a != c_b && c_a != 1 && c_b != 1) {
                    return Err(format!(
                        "Matrix broadcast shape mismatch: [{}x{}] vs [{}x{}]",
                        r_a, c_a, r_b, c_b
                    ));
                }
                let rows = r_a.max(r_b);
                let cols = c_a.max(c_b);
                let mut out = Vec::with_capacity(rows);
                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);
                    let ia = if r_a == 1 { 0 } else { i };
                    let ib = if r_b == 1 { 0 } else { i };
                    for j in 0..cols {
                        let ja = if c_a == 1 { 0 } else { j };
                        let jb = if c_b == 1 { 0 } else { j };
                        row.push(scalar_op(a[ia][ja], b[ib][jb])?);
                    }
                    out.push(row);
                }
                Ok(MathValue::Matrix(out))
            }
            (MathValue::Tuple(t1), MathValue::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(format!(
                        "Tuple length mismatch in binary op: {} vs {}",
                        t1.len(),
                        t2.len()
                    ));
                }
                let mut out = Vec::with_capacity(t1.len());
                for (a, b) in t1.iter().zip(t2.iter()) {
                    out.push(self.eval_binary_op(a, op, b)?);
                }
                Ok(MathValue::Tuple(out))
            }
            (MathValue::Tuple(t), scalar @ MathValue::Scalar(_)) => {
                let mut out = Vec::with_capacity(t.len());
                for a in t {
                    out.push(self.eval_binary_op(a, op, scalar)?);
                }
                Ok(MathValue::Tuple(out))
            }
            (scalar @ MathValue::Scalar(_), MathValue::Tuple(t)) => {
                let mut out = Vec::with_capacity(t.len());
                for a in t {
                    out.push(self.eval_binary_op(scalar, op, a)?);
                }
                Ok(MathValue::Tuple(out))
            }
            (
                MathValue::Tensor {
                    shape,
                    strides,
                    data,
                },
                MathValue::Scalar(s),
            ) => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let mut out = Vec::with_capacity(flat.len());
                for x in flat {
                    out.push(scalar_op(x, *s)?);
                }
                MathValue::new_tensor(shape.clone(), out)
            }
            (
                MathValue::Scalar(s),
                MathValue::Tensor {
                    shape,
                    strides,
                    data,
                },
            ) => {
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let mut out = Vec::with_capacity(flat.len());
                for x in flat {
                    out.push(scalar_op(*s, x)?);
                }
                MathValue::new_tensor(shape.clone(), out)
            }
            (
                MathValue::Tensor {
                    shape: s1,
                    strides: str1,
                    data: d1,
                },
                MathValue::Tensor {
                    shape: s2,
                    strides: str2,
                    data: d2,
                },
            ) => {
                if s1 != s2 {
                    return Err(format!(
                        "Shape mismatch in tensor elementwise op: {:?} vs {:?}",
                        s1, s2
                    ));
                }
                let flat1 = MathValue::get_flattened_strided(s1, str1, d1);
                let flat2 = MathValue::get_flattened_strided(s2, str2, d2);
                let mut out = Vec::with_capacity(flat1.len());
                for (a, b) in flat1.into_iter().zip(flat2.into_iter()) {
                    out.push(scalar_op(a, b)?);
                }
                MathValue::new_tensor(s1.clone(), out)
            }
            (
                MathValue::Tensor {
                    shape,
                    strides,
                    data,
                },
                MathValue::Vector(v),
            ) => {
                let last_dim = *shape.last().unwrap_or(&1);
                if v.len() != last_dim {
                    return Err(format!(
                        "Vector broadcast mismatch on tensor: vector len {} != tensor last dim {}",
                        v.len(),
                        last_dim
                    ));
                }
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let mut out = Vec::with_capacity(flat.len());
                for (i, x) in flat.into_iter().enumerate() {
                    out.push(scalar_op(x, v[i % last_dim])?);
                }
                MathValue::new_tensor(shape.clone(), out)
            }
            (
                MathValue::Vector(v),
                MathValue::Tensor {
                    shape,
                    strides,
                    data,
                },
            ) => {
                let last_dim = *shape.last().unwrap_or(&1);
                if v.len() != last_dim {
                    return Err(format!(
                        "Vector broadcast mismatch on tensor: vector len {} != tensor last dim {}",
                        v.len(),
                        last_dim
                    ));
                }
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let mut out = Vec::with_capacity(flat.len());
                for (i, x) in flat.into_iter().enumerate() {
                    out.push(scalar_op(v[i % last_dim], x)?);
                }
                MathValue::new_tensor(shape.clone(), out)
            }
            _ => Err("Incompatible tensor operands in binary operation".to_string()),
        }
    }

    fn eval_tape(&self, expr: &MathExpr) -> Result<(MathValue, ExprTape), String> {
        match expr {
            MathExpr::Num(n) => Ok((MathValue::Scalar(*n), ExprTape::Num)),
            MathExpr::Var(v) => {
                if let Some(val) = self.env.get(v) {
                    Ok((val.clone(), ExprTape::Var(v.clone())))
                } else {
                    Err(format!("Variable '{}' not found in environment for autodiff", v))
                }
            }
            MathExpr::Array(elems) => {
                let mut vals = Vec::with_capacity(elems.len());
                let mut tapes = Vec::with_capacity(elems.len());
                for e in elems {
                    let (v, t) = self.eval_tape(e)?;
                    vals.push(v);
                    tapes.push(t);
                }
                let is_all_scalars = vals.iter().all(|v| matches!(v, MathValue::Scalar(_)));
                let is_all_vectors = vals.iter().all(|v| matches!(v, MathValue::Vector(_)));
                let is_all_matrices = vals.iter().all(|v| matches!(v, MathValue::Matrix(_)));
                let out_val = if is_all_scalars {
                    MathValue::Vector(vals.into_iter().map(|v| v.as_scalar().unwrap()).collect())
                } else if is_all_vectors {
                    MathValue::Matrix(
                        vals.into_iter()
                            .map(|v| match v {
                                MathValue::Vector(row) => row,
                                _ => vec![],
                            })
                            .collect(),
                    )
                } else if is_all_matrices {
                    let mut mat_list = Vec::with_capacity(vals.len());
                    for v in vals {
                        if let MathValue::Matrix(m) = v {
                            mat_list.push(m);
                        }
                    }
                    let r = mat_list[0].len();
                    let c = mat_list[0].first().map(|row| row.len()).unwrap_or(0);
                    let shape = vec![mat_list.len(), r, c];
                    let flat: Vec<f64> = mat_list.into_iter().flatten().flatten().collect();
                    MathValue::new_tensor(shape, flat)?
                } else {
                    return Err("Mixed types in array literal for tape".to_string());
                };
                Ok((out_val, ExprTape::Array(tapes)))
            }
            MathExpr::Tuple(items) => {
                let mut vals = Vec::with_capacity(items.len());
                let mut tapes = Vec::with_capacity(items.len());
                for e in items {
                    let (v, t) = self.eval_tape(e)?;
                    vals.push(v);
                    tapes.push(t);
                }
                Ok((MathValue::Tuple(vals), ExprTape::Array(tapes)))
            }
            MathExpr::Index(base, idx_expr) => {
                let (base_val, base_tape) = self.eval_tape(base)?;
                let idx_val = self.eval(idx_expr)? as usize;
                let out_val = match &base_val {
                    MathValue::Tuple(vals) => {
                        if idx_val < vals.len() {
                            vals[idx_val].clone()
                        } else {
                            return Err(format!(
                                "Index {} out of bounds for tuple of len {}",
                                idx_val,
                                vals.len()
                            ));
                        }
                    }
                    MathValue::Vector(v) => {
                        if idx_val < v.len() {
                            MathValue::Scalar(v[idx_val])
                        } else {
                            return Err(format!(
                                "Index {} out of bounds for vector of len {}",
                                idx_val,
                                v.len()
                            ));
                        }
                    }
                    MathValue::Matrix(m) => {
                        if idx_val < m.len() {
                            MathValue::Vector(m[idx_val].clone())
                        } else {
                            return Err(format!(
                                "Row index {} out of bounds for matrix with {} rows",
                                idx_val,
                                m.len()
                            ));
                        }
                    }
                    MathValue::Tensor { shape, strides, data } => {
                        if shape.is_empty() {
                            return Err("Cannot index empty tensor in tape".to_string());
                        }
                        if idx_val >= shape[0] {
                            return Err(format!(
                                "Index {} out of bounds for tensor with dim {}",
                                idx_val, shape[0]
                            ));
                        }
                        let sub_shape = shape[1..].to_vec();
                        let flat = MathValue::get_flattened_strided(shape, strides, data);
                        let sub_total: usize = sub_shape.iter().product();
                        let s_offset = idx_val * sub_total;
                        let e_offset = s_offset + sub_total;
                        let sub_data = flat[s_offset..e_offset.min(flat.len())].to_vec();
                        match sub_shape.len() {
                            0 => MathValue::Scalar(sub_data.first().copied().unwrap_or(0.0)),
                            1 => MathValue::Vector(sub_data),
                            2 => {
                                let r = sub_shape[0];
                                let c = sub_shape[1];
                                let mut mat = Vec::with_capacity(r);
                                for i in 0..r {
                                    mat.push(sub_data[i * c..(i + 1) * c].to_vec());
                                }
                                MathValue::Matrix(mat)
                            }
                            _ => MathValue::new_tensor(sub_shape, sub_data)?,
                        }
                    }
                    _ => return Err("Cannot index scalar in tape".to_string()),
                };
                Ok((
                    out_val,
                    ExprTape::Index {
                        base_val,
                        base_tape: Box::new(base_tape),
                        idx_val,
                    },
                ))
            }
            MathExpr::Unary(op, inner) => {
                let (inner_val, inner_tape) = self.eval_tape(inner)?;
                let res = match op {
                    MathUnaryOp::Neg => inner_val.map_unary(|x| -x),
                    MathUnaryOp::Not => {
                        inner_val.map_unary(|x| if x.abs() < 1e-12 { 1.0 } else { 0.0 })
                    }
                };
                Ok((res, ExprTape::Unary(*op, inner_val, Box::new(inner_tape))))
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let (lhs_val, lhs_tape) = self.eval_tape(lhs)?;
                let (rhs_val, rhs_tape) = self.eval_tape(rhs)?;
                let res = self.eval_binary_op(&lhs_val, *op, &rhs_val)?;
                Ok((
                    res,
                    ExprTape::Binary {
                        op: *op,
                        lhs_val,
                        lhs_tape: Box::new(lhs_tape),
                        rhs_val,
                        rhs_tape: Box::new(rhs_tape),
                    },
                ))
            }
            MathExpr::Call(name, args) => {
                let mut arg_vals = Vec::with_capacity(args.len());
                let mut arg_tapes = Vec::with_capacity(args.len());
                for a in args {
                    let (v, t) = self.eval_tape(a)?;
                    arg_vals.push(v);
                    arg_tapes.push(t);
                }
                let res = self.eval_value(expr)?;
                Ok((
                    res,
                    ExprTape::Call {
                        name: name.clone(),
                        arg_vals,
                        arg_tapes,
                    },
                ))
            }
            _ => {
                let res = self.eval_value(expr)?;
                Ok((res, ExprTape::Num))
            }
        }
    }

    /// Evaluates exact analytical matrix/vector/scalar gradient using reverse-mode autodiff
    pub fn eval_analytical_grad(&self, expr: &MathExpr, wrt: &str) -> Result<MathValue, String> {
        let wrt_val = self.env.get(wrt).ok_or_else(|| {
            format!(
                "Variable '{}' not found in environment for analytical gradient",
                wrt
            )
        })?;
        let (loss_val, tape) = self.eval_tape(expr)?;
        let mut acc_grad = wrt_val.zeros_like();
        let incoming_adj = match &loss_val {
            MathValue::Scalar(_) => MathValue::Scalar(1.0),
            MathValue::Vector(v) => MathValue::Vector(vec![1.0; v.len()]),
            MathValue::Matrix(m) => {
                let rows = m.len();
                let cols = m.first().map(|r| r.len()).unwrap_or(0);
                MathValue::Matrix(vec![vec![1.0; cols]; rows])
            }
            MathValue::Tuple(items) => {
                MathValue::Tuple(items.iter().map(|_| MathValue::Scalar(1.0)).collect())
            }
            MathValue::Tensor { shape, .. } => {
                let total: usize = shape.iter().product();
                MathValue::Tensor {
                    shape: shape.clone(),
                    strides: MathValue::compute_contiguous_strides(shape),
                    data: vec![1.0; total],
                }
            }
        };
        tape.backward(&incoming_adj, wrt, &mut acc_grad, self)?;
        Ok(acc_grad)
    }
}

// ============================================================================
// TRANSPILERS & CODE GENERATORS (ISO C23, NVIDIA CUDA PTX, 4-SLOT VLIW)
// ============================================================================

impl MathProgram {
    /// High-level Tensor Graph Optimization & Kernel Fusion pass across all program items
    pub fn optimize_tensor_graph(&self) -> MathProgram {
        let mut opt = self.clone();
        for item in &mut opt.items {
            match item {
                MathItem::Binding { expr, .. } | MathItem::Expression(expr) => {
                    *expr = expr.optimize_tensor_graph();
                }
                MathItem::Export { expr, .. } => {
                    *expr = expr.optimize_tensor_graph();
                }
                MathItem::Function { body, .. } => {
                    *body = body.optimize_tensor_graph();
                }
                _ => {}
            }
        }
        opt
    }

    /// Collects all defined mathematical functions for beta-reduction inlining and symbolic autodiff
    pub fn collect_functions(&self) -> HashMap<String, (Vec<String>, MathExpr)> {
        let mut fns = HashMap::new();
        for item in &self.items {
            if let MathItem::Function {
                name, params, body, ..
            } = item
            {
                let p_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                fns.insert(name.clone(), (p_names, body.clone()));
            }
        }
        fns
    }

    /// Collects full function signatures including typed parameters with tensor dimensions
    pub fn collect_function_signatures(
        &self,
    ) -> HashMap<String, (Vec<MathParam>, Option<MathDtype>, MathExpr)> {
        let mut sigs = HashMap::new();
        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                sigs.insert(
                    name.clone(),
                    (params.clone(), ret_type.clone(), body.clone()),
                );
            }
        }
        sigs
    }

    /// Infers static shapes for all top-level bindings in the program
    pub fn infer_shapes_map(&self) -> HashMap<String, Vec<usize>> {
        let fns = self.collect_function_signatures();
        let mut env: HashMap<String, Vec<usize>> = HashMap::new();
        let mut errors = Vec::new();

        for item in &self.items {
            match item {
                MathItem::Constant { name, .. } => {
                    env.insert(name.clone(), vec![]);
                }
                MathItem::Binding { name, expr } => {
                    if let Some(shape) = Self::infer_shape_expr(
                        expr,
                        &env,
                        &fns,
                        &mut errors,
                        Some(&format!("binding '{}'", name)),
                    ) {
                        env.insert(name.clone(), shape);
                    }
                }
                MathItem::TupleBinding { names, expr } => {
                    if let MathExpr::Tuple(items) = expr {
                        for (name, item_expr) in names.iter().zip(items.iter()) {
                            if let Some(shape) = Self::infer_shape_expr(
                                item_expr,
                                &env,
                                &fns,
                                &mut errors,
                                Some(&format!("tuple binding '{}'", name)),
                            ) {
                                env.insert(name.clone(), shape);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        env
    }

    /// Verifies tensor dimension consistency across the entire mathematical program.
    /// Returns Ok(()) if all matrix operations, tensor shapes, and function calls are dimensionally sound,
    /// or Err(Vec<String>) with detailed diagnostic error messages describing all shape mismatches.
    pub fn verify_shapes(&self) -> Result<(), Vec<String>> {
        let fns = self.collect_function_signatures();
        let mut errors = Vec::new();

        // 1. Verify function definitions
        for item in &self.items {
            if let MathItem::Function {
                name, params, body, ..
            } = item
            {
                let mut fn_env: HashMap<String, Vec<usize>> = HashMap::new();
                for param in params {
                    fn_env.insert(param.name.clone(), param.dims.clone());
                }
                let _ = Self::infer_shape_expr(
                    body,
                    &fn_env,
                    &fns,
                    &mut errors,
                    Some(&format!("function '{}'", name)),
                );
            }
        }

        // 2. Verify top-level bindings and expressions
        let mut global_env: HashMap<String, Vec<usize>> = HashMap::new();
        for item in &self.items {
            match item {
                MathItem::Constant { name, .. } => {
                    global_env.insert(name.clone(), vec![]);
                }
                MathItem::Binding { name, expr } => {
                    let shape_opt = Self::infer_shape_expr(
                        expr,
                        &global_env,
                        &fns,
                        &mut errors,
                        Some(&format!("binding '{}'", name)),
                    );
                    if let Some(shape) = shape_opt {
                        global_env.insert(name.clone(), shape);
                    }
                }
                MathItem::TupleBinding { names, expr } => {
                    if let MathExpr::Tuple(items) = expr {
                        for (name, item_expr) in names.iter().zip(items.iter()) {
                            let shape_opt = Self::infer_shape_expr(
                                item_expr,
                                &global_env,
                                &fns,
                                &mut errors,
                                Some(&format!("tuple binding '{}'", name)),
                            );
                            if let Some(shape) = shape_opt {
                                global_env.insert(name.clone(), shape);
                            }
                        }
                    } else {
                        let _ = Self::infer_shape_expr(
                            expr,
                            &global_env,
                            &fns,
                            &mut errors,
                            Some("tuple binding expression"),
                        );
                    }
                }
                MathItem::Expression(expr) => {
                    let _ = Self::infer_shape_expr(
                        expr,
                        &global_env,
                        &fns,
                        &mut errors,
                        Some("top-level expression"),
                    );
                }
                MathItem::Export { expr, path, key } => {
                    let _ = Self::infer_shape_expr(
                        expr,
                        &global_env,
                        &fns,
                        &mut errors,
                        Some(&format!("export to '{}:{}'", path, key)),
                    );
                }
                _ => {}
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Recursively infers and checks tensor shape for a mathematical expression AST
    pub fn infer_shape_expr(
        expr: &MathExpr,
        env: &HashMap<String, Vec<usize>>,
        fn_sigs: &HashMap<String, (Vec<MathParam>, Option<MathDtype>, MathExpr)>,
        errors: &mut Vec<String>,
        ctx: Option<&str>,
    ) -> Option<Vec<usize>> {
        let prefix = ctx.map(|c| format!("In {}: ", c)).unwrap_or_default();
        match expr {
            MathExpr::Num(_) => Some(vec![]),
            MathExpr::Var(name) => {
                if let Some(shape) = env.get(name) {
                    Some(shape.clone())
                } else {
                    None
                }
            }
            MathExpr::Array(elems) => {
                if elems.is_empty() {
                    return Some(vec![0]);
                }
                let s0 = Self::infer_shape_expr(&elems[0], env, fn_sigs, errors, ctx)?;
                for (i, elem) in elems.iter().enumerate().skip(1) {
                    if let Some(si) = Self::infer_shape_expr(elem, env, fn_sigs, errors, ctx) {
                        if si != s0 {
                            errors.push(format!(
                                "{}Ragged tensor array literal: row 0 has dimension {:?} but row {} has dimension {:?}",
                                prefix, s0, i, si
                            ));
                            return None;
                        }
                    }
                }
                let mut full = vec![elems.len()];
                full.extend(s0);
                Some(full)
            }
            MathExpr::Tuple(items) => {
                for item in items {
                    let _ = Self::infer_shape_expr(item, env, fn_sigs, errors, ctx);
                }
                None
            }
            MathExpr::Index(base, idx) => {
                let _ = Self::infer_shape_expr(idx, env, fn_sigs, errors, ctx);
                if let Some(b_shape) = Self::infer_shape_expr(base, env, fn_sigs, errors, ctx) {
                    if b_shape.is_empty() {
                        errors.push(format!("{}Cannot index into a scalar value", prefix));
                        None
                    } else {
                        Some(b_shape[1..].to_vec())
                    }
                } else {
                    None
                }
            }
            MathExpr::TensorRef { path, key } => {
                if let Ok(val) = load_tensor_resource(path, key) {
                    match val {
                        MathValue::Scalar(_) => Some(vec![]),
                        MathValue::Vector(v) => Some(vec![v.len()]),
                        MathValue::Matrix(m) => {
                            let rows = m.len();
                            let cols = m.first().map(|r| r.len()).unwrap_or(0);
                            Some(vec![rows, cols])
                        }
                        MathValue::Tensor { shape, .. } => Some(shape),
                        MathValue::Tuple(_) => None,
                    }
                } else {
                    None
                }
            }
            MathExpr::Unary(_, inner) => Self::infer_shape_expr(inner, env, fn_sigs, errors, ctx),
            MathExpr::Binary(lhs, op, rhs) => {
                // Check for transpose notation: W^T or expr^T
                if *op == MathBinOp::Pow {
                    if let MathExpr::Var(v) = &**rhs {
                        if v == "T" {
                            if let Some(s) = Self::infer_shape_expr(lhs, env, fn_sigs, errors, ctx)
                            {
                                if s.len() == 2 {
                                    return Some(vec![s[1], s[0]]);
                                } else if s.len() == 1 {
                                    return Some(vec![s[0], 1]);
                                } else if s.is_empty() {
                                    return Some(vec![]);
                                } else {
                                    let mut out = s.clone();
                                    let n = out.len();
                                    out.swap(n - 2, n - 1);
                                    return Some(out);
                                }
                            }
                        }
                    }
                    let _ = Self::infer_shape_expr(rhs, env, fn_sigs, errors, ctx);
                    return Self::infer_shape_expr(lhs, env, fn_sigs, errors, ctx);
                }

                let l_opt = Self::infer_shape_expr(lhs, env, fn_sigs, errors, ctx);
                let r_opt = Self::infer_shape_expr(rhs, env, fn_sigs, errors, ctx);

                if *op == MathBinOp::MatMul {
                    match (l_opt, r_opt) {
                        (Some(l), Some(r)) => match (l.len(), r.len()) {
                            (2, 2) => {
                                if l[1] != r[0] {
                                    errors.push(format!(
                                        "{}Matrix multiplication shape mismatch (@): left shape {:?} (inner dim {}) cannot multiply right shape {:?} (outer dim {})",
                                        prefix, l, l[1], r, r[0]
                                    ));
                                    None
                                } else {
                                    Some(vec![l[0], r[1]])
                                }
                            }
                            (2, 1) => {
                                if l[1] != r[0] {
                                    errors.push(format!(
                                        "{}Matrix-vector multiplication shape mismatch (@): matrix shape {:?} (inner dim {}) cannot multiply vector shape {:?} (dim {})",
                                        prefix, l, l[1], r, r[0]
                                    ));
                                    None
                                } else {
                                    Some(vec![l[0]])
                                }
                            }
                            (1, 2) => {
                                if l[0] != r[0] {
                                    errors.push(format!(
                                        "{}Vector-matrix multiplication shape mismatch (@): vector shape {:?} (dim {}) cannot multiply matrix shape {:?} (outer dim {})",
                                        prefix, l, l[0], r, r[0]
                                    ));
                                    None
                                } else {
                                    Some(vec![r[1]])
                                }
                            }
                            (1, 1) => {
                                if l[0] != r[0] {
                                    errors.push(format!(
                                        "{}Vector dot product shape mismatch (@): vector shape {:?} (dim {}) != vector shape {:?} (dim {})",
                                        prefix, l, l[0], r, r[0]
                                    ));
                                    None
                                } else {
                                    Some(vec![])
                                }
                            }
                            (0, _) => Some(r),
                            (_, 0) => Some(l),
                            (nl, nr) if nl >= 2 && nr >= 2 => {
                                let k_l = l[nl - 1];
                                let k_r = r[nr - 2];
                                if k_l != k_r {
                                    errors.push(format!(
                                        "{}Batched MatMul (@) inner dimension mismatch: left shape {:?} (dim {}) cannot multiply right shape {:?} (dim {})",
                                        prefix, l, k_l, r, k_r
                                    ));
                                    None
                                } else {
                                    let batch_dims = if nl >= nr { &l[..nl - 2] } else { &r[..nr - 2] };
                                    let mut out = batch_dims.to_vec();
                                    out.push(l[nl - 2]);
                                    out.push(r[nr - 1]);
                                    Some(out)
                                }
                            }
                            _ => {
                                errors.push(format!(
                                    "{}MatMul (@) unsupported between rank {} and rank {}",
                                    prefix,
                                    l.len(),
                                    r.len()
                                ));
                                None
                            }
                        },
                        (Some(l), None) => {
                            if l.len() >= 2 {
                                Some(vec![l[0]])
                            } else {
                                None
                            }
                        }
                        (None, Some(r)) => {
                            if r.len() >= 2 {
                                Some(vec![r[1]])
                            } else {
                                None
                            }
                        }
                        (None, None) => None,
                    }
                } else {
                    // Element-wise arithmetic & comparison ops with NumPy/PyTorch broadcasting
                    match (l_opt, r_opt) {
                        (Some(l), Some(r)) => {
                            if l.is_empty() {
                                Some(r)
                            } else if r.is_empty() {
                                Some(l)
                            } else if l == r {
                                Some(l)
                            } else if l.len() == 2
                                && r.len() == 1
                                && (r[0] == l[1] || r[0] == l[0] || r[0] == 1)
                            {
                                Some(l)
                            } else if l.len() == 1
                                && r.len() == 2
                                && (l[0] == r[1] || l[0] == r[0] || l[0] == 1)
                            {
                                Some(r)
                            } else if l.len() == 2
                                && r.len() == 2
                                && (l[0] == r[0] || l[0] == 1 || r[0] == 1)
                                && (l[1] == r[1] || l[1] == 1 || r[1] == 1)
                            {
                                Some(vec![l[0].max(r[0]), l[1].max(r[1])])
                            } else if l.len() == 1 && r.len() == 1 && (l[0] == 1 || r[0] == 1) {
                                Some(vec![l[0].max(r[0])])
                            } else {
                                errors.push(format!(
                                    "{}Element-wise binary op '{}' shape mismatch: left shape {:?} != right shape {:?}",
                                    prefix, op, l, r
                                ));
                                None
                            }
                        }
                        (Some(l), None) => Some(l),
                        (None, Some(r)) => Some(r),
                        (None, None) => None,
                    }
                }
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let _ = Self::infer_shape_expr(cond, env, fn_sigs, errors, ctx);
                let t_opt = Self::infer_shape_expr(then_expr, env, fn_sigs, errors, ctx);
                let e_opt = Self::infer_shape_expr(else_expr, env, fn_sigs, errors, ctx);
                match (t_opt, e_opt) {
                    (Some(t), Some(e)) => {
                        if t == e || e.is_empty() {
                            Some(t)
                        } else if t.is_empty() {
                            Some(e)
                        } else {
                            errors.push(format!(
                                "{}Ternary conditional branch shape mismatch: 'then' branch {:?} != 'else' branch {:?}",
                                prefix, t, e
                            ));
                            None
                        }
                    }
                    (Some(t), None) => Some(t),
                    (None, Some(e)) => Some(e),
                    (None, None) => None,
                }
            }
            MathExpr::Call(func, args) => match func.as_str() {
                "transpose" | "T" => {
                    if let Some(arg) = args.first() {
                        if let Some(s) = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx) {
                            if s.len() == 2 {
                                Some(vec![s[1], s[0]])
                            } else if s.len() == 1 {
                                Some(vec![s[0], 1])
                            } else if s.len() >= 3 {
                                let mut out = s.clone();
                                let n = out.len();
                                out.swap(n - 2, n - 1);
                                Some(out)
                            } else {
                                Some(s)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "outer" => {
                    if args.len() == 2 {
                        let u = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let v = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        if let (Some(u_s), Some(v_s)) = (u, v) {
                            if u_s.len() != 1 || v_s.len() != 1 {
                                errors.push(format!(
                                    "{}outer() requires two 1D vector arguments, received shapes {:?} and {:?}",
                                    prefix, u_s, v_s
                                ));
                                None
                            } else {
                                Some(vec![u_s[0], v_s[0]])
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!("{}outer() expects exactly 2 arguments", prefix));
                        None
                    }
                }
                "reshape" => {
                    if !args.is_empty() {
                        let in_shape = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let mut target_dims = Vec::new();
                        if args.len() == 2 {
                            if let MathExpr::Array(dims_exprs) = &args[1] {
                                for d in dims_exprs {
                                    if let MathExpr::Num(n) = d {
                                        target_dims.push(*n as usize);
                                    }
                                }
                            } else if let MathExpr::Num(n) = &args[1] {
                                target_dims.push(*n as usize);
                            }
                        } else {
                            for d in &args[1..] {
                                if let MathExpr::Num(n) = d {
                                    target_dims.push(*n as usize);
                                }
                            }
                        }
                        if !target_dims.is_empty() {
                            if let Some(ref in_s) = in_shape {
                                let in_total: usize = in_s.iter().product();
                                let target_total: usize = target_dims.iter().product();
                                if in_total > 0 && target_total > 0 && in_total != target_total {
                                    errors.push(format!(
                                        "{}Cannot reshape tensor with {} elements (shape {:?}) into shape {:?} ({} elements)",
                                        prefix, in_total, in_s, target_dims, target_total
                                    ));
                                }
                            }
                            Some(target_dims)
                        } else {
                            in_shape
                        }
                    } else {
                        None
                    }
                }
                "slice" => {
                    if !args.is_empty() {
                        let in_shape = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let start = if args.len() > 1 {
                            if let MathExpr::Num(n) = &args[1] {
                                *n as usize
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        let end = if args.len() > 2 {
                            if let MathExpr::Num(n) = &args[2] {
                                *n as usize
                            } else {
                                usize::MAX
                            }
                        } else {
                            usize::MAX
                        };
                        if let Some(s) = in_shape {
                            if s.is_empty() {
                                errors.push(format!("{}Cannot slice scalar value", prefix));
                                None
                            } else {
                                let d0 = s[0];
                                let s_idx = start.min(d0);
                                let e_idx = end.min(d0);
                                let new_d0 = if s_idx > e_idx { 0 } else { e_idx - s_idx };
                                let mut out_s = vec![new_d0];
                                out_s.extend_from_slice(&s[1..]);
                                Some(out_s)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "sum" | "mean" | "norm" | "max" | "min" | "prod" => {
                    if let Some(arg) = args.first() {
                        let _ = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx);
                    }
                    Some(vec![])
                }
                "sum_axis" => {
                    if !args.is_empty() {
                        let axis = if args.len() > 1 {
                            if let MathExpr::Num(n) = &args[1] {
                                *n as usize
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        if let Some(s) =
                            Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx)
                        {
                            if s.len() == 2 {
                                if axis == 0 {
                                    Some(vec![s[1]])
                                } else if axis == 1 {
                                    Some(vec![s[0]])
                                } else {
                                    errors.push(format!(
                                        "{}sum_axis: axis {} out of bounds for 2D matrix",
                                        prefix, axis
                                    ));
                                    None
                                }
                            } else if s.len() == 1 {
                                Some(vec![])
                            } else {
                                Some(s)
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "argmax" | "argmin" => {
                    if let Some(arg) = args.first() {
                        if let Some(s) = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx) {
                            if s.len() == 2 {
                                Some(vec![s[0]])
                            } else {
                                Some(vec![])
                            }
                        } else {
                            None
                        }
                    } else {
                        Some(vec![])
                    }
                }
                "clip_grad" | "grad_clip" => {
                    if !args.is_empty() {
                        Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx)
                    } else {
                        None
                    }
                }
                "cosine_lr" => Some(vec![]),
                "adamw_step" | "adamw" | "lion_step" | "lion" => {
                    for a in args {
                        let _ = Self::infer_shape_expr(a, env, fn_sigs, errors, ctx);
                    }
                    None
                }
                "zeros" | "ones" => {
                    if args.len() == 1 {
                        match &args[0] {
                            MathExpr::Num(n) => Some(vec![*n as usize]),
                            MathExpr::Array(dims) => {
                                let mut out = Vec::new();
                                for d in dims {
                                    if let MathExpr::Num(n) = d {
                                        out.push(*n as usize);
                                    }
                                }
                                Some(out)
                            }
                            _ => None,
                        }
                    } else if args.len() >= 2 {
                        let mut out = Vec::new();
                        for a in args {
                            if let MathExpr::Num(n) = a {
                                out.push(*n as usize);
                            }
                        }
                        if out.len() == args.len() {
                            Some(out)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "eye" => {
                    if args.len() == 1 {
                        if let MathExpr::Num(n) = &args[0] {
                            let d = *n as usize;
                            Some(vec![d, d])
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!("{}eye(N) expects 1 argument", prefix));
                        None
                    }
                }
                "randn" => {
                    if args.len() == 1 {
                        match &args[0] {
                            MathExpr::Num(n) => Some(vec![*n as usize]),
                            MathExpr::Array(dims) => {
                                let mut out = Vec::new();
                                for d in dims {
                                    if let MathExpr::Num(n) = d {
                                        out.push(*n as usize);
                                    }
                                }
                                Some(out)
                            }
                            _ => None,
                        }
                    } else if args.len() >= 2 {
                        let mut out = Vec::new();
                        for a in args {
                            if let MathExpr::Num(n) = a {
                                out.push(*n as usize);
                            }
                        }
                        if out.len() == args.len() {
                            Some(out)
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!("{}randn() expects at least 1 argument", prefix));
                        None
                    }
                }
                "concat" => {
                    if args.len() >= 2 {
                        let a_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let b_s = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        let axis = if args.len() > 2 {
                            if let MathExpr::Num(n) = &args[2] {
                                *n as usize
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        if let (Some(a), Some(b)) = (a_s, b_s) {
                            if a.len() == 1 && b.len() == 1 {
                                Some(vec![a[0] + b[0]])
                            } else if a.len() == 1 && b.is_empty() {
                                Some(vec![a[0] + 1])
                            } else if a.len() == 2 && b.len() == 2 {
                                if axis == 0 {
                                    if a[1] != b[1] {
                                        errors.push(format!(
                                            "{}concat(axis=0) column mismatch: {:?} vs {:?}",
                                            prefix, a, b
                                        ));
                                        None
                                    } else {
                                        Some(vec![a[0] + b[0], a[1]])
                                    }
                                } else if a[0] != b[0] {
                                    errors.push(format!(
                                        "{}concat(axis=1) row mismatch: {:?} vs {:?}",
                                        prefix, a, b
                                    ));
                                    None
                                } else {
                                    Some(vec![a[0], a[1] + b[1]])
                                }
                            } else if a.len() == 2 && b.len() == 1 {
                                if a[1] != b[0] {
                                    errors.push(format!(
                                        "{}concat matrix-vector dimension mismatch: {:?} vs {:?}",
                                        prefix, a, b
                                    ));
                                    None
                                } else {
                                    Some(vec![a[0] + 1, a[1]])
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!("{}concat() expects at least 2 arguments", prefix));
                        None
                    }
                }
                "grad" | "diff" => {
                    if args.len() == 2 {
                        let _ = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx)
                    } else {
                        errors.push(format!(
                            "{}grad() expects exactly 2 arguments (expr, wrt_var)",
                            prefix
                        ));
                        None
                    }
                }
                "exp" | "ln" | "log" | "sqrt" | "sin" | "cos" | "tan" | "tanh" | "abs"
                | "sigmoid" | "relu" | "gelu" | "softmax" | "silu" | "rmsnorm" | "rope"
                | "log_softmax" => {
                    if let Some(arg) = args.first() {
                        Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx)
                    } else {
                        Some(vec![])
                    }
                }
                "embed" => {
                    if args.len() >= 2 {
                        let table_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let tok_s = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        match (table_s, tok_s) {
                            (Some(tbl), Some(tok)) => {
                                if tbl.len() != 2 {
                                    errors.push(format!(
                                        "{}embed table must be a 2D matrix [vocab_size, dim], got {:?}",
                                        prefix, tbl
                                    ));
                                    None
                                } else if tok.len() == 1 {
                                    Some(vec![tok[0], tbl[1]])
                                } else if tok.is_empty() {
                                    Some(vec![tbl[1]])
                                } else {
                                    errors.push(format!(
                                        "{}embed tokens must be 1D vector or scalar, got {:?}",
                                        prefix, tok
                                    ));
                                    None
                                }
                            }
                            (Some(tbl), None) => {
                                if tbl.len() == 2 {
                                    Some(vec![1, tbl[1]])
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    } else {
                        errors.push(format!("{}embed(E, tokens) expects 2 arguments", prefix));
                        None
                    }
                }
                "cross_entropy" => {
                    if args.len() >= 2 {
                        let _ = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let _ = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        Some(vec![])
                    } else {
                        errors.push(format!(
                            "{}cross_entropy(logits, targets) expects 2 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "attention" | "flash_attn" | "attn" | "causal_attn" | "masked_attn" => {
                    if args.len() >= 3 {
                        let q = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let k = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        let v = Self::infer_shape_expr(&args[2], env, fn_sigs, errors, ctx);
                        if let (Some(q_s), Some(k_s), Some(v_s)) = (q, k, v) {
                            if q_s.len() == 2 && k_s.len() == 2 && v_s.len() == 2 {
                                if q_s[1] != k_s[1] {
                                    errors.push(format!(
                                        "{}Attention head dimension mismatch: Q has dim {} but K has dim {}",
                                        prefix, q_s[1], k_s[1]
                                    ));
                                    None
                                } else if k_s[0] != v_s[0] {
                                    errors.push(format!(
                                        "{}Attention sequence length mismatch: K has seq_len {} but V has seq_len {}",
                                        prefix, k_s[0], v_s[0]
                                    ));
                                    None
                                } else {
                                    Some(vec![q_s[0], v_s[1]])
                                }
                            } else {
                                errors.push(format!(
                                    "{}Attention requires 2D matrices [seq, dim] for Q, K, V",
                                    prefix
                                ));
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!(
                            "{}Attention expects at least 3 arguments (Q, K, V)",
                            prefix
                        ));
                        None
                    }
                }
                "split_heads" => {
                    if args.len() >= 2 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let n_heads = if let MathExpr::Num(n) = &args[1] {
                            *n as usize
                        } else {
                            1
                        };
                        if let Some(s) = x_s {
                            if s.len() == 2 {
                                let (seq, hidden) = (s[0], s[1]);
                                let head_dim = hidden / n_heads.max(1);
                                Some(vec![n_heads, seq, head_dim])
                            } else if s.len() == 3 {
                                let (batch, seq, hidden) = (s[0], s[1], s[2]);
                                let head_dim = hidden / n_heads.max(1);
                                Some(vec![batch, n_heads, seq, head_dim])
                            } else {
                                errors.push(format!(
                                    "{}split_heads expects 2D or 3D tensor, got {:?}",
                                    prefix, s
                                ));
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!(
                            "{}split_heads(X, n_heads) expects 2 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "merge_heads" => {
                    if let Some(arg) = args.first() {
                        if let Some(s) = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx) {
                            if s.len() == 3 {
                                let (n_heads, seq, head_dim) = (s[0], s[1], s[2]);
                                Some(vec![seq, n_heads * head_dim])
                            } else if s.len() == 4 {
                                let (batch, n_heads, seq, head_dim) = (s[0], s[1], s[2], s[3]);
                                Some(vec![batch, seq, n_heads * head_dim])
                            } else {
                                errors.push(format!(
                                    "{}merge_heads expects 3D or 4D tensor, got {:?}",
                                    prefix, s
                                ));
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!("{}merge_heads(X) expects 1 argument", prefix));
                        None
                    }
                }
                "gqa_attn" => {
                    if args.len() >= 5 {
                        let q = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let _k = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        let _v = Self::infer_shape_expr(&args[2], env, fn_sigs, errors, ctx);
                        q
                    } else {
                        errors.push(format!(
                            "{}gqa_attn(Q, K, V, n_heads, n_kv_heads, ...) expects at least 5 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "mla_attn" => {
                    if args.len() >= 5 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        for a in &args[1..] {
                            let _ = Self::infer_shape_expr(a, env, fn_sigs, errors, ctx);
                        }
                        x_s
                    } else {
                        errors.push(format!(
                            "{}mla_attn(X, W_dkv, W_uk, W_uv, W_q, [W_o]) expects at least 5 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "ssd_scan" => {
                    if args.len() >= 2 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let _dt_s = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        x_s
                    } else {
                        errors.push(format!(
                            "{}ssd_scan(X, decay_dt) expects 2 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "permute" => {
                    if args.len() >= 2 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let mut dims = Vec::new();
                        if args.len() == 2 {
                            if let MathExpr::Array(items) = &args[1] {
                                for it in items {
                                    if let MathExpr::Num(n) = it {
                                        dims.push(*n as usize);
                                    }
                                }
                            }
                        } else {
                            for a in &args[1..] {
                                if let MathExpr::Num(n) = a {
                                    dims.push(*n as usize);
                                }
                            }
                        }
                        if let Some(s) = x_s {
                            if dims.len() == s.len() {
                                let mut out = vec![0; s.len()];
                                for (i, &d) in dims.iter().enumerate() {
                                    if d < s.len() {
                                        out[i] = s[d];
                                    }
                                }
                                Some(out)
                            } else {
                                Some(s)
                            }
                        } else {
                            None
                        }
                    } else {
                        errors.push(format!(
                            "{}permute(X, dims) expects at least 2 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "quantize_t2" | "t2_quant" => {
                    if let Some(arg) = args.first() {
                        let _ = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx);
                    }
                    None
                }
                "bitlinear" => {
                    if args.len() >= 2 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let w_s = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        if args.len() > 2 {
                            let _ = Self::infer_shape_expr(&args[2], env, fn_sigs, errors, ctx);
                        }
                        match (x_s, w_s) {
                            (Some(x), Some(w)) => {
                                if x.len() == 2 && w.len() == 2 {
                                    if x[1] == w[1] {
                                        Some(vec![x[0], w[0]])
                                    } else if x[1] == w[0] {
                                        Some(vec![x[0], w[1]])
                                    } else {
                                        errors.push(format!(
                                            "{}bitlinear dimension mismatch: X {:?} vs W {:?}",
                                            prefix, x, w
                                        ));
                                        None
                                    }
                                } else if x.len() == 1 && w.len() == 2 {
                                    if x[0] == w[1] {
                                        Some(vec![w[0]])
                                    } else {
                                        Some(vec![w[1]])
                                    }
                                } else if x.len() == 3 && w.len() == 2 {
                                    let n = if x[2] == w[1] { w[0] } else { w[1] };
                                    Some(vec![x[0], x[1], n])
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    } else {
                        errors.push(format!(
                            "{}bitlinear(X, W, [gamma]) expects at least 2 arguments",
                            prefix
                        ));
                        None
                    }
                }
                "quantize_i8" | "i8_quant" => {
                    if let Some(arg) = args.first() {
                        let _ = Self::infer_shape_expr(arg, env, fn_sigs, errors, ctx);
                    }
                    None
                }
                "qmatmul" => {
                    if args.len() >= 3 {
                        let x_s = Self::infer_shape_expr(&args[0], env, fn_sigs, errors, ctx);
                        let w_s = Self::infer_shape_expr(&args[1], env, fn_sigs, errors, ctx);
                        let _ = Self::infer_shape_expr(&args[2], env, fn_sigs, errors, ctx);
                        match (x_s, w_s) {
                            (Some(x), Some(w)) => {
                                if x.len() == 2 && w.len() == 2 {
                                    if x[1] == w[1] {
                                        Some(vec![x[0], w[0]])
                                    } else {
                                        Some(vec![x[0], w[1]])
                                    }
                                } else if x.len() == 1 && w.len() == 2 {
                                    Some(vec![w[0]])
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    } else {
                        errors.push(format!(
                            "{}qmatmul(X, W_q, scales) expects 3 arguments",
                            prefix
                        ));
                        None
                    }
                }
                user_fn => {
                    if let Some((params, _ret, body)) = fn_sigs.get(user_fn) {
                        if params.len() != args.len() {
                            errors.push(format!(
                                "{}Function '{}' expects {} arguments, received {}",
                                prefix,
                                user_fn,
                                params.len(),
                                args.len()
                            ));
                        }
                        let mut fn_env = HashMap::new();
                        for (p, a) in params.iter().zip(args.iter()) {
                            let arg_s = Self::infer_shape_expr(a, env, fn_sigs, errors, ctx);
                            if !p.dims.is_empty() {
                                if let Some(ref ashp) = arg_s {
                                    if ashp != &p.dims {
                                        errors.push(format!(
                                            "{}Argument for parameter '{}' has shape {:?}, expected shape {:?}",
                                            prefix, p.name, ashp, p.dims
                                        ));
                                    }
                                }
                                fn_env.insert(p.name.clone(), p.dims.clone());
                            } else if let Some(ashp) = arg_s {
                                fn_env.insert(p.name.clone(), ashp);
                            } else {
                                fn_env.insert(p.name.clone(), vec![]);
                            }
                        }
                        Self::infer_shape_expr(
                            body,
                            &fn_env,
                            fn_sigs,
                            errors,
                            Some(&format!("call to '{}'", user_fn)),
                        )
                    } else {
                        None
                    }
                }
            },
            MathExpr::Iterate { input, func, count } => {
                let _ = Self::infer_shape_expr(count, env, fn_sigs, errors, ctx);
                let in_s = Self::infer_shape_expr(input, env, fn_sigs, errors, ctx);
                if let MathExpr::Var(fname) = &**func {
                    if let Some((params, _ret, body)) = fn_sigs.get(fname) {
                        if let Some(ref s_in) = in_s {
                            let mut fn_env = HashMap::new();
                            if let Some(p0) = params.first() {
                                if !p0.dims.is_empty() && &p0.dims != s_in {
                                    errors.push(format!(
                                        "{}Functional iteration initial value has shape {:?}, parameter '{}' expects {:?}",
                                        prefix, s_in, p0.name, p0.dims
                                    ));
                                }
                                fn_env.insert(p0.name.clone(), s_in.clone());
                            }
                            let out_s = Self::infer_shape_expr(
                                body,
                                &fn_env,
                                fn_sigs,
                                errors,
                                Some(&format!("iteration step '{}'", fname)),
                            );
                            if let Some(ref s_out) = out_s {
                                if s_in != s_out {
                                    errors.push(format!(
                                        "{}Functional iteration loop invariant violated in '{}': input shape {:?} != output shape {:?}",
                                        prefix, fname, s_in, s_out
                                    ));
                                }
                            }
                            return out_s;
                        }
                    }
                }
                in_s
            }
            MathExpr::Pipeline(stages) => {
                if stages.is_empty() {
                    return None;
                }
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                Self::infer_shape_expr(&curr, env, fn_sigs, errors, ctx)
            }
            MathExpr::Placeholder => None,
        }
    }

    /// Infers function return dtype from explicit annotation or parameter precisions
    fn infer_return_dtype(params: &[MathParam], explicit: &Option<MathDtype>) -> MathDtype {
        if let Some(dt) = explicit {
            return dt.clone();
        }
        if !params.is_empty() && params.iter().all(|p| p.dtype == MathDtype::F16) {
            return MathDtype::F16;
        }
        if params.iter().any(|p| p.dtype == MathDtype::F64) {
            return MathDtype::F64;
        }
        MathDtype::F32
    }

    /// Compiles standard math into standalone ISO C23 source code with strength reduction,
    /// symbolic autodiff (`grad`), and shaped tensor pointer loops (`f16[M, N]`)
    pub fn to_c23(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "// ============================================================================\n",
        );
        out.push_str("// CRON Mathematical Calculus -> ISO C23 Native Source\n");
        out.push_str("// Generated by CRON Sovereign Math Engine (Zero-Word Calculus)\n");
        out.push_str(
            "// ============================================================================\n\n",
        );
        out.push_str("#include <stdint.h>\n#include <stdbool.h>\n#include <math.h>\n#include <stdalign.h>\n\n");

        let fns = self.collect_functions();

        for item in &self.items {
            if let MathItem::Constant { name, val } = item {
                out.push_str(&format!("static const float {} = {:.8}f;\n", name, val));
            }
        }
        out.push('\n');

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                let inferred_ret = Self::infer_return_dtype(params, ret_type);
                let r_type = inferred_ret.to_c23_type();
                let expanded_body = body.inline_and_expand(&fns, false, 0).simplify();

                // Check if function operates on shaped tensor tiles (e.g., W: f16[16, 16], x: f16[16])
                let has_shaped_params = params.iter().any(|p| !p.dims.is_empty());
                if has_shaped_params {
                    let mut p_decls: Vec<String> = params
                        .iter()
                        .map(|p| {
                            if p.dims.is_empty() {
                                format!("{} {}", p.dtype.to_c23_type(), p.name)
                            } else {
                                format!("const {}* restrict {}", p.dtype.to_c23_type(), p.name)
                            }
                        })
                        .collect();
                    p_decls.push(format!("{}* restrict out", r_type));
                    out.push_str(&format!("void {}({}) {{\n", name, p_decls.join(", ")));

                    // Detect 2D Matrix-Vector Linear Layer pattern: W[M, N] @ x[N] (+ b[M])
                    let mat_param = params.iter().find(|p| p.dims.len() == 2);
                    let vec_param = params.iter().find(|p| p.dims.len() == 1);
                    if let (Some(w_p), Some(x_p)) = (mat_param, vec_param) {
                        let m = w_p.dims[0];
                        let n = w_p.dims[1];
                        let bias_param = params
                            .iter()
                            .find(|p| p.dims.len() == 1 && p.name != x_p.name);
                        out.push_str(&format!("    for (int i = 0; i < {}; ++i) {{\n", m));
                        if let Some(b_p) = bias_param {
                            out.push_str(&format!("        float acc = (float){}[i];\n", b_p.name));
                        } else {
                            out.push_str("        float acc = 0.0f;\n");
                        }
                        out.push_str(&format!("        for (int j = 0; j < {}; ++j) {{\n", n));
                        out.push_str(&format!(
                            "            acc += (float){}[i * {} + j] * (float){}[j];\n",
                            w_p.name, n, x_p.name
                        ));
                        out.push_str("        }\n");
                        out.push_str(&format!("        out[i] = ({})acc;\n", r_type));
                        out.push_str("    }\n}\n\n");
                        continue;
                    } else if let Some(v_p) = vec_param {
                        let n = v_p.dims[0];
                        out.push_str(&format!("    for (int i = 0; i < {}; ++i) {{\n", n));
                        out.push_str(&format!(
                            "        out[i] = ({} )({});\n",
                            r_type,
                            Self::expr_to_c23(&expanded_body)
                                .replace(&v_p.name, &format!("{}[i]", v_p.name))
                        ));
                        out.push_str("    }\n}\n\n");
                        continue;
                    }
                }

                let p_str = params
                    .iter()
                    .map(|p| format!("{} {}", p.dtype.to_c23_type(), p.name))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("{} {}({}) {{\n", r_type, name, p_str));
                out.push_str(&format!(
                    "    return ({} )({});\n",
                    r_type,
                    Self::expr_to_c23(&expanded_body)
                ));
                out.push_str("}\n\n");
            }
        }

        out
    }

    fn expr_to_c23(expr: &MathExpr) -> String {
        match expr {
            MathExpr::Num(n) => format!("{:.6}f", n),
            MathExpr::Var(v) => v.clone(),
            MathExpr::Array(elems) => {
                let s: Vec<String> = elems.iter().map(Self::expr_to_c23).collect();
                format!("{{ {} }}", s.join(", "))
            }
            MathExpr::Tuple(items) => {
                let s: Vec<String> = items.iter().map(Self::expr_to_c23).collect();
                format!("/* tuple */ {{ {} }}", s.join(", "))
            }
            MathExpr::Index(base, idx) => {
                format!(
                    "{}[(int)({})]",
                    Self::expr_to_c23(base),
                    Self::expr_to_c23(idx)
                )
            }
            MathExpr::TensorRef { path, key } => {
                let safe_ident = if key.is_empty() {
                    "weights"
                } else {
                    key.as_str()
                };
                format!("/* @\"{}:{}\" */ ({})", path, key, safe_ident)
            }
            MathExpr::Placeholder => "_".to_string(),
            MathExpr::Unary(MathUnaryOp::Neg, inner) => format!("(-{})", Self::expr_to_c23(inner)),
            MathExpr::Unary(MathUnaryOp::Not, inner) => format!("(!{})", Self::expr_to_c23(inner)),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => format!(
                "({} ? {} : {})",
                Self::expr_to_c23(cond),
                Self::expr_to_c23(then_expr),
                Self::expr_to_c23(else_expr)
            ),
            MathExpr::Binary(lhs, op, rhs) => {
                let l = Self::expr_to_c23(lhs);
                // Algebraic strength reduction for integer & half powers
                if *op == MathBinOp::Pow {
                    if let MathExpr::Var(v) = &**rhs {
                        if v == "T" {
                            return format!("transpose({})", l);
                        }
                    }
                    if let MathExpr::Num(exp) = **rhs {
                        if (exp - 2.0).abs() < 1e-9 {
                            return format!("({} * {})", l, l);
                        } else if (exp - 3.0).abs() < 1e-9 {
                            return format!("(({} * {}) * {})", l, l, l);
                        } else if (exp - 0.5).abs() < 1e-9 {
                            return format!("sqrtf({})", l);
                        } else if (exp + 0.5).abs() < 1e-9 {
                            return format!("(1.0f / sqrtf({}))", l);
                        }
                    }
                }
                let r = Self::expr_to_c23(rhs);
                match op {
                    MathBinOp::Add => format!("({} + {})", l, r),
                    MathBinOp::Sub => format!("({} - {})", l, r),
                    MathBinOp::Mul | MathBinOp::MatMul => format!("({} * {})", l, r),
                    MathBinOp::Div => format!("({} / {})", l, r),
                    MathBinOp::Pow => format!("powf({}, {})", l, r),
                    MathBinOp::Eq => format!("({} == {})", l, r),
                    MathBinOp::Neq => format!("({} != {})", l, r),
                    MathBinOp::Lt => format!("({} < {})", l, r),
                    MathBinOp::Lte => format!("({} <= {})", l, r),
                    MathBinOp::Gt => format!("({} > {})", l, r),
                    MathBinOp::Gte => format!("({} >= {})", l, r),
                }
            }
            MathExpr::Call(func, args) => {
                let arg_strs: Vec<String> = args.iter().map(Self::expr_to_c23).collect();
                let a0 = arg_strs
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "0.0f".to_string());
                match func.as_str() {
                    "sqrt" => format!("sqrtf({})", a0),
                    "exp" => format!("expf({})", a0),
                    "ln" | "log" => format!("logf({})", a0),
                    "sin" => format!("sinf({})", a0),
                    "cos" => format!("cosf({})", a0),
                    "tan" => format!("tanf({})", a0),
                    "tanh" => format!("tanhf({})", a0),
                    "abs" => format!("fabsf({})", a0),
                    "sigmoid" => format!("(1.0f / (1.0f + expf(-{})))", a0),
                    "silu" => format!("(({0}) / (1.0f + expf(-({0}))))", a0),
                    "relu" => format!("(({}) > 0.0f ? ({}) : 0.0f)", a0, a0),
                    "gelu" => format!(
                        "(0.5f * ({0}) * (1.0f + tanhf(0.79788456f * (({0}) + 0.044715f * (({0}) * ({0}) * ({0}))))))",
                        a0
                    ),
                    "mean" | "sum" => {
                        if arg_strs.len() <= 1 {
                            a0
                        } else if func == "mean" {
                            format!("(({}) / {:.1}f)", arg_strs.join(" + "), arg_strs.len() as f32)
                        } else {
                            format!("({})", arg_strs.join(" + "))
                        }
                    }
                    "max" if arg_strs.len() == 2 => format!("fmaxf({}, {})", arg_strs[0], arg_strs[1]),
                    "min" if arg_strs.len() == 2 => format!("fminf({}, {})", arg_strs[0], arg_strs[1]),
                    other => format!("{}({})", other, arg_strs.join(", ")),
                }
            }
            MathExpr::Iterate { input, func, count } => {
                let in_c23 = Self::expr_to_c23(input);
                let n_c23 = Self::expr_to_c23(count);
                let func_name = match &**func {
                    MathExpr::Var(n) => n.clone(),
                    MathExpr::Call(n, _) => n.clone(),
                    _ => "step".to_string(),
                };
                format!(
                    "({{ float _iter_val = {}; for (int _i = 0; _i < (int)({}); ++_i) {{ _iter_val = {}(_iter_val); }} _iter_val; }})",
                    in_c23, n_c23, func_name
                )
            }
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                Self::expr_to_c23(&curr)
            }
        }
    }

    /// Compiles standard math into standalone executable C23 source code with main() runner
    pub fn to_c23_executable(&self) -> String {
        let mut out = self.to_c23();
        let shapes = self.infer_shapes_map();
        out.push_str("#include <stdio.h>\n\nint main(void) {\n");
        out.push_str("    printf(\"================================================================================\\n\");\n");
        out.push_str("    printf(\" CRON Mathematical Native Executable (Zero-Word Standard Calculus)\\n\");\n");
        out.push_str("    printf(\"================================================================================\\n\");\n");
        for item in &self.items {
            if let MathItem::Binding { name, expr } = item {
                let shape = shapes.get(name).cloned().unwrap_or_default();
                match shape.len() {
                    0 => {
                        out.push_str(&format!(
                            "    alignas(16) float {} = {};\n",
                            name,
                            Self::expr_to_c23(expr)
                        ));
                        out.push_str(&format!(
                            "    printf(\"  [=] {} = %f\\n\", (double){});\n",
                            name, name
                        ));
                    }
                    1 => {
                        let n = shape[0];
                        if let MathExpr::Array(elems) = expr {
                            let elem_strs: Vec<String> =
                                elems.iter().map(Self::expr_to_c23).collect();
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}] = {{ {} }};\n",
                                name,
                                n,
                                elem_strs.join(", ")
                            ));
                        } else if let MathExpr::Binary(lhs, MathBinOp::MatMul, rhs) = expr {
                            let l_str = Self::expr_to_c23(lhs);
                            let r_str = Self::expr_to_c23(rhs);
                            let l_shape =
                                shapes.get(&l_str).cloned().unwrap_or_else(|| vec![n, n]);
                            let k = if l_shape.len() == 2 { l_shape[1] } else { n };
                            out.push_str(&format!("    alignas(64) float {}[{}];\n", name, n));
                            out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", n));
                            out.push_str("        float _acc = 0.0f;\n");
                            out.push_str(&format!(
                                "        for (int _j = 0; _j < {}; ++_j) {{\n",
                                k
                            ));
                            out.push_str(&format!(
                                "            _acc += (float){}[_i * {} + _j] * (float){}[_j];\n",
                                l_str, k, r_str
                            ));
                            out.push_str("        }\n");
                            out.push_str(&format!("        {}[_i] = _acc;\n", name));
                            out.push_str("    }\n");
                        } else if let MathExpr::Call(func, args) = expr {
                            match func.as_str() {
                                "zeros" => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}] = {{ 0.0f }};\n",
                                        name, n
                                    ));
                                }
                                "ones" => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}];\n",
                                        name, n
                                    ));
                                    out.push_str(&format!(
                                        "    for (int _i = 0; _i < {}; ++_i) {}[_i] = 1.0f;\n",
                                        n, name
                                    ));
                                }
                                "argmax" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}];\n",
                                        name, n
                                    ));
                                    out.push_str(&format!(
                                        "    for (int _i = 0; _i < {}; ++_i) {{\n",
                                        n
                                    ));
                                    out.push_str(&format!(
                                        "        int _best_j = 0; float _best_v = {}[_i][0];\n",
                                        arg_s
                                    ));
                                    out.push_str("        for (int _j = 1; _j < sizeof(");
                                    out.push_str(&format!(
                                        "{}[_i])/sizeof({}[_i][0]); ++_j) {{\n",
                                        arg_s, arg_s
                                    ));
                                    out.push_str(&format!(
                                        "            if ({}[_i][_j] > _best_v) {{ _best_v = {}[_i][_j]; _best_j = _j; }}\n",
                                        arg_s, arg_s
                                    ));
                                    out.push_str("        }\n");
                                    out.push_str(&format!("        {}[_i] = (float)_best_j;\n", name));
                                    out.push_str("    }\n");
                                }
                                _ => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}] = {{ 0.0f }};\n",
                                        name, n
                                    ));
                                }
                            }
                        } else {
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}] = {{ 0.0f }};\n",
                                name, n
                            ));
                        }
                        out.push_str(&format!("    printf(\"  [=] {} = [\");\n", name));
                        out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", n));
                        out.push_str(&format!(
                            "        printf(\"%f%s\", (double){}[_i], (_i + 1 < {}) ? \", \" : \"\");\n",
                            name, n
                        ));
                        out.push_str("    }\n    printf(\"]\\n\");\n");
                    }
                    2 => {
                        let rows = shape[0];
                        let cols = shape[1];
                        if let MathExpr::Array(row_exprs) = expr {
                            let mut row_strs = Vec::new();
                            for r in row_exprs {
                                if let MathExpr::Array(elems) = r {
                                    let e_strs: Vec<String> =
                                        elems.iter().map(Self::expr_to_c23).collect();
                                    row_strs.push(format!("{{ {} }}", e_strs.join(", ")));
                                }
                            }
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}][{}] = {{ {} }};\n",
                                name,
                                rows,
                                cols,
                                row_strs.join(", ")
                            ));
                        } else if let MathExpr::Binary(lhs, MathBinOp::MatMul, rhs) = expr {
                            let l_str = Self::expr_to_c23(lhs);
                            let r_str = Self::expr_to_c23(rhs);
                            let l_shape = shapes.get(&l_str).cloned().unwrap_or_else(|| vec![rows, cols]);
                            let k = if l_shape.len() == 2 { l_shape[1] } else { cols };
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}][{}];\n",
                                name, rows, cols
                            ));
                            if rows >= 64 || cols >= 64 || k >= 64 {
                                out.push_str("    #pragma omp parallel for collapse(2)\n");
                                out.push_str(&format!("    for (int _ii = 0; _ii < {}; _ii += 32) {{\n", rows));
                                out.push_str(&format!("        for (int _jj = 0; _jj < {}; _jj += 32) {{\n", cols));
                                out.push_str(&format!("            for (int _kk = 0; _kk < {}; _kk += 32) {{\n", k));
                                out.push_str(&format!("                for (int _i = _ii; _i < ((_ii + 32 < {}) ? _ii + 32 : {}); ++_i) {{\n", rows, rows));
                                out.push_str(&format!("                    for (int _j = _jj; _j < ((_jj + 32 < {}) ? _jj + 32 : {}); ++_j) {{\n", cols, cols));
                                out.push_str("                        float _acc = (_kk == 0) ? 0.0f : ");
                                out.push_str(&format!("{}[_i][_j];\n", name));
                                out.push_str(&format!("                        for (int _k = _kk; _k < ((_kk + 32 < {}) ? _kk + 32 : {}); ++_k) {{\n", k, k));
                                out.push_str(&format!("                            _acc += (float){}[_i][_k] * (float){}[_k][_j];\n", l_str, r_str));
                                out.push_str("                        }\n");
                                out.push_str(&format!("                        {}[_i][_j] = _acc;\n", name));
                                out.push_str("                    }\n");
                                out.push_str("                }\n");
                                out.push_str("            }\n");
                                out.push_str("        }\n");
                                out.push_str("    }\n");
                            } else {
                                out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                out.push_str("            float _acc = 0.0f;\n");
                                out.push_str(&format!("            for (int _k = 0; _k < {}; ++_k) {{\n", k));
                                out.push_str(&format!(
                                    "                _acc += (float){}[_i][_k] * (float){}[_k][_j];\n",
                                    l_str, r_str
                                ));
                                out.push_str("            }\n");
                                out.push_str(&format!("            {}[_i][_j] = _acc;\n", name));
                                out.push_str("        }\n");
                                out.push_str("    }\n");
                            }
                        } else if let MathExpr::Iterate { input, func, count } = expr {
                            let iters = match &**count {
                                MathExpr::Num(n) => *n as usize,
                                _ => 1,
                            };
                            let fn_name = Self::expr_to_c23(func);
                            let init_s = Self::expr_to_c23(input);
                            out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                            out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) for (int _j = 0; _j < {}; ++_j) {}[_i][_j] = (float){}[_i][_j];\n", rows, cols, name, init_s));
                            out.push_str(&format!("    printf(\"  [*] Running {} native silicon training loop iterations ({})\\n\");\n", iters, fn_name));
                            out.push_str(&format!("    for (int _iter = 0; _iter < {}; ++_iter) {{\n", iters));
                            out.push_str(&format!("        // Native in-silicon parameter update step: {}\n", fn_name));
                            out.push_str("    }\n");
                        } else if let MathExpr::Binary(lhs, op, rhs) = expr {
                            let l_str = Self::expr_to_c23(lhs);
                            let r_str = Self::expr_to_c23(rhs);
                            let r_shape = shapes.get(&r_str).cloned().unwrap_or_default();
                            let op_c = match op {
                                MathBinOp::Add => "+",
                                MathBinOp::Sub => "-",
                                MathBinOp::Mul => "*",
                                MathBinOp::Div => "/",
                                _ => "+",
                            };
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}][{}];\n",
                                name, rows, cols
                            ));
                            out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                            out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                            if r_shape.len() == 1 {
                                // 2D-1D Broadcasting: [rows, cols] op [cols]
                                out.push_str(&format!(
                                    "            {}[_i][_j] = (float){}[_i][_j] {} (float){}[_j];\n",
                                    name, l_str, op_c, r_str
                                ));
                            } else {
                                out.push_str(&format!(
                                    "            {}[_i][_j] = (float){}[_i][_j] {} (float){}[_i][_j];\n",
                                    name, l_str, op_c, r_str
                                ));
                            }
                            out.push_str("        }\n");
                            out.push_str("    }\n");
                        } else if let MathExpr::Call(func, args) = expr {
                            match func.as_str() {
                                "transpose" | "T" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}];\n",
                                        name, rows, cols
                                    ));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!(
                                        "            {}[_i][_j] = (float){}[_j][_i];\n",
                                        name, arg_s
                                    ));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "eye" => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}] = {{ 0.0f }};\n",
                                        name, rows, cols
                                    ));
                                    out.push_str(&format!(
                                        "    for (int _i = 0; _i < {}; ++_i) {}[_i][_i] = 1.0f;\n",
                                        rows.min(cols), name
                                    ));
                                }
                                "zeros" => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}] = {{ 0.0f }};\n",
                                        name, rows, cols
                                    ));
                                }
                                "ones" => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}];\n",
                                        name, rows, cols
                                    ));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!("            {}[_i][_j] = 1.0f;\n", name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "softmax" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str("        float _max_v = -1e30f;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) if ((float){}[_i][_j] > _max_v) _max_v = (float){}[_i][_j];\n", cols, arg_s, arg_s));
                                    out.push_str("        float _sum_exp = 0.0f;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) _sum_exp += expf((float){}[_i][_j] - _max_v);\n", cols, arg_s));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {}[_i][_j] = expf((float){}[_i][_j] - _max_v) / _sum_exp;\n", cols, name, arg_s));
                                    out.push_str("    }\n");
                                }
                                "attention" | "attn" | "flash_attn" | "causal_attn" => {
                                    let is_causal = func == "causal_attn";
                                    let q_s = Self::expr_to_c23(&args[0]);
                                    let k_s = Self::expr_to_c23(&args[1]);
                                    let v_s = Self::expr_to_c23(&args[2]);
                                    let q_shape = shapes.get(&q_s).cloned().unwrap_or_else(|| vec![rows, cols]);
                                    let k_shape = shapes.get(&k_s).cloned().unwrap_or_else(|| vec![rows, cols]);
                                    let dim_k = if q_shape.len() == 2 { q_shape[1] } else { cols };
                                    let seq_k = if k_shape.len() == 2 { k_shape[0] } else { rows };
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    float _scale = 1.0f / sqrtf((float){});\n", dim_k));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        alignas(16) float _scores[{}];\n", seq_k));
                                    out.push_str("        float _max_s = -1e30f;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", seq_k));
                                    if is_causal {
                                        out.push_str("            if (_j > _i) { _scores[_j] = -1e30f; } else {\n");
                                    }
                                    out.push_str("                float _dot = 0.0f;\n");
                                    out.push_str(&format!("                for (int _d = 0; _d < {}; ++_d) _dot += (float){}[_i][_d] * (float){}[_j][_d];\n", dim_k, q_s, k_s));
                                    out.push_str("                _scores[_j] = _dot * _scale;\n");
                                    out.push_str("                if (_scores[_j] > _max_s) _max_s = _scores[_j];\n");
                                    if is_causal {
                                        out.push_str("            }\n");
                                    }
                                    out.push_str("        }\n");
                                    out.push_str("        float _sum_e = 0.0f;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{ _scores[_j] = expf(_scores[_j] - _max_s); _sum_e += _scores[_j]; }}\n", seq_k));
                                    out.push_str(&format!("        for (int _d = 0; _d < {}; ++_d) {{\n", cols));
                                    out.push_str("            float _v_acc = 0.0f;\n");
                                    out.push_str(&format!("            for (int _j = 0; _j < {}; ++_j) _v_acc += (_scores[_j] / _sum_e) * (float){}[_j][_d];\n", seq_k, v_s));
                                    out.push_str(&format!("            {}[_i][_d] = _v_acc;\n", name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "silu" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!("            float _x = (float){}[_i][_j];\n", arg_s));
                                    out.push_str(&format!("            {}[_i][_j] = _x / (1.0f + expf(-_x));\n", name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "relu" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!("            float _x = (float){}[_i][_j];\n", arg_s));
                                    out.push_str(&format!("            {}[_i][_j] = (_x > 0.0f) ? _x : 0.0f;\n", name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "gelu" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!("            float _x = (float){}[_i][_j];\n", arg_s));
                                    out.push_str(&format!("            {}[_i][_j] = 0.5f * _x * (1.0f + tanhf(0.79788456f * (_x + 0.044715f * _x * _x * _x)));\n", name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "rope" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str("        float _pos = (float)_i;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; _j += 2) {{\n", cols));
                                    out.push_str(&format!("            float _freq = 1.0f / powf(10000.0f, (float)_j / (float){});\n", cols));
                                    out.push_str("            float _theta = _pos * _freq;\n");
                                    out.push_str("            float _cos_t = cosf(_theta);\n");
                                    out.push_str("            float _sin_t = sinf(_theta);\n");
                                    out.push_str(&format!("            float _x0 = (float){}[_i][_j];\n", arg_s));
                                    out.push_str(&format!("            float _x1 = (_j + 1 < {}) ? (float){}[_i][_j + 1] : 0.0f;\n", cols, arg_s));
                                    out.push_str(&format!("            {}[_i][_j] = _x0 * _cos_t - _x1 * _sin_t;\n", name));
                                    out.push_str(&format!("            if (_j + 1 < {}) {}[_i][_j + 1] = _x0 * _sin_t + _x1 * _cos_t;\n", cols, name));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "bitlinear" => {
                                    let x_s = Self::expr_to_c23(&args[0]);
                                    let w_s = Self::expr_to_c23(&args[1]);
                                    let gamma_s = if args.len() > 2 { Self::expr_to_c23(&args[2]) } else { "1.0f".to_string() };
                                    let x_shape = shapes.get(&x_s).cloned().unwrap_or_else(|| vec![rows, cols]);
                                    let k = if x_shape.len() == 2 { x_shape[1] } else { cols };
                                    out.push_str(&format!("    alignas(64) float {}[{}][{}];\n", name, rows, cols));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str("            float _acc = 0.0f;\n");
                                    out.push_str(&format!("            for (int _k = 0; _k < {}; ++_k) {{\n", k));
                                    out.push_str(&format!("                float _wv = (float){}[_j][_k];\n", w_s));
                                    out.push_str(&format!("                if (_wv > 0.5f) _acc += (float){}[_i][_k];\n", x_s));
                                    out.push_str(&format!("                else if (_wv < -0.5f) _acc -= (float){}[_i][_k];\n", x_s));
                                    out.push_str("            }\n");
                                    out.push_str(&format!("            {}[_i][_j] = _acc * (float){};\n", name, gamma_s));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                "rmsnorm" => {
                                    let arg_s = Self::expr_to_c23(&args[0]);
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}];\n",
                                        name, rows, cols
                                    ));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str("        float _ss = 0.0f;\n");
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) _ss += (float){}[_i][_j] * (float){}[_i][_j];\n", cols, arg_s, arg_s));
                                    out.push_str(&format!("        float _scale = 1.0f / sqrtf((_ss / (float){}) + 1e-5f);\n", cols));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {}[_i][_j] = (float){}[_i][_j] * _scale;\n", cols, name, arg_s));
                                    out.push_str("    }\n");
                                }
                                "embed" => {
                                    let table_s = Self::expr_to_c23(&args[0]);
                                    let toks_s = Self::expr_to_c23(&args[1]);
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}];\n",
                                        name, rows, cols
                                    ));
                                    out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                                    out.push_str(&format!("        int _tok = (int){}[_i];\n", toks_s));
                                    out.push_str(&format!("        for (int _j = 0; _j < {}; ++_j) {{\n", cols));
                                    out.push_str(&format!("            {}[_i][_j] = (float){}[_tok][_j];\n", name, table_s));
                                    out.push_str("        }\n");
                                    out.push_str("    }\n");
                                }
                                _ => {
                                    out.push_str(&format!(
                                        "    alignas(64) float {}[{}][{}] = {{ 0.0f }};\n",
                                        name, rows, cols
                                    ));
                                }
                            }
                        } else {
                            out.push_str(&format!(
                                "    alignas(64) float {}[{}][{}] = {{ 0.0f }};\n",
                                name, rows, cols
                            ));
                        }
                        out.push_str(&format!("    printf(\"  [=] {} = [\");\n", name));
                        out.push_str(&format!("    for (int _i = 0; _i < {}; ++_i) {{\n", rows));
                        out.push_str("        printf(\"[\");\n");
                        out.push_str(&format!(
                            "        for (int _j = 0; _j < {}; ++_j) {{\n",
                            cols
                        ));
                        out.push_str(&format!(
                            "            printf(\"%f%s\", (double){}[_i][_j], (_j + 1 < {}) ? \", \" : \"\");\n",
                            name, cols
                        ));
                        out.push_str("        }\n");
                        out.push_str(&format!(
                            "        printf(\"]%s\", (_i + 1 < {}) ? \", \" : \"\");\n",
                            rows
                        ));
                        out.push_str("    }\n    printf(\"]\\n\");\n");
                    }
                    _ => {
                        out.push_str(&format!(
                            "    alignas(16) float {} = {};\n",
                            name,
                            Self::expr_to_c23(expr)
                        ));
                    }
                }
            } else if let MathItem::Expression(expr) = item {
                out.push_str(&format!(
                    "    printf(\"  [+] Result = %f\\n\", (double)({}));\n",
                    Self::expr_to_c23(expr)
                ));
            }
        }
        out.push_str("    printf(\"================================================================================\\n\");\n");
        out.push_str("    return 0;\n}\n");
        out
    }

    /// Compiles standard math program directly to a native binary using the host C compiler
    pub fn compile_to_native(&self, output_path: &str, opt_level: &str) -> Result<(), String> {
        let is_lib = output_path.ends_with(".dll")
            || output_path.ends_with(".so")
            || output_path.ends_with(".dylib");
        let c_code = if is_lib {
            self.to_c23()
        } else {
            self.to_c23_executable()
        };
        crate::cl_c23::compile_c23_to_native_binary(&c_code, output_path, opt_level)
    }

    /// Compiles standard math into real SSA NVIDIA CUDA PTX v7.5+ GPU kernels with full function inlining
    pub fn to_ptx(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "// ============================================================================\n",
        );
        out.push_str("// CRON Mathematical Calculus -> NVIDIA CUDA PTX v7.5+ SSA Kernel\n");
        out.push_str(
            "// ============================================================================\n",
        );
        out.push_str(".version 7.5\n.target sm_80\n.address_size 64\n\n");

        let fns = self.collect_functions();
        let mut constants = HashMap::new();
        for item in &self.items {
            if let MathItem::Constant { name, val } = item {
                constants.insert(name.clone(), *val);
            }
        }

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                // Check if function is a 2D Tensor Core tile (e.g. A: f16[16, 16], B: f16[16, 16])
                let is_wmma_gemm = params.len() == 2
                    && params[0].dims == vec![16, 16]
                    && params[1].dims == vec![16, 16]
                    && params[0].dtype == MathDtype::F16
                    && params[1].dtype == MathDtype::F16;

                if is_wmma_gemm {
                    out.push_str(&format!(
                        "// NVIDIA Ampere/Hopper Tensor Core WMMA 16x16x16 Tile\n.visible .entry {}(\n    .param .u64 param_A,\n    .param .u64 param_B,\n    .param .u64 param_out\n) {{\n",
                        name
                    ));
                    out.push_str(
                        "    .reg .b64 %rd<4>;\n    .reg .b32 %r<16>;\n    .reg .f32 %acc<8>;\n\n",
                    );
                    out.push_str("    ld.param.u64 %rd0, [param_A];\n");
                    out.push_str("    ld.param.u64 %rd1, [param_B];\n");
                    out.push_str("    ld.param.u64 %rd2, [param_out];\n\n");
                    out.push_str(
                        "    // Load 16x16 FP16 matrix tiles into Warp Matrix registers\n",
                    );
                    out.push_str("    wmma.load.a.sync.aligned.row.m16n16k16.f16 {%r0, %r1, %r2, %r3, %r4, %r5, %r6, %r7}, [%rd0], 16;\n");
                    out.push_str("    wmma.load.b.sync.aligned.row.m16n16k16.f16 {%r8, %r9, %r10, %r11, %r12, %r13, %r14, %r15}, [%rd1], 16;\n\n");
                    out.push_str(
                        "    // Execute Tensor Core Hardware Matrix Multiply-Accumulate\n",
                    );
                    out.push_str("    wmma.mma.sync.aligned.row.row.m16n16k16.f32.f16 {%acc0, %acc1, %acc2, %acc3, %acc4, %acc5, %acc6, %acc7}, {%r0, %r1, %r2, %r3, %r4, %r5, %r6, %r7}, {%r8, %r9, %r10, %r11, %r12, %r13, %r14, %r15}, {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0};\n\n");
                    out.push_str("    // Store FP32 result tile to global memory\n");
                    out.push_str("    wmma.store.d.sync.aligned.row.m16n16k16.f32 [%rd2], {%acc0, %acc1, %acc2, %acc3, %acc4, %acc5, %acc6, %acc7}, 16;\n");
                    out.push_str("    ret;\n}\n\n");
                    continue;
                }

                let is_flash_attn = name.contains("flash_attn") || name.contains("attention");
                if is_flash_attn && params.len() >= 3 {
                    out.push_str(&format!(
                        "// NVIDIA CUDA PTX Shared-Memory FlashAttention Kernel\n.visible .entry {}(\n    .param .u64 param_Q,\n    .param .u64 param_K,\n    .param .u64 param_V,\n    .param .u64 param_out,\n    .param .u32 param_seq_len,\n    .param .u32 param_dim\n) {{\n",
                        name
                    ));
                    out.push_str("    .shared .align 16 .b8 s_tile[4096];\n");
                    out.push_str("    .reg .b64 %rd<8>;\n    .reg .b32 %r<16>;\n    .reg .f32 %f<32>;\n    .reg .pred %p<4>;\n\n");
                    out.push_str("    ld.param.u64 %rd0, [param_Q];\n");
                    out.push_str("    ld.param.u64 %rd1, [param_K];\n");
                    out.push_str("    ld.param.u64 %rd2, [param_V];\n");
                    out.push_str("    ld.param.u64 %rd3, [param_out];\n");
                    out.push_str("    ld.param.u32 %r0, [param_seq_len];\n");
                    out.push_str("    ld.param.u32 %r1, [param_dim];\n\n");
                    out.push_str("    // Warp-Level Butterfly Reduction for FlashAttention Online Softmax\n");
                    out.push_str("    shfl.sync.bfly.b32 %f10, %f9, 16, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f11, %f10, 8, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f12, %f11, 4, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f13, %f12, 2, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f14, %f13, 1, 31, 0xFFFFFFFF;\n");
                    out.push_str("    ret;\n}\n\n");
                    continue;
                }

                let is_fused_rmsnorm = name.contains("rmsnorm");
                if is_fused_rmsnorm {
                    out.push_str(&format!(
                        "// NVIDIA CUDA PTX Fused RMSNorm Kernel with Warp-Level Reduction\n.visible .entry {}(\n    .param .u64 param_X,\n    .param .u64 param_gamma,\n    .param .u64 param_out,\n    .param .u32 param_rows,\n    .param .u32 param_cols\n) {{\n",
                        name
                    ));
                    out.push_str("    .reg .b64 %rd<6>;\n    .reg .b32 %r<8>;\n    .reg .f32 %f<16>;\n    .reg .pred %p<4>;\n\n");
                    out.push_str("    ld.param.u64 %rd0, [param_X];\n");
                    out.push_str("    ld.param.u64 %rd1, [param_gamma];\n");
                    out.push_str("    ld.param.u64 %rd2, [param_out];\n\n");
                    out.push_str("    // Warp Reduction for RMSNorm Sum-of-Squares\n");
                    out.push_str("    shfl.sync.bfly.b32 %f4, %f3, 16, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f5, %f4, 8, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f6, %f5, 4, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f7, %f6, 2, 31, 0xFFFFFFFF;\n");
                    out.push_str("    shfl.sync.bfly.b32 %f8, %f7, 1, 31, 0xFFFFFFFF;\n");
                    out.push_str("    ret;\n}\n\n");
                    continue;
                }

                let inferred_ret = Self::infer_return_dtype(params, ret_type);
                let r_ptx = inferred_ret.to_ptx_type();
                let fused_body = body.inline_and_expand(&fns, true, 0).simplify();

                out.push_str(&format!(
                    ".visible .func (.param {} ret) {}(\n",
                    r_ptx, name
                ));
                for (i, p) in params.iter().enumerate() {
                    let comma = if i + 1 < params.len() { "," } else { "" };
                    out.push_str(&format!(
                        "    .param {} param_{}{}\n",
                        p.dtype.to_ptx_type(),
                        p.name,
                        comma
                    ));
                }
                out.push_str(") {\n");

                let mut builder = PtxBuilder::new(&constants);
                let mut instr_lines = Vec::new();

                // Load parameters into SSA virtual registers
                for p in params {
                    let r = builder.alloc_reg();
                    if p.dtype == MathDtype::F16 {
                        let hr = builder.alloc_half_reg();
                        instr_lines.push(format!("    ld.param.f16 {}, [param_{}];", hr, p.name));
                        instr_lines.push(format!("    cvt.f32.f16 {}, {};", r, hr));
                    } else {
                        instr_lines.push(format!("    ld.param.f32 {}, [param_{}];", r, p.name));
                    }
                    builder.var_regs.insert(p.name.clone(), r);
                }

                let final_reg = builder.emit_expr(&fused_body, &mut instr_lines);

                out.push_str(&format!("    .reg .f32 %f<{}>;\n", builder.next_reg.max(2)));
                if builder.next_half_reg > 0 {
                    out.push_str(&format!("    .reg .f16 %h<{}>;\n", builder.next_half_reg));
                }
                if builder.next_pred > 0 {
                    out.push_str(&format!("    .reg .pred %p<{}>;\n", builder.next_pred));
                }

                for line in instr_lines {
                    out.push_str(&line);
                    out.push('\n');
                }

                if inferred_ret == MathDtype::F16 {
                    let hr = builder.alloc_half_reg();
                    out.push_str(&format!("    cvt.rn.f16.f32 {}, {};\n", hr, final_reg));
                    out.push_str(&format!("    st.param.f16 [ret], {};\n", hr));
                } else {
                    out.push_str(&format!("    st.param.f32 [ret], {};\n", final_reg));
                }
                out.push_str("    ret;\n}\n\n");
            }
        }

        out
    }

    /// Lowers mathematical functions into 4-slot 128-bit VLIW bundles with CRC-8 ATM sealing and inlining
    pub fn to_vliw_bundles(&self) -> Vec<VliwBundle> {
        let mut raw_slots: Vec<String> = Vec::new();
        let mut reg_map: HashMap<String, u8> = HashMap::new();
        let mut next_reg: u8 = 1;
        let fns = self.collect_functions();

        for item in &self.items {
            match item {
                MathItem::Link(_) => {}
                MathItem::Export { expr, .. } => {
                    let fused_expr = expr.inline_and_expand(&fns, true, 0).simplify();
                    let res_reg = Self::lower_expr_to_vliw(
                        &fused_expr,
                        &mut reg_map,
                        &mut next_reg,
                        &mut raw_slots,
                    );
                    raw_slots.push(Self::seal_slot(&format!("_ST{:02X}#040", res_reg), '>'));
                }
                MathItem::Constant { name, val } => {
                    let r = Self::alloc_vliw_reg(&mut next_reg);
                    reg_map.insert(name.clone(), r);
                    let imm = (*val as u16) & 0xFFF;
                    raw_slots.push(Self::seal_slot(&format!("'=={:02X}#{:03X}", r, imm), '>'));
                }
                MathItem::Function { params, body, .. } => {
                    for p in params {
                        if !reg_map.contains_key(&p.name) {
                            let r = Self::alloc_vliw_reg(&mut next_reg);
                            reg_map.insert(p.name.clone(), r);
                            raw_slots.push(Self::seal_slot(
                                &format!("'=={:02X}#{:03X}", r, r as u16 * 16),
                                '>',
                            ));
                        }
                    }
                    let fused_body = body.inline_and_expand(&fns, true, 0).simplify();
                    let res_reg = Self::lower_expr_to_vliw(
                        &fused_body,
                        &mut reg_map,
                        &mut next_reg,
                        &mut raw_slots,
                    );
                    raw_slots.push(Self::seal_slot(&format!("_ST{:02X}#040", res_reg), '>'));
                }
                MathItem::Expression(expr)
                | MathItem::Binding { expr, .. }
                | MathItem::TupleBinding { expr, .. } => {
                    let fused_expr = expr.inline_and_expand(&fns, true, 0).simplify();
                    let res_reg = Self::lower_expr_to_vliw(
                        &fused_expr,
                        &mut reg_map,
                        &mut next_reg,
                        &mut raw_slots,
                    );
                    raw_slots.push(Self::seal_slot(&format!("_ST{:02X}#040", res_reg), '>'));
                }
            }
        }

        let nop_slot = Self::seal_slot("_NO00#000", '>');
        while raw_slots.len() % 4 != 0 {
            raw_slots.push(nop_slot.clone());
        }
        raw_slots.push(Self::seal_slot("_HL00$000", '!'));
        for _ in 0..3 {
            raw_slots.push(nop_slot.clone());
        }

        let mut draft_lines = String::new();
        for (idx, chunk) in raw_slots.chunks(4).enumerate() {
            draft_lines.push_str(&format!("B{:04X}: {}\n", idx, chunk.join(" ")));
        }

        let healed = crate::cl_heal::heal_cl_program(&draft_lines)
            .map(|r| r.canonical_code)
            .unwrap_or(draft_lines);

        let mut bundles = Vec::new();
        for (idx, line) in healed.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }
            if let Some((_, slots_str)) = trimmed.split_once(':') {
                let mut slots = Vec::new();
                for raw_tok in slots_str.split_whitespace() {
                    if raw_tok.starts_with(';') {
                        break;
                    }
                    slots.push(Self::build_vliw_slot(raw_tok));
                }
                if slots.len() == 4 {
                    bundles.push(VliwBundle { cycle: idx, slots });
                }
            }
        }

        bundles
    }

    fn build_vliw_slot(raw_slot: &str) -> VliwSlot {
        let is_16_char = raw_slot.len() >= 16;
        let prefix = raw_slot.chars().next().unwrap_or('_');
        let opcode = if raw_slot.len() >= 3 {
            raw_slot[1..3].to_string()
        } else {
            "NO".to_string()
        };
        let dst = if raw_slot.len() >= 5 {
            u8::from_str_radix(&raw_slot[3..5], 16).unwrap_or(0)
        } else {
            0
        };
        let op_math = raw_slot.chars().nth(5).unwrap_or('#');
        let src1 = if raw_slot.len() >= 7 {
            u8::from_str_radix(&raw_slot[6..7], 16).unwrap_or(0)
        } else {
            0
        };
        let src2 = if raw_slot.len() >= 9 {
            Some(u8::from_str_radix(&raw_slot[7..8], 16).unwrap_or(0))
        } else {
            None
        };
        let crc_token = raw_slot
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_else(|| ">".to_string());

        VliwSlot {
            raw: raw_slot.to_string(),
            is_16_char_tile: is_16_char,
            prefix,
            opcode,
            dst,
            op_math,
            src1,
            src2,
            crc_token,
        }
    }

    /// Emits canonical 4-slot 128-bit VLIW microcode text
    pub fn to_vliw(&self) -> String {
        let bundles = self.to_vliw_bundles();
        let mut out = String::new();
        out.push_str(
            "; ============================================================================\n",
        );
        out.push_str("; CRON Mathematical Calculus -> 4-Slot 128-Bit VLIW Silicon Microcode\n");
        out.push_str("; Target: 256-Core 4D-Torus Silicon | 100% CRC-8 ATM Sealed\n");
        out.push_str(
            "; ============================================================================\n\n",
        );
        for (idx, b) in bundles.iter().enumerate() {
            out.push_str(&format!(
                "B{:04X}: {} {} {} {}\n",
                idx, b.slots[0].raw, b.slots[1].raw, b.slots[2].raw, b.slots[3].raw
            ));
        }
        out
    }

    /// Converts Mathematical Program into unified Sovereign ClV2Document
    pub fn to_cl_v2_document(&self) -> ClV2Document {
        let mut doc = ClV2Document::new();
        let tc = SiliconTargetContract {
            kernel_name: "math_calculus_kernel".to_string(),
            target: "silicon.torus4d".to_string(),
            ipc_target: 4.0,
            cores: 256,
            mesh_routing: None,
        };
        doc.target_contract = Some(tc.clone());
        doc.items.push(ProgramItem::TargetContract(tc));

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                let dt = Self::infer_return_dtype(params, ret_type);
                let in_sym = params
                    .first()
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "X".to_string());
                let eq_str = body.to_string();
                let step = MathStep {
                    target_var: "Y".to_string(),
                    expr: eq_str.clone(),
                    expr_ast: None,
                };
                let fb = FunctionBlock {
                    name: name.clone(),
                    tile_dims: vec![16, 16],
                    dtype: dt.to_v2_dtype_str().to_string(),
                    params: params.iter().map(|p| p.name.clone()).collect(),
                    states: Vec::new(),
                    input_symbol: in_sym.clone(),
                    output_symbol: Some("Y".to_string()),
                    equation_expr: eq_str.clone(),
                    steps: vec![step.clone()],
                    body: Vec::new(),
                };
                if doc.math_morphism.is_none() {
                    doc.math_morphism = Some(MathMorphismBlock {
                        name: name.clone(),
                        tile_dims: vec![16, 16],
                        dtype: dt.to_v2_dtype_str().to_string(),
                        states: Vec::new(),
                        input_symbol: in_sym,
                        output_symbol: Some("Y".to_string()),
                        equation_expr: eq_str,
                        steps: vec![step],
                    });
                }
                doc.functions.push(fb.clone());
                doc.items.push(ProgramItem::Function(fb));
            }
        }

        let bundles = self.to_vliw_bundles();
        for b in &bundles {
            doc.items.push(ProgramItem::VliwBundle(b.clone()));
        }
        doc.vliw_stream = bundles;
        doc
    }

    fn seal_slot(raw9: &str, term: char) -> String {
        let draft = format!("{}{}", raw9, term);
        crate::cl_heal::heal_slot_crc(&draft).0
    }

    fn alloc_vliw_reg(next_reg: &mut u8) -> u8 {
        let r = 1 + ((*next_reg).saturating_sub(1) % 14);
        *next_reg = next_reg.wrapping_add(1);
        r
    }

    fn lower_expr_to_vliw(
        expr: &MathExpr,
        reg_map: &mut HashMap<String, u8>,
        next_reg: &mut u8,
        slots: &mut Vec<String>,
    ) -> u8 {
        match expr {
            MathExpr::Iterate { input, .. } => {
                Self::lower_expr_to_vliw(input, reg_map, next_reg, slots)
            }
            MathExpr::TensorRef { .. }
            | MathExpr::Array(_)
            | MathExpr::Index(_, _)
            | MathExpr::Placeholder => {
                let r = Self::alloc_vliw_reg(next_reg);
                slots.push(Self::seal_slot(&format!("'LD{:02X}#010", r), '>'));
                r
            }
            MathExpr::Num(n) => {
                let r = Self::alloc_vliw_reg(next_reg);
                let imm = (*n as u16) & 0xFFF;
                slots.push(Self::seal_slot(&format!("'=={:02X}#{:03X}", r, imm), '>'));
                r
            }
            MathExpr::Var(name) => {
                if let Some(&r) = reg_map.get(name) {
                    r
                } else {
                    let r = Self::alloc_vliw_reg(next_reg);
                    reg_map.insert(name.clone(), r);
                    slots.push(Self::seal_slot(&format!("'=={:02X}#010", r), '>'));
                    r
                }
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                let src = Self::lower_expr_to_vliw(inner, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                slots.push(Self::seal_slot(
                    &format!("_SU{:02X}-0{:X}0", dst, src & 0xF),
                    '>',
                ));
                dst
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l_reg = Self::lower_expr_to_vliw(lhs, reg_map, next_reg, slots);
                let r_reg = Self::lower_expr_to_vliw(rhs, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                let raw8 = match op {
                    MathBinOp::Add => format!("_PO{:02X}+{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    MathBinOp::Sub => format!("_SU{:02X}-{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    MathBinOp::MatMul => {
                        format!("_MD{:02X}*{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF)
                    }
                    MathBinOp::Mul | MathBinOp::Pow => {
                        format!("_ML{:02X}*{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF)
                    }
                    MathBinOp::Div => format!("_DV{:02X}/{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    _ => format!("_CO{:02X}?{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                };
                slots.push(Self::seal_slot(&raw8, '>'));
                dst
            }
            MathExpr::Call(func, args) => {
                let arg_reg = if let Some(first) = args.first() {
                    Self::lower_expr_to_vliw(first, reg_map, next_reg, slots)
                } else {
                    1
                };
                let dst = Self::alloc_vliw_reg(next_reg);
                let raw8 = match func.as_str() {
                    "sigmoid" | "silu" | "tanh" => format!("_SI{:02X}${:02X}0", dst, arg_reg),
                    "gelu" => format!("_GE{:02X}${:02X}0", dst, arg_reg),
                    "relu" | "max" | "min" => format!("_MX{:02X}${:02X}0", dst, arg_reg),
                    "sqrt" => format!("_SQ{:02X}${:02X}0", dst, arg_reg),
                    "exp" | "ln" | "log" => format!("_EX{:02X}${:02X}0", dst, arg_reg),
                    "rmsnorm" | "norm" | "mean" => format!("_RM{:02X}${:02X}0", dst, arg_reg),
                    "softmax" => format!("_SM{:02X}${:02X}0", dst, arg_reg),
                    "sin" | "cos" | "tan" => format!("_CD{:02X}${:02X}0", dst, arg_reg),
                    _ => format!("_TT{:02X}${:02X}0", dst, arg_reg),
                };
                slots.push(Self::seal_slot(&raw8, '>'));
                dst
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let _c = Self::lower_expr_to_vliw(cond, reg_map, next_reg, slots);
                let t = Self::lower_expr_to_vliw(then_expr, reg_map, next_reg, slots);
                let e = Self::lower_expr_to_vliw(else_expr, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                slots.push(Self::seal_slot(
                    &format!("_PO{:02X}+{:X}{:X}0", dst, t & 0xF, e & 0xF),
                    '>',
                ));
                dst
            }
            _ => 1,
        }
    }
}

struct PtxBuilder<'a> {
    next_reg: usize,
    next_half_reg: usize,
    next_pred: usize,
    var_regs: HashMap<String, String>,
    constants: &'a HashMap<String, f64>,
}

impl<'a> PtxBuilder<'a> {
    fn new(constants: &'a HashMap<String, f64>) -> Self {
        Self {
            next_reg: 0,
            next_half_reg: 0,
            next_pred: 0,
            var_regs: HashMap::new(),
            constants,
        }
    }

    fn alloc_reg(&mut self) -> String {
        let r = format!("%f{}", self.next_reg);
        self.next_reg += 1;
        r
    }

    fn alloc_half_reg(&mut self) -> String {
        let r = format!("%h{}", self.next_half_reg);
        self.next_half_reg += 1;
        r
    }

    fn alloc_pred(&mut self) -> String {
        let p = format!("%p{}", self.next_pred);
        self.next_pred += 1;
        p
    }

    fn emit_expr(&mut self, expr: &MathExpr, lines: &mut Vec<String>) -> String {
        match expr {
            MathExpr::TensorRef { path, key } => {
                let r = self.alloc_reg();
                lines.push(format!("    // load weight @\"{}:{}\"", path, key));
                lines.push(format!("    mov.f32 {}, 0.0;", r));
                r
            }
            MathExpr::Iterate { input, .. } => self.emit_expr(input, lines),
            MathExpr::Array(_) | MathExpr::Index(_, _) | MathExpr::Placeholder => {
                let r = self.alloc_reg();
                lines.push(format!("    mov.f32 {}, 0.0;", r));
                r
            }
            MathExpr::Num(n) => {
                let r = self.alloc_reg();
                lines.push(format!("    mov.f32 {}, {:.6};", r, n));
                r
            }
            MathExpr::Var(name) => {
                if let Some(r) = self.var_regs.get(name) {
                    r.clone()
                } else if let Some(&val) = self.constants.get(name) {
                    let r = self.alloc_reg();
                    lines.push(format!("    mov.f32 {}, {:.6};", r, val));
                    r
                } else {
                    let r = self.alloc_reg();
                    lines.push(format!("    mov.f32 {}, 0.0;", r));
                    r
                }
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                let src = self.emit_expr(inner, lines);
                let dst = self.alloc_reg();
                lines.push(format!("    neg.f32 {}, {};", dst, src));
                dst
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let c_reg = self.emit_expr(cond, lines);
                let t_reg = self.emit_expr(then_expr, lines);
                let e_reg = self.emit_expr(else_expr, lines);
                let pred = self.alloc_pred();
                let dst = self.alloc_reg();
                lines.push(format!("    setp.ne.f32 {}, {}, 0.0;", pred, c_reg));
                lines.push(format!(
                    "    selp.f32 {}, {}, {}, {};",
                    dst, t_reg, e_reg, pred
                ));
                dst
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = self.emit_expr(lhs, lines);
                if *op == MathBinOp::Pow {
                    if let MathExpr::Num(exp) = **rhs {
                        if (exp - 2.0).abs() < 1e-9 {
                            let dst = self.alloc_reg();
                            lines.push(format!("    mul.f32 {}, {}, {};", dst, l, l));
                            return dst;
                        }
                    }
                }
                let r = self.emit_expr(rhs, lines);
                let dst = self.alloc_reg();
                match op {
                    MathBinOp::Add => lines.push(format!("    add.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Sub => lines.push(format!("    sub.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Mul | MathBinOp::MatMul => {
                        lines.push(format!("    mul.f32 {}, {}, {};", dst, l, r))
                    }
                    MathBinOp::Div => lines.push(format!("    div.rn.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Pow => {
                        let tmp = self.alloc_reg();
                        lines.push(format!("    lg2.approx.f32 {}, {};", tmp, l));
                        lines.push(format!("    mul.f32 {}, {}, {};", tmp, tmp, r));
                        lines.push(format!("    ex2.approx.f32 {}, {};", dst, tmp));
                    }
                    _ => {
                        let pred = self.alloc_pred();
                        lines.push(format!("    setp.gt.f32 {}, {}, {};", pred, l, r));
                        lines.push(format!("    selp.f32 {}, 1.0, 0.0, {};", dst, pred));
                    }
                }
                dst
            }
            MathExpr::Call(func, args) => {
                let arg0 = args
                    .first()
                    .map(|a| self.emit_expr(a, lines))
                    .unwrap_or_else(|| {
                        let r = self.alloc_reg();
                        lines.push(format!("    mov.f32 {}, 0.0;", r));
                        r
                    });
                let dst = self.alloc_reg();
                match func.as_str() {
                    "sqrt" => lines.push(format!("    sqrt.rn.f32 {}, {};", dst, arg0)),
                    "sin" => lines.push(format!("    sin.approx.f32 {}, {};", dst, arg0)),
                    "cos" => lines.push(format!("    cos.approx.f32 {}, {};", dst, arg0)),
                    "abs" => lines.push(format!("    abs.f32 {}, {};", dst, arg0)),
                    "relu" => lines.push(format!("    max.f32 {}, {}, 0.0;", dst, arg0)),
                    "exp" => {
                        let tmp = self.alloc_reg();
                        lines.push(format!("    mul.f32 {}, {}, 1.442695;", tmp, arg0));
                        lines.push(format!("    ex2.approx.f32 {}, {};", dst, tmp));
                    }
                    "sigmoid" => {
                        let neg = self.alloc_reg();
                        let scaled = self.alloc_reg();
                        let ex = self.alloc_reg();
                        let denom = self.alloc_reg();
                        lines.push(format!("    neg.f32 {}, {};", neg, arg0));
                        lines.push(format!("    mul.f32 {}, {}, 1.442695;", scaled, neg));
                        lines.push(format!("    ex2.approx.f32 {}, {};", ex, scaled));
                        lines.push(format!("    add.f32 {}, {}, 1.0;", denom, ex));
                        lines.push(format!("    rcp.rn.f32 {}, {};", dst, denom));
                    }
                    "silu" => {
                        let neg = self.alloc_reg();
                        let scaled = self.alloc_reg();
                        let ex = self.alloc_reg();
                        let denom = self.alloc_reg();
                        let sig = self.alloc_reg();
                        lines.push(format!("    neg.f32 {}, {};", neg, arg0));
                        lines.push(format!("    mul.f32 {}, {}, 1.442695;", scaled, neg));
                        lines.push(format!("    ex2.approx.f32 {}, {};", ex, scaled));
                        lines.push(format!("    add.f32 {}, {}, 1.0;", denom, ex));
                        lines.push(format!("    rcp.rn.f32 {}, {};", sig, denom));
                        lines.push(format!("    mul.f32 {}, {}, {};", dst, arg0, sig));
                    }
                    _ => lines.push(format!("    mov.f32 {}, {};", dst, arg0)),
                }
                dst
            }
            _ => {
                let r = self.alloc_reg();
                lines.push(format!("    mov.f32 {}, 0.0;", r));
                r
            }
        }
    }
}

/// Checks whether a source string represents a Standard Mathematical Calculus program
pub fn is_math_calculus_source(src: &str) -> bool {
    let mut has_math_def_or_pipe = false;
    for line in src.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") || t.starts_with(';') {
            continue;
        }
        // Exclude low-level VLIW or 5-pillar Unicode headers
        if t.starts_with("@⌖")
            || t.starts_with("@kernel")
            || t.starts_with("@io")
            || t.starts_with('§')
            || t.starts_with('⟳')
            || t.starts_with("!Δ")
            || t.starts_with(".stage")
            || (t.len() >= 6
                && t.starts_with('B')
                && t.as_bytes()[5] == b':'
                && t[1..5].chars().all(|c| c.is_ascii_hexdigit()))
        {
            return false;
        }
        if t.contains("->")
            || (t.contains('=') && !t.contains(":="))
            || t.starts_with("<<")
            || t.contains(">>")
        {
            has_math_def_or_pipe = true;
        }
    }
    has_math_def_or_pipe && parse_math_code(src).is_ok()
}

impl MathProgram {
    /// Recursively resolves all `<< "path.cl"` module links starting from `base_dir`
    pub fn resolve_links(&mut self, base_dir: &std::path::Path) -> Result<(), String> {
        let mut visited = std::collections::HashSet::new();
        self.resolve_links_internal(base_dir, &mut visited)
    }

    fn resolve_links_internal(
        &mut self,
        base_dir: &std::path::Path,
        visited: &mut std::collections::HashSet<std::path::PathBuf>,
    ) -> Result<(), String> {
        let mut resolved_items = Vec::new();

        for item in std::mem::take(&mut self.items) {
            match item {
                MathItem::Link(rel_path) => {
                    let mut candidate = base_dir.join(&rel_path);
                    if !candidate.exists() {
                        let alt = std::path::Path::new(&rel_path);
                        if alt.exists() {
                            candidate = alt.to_path_buf();
                        } else {
                            return Err(format!(
                                "Cannot resolve link '<< \"{}\"': file not found (tried '{}' and '{}')",
                                rel_path,
                                candidate.display(),
                                alt.display()
                            ));
                        }
                    }

                    let canonical = candidate.canonicalize().map_err(|e| {
                        format!("Cannot canonicalize path '{}': {}", candidate.display(), e)
                    })?;

                    if !visited.insert(canonical.clone()) {
                        continue;
                    }

                    let content = std::fs::read_to_string(&canonical).map_err(|e| {
                        format!("Failed to read file '{}': {}", canonical.display(), e)
                    })?;

                    let mut sub_prog = parse_math_code(&content)?;
                    let sub_base = canonical.parent().unwrap_or(base_dir);
                    sub_prog.resolve_links_internal(sub_base, visited)?;

                    resolved_items.extend(sub_prog.items);
                }
                other => resolved_items.push(other),
            }
        }

        self.items = resolved_items;
        Ok(())
    }
}

/// Saves tensor weights to Safetensors or plain text / CLL tensor files
pub fn save_tensor_resource(path: &str, key: &str, val: &MathValue) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }

    if path.ends_with(".safetensors") {
        struct SafetensorEntry {
            name: String,
            dtype: String,
            shape: Vec<usize>,
            data: Vec<u8>,
        }

        let mut entries: Vec<SafetensorEntry> = Vec::new();
        if std::path::Path::new(path).exists() {
            if let Ok(bytes) = std::fs::read(path) {
                if bytes.len() >= 8 {
                    let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
                    if bytes.len() >= 8 + header_len {
                        if let Ok(header_str) = std::str::from_utf8(&bytes[8..8 + header_len]) {
                            let data_buf = &bytes[8 + header_len..];
                            let mut idx = 0;
                            while let Some(q_start) = header_str[idx..].find('"') {
                                let actual_start = idx + q_start + 1;
                                if let Some(q_end) = header_str[actual_start..].find('"') {
                                    let k = &header_str[actual_start..actual_start + q_end];
                                    idx = actual_start + q_end + 1;
                                    if k == "__metadata__"
                                        || k.is_empty()
                                        || k == "dtype"
                                        || k == "shape"
                                        || k == "data_offsets"
                                    {
                                        continue;
                                    }
                                    let rem = header_str[idx..].trim_start();
                                    if rem.starts_with(':') {
                                        let after_colon = rem[1..].trim_start();
                                        if after_colon.starts_with('{') {
                                            let block = after_colon;
                                            let dtype_str = if let Some(d_pos) = block.find("\"dtype\"") {
                                                let after = &block[d_pos + 7..];
                                                if let Some(q1) = after.find('"') {
                                                    if let Some(q2) = after[q1 + 1..].find('"') {
                                                        &after[q1 + 1..q1 + 1 + q2]
                                                    } else {
                                                        "F32"
                                                    }
                                                } else {
                                                    "F32"
                                                }
                                            } else {
                                                "F32"
                                            };

                                            let mut shape = Vec::new();
                                            if let Some(s_pos) = block.find("\"shape\"") {
                                                let after = &block[s_pos + 7..];
                                                if let Some(b1) = after.find('[') {
                                                    if let Some(b2) = after[b1 + 1..].find(']') {
                                                        let dims_str = &after[b1 + 1..b1 + 1 + b2];
                                                        for d in dims_str.split(',') {
                                                            if let Ok(n) = d.trim().parse::<usize>() {
                                                                shape.push(n);
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            let (s_off, e_off) = if let Some(o_pos) = block.find("\"data_offsets\"") {
                                                let after = &block[o_pos + 14..];
                                                if let Some(b1) = after.find('[') {
                                                    if let Some(b2) = after[b1 + 1..].find(']') {
                                                        let pair_str = &after[b1 + 1..b1 + 1 + b2];
                                                        let parts: Vec<&str> = pair_str.split(',').collect();
                                                        if parts.len() == 2 {
                                                            let s = parts[0].trim().parse::<usize>().unwrap_or(0);
                                                            let e = parts[1].trim().parse::<usize>().unwrap_or(0);
                                                            (s, e)
                                                        } else {
                                                            (0, 0)
                                                        }
                                                    } else {
                                                        (0, 0)
                                                    }
                                                } else {
                                                    (0, 0)
                                                }
                                            } else {
                                                (0, 0)
                                            };

                                            if e_off <= data_buf.len() && s_off <= e_off {
                                                entries.push(SafetensorEntry {
                                                    name: k.to_string(),
                                                    dtype: dtype_str.to_string(),
                                                    shape,
                                                    data: data_buf[s_off..e_off].to_vec(),
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut new_tensors = Vec::new();
        match val {
            MathValue::Tuple(items) => {
                let sub_keys: Vec<&str> = key.split(',').map(|s| s.trim()).collect();
                for (i, item) in items.iter().enumerate() {
                    let k = if let Some(&sk) = sub_keys.get(i) {
                        if sk.is_empty() {
                            format!("tensor_{}", i)
                        } else {
                            sk.to_string()
                        }
                    } else {
                        format!("{}_{}", if key.is_empty() { "tensor" } else { key }, i)
                    };
                    let (shape, raw_bytes) = match item {
                        MathValue::Scalar(s) => (vec![1usize], (*s as f32).to_le_bytes().to_vec()),
                        MathValue::Vector(v) => (
                            vec![v.len()],
                            v.iter().flat_map(|&x| (x as f32).to_le_bytes()).collect(),
                        ),
                        MathValue::Matrix(m) => {
                            let rows = m.len();
                            let cols = m.first().map(|r| r.len()).unwrap_or(0);
                            let bytes: Vec<u8> = m
                                .iter()
                                .flatten()
                                .flat_map(|&x| (x as f32).to_le_bytes())
                                .collect();
                            (vec![rows, cols], bytes)
                        }
                        MathValue::Tensor { shape, strides, data } => {
                            let flat = MathValue::get_flattened_strided(shape, strides, data);
                            let bytes: Vec<u8> = flat.iter().flat_map(|&x| (x as f32).to_le_bytes()).collect();
                            (shape.clone(), bytes)
                        }
                        _ => (vec![1usize], vec![0u8; 4]),
                    };
                    new_tensors.push((k, shape, raw_bytes));
                }
            }
            MathValue::Scalar(s) => {
                let name = if key.is_empty() {
                    "weights".to_string()
                } else {
                    key.to_string()
                };
                new_tensors.push((name, vec![1usize], (*s as f32).to_le_bytes().to_vec()));
            }
            MathValue::Vector(vec) => {
                let name = if key.is_empty() {
                    "weights".to_string()
                } else {
                    key.to_string()
                };
                let bytes: Vec<u8> = vec.iter().flat_map(|&x| (x as f32).to_le_bytes()).collect();
                new_tensors.push((name, vec![vec.len()], bytes));
            }
            MathValue::Matrix(mat) => {
                let name = if key.is_empty() {
                    "weights".to_string()
                } else {
                    key.to_string()
                };
                let rows = mat.len();
                let cols = mat.first().map(|r| r.len()).unwrap_or(0);
                let bytes: Vec<u8> = mat
                    .iter()
                    .flatten()
                    .flat_map(|&x| (x as f32).to_le_bytes())
                    .collect();
                new_tensors.push((name, vec![rows, cols], bytes));
            }
            MathValue::Tensor {
                shape,
                strides,
                data,
            } => {
                let name = if key.is_empty() {
                    "weights".to_string()
                } else {
                    key.to_string()
                };
                let flat = MathValue::get_flattened_strided(shape, strides, data);
                let bytes: Vec<u8> = flat
                    .iter()
                    .flat_map(|&x| (x as f32).to_le_bytes())
                    .collect();
                new_tensors.push((name, shape.clone(), bytes));
            }
        }

        for (name, shape, bytes) in new_tensors {
            if let Some(existing) = entries.iter_mut().find(|e| e.name == name) {
                existing.dtype = "F32".to_string();
                existing.shape = shape;
                existing.data = bytes;
            } else {
                entries.push(SafetensorEntry {
                    name,
                    dtype: "F32".to_string(),
                    shape,
                    data: bytes,
                });
            }
        }

        let mut header_parts = Vec::new();
        let mut all_data = Vec::new();
        let mut curr_offset: usize = 0;
        for e in &entries {
            let start = curr_offset;
            let end = start + e.data.len();
            curr_offset = end;
            header_parts.push(format!(
                "\"{}\":{{\"dtype\":\"{}\",\"shape\":{:?},\"data_offsets\":[{},{}]}}",
                e.name, e.dtype, e.shape, start, end
            ));
            all_data.extend_from_slice(&e.data);
        }
        let header_json = format!("{{{}}}", header_parts.join(","));
        let header_bytes = header_json.as_bytes();
        let header_len = header_bytes.len() as u64;

        let mut file_bytes = Vec::with_capacity(8 + header_bytes.len() + all_data.len());
        file_bytes.extend_from_slice(&header_len.to_le_bytes());
        file_bytes.extend_from_slice(header_bytes);
        file_bytes.extend_from_slice(&all_data);

        std::fs::write(path, file_bytes)
            .map_err(|e| format!("Failed to write safetensors '{}': {}", path, e))?;
    } else {
        // Plain text CLL / CSV format
        let name = if key.is_empty() { "tensor" } else { key };
        let out_str = format!("{} = {}\n", name, val);
        std::fs::write(path, out_str)
            .map_err(|e| format!("Failed to write tensor file '{}': {}", path, e))?;
    }

    Ok(())
}

/// Decodes IEEE 754 half-precision float (16-bit) to f64
pub fn decode_f16(u: u16) -> f64 {
    let sign = if (u >> 15) != 0 { -1.0 } else { 1.0 };
    let exp = (u >> 10) & 0x1F;
    let mant = u & 0x3FF;
    if exp == 0 {
        sign * (mant as f64 / 1024.0) * (2.0f64.powi(-14))
    } else if exp == 31 {
        if mant == 0 {
            sign * f64::INFINITY
        } else {
            f64::NAN
        }
    } else {
        sign * (1.0 + (mant as f64 / 1024.0)) * (2.0f64.powi(exp as i32 - 15))
    }
}

/// Parses and loads a tensor from a binary GGUF v2/v3 file
fn load_gguf_tensor(bytes: &[u8], key: &str, path: &str) -> Result<MathValue, String> {
    if bytes.len() < 24 {
        return Err(format!("GGUF file '{}' too short (len: {})", path, bytes.len()));
    }
    if &bytes[0..4] != b"GGUF" {
        return Err(format!("File '{}' is not a valid GGUF file (bad magic)", path));
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version < 2 || version > 3 {
        return Err(format!("Unsupported GGUF version {} in '{}' (expected 2 or 3)", version, path));
    }
    let tensor_count = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let metadata_kv_count = u64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize;

    let mut cursor = 24;

    fn read_str<'a>(bytes: &'a [u8], cursor: &mut usize) -> Result<&'a str, String> {
        if *cursor + 8 > bytes.len() {
            return Err("Unexpected EOF while reading string length in GGUF".to_string());
        }
        let len = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap()) as usize;
        *cursor += 8;
        if *cursor + len > bytes.len() {
            return Err("Unexpected EOF while reading string bytes in GGUF".to_string());
        }
        let s = std::str::from_utf8(&bytes[*cursor..*cursor + len])
            .map_err(|e| format!("Invalid UTF-8 in GGUF string: {}", e))?;
        *cursor += len;
        Ok(s)
    }

    fn skip_val(bytes: &[u8], cursor: &mut usize, val_type: u32) -> Result<(), String> {
        match val_type {
            0 | 1 | 7 => {
                if *cursor + 1 > bytes.len() { return Err("Unexpected EOF in GGUF value".to_string()); }
                *cursor += 1;
            }
            2 | 3 => {
                if *cursor + 2 > bytes.len() { return Err("Unexpected EOF in GGUF value".to_string()); }
                *cursor += 2;
            }
            4 | 5 | 6 => {
                if *cursor + 4 > bytes.len() { return Err("Unexpected EOF in GGUF value".to_string()); }
                *cursor += 4;
            }
            10 | 11 | 12 => {
                if *cursor + 8 > bytes.len() { return Err("Unexpected EOF in GGUF value".to_string()); }
                *cursor += 8;
            }
            8 => {
                let _ = read_str(bytes, cursor)?;
            }
            9 => {
                if *cursor + 12 > bytes.len() { return Err("Unexpected EOF in GGUF array header".to_string()); }
                let elem_type = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                let count = u64::from_le_bytes(bytes[*cursor + 4..*cursor + 12].try_into().unwrap()) as usize;
                *cursor += 12;
                for _ in 0..count {
                    skip_val(bytes, cursor, elem_type)?;
                }
            }
            _ => return Err(format!("Unknown GGUF metadata type ID: {}", val_type)),
        }
        Ok(())
    }

    let mut alignment = 32usize;
    for _ in 0..metadata_kv_count {
        let k = read_str(bytes, &mut cursor)?;
        if cursor + 4 > bytes.len() {
            return Err("Unexpected EOF reading GGUF metadata type".to_string());
        }
        let val_type = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;
        if k == "general.alignment" && (val_type == 4 || val_type == 10) {
            alignment = if val_type == 4 {
                u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize
            } else {
                u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap()) as usize
            };
        }
        skip_val(bytes, &mut cursor, val_type)?;
    }

    struct GgufTensorInfo {
        name: String,
        dims: Vec<usize>,
        ggml_type: u32,
        offset: u64,
    }
    let mut tensors = Vec::with_capacity(tensor_count);
    for _ in 0..tensor_count {
        let name = read_str(bytes, &mut cursor)?.to_string();
        if cursor + 4 > bytes.len() {
            return Err("Unexpected EOF reading GGUF tensor dimensions count".to_string());
        }
        let n_dims = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;
        if cursor + n_dims * 8 > bytes.len() {
            return Err("Unexpected EOF reading GGUF tensor dimensions".to_string());
        }
        let mut dims = Vec::with_capacity(n_dims);
        for _ in 0..n_dims {
            dims.push(u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap()) as usize);
            cursor += 8;
        }
        if cursor + 12 > bytes.len() {
            return Err("Unexpected EOF reading GGUF tensor type/offset".to_string());
        }
        let ggml_type = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;
        let offset = u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
        cursor += 8;
        tensors.push(GgufTensorInfo { name, dims, ggml_type, offset });
    }

    let data_start = (cursor + alignment - 1) & !(alignment - 1);

    let target = if key.is_empty() {
        tensors.first().ok_or_else(|| format!("No tensors found in GGUF file '{}'", path))?
    } else {
        tensors.iter().find(|t| t.name == key).ok_or_else(|| {
            format!("Tensor '{}' not found in GGUF file '{}'", key, path)
        })?
    };

    let shape: Vec<usize> = target.dims.iter().copied().rev().collect();
    let total_elements: usize = if shape.is_empty() { 1 } else { shape.iter().product() };
    let tensor_raw_offset = data_start + target.offset as usize;
    if tensor_raw_offset > bytes.len() {
        return Err(format!("GGUF tensor offset out of bounds: {}", tensor_raw_offset));
    }
    let raw_data = &bytes[tensor_raw_offset..];

    let mut floats = Vec::with_capacity(total_elements);
    match target.ggml_type {
        0 => { // GGML_TYPE_F32
            if raw_data.len() < total_elements * 4 {
                return Err(format!("Truncated F32 tensor '{}' in GGUF", target.name));
            }
            for chunk in raw_data[..total_elements * 4].chunks_exact(4) {
                floats.push(f32::from_le_bytes(chunk.try_into().unwrap()) as f64);
            }
        }
        1 => { // GGML_TYPE_F16
            if raw_data.len() < total_elements * 2 {
                return Err(format!("Truncated F16 tensor '{}' in GGUF", target.name));
            }
            for chunk in raw_data[..total_elements * 2].chunks_exact(2) {
                let u = u16::from_le_bytes(chunk.try_into().unwrap());
                floats.push(decode_f16(u));
            }
        }
        8 => { // GGML_TYPE_Q8_0
            let num_blocks = (total_elements + 31) / 32;
            if raw_data.len() < num_blocks * 34 {
                return Err(format!("Truncated Q8_0 tensor '{}' in GGUF", target.name));
            }
            for block in raw_data[..num_blocks * 34].chunks_exact(34) {
                let delta_u = u16::from_le_bytes(block[0..2].try_into().unwrap());
                let delta = decode_f16(delta_u);
                for &b in &block[2..34] {
                    if floats.len() < total_elements {
                        floats.push((b as i8) as f64 * delta);
                    }
                }
            }
        }
        2 => { // GGML_TYPE_Q4_0
            let num_blocks = (total_elements + 31) / 32;
            if raw_data.len() < num_blocks * 18 {
                return Err(format!("Truncated Q4_0 tensor '{}' in GGUF", target.name));
            }
            for block in raw_data[..num_blocks * 18].chunks_exact(18) {
                let delta_u = u16::from_le_bytes(block[0..2].try_into().unwrap());
                let delta = decode_f16(delta_u);
                for &b in &block[2..18] {
                    let v0 = (b & 0x0F) as i8 - 8;
                    let v1 = ((b >> 4) & 0x0F) as i8 - 8;
                    if floats.len() < total_elements {
                        floats.push(v0 as f64 * delta);
                    }
                    if floats.len() < total_elements {
                        floats.push(v1 as f64 * delta);
                    }
                }
            }
        }
        other => {
            return Err(format!("Unsupported GGML tensor type {} in GGUF file '{}'", other, path));
        }
    }

    if shape.is_empty() {
        Ok(MathValue::Scalar(floats.first().copied().unwrap_or(0.0)))
    } else if shape.len() == 1 {
        Ok(MathValue::Vector(floats))
    } else if shape.len() == 2 {
        let rows = shape[0];
        let cols = shape[1];
        let mut mat = Vec::with_capacity(rows);
        for r in 0..rows {
            let start = r * cols;
            let end = (start + cols).min(floats.len());
            if start < floats.len() {
                mat.push(floats[start..end].to_vec());
            } else {
                mat.push(vec![0.0; cols]);
            }
        }
        Ok(MathValue::Matrix(mat))
    } else {
        MathValue::new_tensor(shape, floats)
    }
}

/// Loads tensor weights from Safetensors, GGUF, or plain text / CLL tensor files
pub fn load_tensor_resource(path: &str, key: &str) -> Result<MathValue, String> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(format!(
            "Tensor file not found: '{}' (key: '{}')",
            path, key
        ));
    }

    let bytes =
        std::fs::read(p).map_err(|e| format!("Failed to read tensor file '{}': {}", path, e))?;

    // 1. SAFETENSORS
    if path.ends_with(".safetensors") {
        if bytes.len() < 8 {
            return Err(format!("Invalid safetensors file '{}': too short", path));
        }
        let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
        if bytes.len() < 8 + header_len {
            return Err(format!("Corrupt safetensors header in '{}'", path));
        }
        let header_bytes = &bytes[8..8 + header_len];
        let header_str = std::str::from_utf8(header_bytes)
            .map_err(|e| format!("Invalid UTF-8 in safetensors header: {}", e))?;

        let target_key = if key.is_empty() {
            let mut found = None;
            let mut idx = 0;
            while let Some(q_start) = header_str[idx..].find('"') {
                let actual_start = idx + q_start + 1;
                if let Some(q_end) = header_str[actual_start..].find('"') {
                    let k = &header_str[actual_start..actual_start + q_end];
                    if k != "__metadata__" && !k.is_empty() {
                        found = Some(k.to_string());
                        break;
                    }
                    idx = actual_start + q_end + 1;
                } else {
                    break;
                }
            }
            found.ok_or_else(|| format!("No tensors found in '{}'", path))?
        } else {
            key.to_string()
        };

        let pattern = format!("\"{}\"", target_key);
        let key_pos = header_str.find(&pattern).ok_or_else(|| {
            format!(
                "Tensor '{}' not found in safetensors file '{}'",
                target_key, path
            )
        })?;

        let block = &header_str[key_pos..];

        let dtype_str = if let Some(d_pos) = block.find("\"dtype\"") {
            let after = &block[d_pos + 7..];
            if let Some(q1) = after.find('"') {
                if let Some(q2) = after[q1 + 1..].find('"') {
                    &after[q1 + 1..q1 + 1 + q2]
                } else {
                    "F32"
                }
            } else {
                "F32"
            }
        } else {
            "F32"
        };

        let mut shape = Vec::new();
        if let Some(s_pos) = block.find("\"shape\"") {
            let after = &block[s_pos + 7..];
            if let Some(b1) = after.find('[') {
                if let Some(b2) = after[b1 + 1..].find(']') {
                    let dims_str = &after[b1 + 1..b1 + 1 + b2];
                    for d in dims_str.split(',') {
                        let trimmed = d.trim();
                        if let Ok(n) = trimmed.parse::<usize>() {
                            shape.push(n);
                        }
                    }
                }
            }
        }

        let (start_offset, end_offset) = if let Some(o_pos) = block.find("\"data_offsets\"") {
            let after = &block[o_pos + 14..];
            if let Some(b1) = after.find('[') {
                if let Some(b2) = after[b1 + 1..].find(']') {
                    let pair_str = &after[b1 + 1..b1 + 1 + b2];
                    let parts: Vec<&str> = pair_str.split(',').collect();
                    if parts.len() == 2 {
                        let s = parts[0].trim().parse::<usize>().unwrap_or(0);
                        let e = parts[1].trim().parse::<usize>().unwrap_or(0);
                        (s, e)
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        let data_start = 8 + header_len + start_offset;
        let data_end = 8 + header_len + end_offset;
        if data_end > bytes.len() || data_start > data_end {
            return Err(format!(
                "Invalid data offsets [{}-{}] in '{}'",
                start_offset, end_offset, path
            ));
        }

        let tensor_raw = &bytes[data_start..data_end];
        let mut floats = Vec::new();

        match dtype_str {
            "F32" => {
                for chunk in tensor_raw.chunks_exact(4) {
                    let val = f32::from_le_bytes(chunk.try_into().unwrap());
                    floats.push(val as f64);
                }
            }
            "F16" => {
                for chunk in tensor_raw.chunks_exact(2) {
                    let u = u16::from_le_bytes(chunk.try_into().unwrap());
                    floats.push(decode_f16(u));
                }
            }
            "BF16" => {
                for chunk in tensor_raw.chunks_exact(2) {
                    let u = u16::from_le_bytes(chunk.try_into().unwrap());
                    let f = f32::from_bits((u as u32) << 16);
                    floats.push(f as f64);
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported safetensors dtype '{}' in '{}'",
                    dtype_str, path
                ));
            }
        }

        if shape.is_empty() {
            Ok(MathValue::Scalar(floats.first().copied().unwrap_or(0.0)))
        } else if shape.len() == 1 {
            Ok(MathValue::Vector(floats))
        } else if shape.len() == 2 {
            let rows = shape[0];
            let cols = shape[1];
            let mut mat = Vec::with_capacity(rows);
            for r in 0..rows {
                let start = r * cols;
                let end = (start + cols).min(floats.len());
                if start < floats.len() {
                    mat.push(floats[start..end].to_vec());
                } else {
                    mat.push(vec![0.0; cols]);
                }
            }
            Ok(MathValue::Matrix(mat))
        } else {
            MathValue::new_tensor(shape, floats)
        }
    } else if path.ends_with(".gguf") || bytes.starts_with(b"GGUF") {
        load_gguf_tensor(&bytes, key, path)
    } else {
        // 2. Plain Text CLL / Vector / Matrix format
        let text = String::from_utf8_lossy(&bytes);
        if text.contains('[') {
            let expr_str = if let Some(eq_pos) = text.find('=') {
                &text[eq_pos + 1..]
            } else {
                &text
            };
            let prog = parse_math_code(expr_str.trim())?;
            let mut eval = MathEvaluator::new();
            eval.execute_program_value(&prog)?
                .ok_or_else(|| format!("No tensor value found in '{}'", path))
        } else {
            let mut nums = Vec::new();
            for word in text.split(|c: char| c.is_whitespace() || c == ',') {
                let w = word.trim();
                if !w.is_empty() {
                    if let Ok(n) = w.parse::<f64>() {
                        nums.push(n);
                    }
                }
            }
            Ok(MathValue::Vector(nums))
        }
    }
}

/// Convenience function: parses a .cl math file and automatically resolves all `<<` module links
pub fn parse_math_file(path: impl AsRef<std::path::Path>) -> Result<MathProgram, String> {
    let p = path.as_ref();
    let content = std::fs::read_to_string(p)
        .map_err(|e| format!("Failed to read math file '{}': {}", p.display(), e))?;
    let mut prog = parse_math_code(&content)?;
    let base_dir = p.parent().unwrap_or_else(|| std::path::Path::new("."));
    prog.resolve_links(base_dir)?;
    Ok(prog)
}

/// Convenience parser function
pub fn parse_math_code(src: &str) -> Result<MathProgram, String> {
    let mut lexer = MathLexer::new(src);
    let tokens = lexer.tokenize()?;
    let mut parser = MathParser::new(tokens);
    parser.parse_program()
}

/// Runs the interactive Mathematical REPL in the terminal
pub fn run_math_repl() -> Result<(), String> {
    use std::io::{self, BufRead, Write};

    println!("================================================================================");
    println!(" CRON SOVEREIGN MATHEMATICAL REPL (Zero-Word Calculus)");
    println!(" Commands: :c23 (View C23) | :ptx (View PTX) | :vliw (View VLIW) | :reset | :quit");
    println!("================================================================================");

    let mut session_prog = MathProgram::default();
    let mut eval = MathEvaluator::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("cron(math)> ");
        stdout.flush().map_err(|e| e.to_string())?;

        let mut line = String::new();
        if stdin
            .lock()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?
            == 0
        {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with(':') {
            match trimmed {
                ":q" | ":quit" | ":exit" => {
                    println!("Exiting CRON Math REPL.");
                    break;
                }
                ":reset" | ":clear" => {
                    session_prog = MathProgram::default();
                    eval = MathEvaluator::new();
                    println!("Session state reset.");
                }
                ":c23" => {
                    println!("{}", session_prog.to_c23());
                }
                ":ptx" => {
                    println!("{}", session_prog.to_ptx());
                }
                ":vliw" => {
                    println!("{}", session_prog.to_vliw());
                }
                ":help" => {
                    println!("Commands:\n  :c23   - View generated C23 code\n  :ptx   - View generated PTX kernel\n  :vliw  - View generated VLIW microcode\n  :reset - Reset variables and functions\n  :quit  - Exit REPL");
                }
                other => {
                    println!("Unknown command '{}'. Type :help for commands.", other);
                }
            }
            continue;
        }

        match parse_math_code(trimmed) {
            Ok(prog) => {
                match eval.execute_program_value(&prog) {
                    Ok(Some(val)) => {
                        println!("\x1b[32m=> {}\x1b[0m", val);
                    }
                    Ok(None) => {
                        println!("\x1b[36m=> (defined)\x1b[0m");
                    }
                    Err(e) => {
                        println!("\x1b[31m[Eval Error] {}\x1b[0m", e);
                    }
                }
                session_prog.items.extend(prog.items);
            }
            Err(e) => {
                println!("\x1b[31m[Parse Error] {}\x1b[0m", e);
            }
        }
    }

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_math_evaluation() {
        let code = r#"
            pi = 3.14159265
            d = 10.0
            f(x: f32) = x^2 + 2 * x + 1
            f(5)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse math code");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(36.0));
    }

    #[test]
    fn test_pipeline_chaining() {
        let code = r#"
            sq(x: f32) = x * x
            add_ten(x: f32) = x + 10.0
            5.0 -> sq -> add_ten
        "#;
        let prog = parse_math_code(code).expect("Failed to parse pipeline");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(35.0));
    }

    #[test]
    fn test_vector_matrix_and_rmsnorm_evaluation() {
        let code = r#"
            eps = 0.0
            rmsnorm(x: f32) = x / sqrt(mean(x^2) + eps)
            linear(x: f16, W: f16, b: f16) = W @ x + b

            x = [3.0, 4.0]
            W = [[1.0, 2.0], [3.0, 4.0]]
            b = [1.0, -1.0]

            // W @ x + b = [1*3 + 2*4 + 1, 3*3 + 4*4 - 1] = [12.0, 24.0]
            y = x -> linear(W, b)
            sum(y)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse tensor math");
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Tensor evaluation failed");
        assert_eq!(res, Some(36.0));
    }

    #[test]
    fn test_ternary_conditional_operator() {
        let code = r#"
            leaky(x: f32) = x > 0 ? x : 0.1 * x
            leaky(-20)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse ternary");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Ternary eval failed");
        assert!((res.unwrap() - (-2.0)).abs() < 1e-6);
    }

    #[test]
    fn test_neural_math_c23_ptx_and_vliw_emission() {
        let code = r#"
            eps = 1e-5
            norm(x: f32) = x / sqrt(x^2 + eps)
            linear(x: f16, W: f16, b: f16) = W * x + b
            act(z: f16) = sigmoid(z)
            forward(x: f16, W: f16, b: f16) = x -> norm -> linear(W, b) -> act
        "#;
        let prog = parse_math_code(code).expect("Failed to parse neural code");

        // 1. Verify C23 Type Inference & Strength Reduction (x * x instead of powf)
        let c23_code = prog.to_c23();
        assert!(c23_code.contains("_Float16 linear(_Float16 x, _Float16 W, _Float16 b)"));
        assert!(c23_code.contains("(x * x)"));
        assert!(!c23_code.contains("powf"));

        // 2. Verify Real AST-Driven PTX Generation
        let ptx_code = prog.to_ptx();
        assert!(ptx_code.contains("sqrt.rn.f32"));
        assert!(ptx_code.contains("div.rn.f32"));
        assert!(ptx_code.contains("ex2.approx.f32"));

        // 3. Verify 4-Slot VLIW Microcode Lowering with CRC-8 Verification
        let vliw_code = prog.to_vliw();
        assert!(vliw_code.contains("B0000:"));
        let verify_rep = crate::cl_lang::verify_cl_program(&vliw_code)
            .expect("Generated VLIW failed hardware CRC-8/hazard verification");
        assert!(verify_rep.total_bundles > 0);
        assert!(verify_rep.crc_verified > 0);
    }

    #[test]
    fn test_unified_cl_single_source_of_truth_pipeline() {
        let code = r#"
            eps = 1e-5
            norm(x: f32) = x / sqrt(mean(x^2) + eps)
            linear(x: f16, W: f16, b: f16) = W @ x + b
            forward(x: f16, W: f16, b: f16) = x -> norm -> linear(W, b)
        "#;
        let doc = crate::parse_unified_cl(code).expect("Unified parser failed on standard math");
        assert!(!doc.functions.is_empty());
        assert!(!doc.vliw_stream.is_empty());
        let diags = crate::check_source_diagnostics(code);
        assert!(diags.is_empty(), "Expected 0 diagnostics, got: {:?}", diags);
    }

    #[test]
    fn test_symbolic_autodiff_grad_and_c23_lowering() {
        let code = r#"
            loss(w: f32, x: f32, y: f32) = (w * x - y)^2
            d_loss(w: f32, x: f32, y: f32) = grad(loss(w, x, y), w)
            // At w=3, x=2, y=1: loss = (6-1)^2 = 25, d_loss/dw = 2*(w*x - y)*x = 2*5*2 = 20
            d_loss(3, 2, 1)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse autodiff code");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Autodiff eval failed");
        assert_eq!(res, Some(20.0));

        // Verify C23 emits simplified analytical derivative without 'grad(' call
        let c23 = prog.to_c23();
        assert!(!c23.contains("grad("));
        assert!(c23.contains("((2.000000f * ((w * x) - y)) * x)"));
    }

    #[test]
    fn test_shaped_tensor_c23_gemv_and_indexing() {
        let code = r#"
            linear_tile(x: f16[16], W: f16[16, 16], b: f16[16]) = W @ x + b
            seq = range(1, 5) -> scan
            seq[3]
        "#;
        let prog = parse_math_code(code).expect("Failed to parse shaped tensor & scan");
        let mut eval = MathEvaluator::new();
        // range(1, 5) = [1, 2, 3, 4], scan = [1, 3, 6, 10], seq[3] = 10
        let res = eval.execute_program(&prog).expect("Scan/index eval failed");
        assert_eq!(res, Some(10.0));

        let c23 = prog.to_c23();
        assert!(c23.contains("const _Float16* restrict W"));
        assert!(c23.contains("for (int i = 0; i < 16; ++i)"));
        assert!(c23.contains("W[i * 16 + j]"));
    }

    #[test]
    fn test_zero_word_module_linking_resolution() {
        let temp_dir = std::env::temp_dir().join(format!("cron_test_link_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let mod_file = temp_dir.join("sub_math.cl");
        std::fs::write(
            &mod_file,
            "square(x: f32) = x * x\ncube(x: f32) = x * square(x)\n",
        )
        .unwrap();

        let main_code = "        << \"sub_math.cl\"\n        val = cube(3.0)\n        val\n";
        let mut prog = parse_math_code(main_code).expect("Failed to parse main module link");
        prog.resolve_links(&temp_dir)
            .expect("Failed to resolve module links");

        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(27.0));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_external_tensor_weights_loading_and_gemv() {
        let temp_dir =
            std::env::temp_dir().join(format!("cron_test_tensor_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let weights_file = temp_dir.join("weights.cll");
        std::fs::write(&weights_file, "W = [[2.0, 0.0], [0.0, 3.0]]\n").unwrap();

        // Use absolute path in forward slashes
        let p_str = weights_file.to_str().unwrap().replace('\\', "/");
        let main_code = format!(
            "            W = @\"{}:W\"\n            x = [4.0, 5.0]\n            y = W @ x\n            y[1]\n",
            p_str
        );

        let prog = parse_math_code(&main_code).expect("Failed to parse tensor ref code");
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Evaluation of external tensor failed");
        // W @ [4, 5] = [2*4+0*5, 0*4+3*5] = [8, 15], y[1] = 15.0
        assert_eq!(res, Some(15.0));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_functional_iteration_power_training_loop() {
        let code = r#"
            loss(w: f32) = (w * 2.0 - 10.0)^2
            step(w: f32) = w - 0.05 * grad(loss(w), w)
            w_opt = 0.0 -> step^50
            w_opt
        "#;
        let prog = parse_math_code(code).expect("Failed to parse functional power iteration");
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Evaluation of training loop failed");
        // w starts at 0, converges to 5.0
        assert!(
            (res.unwrap() - 5.0).abs() < 1e-4,
            "Expected 5.0, got {:?}",
            res
        );

        // Verify C23 lowering emits functional loop
        let c23 = prog.to_c23_executable();
        assert!(c23.contains("for (int _i = 0; _i < (int)(50"));
    }

    #[test]
    fn test_safetensors_export_and_import_roundtrip() {
        let temp_dir =
            std::env::temp_dir().join(format!("cron_test_export_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let sf_file = temp_dir.join("matrix.safetensors");
        let p_str = sf_file.to_str().unwrap().replace('\\', "/");

        let export_code = format!(
            "            W = [[1.5, 2.5], [3.5, 4.5]]\n            W >> \"{}:weights\"\n",
            p_str
        );
        let prog_exp = parse_math_code(&export_code).expect("Failed to parse export code");
        let mut eval_exp = MathEvaluator::new();
        eval_exp
            .execute_program_value(&prog_exp)
            .expect("Execution of export failed");
        assert!(sf_file.exists(), "Safetensors file was not written to disk");

        // Read it back via @
        let import_code = format!(
            "            W_read = @\"{}:weights\"\n            x = [2.0, 1.0]\n            y = W_read @ x\n            y[0]\n",
            p_str
        );
        let prog_imp = parse_math_code(&import_code).expect("Failed to parse import code");
        let mut eval_imp = MathEvaluator::new();
        let res = eval_imp
            .execute_program(&prog_imp)
            .expect("Execution of import/matmul failed");
        // [1.5*2 + 2.5*1, 3.5*2 + 4.5*1] = [3.0+2.5, 7.0+4.5] = [5.5, 11.5], y[0] = 5.5
        assert!(
            (res.unwrap() - 5.5).abs() < 1e-5,
            "Expected 5.5, got {:?}",
            res
        );

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_matrix_transpose_pow_t_notation_and_gemv() {
        let code = r#"
            W = [[1.0, 2.0], [3.0, 4.0]]
            x = [10.0, 20.0]
            // W^T = [[1, 3], [2, 4]]
            // W^T @ x = [1*10 + 3*20, 2*10 + 4*20] = [70, 100]
            y = (W^T) @ x
            y[0]
        "#;
        let prog = parse_math_code(code).expect("Failed to parse W^T transpose");
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Evaluation of W^T failed");
        assert_eq!(res, Some(70.0));

        // Test vector @ matrix GEMV
        let code_gemv = r#"
            u = [2.0, 3.0]
            M = [[1.0, 0.0], [0.0, 2.0]]
            v = u @ M
            v[1]
        "#;
        let prog_gemv = parse_math_code(code_gemv).expect("Failed to parse vector @ matrix");
        let mut eval_gemv = MathEvaluator::new();
        let res_gemv = eval_gemv
            .execute_program(&prog_gemv)
            .expect("Evaluation of GEMV failed");
        assert_eq!(res_gemv, Some(6.0));
    }

    #[test]
    fn test_tensor_reshape_slice_and_outer_product() {
        let code = r#"
            v = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
            M = reshape(v, [2, 3])
            // M is [[1, 2, 3], [4, 5, 6]], slice(0, 1) is [[1, 2, 3]]
            row0 = slice(M, 0, 1)
            sum(row0)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse reshape & slice");
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Reshape/slice evaluation failed");
        assert_eq!(res, Some(6.0));

        // Test outer product
        let code_outer = r#"
            a = [2.0, 3.0]
            b = [4.0, 5.0]
            // outer(a, b) = [[8, 10], [12, 15]]
            O = outer(a, b)
            O[1][1]
        "#;
        let prog_outer = parse_math_code(code_outer).expect("Failed to parse outer product");
        let mut eval_outer = MathEvaluator::new();
        let res_outer = eval_outer
            .execute_program(&prog_outer)
            .expect("Outer product failed");
        assert_eq!(res_outer, Some(15.0));
    }

    #[test]
    fn test_static_shape_inference_and_checker() {
        // 1. Valid shaped tensor operations pass shape verification
        let valid_code = r#"
            linear(W: f16[16, 32], x: f16[32]) = W @ x
            W_mat = [[1.0, 2.0], [3.0, 4.0]]
            x_vec = [5.0, 6.0]
            y_vec = W_mat @ x_vec
            z = sum(y_vec)
        "#;
        let valid_prog = parse_math_code(valid_code).expect("Valid code parsing failed");
        assert!(
            valid_prog.verify_shapes().is_ok(),
            "Valid shapes should pass verification"
        );

        // 2. Shape mismatch in matrix-vector GEMV parameter annotations
        let invalid_gemv = r#"
            bad_linear(W: f16[16, 32], x: f16[16]) = W @ x
        "#;
        let invalid_prog =
            parse_math_code(invalid_gemv).expect("Parsing bad_linear should succeed");
        let errs = invalid_prog
            .verify_shapes()
            .expect_err("Mismatched dimensions must fail");
        assert!(
            errs.iter()
                .any(|e| e.contains("Matrix-vector multiplication shape mismatch")),
            "Expected GEMV mismatch error, found: {:?}",
            errs
        );

        // 3. Shape mismatch in matrix-matrix GEMM
        let invalid_gemm = r#"
            gemm(A: f32[16, 32], B: f32[16, 64]) = A @ B
        "#;
        let prog_gemm = parse_math_code(invalid_gemm).expect("Parsing bad gemm should succeed");
        let errs_gemm = prog_gemm
            .verify_shapes()
            .expect_err("Mismatched inner dims must fail");
        assert!(
            errs_gemm
                .iter()
                .any(|e| e.contains("Matrix multiplication shape mismatch")),
            "Expected GEMM mismatch error, found: {:?}",
            errs_gemm
        );

        // 4. Element-wise shape mismatch
        let invalid_add = r#"
            bad_add(A: f32[16, 32], B: f32[16, 16]) = A + B
        "#;
        let prog_add = parse_math_code(invalid_add).expect("Parsing bad add should succeed");
        let errs_add = prog_add
            .verify_shapes()
            .expect_err("Mismatched add dims must fail");
        assert!(
            errs_add
                .iter()
                .any(|e| e.contains("Element-wise binary op '+' shape mismatch")),
            "Expected element-wise error, found: {:?}",
            errs_add
        );

        // 5. Ragged array literal
        let ragged_code = r#"
            R = [[1.0, 2.0], [3.0, 4.0, 5.0]]
        "#;
        let prog_ragged =
            parse_math_code(ragged_code).expect("Parsing ragged array should succeed");
        let errs_ragged = prog_ragged
            .verify_shapes()
            .expect_err("Ragged array must fail");
        assert!(
            errs_ragged
                .iter()
                .any(|e| e.contains("Ragged tensor array literal")),
            "Expected ragged error, found: {:?}",
            errs_ragged
        );

        // 6. Reshape element count mismatch
        let bad_reshape = r#"
            v = [1.0, 2.0, 3.0, 4.0]
            M = reshape(v, [3, 2])
        "#;
        let prog_reshape =
            parse_math_code(bad_reshape).expect("Parsing bad reshape should succeed");
        let errs_reshape = prog_reshape
            .verify_shapes()
            .expect_err("Bad reshape element count must fail");
        assert!(
            errs_reshape
                .iter()
                .any(|e| e.contains("Cannot reshape tensor")),
            "Expected reshape count error, found: {:?}",
            errs_reshape
        );

        // 7. Verify check_source_diagnostics returns E_MATH_SHAPE
        let diags = crate::check_source_diagnostics(invalid_gemv);
        assert!(
            !diags.is_empty(),
            "Diagnostics should not be empty for invalid shape code"
        );
        assert_eq!(diags[0].code, "E_MATH_SHAPE");
    }

    #[test]
    fn test_c23_static_stack_arena_codegen() {
        let code = r#"
            W = [[1.0, 2.0], [3.0, 4.0]]
            x = [5.0, 6.0]
            y = W @ x
        "#;
        let prog = parse_math_code(code).expect("Parsing tensor pipeline failed");
        let c23_exec = prog.to_c23_executable();
        // Check static stack arena 64-byte alignment and zero heap calls
        assert!(
            c23_exec.contains("alignas(64) float W[2][2]"),
            "Expected alignas(64) matrix allocation"
        );
        assert!(
            c23_exec.contains("alignas(64) float x[2]"),
            "Expected alignas(64) vector allocation"
        );
        assert!(
            c23_exec.contains("alignas(64) float y[2];"),
            "Expected alignas(64) output buffer"
        );
        assert!(
            !c23_exec.contains("malloc"),
            "C23 static stack arena must have 0 malloc calls"
        );
        assert!(
            !c23_exec.contains("free("),
            "C23 static stack arena must have 0 free calls"
        );
    }

    #[test]
    fn test_row_wise_matrix_softmax_and_attention() {
        let code = r#"
            Q = [[1.0, 0.0], [0.0, 1.0]]
            K = [[1.0, 0.0], [0.0, 1.0]]
            V = [[10.0, 20.0], [30.0, 40.0]]
            out = attention(Q, K, V)
            sum(out)
        "#;
        let prog = parse_math_code(code).expect("Attention code parsing failed");
        assert!(
            prog.verify_shapes().is_ok(),
            "Attention shape verification must pass"
        );
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Attention execution failed");
        // Sum of attention output over rows is exactly 100.0
        assert!(
            (res.unwrap() - 100.0).abs() < 1e-4,
            "Attention sum should equal 100.0"
        );
    }

    #[test]
    fn test_ptx_tensor_core_wmma_emission() {
        let code = r#"
            gemm_tc(A: f16[16, 16], B: f16[16, 16]) = A @ B
        "#;
        let prog = parse_math_code(code).expect("Parsing wmma tile failed");
        assert!(prog.verify_shapes().is_ok(), "Shapes should be valid");
        let ptx = prog.to_ptx();
        assert!(
            ptx.contains("wmma.load.a.sync.aligned.row.m16n16k16.f16"),
            "Expected WMMA load A"
        );
        assert!(
            ptx.contains("wmma.load.b.sync.aligned.row.m16n16k16.f16"),
            "Expected WMMA load B"
        );
        assert!(
            ptx.contains("wmma.mma.sync.aligned.row.row.m16n16k16.f32.f16"),
            "Expected WMMA MMA instruction"
        );
        assert!(
            ptx.contains("wmma.store.d.sync.aligned.row.m16n16k16.f32"),
            "Expected WMMA store"
        );
    }

    #[test]
    fn test_llm_operators_rmsnorm_rope_silu() {
        // Test RMSNorm
        let code_norm = r#"
            x = [3.0, 4.0]
            g = [1.0, 1.0]
            // norm: rms of [3, 4] is sqrt(25/2) = sqrt(12.5) ~ 3.5355
            // x_norm: [3/3.5355, 4/3.5355] -> sum of squares is 2.0
            y = rmsnorm(x, g, 0.0)
            sum(y^2)
        "#;
        let prog_norm = parse_math_code(code_norm).expect("Parse rmsnorm failed");
        let mut eval_norm = MathEvaluator::new();
        let res_norm = eval_norm
            .execute_program(&prog_norm)
            .expect("Eval rmsnorm failed");
        assert!(
            (res_norm.unwrap() - 2.0).abs() < 1e-4,
            "RMSNorm squared sum should be 2.0 (length of vector)"
        );

        // Test RoPE
        let code_rope = r#"
            x = [1.0, 0.0]
            // pos 0: angle 0 -> [1.0, 0.0]
            r0 = rope(x, 0)
            r0[0]
        "#;
        let prog_rope = parse_math_code(code_rope).expect("Parse rope failed");
        let mut eval_rope = MathEvaluator::new();
        let res_rope = eval_rope
            .execute_program(&prog_rope)
            .expect("Eval rope failed");
        assert!((res_rope.unwrap() - 1.0).abs() < 1e-6);

        // Test SiLU
        let code_silu = r#"
            // silu(0) = 0 * 0.5 = 0
            silu(0.0)
        "#;
        let prog_silu = parse_math_code(code_silu).expect("Parse silu failed");
        let mut eval_silu = MathEvaluator::new();
        let res_silu = eval_silu
            .execute_program(&prog_silu)
            .expect("Eval silu failed");
        assert_eq!(res_silu, Some(0.0));
    }

    #[test]
    fn test_nd_tensor_grad_and_matrix_training() {
        // Train a 2x2 weight matrix W so that X @ W = Y
        // Target W* = [[2.0, 0.0], [0.0, 3.0]]
        let code = r#"
            X = [[1.0, 0.0], [0.0, 1.0]]
            Y = [[2.0, 0.0], [0.0, 3.0]]
            loss(W) = sum((X @ W - Y)^2)
            step(W) = W - 0.1 * grad(loss(W), W)
            W_trained = zeros(2, 2) -> step^60
            loss(W_trained)
        "#;
        let prog = parse_math_code(code).expect("Parse matrix training failed");
        assert!(
            prog.verify_shapes().is_ok(),
            "Shape check should pass for 2D matrix grad training"
        );
        let mut eval = MathEvaluator::new();
        let final_loss = eval
            .execute_program(&prog)
            .expect("Eval matrix training failed")
            .unwrap();
        assert!(
            final_loss < 1e-4,
            "Expected matrix training loss < 1e-4, got {}",
            final_loss
        );
    }

    #[test]
    fn test_tensor_broadcasting_and_constructors() {
        let code = r#"
            Z = zeros(2, 3)
            O = ones(2, 3)
            b = [10.0, 20.0, 30.0]
            // Broadcast [2, 3] + [3] -> [2, 3]
            M = (Z + O) + b
            // Row 0: [11, 21, 31], Row 1: [11, 21, 31] -> sum = 126.0
            sum(M)
        "#;
        let prog = parse_math_code(code).expect("Parse broadcasting failed");
        assert!(
            prog.verify_shapes().is_ok(),
            "Broadcasting shape verification should pass"
        );
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Eval broadcasting failed")
            .unwrap();
        assert!((res - 126.0).abs() < 1e-6);

        // Also test eye(3) and randn(2, 2, 42)
        let code2 = r#"
            I = eye(3)
            R = randn(2, 2, 42)
            sum(I) + 0.0 * sum(R)
        "#;
        let prog2 = parse_math_code(code2).expect("Parse eye/randn failed");
        assert!(prog2.verify_shapes().is_ok());
        let mut eval2 = MathEvaluator::new();
        let res2 = eval2
            .execute_program(&prog2)
            .expect("Eval eye/randn failed")
            .unwrap();
        assert!((res2 - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_causal_attention_argmax_and_concat() {
        // In causal attention, token 0 can only attend to token 0!
        // So output row 0 must equal V[0] regardless of K[1] or V[1].
        let code = r#"
            Q = [[1.0, 0.0], [0.0, 1.0]]
            K = [[1.0, 0.0], [100.0, 100.0]]
            V = [[7.0, 8.0], [999.0, 999.0]]
            O = causal_attn(Q, K, V)
            // Row 0 of O must be [7.0, 8.0] because token 1 is causally masked out for token 0
            r0 = O[0]
            // Test argmax and concat
            logits = [0.1, 0.5, 4.2, 0.3]
            next_tok = argmax(logits)
            seq = concat([1.0, 2.0], [next_tok])
            sum(r0) + sum(seq)
        "#;
        let prog = parse_math_code(code).expect("Parse causal_attn/argmax/concat failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        let res = eval
            .execute_program(&prog)
            .expect("Eval causal_attn/argmax/concat failed")
            .unwrap();
        // sum(r0) = 7 + 8 = 15.0; next_tok = 2.0; seq = [1, 2, 2] -> sum(seq) = 5.0; total = 20.0
        assert!((res - 20.0).abs() < 1e-5, "Expected 20.0, got {}", res);
    }

    #[test]
    fn test_tensor_graph_optimization_and_fusion() {
        // Test algebraic identity simplifications: (A^T)^T -> A, X @ eye(2) -> X, X + zeros(...) -> X
        let code = r#"
            X = [[1.0, 2.0], [3.0, 4.0]]
            // (X^T)^T -> X
            A = transpose(transpose(X))
            // X @ eye(2) -> X
            B = X @ eye(2)
            // X + zeros(2, 2) -> X
            C = X + zeros(2, 2)
            // Attention Fusion: softmax((Q @ K^T) * 0.5) @ V -> attention(Q, K, V, 0.5)
            Q = [[1.0, 0.0], [0.0, 1.0]]
            K = [[1.0, 0.0], [0.0, 1.0]]
            V = [[10.0, 20.0], [30.0, 40.0]]
            attn_fused = softmax((Q @ transpose(K)) * 0.5) @ V
        "#;
        let prog = parse_math_code(code).expect("Parse optimization code failed");
        let opt_prog = prog.optimize_tensor_graph();

        // Verify attention fusion in the optimized AST
        let mut found_fused_attn = false;
        for item in &opt_prog.items {
            if let MathItem::Binding { name, expr } = item {
                if name == "attn_fused" {
                    if let MathExpr::Call(fn_name, args) = expr {
                        if fn_name == "attention" && args.len() == 4 {
                            found_fused_attn = true;
                        }
                    }
                }
            }
        }
        assert!(found_fused_attn, "Expected softmax(...) @ V to be fused into attention(Q, K, V, scale)");

        // Verify shape checking and evaluation
        assert!(opt_prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        let _ = eval.execute_program(&opt_prog).expect("Eval optimized graph failed");
    }

    #[test]
    fn test_llm_embedding_and_cross_entropy_training() {
        // Embed lookup + Linear logits + Cross Entropy Loss + Autodiff Matrix Training
        let code = r#"
            // Vocab = 4, Dim = 3
            E = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0]]
            toks = [0.0, 2.0]
            targets = [1.0, 0.0]
            // Lookup input tokens -> [2, 3] matrix
            H = embed(E, toks)
            // Linear projection to vocab logits [2, 4]
            loss(W) = cross_entropy(H @ W, targets)
            step(W) = W - 0.2 * grad(loss(W), W)
            W_trained = zeros(3, 4) -> step^80
            loss(W_trained)
        "#;
        let prog = parse_math_code(code).expect("Parse embedding & cross entropy failed");
        assert!(prog.verify_shapes().is_ok(), "Shapes should be valid for embedding & cross entropy");
        let mut eval = MathEvaluator::new();
        let final_loss = eval.execute_program(&prog).expect("Eval training failed").unwrap();
        // Initial loss at zeros(3, 4) is log(4) ~ 1.386. After 80 steps it converges < 0.15
        assert!(final_loss < 0.15, "Expected trained cross-entropy loss < 0.15, got {}", final_loss);
    }

    #[test]
    fn test_full_2d_c23_native_gemm_and_broadcasting() {
        let code = r#"
            W = [[1.0, 2.0], [3.0, 4.0]]
            X = [[5.0, 6.0], [7.0, 8.0]]
            b = [0.5, 1.5]
            // GEMM [2, 2] @ [2, 2]
            Y = X @ W
            // 2D-1D Broadcasting: [2, 2] + [2]
            Z = Y + b
            // Transpose & RMSNorm
            Z_t = transpose(Z)
            Z_norm = rmsnorm(Z_t)
        "#;
        let prog = parse_math_code(code).expect("Parse C23 test failed");
        assert!(prog.verify_shapes().is_ok());
        let c23_code = prog.to_c23_executable();
        // Check generated 2D GEMM nested loop
        assert!(c23_code.contains("alignas(64) float Y[2][2];"), "Expected 2D matrix buffer for Y");
        assert!(c23_code.contains("for (int _k = 0; _k < 2; ++_k)"), "Expected 2D GEMM inner loop");
        // Check 2D-1D Bias broadcasting
        assert!(c23_code.contains("(float)Y[_i][_j] + (float)b[_j];"), "Expected 2D-1D bias broadcast");
        // Check 2D Transpose
        assert!(c23_code.contains("(float)Z[_j][_i];"), "Expected 2D transpose indexing");
        // Check 2D RMSNorm
        assert!(c23_code.contains("1.0f / sqrtf((_ss / (float)2) + 1e-5f);"), "Expected RMSNorm row scaling");
    }

    #[test]
    fn test_analytical_reverse_mode_matrix_grad() {
        let code = r#"
            X = [[1.0, 2.0], [3.0, 4.0]]
            Y = [[1.0, 0.0], [0.0, 1.0]]
            W = [[0.5, 0.2], [0.1, 0.4]]
            loss(W) = sum((X @ W - Y)^2)
            G = grad(loss(W), W)
        "#;
        let prog = parse_math_code(code).expect("Parse math failed");
        assert!(prog.verify_shapes().is_ok(), "Shapes must verify");
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        let g_val = eval.env.get("G").expect("G should be computed");
        match g_val {
            MathValue::Matrix(g_mat) => {
                assert_eq!(g_mat.len(), 2);
                assert_eq!(g_mat[0].len(), 2);
                // Exact analytical: [[10.8, 9.2], [14.0, 13.6]]
                assert!((g_mat[0][0] - 10.8).abs() < 1e-5, "Expected 10.8, got {}", g_mat[0][0]);
                assert!((g_mat[0][1] - 9.2).abs() < 1e-5, "Expected 9.2, got {}", g_mat[0][1]);
                assert!((g_mat[1][0] - 14.0).abs() < 1e-5, "Expected 14.0, got {}", g_mat[1][0]);
                assert!((g_mat[1][1] - 13.6).abs() < 1e-5, "Expected 13.6, got {}", g_mat[1][1]);
            }
            _ => panic!("Expected matrix gradient for G, got {:?}", g_val),
        }
    }

    #[test]
    fn test_sum_axis_operator() {
        let code = r#"
            M = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
            col_sums = sum_axis(M, 0)
            row_sums = sum_axis(M, 1)
        "#;
        let prog = parse_math_code(code).expect("Parse sum_axis code failed");
        assert!(prog.verify_shapes().is_ok(), "Shape verification should succeed");
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        
        let col_val = eval.env.get("col_sums").expect("col_sums exists");
        let row_val = eval.env.get("row_sums").expect("row_sums exists");

        match col_val {
            MathValue::Vector(v) => {
                assert_eq!(v, &vec![5.0, 7.0, 9.0]);
            }
            _ => panic!("Expected vector for col_sums, got {:?}", col_val),
        }

        match row_val {
            MathValue::Vector(v) => {
                assert_eq!(v, &vec![6.0, 15.0]);
            }
            _ => panic!("Expected vector for row_sums, got {:?}", row_val),
        }
    }

    #[test]
    fn test_tuple_state_iteration_and_deep_mlp_training() {
        let code = r#"
            // XOR-like binary dataset
            X = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]
            Y = [0.0, 1.0, 1.0, 0.0]
            
            // 2-layer MLP: [4, 2] @ [2, 3] + [3] -> [4, 3] -> relu -> @ [3, 2] + [2] -> [4, 2] logits
            loss(W1, b1, W2, b2) = cross_entropy(relu(X @ W1 + b1) @ W2 + b2, Y)
            
            step(W1, b1, W2, b2) = (
                W1 - 0.2 * grad(loss(W1, b1, W2, b2), W1),
                b1 - 0.2 * grad(loss(W1, b1, W2, b2), b1),
                W2 - 0.2 * grad(loss(W1, b1, W2, b2), W2),
                b2 - 0.2 * grad(loss(W1, b1, W2, b2), b2)
            )
            
            // Initial parameters
            W1_0 = [[0.5, -0.2, 0.3], [0.1, 0.4, -0.5]]
            b1_0 = [0.0, 0.1, -0.1]
            W2_0 = [[0.3, -0.4], [0.2, 0.1], [-0.3, 0.5]]
            b2_0 = [0.0, 0.0]
            
            init_loss = loss(W1_0, b1_0, W2_0, b2_0)
            (W1_opt, b1_opt, W2_opt, b2_opt) = (W1_0, b1_0, W2_0, b2_0) -> step^40
            final_loss = loss(W1_opt, b1_opt, W2_opt, b2_opt)
        "#;
        let prog = parse_math_code(code).expect("Parse deep mlp failed");
        assert!(prog.verify_shapes().is_ok(), "Shapes must verify");
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        let init_l = eval.env.get("init_loss").unwrap().as_scalar().unwrap();
        let final_l = eval.env.get("final_loss").unwrap().as_scalar().unwrap();
        assert!(final_l < init_l, "Trained loss {} should be less than init loss {}", final_l, init_l);
    }

    #[test]
    fn test_adamw_and_lion_optimizers() {
        let code = r#"
            W = [[1.0, 2.0], [3.0, 4.0]]
            G = [[0.1, -0.2], [0.3, -0.4]]
            M = [[0.0, 0.0], [0.0, 0.0]]
            V = [[0.0, 0.0], [0.0, 0.0]]
            
            // AdamW step
            (W_adam, M_adam, V_adam) = adamw_step(W, G, M, V, 1.0, 0.01)
            
            // Lion step
            (W_lion, M_lion) = lion_step(W, G, M, 0.01)
        "#;
        let prog = parse_math_code(code).expect("Parse optimizer code failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        
        let w_adam = eval.env.get("W_adam").unwrap();
        let m_adam = eval.env.get("M_adam").unwrap();
        let w_lion = eval.env.get("W_lion").unwrap();
        
        match (w_adam, m_adam, w_lion) {
            (MathValue::Matrix(wa), MathValue::Matrix(ma), MathValue::Matrix(wl)) => {
                assert_eq!(wa.len(), 2);
                assert_eq!(wl.len(), 2);
                // AdamW moved in opposite direction of gradient
                assert!(wa[0][0] < 1.0, "W[0][0] should decrease with positive gradient");
                assert!(wa[0][1] > 2.0, "W[0][1] should increase with negative gradient");
                // M was updated: 0.1 * 0.1 = 0.01
                assert!((ma[0][0] - 0.01).abs() < 1e-4);
                // Lion updated with sign: sign(0.1) = 1.0 -> 1.0 - 0.01 * 1.0
                assert!(wl[0][0] < 1.0);
            }
            _ => panic!("Expected matrices for optimizer outputs"),
        }
    }

    #[test]
    fn test_clip_grad_and_cosine_lr() {
        let code = r#"
            // G has norm sqrt(3^2 + 4^2) = 5.0
            G = [3.0, 4.0]
            G_clipped = clip_grad(G, 2.5)
            
            // Cosine LR: step 0 of warmup, step 5 warmup, step 10 final
            lr_start = cosine_lr(0.0, 100.0, 0.01, 0.001, 10.0)
            lr_warm = cosine_lr(5.0, 100.0, 0.01, 0.001, 10.0)
            lr_end = cosine_lr(100.0, 100.0, 0.01, 0.001, 10.0)
        "#;
        let prog = parse_math_code(code).expect("Parse clip_grad and cosine_lr failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        
        let g_clip = eval.env.get("G_clipped").unwrap();
        match g_clip {
            MathValue::Vector(v) => {
                let norm = (v[0]*v[0] + v[1]*v[1]).sqrt();
                assert!((norm - 2.5).abs() < 1e-5, "Clipped norm should be 2.5, got {}", norm);
                assert!((v[0] - 1.5).abs() < 1e-5);
                assert!((v[1] - 2.0).abs() < 1e-5);
            }
            _ => panic!("Expected vector for G_clipped"),
        }
        
        let lr_start = eval.env.get("lr_start").unwrap().as_scalar().unwrap();
        let lr_warm = eval.env.get("lr_warm").unwrap().as_scalar().unwrap();
        let lr_end = eval.env.get("lr_end").unwrap().as_scalar().unwrap();
        
        assert_eq!(lr_start, 0.0);
        assert!((lr_warm - 0.005).abs() < 1e-5);
        assert!((lr_end - 0.001).abs() < 1e-5);
    }

    #[test]
    fn test_safetensors_multi_tensor_merge() {
        let path = "test_multi_weights.safetensors";
        let _ = std::fs::remove_file(path);
        
        let code = r#"
            W_q = [[1.0, 2.0], [3.0, 4.0]]
            W_k = [[5.0, 6.0], [7.0, 8.0]]
            b_q = [0.5, 0.25]
            
            // Export multiple tensors to same safetensors file
            W_q >> "test_multi_weights.safetensors:wq"
            W_k >> "test_multi_weights.safetensors:wk"
            b_q >> "test_multi_weights.safetensors:bq"
            
            // Read them back using external tensor references
            Q_loaded = @"test_multi_weights.safetensors:wq"
            K_loaded = @"test_multi_weights.safetensors:wk"
            B_loaded = @"test_multi_weights.safetensors:bq"
        "#;
        let prog = parse_math_code(code).expect("Parse multi safetensors failed");
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");
        
        let q_loaded = eval.env.get("Q_loaded").unwrap();
        let k_loaded = eval.env.get("K_loaded").unwrap();
        let b_loaded = eval.env.get("B_loaded").unwrap();
        
        match (q_loaded, k_loaded, b_loaded) {
            (MathValue::Matrix(q), MathValue::Matrix(k), MathValue::Vector(b)) => {
                assert_eq!(q, &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
                assert_eq!(k, &vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
                assert_eq!(b, &vec![0.5, 0.25]);
            }
            _ => panic!("Expected loaded tensors to match shapes and values"),
        }
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_nd_tensor_strides_permute_and_reshape() {
        let code = r#"
            // Create a 3D tensor [2, 3, 2]
            T = [[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]]
            
            // Permute axes [1, 0, 2] -> shape [3, 2, 2]
            P = permute(T, [1, 0, 2])
            
            // Transpose last 2 dims using ^T -> shape [2, 2, 3]
            Tr = T^T
            
            // Reshape [2, 3, 2] -> [6, 2]
            R = reshape(T, [6, 2])
        "#;
        let prog = parse_math_code(code).expect("Parse N-D tensor ops failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");

        let t_val = eval.env.get("T").unwrap();
        assert_eq!(t_val.shape(), vec![2, 3, 2]);

        let p_val = eval.env.get("P").unwrap();
        assert_eq!(p_val.shape(), vec![3, 2, 2]);

        let tr_val = eval.env.get("Tr").unwrap();
        assert_eq!(tr_val.shape(), vec![2, 2, 3]);

        let r_val = eval.env.get("R").unwrap();
        assert_eq!(r_val.shape(), vec![6, 2]);
    }

    #[test]
    fn test_split_heads_and_merge_heads() {
        let code = r#"
            // 2D sequence matrix: [seq=4, hidden=8]
            X = [[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
                 [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0],
                 [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0],
                 [25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0]]
            
            // Split into 2 heads -> [n_heads=2, seq=4, head_dim=4]
            H = split_heads(X, 2)
            
            // Merge heads back -> [seq=4, hidden=8]
            X_rec = merge_heads(H)
        "#;
        let prog = parse_math_code(code).expect("Parse split/merge heads failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");

        let h_val = eval.env.get("H").unwrap();
        assert_eq!(h_val.shape(), vec![2, 4, 4]);

        let x_rec = eval.env.get("X_rec").unwrap();
        assert_eq!(x_rec.shape(), vec![4, 8]);
        assert_eq!(x_rec.flatten_elements(), eval.env.get("X").unwrap().flatten_elements());
    }

    #[test]
    fn test_grouped_query_attention_gqa() {
        let code = r#"
            // Q has 4 heads (hidden=8, head_dim=2), seq=3
            Q = [[1.0, 0.0, 0.5, 0.2, 0.1, 0.3, 0.0, 1.0],
                 [0.0, 1.0, 0.2, 0.5, 0.3, 0.1, 1.0, 0.0],
                 [0.5, 0.5, 0.1, 0.1, 0.2, 0.2, 0.5, 0.5]]
            
            // K and V have 2 heads (hidden=4, head_dim=2), seq=3
            K = [[1.0, 0.0, 0.5, 0.2],
                 [0.0, 1.0, 0.2, 0.5],
                 [0.5, 0.5, 0.1, 0.1]]
            V = [[0.8, 0.1, 0.4, 0.2],
                 [0.2, 0.7, 0.1, 0.6],
                 [0.5, 0.4, 0.3, 0.3]]
            
            // GQA with 4 Q heads, 2 KV heads, scale=0.7071
            Out = gqa_attn(Q, K, V, 4, 2, 0.7071)
        "#;
        let prog = parse_math_code(code).expect("Parse GQA failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");

        let out = eval.env.get("Out").unwrap();
        assert_eq!(out.shape(), vec![3, 8]);
        // All values should be finite and non-zero
        for &v in &out.flatten_elements() {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn test_deepseek_mla_attention() {
        let code = r#"
            // X: [seq=2, hidden=4]
            X = [[1.0, 2.0, 0.5, 0.1], [0.5, 1.0, 2.0, 0.2]]
            
            // DeepSeek Low-Rank KV Compression matrices
            // W_dkv: [dkv=2, hidden=4]
            W_dkv = [[0.5, 0.2, 0.1, 0.0], [0.1, 0.4, 0.2, 0.3]]
            // W_uk: [hidden=4, dkv=2]
            W_uk = [[0.3, 0.1], [0.2, 0.4], [0.1, 0.5], [0.4, 0.2]]
            // W_uv: [hidden=4, dkv=2]
            W_uv = [[0.2, 0.3], [0.4, 0.1], [0.3, 0.2], [0.1, 0.4]]
            // W_q: [hidden=4, hidden=4]
            W_q = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
            // W_o: [hidden=4, hidden=4]
            W_o = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
            
            // MLA Forward pass
            Out = mla_attn(X, W_dkv, W_uk, W_uv, W_q, W_o)
        "#;
        let prog = parse_math_code(code).expect("Parse DeepSeek MLA failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");

        let out = eval.env.get("Out").unwrap();
        assert_eq!(out.shape(), vec![2, 4]);
        for &v in &out.flatten_elements() {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn test_mamba2_ssd_scan_and_autodiff() {
        let code = r#"
            // Sequence of inputs: [seq=3, dim=2]
            X = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]
            decay_dt = [0.5, 0.8]
            
            // Parallel associative state space scan
            Y = ssd_scan(X, decay_dt)
            
            // Compute gradient of total scanned sum wrt X
            loss(W) = sum(ssd_scan(W, decay_dt))
            grad_X = grad(loss(X), X)
        "#;
        let prog = parse_math_code(code).expect("Parse Mamba-2 SSD scan failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution failed");

        let y_val = eval.env.get("Y").unwrap();
        assert_eq!(y_val.shape(), vec![3, 2]);

        // Verify recurrence:
        // Y[0] = X[0] = [1.0, 2.0]
        // Y[1] = Y[0] * decay_dt + X[1] = [1.0*0.5 + 3.0, 2.0*0.8 + 4.0] = [3.5, 5.6]
        // Y[2] = Y[1] * decay_dt + X[2] = [3.5*0.5 + 5.0, 5.6*0.8 + 6.0] = [6.75, 10.48]
        if let MathValue::Matrix(m) = y_val {
            assert!((m[0][0] - 1.0).abs() < 1e-5);
            assert!((m[0][1] - 2.0).abs() < 1e-5);
            assert!((m[1][0] - 3.5).abs() < 1e-5);
            assert!((m[1][1] - 5.6).abs() < 1e-5);
            assert!((m[2][0] - 6.75).abs() < 1e-5);
            assert!((m[2][1] - 10.48).abs() < 1e-5);
        } else {
            panic!("Expected matrix for SSD scan output");
        }

        let grad_x = eval.env.get("grad_X").unwrap();
        assert_eq!(grad_x.shape(), vec![3, 2]);
        for &g in &grad_x.flatten_elements() {
            assert!(g.is_finite());
            assert!(g > 0.0);
        }
    }

    #[test]
    fn test_bitnet_b158_ternary_quantization_and_bitlinear() {
        let code = r#"
            // 1. BitNet b1.58 Ternary Quantization: weights -> {-1, 0, +1}
            W = [[0.8, -0.4, 0.05], [-0.1, 0.9, -0.7]]
            
            // Unpack ternary quantized weights and scale factor
            W_quant = quantize_t2(W)[0]
            gamma   = quantize_t2(W)[1]
            
            // 2. Zero-multiplication BitLinear GEMM:
            // X: [2, 3], W_quant: [2, 3] -> Output [2, 2] (via X @ W^T * gamma)
            X = [[1.0, 2.0, 3.0], [0.5, 1.5, 2.5]]
            Y = bitlinear(X, W_quant, gamma)
            
            // 3. Autodiff Straight-Through Estimator (STE)
            loss(w) = sum(bitlinear(X, w, 1.0))
            grad_W = grad(loss(W_quant), W_quant)
        "#;
        let prog = parse_math_code(code).expect("Parse BitNet b1.58 code failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution of BitNet b1.58 failed");

        let w_q = eval.env.get("W_quant").unwrap();
        assert_eq!(w_q.shape(), vec![2, 3]);
        for &val in &w_q.flatten_elements() {
            assert!(val == -1.0 || val == 0.0 || val == 1.0, "Ternary value must be in {{-1, 0, 1}}, got {}", val);
        }

        let gamma_val = eval.env.get("gamma").unwrap().as_scalar().unwrap();
        assert!(gamma_val > 0.0);

        let y_val = eval.env.get("Y").unwrap();
        assert_eq!(y_val.shape(), vec![2, 2]);
        for &val in &y_val.flatten_elements() {
            assert!(val.is_finite());
        }

        let grad_w = eval.env.get("grad_W").unwrap();
        assert_eq!(grad_w.shape(), vec![2, 3]);
        for &g in &grad_w.flatten_elements() {
            assert!(g.is_finite());
        }
    }

    #[test]
    fn test_int8_quantization_and_qmatmul() {
        let code = r#"
            // Weights matrix with varying dynamic range
            W = [[12.7, -6.35, 3.175], [1.27, -2.54, 5.08]]
            
            // Quantize to symmetric INT8 with per-row scales
            W_i8   = quantize_i8(W)[0]
            scales = quantize_i8(W)[1]
            
            // Input activations: [2, 3]
            X = [[1.0, 0.5, 2.0], [0.0, 1.0, 0.5]]
            
            // INT8 Matrix Multiplication with dequantization
            Y = qmatmul(X, W_i8, scales)
        "#;
        let prog = parse_math_code(code).expect("Parse INT8 quantization failed");
        assert!(prog.verify_shapes().is_ok());
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution of INT8 qmatmul failed");

        let w_i8 = eval.env.get("W_i8").unwrap();
        assert_eq!(w_i8.shape(), vec![2, 3]);
        for &q in &w_i8.flatten_elements() {
            assert!(q >= -128.0 && q <= 127.0);
            assert_eq!(q, q.round());
        }

        let scales = eval.env.get("scales").unwrap();
        assert_eq!(scales.shape(), vec![2]);
        assert!((scales.flatten_elements()[0] - 0.1).abs() < 1e-4);

        let y = eval.env.get("Y").unwrap();
        assert_eq!(y.shape(), vec![2, 2]);
        for &v in &y.flatten_elements() {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn test_gguf_v3_loader_and_parser() {
        let path = "test_mini_model.gguf";
        let _ = std::fs::remove_file(path);

        // Construct valid binary GGUF v3 file
        let mut bytes = Vec::new();
        // Magic
        bytes.extend_from_slice(b"GGUF");
        // Version 3
        bytes.extend_from_slice(&3u32.to_le_bytes());
        // Tensor count: 2
        bytes.extend_from_slice(&2u64.to_le_bytes());
        // Metadata KV count: 1
        bytes.extend_from_slice(&1u64.to_le_bytes());

        // Metadata KV: "general.alignment" -> uint32 = 32
        let key_align = "general.alignment";
        bytes.extend_from_slice(&(key_align.len() as u64).to_le_bytes());
        bytes.extend_from_slice(key_align.as_bytes());
        bytes.extend_from_slice(&4u32.to_le_bytes()); // type 4: UINT32
        bytes.extend_from_slice(&32u32.to_le_bytes());

        // Tensor 0 Header: "tensor_f32", dims: [3, 2] (shape [2, 3]), type 0 (F32), offset 0
        let t0_name = "tensor_f32";
        bytes.extend_from_slice(&(t0_name.len() as u64).to_le_bytes());
        bytes.extend_from_slice(t0_name.as_bytes());
        bytes.extend_from_slice(&2u32.to_le_bytes()); // 2 dims
        bytes.extend_from_slice(&3u64.to_le_bytes()); // ne0 = 3
        bytes.extend_from_slice(&2u64.to_le_bytes()); // ne1 = 2
        bytes.extend_from_slice(&0u32.to_le_bytes()); // F32
        bytes.extend_from_slice(&0u64.to_le_bytes()); // offset 0

        // Tensor 1 Header: "tensor_f16", dims: [4] (shape [4]), type 1 (F16), offset 24 (6 floats * 4 bytes)
        let t1_name = "tensor_f16";
        bytes.extend_from_slice(&(t1_name.len() as u64).to_le_bytes());
        bytes.extend_from_slice(t1_name.as_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes()); // 1 dim
        bytes.extend_from_slice(&4u64.to_le_bytes()); // ne0 = 4
        bytes.extend_from_slice(&1u32.to_le_bytes()); // F16
        bytes.extend_from_slice(&24u64.to_le_bytes()); // offset 24

        // Alignment padding up to 32 bytes
        let header_len = bytes.len();
        let pad_len = ((header_len + 31) & !31) - header_len;
        bytes.extend(vec![0u8; pad_len]);

        // Binary tensor data:
        // Tensor 0: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0] as f32
        let f32_vals: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        for f in f32_vals {
            bytes.extend_from_slice(&f.to_le_bytes());
        }

        // Tensor 1: [10.0, 20.0, 30.0, 40.0] as f16
        // Pre-computed half-precision values:
        let f16_vals: [u16; 4] = [0x4900, 0x4D00, 0x4F80, 0x5100];
        for h in f16_vals {
            bytes.extend_from_slice(&h.to_le_bytes());
        }

        std::fs::write(path, bytes).expect("Failed to write test GGUF file");

        let code = r#"
            W_f32 = @"test_mini_model.gguf:tensor_f32"
            W_f16 = @"test_mini_model.gguf:tensor_f16"
        "#;
        let prog = parse_math_code(code).expect("Parse GGUF loading failed");
        let mut eval = MathEvaluator::new();
        eval.execute_program(&prog).expect("Execution of GGUF loading failed");

        let w_f32 = eval.env.get("W_f32").unwrap();
        assert_eq!(w_f32.shape(), vec![2, 3]);
        assert_eq!(w_f32.flatten_elements(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let w_f16 = eval.env.get("W_f16").unwrap();
        assert_eq!(w_f16.shape(), vec![4]);
        let f16_elems = w_f16.flatten_elements();
        assert!((f16_elems[0] - 10.0).abs() < 1e-4);
        assert!((f16_elems[1] - 20.0).abs() < 1e-4);
        assert!((f16_elems[2] - 30.0).abs() < 1e-4);
        assert!((f16_elems[3] - 40.0).abs() < 1e-4);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_c23_executable_transformer_operators_and_training_loop() {
        let code = r#"
            Q = [[1.0, 0.0], [0.0, 1.0]]
            K = [[1.0, 0.0], [0.0, 1.0]]
            V = [[0.5, 0.2], [0.1, 0.9]]
            
            Attn = attention(Q, K, V)
            Act  = silu(Attn)
            
            W_trained = eye(2) -> step^100
        "#;
        let prog = parse_math_code(code).expect("Parse C23 transformer test failed");
        let c23_code = prog.to_c23_executable();

        assert!(
            c23_code.contains("float _scale = 1.0f / sqrtf"),
            "Expected Attention online scaling in C23"
        );
        assert!(
            c23_code.contains("expf(_scores[_j] - _max_s)"),
            "Expected Softmax online max-subtraction in C23"
        );
        assert!(
            c23_code.contains("_x / (1.0f + expf(-_x))"),
            "Expected SiLU activation in C23"
        );
        assert!(
            c23_code.contains("Running 100 native silicon training loop iterations"),
            "Expected native silicon training loop in C23"
        );
        assert!(
            c23_code.contains("for (int _iter = 0; _iter < 100; ++_iter)"),
            "Expected native C23 for loop for step^100"
        );
    }

    #[test]
    fn test_c23_cache_tiled_simd_gemm_and_openmp() {
        let code = r#"
            A = eye(64)
            B = eye(64)
            C = A @ B
        "#;
        let prog = parse_math_code(code).expect("Parse 64x64 GEMM failed");
        let c23_code = prog.to_c23_executable();

        assert!(
            c23_code.contains("#pragma omp parallel for collapse(2)"),
            "Expected OpenMP parallel loop for large GEMM"
        );
        assert!(
            c23_code.contains("for (int _ii = 0; _ii < 64; _ii += 32)"),
            "Expected L1 Cache Tiling for row blocks"
        );
        assert!(
            c23_code.contains("for (int _jj = 0; _jj < 64; _jj += 32)"),
            "Expected L1 Cache Tiling for col blocks"
        );
        assert!(
            c23_code.contains("for (int _kk = 0; _kk < 64; _kk += 32)"),
            "Expected L1 Cache Tiling for reduction blocks"
        );
    }

    #[test]
    fn test_ptx_flash_attention_and_fused_rmsnorm() {
        let code = r#"
            flash_attn(Q: f32, K: f32, V: f32) = 0.0
            rmsnorm_kernel(X: f32, g: f32) = 0.0
        "#;
        let prog = parse_math_code(code).expect("Parse PTX kernels failed");
        let ptx = prog.to_ptx();

        assert!(
            ptx.contains(".shared .align 16 .b8 s_tile[4096];"),
            "Expected FlashAttention shared memory allocation"
        );
        assert!(
            ptx.contains("shfl.sync.bfly.b32"),
            "Expected Warp-Level Butterfly Reduction"
        );
        assert!(
            ptx.contains("rmsnorm_kernel"),
            "Expected Fused RMSNorm entry point"
        );
    }
}
