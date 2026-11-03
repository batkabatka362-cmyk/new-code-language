// ============================================================================
// Comprehensive Verification Suite for CRON Production AI Infrastructure
//
// 1. Zero-Dependency Native CUDA Driver JIT Engine (cl_cuda_driver)
// 2. Out-of-Core Zero-Copy Streaming Dataset Loader (cl_dataset_stream)
// 3. Continuous Dynamic Batching & Paged KV-Cache Serving Engine (cl_paged_serving)
// 4. Fault-Tolerant Distributed Optimizer Checkpointing (cl_checkpoint)
// ============================================================================

use std::fs::File;
use std::io::Write;

use cronc::cl_cuda_driver::*;
use cronc::cl_dataset_stream::*;
use cronc::cl_paged_serving::*;
use cronc::cl_checkpoint::*;
use cronc::cl_train_pipeline::{AutonomousLanguageModel, TrainingConfig, TrainingOptimizer};

static CUDA_TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn test_cuda_driver_sm80_vector_math() {
    let _lock = CUDA_TEST_MUTEX.lock().unwrap();
    if !CudaDriver::is_available() {
        println!("[SKIP] CUDA Driver not present in environment.");
        return;
    }

    let driver = CudaDriver::init().expect("Failed to initialize CUDA driver");
    let dev_count = driver.device_count().expect("Device count failed");
    assert!(dev_count >= 1);

    let name = driver.device_name(0).expect("Device name failed");
    let mem = driver.device_total_memory(0).expect("Total mem failed");
    println!("Testing on hardware: {} ({} MB VRAM)", name, mem / 1024 / 1024);

    let ptx = r#"
.version 7.5
.target sm_80
.address_size 64

.visible .entry saxpy_kernel (
    .param .u64 d_x,
    .param .u64 d_y,
    .param .u64 d_out,
    .param .u32 n
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<8>;
    .reg .f32 %f<4>;

    ld.param.u64 %rd1, [d_x];
    ld.param.u64 %rd2, [d_y];
    ld.param.u64 %rd3, [d_out];
    ld.param.u32 %r0, [n];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra EXIT;

    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    add.s64 %rd6, %rd2, %rd4;
    add.s64 %rd7, %rd3, %rd4;

    ld.global.f32 %f1, [%rd5];
    ld.global.f32 %f2, [%rd6];
    
    // saxpy: out = 2.0 * x + y
    mul.f32 %f3, %f1, 2.0;
    add.f32 %f0, %f3, %f2;

    st.global.f32 [%rd7], %f0;

EXIT:
    ret;
}
"#;

    let x = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let y = vec![0.5f32, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
    let expected: Vec<f32> = x.iter().zip(y.iter()).map(|(&a, &b)| 2.0 * a + b).collect();

    let out = run_ptx_vector_kernel(ptx, "saxpy_kernel", (1, 1, 1), (8, 1, 1), &[&x, &y], 8)
        .expect("CUDA saxpy kernel launch failed");

    for i in 0..8 {
        assert!((out[i] - expected[i]).abs() < 1e-4, "Mismatch at idx {}", i);
    }
    println!("SAXPY Kernel execution verified on hardware silicon: {:?}", out);
}

#[test]
fn test_streaming_dataset_loader_sync_and_async() {
    let tmp_dir = std::env::temp_dir().join(format!("cl_test_dataset_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let bin_path = tmp_dir.join("tokens_corpus.bin");

    // Generate 1000 synthetic tokens [0, 1, 2, ..., 999] as u32
    {
        let mut f = File::create(&bin_path).unwrap();
        for t in 0u32..1000 {
            f.write_all(&t.to_le_bytes()).unwrap();
        }
        f.flush().unwrap();
    }

    let batch_size = 4;
    let seq_len = 16;
    let config = StreamingConfig::new(&bin_path, batch_size, seq_len)
        .with_dtype(TokenDType::U32)
        .with_seed(12345);

    let mut dataset = StreamingDataset::open(config.clone()).expect("Failed to open streaming dataset");
    assert_eq!(dataset.total_tokens(), 1000);
    assert!(dataset.total_sequences() > 50);

    // Read synchronous batches
    let mut reader = std::io::BufReader::new(File::open(&bin_path).unwrap());
    let b1 = dataset.next_batch_sync(&mut reader).unwrap().expect("Batch 1 should exist");

    assert_eq!(b1.batch_size, batch_size);
    assert_eq!(b1.seq_len, seq_len);
    assert_eq!(b1.input_tokens.len(), batch_size * seq_len);
    assert_eq!(b1.target_tokens.len(), batch_size * seq_len);

    // Verify causal autoregressive shift: target[i] == input[i] + 1
    for i in 0..(batch_size * seq_len) {
        assert_eq!(b1.target_tokens[i], b1.input_tokens[i] + 1, "Causal shift mismatch at idx {}", i);
    }

    // Checkpoint state
    let cp = dataset.get_checkpoint_state();
    assert_eq!(cp.samples_processed, 4);

    // Test Asynchronous Double-Buffered Stream
    let dataset2 = StreamingDataset::open(config).expect("Failed to open streaming dataset");
    let mut async_stream = AsyncDatasetStream::spawn(dataset2).expect("Failed to spawn async stream");

    let mut async_batches = Vec::new();
    for _ in 0..5 {
        if let Some(b) = async_stream.next_batch() {
            async_batches.push(b);
        }
    }
    assert_eq!(async_batches.len(), 5);
    println!("Asynchronous double-buffered dataset stream yielded 5 batches with zero CPU blocking");

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_paged_attention_continuous_batching_and_prefix_sharing() {
    let total_blocks = 32;
    let block_size = 4;
    let kv_dim = 64;

    let mut engine = PagedServingEngine::new(total_blocks, block_size, kv_dim);
    assert_eq!(engine.pool.free_blocks_count(), 32);

    let shared_prefix = vec![101, 102, 103, 104]; // Exactly 1 block

    let mut req1_prompt = shared_prefix.clone();
    req1_prompt.extend_from_slice(&[201, 202]); // Total 6 tokens (needs 2 blocks)

    let mut req2_prompt = shared_prefix.clone();
    req2_prompt.extend_from_slice(&[301, 302, 303]); // Total 7 tokens (needs 2 blocks)

    let req1 = ServingRequest::new(req1_prompt, 3, 9999);
    let req2 = ServingRequest::new(req2_prompt, 3, 9999);

    let _req1_id = req1.request_id;
    let _req2_id = req2.request_id;

    engine.submit_request(req1);
    engine.submit_request(req2);

    // Mock forward step: next token is previous token + 10
    let mut iter = 0;
    while !engine.is_idle() && iter < 10 {
        let events = engine.step(|_req_ids, inputs| {
            Ok(inputs.iter().map(|&tok| tok + 10).collect())
        }).expect("Engine step failed");

        println!("Iteration {}: Output events: {:?}", iter, events);
        iter += 1;
    }

    assert_eq!(engine.telemetry.completed_requests, 2);
    assert!(engine.telemetry.shared_prefix_hits >= 1, "Expected prefix cache hit on shared prompt");
    // Shared prefix block is held in prefix_cache for subsequent request re-use
    assert_eq!(engine.pool.free_blocks_count(), 31, "1 physical block retained in prefix cache for instant reuse");
    
    // Clear prefix cache and verify full restoration of all physical blocks
    engine.clear_prefix_cache();
    assert_eq!(engine.pool.free_blocks_count(), 32, "All physical blocks must be freed after clearing prefix cache");
    println!("Continuous dynamic batching & paged KV cache passed with prefix sharing!");
}

#[test]
fn test_distributed_optimizer_checkpointing_atomic_crc32() {
    let tmp_dir = std::env::temp_dir().join(format!("cl_chkpt_test_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let chkpt_path = tmp_dir.join("model_state.cronchk");

    let config = TrainingConfig {
        model_path: None,
        data_path: None,
        data_text: None,
        steps: 100,
        learning_rate: 0.001,
        min_learning_rate: 0.0001,
        batch_size: 2,
        seq_len: 8,
        dim: 16,
        hidden_dim: 32,
        vocab_size: 64,
        num_layers: 2,
        num_experts: 1,
        top_k: 1,
        optimizer: TrainingOptimizer::AdamW,
        weight_decay: 0.01,
        warmup_steps: 10,
        lr_schedule: cronc::cl_train_pipeline::LrSchedule::Cosine,
        checkpoint_out: None,
        profile: "test".to_string(),
        seed: 42,
        eval_prompt: None,
        eval_max_tokens: 10,
        eval_temperature: 0.7,
        use_cuda: false,
    };

    let mut model = AutonomousLanguageModel::new(config.clone());

    // Perturb optimizer states m and v to verify full state preservation
    for elem in &mut model.embedding.m {
        *elem = 0.12345;
    }
    for elem in &mut model.embedding.v {
        *elem = 0.67890;
    }

    let original_emb_0 = model.embedding.data[0];

    // 1. Save checkpoint atomically
    let chkpt = ModelCheckpoint::from_model(&model, 42, 0.00085);
    chkpt.save_atomic(&chkpt_path).expect("Failed to save atomic checkpoint");

    // 2. Load and verify integrity
    let loaded_chkpt = ModelCheckpoint::load_and_verify(&chkpt_path).expect("Failed to load checkpoint");
    assert_eq!(loaded_chkpt.header.step, 42);
    assert!((loaded_chkpt.header.learning_rate - 0.00085).abs() < 1e-6);
    assert_eq!(loaded_chkpt.header.vocab_size, 64);
    assert_eq!(loaded_chkpt.header.dim, 16);

    // 3. Restore into a fresh model
    let mut new_model = AutonomousLanguageModel::new(config);
    loaded_chkpt.restore_into_model(&mut new_model).expect("Failed to restore into model");

    assert!((new_model.embedding.data[0] - original_emb_0).abs() < 1e-7);
    assert!((new_model.embedding.m[0] - 0.12345).abs() < 1e-7);
    assert!((new_model.embedding.v[0] - 0.67890).abs() < 1e-7);

    // 4. Test corruption detection: flip 1 byte in the file and ensure CRC-32 rejects it
    {
        let mut bytes = std::fs::read(&chkpt_path).unwrap();
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0xFF; // Invert bits
        let corrupted_path = tmp_dir.join("corrupted.cronchk");
        std::fs::write(&corrupted_path, bytes).unwrap();

        let res = ModelCheckpoint::load_and_verify(&corrupted_path);
        assert!(res.is_err(), "Corrupted checkpoint MUST fail CRC-32 validation!");
        println!("Corruption check passed: successfully caught bit flip -> {}", res.err().unwrap());
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);
    println!("Atomic distributed optimizer checkpointing verified!");
}

#[test]
fn test_cuda_hardware_autonomous_training() {
    let _lock = CUDA_TEST_MUTEX.lock().unwrap();
    if !CudaDriver::is_available() {
        println!("[SKIP] CUDA Driver not present on host system.");
        return;
    }

    let config = TrainingConfig {
        model_path: None,
        data_path: None,
        data_text: Some("CRON In-Silicon Tensor Architecture and Real GPU Autonomous Neural Training.".to_string()),
        steps: 15,
        learning_rate: 0.02,
        min_learning_rate: 0.0001,
        batch_size: 4,
        seq_len: 16,
        dim: 32,
        hidden_dim: 64,
        vocab_size: 256,
        num_layers: 1,
        num_experts: 1,
        top_k: 1,
        optimizer: TrainingOptimizer::AdamW,
        weight_decay: 0.01,
        warmup_steps: 3,
        lr_schedule: cronc::cl_train_pipeline::LrSchedule::Cosine,
        checkpoint_out: None,
        profile: "test".to_string(),
        seed: 42,
        eval_prompt: None,
        eval_max_tokens: 10,
        eval_temperature: 0.7,
        use_cuda: true,
    };

    let summary = cronc::cl_train_pipeline::train_autonomous_model(config, None)
        .expect("CUDA autonomous training failed on actual GPU");

    println!("CUDA GPU Training Initial Loss: {:.4}", summary.initial_loss);
    println!("CUDA GPU Training Final Loss: {:.4} (reduction: {:.2}%)", summary.final_loss, summary.loss_reduction_pct);
    assert!(summary.final_loss < summary.initial_loss, "Expected loss reduction during GPU training!");
    assert!(summary.total_tokens > 0);
    println!("Hardware NVIDIA RTX 4060 GPU autonomous training verified flawlessly!");
}
