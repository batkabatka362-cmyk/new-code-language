// ============================================================================
// CRON Automated Verification Suite: Multi-Modal Silicon Vision & Audio Processing
// Test File: test_multimodal_silicon.rs
//
// 100% Pure Rust - Zero External Dependencies
// ============================================================================

use cronc::cl_multimodal::{
    AudioConfig, AudioSpectrogramProcessor, ModalityKind, MultiModalFusion,
    VisionConfig, VisionPatchProcessor,
};

#[test]
fn test_vision_patch_extraction_and_shapes() {
    let config = VisionConfig {
        width: 224,
        height: 224,
        channels: 3,
        patch_size: 16,
        embed_dim: 64,
        is_ternary: true,
    };

    // Synthetic 224x224x3 RGB image
    let total_bytes = config.width * config.height * config.channels;
    let mut rgb = vec![0u8; total_bytes];
    for (i, b) in rgb.iter_mut().enumerate() {
        *b = (i % 256) as u8;
    }

    let patches = VisionPatchProcessor::extract_patches(&rgb, &config);

    // 224 / 16 = 14 patches in x and y -> 14 * 14 = 196 patches
    assert_eq!(patches.len(), 196, "Must yield exactly 196 patches for 224x224 with 16x16 patch size");

    // Each patch: 16 * 16 * 3 = 768 floats
    let expected_patch_dim = 16 * 16 * 3;
    for patch in &patches {
        assert_eq!(patch.len(), expected_patch_dim);
        for &val in patch {
            assert!(val >= -1.0 && val <= 1.0, "Normalized values must fall in [-1.0, 1.0]");
        }
    }
}

#[test]
fn test_vision_ternary_projection_and_positional_embeddings() {
    let config = VisionConfig {
        width: 64,
        height: 64,
        channels: 3,
        patch_size: 16,
        embed_dim: 32,
        is_ternary: true,
    };

    let total_bytes = config.width * config.height * config.channels;
    let rgb = vec![128u8; total_bytes];

    let patches = VisionPatchProcessor::extract_patches(&rgb, &config);
    assert_eq!(patches.len(), 16); // (64/16) * (64/16) = 16 patches

    let projected = VisionPatchProcessor::project_patches(&patches, &config);
    assert_eq!(projected.len(), 16);

    for emb in &projected {
        assert_eq!(emb.len(), config.embed_dim);
        assert!(emb.iter().any(|&v| v != 0.0), "Projected embedding must not be all zeros");
        for &v in emb {
            assert!(v.is_finite(), "Projected values must be finite");
        }
    }

    // Check positional variance between patch 0 (0,0) and patch 15 (3,3)
    let p0 = &projected[0];
    let p15 = &projected[15];
    assert_ne!(p0, p15, "Different spatial positions must have different embeddings");
}

#[test]
fn test_audio_stft_and_mel_filterbank() {
    let config = AudioConfig {
        sample_rate: 16000,
        fft_size: 512,
        hop_length: 160,
        mel_bands: 80,
        embed_dim: 64,
    };

    // Synthesize 1-second 440 Hz test tone at 16 kHz
    let num_samples = 16000;
    let mut pcm = vec![0.0f32; num_samples];
    for i in 0..num_samples {
        let t = i as f32 / config.sample_rate as f32;
        pcm[i] = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.8;
    }

    let mel_frames = AudioSpectrogramProcessor::compute_mel_spectrogram(&pcm, &config);

    // Expected frames: (16000 - 512) / 160 + 1 = 97
    assert!(!mel_frames.is_empty());
    assert_eq!(mel_frames.len(), (num_samples - config.fft_size) / config.hop_length + 1);

    for frame in &mel_frames {
        assert_eq!(frame.len(), config.mel_bands);
        for &val in frame {
            assert!(val.is_finite());
            assert!(val >= 0.0, "Log-mel energies must be non-negative");
        }
    }

    // Projection to hidden dimension
    let projected_audio = AudioSpectrogramProcessor::project_audio_frames(&mel_frames, config.embed_dim);
    assert_eq!(projected_audio.len(), mel_frames.len());
    assert_eq!(projected_audio[0].len(), config.embed_dim);
}

#[test]
fn test_multimodal_cross_modal_sequence_fusion() {
    let dim = 64;

    // 4 text tokens
    let text_emb = vec![vec![0.1f32; dim]; 4];
    // 6 vision patch tokens
    let vis_emb = vec![vec![0.5f32; dim]; 6];
    // 8 audio frame tokens
    let aud_emb = vec![vec![0.9f32; dim]; 8];

    let fused = MultiModalFusion::fuse_modalities(&text_emb, &vis_emb, &aud_emb);

    assert_eq!(fused.len(), 4 + 6 + 8);
    assert_eq!(fused[0].kind, ModalityKind::Text);
    assert_eq!(fused[3].kind, ModalityKind::Text);
    assert_eq!(fused[4].kind, ModalityKind::Vision);
    assert_eq!(fused[9].kind, ModalityKind::Vision);
    assert_eq!(fused[10].kind, ModalityKind::Audio);
    assert_eq!(fused[17].kind, ModalityKind::Audio);
}

#[test]
fn test_terminal_ascii_waterfall_rendering() {
    let config = AudioConfig::default();
    let pcm = vec![0.5f32; 1600];
    let mel = AudioSpectrogramProcessor::compute_mel_spectrogram(&pcm, &config);

    let ascii = MultiModalFusion::render_ascii_spectrogram(&mel, 32);
    assert!(ascii.contains("Mel-Frequency Waterfall"));
    assert!(ascii.contains("Time (Frames)"));
}
