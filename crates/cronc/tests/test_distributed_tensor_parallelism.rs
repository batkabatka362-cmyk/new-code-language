// ============================================================================
// Integration Test: Multi-GPU & Distributed Topology Parallelism
// Tests: crates/cronc/src/cl_distributed.rs
//
// Verifies:
// 1. Column-Parallel & Row-Parallel GEMM sharding across device ranks
// 2. High-performance Ring All-Reduce communication parity
// 3. Bit-exact mathematical equivalence between distributed Megatron-style MLP
//    and single-device monolithic execution (< 1e-5 numerical delta)
// 4. Multi-Device C23 Execution Harness Code Generation
// ============================================================================

use cronc::cl_distributed::{
    execute_distributed_megatron_mlp, execute_reference_monolithic_mlp, ring_allreduce_sum,
    synthesize_distributed_c23_harness, DeviceMesh, DeviceRank, ShardedWeight,
};

#[test]
fn test_device_rank_and_mesh_topologies() {
    let rank0 = DeviceRank::new(0, 4);
    let rank1 = DeviceRank::new(1, 4);
    let rank3 = DeviceRank::new(3, 4);

    assert!(rank0.is_root());
    assert!(!rank1.is_root());
    assert_eq!(rank0.prev_rank(), 3);
    assert_eq!(rank0.next_rank(), 1);
    assert_eq!(rank3.next_rank(), 0);

    let mesh = DeviceMesh::new(8, 2, 2, 2).expect("Valid 3D mesh");
    assert_eq!(mesh.tensor_parallel_size, 2);
    assert_eq!(mesh.pipeline_parallel_size, 2);
    assert_eq!(mesh.data_parallel_size, 2);
}

#[test]
fn test_column_and_row_parallel_sharding() {
    let rows = 4;
    let cols = 8;
    let world_size = 2;
    let mut data = Vec::with_capacity(rows * cols);
    for i in 0..(rows * cols) {
        data.push(i as f32);
    }

    // Column parallel: splits 8 columns into two [4, 4] shards
    let col_shards =
        ShardedWeight::from_monolithic(&data, rows, cols, true, world_size).unwrap();
    assert_eq!(col_shards.len(), 2);
    assert_eq!(col_shards[0].local_cols, 4);
    assert_eq!(col_shards[1].local_cols, 4);

    // Row parallel: splits 4 rows into two [2, 8] shards
    let row_shards =
        ShardedWeight::from_monolithic(&data, rows, cols, false, world_size).unwrap();
    assert_eq!(row_shards.len(), 2);
    assert_eq!(row_shards[0].local_rows, 2);
    assert_eq!(row_shards[1].local_rows, 2);
}

#[test]
fn test_ring_allreduce_sum_numerical_parity() {
    let world_size = 4;
    let n_elements = 16;
    let mut shards = Vec::with_capacity(world_size);

    for r in 0..world_size {
        let shard: Vec<f32> = (0..n_elements).map(|i| (i * 10 + r) as f32).collect();
        shards.push(shard);
    }

    // Expected sum for each element i: sum_{r=0..3} (i * 10 + r) = 4 * i * 10 + (0+1+2+3) = 40 * i + 6
    ring_allreduce_sum(&mut shards);

    for r in 0..world_size {
        for i in 0..n_elements {
            let expected = (40 * i + 6) as f32;
            let actual = shards[r][i];
            let delta = (actual - expected).abs();
            assert!(
                delta < 1e-4,
                "Rank {} index {} mismatch: expected {}, got {}",
                r,
                i,
                expected,
                actual
            );
        }
    }
}

#[test]
fn test_megatron_mlp_distributed_parity_against_monolithic() {
    let d_in = 16;
    let d_hidden = 32;
    let d_out = 8;
    let batch_size = 4;

    // Generate pseudo-random inputs and weights
    let mut seed = 123456789u64;
    let mut rand_f32 = || -> f32 {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        let raw = seed.wrapping_mul(0x2545F4914F6CDD1D) >> 32;
        ((raw & 0xFFFF) as f32) / 65536.0 - 0.5
    };

    let input: Vec<f32> = (0..batch_size * d_in).map(|_| rand_f32()).collect();
    let w1: Vec<f32> = (0..d_in * d_hidden).map(|_| rand_f32()).collect();
    let w2: Vec<f32> = (0..d_hidden * d_out).map(|_| rand_f32()).collect();

    // 1. Single-device monolithic reference execution
    let ref_out = execute_reference_monolithic_mlp(
        &input, &w1, &w2, d_in, d_hidden, d_out, batch_size,
    );

    // 2. Distributed Megatron-style execution with 2 ranks
    let dist_out_2 = execute_distributed_megatron_mlp(
        &input, &w1, &w2, d_in, d_hidden, d_out, batch_size, 2,
    )
    .expect("Distributed 2-rank execution");

    // 3. Distributed Megatron-style execution with 4 ranks
    let dist_out_4 = execute_distributed_megatron_mlp(
        &input, &w1, &w2, d_in, d_hidden, d_out, batch_size, 4,
    )
    .expect("Distributed 4-rank execution");

    assert_eq!(ref_out.len(), dist_out_2.len());
    assert_eq!(ref_out.len(), dist_out_4.len());

    let mut max_err_2 = 0.0f32;
    let mut max_err_4 = 0.0f32;

    for i in 0..ref_out.len() {
        let diff_2 = (dist_out_2[i] - ref_out[i]).abs();
        let diff_4 = (dist_out_4[i] - ref_out[i]).abs();
        if diff_2 > max_err_2 {
            max_err_2 = diff_2;
        }
        if diff_4 > max_err_4 {
            max_err_4 = diff_4;
        }
    }

    println!("Megatron Distributed GEMM Parity Test:");
    println!("  2 Ranks Max Numerical Error: {:.8}", max_err_2);
    println!("  4 Ranks Max Numerical Error: {:.8}", max_err_4);

    assert!(
        max_err_2 < 1e-5,
        "2-rank distributed output must match monolithic reference within 1e-5 (got {:.8})",
        max_err_2
    );
    assert!(
        max_err_4 < 1e-5,
        "4-rank distributed output must match monolithic reference within 1e-5 (got {:.8})",
        max_err_4
    );
}

#[test]
fn test_distributed_c23_harness_synthesis() {
    let harness = synthesize_distributed_c23_harness("swiglu_megatron", 4, 4, 16, 64, 16);
    assert!(harness.contains("#define WORLD_SIZE 4"));
    assert!(harness.contains("#define TP_SIZE 4"));
    assert!(harness.contains("run_device_rank"));
    assert!(harness.contains("silu_f32"));
    assert!(harness.contains("pthread_barrier_wait"));
}
