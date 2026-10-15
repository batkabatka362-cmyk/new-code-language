// ============================================================================
// CRON CL-LoRA Ultra-Low Resource Fine-Tuning Test Suite
// Verifies:
// 1. 5-trits sub-byte packing & unpacking roundtrip fidelity.
// 2. Discrete low-rank forward projection without floating point weights.
// 3. Binary serialization footprint (< 2 KB for 32-layer adapter).
// 4. Fast CPU training (< 1.0 s) on question/answer pairs.
// 5. JSONL dataset extraction.
// 6. CL 94-character VLIW microcode bundle generation.
// ============================================================================

use cronc::cl_lora::{
    pack_trits_5, parse_jsonl_training_pairs, train_cl_lora_from_pairs, unpack_trits_5,
    ClLoraAdapter, ClLoraConfig, DiscreteLoraLayer, CLLORA_MAGIC,
};
use std::time::Instant;

#[test]
fn test_subbyte_trit_packing_roundtrip() {
    let test_trits = vec![
        -1, 0, 1, 1, -1, // chunk 0
        0, 0, 0, 1, -1,  // chunk 1
        1, 1, 1, 1, 1,   // chunk 2
        -1, -1, -1, -1, -1, // chunk 3
        0, 1, -1,        // partial chunk
    ];

    let packed = pack_trits_5(&test_trits);
    let unpacked = unpack_trits_5(&packed, test_trits.len());

    assert_eq!(test_trits, unpacked, "Sub-byte 5-trits roundtrip must match exactly");
}

#[test]
fn test_discrete_lora_forward_delta() {
    let in_dim = 4;
    let out_dim = 4;
    let rank = 2;

    // A: 2 x 4 matrix
    // Row 0: [ 1,  0, -1,  0]
    // Row 1: [ 0,  1,  0, -1]
    let a_trits = vec![1, 0, -1, 0, 0, 1, 0, -1];

    // B: 4 x 2 matrix
    // Row 0: [ 1,  0]
    // Row 1: [-1,  1]
    // Row 2: [ 0, -1]
    // Row 3: [ 1,  1]
    let b_trits = vec![1, 0, -1, 1, 0, -1, 1, 1];

    let layer = DiscreteLoraLayer::new(0, "q_proj", in_dim, out_dim, rank, &a_trits, &b_trits);

    let x = vec![10.0, 5.0, 2.0, 1.0];
    // h[0] = 1*10 + 0*5 - 1*2 + 0*1 = 8.0
    // h[1] = 0*10 + 1*5 + 0*2 - 1*1 = 4.0

    // Delta y:
    // o[0] = 1*8 + 0*4 = 8.0
    // o[1] = -1*8 + 1*4 = -4.0
    // o[2] = 0*8 - 1*4 = -4.0
    // o[3] = 1*8 + 1*4 = 12.0
    // scale = 1.0
    let delta = layer.forward_delta(&x, 1.0);

    assert_eq!(delta.len(), 4);
    assert!((delta[0] - 8.0).abs() < 1e-6);
    assert!((delta[1] - (-4.0)).abs() < 1e-6);
    assert!((delta[2] - (-4.0)).abs() < 1e-6);
    assert!((delta[3] - 12.0).abs() < 1e-6);
}

#[test]
fn test_adapter_serialization_and_footprint() {
    let config = ClLoraConfig {
        rank: 2,
        alpha: 2.0,
        hidden_dim: 64,
        num_layers: 32,
        target_modules: vec!["q_proj".to_string()],
    };

    let pairs = vec![
        ("What is CRON?", "CRON is a 0-FLOP sovereign AI operating system."),
        ("Монгол хэл дээр ярьж чадах уу?", "Тийм ээ, би монгол хэлээр бүрэн тооцоолж чадна."),
    ];

    let adapter = train_cl_lora_from_pairs("cron-test-adapter", &pairs, &config);
    let bytes = adapter.save_to_bytes();

    assert_eq!(&bytes[0..4], CLLORA_MAGIC);
    println!("Trained 32-layer adapter binary size: {} bytes", bytes.len());

    // CRITICAL: Adapter footprint must be < 2 KB (2,048 bytes) to fit easily in 35.3 KB L1 cache!
    assert!(
        bytes.len() < 2048,
        "Adapter size ({} bytes) must be strictly less than 2 KB (2048 bytes)!",
        bytes.len()
    );

    // Test roundtrip deserialization
    let loaded = ClLoraAdapter::load_from_bytes(&bytes).expect("Failed to load adapter from bytes");
    assert_eq!(loaded.name, "cron-test-adapter");
    assert_eq!(loaded.layers.len(), 32);
    assert_eq!(loaded.config.rank, 2);
    assert_eq!(loaded.config.hidden_dim, 64);
}

#[test]
fn test_fast_cpu_training_benchmark() {
    let config = ClLoraConfig {
        rank: 2,
        alpha: 2.0,
        hidden_dim: 64,
        num_layers: 16,
        target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
    };

    let pairs = vec![
        ("define bitnet", "BitNet replaces float multipliers with ternary {-1, 0, 1} additions."),
        ("what is sphere matrix?", "Sphere Matrix computes attention along S^3 Riemannian geodesics."),
        ("how does dpsta pack data?", "DPSTA packs 5 ternary weights into 1 byte with 100% lossless recovery."),
        ("хэт бага нөөцийн fine-tuning гэж юу вэ?", "CL-LoRA нь 2KB хэмжээтэй адаптерийг L1 кэшд 1 секундэд сургах систем юм."),
    ];

    let start = Instant::now();
    let adapter = train_cl_lora_from_pairs("benchmark-adapter", &pairs, &config);
    let duration = start.elapsed();

    println!("Fine-tuning completed in: {:?}", duration);
    // Training must finish in well under 1.0 second on CPU
    assert!(
        duration.as_secs_f64() < 1.0,
        "Training must complete in < 1.0s on CPU, took {:?}",
        duration
    );

    // Verify delta application
    let x = vec![0.5f64; 64];
    let delta = adapter.apply_delta(0, "q_proj", &x);
    assert!(delta.is_some());
    assert_eq!(delta.unwrap().len(), 64);
}

#[test]
fn test_jsonl_parser() {
    let jsonl_data = r#"
{"prompt": "Hello", "response": "Hi there!"}
{"instruction": "Calculate 2+2", "output": "4"}
{"input": "Translate cat", "output": "Муур"}
"#;

    let pairs = parse_jsonl_training_pairs(jsonl_data);
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs[0], ("Hello".to_string(), "Hi there!".to_string()));
    assert_eq!(pairs[1], ("Calculate 2+2".to_string(), "4".to_string()));
    assert_eq!(pairs[2], ("Translate cat".to_string(), "Муур".to_string()));
}

#[test]
fn test_cl_microcode_export() {
    let config = ClLoraConfig {
        rank: 2,
        alpha: 2.0,
        hidden_dim: 16,
        num_layers: 1,
        target_modules: vec!["q_proj".to_string()],
    };

    let pairs = vec![("test", "pass")];
    let adapter = train_cl_lora_from_pairs("microcode-export-test", &pairs, &config);

    let cl_code = adapter.export_cl_machine_code();
    assert!(cl_code.contains("// CRON CL-LoRA Microcode: microcode-export-test"));
    assert!(cl_code.contains("B0000: "));
    println!("Generated CL Microcode sample:\n{}", &cl_code[..cl_code.len().min(300)]);
}
