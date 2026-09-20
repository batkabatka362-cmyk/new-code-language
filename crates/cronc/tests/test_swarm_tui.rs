// ============================================================================
// Unit Tests for CRON Swarm Interactive Real-Time TUI Visualizer
// ============================================================================

use cronc::{SwarmTuiConfig, SwarmTuiModel, TuiViewMode};

#[test]
fn test_swarm_tui_initial_state() {
    let config = SwarmTuiConfig::default();
    let model = SwarmTuiModel::new(config);

    assert_eq!(model.tick_count, 0);
    assert!(!model.is_paused);
    assert_eq!(model.config.active_view, TuiViewMode::TorusPlane);
    assert_eq!(model.config.selected_z, 0);
    assert_eq!(model.config.selected_w, 0);
    assert!(model.simulated_ipc > 3.0);
    assert_eq!(model.total_healed_faults, 0);
}

#[test]
fn test_swarm_tui_simulation_stepping() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    for _ in 0..15 {
        model.step_tick();
    }

    assert_eq!(model.tick_count, 15);
    assert!(model.total_packets_routed > 0 || !model.packets.is_empty());
    assert!(model.tier2_allreduce_step > 0);
}

#[test]
fn test_swarm_tui_all_four_view_modes_render() {
    let config = SwarmTuiConfig {
        use_color: false,
        ..Default::default()
    };
    let mut model = SwarmTuiModel::new(config);

    // 1. Torus Plane
    model.set_view_mode(TuiViewMode::TorusPlane);
    let frame1 = model.render_frame(92, 24);
    assert!(frame1.contains("4D TORUS (4x4x4x4) SPATIAL MESH - 2D PLANE SLICE"));
    assert!(frame1.contains("CORE INSPECTOR & COT TRACE"));
    assert!(frame1.contains("Toroidal Wraparound"));

    // 2. Cluster Macro
    model.set_view_mode(TuiViewMode::ClusterMacro);
    let frame2 = model.render_frame(92, 24);
    assert!(frame2.contains("16-CHIP 4,096-CORE DISTRIBUTED SWARM CLUSTER"));
    assert!(frame2.contains("Tier-2 AllReduce Ring"));
    assert!(frame2.contains("DWDM OPTICAL MESH"));

    // 3. Router Heatmap
    model.set_view_mode(TuiViewMode::RouterHeatmap);
    let frame3 = model.render_frame(92, 24);
    assert!(frame3.contains("9-PORT VIRTUAL CHANNEL ROUTER MICROARCHITECTURE"));
    assert!(frame3.contains("+X (East)"));
    assert!(frame3.contains("VC0:"));
    assert!(frame3.contains("Credit-Based VC Turn-Model"));

    // 4. Swarm Telemetry
    model.set_view_mode(TuiViewMode::SwarmTelemetry);
    let frame4 = model.render_frame(92, 24);
    assert!(frame4.contains("AUTONOMOUS SWARM SELF-SYNTHESIS & CONTINUOUS SILICON VIBE-HEALING"));
    assert!(frame4.contains("Target Workload:"));
    assert!(frame4.contains("Consensus Quorum:"));
}

#[test]
fn test_swarm_tui_dimension_slice_navigation() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    assert_eq!(model.config.selected_z, 0);
    model.cycle_z(true);
    assert_eq!(model.config.selected_z, 1);
    model.cycle_z(true);
    assert_eq!(model.config.selected_z, 2);
    model.cycle_z(false);
    assert_eq!(model.config.selected_z, 1);

    assert_eq!(model.config.selected_w, 0);
    model.cycle_w(true);
    assert_eq!(model.config.selected_w, 1);
    model.cycle_w(false);
    assert_eq!(model.config.selected_w, 0);
    model.cycle_w(false);
    assert_eq!(model.config.selected_w, 3); // Wraparound
}

#[test]
fn test_swarm_tui_fault_injection_and_vibe_healing() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    assert_eq!(model.total_healed_faults, 0);
    assert!(model.last_fault_msg.is_none());

    model.inject_fault();
    assert_eq!(model.total_healed_faults, 1);
    assert!(model.last_fault_msg.is_some());
    assert!(model.last_fault_msg.as_ref().unwrap().contains("HEALED"));

    model.inject_fault();
    assert_eq!(model.total_healed_faults, 2);
}

#[test]
fn test_swarm_tui_packet_injection() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    let prev_len = model.packets.len();
    model.inject_packet(0, 15);
    assert_eq!(model.packets.len(), prev_len + 1);

    let last_pkt = model.packets.last().unwrap();
    assert_eq!(last_pkt.src_chip, 0);
    assert_eq!(last_pkt.dst_chip, 15);
    assert_eq!(last_pkt.kind_symbol, '★');
}

#[test]
fn test_swarm_tui_toggle_pause() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    assert!(!model.is_paused);
    model.toggle_pause();
    assert!(model.is_paused);
    model.toggle_pause();
    assert!(!model.is_paused);
}

#[test]
fn test_swarm_tui_snapshot_rendering() {
    let config = SwarmTuiConfig {
        use_color: false,
        ..Default::default()
    };
    let mut model = SwarmTuiModel::new(config);

    let snapshot = model.render_snapshot(10);
    assert_eq!(model.tick_count, 10);
    assert!(snapshot.contains("CRON 4D/6D SILICON SWARM"));
    assert!(snapshot.contains("Tick: 00010"));
}

#[test]
fn test_swarm_tui_json_telemetry() {
    let config = SwarmTuiConfig::default();
    let mut model = SwarmTuiModel::new(config);

    model.step_tick();
    let json = model.telemetry_json();
    assert!(json.contains("\"tick_count\": 1"));
    assert!(json.contains("\"simulated_ipc\":"));
    assert!(json.contains("\"chip_traffic_gbps\":"));
    assert!(json.contains("\"consensus_reached\": true"));
}

#[test]
fn test_swarm_tui_ansi_color_formatting() {
    let color_config = SwarmTuiConfig {
        use_color: true,
        ..Default::default()
    };
    let model = SwarmTuiModel::new(color_config);
    let frame_color = model.render_frame(90, 24);
    assert!(frame_color.contains("\x1b["));

    let no_color_config = SwarmTuiConfig {
        use_color: false,
        ..Default::default()
    };
    let model_no_color = SwarmTuiModel::new(no_color_config);
    let frame_no_color = model_no_color.render_frame(90, 24);
    assert!(!frame_no_color.contains("\x1b["));
}

#[test]
fn test_swarm_tui_view_mode_indexing() {
    assert_eq!(TuiViewMode::from_index(1), TuiViewMode::TorusPlane);
    assert_eq!(TuiViewMode::from_index(2), TuiViewMode::ClusterMacro);
    assert_eq!(TuiViewMode::from_index(3), TuiViewMode::RouterHeatmap);
    assert_eq!(TuiViewMode::from_index(4), TuiViewMode::SwarmTelemetry);
    assert_eq!(TuiViewMode::from_index(99), TuiViewMode::TorusPlane);

    assert_eq!(TuiViewMode::TorusPlane.index(), 1);
    assert_eq!(TuiViewMode::ClusterMacro.index(), 2);
    assert_eq!(TuiViewMode::RouterHeatmap.index(), 3);
    assert_eq!(TuiViewMode::SwarmTelemetry.index(), 4);
}
