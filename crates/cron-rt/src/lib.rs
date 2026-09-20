// ============================================================================
// CRON Native Runtime & C-ABI Shared Library (cron_rt)
// Zero-overhead foreign function interface for C++, C#, Python, and SAGI Tree Graph.
// ============================================================================

#![allow(clippy::missing_safety_doc)]

pub mod fiber;
pub mod http_server;
pub mod mmap_streamer;

pub use fiber::{FiberChannel, FiberConfig, FiberHandle, FiberScheduler, FiberState};
pub use http_server::{HttpRequest, HttpServerConfig, LiveHttpServer, RegionScratchpad};
pub use mmap_streamer::{LayerMetadata, PagedWeightStreamer, StreamerTelemetry};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::slice;

// ----------------------------------------------------------------------------
// 1. Telemetry & Hardware State Structures
// ----------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CronTelemetry {
    pub total_cycles: u64,
    pub active_cores: u32,
    pub optical_gemm_ops: u64,
    pub reversible_gate_ops: u64,
    pub stdp_synapse_updates: u64,
    pub mesh_packets_routed: u64,
    pub peak_temperature_c: u32,
    pub dram_bandwidth_saved_mb: f64,
    pub success: bool,
}

impl Default for CronTelemetry {
    fn default() -> Self {
        Self {
            total_cycles: 0,
            active_cores: 256,
            optical_gemm_ops: 0,
            reversible_gate_ops: 0,
            stdp_synapse_updates: 0,
            mesh_packets_routed: 0,
            peak_temperature_c: 42,
            dram_bandwidth_saved_mb: 0.0,
            success: false,
        }
    }
}

// ----------------------------------------------------------------------------
// 2. VM Handle
// ----------------------------------------------------------------------------

pub struct CronVmHandle {
    pub simulator: cron_vm::Simulator,
}

// ----------------------------------------------------------------------------
// 3. Zero-Copy 4D Tensor
// ----------------------------------------------------------------------------

#[repr(C)]
pub struct CronTensor4D {
    pub shape: [i32; 4],   // [N, C, H, W]
    pub strides: [i32; 4], // [C*H*W, H*W, W, 1]
    pub elem_type: i32,    // 0 = f32, 1 = f64, 2 = i32, 3 = i64
    pub data: *mut u8,
    pub total_elements: usize,
    pub size_bytes: usize,
}

