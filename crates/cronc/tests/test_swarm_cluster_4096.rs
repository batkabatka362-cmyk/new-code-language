// =============================================================================
// CRON 4,096-Core Multi-Chip Distributed Swarm Cluster: Integration Test Suite
// Validates: 6D-Torus topology, 6D-DOR routing, 2-Tier Hierarchical Consensus,
// Optical DWDM collectives, agent hierarchy, and full cluster telemetry.
// =============================================================================

use cronc::cl_swarm::{AgentRole, AgentState};
use cronc::cl_swarm_cluster::{
    ClusterDORRouter, ClusterSwarmMesh, Coord6D, HierarchyLevel, TOTAL_CLUSTER_CORES,
};

// -----------------------------------------------------------------------------
// 1. 6D Spatial Coordinate System & Bijective Mapping Tests
// -----------------------------------------------------------------------------

#[test]
fn test_coord6d_bijective_mapping_all_4096_cores() {
    for id in 0..TOTAL_CLUSTER_CORES {
        let coord = Coord6D::from_global_id(id);
        let reconstructed = coord.to_global_id();
        assert_eq!(
            id, reconstructed,
            "Bijective mapping failed for Core ID {}: got {:?}",
            id, coord
        );
    }
}

#[test]
fn test_coord6d_chip_id_and_local_core_id() {
    let coord = Coord6D::new(2, 3, 1, 2, 3, 0);
    // Chip ID: 3 * 4 + 2 = 14
    assert_eq!(coord.chip_id(), 14);
    // Local Core ID: 0 * 64 + 3 * 16 + 2 * 4 + 1 = 57
    assert_eq!(coord.local_core_id(), 57);
    assert_eq!(coord.to_global_id(), 14 * 256 + 57);
}

#[test]
fn test_coord6d_coordinate_bounds_wrapping() {
    let coord = Coord6D::new(4, 5, 8, 9, 10, 11);
    assert_eq!(coord.chip_x, 0); // 4 % 4
    assert_eq!(coord.chip_y, 1); // 5 % 4
    assert_eq!(coord.x, 0);      // 8 % 4
    assert_eq!(coord.y, 1);      // 9 % 4
    assert_eq!(coord.z, 2);      // 10 % 4
    assert_eq!(coord.w, 3);      // 11 % 4
}

// -----------------------------------------------------------------------------
// 2. 6D Dimension-Order Routing (6D-DOR) Distance & Path Tests
// -----------------------------------------------------------------------------

#[test]
fn test_cluster_dor_same_coordinate_distance_zero() {
    let c = Coord6D::new(1, 2, 3, 0, 1, 2);
    assert_eq!(ClusterDORRouter::cluster_distance(&c, &c), 0);
}

#[test]
fn test_cluster_dor_adjacent_intra_core_distance_one() {
    let c1 = Coord6D::new(0, 0, 0, 0, 0, 0);
    let c2 = Coord6D::new(0, 0, 1, 0, 0, 0);
    assert_eq!(ClusterDORRouter::cluster_distance(&c1, &c2), 1);
}

#[test]
fn test_cluster_dor_adjacent_chip_distance_one() {
    let c1 = Coord6D::new(0, 0, 0, 0, 0, 0);
    let c2 = Coord6D::new(1, 0, 0, 0, 0, 0);
    assert_eq!(ClusterDORRouter::cluster_distance(&c1, &c2), 1);
}

#[test]
fn test_cluster_dor_torus_wraparound_minimizes_distance() {
    // Distance between 0 and 3 in any dimension is 1 (wraparound), not 3
    let c1 = Coord6D::new(0, 0, 0, 0, 0, 0);
    let c2 = Coord6D::new(3, 3, 3, 3, 3, 3);
    // 6 dimensions, each distance 1 => total distance = 6
    assert_eq!(ClusterDORRouter::cluster_distance(&c1, &c2), 6);
}

