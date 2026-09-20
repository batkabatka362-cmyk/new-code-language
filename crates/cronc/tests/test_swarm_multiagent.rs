// =============================================================================
// CRON 256-Core Autonomous Multi-Agent Swarm Runtime: Integration Test Suite
// Validates: 4D-Torus topology, DOR routing, agent lifecycle, consensus protocol
// =============================================================================

use cronc::cl_swarm::*;

// ---------------------------------------------------------------------------
// 1. Mesh Initialization & Agent Topology
// ---------------------------------------------------------------------------

#[test]
fn test_swarm_mesh_creates_256_agents() {
    let mesh = SwarmMesh::new_256();
    assert_eq!(mesh.agents.len(), 256, "SwarmMesh must contain exactly 256 agents");
    assert_eq!(mesh.inboxes.len(), 256, "SwarmMesh must have 256 inbox queues");
    assert_eq!(mesh.total_packets, 0, "Initial packet count must be zero");
}

#[test]
fn test_swarm_default_creates_256() {
    let mesh = SwarmMesh::default();
    assert_eq!(mesh.agents.len(), 256);
}

#[test]
fn test_agent_coordinate_mapping_4x4x4x4() {
    let mesh = SwarmMesh::new_256();
    // Verify each agent's 4D coordinate matches its linear ID
    for id in 0..256 {
        let agent = &mesh.agents[id];
        assert_eq!(agent.id, id);
        let expected_x = id % 4;
        let expected_y = (id / 4) % 4;
        let expected_z = (id / 16) % 4;
        let expected_w = (id / 64) % 4;
        assert_eq!(agent.coord, (expected_x, expected_y, expected_z, expected_w),
            "Agent {} coordinate mismatch: got {:?}, expected ({},{},{},{})",
            id, agent.coord, expected_x, expected_y, expected_z, expected_w);
    }
}

#[test]
fn test_agent_role_assignment() {
    let mesh = SwarmMesh::new_256();
    // Core 0 = Planner
    assert_eq!(mesh.agents[0].role, AgentRole::Planner);
    // Core 255 = Actuator
    assert_eq!(mesh.agents[255].role, AgentRole::Actuator);
    // Cores 1..=64 = Coder
    for id in 1..=64 {
        assert_eq!(mesh.agents[id].role, AgentRole::Coder,
            "Agent {} should be Coder", id);
    }
    // Cores 65..=128 = Verifier
    for id in 65..=128 {
        assert_eq!(mesh.agents[id].role, AgentRole::Verifier,
            "Agent {} should be Verifier", id);
    }
    // Cores 129..=176 = Critic
    for id in 129..=176 {
        assert_eq!(mesh.agents[id].role, AgentRole::Critic,
            "Agent {} should be Critic", id);
    }
    // Cores 177..=208 = Router
    for id in 177..=208 {
        assert_eq!(mesh.agents[id].role, AgentRole::Router,
            "Agent {} should be Router", id);
    }
    // Cores 209..=232 = MemoryArbiter
    for id in 209..=232 {
        assert_eq!(mesh.agents[id].role, AgentRole::MemoryArbiter,
            "Agent {} should be MemoryArbiter", id);
    }
    // Cores 233..=254 = SensorIngest
    for id in 233..=254 {
        assert_eq!(mesh.agents[id].role, AgentRole::SensorIngest,
            "Agent {} should be SensorIngest", id);
    }
}

#[test]
fn test_initial_agent_state_is_idle() {
    let mesh = SwarmMesh::new_256();
    for agent in &mesh.agents {
        assert_eq!(agent.state, AgentState::Idle,
            "Agent {} initial state must be Idle", agent.id);
        assert!(agent.scratchpad.is_empty());
        assert!(agent.cot_trace.is_empty());
        assert_eq!(agent.step_count, 0);
    }
}

// ---------------------------------------------------------------------------
// 2. Dimension-Order Routing (DOR) Verification
// ---------------------------------------------------------------------------

#[test]
fn test_dor_same_coordinate_distance_is_zero() {
    let c = (2, 3, 1, 0);
    assert_eq!(DimensionOrderRouter::torus_distance(c, c), 0);
}

#[test]
fn test_dor_adjacent_cores_distance_is_one() {
    let c1 = (0, 0, 0, 0);
    let c2 = (1, 0, 0, 0);
    assert_eq!(DimensionOrderRouter::torus_distance(c1, c2), 1);
}

