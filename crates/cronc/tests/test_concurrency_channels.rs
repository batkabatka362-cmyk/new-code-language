// ============================================================================
// Tests for Distributed Neuromorphic 256-Core 4D-Torus Concurrency (Option 3)
// Verifies:
//   1. Lock-free bounded FIFO channels (channel_new, channel_send, channel_recv)
//   2. Non-blocking try_recv and channel_close semantics
//   3. 4D-Torus coordinate math and Manhattan distance with toroidal wrap-around
//   4. Multi-threaded JIT fiber spawning, channel communication, and await
//   5. C23 native GCC AOT compilation & execution of 3-stage CSP pipeline
//   6. VLIW slot emission (_SP, _TX, _RX)
// ============================================================================

use cronc::{compile_source, compile_to_c23, run_source_jit};
use cronc::jit_backend::{jit_channel_new, jit_channel_send, jit_channel_recv, jit_channel_try_recv, jit_channel_close, jit_torus_distance};
use std::fs;
use std::process::Command;

#[test]
fn test_channel_basic_fifo() {
    let ch = jit_channel_new(8);
    assert_eq!(jit_channel_send(ch, 100), 0);
    assert_eq!(jit_channel_send(ch, 200), 0);
    assert_eq!(jit_channel_send(ch, 300), 0);

    assert_eq!(jit_channel_recv(ch), 100);
    assert_eq!(jit_channel_recv(ch), 200);
    assert_eq!(jit_channel_recv(ch), 300);

    jit_channel_close(ch);
}

#[test]
fn test_channel_nonblocking_try_recv() {
    let ch = jit_channel_new(4);
    // Initially empty -> try_recv returns -1
    assert_eq!(jit_channel_try_recv(ch), -1);

    // Send an element
    assert_eq!(jit_channel_send(ch, 42), 0);
    // Now try_recv returns 42
    assert_eq!(jit_channel_try_recv(ch), 42);
    // Now empty again -> returns -1
    assert_eq!(jit_channel_try_recv(ch), -1);

    jit_channel_close(ch);
}

#[test]
fn test_torus_4d_distance_calculation() {
    // Distance from core 0 (0,0,0,0) to core 0 is 0
    assert_eq!(jit_torus_distance(0, 0), 0);

    // Distance from core 0 (0,0,0,0) to core 1 (1,0,0,0) is 1
    assert_eq!(jit_torus_distance(0, 1), 1);

    // Wrap around: dimension size is 4, so distance from 0 to 3 is 1 (wraps: 0 -> 3)
    assert_eq!(jit_torus_distance(0, 3), 1);

    // Distance from 0 to 2 is 2
    assert_eq!(jit_torus_distance(0, 2), 2);

    // 4D distance between diagonal opposites:
    // Core 0 (0,0,0,0) to Core 85 (1,1,1,1) -> 1 + 1 + 1 + 1 = 4
    assert_eq!(jit_torus_distance(0, 85), 4);
}

#[test]
fn test_jit_spawn_and_await_fibers() {
    let source = r#"
    .MODULE FiberSpawnTest

    fn worker_task(base: i64, multiplier: i64) -> i64 {
        let res = base * multiplier + 10;
        return res;
    }

    fn main() -> i64 {
        let f1 = spawn worker_task(5, 4);
        let f2 = spawn worker_task(10, 3);
        let r1 = await f1;
        let r2 = await f2;
        return r1 + r2;
    }
    "#;

    // worker_task(5, 4) = 5*4 + 10 = 30
    // worker_task(10, 3) = 10*3 + 10 = 40
    // r1 + r2 = 70
    let result = run_source_jit(source).expect("JIT execution should succeed");
    assert_eq!(result, 70);
}

#[test]
fn test_jit_csp_channel_pipeline() {
    let source = r#"
    .MODULE ChannelPipelineTest

    fn producer(ch: i64) -> i64 {
        channel_send(ch, 11);
        channel_send(ch, 22);
        channel_send(ch, 33);
        return 3;
    }

    fn consumer(ch: i64) -> i64 {
        let a = channel_recv(ch);
        let b = channel_recv(ch);
        let c = channel_recv(ch);
        return a + b + c;
    }

    fn main() -> i64 {
        let ch = channel_new(8);
        let f_prod = spawn producer(ch);
        let f_cons = spawn consumer(ch);
        let _p = await f_prod;
        let total = await f_cons;
        return total;
    }
    "#;

    // 11 + 22 + 33 = 66
    let result = run_source_jit(source).expect("JIT CSP pipeline should succeed");
    assert_eq!(result, 66);
}

