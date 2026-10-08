// =============================================================================
// CRON 65,536-Core Wafer-Scale Autonomous Swarm Engine: Integration Test Suite
// Validates: 8D Hyper-Torus topology, 8D-DOR routing, 3-Tier Hierarchical Consensus,
// 1F1B Zero-Bubble Pipeline Parallelism, agent hierarchy, and full wafer telemetry.
// =============================================================================

use cronc::cl_swarm::{AgentRole, PacketKind};
use cronc::cl_swarm_wafer::{
    CollisionAvoidanceBuffer, Coord8D, PhotonicWaveguidePhysics, PipelineSchedule,
    VirtualChannel, WaferDORRouter, WaferHierarchy, WaferSwarmMesh, WaferSwarmPacket,
    CORES_PER_DIE, TOTAL_DIES, TOTAL_WAFER_CORES, WAFER_DIES_X, WAFER_DIES_Y,
};

// -----------------------------------------------------------------------------
// 1. 8D Spatial Coordinate System & Bijective Mapping Tests
// -----------------------------------------------------------------------------

#[test]
fn test_coord8d_bijective_mapping_all_65536_cores() {
    for id in 0..TOTAL_WAFER_CORES {
        let coord = Coord8D::from_global_id(id);
        let reconstructed = coord.to_global_id();
        assert_eq!(
            id, reconstructed,
            "Bijective mapping failed for Core ID {}: got {:?}",
            id, coord
        );
    }
}

#[test]
fn test_coord8d_die_id_and_local_core_id() {
    let coord = Coord8D::new(5, 7, 1, 3, 2, 1, 3, 0);
    // Die ID: 7 * 16 + 5 = 117
    assert_eq!(coord.die_id(), 117);
    // Local Core ID: 0 * 64 + 3 * 16 + 1 * 4 + 2 = 54
    assert_eq!(coord.local_core_id(), 54);
    assert_eq!(coord.to_global_id(), 117 * CORES_PER_DIE + 54);
}

#[test]
fn test_coord8d_coordinate_bounds_wrapping() {
    let coord = Coord8D::new(20, 35, 6, 7, 8, 9, 10, 11);
    assert_eq!(coord.wafer_x, 20 % WAFER_DIES_X); // 4
    assert_eq!(coord.wafer_y, 35 % WAFER_DIES_Y); // 3
    assert_eq!(coord.chip_x, 6 % 4);             // 2
    assert_eq!(coord.chip_y, 7 % 4);             // 3
    assert_eq!(coord.x, 8 % 4);                  // 0
    assert_eq!(coord.y, 9 % 4);                  // 1
    assert_eq!(coord.z, 10 % 4);                 // 2
    assert_eq!(coord.w, 11 % 4);                 // 3
}

