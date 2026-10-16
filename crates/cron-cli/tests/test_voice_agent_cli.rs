// ============================================================================
// CRON CLI Sovereign Voice AI Agent Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_voice_agent_text_and_json() {
    let out_file = "test_voice_reply.wav";
    let _ = fs::remove_file(out_file);

    // 1. Text Query Voice Interaction
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["voice", "-p", "CRON цахиурын хэл гэж юу вэ?", "-o", out_file, "--json"])
        .output()
        .expect("Failed to run cron voice");

    assert!(out.status.success(), "cron voice must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"user_query\""));
    assert!(stdout.contains("\"response_text\""));
    assert!(stdout.contains("\"spoken_text\""));
    assert!(stdout.contains("\"brain_sector\""));
    assert!(stdout.contains("\"wav_bytes\""));

    assert!(std::path::Path::new(out_file).exists());
    let wav_bytes = fs::read(out_file).expect("Failed to read voice output WAV");
    assert!(wav_bytes.len() > 10000);
    assert_eq!(&wav_bytes[0..4], b"RIFF");
    assert_eq!(&wav_bytes[8..12], b"WAVE");

    // 2. Audio-to-Audio Interaction (Input WAV -> STT -> Sovereign Mind -> TTS)
    let reply_file = "test_voice_turn2.wav";
    let _ = fs::remove_file(reply_file);

    let audio_turn = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["voice", "-i", out_file, "-o", reply_file])
        .output()
        .expect("Failed to run cron voice with audio input");

    assert!(audio_turn.status.success(), "cron voice -i must succeed");
    let audio_stdout = String::from_utf8_lossy(&audio_turn.stdout);
    assert!(audio_stdout.contains("CRON SOVEREIGN END-TO-END VOICE AI AGENT"));
    assert!(audio_stdout.contains("Transcribed from audio"));
    assert!(audio_stdout.contains("Cortex Resonance"));
    assert!(audio_stdout.contains("Speech Duration"));

    assert!(std::path::Path::new(reply_file).exists());

    let _ = fs::remove_file(out_file);
    let _ = fs::remove_file(reply_file);
}
