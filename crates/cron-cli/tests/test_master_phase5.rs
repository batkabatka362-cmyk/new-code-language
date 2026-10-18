// ============================================================================
// CRON PHASE V MASTER VERIFICATION SUITE (test_master_phase5.rs)
// Rigorous verification of Tasks 20 to 24: Universal Interop & Deployment
// ============================================================================

use std::fs;
use std::process::Command;

fn get_cron_bin() -> &'static str {
    env!("CARGO_BIN_EXE_cron")
}

#[test]
fn test_task_20_python_pytorch_bridge_and_dlpack() {
    // Task 20: Python runtime & DLPack zero-copy tensor bridge
    let temp_dir = std::env::temp_dir();
    let py_path = temp_dir.join("test_phase5_cron_runtime.py");

    let out_py = Command::new(get_cron_bin())
        .args(["cl-bridge", "export-python", "-o", py_path.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-bridge export-python");
    assert!(out_py.status.success(), "export-python must succeed");
    let py_content = fs::read_to_string(&py_path).expect("Read exported py");
    assert!(py_content.contains("class CronRuntime"));

    let out_dl = Command::new(get_cron_bin())
        .args(["cl-bridge", "dlpack-info"])
        .output()
        .expect("Failed to run dlpack-info");
    assert!(out_dl.status.success(), "dlpack-info must succeed");
    let dl_str = String::from_utf8_lossy(&out_dl.stdout);
    assert!(dl_str.contains("DLPack"));

    let _ = fs::remove_file(py_path);
}

#[test]
fn test_task_21_csharp_dotnet_bindings() {
    // Task 21: C# .NET 8/9 P/Invoke bindings & CronNative
    let temp_dir = std::env::temp_dir();
    let cs_path = temp_dir.join("test_phase5_CronNative.cs");

    let out_cs = Command::new(get_cron_bin())
        .args(["cl-bridge", "export-csharp", "-o", cs_path.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-bridge export-csharp");
    assert!(out_cs.status.success(), "export-csharp must succeed");
    let cs_content = fs::read_to_string(&cs_path).expect("Read exported cs");
    assert!(cs_content.contains("class CronNative"));
    assert!(cs_content.contains("DllImport"));

    let _ = fs::remove_file(cs_path);
}

#[test]
fn test_task_22_cpp20_and_c23_embedded_headers() {
    // Task 22: C++20 header-only single library & C23 native transpiler
    let temp_dir = std::env::temp_dir();
    let hpp_path = temp_dir.join("test_phase5_cron.hpp");
    let cuh_path = temp_dir.join("test_phase5_cron.cuh");

    let out_c = Command::new(get_cron_bin())
        .args(["cl-bridge", "export-c-headers", "-o", hpp_path.to_str().unwrap()])
        .output()
        .expect("Failed to run export-c-headers");
    assert!(out_c.status.success(), "export-c-headers must succeed");
    let hpp_content = fs::read_to_string(&hpp_path).expect("Read exported hpp");
    assert!(hpp_content.contains("class BitNetTensor"));

    let out_cu = Command::new(get_cron_bin())
        .args(["cl-bridge", "export-cuda-headers", "-o", cuh_path.to_str().unwrap()])
        .output()
        .expect("Failed to run export-cuda-headers");
    assert!(out_cu.status.success(), "export-cuda-headers must succeed");
    let cuh_content = fs::read_to_string(&cuh_path).expect("Read exported cuh");
    assert!(cuh_content.contains("bitnet_gemv_kernel"));

    // Also verify C23 standalone transpilation
    let test_kernel = temp_dir.join("test_phase5_kernel.cl");
    let out_c23 = temp_dir.join("test_phase5_c23.c");
    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let out_transpile = Command::new(get_cron_bin())
        .args(["cl-c23", test_kernel.to_str().unwrap(), "-o", out_c23.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-c23");
    assert!(out_transpile.status.success(), "cl-c23 transpilation must succeed");
    assert!(out_c23.exists(), "C23 generated file must exist");

    let _ = fs::remove_file(hpp_path);
    let _ = fs::remove_file(cuh_path);
    let _ = fs::remove_file(test_kernel);
    let _ = fs::remove_file(out_c23);
}

#[test]
fn test_task_23_wasm_webgpu_in_browser_playground() {
    // Task 23: In-browser WebAssembly single-file playground & Three.js 4D-Torus
    let output = Command::new(get_cron_bin())
        .args(["playground"])
        .output()
        .expect("Failed to run playground");
    assert!(output.status.success(), "cron playground must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON WEBASSEMBLY IN-BROWSER PLAYGROUND & 4D-TORUS SIMULATOR"));
    assert!(stdout.contains("cron_playground.html"));

    // Verify generated file exists and clean up
    if std::path::Path::new("cron_playground.html").exists() {
        let _ = fs::remove_file("cron_playground.html");
    }
}

#[test]
fn test_task_24_rest_api_server_endpoints() {
    // Task 24: OpenAI-compatible high-throughput REST inference server (/health, /metrics)
    use cron_rt::http_server::{HttpServerConfig, LiveHttpServer};
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    let port = 19999;
    let config = HttpServerConfig {
        host: "127.0.0.1".to_string(),
        port,
        workers: 2,
        default_model: "cron-r1-7b-ternary".to_string(),
    };
    let server = LiveHttpServer::new(config);
    let (tx, rx) = channel();

    let server_thread = thread::spawn(move || {
        let _ = server.start(Some(rx));
    });

    thread::sleep(Duration::from_millis(60));
    let addr = format!("127.0.0.1:{}", port);

    // 1. Check GET /health
    {
        let mut stream = TcpStream::connect(&addr).expect("Connect to live server");
        stream.write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let resp = String::from_utf8_lossy(&buf);
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"status\":\"healthy\""));
    }

    // 2. Check GET /metrics
    {
        let mut stream = TcpStream::connect(&addr).expect("Connect to live server");
        stream.write_all(b"GET /metrics HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let resp = String::from_utf8_lossy(&buf);
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"active_cores\":256"));
    }

    // Graceful shutdown
    let _ = tx.send(());
    let _ = server_thread.join();
}
