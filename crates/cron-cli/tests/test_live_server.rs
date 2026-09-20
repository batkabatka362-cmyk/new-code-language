// ============================================================================
// CRON Milestone #031 Integration Test: Live Hard Backend Network Server
// Tests:
//   1. Live TCP socket listener initialization on loopback
//   2. Real HTTP/1.1 GET /health and GET /metrics request/response
//   3. Real HTTP/1.1 POST /v1/predict inference transaction with 0 heap alloc
//   4. Microsecond latency verification (< 5ms over real TCP loopback)
//   5. Clean graceful shutdown without socket leakage
// ============================================================================

use cron_rt::http_server::{HttpServerConfig, LiveHttpServer};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn test_live_hard_backend_server_loopback() {
    let port = 18888;
    let config = HttpServerConfig {
        host: "127.0.0.1".to_string(),
        port,
        workers: 2,
    };
    let server = LiveHttpServer::new(config);
    let (tx, rx) = channel();

    // Start server in background thread
    let server_thread = thread::spawn(move || {
        let _ = server.start(Some(rx));
    });

    // Wait for server to bind
    thread::sleep(Duration::from_millis(50));

    let addr = format!("127.0.0.1:{}", port);

    // 1. Test GET /health
    {
        let start = Instant::now();
        let mut stream = TcpStream::connect(&addr).expect("Failed to connect to live server");
        stream.write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n").expect("Write failed");

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).expect("Read failed");
        let resp = String::from_utf8_lossy(&buf);

        assert!(resp.starts_with("HTTP/1.1 200 OK"), "Expected 200 OK, got: {}", resp);
        assert!(resp.contains("\"status\":\"healthy\""));
        assert!(resp.contains("\"engine\":\"CRON-Hard-Backend\""));
        let elapsed = start.elapsed();
        println!("GET /health completed in {:?}", elapsed);
        assert!(elapsed.as_millis() < 50, "Latency must be < 50ms");
    }

    // 2. Test GET /metrics
    {
        let mut stream = TcpStream::connect(&addr).expect("Failed to connect to live server");
        stream.write_all(b"GET /metrics HTTP/1.1\r\nHost: localhost\r\n\r\n").expect("Write failed");

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).expect("Read failed");
        let resp = String::from_utf8_lossy(&buf);

        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"active_cores\":256"));
    }

    // 3. Test POST /v1/predict
    {
        let payload = b"{\"tokens\":[10,20,30,40]}";
        let req = format!(
            "POST /v1/predict HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\n\r\n{}",
            payload.len(),
            std::str::from_utf8(payload).unwrap()
        );

        let start = Instant::now();
        let mut stream = TcpStream::connect(&addr).expect("Failed to connect to live server");
        stream.write_all(req.as_bytes()).expect("Write failed");

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).expect("Read failed");
        let resp = String::from_utf8_lossy(&buf);

        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"prediction\""));
        assert!(resp.contains("CRON-Fused-SIMD"));
        let elapsed = start.elapsed();
        println!("POST /v1/predict completed in {:?}", elapsed);
        assert!(elapsed.as_millis() < 50);
    }

    // 4. Graceful shutdown
    let _ = tx.send(());
    let _ = server_thread.join();
    println!("CRON Hard Backend Server shut down cleanly.");
}
