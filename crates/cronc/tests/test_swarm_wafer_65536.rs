// =============================================================================
// CRON 65,536-Core Wafer-Scale Autonomous Swarm Engine: Integration Test Suite
// Validates: 8D Hyper-Torus topology, 8D-DOR routing, 3-Tier Hierarchical Consensus,
// 1F1B Zero-Bubble Pipeline Parallelism, agent hierarchy, and full wafer telemetry.
// =============================================================================

use cronc::cl_swarm::AgentRole;
use cronc::cl_swarm_wafer::{
    Coord8D, PipelineSchedule, WaferDORRouter, WaferHierarchy, WaferSwarmMesh,
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
