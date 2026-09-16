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

