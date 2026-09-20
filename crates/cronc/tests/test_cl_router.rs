// ============================================================================
// Unit Tests: CRON 4D-Torus 9-Port Virtual Channel Router (test_cl_router.rs)
// ============================================================================

use cronc::cl_link::Coord4D;
use cronc::cl_router::{
    render_ascii_router_hud, synthesize_verilog_router, NoCRouterConfig,
    NoCRouterSimulation, RouterPort,
};

#[test]
fn test_router_port_mapping_and_opposite() {
    let all = RouterPort::all();
    assert_eq!(all.len(), 9);

    assert_eq!(RouterPort::EastX.opposite(), RouterPort::WestX);
    assert_eq!(RouterPort::WestX.opposite(), RouterPort::EastX);
    assert_eq!(RouterPort::NorthY.opposite(), RouterPort::SouthY);
    assert_eq!(RouterPort::SouthY.opposite(), RouterPort::NorthY);
    assert_eq!(RouterPort::UpZ.opposite(), RouterPort::DownZ);
    assert_eq!(RouterPort::DownZ.opposite(), RouterPort::UpZ);
    assert_eq!(RouterPort::InW.opposite(), RouterPort::OutW);
    assert_eq!(RouterPort::OutW.opposite(), RouterPort::InW);
    assert_eq!(RouterPort::Local.opposite(), RouterPort::Local);

    for (idx, p) in all.iter().enumerate() {
        assert_eq!(p.index(), idx);
        assert_eq!(RouterPort::from_index(idx).unwrap(), *p);
    }
}

#[test]
fn test_route_compute_dor_and_escape_vc() {
    let cfg = NoCRouterConfig {
        coord: Coord4D::new(0, 0, 0, 0).unwrap(),
        ..Default::default()
    };
    let sim = NoCRouterSimulation::new(cfg);

    // Forward routing (+X) without wrap -> VC0
    let (p1, vc1) = sim.route_compute_dor(&Coord4D::new(2, 0, 0, 0).unwrap());
    assert_eq!(p1, RouterPort::EastX);
    assert_eq!(vc1, 0);

    // Wrap-around routing (-X wrap) -> VC1 (Dally-Seitz escape channel)
    let (p2, vc2) = sim.route_compute_dor(&Coord4D::new(3, 0, 0, 0).unwrap());
    assert_eq!(p2, RouterPort::WestX);
    assert_eq!(vc2, 1);

    // Dimension Y after X matched
    let (p3, vc3) = sim.route_compute_dor(&Coord4D::new(0, 1, 0, 0).unwrap());
    assert_eq!(p3, RouterPort::NorthY);
    assert_eq!(vc3, 0);

    // Local ejection
    let (p4, vc4) = sim.route_compute_dor(&Coord4D::new(0, 0, 0, 0).unwrap());
    assert_eq!(p4, RouterPort::Local);
    assert_eq!(vc4, 0);
}

#[test]
fn test_flit_packet_injection_and_cycle_traversal() {
    let cfg = NoCRouterConfig {
        coord: Coord4D::new(1, 1, 1, 1).unwrap(),
        ..Default::default()
    };
    let mut sim = NoCRouterSimulation::new(cfg);

    let payload = [[1u32, 2, 3, 4], [5, 6, 7, 8]];
    let dst = Coord4D::new(2, 1, 1, 1).unwrap();

    let res = sim.inject_packet(dst, &payload);
    assert!(res.is_ok(), "Packet injection should succeed");
    assert_eq!(sim.total_flits_injected, 4); // Head + 2 Bodies + Tail

    // Step cycle
    sim.step_cycle();
    assert!(sim.total_flits_traversed > 0);
    assert_eq!(sim.current_cycle, 1);
}

#[test]
fn test_credit_based_backpressure() {
    let cfg = NoCRouterConfig {
        coord: Coord4D::new(0, 0, 0, 0).unwrap(),
        buffer_depth_per_vc: 4,
        ..Default::default()
    };
    let mut sim = NoCRouterSimulation::new(cfg);

    let payload = [[1u32; 4]];
    let dst = Coord4D::new(1, 0, 0, 0).unwrap();

    // First injection: 3 flits (Head, Body, Tail) -> Fits in 4-flit buffer
    assert!(sim.inject_packet(dst, &payload).is_ok());

    // Second injection: 3 flits -> Exceeds remaining buffer space (1 flit left) -> Backpressure
    let overflow = sim.inject_packet(dst, &payload);
    assert!(overflow.is_err(), "Must apply credit backpressure when buffer cannot fit packet");
}

#[test]
fn test_synthesize_verilog_router() {
    let cfg = NoCRouterConfig::default();
    let verilog = synthesize_verilog_router(&cfg);

    assert!(verilog.contains("module noc_router_4d"));
    assert!(verilog.contains("parameter FLIT_WIDTH = 128"));
    assert!(verilog.contains("parameter NUM_PORTS  = 9"));
    assert!(verilog.contains("credit_out"));
    assert!(verilog.contains("credit_in"));
    assert!(verilog.contains("crossbar_grants"));
    assert!(verilog.contains("endmodule"));
}

#[test]
fn test_ascii_router_hud_and_json() {
    let cfg = NoCRouterConfig {
        coord: Coord4D::new(1, 2, 0, 3).unwrap(),
        ..Default::default()
    };
    let mut sim = NoCRouterSimulation::new(cfg);
    sim.step_cycle();

    let hud = render_ascii_router_hud(&sim);
    assert!(hud.contains("CRON 4D-TORUS 9-PORT VIRTUAL CHANNEL ROUTER MICRO-ARCHITECTURE HUD"));
    assert!(hud.contains("9-Port Virtual Channel Buffer Occupancy"));
    assert!(hud.contains("9x9 Crossbar Switch Routing & Arbitration Matrix"));
    assert!(hud.contains("Peak Aggregate BW:       360.00 GB/s"));

    let json = sim.to_json();
    assert!(json.contains("\"current_cycle\": 1"));
    assert!(json.contains("\"router_coord\": [1, 2, 0, 3]"));
    assert!(json.contains("\"num_ports\": 9"));
    assert!(json.contains("\"virtual_channels\""));
}
