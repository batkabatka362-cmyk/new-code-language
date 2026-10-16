// ============================================================================
// CRON Real-Time Streaming Voice Pipeline Integration Tests
// ============================================================================

use cronc::{AudioStreamChunk, StreamingVad, VoiceAgent};

#[test]
fn test_streaming_voice_clause_chunking_and_latency() {
    let mut agent = VoiceAgent::default();
    let mut chunks: Vec<AudioStreamChunk> = Vec::new();

    let text = "Сайн байна уу! Би бол CRON цахиурын хэл. Бие даасан оюун ухаан байна.";
    let result = agent.stream_interact_text(text, |chunk| {
        chunks.push(chunk);
    });

    assert!(!chunks.is_empty(), "Streaming should generate at least one chunk");
    assert!(chunks.len() >= 2, "Expected multiple phrasal chunks, got {}", chunks.len());

    // Verify first chunk latency is under 45ms (45,000 µs)
    let first_chunk = &chunks[0];
    assert!(
        first_chunk.latency_us < 45_000.0,
        "First chunk latency must be under 45ms, got {:.1} µs",
        first_chunk.latency_us
    );
    assert!(!first_chunk.wav_bytes.is_empty());

    // Verify final chunk marker
    assert!(chunks.last().unwrap().is_final);

    // Verify complete assembled result
    assert!(!result.wav_bytes.is_empty());
    assert_eq!(result.user_query, text);
}

#[test]
fn test_streaming_vad_energy_segmentation() {
    let mut vad = StreamingVad::new(16000, 0.01); // 16kHz, energy threshold 0.01
    let frame_len = vad.frame_size; // 320 samples (20ms)

    let silence_frame = vec![0.0f32; frame_len];
    let speech_frame = vec![0.5f32; frame_len];

    // 1. Initial silence should yield no speech
    assert!(vad.feed_frame(&silence_frame).is_none());

    // 2. Speech frames (active speech starts)
    for _ in 0..5 {
        assert!(vad.feed_frame(&speech_frame).is_none());
    }

    // 3. Silence frames until utterance terminates (15 frames = 300ms)
    let mut utterance = None;
    for _ in 0..15 {
        if let Some(utt) = vad.feed_frame(&silence_frame) {
            utterance = Some(utt);
            break;
        }
    }

    assert!(utterance.is_some(), "VAD must detect end of utterance after silence");
    let samples = utterance.unwrap();
    // 5 speech frames + 15 silence frames = 20 frames * 320 = 6400 samples
    assert!(samples.len() >= 5 * frame_len);
}