#[test]
fn test_cluster_dor_max_cluster_diameter_bounded() {
    // In a 4^6 6D torus, maximum distance in any dimension is 2.
    // 6 dimensions * 2 max hops = 12 hops maximum diameter.
    let c1 = Coord6D::new(0, 0, 0, 0, 0, 0);
    let c2 = Coord6D::new(2, 2, 2, 2, 2, 2);
    let max_dist = ClusterDORRouter::cluster_distance(&c1, &c2);
    assert_eq!(max_dist, 12);
    assert!(max_dist <= 20, "Cluster diameter must never exceed 20 hops");
}

#[test]
fn test_cluster_dor_route_step_advances_chip_x_first() {
    let src = Coord6D::new(0, 0, 0, 0, 0, 0);
    let dst = Coord6D::new(2, 2, 2, 2, 2, 2);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.chip_x, 1);
    assert_eq!(next.chip_y, 0);
    assert_eq!(next.x, 0);
}

#[test]
fn test_cluster_dor_route_step_advances_chip_y_when_chip_x_matches() {
    let src = Coord6D::new(2, 0, 0, 0, 0, 0);
    let dst = Coord6D::new(2, 3, 1, 1, 1, 1);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.chip_x, 2);
    assert_eq!(next.chip_y, 1);
    assert_eq!(next.x, 0);
}

#[test]
fn test_cluster_dor_route_step_advances_core_x_when_chips_match() {
    let src = Coord6D::new(1, 1, 0, 0, 0, 0);
    let dst = Coord6D::new(1, 1, 3, 2, 1, 0);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.chip_x, 1);
    assert_eq!(next.chip_y, 1);
    assert_eq!(next.x, 1);
    assert_eq!(next.y, 0);
}

#[test]
fn test_cluster_dor_route_step_advances_core_y_when_chips_and_x_match() {
    let src = Coord6D::new(1, 1, 3, 0, 0, 0);
    let dst = Coord6D::new(1, 1, 3, 2, 1, 0);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.y, 1);
}

#[test]
fn test_cluster_dor_route_step_advances_core_z_when_chips_and_xy_match() {
    let src = Coord6D::new(1, 1, 3, 2, 0, 0);
    let dst = Coord6D::new(1, 1, 3, 2, 2, 0);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.z, 1);
}

#[test]
fn test_cluster_dor_route_step_advances_core_w_when_chips_and_xyz_match() {
    let src = Coord6D::new(1, 1, 3, 2, 2, 0);
    let dst = Coord6D::new(1, 1, 3, 2, 2, 3);
    let next = ClusterDORRouter::route_step(&src, &dst);
    assert_eq!(next.w, 1);
}

#[test]
fn test_cluster_dor_route_step_stays_at_target() {
    let target = Coord6D::new(2, 3, 1, 0, 2, 3);
    let next = ClusterDORRouter::route_step(&target, &target);
    assert_eq!(next, target);
}

// -----------------------------------------------------------------------------
// 3. 4,096-Core Mesh Initialization & Hierarchy Tests
// -----------------------------------------------------------------------------

#[test]
fn test_mesh_creates_exactly_4096_agents() {
    let mesh = ClusterSwarmMesh::new_4096();
    assert_eq!(mesh.agents.len(), 4096);
    assert_eq!(mesh.inboxes.len(), 4096);
    assert_eq!(mesh.total_packets, 0);
    assert_eq!(mesh.inter_chip_packets, 0);
    assert_eq!(mesh.intra_chip_packets, 0);
}

#[test]
fn test_mesh_hierarchy_distribution() {
    let mesh = ClusterSwarmMesh::new_4096();
    let mut roots = 0usize;
    let mut chip_leaders = 0usize;
    let mut gateways = 0usize;
    let mut workers = 0usize;

    for agent in &mesh.agents {
        match agent.hierarchy {
            HierarchyLevel::ClusterRoot => roots += 1,
            HierarchyLevel::ChipLeader => chip_leaders += 1,
            HierarchyLevel::OpticalGateway => gateways += 1,
            HierarchyLevel::WorkerCore => workers += 1,
        }
    }

    assert_eq!(roots, 1, "Exactly 1 ClusterRoot at Core 0");
    assert_eq!(chip_leaders, 15, "15 ChipLeaders for the remaining sockets");
    assert_eq!(gateways, 16, "16 Optical Gateways (1 per socket)");
    assert_eq!(workers, 4096 - 1 - 15 - 16);
}

