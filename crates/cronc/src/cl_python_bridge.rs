//! C-ABI Foreign Function Interface (FFI) for Python, PyTorch, C, and Julia Integration
//!
//! Exposes thread-safe native C-compatible entry points for `.cl` machine code execution,
//! HDC hyperdimensional holographic vector similarity, and Living AGI Mind orchestration.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float, c_void};
use crate::cl_agi_orchestrator::LivingAgiMind;
use crate::cl_hyperdimensional::{HyperVector, HDC_DIM};
use crate::cl_jit::run_cl_jit;

/// Create a new instance of the Living AGI Cognitive Mind
#[no_mangle]
pub extern "C" fn cron_mind_create() -> *mut c_void {
    let mind = Box::new(LivingAgiMind::new());
    Box::into_raw(mind) as *mut c_void
}

/// Free a Living AGI Cognitive Mind instance
#[no_mangle]
pub extern "C" fn cron_mind_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut LivingAgiMind);
        }
    }
}

/// Process a text input string through the Living AGI Mind and return synthesized response
#[no_mangle]
pub extern "C" fn cron_mind_process(
    ptr: *mut c_void,
    input_cstr: *const c_char,
    out_buf: *mut c_char,
    max_len: usize,
) -> i32 {
    if ptr.is_null() || input_cstr.is_null() || out_buf.is_null() || max_len == 0 {
        return -1;
    }

    unsafe {
        let mind = &mut *(ptr as *mut LivingAgiMind);
        let c_str = CStr::from_ptr(input_cstr);
        let input_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let result = mind.process_turn(input_str, 0.5, false);
        let c_response = match CString::new(result.response_text) {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let bytes = c_response.as_bytes_with_nul();
        let copy_len = bytes.len().min(max_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, copy_len);
        // Ensure null-terminated
        *out_buf.add(copy_len - 1) = 0;
        0
    }
}

/// Get current neuromodulatory chemical concentrations (DA, 5-HT, NE, ACh)
#[no_mangle]
pub extern "C" fn cron_mind_get_chemicals(
    ptr: *mut c_void,
    out_da: *mut c_float,
    out_5ht: *mut c_float,
    out_ne: *mut c_float,
    out_ach: *mut c_float,
) -> i32 {
    if ptr.is_null() {
        return -1;
    }
    unsafe {
        let mind = &*(ptr as *mut LivingAgiMind);
        if !out_da.is_null() {
            *out_da = mind.neuro.current_state.dopamine;
        }
        if !out_5ht.is_null() {
            *out_5ht = mind.neuro.current_state.serotonin;
        }
        if !out_ne.is_null() {
            *out_ne = mind.neuro.current_state.norepinephrine;
        }
        if !out_ach.is_null() {
            *out_ach = mind.neuro.current_state.acetylcholine;
        }
        0
    }
}

/// Compute $O(1)$ Cosine Similarity between two 1024-trit hypervectors
#[no_mangle]
pub extern "C" fn cron_hdc_similarity(vec_a: *const i8, vec_b: *const i8) -> c_float {
    if vec_a.is_null() || vec_b.is_null() {
        return 0.0;
    }
    unsafe {
        let mut a = HyperVector::zero();
        let mut b = HyperVector::zero();
        for i in 0..HDC_DIM {
            a.set_trit(i, *vec_a.add(i));
            b.set_trit(i, *vec_b.add(i));
        }
        a.similarity(&b)
    }
}

/// Execute raw `.cl` source code in RAM via high-performance JIT and return register outputs
#[no_mangle]
pub extern "C" fn cron_cl_jit_exec(cl_source_cstr: *const c_char, out_regs: *mut u32) -> i32 {
    if cl_source_cstr.is_null() || out_regs.is_null() {
        return -1;
    }
    unsafe {
        let c_str = CStr::from_ptr(cl_source_cstr);
        let source = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        match run_cl_jit(source) {
            Ok(core) => {
                for (i, &r) in core.r.iter().take(16).enumerate() {
                    *out_regs.add(i) = r;
                }
                0
            }
            Err(_) => -3,
        }
    }
}

/// Quantize a continuous float weight tensor into BitNet b1.58 ternary weights {-1, 0, +1}
#[no_mangle]
pub extern "C" fn cron_sphere_quantize_tensor(
    weights: *const c_float,
    count: usize,
    out_ternary: *mut i8,
    out_scale: *mut c_float,
) -> i32 {
    if weights.is_null() || out_ternary.is_null() || count == 0 {
        return -1;
    }
    unsafe {
        let slice = std::slice::from_raw_parts(weights, count);
        let (ternary, params) = crate::model_importer::ternary::quantize_to_ternary(slice);
        std::ptr::copy_nonoverlapping(ternary.as_ptr(), out_ternary, count);
        if !out_scale.is_null() {
            *out_scale = params.scale;
        }
        0
    }
}

/// Convert a 2D PyTorch/NumPy weight matrix directly into 4D Spherical CL microcode
#[no_mangle]
pub extern "C" fn cron_sphere_export_layer_cl(
    layer_name: *const c_char,
    weights: *const c_float,
    rows: usize,
    cols: usize,
    out_cl_buf: *mut c_char,
    max_len: usize,
) -> i32 {
    if weights.is_null() || out_cl_buf.is_null() || rows == 0 || cols == 0 || max_len == 0 {
        return -1;
    }
    unsafe {
        let name_str = if layer_name.is_null() {
            "pytorch_layer"
        } else {
            CStr::from_ptr(layer_name).to_str().unwrap_or("pytorch_layer")
        };

        let count = rows * cols;
        let slice = std::slice::from_raw_parts(weights, count);
        let sphere_mat = crate::cl_sphere_infer::SphereWeightMatrix::from_floats(name_str, rows, cols, slice);
        let dummy_acts = vec![1i32; cols.min(16)];
        let cl_code = crate::cl_sphere_infer::compile_weights_to_sphere_cl(
            &sphere_mat,
            &dummy_acts,
            &crate::cl_sphere_infer::SphereCompileOptions::default(),
        );

        let c_str = match CString::new(cl_code) {
            Ok(s) => s,
            Err(_) => return -2,
        };
        let bytes = c_str.as_bytes_with_nul();
        let copy_len = bytes.len().min(max_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_cl_buf, copy_len);
        *out_cl_buf.add(copy_len - 1) = 0;
        0
    }
}

/// Convert a SafeTensors, GGUF, or PyTorch model file and output JSON conversion telemetry
#[no_mangle]
pub extern "C" fn cron_sphere_convert_file(
    in_path_cstr: *const c_char,
    out_path_cstr: *const c_char,
    out_report_buf: *mut c_char,
    max_len: usize,
) -> i32 {
    if in_path_cstr.is_null() {
        return -1;
    }
    unsafe {
        let in_path = match CStr::from_ptr(in_path_cstr).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };
        let out_path = if out_path_cstr.is_null() {
            None
        } else {
            CStr::from_ptr(out_path_cstr).to_str().ok()
        };

        match crate::cl_convert::convert_ai_model_file(
            in_path,
            out_path,
            &crate::cl_convert::ModelConvertOptions::default(),
        ) {
            Ok(rep) => {
                if !out_report_buf.is_null() && max_len > 0 {
                    let json = rep.to_json();
                    if let Ok(c_json) = CString::new(json) {
                        let bytes = c_json.as_bytes_with_nul();
                        let copy_len = bytes.len().min(max_len);
                        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_report_buf, copy_len);
                        *out_report_buf.add(copy_len - 1) = 0;
                    }
                }
                0
            }
            Err(_) => -3,
        }
    }
}

