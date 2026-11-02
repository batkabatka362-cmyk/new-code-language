use cronc::cl_cuda_driver::*;

#[test]
fn test_cuda_driver_discovery_and_hardware_execution() {
    if !CudaDriver::is_available() {
        eprintln!("[SKIP] CUDA Driver (nvcuda.dll / libcuda.so) not present on this host system.");
        return;
    }

    let driver = CudaDriver::init().expect("Failed to initialize native CUDA driver");
    let version = driver.driver_version().expect("Failed to query driver version");
    let count = driver.device_count().expect("Failed to query device count");

    println!("Detected NVIDIA CUDA Driver Version: {}", version);
    println!("CUDA Device Count: {}", count);
    assert!(count > 0, "Expected at least 1 CUDA device");

    for i in 0..count {
        let name = driver.device_name(i).expect("Failed to get device name");
        let total_mem = driver.device_total_memory(i).expect("Failed to get total memory");
        println!("  GPU {}: {} (VRAM: {} MB)", i, name, total_mem / (1024 * 1024));
        assert!(!name.is_empty());
        assert!(total_mem > 0);
    }

    // Test real kernel execution on GPU 0
    let ptx_code = r#"
.version 7.5
.target sm_80
.address_size 64

.visible .entry vec_add_kernel (
    .param .u64 d_a,
    .param .u64 d_b,
    .param .u64 d_c,
    .param .u32 n
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<8>;
    .reg .f32 %f<3>;

    ld.param.u64 %rd1, [d_a];
    ld.param.u64 %rd2, [d_b];
    ld.param.u64 %rd3, [d_c];
    ld.param.u32 %r0, [n];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra DONE;

    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    add.s64 %rd6, %rd2, %rd4;
    add.s64 %rd7, %rd3, %rd4;

    ld.global.f32 %f1, [%rd5];
    ld.global.f32 %f2, [%rd6];
    add.f32 %f0, %f1, %f2;
    st.global.f32 [%rd7], %f0;

DONE:
    ret;
}
"#;

    let a = vec![1.5f32, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5];
    let b = vec![10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let expected = vec![11.5f32, 22.5, 33.5, 44.5, 55.5, 66.5, 77.5, 88.5];

    let grid = (1, 1, 1);
    let block = (8, 1, 1);

    let res = run_ptx_vector_kernel(ptx_code, "vec_add_kernel", grid, block, &[&a, &b], 8)
        .expect("Real GPU kernel execution failed");

    println!("GPU Kernel Output: {:?}", res);
    for i in 0..8 {
        assert!((res[i] - expected[i]).abs() < 1e-4, "Mismatch at {}: got {}, expected {}", i, res[i], expected[i]);
    }
    println!("Hardware NVIDIA CUDA Driver execution test passed flawlessly on actual GPU silicon!");
}

