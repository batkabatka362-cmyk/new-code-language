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
