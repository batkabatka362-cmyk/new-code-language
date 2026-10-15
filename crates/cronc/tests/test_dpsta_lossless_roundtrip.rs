//! test_dpsta_lossless_roundtrip.rs
//! Tests 100% lossless bijective 94-state data compaction, Cayley Latin Square
//! isomorphism, and streaming DPSTA memory bounds.

use cronc::cl_dpsta::{
    CompactDatasetEngine, LosslessBijective94, StreamingDpstaReader,
};
use std::fs;

#[test]
fn test_lossless_bijective_94_multilingual_roundtrip() {
    let corpora = [
        "Монгол хэл дээрх DPSTA өгөгдлийн дамжлага: 94 дискрет төлөвт алдагдалгүй шахалт.",
        "BitNet b1.58: Multiplier-free ternary weights {-1, 0, +1} in sub-byte packaging.",
        "def compute_stdp(pre: u32, post: u32) -> i32 {\n\tlet delta = post - pre;\n\treturn delta * 42;\n}\n",
        "4D Clifford Rotor: e_12 e_34 + \\theta \\cdot S^3 hyperspherical geodesic.",
        "Spaces   tabs\t\t\n\nnewlines and punctuation: !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
        "A",
        "AB",
        "ABC",
        "ABCD",
        "ABCDE",
    ];

    for (i, &corpus) in corpora.iter().enumerate() {
        let (states, orig_len) = LosslessBijective94::encode_bytes(corpus.as_bytes());
        for &s in &states {
            assert!(s < 94, "State token must be in 0..93 range");
        }

        let decoded_bytes = LosslessBijective94::decode_bytes(&states, orig_len);
        let recovered = String::from_utf8(decoded_bytes).expect("Valid UTF-8");
        assert_eq!(recovered, corpus, "Corpus #{} failed lossless roundtrip", i);
    }
}

#[test]
fn test_lossless_binary_bytes_roundtrip() {
    // Generate pseudo-random binary data across all 256 byte values
    let mut binary_data = Vec::with_capacity(1024);
    for i in 0..1024 {
        binary_data.push((i * 101 + 37) as u8);
    }

    let (states, orig_len) = LosslessBijective94::encode_bytes(&binary_data);
    let recovered = LosslessBijective94::decode_bytes(&states, orig_len);

    assert_eq!(recovered.len(), binary_data.len());
    assert_eq!(recovered, binary_data, "Binary byte recovery must be 100% exact");
}

#[test]
fn test_cayley_latin_square_bijective_isomorphism() {
    // Every state in 0..93 under every key in 0..93 must uniquely invert
    for key in 0..94u8 {
        let mut forward_seen = [false; 94];
        for state in 0..94u8 {
            let encoded = LosslessBijective94::cayley_forward(state, key);
            assert!(encoded < 94);
            assert!(!forward_seen[encoded as usize], "Collision detected in Cayley Latin Square");
            forward_seen[encoded as usize] = true;

            let decoded = LosslessBijective94::cayley_inverse(encoded, key);
            assert_eq!(decoded, state, "Inverse failed for state {} with key {}", state, key);
        }
    }
}

#[test]
fn test_compact_dataset_lossless_package_file_roundtrip() {
    let text = "Sovereign AGI Core Data Package\nTesting DPSTA 94-State Crystalline Stream.\nМонгол тооцоолол 100%.".repeat(50);
    let tmp_path = std::env::temp_dir().join("test_dpsta_master_roundtrip.dpdata");

    let written_bytes = CompactDatasetEngine::create_lossless_text_package(&text, &tmp_path)
        .expect("Packaging must succeed");
    assert!(written_bytes > 0);

    let recovered = CompactDatasetEngine::load_lossless_text_package(&tmp_path)
        .expect("Loading must succeed");

    assert_eq!(recovered, text, "File roundtrip text must match identically");

    let _ = fs::remove_file(tmp_path);
}

#[test]
fn test_streaming_reader_bounded_rss() {
    // Large synthetic stream (150 KB) to test multiple 35.3 KB chunks
    let content = "DPSTA_L1_BOUNDED_CHUNK_STREAM_DATA_0123456789_!@#$%^&*()_+|".repeat(3000);
    let tmp_path = std::env::temp_dir().join("test_large_stream.dpdata");

    CompactDatasetEngine::create_lossless_text_package(&content, &tmp_path).unwrap();

    let file = fs::File::open(&tmp_path).unwrap();
    let mut reader = StreamingDpstaReader::new(file).unwrap();

    let mut chunks_count = 0;
    let mut total_bytes = 0;

    while let Ok(Some(chunk)) = reader.read_next_chunk() {
        chunks_count += 1;
        total_bytes += chunk.len();
        // Memory per chunk never exceeds L1 cache budget of 35,344 bytes
        assert!(chunk.len() <= StreamingDpstaReader::<fs::File>::BUFFER_SIZE);
    }

    assert!(chunks_count > 1, "Must span multiple chunks for 150 KB payload");
    assert_eq!(total_bytes, reader.token_count);

    let _ = fs::remove_file(tmp_path);
}
