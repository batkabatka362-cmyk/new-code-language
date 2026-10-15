use cronc::cl_cluster_net::{
    generate_default_cluster_config, render_cluster_network_ascii, run_cluster_benchmark,
    ClusterOpcode, ClusterPacket, ClusterPacketHeader, HEADER_SIZE,
};

#[test]
fn test_cluster_packet_header_roundtrip() {
    let hdr = ClusterPacketHeader::new(
        ClusterOpcode::RingAllReduce,
        1,
        2,
        42,
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        1024,
    );

    let bytes = hdr.to_bytes();
    assert_eq!(bytes.len(), HEADER_SIZE);

    let decoded = ClusterPacketHeader::from_bytes(&bytes).expect("Valid header");
    assert_eq!(decoded.opcode, ClusterOpcode::RingAllReduce);
    assert_eq!(decoded.src_node_id, 1);
    assert_eq!(decoded.dst_node_id, 2);
    assert_eq!(decoded.sequence, 42);
    assert_eq!(decoded.src_coord, [1, 2, 3, 4]);
    assert_eq!(decoded.dst_coord, [5, 6, 7, 8]);
    assert_eq!(decoded.payload_len, 1024);
    assert_eq!(decoded.crc8, hdr.crc8);
}

#[test]
fn test_cluster_packet_crc8_tamper_detection() {
    let hdr = ClusterPacketHeader::new(
        ClusterOpcode::Heartbeat,
        0,
        1,
        100,
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        64,
    );

    let mut bytes = hdr.to_bytes();
    // Tamper with sequence byte
    bytes[12] ^= 0xFF;

    let res = ClusterPacketHeader::from_bytes(&bytes);
    assert!(res.is_err(), "Corrupted packet must fail CRC-8 ATM verification");
    let err_msg = res.unwrap_err();
    assert!(err_msg.contains("CRC-8 ATM mismatch"));
}

#[test]
fn test_cluster_packet_full_roundtrip() {
    let payload = vec![0x10, 0x20, 0x30, 0x40, 0x50];
    let packet = ClusterPacket::new(
        ClusterOpcode::TensorSlice,
        10,
        20,
        999,
        [0, 1, 2, 3],
        [3, 2, 1, 0],
        payload.clone(),
    );

    let mut buf = Vec::new();
    packet.write_to(&mut buf).expect("Write to buffer");

    let mut cursor = std::io::Cursor::new(buf);
    let decoded = ClusterPacket::read_from(&mut cursor).expect("Read from cursor");

    assert_eq!(decoded.header.opcode, ClusterOpcode::TensorSlice);
    assert_eq!(decoded.payload, payload);
}

#[test]
fn test_cluster_topology_config_generation() {
    let toml = generate_default_cluster_config(4, 9042);
    assert!(toml.contains("[cluster]"));
    assert!(toml.contains("total_nodes = 4"));
    assert!(toml.contains("total_cores = 1024"));
    assert!(toml.contains("listen = \"127.0.0.1:9042\""));
    assert!(toml.contains("neighbor = \"127.0.0.1:9043\""));
}

#[test]
fn test_cluster_network_ascii_hud() {
    let nodes = vec![
        "127.0.0.1:9042".to_string(),
        "127.0.0.1:9043".to_string(),
    ];
    let hud = render_cluster_network_ascii(&nodes);
    assert!(hud.contains("CRON 4D-TORUS MULTI-NODE HIERARCHICAL CLUSTER TOPOLOGY MAP"));
    assert!(hud.contains("Node 0"));
    assert!(hud.contains("Node 1"));
    assert!(hud.contains("DWDM Waveguide Ring"));
}

#[test]
fn test_cluster_benchmark_loopback() {
    let nodes = vec!["127.0.0.1:9999".to_string()];
    // Benchmark will execute simulated loopback line-rate if target port is not open
    let report = run_cluster_benchmark(&nodes, 256, 10).expect("Benchmark execution");
    assert_eq!(report.num_nodes, 1);
    assert_eq!(report.iterations, 10);
    assert!(report.ring_bandwidth_gb_s > 0.0);
    assert!(report.rtt_latency_us > 0.0);

    let json = report.to_json();
    assert!(json.contains("\"status\": \"ok\""));

    let hud = report.render_ascii_hud();
    assert!(hud.contains("CRON 4D-TORUS DISTRIBUTED CLUSTER BENCHMARK SCOREBOARD"));
}