#[test]
fn test_coord8d_quadrant_id_calculation() {
    // Quadrant is (wafer_y / 4) * 4 + (wafer_x / 4) in 0..15
    let c0 = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(c0.quadrant_id(), 0);

    let c_top_right = Coord8D::new(15, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(c_top_right.quadrant_id(), 3);

    let c_bottom_right = Coord8D::new(15, 15, 0, 0, 0, 0, 0, 0);
    assert_eq!(c_bottom_right.quadrant_id(), 15);
}

// -----------------------------------------------------------------------------
// 2. 8D Dimension-Order Routing (8D-DOR) Distance & Path Tests
// -----------------------------------------------------------------------------

#[test]
fn test_wafer_dor_same_coordinate_distance_zero() {
    let c = Coord8D::new(4, 8, 1, 2, 3, 0, 1, 2);
    assert_eq!(WaferDORRouter::wafer_distance(&c, &c), 0);
}

#[test]
fn test_wafer_dor_adjacent_intra_core_distance_one() {
    let c1 = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    let c2 = Coord8D::new(0, 0, 0, 0, 1, 0, 0, 0);
    assert_eq!(WaferDORRouter::wafer_distance(&c1, &c2), 1);
}

#[test]
fn test_wafer_dor_adjacent_die_distance_one() {
    let c1 = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    let c2 = Coord8D::new(1, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(WaferDORRouter::wafer_distance(&c1, &c2), 1);
}

#[test]
fn test_wafer_dor_torus_wraparound_minimizes_distance() {
    // Distance between 0 and 15 in wafer dimension is 1 (wraparound)
    let c1 = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    let c2 = Coord8D::new(15, 15, 3, 3, 3, 3, 3, 3);
    // Wafer X: 1, Wafer Y: 1, Chip X: 1, Chip Y: 1, Core X: 1, Y: 1, Z: 1, W: 1
    // Total = 8
    assert_eq!(WaferDORRouter::wafer_distance(&c1, &c2), 8);
}

#[test]
fn test_wafer_dor_max_diameter_bounded() {
    // In 16x16 wafer torus, max distance is 8 per wafer dim.
    // In 4^4 chip & core torus, max distance is 2 per dim.
    // Max diameter = 8 + 8 + 2 + 2 + 2 + 2 + 2 + 2 = 28 hops.
    let c1 = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    let c2 = Coord8D::new(8, 8, 2, 2, 2, 2, 2, 2);
    let max_dist = WaferDORRouter::wafer_distance(&c1, &c2);
    assert_eq!(max_dist, 28);
    assert!(max_dist <= 32, "Wafer diameter must never exceed 32 hops");
}

#[test]
fn test_wafer_dor_route_step_advances_wafer_x_first() {
    let src = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    let dst = Coord8D::new(3, 4, 1, 1, 1, 1, 1, 1);
    let next = WaferDORRouter::route_step(&src, &dst);
    assert_eq!(next.wafer_x, 1);
    assert_eq!(next.wafer_y, 0);
    assert_eq!(next.chip_x, 0);
}

#[test]
fn test_wafer_dor_route_step_advances_wafer_y_when_wafer_x_matches() {
    let src = Coord8D::new(3, 0, 0, 0, 0, 0, 0, 0);
    let dst = Coord8D::new(3, 4, 1, 1, 1, 1, 1, 1);
    let next = WaferDORRouter::route_step(&src, &dst);
    assert_eq!(next.wafer_x, 3);
    assert_eq!(next.wafer_y, 1);
}

#[test]
fn test_wafer_dor_route_step_stays_at_target() {
    let target = Coord8D::new(5, 9, 2, 1, 0, 3, 2, 1);
    let next = WaferDORRouter::route_step(&target, &target);
    assert_eq!(next, target);
}

// -----------------------------------------------------------------------------
// 3. 1F1B Pipeline Schedule Tests
// -----------------------------------------------------------------------------

#[test]
fn test_1f1b_pipeline_schedule_creation() {
    let schedule = PipelineSchedule::new_1f1b(8, 32);
    assert_eq!(schedule.num_stages, 8);
    assert_eq!(schedule.num_micro_batches, 32);
    assert_eq!(schedule.total_bubbles, 7); // stages - 1
    assert_eq!(schedule.forward_slots.len(), 8);
    assert_eq!(schedule.backward_slots.len(), 8);
    for stage in 0..8 {
        assert_eq!(schedule.forward_slots[stage].len(), 32);
        assert_eq!(schedule.backward_slots[stage].len(), 32);
    }
}

// -----------------------------------------------------------------------------
// 4. 65,536-Core Mesh Initialization & Hierarchy Tests
// -----------------------------------------------------------------------------

#[test]
fn test_mesh_creates_exactly_65536_agents() {
    let mesh = WaferSwarmMesh::new_65536();
    assert_eq!(mesh.agents.len(), TOTAL_WAFER_CORES);
    assert_eq!(mesh.inboxes.len(), TOTAL_WAFER_CORES);
    assert_eq!(mesh.total_packets, 0);
    assert_eq!(mesh.inter_die_packets, 0);
    assert_eq!(mesh.intra_die_packets, 0);
}

#[test]
fn test_mesh_hierarchy_distribution() {
    let mesh = WaferSwarmMesh::new_65536();
    let mut roots = 0usize;
    let mut quadrant_leaders = 0usize;
    let mut die_leaders = 0usize;
    let mut gateways = 0usize;
    let mut workers = 0usize;

    for agent in &mesh.agents {
        match agent.hierarchy {
            WaferHierarchy::WaferRoot => roots += 1,
            WaferHierarchy::QuadrantLeader => quadrant_leaders += 1,
            WaferHierarchy::DieLeader => die_leaders += 1,
            WaferHierarchy::OpticalGateway => gateways += 1,
            WaferHierarchy::WorkerCore => workers += 1,
        }
    }

    assert_eq!(roots, 1, "Exactly 1 WaferRoot at Core 0");
    assert_eq!(quadrant_leaders, 15, "15 QuadrantLeaders for 16-quadrant layout");
    assert_eq!(die_leaders, 240, "240 DieLeaders for the remaining dies");
    assert_eq!(gateways, 256, "256 Optical Gateways (1 per die)");
    assert_eq!(workers, 65536 - 1 - 15 - 240 - 256);
}

#[test]
fn test_mesh_agent_roles_span_all_types() {
    let mesh = WaferSwarmMesh::new_65536();
    let mut roles_seen = std::collections::HashSet::new();
    for agent in &mesh.agents {
        roles_seen.insert(agent.role);
    }

    assert!(roles_seen.contains(&AgentRole::Planner));
    assert!(roles_seen.contains(&AgentRole::Coder));
    assert!(roles_seen.contains(&AgentRole::Verifier));
    assert!(roles_seen.contains(&AgentRole::Critic));
    assert!(roles_seen.contains(&AgentRole::Router));
    assert!(roles_seen.contains(&AgentRole::MemoryArbiter));
    assert!(roles_seen.contains(&AgentRole::SensorIngest));
    assert!(roles_seen.contains(&AgentRole::Actuator));
}

// -----------------------------------------------------------------------------
// 5. Three-Tier Consensus Protocol Execution & Telemetry Tests
// -----------------------------------------------------------------------------

#[test]
fn test_execute_task_achieves_tier1_quorums() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let report = mesh.execute_task("Synthesize 65,536-Core Photonic Attention");

    assert_eq!(report.die_status_matrix.len(), TOTAL_DIES);
    for (die_id, cores, pct, quorum) in &report.die_status_matrix {
        assert_eq!(*cores, 256);
        assert!(*pct >= 66.67, "Die {} did not reach quorum: {}", die_id, pct);
        assert!(*quorum, "Die {} quorum flag must be true", die_id);
    }
}

#[test]
fn test_execute_task_achieves_all_three_tiers() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let report = mesh.execute_task("Partition 100B Parameter Model across 256 Photonic Dies");

    assert!(report.consensus_achieved);
    assert_eq!(report.telemetry.consensus_score, 100.0);
    assert_eq!(report.telemetry.tier1_quorums_achieved, TOTAL_DIES);
    assert_eq!(report.telemetry.tier2_quadrant_quorums, 16);
    assert!(report.telemetry.tier3_wafer_quorum);
    assert_eq!(report.telemetry.total_dies, 256);
    assert_eq!(report.telemetry.total_cores, 65536);
    assert_eq!(report.telemetry.active_cores, 65536);
}

