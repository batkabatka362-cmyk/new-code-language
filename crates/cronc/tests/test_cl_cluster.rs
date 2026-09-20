// ============================================================================
// CRON Multi-Die Distributed Mesh & Collective Communication Test Suite
// ============================================================================

use cronc::cl_cluster::{
    generate_distributed_c23_harness, render_cluster_topology_ascii,
    synthesize_collective_schedule, ClusterCoord, CollectiveType, CORES_PER_DIE,
    MAX_DIES_PER_POD,
};
use cronc::verify_cl_program;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_cluster_coord_and_global_indexing() {
    // Core [2, 1, 2, 3] on Die 3
    let c = ClusterCoord::new(3, 2, 1, 2, 3).expect("Valid cluster coordinate");
    let global_id = c.global_core_id();

    // Die 3 * 256 + (2 + 4*1 + 16*2 + 64*3) = 768 + (2 + 4 + 32 + 192) = 768 + 230 = 998
    assert_eq!(global_id, 3 * CORES_PER_DIE + 230);

    // Roundtrip back from global ID
    let reconstructed = ClusterCoord::from_global_core_id(global_id)
        .expect("Reconstruction from global ID must succeed");
    assert_eq!(c, reconstructed);

    // Test bounds checking
    assert!(ClusterCoord::new(MAX_DIES_PER_POD, 0, 0, 0, 0).is_err());
    assert!(ClusterCoord::new(0, 4, 0, 0, 0).is_err());
}

#[test]
fn test_hierarchical_hops() {
    let src = ClusterCoord::new(0, 0, 0, 0, 0).unwrap();
    let neighbor_same_die = ClusterCoord::new(0, 1, 0, 0, 0).unwrap();
    assert_eq!(src.hierarchical_hops(&neighbor_same_die), 1);

    // Remote core on Die 1: Core [0,0,0,0] on Die 0 to Core [0,0,0,0] on Die 1
    // Intra to GW [3,3,3,3]: 1+1+1+1 = 4 hops
    // Die hop: 1
    // Remote GW to dest: 4 hops
    // Total = 9 hops
    let remote_core = ClusterCoord::new(1, 0, 0, 0, 0).unwrap();
    let hops = src.hierarchical_hops(&remote_core);
    assert_eq!(hops, 9);
}

#[test]
fn test_synthesize_collective_schedules() {
    for c_type in [
        CollectiveType::AllReduce,
        CollectiveType::AllGather,
        CollectiveType::ReduceScatter,
        CollectiveType::Broadcast,
    ] {
        let schedule = synthesize_collective_schedule(c_type, 4, 1024)
            .expect("Collective synthesis must succeed");

        assert_eq!(schedule.num_dies, 4);
        assert_eq!(schedule.total_cores, 1024);
        assert!(schedule.ring_steps > 0);
        assert!(schedule.total_optical_packets > 0);
        assert!(schedule.theoretical_latency_cycles > 0);

        // Verify generated microcode bundles
        let report = verify_cl_program(&schedule.microcode_cl)
            .expect("Synthesized collective microcode must pass .cl syntax rules");
        assert!(report.total_bundles >= 4);
        assert_eq!(report.total_slots, report.total_bundles * 4);
    }
}

#[test]
fn test_render_cluster_topology_ascii() {
    let ascii = render_cluster_topology_ascii(4);
    assert!(ascii.contains("CRON MULTI-DIE 5D HIERARCHICAL CLUSTER TOPOLOGY"));
    assert!(ascii.contains("4 Physical Silicon Dies | 1024 Distributed Cores"));
    assert!(ascii.contains("DIE #00"));
    assert!(ascii.contains("DIE #03"));
}

#[test]
fn test_generate_distributed_c23_harness_and_execution() {
    let dummy_cl = "B0000: '==01#00A> _NO00#000> _NO00#000> _HL00#000!\n";
    let c23_code = generate_distributed_c23_harness(dummy_cl, 4);

    assert!(c23_code.contains("#define NUM_DIES 4"));
    assert!(c23_code.contains("#define TOTAL_CORES 1024"));
    assert!(c23_code.contains("CronClusterCore g_cluster_cores[TOTAL_CORES];"));

    // Compile and run with GCC if available
    let temp_dir = std::env::temp_dir();
    let c_file: PathBuf = temp_dir.join("test_cron_cluster_dist.c");
    let exe_file: PathBuf = temp_dir.join("test_cron_cluster_dist.exe");

    fs::write(&c_file, &c23_code).expect("Failed to write C23 harness");

    let gcc_status = Command::new("gcc")
        .args([
            "-O3",
            c_file.to_str().unwrap(),
            "-o",
            exe_file.to_str().unwrap(),
        ])
        .status();

    if let Ok(st) = gcc_status {
        if st.success() {
            let run_res = Command::new(&exe_file)
                .output()
                .expect("Failed to run cluster harness executable");
            assert!(run_res.status.success(), "Cluster harness executable must exit 0");
            let out_str = String::from_utf8_lossy(&run_res.stdout);
            assert!(out_str.contains("[CRON Cluster Engine] Initializing 4 Dies (1024 Cores)..."));
            assert!(out_str.contains("Execution Status: SUCCESS (100% Parity)"));
        }
    }

    let _ = fs::remove_file(c_file);
    let _ = fs::remove_file(exe_file);
}
