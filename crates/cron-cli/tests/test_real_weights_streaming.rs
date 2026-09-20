// ============================================================================
// CRON CLI Real Model Weights Ingestion & Paged Streaming Tests
// Module: cron-cli::tests::test_real_weights_streaming
// ============================================================================

use cron_cli::chat::{ChatConfig, ChatSession};
use cron_rt::PagedWeightStreamer;
use cronc::model_importer::gguf::{create_synthetic_gguf, GgufTensorType, GgufValue};
use cronc::model_importer::safetensors::{create_synthetic_safetensors, SafeTensorDType};
use std::fs;

#[test]
fn test_safetensors_real_file_paged_streaming() {
    let temp_dir = std::env::temp_dir().join("cron_test_safetensors_paged");
    let _ = fs::create_dir_all(&temp_dir);
    let st_path = temp_dir.join("model.safetensors");

    // 1. Create a 4-layer synthetic SafeTensors model file on disk
    let raw_w0 = vec![0.1f32; 1024];
    let raw_w1 = vec![0.2f32; 1024];
    let raw_w2 = vec![0.3f32; 1024];
    let raw_w3 = vec![0.4f32; 1024];

    let st_bytes = create_synthetic_safetensors(&[
        ("model.layers.0.weight", SafeTensorDType::F32, &[32, 32], &raw_w0),
        ("model.layers.1.weight", SafeTensorDType::F32, &[32, 32], &raw_w1),
        ("model.layers.2.weight", SafeTensorDType::F32, &[32, 32], &raw_w2),
        ("model.layers.3.weight", SafeTensorDType::F32, &[32, 32], &raw_w3),
    ]);
    fs::write(&st_path, &st_bytes).expect("Write SafeTensors file to disk");

    // 2. Open via PagedWeightStreamer (zero full-model RAM allocation)
    let page_capacity_bytes = 1024 * 1024; // 1 MB
    let streamer = PagedWeightStreamer::from_safetensors_file(&st_path, page_capacity_bytes)
        .expect("Should open SafeTensors file for streaming");

    assert_eq!(streamer.telemetry().layers_streamed, 4);

    let mut buf = vec![0u8; page_capacity_bytes];
    for l in 0..4 {
        let bytes_read = streamer.stream_layer(l, &mut buf).expect("Stream layer");
        assert_eq!(bytes_read, 4096); // 1024 f32 * 4 bytes = 4096 bytes
        assert!(streamer.telemetry().active_memory_bytes <= page_capacity_bytes);
        streamer.evict_layer();
        assert_eq!(streamer.telemetry().active_memory_bytes, 0);
    }

    assert_eq!(streamer.telemetry().evictions_count, 4);
    assert_eq!(streamer.telemetry().total_bytes_read, 4096 * 4);

    // 3. Test ChatSession with real weights path
    let config = ChatConfig {
        model_name: "cron-bitnet-test".to_string(),
        num_layers: 4,
        page_size_mb: 4,
        k_speculative: 4,
        grammar_mode: "unconstrained".to_string(),
        weights_path: Some(st_path.to_str().unwrap().to_string()),
    };

    let mut session = ChatSession::new(config);
    let mut out = Vec::new();
    let telemetry = session.execute_turn("Execute on-disk weights", &mut out)
        .expect("Turn execution with real weights");

    let response = String::from_utf8(out).unwrap();
    assert!(response.contains("CRON cognitive engine processed"));
    assert_eq!(telemetry.active_ram_mb, 4);
}

#[test]
fn test_gguf_real_file_paged_streaming() {
    let temp_dir = std::env::temp_dir().join("cron_test_gguf_paged");
    let _ = fs::create_dir_all(&temp_dir);
    let gguf_path = temp_dir.join("model.gguf");

    // 1. Create a 3-layer synthetic GGUF model file on disk
    let metadata = vec![
        ("general.architecture", GgufValue::String("bitnet".to_string())),
        ("general.alignment", GgufValue::Uint32(32)),
    ];

    let w0 = vec![1u8; 2048];
    let w1 = vec![2u8; 2048];
    let w2 = vec![3u8; 2048];

    let tensors = vec![
        ("blk.0.attn_q.weight", vec![32, 64], GgufTensorType::Q8_0, w0.as_slice()),
        ("blk.1.attn_q.weight", vec![32, 64], GgufTensorType::Q8_0, w1.as_slice()),
        ("blk.2.attn_q.weight", vec![32, 64], GgufTensorType::Q8_0, w2.as_slice()),
    ];

    let gguf_bytes = create_synthetic_gguf(&metadata, &tensors, 32);
    fs::write(&gguf_path, &gguf_bytes).expect("Write GGUF file to disk");

    // 2. Open via PagedWeightStreamer
    let page_capacity_bytes = 512 * 1024; // 512 KB
    let streamer = PagedWeightStreamer::from_gguf_file(&gguf_path, page_capacity_bytes)
        .expect("Should open GGUF file for streaming");

    assert_eq!(streamer.telemetry().layers_streamed, 3);

    let mut buf = vec![0u8; page_capacity_bytes];
    for l in 0..3 {
        let bytes_read = streamer.stream_layer(l, &mut buf).expect("Stream GGUF layer");
        assert_eq!(bytes_read, 2048);
        assert!(streamer.telemetry().active_memory_bytes <= page_capacity_bytes);
        streamer.evict_layer();
    }

    assert_eq!(streamer.telemetry().evictions_count, 3);
}