#[test]
fn test_execute_task_routes_both_inter_and_intra_die_packets() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let report = mesh.execute_task("Analyze 8D Hyper-Torus packet latency");

    assert!(report.telemetry.inter_die_packets > 0, "Must route inter-die optical packets");
    assert!(report.telemetry.intra_die_packets > 0, "Must route intra-die torus packets");
    assert_eq!(
        report.telemetry.total_packets_routed,
        report.telemetry.inter_die_packets + report.telemetry.intra_die_packets
    );
    assert!(report.telemetry.avg_hop_count > 0.0);
    assert!(report.telemetry.aggregate_photonic_bandwidth_tbps > 0.0);
}

#[test]
fn test_report_json_serialization() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let report = mesh.execute_task("Verify 1F1B Pipeline Parallelism on Wafer");
    let json = report.to_json();

    assert!(json.contains("\"task\": \"Verify 1F1B Pipeline Parallelism on Wafer\""));
    assert!(json.contains("\"consensus_achieved\": true"));
    assert!(json.contains("\"total_dies\": 256"));
    assert!(json.contains("\"total_cores\": 65536"));
    assert!(json.contains("\"tier1_quorums_achieved\": 256"));
    assert!(json.contains("\"tier2_quadrant_quorums\": 16"));
    assert!(json.contains("\"tier3_wafer_quorum\": true"));
    assert!(json.contains("\"pipeline_stages\": 16"));
}

#[test]
fn test_ascii_wafer_hud_rendering() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let report = mesh.execute_task("Render 256-Die Photonic HUD");
    let hud = &report.ascii_wafer_hud;

    assert!(hud.contains("256-Die 65,536-Core 8D Hyper-Torus Wafer-Scale Swarm"));
    assert!(hud.contains("Telemetry: 65536 Cores | 256 Dies"));
    assert!(hud.contains("Pipeline: 16 Stages"));
    assert!(hud.contains("Tier1: 256/256 | Tier2: 16/16 | Tier3: PASS"));
    assert!(hud.contains("Inter-Die Photonic Mesh: 12.8 Tbps/die"));
}

// -----------------------------------------------------------------------------
// 6. Virtual Channels & Zero-Collision Collision Avoidance Tests
// -----------------------------------------------------------------------------