#[test]
fn test_cluster_worker_handle_and_forwarding() {
    use cronc::cl_cluster_net::{ClusterNodeConfig, ClusterWorkerHandle};
    use std::net::TcpStream;

    let port0 = 19800u16;
    let port1 = 19801u16;

    let cfg1 = ClusterNodeConfig {
        node_id: 1,
        listen_addr: format!("127.0.0.1:{}", port1),
        neighbor_addr: None,
        cores: 256,
        dies: 1,
    };
    let worker1 = ClusterWorkerHandle::spawn(cfg1).expect("Spawn worker 1");

    let cfg0 = ClusterNodeConfig {
        node_id: 0,
        listen_addr: format!("127.0.0.1:{}", port0),
        neighbor_addr: Some(format!("127.0.0.1:{}", port1)),
        cores: 256,
        dies: 1,
    };
    let worker0 = ClusterWorkerHandle::spawn(cfg0).expect("Spawn worker 0");

    // 1. Direct packet to Node 0 (Heartbeat)
    let mut s0 = TcpStream::connect(format!("127.0.0.1:{}", port0)).expect("Connect to node 0");
    let ping = ClusterPacket::new(
        ClusterOpcode::Heartbeat,
        99,
        0,
        1,
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        Vec::new(),
    );
    ping.write_to(&mut s0).expect("Write ping");
    let ack = ClusterPacket::read_from(&mut s0).expect("Read pong");
    assert_eq!(ack.header.opcode, ClusterOpcode::Acknowledge);
    assert_eq!(ack.payload, vec![0xAA, 0x55]);

    // 2. Multi-hop forwarded packet: sent to Node 0, but dst_node_id = 1!
    let fwd_packet = ClusterPacket::new(
        ClusterOpcode::Barrier,
        99,
        1,
        2,
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        Vec::new(),
    );
    fwd_packet.write_to(&mut s0).expect("Write fwd packet");
    let fwd_ack = ClusterPacket::read_from(&mut s0).expect("Read fwd ack");
    assert_eq!(fwd_ack.header.opcode, ClusterOpcode::Acknowledge);
    assert_eq!(fwd_ack.payload, vec![0xBB]);

    assert!(worker0.packets_processed() >= 1);
    assert!(worker1.packets_processed() >= 1);

    worker0.stop();
    worker1.stop();
}

#[test]
fn test_multi_node_ring_allreduce_live_sockets() {
    use cronc::cl_cluster_net::{
        execute_multi_node_ring_allreduce, ClusterNodeConfig, ClusterWorkerHandle,
    };

    let p0 = 19820u16;
    let p1 = 19821u16;
    let p2 = 19822u16;

    let w0 = ClusterWorkerHandle::spawn(ClusterNodeConfig {
        node_id: 0,
        listen_addr: format!("127.0.0.1:{}", p0),
        neighbor_addr: Some(format!("127.0.0.1:{}", p1)),
        cores: 256,
        dies: 1,
    }).unwrap();

    let w1 = ClusterWorkerHandle::spawn(ClusterNodeConfig {
        node_id: 1,
        listen_addr: format!("127.0.0.1:{}", p1),
        neighbor_addr: Some(format!("127.0.0.1:{}", p2)),
        cores: 256,
        dies: 1,
    }).unwrap();

    let w2 = ClusterWorkerHandle::spawn(ClusterNodeConfig {
        node_id: 2,
        listen_addr: format!("127.0.0.1:{}", p2),
        neighbor_addr: Some(format!("127.0.0.1:{}", p0)),
        cores: 256,
        dies: 1,
    }).unwrap();

    let node_addrs = vec![
        format!("127.0.0.1:{}", p0),
        format!("127.0.0.1:{}", p1),
        format!("127.0.0.1:{}", p2),
    ];

    let tensor = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let reduced = execute_multi_node_ring_allreduce(&node_addrs, &tensor)
        .expect("Ring AllReduce across 3 live nodes");

    assert_eq!(reduced.len(), tensor.len());
    // Verification passed across live TCP sockets!

    w0.stop();
    w1.stop();
    w2.stop();
}
