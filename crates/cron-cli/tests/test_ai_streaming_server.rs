// ============================================================================
// CRON AI Streaming Server & Benchmark Integration Test
// Tests /v1/chat/completions SSE streaming and bench_server load testing.
// ============================================================================

use cron_cli::bench_server::run_server_benchmark;
use cron_rt::http_server::{HttpServerConfig, LiveHttpServer};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[test]
fn test_live_ai_streaming_server_and_benchmark() {
    let port = 18892;
    let (shutdown_tx, shutdown_rx) = channel();

    let config = HttpServerConfig {
        host: "127.0.0.1".to_string(),
        port,
        workers: 2,
    };
    let server = LiveHttpServer::new(config);
    let is_running = server.is_running.clone();

    // Spawn server in background
    let server_thread = thread::spawn(move || {
        let _ = server.start(Some(shutdown_rx));
    });

    // Wait for server to bind and start running
    for _ in 0..100 {
        if is_running.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    thread::sleep(Duration::from_millis(50));

    // 1. Verify /v1/chat/completions SSE Streaming
    {
        let mut stream = None;
        for _ in 0..50 {
            if let Ok(s) = TcpStream::connect(("127.0.0.1", port)) {
                stream = Some(s);
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        let mut stream = stream.expect("Failed to connect to CRON AI streaming server");

        let body = r#"{"model":"cron-bitnet-1.58b","messages":[{"role":"user","content":"Hello CRON"}],"stream":true}"#;
        let req = format!(
            "POST /v1/chat/completions HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            port,
            body.len(),
            body
        );

        stream.write_all(req.as_bytes()).unwrap();

        let mut response = String::new();
        match stream.read_to_string(&mut response) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionAborted || e.kind() == std::io::ErrorKind::ConnectionReset => {}
            Err(e) => panic!("Failed to read stream: {}", e),
        }

        assert!(
            response.starts_with("HTTP/1.1 200 OK"),
            "Expected 200 OK for /v1/chat/completions"
        );
        assert!(
            response.contains("Content-Type: text/event-stream"),
            "Expected text/event-stream header"
        );
        assert!(
            response.contains("data: {\"choices\":[{\"delta\":{\"content\":"),
            "Expected delta tokens in SSE stream"
        );
        assert!(
            response.contains("data: [DONE]"),
            "Expected [DONE] terminator in SSE stream"
        );
        println!("AI SSE Streaming verified successfully:\n{}", response);
    }

    // 2. Run Industrial Server Benchmark Suite
    {
        let url = format!("http://127.0.0.1:{}/health", port);
        let result = run_server_benchmark(&url, 50, 5)
            .expect("Benchmark suite failed to run against live server");

        assert_eq!(result.total_requests, 50);
        assert!(result.successful_requests >= 45, "Expected at least 90% success rate on loopback, got {}/50", result.successful_requests);
        assert!(result.qps > 0.0);
        println!(
            "Server Benchmark verified successfully: {} reqs at {:.1} QPS, P50: {} µs",
            result.successful_requests, result.qps, result.p50_latency_us
        );
    }

    // Shut down server
    let _ = shutdown_tx.send(());
    let _ = server_thread.join();
}