#[test]
fn test_virtual_channel_allocation_direct_and_wraparound() {
    let c_start = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    // Direct step: 0 -> 1 along Wafer X
    let c_direct = Coord8D::new(1, 0, 0, 0, 0, 0, 0, 0);
    let (next, vc) = WaferDORRouter::route_step_vc(&c_start, &c_direct);
    assert_eq!(next.wafer_x, 1);
    assert_eq!(vc, VirtualChannel::VC0_Direct);

    // Wraparound step: 0 -> 15 along Wafer X
    let c_wrap = Coord8D::new(15, 0, 0, 0, 0, 0, 0, 0);
    let (next_wrap, vc_wrap) = WaferDORRouter::route_step_vc(&c_start, &c_wrap);
    assert_eq!(next_wrap.wafer_x, 15);
    assert_eq!(vc_wrap, VirtualChannel::VC1_Wraparound);
}

#[test]
fn test_collision_avoidance_buffer_priority_and_isolation() {
    let mut buf = CollisionAvoidanceBuffer::new();
    assert_eq!(buf.collisions_detected, 0);

    let mk_pkt = |vc: VirtualChannel, payload: &str| WaferSwarmPacket {
        src: Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0),
        dst: Coord8D::new(1, 1, 0, 0, 0, 0, 0, 0),
        kind: PacketKind::Proposal,
        payload: payload.to_string(),
        hops: 1,
        is_inter_die: true,
        optical_wavelength: 0,
        pipeline_tag: 0,
        virtual_channel: vc,
        was_deflected: false,
    };

    // Push in reverse priority order: VC0, VC1, VC2, VC3
    buf.push(mk_pkt(VirtualChannel::VC0_Direct, "direct"));
    buf.push(mk_pkt(VirtualChannel::VC1_Wraparound, "wrap"));
    buf.push(mk_pkt(VirtualChannel::VC2_Detour, "detour"));
    buf.push(mk_pkt(VirtualChannel::VC3_Priority, "priority"));

    assert_eq!(buf.total_packets(), 4);
    assert_eq!(buf.collisions_detected, 0);

    // Must pop in strict priority order: VC3 -> VC2 -> VC1 -> VC0
    let p1 = buf.pop().unwrap();
    assert_eq!(p1.virtual_channel, VirtualChannel::VC3_Priority);
    let p2 = buf.pop().unwrap();
    assert_eq!(p2.virtual_channel, VirtualChannel::VC2_Detour);
    let p3 = buf.pop().unwrap();
    assert_eq!(p3.virtual_channel, VirtualChannel::VC1_Wraparound);
    let p4 = buf.pop().unwrap();
    assert_eq!(p4.virtual_channel, VirtualChannel::VC0_Direct);
    assert!(buf.pop().is_none());
}

#[test]
fn test_8d_thermal_deflection_routing_bypasses_throttled_die() {
    let mut throttled_dies = [false; TOTAL_DIES];
    // Start at Die 0: (0,0)
    let c_start = Coord8D::new(0, 0, 0, 0, 0, 0, 0, 0);
    // Destination at Die 2: (2,0)
    let c_target = Coord8D::new(2, 0, 0, 0, 0, 0, 0, 0);

    // Natural DOR step from (0,0) is Die 1: (1,0) -> id 1
    // Mark Die 1 as thermally throttled (> 105°C)
    throttled_dies[1] = true;

    // Resilient step should detect hot Die 1 and detour orthogonally
    let (detour_next, vc, deflected) = WaferDORRouter::route_step_with_thermal_deflection(
        &c_start,
        &c_target,
        &throttled_dies,
    );

    assert!(deflected, "Routing must deflect around hot Die 1");
    assert_eq!(vc, VirtualChannel::VC2_Detour);
    assert_ne!(detour_next.die_id(), 1, "Next hop must not be the hot die");

    // Full path routing should reach the target without stalls or drops
    let (path, hops, any_deflected) = WaferDORRouter::route_path_zero_collision(
        &c_start,
        &c_target,
        &throttled_dies,
    );
    assert!(any_deflected);
    assert_eq!(*path.last().unwrap(), c_target);
    assert!(hops >= 2);
}

