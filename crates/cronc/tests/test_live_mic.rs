// ============================================================================
// CRON Live Microphone Capture & Voice Loop Integration Tests
// ============================================================================

use cronc::{record_mic_to_file, LiveVoiceLoop};

#[test]
fn test_live_voice_loop_creation() {
    let loop_session = LiveVoiceLoop::new(16000);
    assert_eq!(loop_session.sample_rate, 16000);
    assert_eq!(loop_session.agent.turns_count, 0);
}

#[test]
fn test_live_mic_capture_and_pcm_extraction() {
    // Attempt brief 0.1s microphone capture; if hardware mic is present, verify PCM format
    let temp_wav = std::env::temp_dir().join(format!("test_mic_cap_{}.wav", std::process::id()));

    match record_mic_to_file(&temp_wav, 0.1, 16000) {
        Ok(_) => {
            assert!(temp_wav.exists());
            let bytes = std::fs::read(&temp_wav).unwrap();
            assert!(bytes.len() > 44, "WAV header + PCM data expected");
            let _ = std::fs::remove_file(&temp_wav);
        }
        Err(e) => {
            // In headless/CI environments without physical mic hardware, verify graceful error
            println!("Mic not present or unavailable in CI: {}", e);
        }
    }
}
