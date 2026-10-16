// ============================================================================
// CRON CLI Neural Speech & Audio Spectrogram Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_audio_synth_and_stft() {
    let out_file = "test_cli_audio.wav";
    let _ = fs::remove_file(out_file);

    // 1. Synthesize Mongolian speech
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["audio", "synth", "Сайн байна уу, CRON цахиурын хэл!", "-o", out_file])
        .output()
        .expect("Failed to run cron audio synth");

    assert!(out.status.success(), "cron audio synth must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON NEURAL FORMANT SPEECH SYNTHESIZER"));
    assert!(stdout.contains("16-bit PCM RIFF/WAVE"));
    assert!(stdout.contains("acoustic formant frames"));

    assert!(std::path::Path::new(out_file).exists());
    let wav_bytes = fs::read(out_file).expect("Failed to read generated WAV");
    assert!(wav_bytes.len() > 1000);
    assert_eq!(&wav_bytes[0..4], b"RIFF");
    assert_eq!(&wav_bytes[8..12], b"WAVE");

    // 2. Perform STFT analysis & ASCII waterfall
    let stft_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["audio", "stft", out_file, "--ascii"])
        .output()
        .expect("Failed to run cron audio stft");

    assert!(stft_out.status.success(), "cron audio stft must succeed");
    let stft_stdout = String::from_utf8_lossy(&stft_out.stdout);
    assert!(stft_stdout.contains("CRON AUDIO LOG-MEL SPECTROGRAM"));
    assert!(stft_stdout.contains("Mel-Frequency Waterfall"));

    // 3. JSON output mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["audio", "synth", "Artificial Intelligence on 4D Torus", "-o", out_file, "--json"])
        .output()
        .expect("Failed to run cron audio synth --json");

    assert!(json_out.status.success(), "cron audio synth --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"sample_rate\": 16000"));
    assert!(json_str.contains("\"duration_seconds\""));
    assert!(json_str.contains("\"peak_amplitude\""));

    let _ = fs::remove_file(out_file);
}