#[test]
fn test_wafer_scale_execution_with_hotspots_and_zero_collision() {
    let mut mesh = WaferSwarmMesh::new_65536();
    // Inject 12 thermal hotspots across the wafer
    mesh.inject_thermal_hotspots(12);

    let report = mesh.execute_task("Synthesize Resilient 8D Convolution across 65,536 Cores");

    assert!(report.consensus_achieved);
    assert_eq!(report.telemetry.packet_collisions, 0, "Packet collisions must be strictly 0");
    assert!(report.telemetry.zero_collision_verified);
    assert!(report.telemetry.throttled_dies_count > 0, "Hotspots must be recorded as throttled");
    assert!(report.telemetry.peak_die_temp_c >= 105.0);
    assert!(report.telemetry.thermal_deflections_count > 0, "Deflection count must be positive");
    assert!(report.ascii_wafer_hud.contains("0 Collisions (VERIFIED)"));
}

// -----------------------------------------------------------------------------
// 10. 4D Sphere LLM Inference, Photonic Physics & SVG/HTML Dashboard Tests
// -----------------------------------------------------------------------------

#[test]
fn test_photonic_physics_modeling() {
    let siph = PhotonicWaveguidePhysics::new();
    assert_eq!(siph.num_dwdm_channels, 8);
    assert_eq!(siph.channel_spacing_ghz, 100.0);
    assert_eq!(siph.waveguide_loss_db_per_cm, 0.15);
    assert_eq!(siph.laser_power_watts, 48.0);
    assert!(siph.bit_error_rate < 1e-14);

    // Test loss over 20 cm
    let loss_20cm = siph.optical_loss_db(20.0);
    assert!((loss_20cm - 4.2).abs() < 1e-6); // 20 * 0.15 + 1.2 = 4.2 dB

    // Test thermal wavelength drift
    let drift_10c = siph.thermal_wavelength_shift_pm(10.0);
    assert_eq!(drift_10c, 1000.0); // 10 * 100 pm/°C = 1000 pm = 1 nm

    // Test micro-heater tuning power
    let tuning_pwr = siph.tuning_power_mw(5.0);
    assert_eq!(tuning_pwr, 62.5); // 5 * 12.5 mW = 62.5 mW
}

#[test]
fn test_wafer_scale_sphere_llm_inference_execution() {
    let mut mesh = WaferSwarmMesh::new_65536();
    let num_tokens = 32usize;
    let report = mesh.execute_sphere_llm_inference("Predict Clifford Geodesic Trajectory", num_tokens);

    assert!(report.consensus_achieved);
    assert_eq!(report.telemetry.active_cores, TOTAL_WAFER_CORES);
    assert_eq!(report.telemetry.packet_collisions, 0);
    assert!(report.telemetry.zero_collision_verified);
    assert!(report.telemetry.tier3_wafer_quorum);

    // Multiplier-free 0 DSP discipline & Geodesic curvature verification
    let expected_geodesic_ops = (num_tokens as u64) * (TOTAL_WAFER_CORES as u64) * 64u64;
    assert_eq!(report.telemetry.total_geodesic_ops, expected_geodesic_ops);
    assert!(report.telemetry.total_geodesic_ops > 0);
    assert_eq!(report.telemetry.aggregate_tera_ops_sec, 655.36);
    assert_eq!(report.telemetry.optical_laser_power_watts, 48.0);
    assert!(report.telemetry.aggregate_photonic_bandwidth_tbps > 0.0);

    // Pipeline verification
    assert_eq!(report.telemetry.pipeline_stages, 16);
    assert_eq!(report.telemetry.pipeline_bubbles, 15);

    // Artifact verification
    assert!(report.svg_wafer_diagram.contains("<svg"));
    assert!(report.svg_wafer_diagram.contains("CRON 65,536-CORE WAFER-SCALE SUPERCOMPUTER"));
    assert!(report.svg_wafer_diagram.contains("300mm Monolithic Silicon Wafer"));
    assert!(report.html_dashboard.contains("<!DOCTYPE html>"));
    assert!(report.html_dashboard.contains("CRON 65,536-Core Wafer-Scale Supercomputer Dashboard"));
    assert!(report.html_dashboard.contains("Photonic Mesh Physics"));
}

#[test]
fn test_wafer_svg_and_html_generation_directly() {
    let mut mesh = WaferSwarmMesh::new_65536();
    mesh.inject_thermal_hotspots(6);
    let report = mesh.execute_task("Direct Visualization Test");

    let svg = mesh.generate_wafer_svg(&report);
    assert!(svg.contains("<circle cx=\"500\" cy=\"500\" r=\"470\""));
    assert!(svg.contains("Die #"));

    let html = mesh.generate_wafer_html_dashboard(&report);
    assert!(html.contains("<html lang=\"en\">"));
    assert!(html.contains("3-Tier Quorum Consensus"));
    assert!(html.contains("Thermal Telemetry"));
}

