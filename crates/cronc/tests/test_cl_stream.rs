use cronc::{
    synthesize_streaming_pipeline, verify_cl_program, run_cl_jit,
    StreamModality, StreamPipelineConfig,
};

#[test]
fn test_stream_audio_spectrogram_synthesis() {
    let config = StreamPipelineConfig {
        modality: StreamModality::AudioSpectrogram {
            window_size: 512,
            hop_length: 160,
            mel_bands: 80,
            quant_bits: 4,
        },
        buffer_depth_frames: 4,
        core_clock_ghz: 1.6,
    };

    let report = synthesize_streaming_pipeline(&config);

    assert_eq!(report.modality_name, "Audio STFT Spectrogram");
    assert_eq!(report.tokens_per_frame, 80);
    assert_eq!(report.total_pipeline_stages, 4);
    assert!(report.frame_latency_us > 0.0);
    assert!(report.throughput_fps > 0.0);
    assert!(report.noc_bandwidth_gbps > 0.0);
    assert!(report.zero_copy_verified);
    assert_eq!(report.stages.len(), 4);
}

#[test]
fn test_stream_vision_patches_with_sparsity() {
    let config = StreamPipelineConfig {
        modality: StreamModality::VisionPatches {
            frame_width: 224,
            frame_height: 224,
            patch_size: 16,
            channels: 3,
            enable_2_4_sparsity: true,
        },
        buffer_depth_frames: 2,
        core_clock_ghz: 2.0,
    };

    let report = synthesize_streaming_pipeline(&config);

    assert_eq!(report.modality_name, "Vision Transformer Patches");
    assert_eq!(report.tokens_per_frame, 196); // (224/16) * (224/16) = 14 * 14 = 196
    assert!(report.throughput_fps > 1000.0); // Ultra high throughput on 2.0 GHz silicon
    assert!(report.total_pgas_scratchpad_kb > 0.0);
}

#[test]
fn test_stream_pipeline_cl_validity_and_jit() {
    let config = StreamPipelineConfig {
        modality: StreamModality::Sensor1D {
            channels: 16,
            sample_rate_hz: 8000,
            window_samples: 32,
        },
        buffer_depth_frames: 2,
        core_clock_ghz: 1.2,
    };

    let report = synthesize_streaming_pipeline(&config);
    let cl_code = &report.synthesized_cl_pipeline;

    // Verify .cl code format
    let cl_rep = verify_cl_program(cl_code);
    assert!(cl_rep.is_ok(), "Streaming pipeline microcode must be valid .cl: {:?}", cl_rep.err());

    // Execute via JIT
    let jit_res = run_cl_jit(cl_code);
    assert!(jit_res.is_ok(), "JIT execution failed: {:?}", jit_res.err());
}

#[test]
fn test_stream_reports_ascii_and_json() {
    let config = StreamPipelineConfig::default();
    let report = synthesize_streaming_pipeline(&config);

    // Test ASCII rendering
    let ascii = report.render_ascii();
    assert!(ascii.contains("CRON ZERO-COPY MULTI-MODAL SPATIAL STREAMING PIPELINE MAP"));
    assert!(ascii.contains("Stream Modality"));
    assert!(ascii.contains("4D-Torus Spatial Core Dataflow Mapping"));

    // Test JSON serialization
    let json = report.to_json();
    assert!(json.contains("\"modality\":"));
    assert!(json.contains("\"tokens_per_frame\":"));
    assert!(json.contains("\"stages\":"));
    assert!(json.contains("\"zero_copy_verified\": true"));
}