#[test]
fn test_dor_torus_wraparound_minimizes_distance() {
    // On a 4-wide torus, distance from 0 to 3 wraps around to 1
    let c1 = (0, 0, 0, 0);
    let c2 = (3, 0, 0, 0);
    assert_eq!(DimensionOrderRouter::torus_distance(c1, c2), 1,
        "Torus wrap-around: dist(0,3) on dim-4 should be 1, not 3");
}

#[test]
fn test_dor_max_manhattan_distance_is_8() {
    // Maximum Manhattan distance on 4x4x4x4 torus = 2+2+2+2 = 8
    let mesh = SwarmMesh::new_256();
    let mut max_dist = 0;
    for i in 0..256 {
        for j in (i + 1)..256 {
            let d = DimensionOrderRouter::torus_distance(
                mesh.agents[i].coord,
                mesh.agents[j].coord,
            );
            if d > max_dist {
                max_dist = d;
            }
        }
    }
    assert!(max_dist <= 8,
        "Maximum Manhattan distance on 4x4x4x4 torus must be <= 8, got {}", max_dist);
}

#[test]
fn test_dor_route_step_advances_x_first() {
    let current = (0, 0, 0, 0);
    let target = (2, 3, 1, 1);
    let next = DimensionOrderRouter::route_step(current, target);
    // DOR: X -> Y -> Z -> W, so X should advance first
    assert_eq!(next, (1, 0, 0, 0), "DOR should route X dimension first");
}

#[test]
fn test_dor_route_step_advances_y_when_x_matches() {
    let current = (2, 0, 0, 0);
    let target = (2, 3, 1, 0);
    let next = DimensionOrderRouter::route_step(current, target);
    assert_eq!(next, (2, 1, 0, 0), "DOR should route Y when X matches");
}

#[test]
fn test_dor_route_step_advances_z_when_xy_match() {
    let current = (2, 3, 0, 0);
    let target = (2, 3, 2, 0);
    let next = DimensionOrderRouter::route_step(current, target);
    assert_eq!(next, (2, 3, 1, 0), "DOR should route Z when X,Y match");
}

#[test]
fn test_dor_route_step_advances_w_when_xyz_match() {
    let current = (2, 3, 2, 0);
    let target = (2, 3, 2, 3);
    let next = DimensionOrderRouter::route_step(current, target);
    assert_eq!(next, (2, 3, 2, 1), "DOR should route W when X,Y,Z match");
}

#[test]
fn test_dor_route_step_stays_when_at_target() {
    let target = (1, 2, 3, 0);
    let next = DimensionOrderRouter::route_step(target, target);
    assert_eq!(next, target, "DOR should stay put when already at target");
}

// ---------------------------------------------------------------------------
// 3. Agent Role Symbols & Strings
// ---------------------------------------------------------------------------

#[test]
fn test_agent_role_symbols() {
    assert_eq!(AgentRole::Planner.symbol(), 'P');
    assert_eq!(AgentRole::Coder.symbol(), 'C');
    assert_eq!(AgentRole::Verifier.symbol(), 'V');
    assert_eq!(AgentRole::Critic.symbol(), 'K');
    assert_eq!(AgentRole::Router.symbol(), 'R');
    assert_eq!(AgentRole::MemoryArbiter.symbol(), 'M');
    assert_eq!(AgentRole::SensorIngest.symbol(), 'S');
    assert_eq!(AgentRole::Actuator.symbol(), 'A');
}

#[test]
fn test_agent_role_as_str() {
    assert_eq!(AgentRole::Planner.as_str(), "Planner");
    assert_eq!(AgentRole::Coder.as_str(), "Coder");
    assert_eq!(AgentRole::Verifier.as_str(), "Verifier");
    assert_eq!(AgentRole::Critic.as_str(), "Critic");
    assert_eq!(AgentRole::Router.as_str(), "Router");
    assert_eq!(AgentRole::MemoryArbiter.as_str(), "MemoryArbiter");
    assert_eq!(AgentRole::SensorIngest.as_str(), "SensorIngest");
    assert_eq!(AgentRole::Actuator.as_str(), "Actuator");
}

// ---------------------------------------------------------------------------
// 4. Swarm Task Execution & Consensus
// ---------------------------------------------------------------------------