/// Ultra-fast native C-ABI BitNet b1.58 ternary matrix-vector inference directly over raw buffers
#[no_mangle]
pub extern "C" fn cron_infer_forward_ffi(
    packed_weights: *const u8,
    inputs: *const c_float,
    scale: c_float,
    rows: usize,
    cols: usize,
    out_activations: *mut c_float,
) -> i32 {
    if packed_weights.is_null() || inputs.is_null() || out_activations.is_null() || rows == 0 || cols == 0 {
        return -1;
    }
    unsafe {
        let input_slice = std::slice::from_raw_parts(inputs, cols);
        let out_slice = std::slice::from_raw_parts_mut(out_activations, rows);
        let bytes_per_row = (cols + 3) / 4;

        for r in 0..rows {
            let row_bytes = std::slice::from_raw_parts(packed_weights.add(r * bytes_per_row), bytes_per_row);
            let mut acc: i32 = 0;
            for c in 0..cols {
                let byte_val = row_bytes[c / 4];
                let code = (byte_val >> ((c % 4) * 2)) & 0x03;
                let trit: i8 = match code {
                    1 => 1,
                    2 => -1,
                    _ => 0,
                };
                if trit > 0 {
                    acc += input_slice[c] as i32;
                } else if trit < 0 {
                    acc -= input_slice[c] as i32;
                }
            }
            out_slice[r] = (acc as f32) * scale;
        }
        0
    }
}

