use cron_vm::cluster::{ClusterCoord, ClusterSimulator};

#[test]
fn test_cluster_initialization_4096_cores() {
    // Initialize standard 16-chip cluster (16 * 256 = 4,096 cores)
    let cluster = ClusterSimulator::new(16);
    assert_eq!(cluster.num_chips, 16);
    assert_eq!(cluster.total_cores, 4096);
    assert_eq!(cluster.nodes.len(), 16);

    // Verify first and last chip's cores
    let first_core = cluster.get_global_core_dump(0);
    assert_eq!(first_core[14], 0); // Global ID 0
    assert_eq!(first_core[15], 0); // Chip ID 0

    let last_core = cluster.get_global_core_dump(4095);
    assert_eq!(last_core[14], 4095); // Global ID 4095
    assert_eq!(last_core[15], 15);   // Chip ID 15
}

#[test]
fn test_cluster_hierarchical_6d_coordinates() {
    // Test conversion between 6D coordinate and linear global ID
    for global_id in [0, 1, 255, 256, 1024, 2048, 4095] {
        let coord = ClusterCoord::from_global_id(global_id);
        let reconstructed = coord.to_global_id();
        assert_eq!(global_id, reconstructed, "Mismatch at global_id {}", global_id);

        let chip_id = global_id / 256;
        let local_id = global_id % 256;
        assert_eq!(coord.chip_id(), chip_id);
        assert_eq!(coord.local_core_id(), local_id);
    }
}

#[test]
fn test_cluster_global_barrier_synchronization() {
    let mut cluster = ClusterSimulator::new(16);

    let cl_code = r#"
    B0000:_bb00$000> 'NO00$000> ~NO00$000> @NO00$000>
    B0001:_bb00$000> 'NO00$000> ~NO00$000> @NO00$000>
    "#;

    cluster.load_machine_code(cl_code);
    cluster.run(10);

    let stats = cluster.cluster_stats();
    assert_eq!(stats.total_cycles, 2);
    assert_eq!(stats.cluster_barrier_syncs, 2);

    // Verify all 4,096 cores advanced their barrier_count
    for chip in &cluster.nodes {
        for core in &chip.simulator.cores {
            assert!(core.barrier_count >= 2, "Core barrier count expected >= 2");
        }
    }
}

#[test]
fn test_cross_chip_rdma_dma_transfer() {
    let mut cluster = ClusterSimulator::new(16);

    // Direct DMA from Chip 0 (Core 0) to Chip 15 (Core 255 -> Global ID 4095)
    let payload = 0xCAFE_BABE;
    cluster.cross_chip_dma_transfer(0, 4095, payload);

    let dump_target = cluster.get_global_core_dump(4095);
    assert_eq!(dump_target[0], payload, "Target core R0 should receive DMA payload");

    assert_eq!(cluster.cluster_stats().cross_chip_dma_bursts, 1);
}

#[test]
fn test_cross_chip_packet_delivery() {
    let mut cluster = ClusterSimulator::new(16);

    // Step a program while sending a cross-chip packet from Chip 1 to Chip 3
    let cl_code = r#"
    B0000:_NO00$000> 'NO00$000> ~NO00$000> @NO00$000>
    "#;
    cluster.load_machine_code(cl_code);

    let packet_payload = 0x1234_5678;
    cluster.send_cross_chip_packet(256 /* Chip 1, Core 0 */, 768 /* Chip 3, Core 0 */, packet_payload);

    assert_eq!(cluster.cluster_stats().cross_chip_packets, 1);

    // Execute step to flush outbox into inbox
    cluster.step();

    // Verify Chip 3 received packet in its inbox and mailbox register (R1)
    let dump = cluster.get_global_core_dump(768);
    assert_eq!(dump[1], packet_payload, "Chip 3 Core 0 mailbox (R1) should contain packet payload");
}

#[test]
fn test_cluster_wide_cache_invalidation_cc() {
    let mut cluster = ClusterSimulator::new(16);

    let cl_code = r#"
    B0000:_CC00$000> 'NO00$000> ~NO00$000> @NO00$000>
    "#;

    cluster.load_machine_code(cl_code);
    cluster.run(5);

    // All 4,096 cores must have cache_invalidations == 1 and csr_stall_cnt == 0
    for chip in &cluster.nodes {
        for core in &chip.simulator.cores {
            assert!(core.cache_invalidations >= 1);
            assert_eq!(core.csr_stall_cnt, 0);
        }
    }
}

#[test]
fn test_cluster_dvfs_energy_scaling_ee() {
    let mut cluster = ClusterSimulator::new(16);

    let cl_code = r#"
    B0000:_EE00$000> 'NO00$000> ~NO00$000> @NO00$000>
    "#;

    cluster.load_machine_code(cl_code);
    cluster.run(5);

    // Each core saves 450 uW. Across 4,096 cores = 1,843,200 uW total!
    let total_saved = cluster.cluster_stats().aggregate_energy_saved_uw;
    assert!(total_saved >= 1_843_200, "Expected >= 1,843,200 uW saved, got {}", total_saved);

    for chip in &cluster.nodes {
        for core in &chip.simulator.cores {
            assert_eq!(core.thermal_level, 25);
            assert_eq!(core.dvfs_energy_state, 1);
        }
    }
}