#[test]
fn test_execute_task_returns_valid_report() {
    let mut mesh = SwarmMesh::new_256();
    let report = mesh.execute_task("Optimize BitNet GEMM kernel");

    assert_eq!(report.task, "Optimize BitNet GEMM kernel");
    assert!(report.consensus_achieved, "256-core consensus should achieve quorum");
    assert!(report.telemetry.consensus_score >= 67.0,
        "Consensus score must be >= 67% for quorum, got {:.1}%", report.telemetry.consensus_score);
    assert_eq!(report.telemetry.total_agents, 256);
    assert_eq!(report.telemetry.active_agents, 256);
    assert!(report.telemetry.total_packets_routed > 0);
    assert!(report.telemetry.avg_hop_count > 0.0);
    assert!(report.telemetry.max_hop_count <= 8);
    assert!(!report.resolution.is_empty());
}

#[test]
fn test_execute_task_broadcasts_255_packets() {
    let mut mesh = SwarmMesh::new_256();
    let report = mesh.execute_task("Test broadcast fidelity");
    // Core 0 sends 1 packet to each of cores 1..255 = 255 packets
    assert_eq!(report.telemetry.total_packets_routed, 255);
}

#[test]
fn test_execute_task_all_agents_reach_completed() {
    let mut mesh = SwarmMesh::new_256();
    let _report = mesh.execute_task("Verify state transitions");
    // Core 0 (Planner) and Core 255 (Actuator) should both be Completed
    assert_eq!(mesh.agents[0].state, AgentState::Completed);
    assert_eq!(mesh.agents[255].state, AgentState::Completed);
    // All processing cores should be Completed
    for id in 1..255 {
        assert_eq!(mesh.agents[id].state, AgentState::Completed,
            "Agent {} should be in Completed state after task execution", id);
    }
}

#[test]
fn test_execute_task_coder_agents_generate_kernel() {
    let mut mesh = SwarmMesh::new_256();
    let _report = mesh.execute_task("Synthesize VLIW kernel");
    // Coder agents (1..=64) should have kernel_synth in scratchpad
    for id in 1..=64 {
        assert!(mesh.agents[id].scratchpad.contains_key("kernel_synth"),
            "Coder agent {} should have 'kernel_synth' in scratchpad", id);
        assert!(!mesh.agents[id].cot_trace.is_empty(),
            "Coder agent {} should have CoT trace entries", id);
    }
}

#[test]
fn test_execute_task_verifier_agents_prove_safety() {
    let mut mesh = SwarmMesh::new_256();
    let _report = mesh.execute_task("Formal safety verification");
    // Verifier agents (65..=128) should have safety_proof in scratchpad
    for id in 65..=128 {
        assert!(mesh.agents[id].scratchpad.contains_key("safety_proof"),
            "Verifier agent {} should have 'safety_proof' in scratchpad", id);
    }
}

#[test]
fn test_execute_task_critic_agents_evaluate_cost() {
    let mut mesh = SwarmMesh::new_256();
    let _report = mesh.execute_task("Evaluate energy budget");
    // Critic agents (129..=176) should have cost_metric in scratchpad
    for id in 129..=176 {
        assert!(mesh.agents[id].scratchpad.contains_key("cost_metric"),
            "Critic agent {} should have 'cost_metric' in scratchpad", id);
    }
}

#[test]
fn test_agent_breakdown_contains_all_roles() {
    let mut mesh = SwarmMesh::new_256();
    let report = mesh.execute_task("Role breakdown test");
    // The breakdown should include all 8 roles
    let roles_in_breakdown: Vec<_> = report.agent_breakdown.iter().map(|(r, _)| *r).collect();
    assert!(roles_in_breakdown.contains(&AgentRole::Planner));
    assert!(roles_in_breakdown.contains(&AgentRole::Coder));
    assert!(roles_in_breakdown.contains(&AgentRole::Verifier));
    assert!(roles_in_breakdown.contains(&AgentRole::Critic));
    assert!(roles_in_breakdown.contains(&AgentRole::Router));
    assert!(roles_in_breakdown.contains(&AgentRole::MemoryArbiter));
    assert!(roles_in_breakdown.contains(&AgentRole::SensorIngest));
    assert!(roles_in_breakdown.contains(&AgentRole::Actuator));
    // Total from breakdown should sum to 256
    let total: usize = report.agent_breakdown.iter().map(|(_, c)| c).sum();
    assert_eq!(total, 256, "Agent breakdown must sum to 256");
}