// ============================================================================
// DLPack Standard Protocol Types for Zero-Copy PyTorch / JAX Interoperability
// ============================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DLDeviceType {
    KDLCpu = 1,
    KDLCuda = 2,
    KDLCudaHost = 3,
    KDLOpenCl = 4,
    KDLVulkan = 7,
    KDLMetal = 8,
    KDLVpi = 9,
    KDLRocm = 10,
    KDLExtDev = 12,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DLDevice {
    pub device_type: DLDeviceType,
    pub device_id: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DLDataType {
    pub code: u8,
    pub bits: u8,
    pub lanes: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DLTensor {
    pub data: *mut c_void,
    pub device: DLDevice,
    pub ndim: i32,
    pub dtype: DLDataType,
    pub shape: *mut i64,
    pub strides: *mut i64,
    pub byte_offset: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct DLManagedTensor {
    pub dl_tensor: DLTensor,
    pub manager_ctx: *mut c_void,
    pub deleter: Option<unsafe extern "C" fn(self_: *mut DLManagedTensor)>,
}

unsafe extern "C" fn dlpack_deleter_simple(tensor: *mut DLManagedTensor) {
    if !tensor.is_null() {
        let boxed = Box::from_raw(tensor);
        if !boxed.dl_tensor.shape.is_null() {
            let _ = Vec::from_raw_parts(boxed.dl_tensor.shape, boxed.dl_tensor.ndim as usize, boxed.dl_tensor.ndim as usize);
        }
        if !boxed.dl_tensor.strides.is_null() {
            let _ = Vec::from_raw_parts(boxed.dl_tensor.strides, boxed.dl_tensor.ndim as usize, boxed.dl_tensor.ndim as usize);
        }
    }
}

/// Zero-Copy DLPack tensor exporter for zero-overhead Python/PyTorch/JAX interop
#[no_mangle]
pub extern "C" fn cron_dlpack_tensor_export(
    data_ptr: *mut c_float,
    rows: i64,
    cols: i64,
) -> *mut DLManagedTensor {
    if data_ptr.is_null() || rows <= 0 || cols <= 0 {
        return std::ptr::null_mut();
    }

    let mut shape = vec![rows, cols];
    let mut strides = vec![cols, 1];

    let shape_ptr = shape.as_mut_ptr();
    let strides_ptr = strides.as_mut_ptr();
    std::mem::forget(shape);
    std::mem::forget(strides);

    let dl_tensor = DLTensor {
        data: data_ptr as *mut c_void,
        device: DLDevice {
            device_type: DLDeviceType::KDLCpu,
            device_id: 0,
        },
        ndim: 2,
        dtype: DLDataType {
            code: 2, // kDLFloat
            bits: 32,
            lanes: 1,
        },
        shape: shape_ptr,
        strides: strides_ptr,
        byte_offset: 0,
    };

    let managed = Box::new(DLManagedTensor {
        dl_tensor,
        manager_ctx: std::ptr::null_mut(),
        deleter: Some(dlpack_deleter_simple),
    });

    Box::into_raw(managed)
}