// ----------------------------------------------------------------------------
// 4. Metadata & Versioning API
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn cron_version() -> *const c_char {
    static VERSION: &[u8] = b"1.0.0-tier1-monomorphized\0";
    VERSION.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn cron_target_info() -> *const c_char {
    static TARGET: &[u8] = b"256-Core 4D-Torus Neuromorphic Photonic Processor (4x4x4x4 Mesh)\0";
    TARGET.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn cron_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// ----------------------------------------------------------------------------
// 5. In-Memory Compilation API (Zero-Disk I/O)
// ----------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cron_compile_source(
    cr_source: *const c_char,
    out_error: *mut *mut c_char,
) -> *mut c_char {
    if cr_source.is_null() {
        return std::ptr::null_mut();
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                *out_error = CString::new(format!("UTF-8 decode error: {}", e)).unwrap().into_raw();
            }
            return std::ptr::null_mut();
        }
    };

    match cronc::compile_source(src) {
        Ok(cl_output) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            CString::new(cl_output).unwrap().into_raw()
        }
        Err(err) => {
            if !out_error.is_null() {
                *out_error = CString::new(err).unwrap().into_raw();
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_compile_to_c23(
    cr_source: *const c_char,
    out_error: *mut *mut c_char,
) -> *mut c_char {
    if cr_source.is_null() {
        return std::ptr::null_mut();
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                *out_error = CString::new(format!("UTF-8 decode error: {}", e)).unwrap().into_raw();
            }
            return std::ptr::null_mut();
        }
    };

    match cronc::compile_to_c23(src) {
        Ok(c_code) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            CString::new(c_code).unwrap().into_raw()
        }
        Err(err) => {
            if !out_error.is_null() {
                *out_error = CString::new(err).unwrap().into_raw();
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_compile_to_llvm(
    cr_source: *const c_char,
    out_error: *mut *mut c_char,
) -> *mut c_char {
    if cr_source.is_null() {
        return std::ptr::null_mut();
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                *out_error = CString::new(format!("UTF-8 decode error: {}", e)).unwrap().into_raw();
            }
            return std::ptr::null_mut();
        }
    };

    match cronc::compile_to_llvm(src) {
        Ok(llvm_ir) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            CString::new(llvm_ir).unwrap().into_raw()
        }
        Err(err) => {
            if !out_error.is_null() {
                *out_error = CString::new(err).unwrap().into_raw();
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_check_syntax(
    cr_source: *const c_char,
    out_error: *mut *mut c_char,
) -> bool {
    if cr_source.is_null() {
        return false;
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let diags = cronc::check_source_diagnostics(src);
    if diags.is_empty() {
        if !out_error.is_null() {
            *out_error = std::ptr::null_mut();
        }
        true
    } else {
        if !out_error.is_null() {
            let msg = diags.iter().map(|d| d.render(None)).collect::<Vec<_>>().join("\n");
            *out_error = CString::new(msg).unwrap().into_raw();
        }
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_jit_execute(
    cr_source: *const c_char,
    out_error: *mut *mut c_char,
) -> i64 {
    if cr_source.is_null() {
        if !out_error.is_null() {
            *out_error = CString::new("Null source pointer provided").unwrap().into_raw();
        }
        return -1;
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                *out_error = CString::new(format!("UTF-8 decode error: {}", e)).unwrap().into_raw();
            }
            return -1;
        }
    };

    match cronc::execute_jit(src) {
        Ok(res) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            res
        }
        Err(err) => {
            if !out_error.is_null() {
                *out_error = CString::new(err).unwrap().into_raw();
            }
            -1
        }
    }
}

// ----------------------------------------------------------------------------
// 6. In-Process 256-Core 4D-Torus Simulator Lifecycle API
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn cron_vm_create() -> *mut CronVmHandle {
    let handle = Box::new(CronVmHandle {
        simulator: cron_vm::Simulator::new(),
    });
    Box::into_raw(handle)
}

#[no_mangle]
pub unsafe extern "C" fn cron_vm_destroy(handle: *mut CronVmHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_vm_reset(handle: *mut CronVmHandle) {
    if let Some(h) = handle.as_mut() {
        h.simulator = cron_vm::Simulator::new();
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_vm_run_cl(
    handle: *mut CronVmHandle,
    cl_code: *const c_char,
    out_telemetry: *mut CronTelemetry,
) -> bool {
    if handle.is_null() || cl_code.is_null() {
        return false;
    }
    let cl_str = match CStr::from_ptr(cl_code).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let h = match handle.as_mut() {
        Some(h) => h,
        None => return false,
    };

    h.simulator.load_machine_code(cl_str);
    h.simulator.run();

    if !out_telemetry.is_null() {
        let st = &h.simulator.stats;
        *out_telemetry = CronTelemetry {
            total_cycles: st.total_cycles as u64,
            active_cores: 256,
            optical_gemm_ops: st.optical_gemm_ops as u64,
            reversible_gate_ops: st.reversible_gate_ops as u64,
            stdp_synapse_updates: st.stdp_synapse_updates as u64,
            mesh_packets_routed: st.mesh_packets_routed as u64,
            peak_temperature_c: st.peak_temperature_c,
            dram_bandwidth_saved_mb: st.dram_bandwidth_saved_mb,
            success: true,
        };
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn cron_vm_run_cr(
    handle: *mut CronVmHandle,
    cr_source: *const c_char,
    out_telemetry: *mut CronTelemetry,
) -> bool {
    if handle.is_null() || cr_source.is_null() {
        return false;
    }
    let src = match CStr::from_ptr(cr_source).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let cl_code = match cronc::compile_source(src) {
        Ok(code) => code,
        Err(_) => {
            if !out_telemetry.is_null() {
                *out_telemetry = CronTelemetry::default();
            }
            return false;
        }
    };

    let h = match handle.as_mut() {
        Some(h) => h,
        None => return false,
    };

    h.simulator.load_machine_code(&cl_code);
    h.simulator.run();

    if !out_telemetry.is_null() {
        let st = &h.simulator.stats;
        *out_telemetry = CronTelemetry {
            total_cycles: st.total_cycles as u64,
            active_cores: 256,
            optical_gemm_ops: st.optical_gemm_ops as u64,
            reversible_gate_ops: st.reversible_gate_ops as u64,
            stdp_synapse_updates: st.stdp_synapse_updates as u64,
            mesh_packets_routed: st.mesh_packets_routed as u64,
            peak_temperature_c: st.peak_temperature_c,
            dram_bandwidth_saved_mb: st.dram_bandwidth_saved_mb,
            success: true,
        };
    }

    true
}

// ----------------------------------------------------------------------------
// 7. Zero-Copy 4D Tensor Memory Bridge
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn cron_tensor4d_create(
    n: i32,
    c: i32,
    h: i32,
    w: i32,
    elem_type: i32,
) -> *mut CronTensor4D {
    let elem_size = match elem_type {
        0 => std::mem::size_of::<f32>(),
        1 => std::mem::size_of::<f64>(),
        2 => std::mem::size_of::<i32>(),
        3 => std::mem::size_of::<i64>(),
        _ => std::mem::size_of::<f32>(),
    };

    let total_elements = (n.max(1) * c.max(1) * h.max(1) * w.max(1)) as usize;
    let size_bytes = total_elements * elem_size;

    // 64-byte cache-line aligned allocation
    let layout = std::alloc::Layout::from_size_align(size_bytes, 64).unwrap_or(
        std::alloc::Layout::from_size_align(64, 64).unwrap()
    );
    let data_ptr = unsafe { std::alloc::alloc_zeroed(layout) };

    let strides = [
        c * h * w,
        h * w,
        w,
        1,
    ];

    let tensor = Box::new(CronTensor4D {
        shape: [n, c, h, w],
        strides,
        elem_type,
        data: data_ptr,
        total_elements,
        size_bytes,
    });

    Box::into_raw(tensor)
}

#[no_mangle]
pub unsafe extern "C" fn cron_tensor4d_data_ptr(tensor: *mut CronTensor4D) -> *mut c_void {
    if let Some(t) = tensor.as_mut() {
        t.data as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_tensor4d_shape(tensor: *const CronTensor4D, out_shape: *mut i32) {
    if let (Some(t), Some(out)) = (tensor.as_ref(), out_shape.as_mut()) {
        let out_slice = slice::from_raw_parts_mut(out, 4);
        out_slice.copy_from_slice(&t.shape);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_tensor4d_destroy(tensor: *mut CronTensor4D) {
    if !tensor.is_null() {
        let t = Box::from_raw(tensor);
        if !t.data.is_null() {
            let layout = std::alloc::Layout::from_size_align(t.size_bytes, 64).unwrap_or(
                std::alloc::Layout::from_size_align(64, 64).unwrap()
            );
            std::alloc::dealloc(t.data, layout);
        }
    }
}

// ----------------------------------------------------------------------------
// 8. Hardware Photonic MZI & STDP Accelerators
// ----------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cron_photonic_mzi_gemm(
    amps: *const f64,
    phases: *const f64,
    count: usize,
    out_amps: *mut f64,
    out_phases: *mut f64,
) {
    if amps.is_null() || phases.is_null() || out_amps.is_null() || out_phases.is_null() {
        return;
    }

    let in_a = slice::from_raw_parts(amps, count);
    let in_p = slice::from_raw_parts(phases, count);
    let out_a = slice::from_raw_parts_mut(out_amps, count);
    let out_p = slice::from_raw_parts_mut(out_phases, count);

    // 0-cycle Optical Phase Shift & Wavefront Interference Simulation
    for i in 0..count {
        let theta = in_p[i];
        let c = theta.cos();
        let s = theta.sin();
        // Mach-Zehnder optical transfer matrix: [cos(theta) -sin(theta); sin(theta) cos(theta)]
        out_a[i] = in_a[i] * c.abs();
        out_p[i] = (theta + s * 0.1).fract() * std::f64::consts::PI;
    }
}

// ----------------------------------------------------------------------------
// 9. CSP Typed Channels & 4D-Torus Mesh Concurrency
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn cron_channel_create_c(capacity: u32) -> u64 {
    cronc::jit_backend::jit_channel_new(capacity as i64)
}

#[no_mangle]
pub extern "C" fn cron_channel_send_c(channel_id: u64, val: i64) -> i64 {
    cronc::jit_backend::jit_channel_send(channel_id, val)
}

#[no_mangle]
pub extern "C" fn cron_channel_recv_c(channel_id: u64) -> i64 {
    cronc::jit_backend::jit_channel_recv(channel_id)
}

#[no_mangle]
pub extern "C" fn cron_channel_try_recv_c(channel_id: u64) -> i64 {
    cronc::jit_backend::jit_channel_try_recv(channel_id)
}

#[no_mangle]
pub extern "C" fn cron_channel_close_c(channel_id: u64) {
    cronc::jit_backend::jit_channel_close(channel_id);
}

#[no_mangle]
pub extern "C" fn cron_torus_distance_c(c1: i64, c2: i64) -> i64 {
    cronc::jit_backend::jit_torus_distance(c1, c2)
}

// ----------------------------------------------------------------------------
// 10. AI BitNet 1.58-Bit Ternary & CL 2.0 Native Silicon C-ABI
// ----------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cron_ternary_quantize(
    weights: *const f32,
    count: usize,
    out_packed: *mut u8,
    out_scale: *mut f32,
) -> bool {
    if weights.is_null() || out_packed.is_null() || out_scale.is_null() || count == 0 {
        return false;
    }

    let slice = slice::from_raw_parts(weights, count);
    let (ternary, params) = cronc::model_importer::ternary::quantize_to_ternary(slice);
    let packed = cronc::model_importer::ternary::pack_ternary_2bit(&ternary);

    let out_slice = slice::from_raw_parts_mut(out_packed, packed.len());
    out_slice.copy_from_slice(&packed);
    *out_scale = params.scale;

    true
}

#[no_mangle]
pub unsafe extern "C" fn cron_ternary_dot_product(
    packed_weights: *const u8,
    activations: *const f32,
    count: usize,
    scale: f32,
) -> f32 {
    if packed_weights.is_null() || activations.is_null() || count == 0 {
        return 0.0;
    }

    let packed_len = (count + 3) / 4;
    let packed_slice = slice::from_raw_parts(packed_weights, packed_len);
    let act_slice = slice::from_raw_parts(activations, count);

    cronc::model_importer::ternary::ternary_dot_product(packed_slice, act_slice, scale)
}

#[no_mangle]
pub unsafe extern "C" fn cron_cl_audit(
    cl_source: *const c_char,
    out_report_json: *mut *mut c_char,
) -> bool {
    if cl_source.is_null() || out_report_json.is_null() {
        return false;
    }

    let c_str = CStr::from_ptr(cl_source);
    let cl_code = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let verify_res = cronc::cl_lang::verify_cl_program(cl_code);
    let audit_res = cronc::cl_lang::audit_alphabet_coverage(cl_code);

    let (total_bundles, total_slots, hazards_count) = match verify_res {
        Ok(v) => (v.total_bundles, v.total_slots, v.hazards.len()),
        Err(_) => (0, 0, 0),
    };

    let json = format!(
        "{{\"total_bundles\":{},\"total_slots\":{},\"hazards\":{},\"unique_chars\":{},\"coverage_pct\":{:.2},\"entropy\":{:.4}}}",
        total_bundles,
        total_slots,
        hazards_count,
        audit_res.unique_characters_used,
        audit_res.coverage_percentage,
        audit_res.entropy_bits_per_char
    );

    if let Ok(c_json) = CString::new(json) {
        *out_report_json = c_json.into_raw();
        true
    } else {
        false
    }
}

// ----------------------------------------------------------------------------
// 11. BPE Tokenizer & Autoregressive Inference C-ABI
// ----------------------------------------------------------------------------

pub struct CronBpeTokenizerOpaque {
    pub inner: cronc::bpe_tokenizer::BpeTokenizer,
}

#[no_mangle]
pub extern "C" fn cron_bpe_create_default() -> *mut CronBpeTokenizerOpaque {
    let tok = Box::new(CronBpeTokenizerOpaque {
        inner: cronc::bpe_tokenizer::BpeTokenizer::from_default_vocab(),
    });
    Box::into_raw(tok)
}

#[no_mangle]
pub unsafe extern "C" fn cron_bpe_encode(
    tokenizer: *mut CronBpeTokenizerOpaque,
    text: *const c_char,
    out_ids: *mut u32,
    max_ids: usize,
    out_len: *mut usize,
) -> bool {
    if tokenizer.is_null() || text.is_null() || out_ids.is_null() || out_len.is_null() {
        return false;
    }

    let c_str = CStr::from_ptr(text);
    let s = match c_str.to_str() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let tok = &(*tokenizer).inner;
    let ids = tok.encode(s);
    let copy_count = ids.len().min(max_ids);
    let out_slice = slice::from_raw_parts_mut(out_ids, copy_count);
    out_slice.copy_from_slice(&ids[..copy_count]);
    *out_len = ids.len();

    true
}

#[no_mangle]
pub unsafe extern "C" fn cron_bpe_decode(
    tokenizer: *mut CronBpeTokenizerOpaque,
    ids: *const u32,
    num_ids: usize,
    out_str: *mut *mut c_char,
) -> bool {
    if tokenizer.is_null() || ids.is_null() || out_str.is_null() {
        return false;
    }

    let tok = &(*tokenizer).inner;
    let slice = slice::from_raw_parts(ids, num_ids);
    let decoded = tok.decode(slice);

    if let Ok(c_res) = CString::new(decoded) {
        *out_str = c_res.into_raw();
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_bpe_free(tokenizer: *mut CronBpeTokenizerOpaque) {
    if !tokenizer.is_null() {
        drop(Box::from_raw(tokenizer));
    }
}

// ----------------------------------------------------------------------------
// 12. Multi-Modal Vision & Audio Silicon C-ABI
// ----------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cron_vision_extract_patches(
    rgb_data: *const u8,
    width: usize,
    height: usize,
    channels: usize,
    patch_size: usize,
    embed_dim: usize,
    out_embeddings: *mut f32,
    max_elements: usize,
    out_patch_count: *mut usize,
) -> bool {
    if rgb_data.is_null() || out_embeddings.is_null() || out_patch_count.is_null() {
        return false;
    }

    let total_bytes = width * height * channels;
    let rgb_slice = slice::from_raw_parts(rgb_data, total_bytes);

    let config = cronc::cl_multimodal::VisionConfig {
        width,
        height,
        channels,
        patch_size,
        embed_dim,
        is_ternary: true,
    };

    let patches = cronc::cl_multimodal::VisionPatchProcessor::extract_patches(rgb_slice, &config);
    let projected = cronc::cl_multimodal::VisionPatchProcessor::project_patches(&patches, &config);

    *out_patch_count = projected.len();
    let total_emb_elements = projected.len() * embed_dim;
    let copy_count = total_emb_elements.min(max_elements);

    let out_slice = slice::from_raw_parts_mut(out_embeddings, copy_count);
    let mut offset = 0;
    for emb in &projected {
        for &val in emb {
            if offset < copy_count {
                out_slice[offset] = val;
                offset += 1;
            }
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn cron_audio_mel_spectrogram(
    pcm_samples: *const f32,
    num_samples: usize,
    sample_rate: usize,
    mel_bands: usize,
    embed_dim: usize,
    out_embeddings: *mut f32,
    max_elements: usize,
    out_frame_count: *mut usize,
) -> bool {
    if pcm_samples.is_null() || out_embeddings.is_null() || out_frame_count.is_null() {
        return false;
    }

    let pcm_slice = slice::from_raw_parts(pcm_samples, num_samples);
    let config = cronc::cl_multimodal::AudioConfig {
        sample_rate,
        fft_size: 512,
        hop_length: 160,
        mel_bands,
        embed_dim,
    };

    let mel_frames = cronc::cl_multimodal::AudioSpectrogramProcessor::compute_mel_spectrogram(pcm_slice, &config);
    let projected = cronc::cl_multimodal::AudioSpectrogramProcessor::project_audio_frames(&mel_frames, embed_dim);

    *out_frame_count = projected.len();
    let total_elements = projected.len() * embed_dim;
    let copy_count = total_elements.min(max_elements);

    let out_slice = slice::from_raw_parts_mut(out_embeddings, copy_count);
    let mut offset = 0;
    for frame in &projected {
        for &val in frame {
            if offset < copy_count {
                out_slice[offset] = val;
                offset += 1;
            }
        }
    }

    true
}

// ----------------------------------------------------------------------------
// 13. 256-Core Autonomous Swarm Mesh C-ABI
// ----------------------------------------------------------------------------

pub struct CronSwarmOpaque {
    pub inner: cronc::cl_swarm::SwarmMesh,
}

#[no_mangle]
pub extern "C" fn cron_swarm_create_256() -> *mut CronSwarmOpaque {
    let swarm = Box::new(CronSwarmOpaque {
        inner: cronc::cl_swarm::SwarmMesh::new_256(),
    });
    Box::into_raw(swarm)
}

#[no_mangle]
pub unsafe extern "C" fn cron_swarm_execute_task(
    swarm: *mut CronSwarmOpaque,
    task_desc: *const c_char,
    out_report_json: *mut *mut c_char,
) -> bool {
    if swarm.is_null() || task_desc.is_null() || out_report_json.is_null() {
        return false;
    }

    let c_str = CStr::from_ptr(task_desc);
    let task = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let report = (*swarm).inner.execute_task(task);

    let json = format!(
        "{{\"task\":\"{}\",\"consensus_achieved\":{},\"consensus_score\":{:.2},\"active_agents\":{},\"total_packets\":{},\"avg_hops\":{:.2},\"max_hops\":{},\"latency_us\":{:.2}}}",
        report.task.replace('"', "\\\""),
        report.consensus_achieved,
        report.telemetry.consensus_score,
        report.telemetry.active_agents,
        report.telemetry.total_packets_routed,
        report.telemetry.avg_hop_count,
        report.telemetry.max_hop_count,
        report.telemetry.consensus_latency_us
    );

    if let Ok(c_json) = CString::new(json) {
        *out_report_json = c_json.into_raw();
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn cron_swarm_free(swarm: *mut CronSwarmOpaque) {
    if !swarm.is_null() {
        drop(Box::from_raw(swarm));
    }
}
