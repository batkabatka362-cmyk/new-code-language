use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use cron_rt::*;

#[test]
fn test_ffi_version_and_metadata() {
    let ver_ptr = cron_version();
    assert!(!ver_ptr.is_null());
    let ver_str = unsafe { CStr::from_ptr(ver_ptr).to_str().unwrap() };
    assert!(ver_str.contains("1.0.0"));

    let target_ptr = cron_target_info();
    assert!(!target_ptr.is_null());
    let target_str = unsafe { CStr::from_ptr(target_ptr).to_str().unwrap() };
    assert!(target_str.contains("4D-Torus"));
}

#[test]
fn test_ffi_in_memory_compilation() {
    let source = CString::new(r#"
    .MODULE FfiCompileTest
    _main:
        let a: i32 = 12
        let b: i32 = 34
        let res: i32 = a + b
    .END
    "#).unwrap();

    let mut err_ptr: *mut c_char = std::ptr::null_mut();

    // 1. Compile to VLIW
    let vliw_ptr = unsafe { cron_compile_source(source.as_ptr(), &mut err_ptr) };
    assert!(!vliw_ptr.is_null());
    assert!(err_ptr.is_null());
    let vliw_str = unsafe { CStr::from_ptr(vliw_ptr).to_str().unwrap() };
    assert!(vliw_str.contains("B0001:"));
    unsafe { cron_string_free(vliw_ptr); }

    // 2. Compile to C23
    let c23_ptr = unsafe { cron_compile_to_c23(source.as_ptr(), &mut err_ptr) };
    assert!(!c23_ptr.is_null());
    let c23_str = unsafe { CStr::from_ptr(c23_ptr).to_str().unwrap() };
    assert!(c23_str.contains("int32_t a = 12;"));
    unsafe { cron_string_free(c23_ptr); }

    // 3. Compile to LLVM
    let llvm_ptr = unsafe { cron_compile_to_llvm(source.as_ptr(), &mut err_ptr) };
    assert!(!llvm_ptr.is_null());
    let llvm_str = unsafe { CStr::from_ptr(llvm_ptr).to_str().unwrap() };
    assert!(llvm_str.contains("define i32 @main"));
    unsafe { cron_string_free(llvm_ptr); }

    // 4. Check syntax
    let ok = unsafe { cron_check_syntax(source.as_ptr(), &mut err_ptr) };
    assert!(ok);
    assert!(err_ptr.is_null());
}

#[test]
fn test_ffi_vm_execution_and_telemetry() {
    let source = CString::new(r#"
    .MODULE FfiVmTest
    _main:
        let lin photon = 100
        consume(photon)
    .END
    "#).unwrap();

    let vm = cron_vm_create();
    assert!(!vm.is_null());

    let mut telemetry = CronTelemetry::default();
    let ok = unsafe { cron_vm_run_cr(vm, source.as_ptr(), &mut telemetry) };
    assert!(ok);
    assert_eq!(telemetry.active_cores, 256);
    assert!(telemetry.total_cycles > 0);
    assert!(telemetry.success);

    unsafe { cron_vm_destroy(vm); }
}

#[test]
fn test_ffi_zero_copy_4d_tensor() {
    // Create [1, 4, 8, 8] float tensor
    let tensor = cron_tensor4d_create(1, 4, 8, 8, 0); // 0 = f32
    assert!(!tensor.is_null());

    unsafe {
        let data_ptr = cron_tensor4d_data_ptr(tensor) as *mut f32;
        assert!(!data_ptr.is_null());

        // Zero-copy direct write
        *data_ptr.offset(0) = 42.5;
        *data_ptr.offset(1) = 99.0;

        // Zero-copy direct read
        assert_eq!(*data_ptr.offset(0), 42.5);
        assert_eq!(*data_ptr.offset(1), 99.0);

        let mut shape = [0i32; 4];
        cron_tensor4d_shape(tensor, shape.as_mut_ptr());
        assert_eq!(shape, [1, 4, 8, 8]);

        cron_tensor4d_destroy(tensor);
    }
}

#[test]
fn test_ffi_photonic_mzi_accelerator() {
    let amps = [1.0, 2.0, 3.0, 4.0];
    let phases = [0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI, 0.5];
    let mut out_amps = [0.0; 4];
    let mut out_phases = [0.0; 4];

    unsafe {
        cron_photonic_mzi_gemm(
            amps.as_ptr(),
            phases.as_ptr(),
            4,
            out_amps.as_mut_ptr(),
            out_phases.as_mut_ptr(),
        );
    }

    assert_eq!(out_amps[0], 1.0); // cos(0) = 1.0 -> 1.0 * 1.0 = 1.0
    assert!(out_amps[1] < 1e-10); // cos(pi/2) = 0.0 -> 2.0 * 0.0 = 0.0
}

#[test]
fn test_ffi_jit_execution() {
    let source = CString::new(r#"
    .MODULE FfiJitTest
    _main:
        let a: i32 = 100
        let b: i32 = 42
        let res: i32 = a + b
        return res
    .END
    "#).unwrap();

    let mut err_ptr: *mut c_char = std::ptr::null_mut();
    let res = unsafe { cron_jit_execute(source.as_ptr(), &mut err_ptr) };
    assert!(err_ptr.is_null());
    assert_eq!(res, 142);
}

#[test]
fn test_ffi_channel_concurrency() {
    let ch = cron_channel_create_c(4);
    assert_ne!(ch, 0);

    // Empty non-blocking try_recv should return -1
    assert_eq!(cron_channel_try_recv_c(ch), -1);

    // Send values
    assert_eq!(cron_channel_send_c(ch, 12345), 0);
    assert_eq!(cron_channel_send_c(ch, 67890), 0);

    // Receive values
    assert_eq!(cron_channel_recv_c(ch), 12345);
    assert_eq!(cron_channel_try_recv_c(ch), 67890);

    // Channel empty again
    assert_eq!(cron_channel_try_recv_c(ch), -1);

    cron_channel_close_c(ch);
}

#[test]
fn test_ffi_torus_distance() {
    // Core 0 (0,0,0,0) to Core 42 (2,2,2,0) is 6 hops
    assert_eq!(cron_torus_distance_c(0, 42), 6);
    // Core 42 (2,2,2,0) to Core 127 (3,3,3,1) is 4 hops
    assert_eq!(cron_torus_distance_c(42, 127), 4);
    // Core 0 to Core 0 is 0 hops
    assert_eq!(cron_torus_distance_c(0, 0), 0);
}

#[test]
fn test_ffi_ternary_quantize_and_dot_product() {
    let weights = [1.5f32, -0.8, 0.0, 1.2, -1.0, 0.1, -0.05, 2.0];
    let mut packed = [0u8; 2];
    let mut scale = 0.0f32;

    let ok = unsafe {
        cron_ternary_quantize(
            weights.as_ptr(),
            weights.len(),
            packed.as_mut_ptr(),
            &mut scale,
        )
    };
    assert!(ok);
    assert!(scale > 0.0);

    let activations = [1.0f32, 2.0, 3.0, 1.0, 4.0, 0.5, 1.5, 2.0];
    let dot = unsafe {
        cron_ternary_dot_product(
            packed.as_ptr(),
            activations.as_ptr(),
            activations.len(),
            scale,
        )
    };
    assert!(dot.is_finite());
}

#[test]
fn test_ffi_cl_audit() {
    let cl_code = CString::new(r#"
.core [0, 0, 0, 0]:
@entry:
B0000: '==01#010> '==02#020> _MD05$001> _HL00#000!
"#).unwrap();

    let mut report_ptr: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { cron_cl_audit(cl_code.as_ptr(), &mut report_ptr) };
    assert!(ok);
    assert!(!report_ptr.is_null());

    let report_str = unsafe { CStr::from_ptr(report_ptr).to_str().unwrap() };
    assert!(report_str.contains("\"total_bundles\":1"));
    assert!(report_str.contains("\"total_slots\":4"));
    assert!(report_str.contains("\"hazards\":0"));

    unsafe { cron_string_free(report_ptr); }
}

#[test]
fn test_ffi_bpe_tokenizer() {
    let tok = cron_bpe_create_default();
    assert!(!tok.is_null());

    let text = CString::new("CRON BitNet 1.58b AI").unwrap();
    let mut out_ids = [0u32; 64];
    let mut out_len = 0usize;

    let ok = unsafe {
        cron_bpe_encode(
            tok,
            text.as_ptr(),
            out_ids.as_mut_ptr(),
            out_ids.len(),
            &mut out_len,
        )
    };
    assert!(ok);
    assert!(out_len > 0);

    let mut out_str: *mut c_char = std::ptr::null_mut();
    let decode_ok = unsafe {
        cron_bpe_decode(
            tok,
            out_ids.as_ptr(),
            out_len,
            &mut out_str,
        )
    };
    assert!(decode_ok);
    assert!(!out_str.is_null());

    let decoded = unsafe { CStr::from_ptr(out_str).to_str().unwrap() };
    assert_eq!(decoded, "CRON BitNet 1.58b AI");

    unsafe {
        cron_string_free(out_str);
        cron_bpe_free(tok);
    }
}

#[test]
fn test_ffi_multimodal_vision_and_audio() {
    // 1. Vision patch extraction via C-ABI
    let width = 64;
    let height = 64;
    let rgb = vec![128u8; width * height * 3];
    let mut vis_emb = vec![0.0f32; 16 * 32];
    let mut patch_count = 0usize;

    let vis_ok = unsafe {
        cron_vision_extract_patches(
            rgb.as_ptr(),
            width,
            height,
            3,
            16,
            32,
            vis_emb.as_mut_ptr(),
            vis_emb.len(),
            &mut patch_count,
        )
    };
    assert!(vis_ok);
    assert_eq!(patch_count, 16);

    // 2. Audio Mel-spectrogram via C-ABI
    let pcm = vec![0.5f32; 1600];
    let mut aud_emb = vec![0.0f32; 32 * 32];
    let mut frame_count = 0usize;

    let aud_ok = unsafe {
        cron_audio_mel_spectrogram(
            pcm.as_ptr(),
            pcm.len(),
            16000,
            80,
            32,
            aud_emb.as_mut_ptr(),
            aud_emb.len(),
            &mut frame_count,
        )
    };
    assert!(aud_ok);
    assert!(frame_count > 0);
}

#[test]
fn test_ffi_swarm_256_mesh() {
    let swarm = cron_swarm_create_256();
    assert!(!swarm.is_null());

    let task = CString::new("Synthesize ternary quantization matrix across 256 cores").unwrap();
    let mut report_ptr: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { cron_swarm_execute_task(swarm, task.as_ptr(), &mut report_ptr) };
    assert!(ok);
    assert!(!report_ptr.is_null());

    let report_str = unsafe { CStr::from_ptr(report_ptr).to_str().unwrap() };
    assert!(report_str.contains("\"consensus_achieved\":true"));
    assert!(report_str.contains("\"active_agents\":256"));
    assert!(report_str.contains("\"avg_hops\":"));

    unsafe {
        cron_string_free(report_ptr);
        cron_swarm_free(swarm);
    }
}

#[test]
fn test_ffi_cluster_swarm_4096() {
    let cluster = cron_cluster_swarm_create_4096();
    assert!(!cluster.is_null());

    let task = CString::new("Partition 4,096-core BitNet FlashAttention across 16 optical sockets").unwrap();
    let mut report_ptr: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { cron_cluster_swarm_execute_task(cluster, task.as_ptr(), &mut report_ptr) };
    assert!(ok);
    assert!(!report_ptr.is_null());

    let report_str = unsafe { CStr::from_ptr(report_ptr).to_str().unwrap() };
    assert!(report_str.contains("\"consensus_achieved\":true"));
    assert!(report_str.contains("\"total_chips\":16"));
    assert!(report_str.contains("\"total_cores\":4096"));
    assert!(report_str.contains("\"inter_chip_packets\":"));
    assert!(report_str.contains("\"optical_bandwidth_tbps\":"));

    unsafe {
        cron_string_free(report_ptr);
        cron_cluster_swarm_free(cluster);
    }
}

#[test]
fn test_ffi_swarm_synthesize_and_heal() {
    let prompt = CString::new("Synthesize FlashAttention-2 with optical MZI attention").unwrap();
    let mut code_ptr: *mut c_char = std::ptr::null_mut();
    let mut report_ptr: *mut c_char = std::ptr::null_mut();

    let ok = unsafe {
        cron_swarm_synthesize_and_heal(
            prompt.as_ptr(),
            3,
            true,
            true,
            &mut code_ptr,
            &mut report_ptr,
        )
    };
    assert!(ok);
    assert!(!code_ptr.is_null());
    assert!(!report_ptr.is_null());

    let code_str = unsafe { CStr::from_ptr(code_ptr).to_str().unwrap() };
    assert!(code_str.contains(".core [0, 0, 0, 0]:"));
    assert!(code_str.contains("flash-attn"));

    let report_str = unsafe { CStr::from_ptr(report_ptr).to_str().unwrap() };
    assert!(report_str.contains("\"consensus_achieved\": true"));
    assert!(report_str.contains("\"status\": \"synthesis_complete\""));

    unsafe {
        cron_string_free(code_ptr);
        cron_string_free(report_ptr);
    }
}

#[test]
fn test_ffi_swarm_tui_render_snapshot() {
    let mut snapshot_ptr: *mut c_char = std::ptr::null_mut();
    let ok = unsafe {
        cron_swarm_tui_render_snapshot(
            5,
            1, // TorusPlane
            false,
            &mut snapshot_ptr,
        )
    };
    assert!(ok);
    assert!(!snapshot_ptr.is_null());

    let snapshot_str = unsafe { CStr::from_ptr(snapshot_ptr).to_str().unwrap() };
    assert!(snapshot_str.contains("CRON 4D/6D SILICON SWARM"));
    assert!(snapshot_str.contains("4D TORUS"));
    assert!(snapshot_str.contains("Tick: 00005"));

    unsafe {
        cron_string_free(snapshot_ptr);
    }
}

#[test]
fn test_ffi_swarm_tui_telemetry_json() {
    let mut json_ptr: *mut c_char = std::ptr::null_mut();
    let ok = unsafe {
        cron_swarm_tui_telemetry_json(
            3,
            &mut json_ptr,
        )
    };
    assert!(ok);
    assert!(!json_ptr.is_null());

    let json_str = unsafe { CStr::from_ptr(json_ptr).to_str().unwrap() };
    assert!(json_str.contains("\"tick_count\": 3"));
    assert!(json_str.contains("\"simulated_ipc\":"));
    assert!(json_str.contains("\"chip_traffic_gbps\":"));

    unsafe {
        cron_string_free(json_ptr);
    }
}



