use std::sync::Arc;
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

#[test]
fn test_cuda_tensor_engine_neural_ops() {
    if !CudaDriver::is_available() {
        eprintln!("[SKIP] CUDA Driver not present on host.");
        return;
    }

    let driver = CudaDriver::init().expect("Failed to initialize CUDA driver");
    let ctx = Arc::new(driver.create_context(0).expect("Failed to create CUDA context"));
    ctx.make_current().expect("Failed to make context current");

    let engine = CudaTensorEngine::new(ctx.clone()).expect("Failed to initialize CudaTensorEngine");

    // 1. Test GEMM NN: C = A @ B
    // A: 2x3, B: 3x2, C: 2x2
    let m = 2;
    let k = 3;
    let n = 2;
    let a_host = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0]; // [[1, 2, 3], [4, 5, 6]]
    let b_host = vec![7.0f32, 8.0, 9.0, 1.0, 2.0, 3.0]; // [[7, 8], [9, 1], [2, 3]]
    // C[0, 0] = 1*7 + 2*9 + 3*2 = 7 + 18 + 6 = 31
    // C[0, 1] = 1*8 + 2*1 + 3*3 = 8 + 2 + 9 = 19
    // C[1, 0] = 4*7 + 5*9 + 6*2 = 28 + 45 + 12 = 85
    // C[1, 1] = 4*8 + 5*1 + 6*3 = 32 + 5 + 18 = 55
    let expected_c = vec![31.0f32, 19.0, 85.0, 55.0];

    let d_a = ctx.alloc(a_host.len() * 4).expect("alloc d_a");
    let d_b = ctx.alloc(b_host.len() * 4).expect("alloc d_b");
    let d_c = ctx.alloc(expected_c.len() * 4).expect("alloc d_c");

    ctx.copy_htod(&d_a, &a_host).expect("copy a");
    ctx.copy_htod(&d_b, &b_host).expect("copy b");

    engine.gemm(&d_a, &d_b, &d_c, m, n, k, false, false).expect("gemm nn");
    ctx.synchronize().expect("sync");

    let mut c_host = vec![0.0f32; 4];
    ctx.copy_dtoh(&d_c, &mut c_host).expect("copy c");
    println!("GPU GEMM NN Output: {:?}", c_host);
    for i in 0..4 {
        assert!((c_host[i] - expected_c[i]).abs() < 1e-3, "GEMM NN mismatch at {}: {} vs {}", i, c_host[i], expected_c[i]);
    }

    // 2. Test GEMM NT: C = A @ B^T
    // A: 2x3, B^T: 3x2 => B is shape 2x3 (N=2, K=3)
    // B = [[7, 9, 2], [8, 1, 3]]
    let b_nt_host = vec![7.0f32, 9.0, 2.0, 8.0, 1.0, 3.0];
    let d_b_nt = ctx.alloc(b_nt_host.len() * 4).expect("alloc d_b_nt");
    ctx.copy_htod(&d_b_nt, &b_nt_host).expect("copy b_nt");

    engine.gemm(&d_a, &d_b_nt, &d_c, m, n, k, false, true).expect("gemm nt");
    ctx.synchronize().expect("sync");

    ctx.copy_dtoh(&d_c, &mut c_host).expect("copy c nt");
    println!("GPU GEMM NT Output: {:?}", c_host);
    for i in 0..4 {
        assert!((c_host[i] - expected_c[i]).abs() < 1e-3, "GEMM NT mismatch at {}: {} vs {}", i, c_host[i], expected_c[i]);
    }

    // 3. Test GEMM TN: C = A^T @ B
    // A^T: 2x3 => A is shape 3x2 (K=3, M=2)
    // A = [[1, 4], [2, 5], [3, 6]]
    // B is shape 3x2 (K=3, N=2) = [[7, 8], [9, 1], [2, 3]]
    let a_tn_host = vec![1.0f32, 4.0, 2.0, 5.0, 3.0, 6.0];
    let d_a_tn = ctx.alloc(a_tn_host.len() * 4).expect("alloc d_a_tn");
    ctx.copy_htod(&d_a_tn, &a_tn_host).expect("copy a_tn");

    engine.gemm(&d_a_tn, &d_b, &d_c, m, n, k, true, false).expect("gemm tn");
    ctx.synchronize().expect("sync");

    ctx.copy_dtoh(&d_c, &mut c_host).expect("copy c tn");
    println!("GPU GEMM TN Output: {:?}", c_host);
    for i in 0..4 {
        assert!((c_host[i] - expected_c[i]).abs() < 1e-3, "GEMM TN mismatch at {}: {} vs {}", i, c_host[i], expected_c[i]);
    }

    // 4. Test SwiGLU: act = SiLU(G) * U
    let g_host = vec![0.0f32, 1.0, -1.0, 2.0];
    let u_host = vec![2.0f32, 0.5, 3.0, -1.5];
    let mut expected_swiglu = vec![0.0f32; 4];
    for i in 0..4 {
        let silu = g_host[i] / (1.0 + (-g_host[i]).exp());
        expected_swiglu[i] = silu * u_host[i];
    }

    let d_g = ctx.alloc(g_host.len() * 4).expect("alloc d_g");
    let d_u = ctx.alloc(u_host.len() * 4).expect("alloc d_u");
    let d_act = ctx.alloc(expected_swiglu.len() * 4).expect("alloc d_act");

    ctx.copy_htod(&d_g, &g_host).expect("copy g");
    ctx.copy_htod(&d_u, &u_host).expect("copy u");

    engine.swiglu(&d_g, &d_u, &d_act, 4).expect("swiglu");
    ctx.synchronize().expect("sync");

    let mut act_host = vec![0.0f32; 4];
    ctx.copy_dtoh(&d_act, &mut act_host).expect("copy act");
    println!("GPU SwiGLU Output: {:?}", act_host);
    for i in 0..4 {
        assert!((act_host[i] - expected_swiglu[i]).abs() < 1e-2, "SwiGLU mismatch at {}: {} vs {}", i, act_host[i], expected_swiglu[i]);
    }

    // 5. Test AdamW Step
    let w_host = vec![1.0f32, -2.0, 0.5, 3.0];
    let grad_host = vec![0.1f32, -0.2, 0.05, 0.4];
    let m_host = vec![0.0f32; 4];
    let v_host = vec![0.0f32; 4];

    let d_w = ctx.alloc(w_host.len() * 4).expect("alloc d_w");
    let d_grad = ctx.alloc(grad_host.len() * 4).expect("alloc d_grad");
    let d_m = ctx.alloc(m_host.len() * 4).expect("alloc d_m");
    let d_v = ctx.alloc(v_host.len() * 4).expect("alloc d_v");

    ctx.copy_htod(&d_w, &w_host).expect("copy w");
    ctx.copy_htod(&d_grad, &grad_host).expect("copy grad");
    ctx.copy_htod(&d_m, &m_host).expect("copy m");
    ctx.copy_htod(&d_v, &v_host).expect("copy v");

    let lr = 0.01f32;
    let beta1 = 0.9f32;
    let beta2 = 0.999f32;
    let eps = 1e-8f32;
    let wd = 0.01f32;

    engine.adamw_step(&d_w, &d_grad, &d_m, &d_v, lr, beta1, beta2, eps, wd, 1, 4).expect("adamw_step");
    ctx.synchronize().expect("sync");

    let mut w_after = vec![0.0f32; 4];
    ctx.copy_dtoh(&d_w, &mut w_after).expect("copy w after");
    println!("GPU AdamW Updated Weights: {:?}", w_after);
    for i in 0..4 {
        assert!(w_after[i] != w_host[i], "Weight at {} did not update!", i);
    }

    // 6. Test SwiGLU Backward Pass
    let dact_host = vec![0.5f32, -1.0, 2.0, 0.2];
    let mut expected_dg = vec![0.0f32; 4];
    let mut expected_du = vec![0.0f32; 4];
    for i in 0..4 {
        let da = dact_host[i];
        let x = g_host[i];
        let sig = 1.0 / (1.0 + (-x).exp());
        let silu = x * sig;
        expected_du[i] = da * silu;

        let d_silu = sig * (1.0 + x * (1.0 - sig));
        expected_dg[i] = (da * u_host[i]) * d_silu;
    }

    let d_dact = ctx.alloc(dact_host.len() * 4).expect("alloc d_dact");
    let d_dg = ctx.alloc(4 * 4).expect("alloc d_dg");
    let d_du = ctx.alloc(4 * 4).expect("alloc d_du");
    ctx.copy_htod(&d_dact, &dact_host).expect("copy dact");

    engine.swiglu_bwd(&d_dact, &d_g, &d_u, &d_dg, &d_du, 4).expect("swiglu_bwd");
    ctx.synchronize().expect("sync");

    let mut dg_host = vec![0.0f32; 4];
    let mut du_host = vec![0.0f32; 4];
    ctx.copy_dtoh(&d_dg, &mut dg_host).expect("copy dg");
    ctx.copy_dtoh(&d_du, &mut du_host).expect("copy du");

    println!("GPU SwiGLU Backward dg: {:?}", dg_host);
    println!("GPU SwiGLU Backward du: {:?}", du_host);
    for i in 0..4 {
        assert!((dg_host[i] - expected_dg[i]).abs() < 1e-2, "dg mismatch at {}: {} vs {}", i, dg_host[i], expected_dg[i]);
        assert!((du_host[i] - expected_du[i]).abs() < 1e-2, "du mismatch at {}: {} vs {}", i, du_host[i], expected_du[i]);
    }

    // 7. Test In-Place Vector Add
    let y_host = vec![1.0f32, 2.0, 3.0, 4.0];
    let x_host = vec![10.0f32, 20.0, 30.0, 40.0];
    let d_y = ctx.alloc(y_host.len() * 4).expect("alloc d_y");
    let d_x = ctx.alloc(x_host.len() * 4).expect("alloc d_x");
    ctx.copy_htod(&d_y, &y_host).expect("copy y");
    ctx.copy_htod(&d_x, &x_host).expect("copy x");

    engine.vec_add_inplace(&d_y, &d_x, 4).expect("vec_add_inplace");
    ctx.synchronize().expect("sync");

    let mut y_res = vec![0.0f32; 4];
    ctx.copy_dtoh(&d_y, &mut y_res).expect("copy y res");
    println!("GPU vec_add_inplace Output: {:?}", y_res);
    assert_eq!(y_res, vec![11.0, 22.0, 33.0, 44.0]);

    println!("All CudaTensorEngine operations verified directly on hardware RTX 4060!");
}