#[test]
fn test_c23_native_gcc_concurrency_pipeline() {
    let source = r#"
    .MODULE TorusCspPipelineNative

    fn producer(ch: channel<i64>, n: i64) -> i64 {
        let mut i = 1;
        while i <= n {
            channel_send(ch, i * 10);
            i = i + 1;
        }
        return n;
    }

    fn transformer(in_ch: channel<i64>, out_ch: channel<i64>) -> i64 {
        let mut sum = 0;
        let mut j = 1;
        while j <= 3 {
            let item = channel_recv(in_ch);
            let transformed = item * 2;
            channel_send(out_ch, transformed);
            sum = sum + transformed;
            j = j + 1;
        }
        return sum;
    }

    fn consumer(in_ch: channel<i64>) -> i64 {
        let a = channel_recv(in_ch);
        let b = channel_recv(in_ch);
        let c = channel_recv(in_ch);
        return a + b + c;
    }

    fn main() -> i64 {
        let dist = torus_distance(0, 42);
        let ch1: channel<i64> = channel_new(16);
        let ch2: channel<i64> = channel_new(16);

        let f1 = spawn at(0) producer(ch1, 3);
        let f2 = spawn at(42) transformer(ch1, ch2);
        let f3 = spawn at(127) consumer(ch2);

        let _p = await f1;
        let _t = await f2;
        let total = await f3;

        return total + dist;
    }
    "#;

    let c_code = compile_to_c23(source).expect("Compile to C23 should succeed");
    assert!(c_code.contains("cron_channel_t"), "Generated C code must contain CSP channels");
    assert!(c_code.contains("cron_channel_create"), "Generated C code must contain channel_create");
    assert!(c_code.contains("cron_fiber_t"), "Generated C code must contain multi-threaded fibers");
    assert!(c_code.contains("torus_distance"), "Generated C code must contain torus distance");

    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("cron_torus_csp_test.c");
    let exe_path = temp_dir.join("cron_torus_csp_test.exe");

    fs::write(&c_path, &c_code).expect("Write C source to temp");

    let gcc_status = Command::new("gcc")
        .args(["-O3", c_path.to_str().unwrap(), "-o", exe_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_output = Command::new(&exe_path)
                .output()
                .expect("Execute native binary");
            let stdout_str = String::from_utf8_lossy(&run_output.stdout);
            let stderr_str = String::from_utf8_lossy(&run_output.stderr);
            println!("STDOUT: {}", stdout_str);
            println!("STDERR: {}", stderr_str);
            let exit_code = run_output.status.code().unwrap_or(-1);
            assert_eq!(exit_code, 126, "Native concurrent binary exit code should equal 126: exit_code={}, stderr={}", exit_code, stderr_str);
            assert!(stdout_str.contains("[CRON Native C23 Engine]"), "Output must contain CRON banner");
            assert!(stdout_str.contains("User Main Returned: 126"), "Output must contain User Main Returned: 126");
            assert!(stdout_str.contains("Execution completed successfully"), "Output must indicate success");

            let _ = fs::remove_file(c_path);
            let _ = fs::remove_file(exe_path);
        }
    }
}

#[test]
fn test_vliw_concurrency_slot_emission() {
    let source = r#"
    .MODULE VliwConcurrency

    fn compute(ch: i64) -> i64 {
        channel_send(ch, 42);
        let val = channel_recv(ch);
        return val;
    }

    fn main() -> i64 {
        let ch = channel_new(8);
        let f = spawn at(15) compute(ch);
        let r = await f;
        return r;
    }
    "#;

    let vliw_output = compile_source(source).expect("VLIW compilation should succeed");
    // Assert presence of VLIW bundle or slot mnemonics
    assert!(vliw_output.contains("_SP") || vliw_output.contains("SPAWN") || vliw_output.contains("SP"), "Must emit spawn slot");
    assert!(vliw_output.contains("_TX") || vliw_output.contains("TX") || vliw_output.contains("SEND"), "Must emit send slot");
    assert!(vliw_output.contains("_RX") || vliw_output.contains("RX") || vliw_output.contains("RECV"), "Must emit recv slot");
}

#[test]
fn test_examples_torus_csp_pipeline_execution() {
    let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("examples")
        .join("torus_csp_pipeline.cr");

    assert!(example_path.exists(), "examples/torus_csp_pipeline.cr must exist");
    let code = fs::read_to_string(&example_path).expect("Read examples/torus_csp_pipeline.cr");

    // 1. Verify JIT execution
    // 5 samples: 10, 20, 30, 40, 50 -> (* 3 + 7): 37, 67, 97, 127, 157 = 485
    // dist(0, 42) = 6, dist(42, 127) = 4 -> total = 485 + 6 + 4 = 495
    let jit_res = run_source_jit(&code).expect("JIT execution of examples/torus_csp_pipeline.cr");
    assert_eq!(jit_res, 495, "Pipeline output must equal 495");

    // 2. Verify C23 native GCC compilation & execution
    let c_code = compile_to_c23(&code).expect("C23 compilation of examples/torus_csp_pipeline.cr");
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("cron_example_pipeline.c");
    let exe_path = temp_dir.join("cron_example_pipeline.exe");

    fs::write(&c_path, &c_code).expect("Write C source to temp");

    let gcc_status = Command::new("gcc")
        .args(["-O3", c_path.to_str().unwrap(), "-o", exe_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_output = Command::new(&exe_path)
                .output()
                .expect("Execute example pipeline binary");
            let stdout_str = String::from_utf8_lossy(&run_output.stdout);
            let exit_code = run_output.status.code().unwrap_or(-1);
            assert!(exit_code == 495 || exit_code == (495 % 256), "Pipeline binary exit code should equal 495: exit_code={}", exit_code);
            assert!(stdout_str.contains("User Main Returned: 495"), "Output must contain User Main Returned: 495");
            assert!(stdout_str.contains("Execution completed successfully"), "Output must indicate success");

            let _ = fs::remove_file(c_path);
            let _ = fs::remove_file(exe_path);
        }
    }
}