#[test]
fn test_execute_task_actuator_cot_contains_quorum() {
    let mut mesh = SwarmMesh::new_256();
    let _report = mesh.execute_task("Quorum check");
    let actuator = &mesh.agents[255];
    assert!(!actuator.cot_trace.is_empty(),
        "Actuator (Core 255) must have CoT trace after task");
    assert!(actuator.cot_trace[0].contains("quorum"),
        "Actuator CoT should mention quorum");
}

// ---------------------------------------------------------------------------
// 5. Swarm ASCII Visualization
// ---------------------------------------------------------------------------

#[test]
fn test_ascii_swarm_matrix_contains_all_role_symbols() {
    let mesh = SwarmMesh::new_256();
    let ascii = mesh.render_ascii_swarm_matrix();
    assert!(ascii.contains('P'), "ASCII matrix must show Planner");
    assert!(ascii.contains('C'), "ASCII matrix must show Coder");
    assert!(ascii.contains('V'), "ASCII matrix must show Verifier");
    assert!(ascii.contains('K'), "ASCII matrix must show Critic");
    assert!(ascii.contains('R'), "ASCII matrix must show Router");
    assert!(ascii.contains('M'), "ASCII matrix must show MemoryArbiter");
    assert!(ascii.contains('S'), "ASCII matrix must show SensorIngest");
    assert!(ascii.contains('A'), "ASCII matrix must show Actuator");
    assert!(ascii.contains("Legend"), "ASCII matrix must include legend");
}

// ---------------------------------------------------------------------------
// 6. Packet & Routing Invariants
// ---------------------------------------------------------------------------

#[test]
fn test_packet_kind_variants() {
    let kinds = vec![
        PacketKind::TaskBroadcast,
        PacketKind::Proposal,
        PacketKind::Vote,
        PacketKind::ConsensusResult,
    ];
    // Ensure equality and debug formatting work
    assert_eq!(kinds[0], PacketKind::TaskBroadcast);
    assert_ne!(kinds[0], PacketKind::Vote);
    let dbg = format!("{:?}", PacketKind::Proposal);
    assert!(dbg.contains("Proposal"));
}

#[test]
fn test_swarm_packet_hop_count_matches_dor() {
    let src = (0, 0, 0, 0);
    let dst = (2, 1, 3, 2);
    let expected_hops = DimensionOrderRouter::torus_distance(src, dst);
    let packet = SwarmPacket {
        src,
        dst,
        kind: PacketKind::TaskBroadcast,
        payload: "test".to_string(),
        hops: expected_hops,
    };
    assert_eq!(packet.hops, expected_hops);
}

// ---------------------------------------------------------------------------
// 7. Telemetry Structure Validation
// ---------------------------------------------------------------------------

#[test]
fn test_swarm_telemetry_default() {
    let t = SwarmTelemetry::default();
    assert_eq!(t.total_agents, 0);
    assert_eq!(t.active_agents, 0);
    assert_eq!(t.total_packets_routed, 0);
    assert_eq!(t.avg_hop_count, 0.0);
    assert_eq!(t.max_hop_count, 0);
    assert_eq!(t.consensus_score, 0.0);
    assert_eq!(t.execution_cycles, 0);
    assert_eq!(t.consensus_latency_us, 0.0);
}

#[test]
fn test_telemetry_populated_after_execution() {
    let mut mesh = SwarmMesh::new_256();
    let report = mesh.execute_task("Telemetry population test");
    let t = &report.telemetry;
    assert_eq!(t.total_agents, 256);
    assert_eq!(t.active_agents, 256);
    assert!(t.total_packets_routed > 0);
    assert!(t.avg_hop_count > 0.0);
    assert!(t.max_hop_count > 0);
    assert!(t.consensus_score > 0.0);
    assert!(t.execution_cycles > 0);
    assert!(t.consensus_latency_us > 0.0);
}

// ---------------------------------------------------------------------------
// 8. Sequential Multi-Task Execution
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_accumulates_total_packets_across_tasks() {
    let mut mesh = SwarmMesh::new_256();
    let _r1 = mesh.execute_task("Task Alpha");
    let packets_after_1 = mesh.total_packets;
    assert_eq!(packets_after_1, 255);

    let _r2 = mesh.execute_task("Task Beta");
    let packets_after_2 = mesh.total_packets;
    assert_eq!(packets_after_2, 510, "Total packets should accumulate: 255 + 255 = 510");
}