#[test]
fn test_mesh_agent_roles_span_all_types() {
    let mesh = ClusterSwarmMesh::new_4096();
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
// 4. Two-Tier Consensus Protocol Execution Tests
// -----------------------------------------------------------------------------

#[test]
fn test_execute_task_achieves_tier1_quorums() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let report = mesh.execute_task("Synthesize 4,096-Core BitNet SNN Attention");

    assert_eq!(report.chip_status_matrix.len(), 16);
    for (chip_id, cores, pct, quorum) in &report.chip_status_matrix {
        assert_eq!(*cores, 256);
        assert!(*pct >= 66.67, "Chip {} did not reach quorum: {}", chip_id, pct);
        assert!(*quorum, "Chip {} quorum flag must be true", chip_id);
    }
}

#[test]
fn test_execute_task_achieves_tier2_global_quorum() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let report = mesh.execute_task("Partition BitNet weights across 16 optical sockets");

    assert!(report.consensus_achieved);
    assert_eq!(report.telemetry.consensus_score, 100.0);
    assert!(report.telemetry.tier2_global_quorum);
    assert_eq!(report.telemetry.tier1_quorums_achieved, 16);
    assert_eq!(report.telemetry.total_chips, 16);
    assert_eq!(report.telemetry.total_cores, 4096);
    assert_eq!(report.telemetry.active_cores, 4096);
}

#[test]
fn test_execute_task_routes_both_inter_and_intra_chip_packets() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let report = mesh.execute_task("Analyze 6D-Torus packet congestion");

    assert!(report.telemetry.inter_chip_packets > 0, "Must route inter-chip optical packets");
    assert!(report.telemetry.intra_chip_packets > 0, "Must route intra-chip torus packets");
    assert_eq!(
        report.telemetry.total_packets_routed,
        report.telemetry.inter_chip_packets + report.telemetry.intra_chip_packets
    );
    assert!(report.telemetry.total_packets_routed >= 4096);
}

#[test]
fn test_execute_task_optical_bandwidth_sustained() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let report = mesh.execute_task("Measure DWDM link utilization");

    assert!(report.telemetry.aggregate_optical_bandwidth_tbps > 0.0);
    assert!(report.telemetry.aggregate_optical_bandwidth_tbps <= 51.2);
}

#[test]
fn test_execute_task_all_agents_reach_completed() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    mesh.execute_task("Verify state machine convergence");

    for agent in &mesh.agents {
        assert_eq!(
            agent.state,
            AgentState::Completed,
            "Agent {} not completed",
            agent.global_id
        );
        assert!(!agent.cot_trace.is_empty(), "Agent {} CoT trace empty", agent.global_id);
    }
}

#[test]
fn test_execute_task_accumulates_packets_across_multiple_tasks() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let r1 = mesh.execute_task("Task 1");
    let pkts1 = r1.telemetry.total_packets_routed;

    let r2 = mesh.execute_task("Task 2");
    let pkts2 = r2.telemetry.total_packets_routed;

    assert!(pkts2 > pkts1, "Packet count must accumulate across tasks");
}

#[test]
fn test_ascii_cluster_hud_rendering() {
    let mut mesh = ClusterSwarmMesh::new_4096();
    let report = mesh.execute_task("Render 16-chip topology HUD");

    assert!(report.ascii_cluster_hud.contains("16-Chip 4,096-Core 6D-Torus Swarm Cluster"));
    assert!(report.ascii_cluster_hud.contains("Inter-Chip DWDM Optical Mesh: 3.2 Tbps/socket"));
    assert!(report.ascii_cluster_hud.contains("[Chip 00:"));
    assert!(report.ascii_cluster_hud.contains("[Chip 15:"));
    assert!(report.ascii_cluster_hud.contains("Telemetry: 4096 Cores"));
}
