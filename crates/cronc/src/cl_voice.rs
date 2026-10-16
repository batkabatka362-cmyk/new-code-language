// ============================================================================
// CRON Sovereign Voice AI Agent (cl_voice.rs)
// Pure standard library Rust - Zero external dependencies (No Python, No C++)
//
// Capabilities:
// 1. End-to-End Voice Interactive Agent Loop (STT -> Sovereign Brain -> TTS)
// 2. Multiplier-Free 0-DSP Mongolian Speech Recognition (STT)
// 3. 256-Core Neuromorphic Sovereign Mind Cognition & R1 Reasoning
// 4. Klatt/Fant Acoustic Formant Waveform Synthesizer (TTS)
// 5. Automatic Audio Playback Driver (PowerShell SoundPlayer / afplay / aplay)
// ============================================================================

use crate::cl_audio::{
    synthesize_speech_wav, transcribe_speech_pcm, transcribe_speech_wav,
};
use crate::cl_sovereign_mind::SovereignMind;
use std::path::Path;
use std::time::Instant;

/// Result of an End-to-End Voice Interaction Turn
#[derive(Debug, Clone)]
pub struct VoiceTurnResult {
    pub user_query: String,
    pub input_was_audio: bool,
    pub stt_confidence: Option<f32>,
    pub response_text: String,
    pub spoken_text: String,
    pub brain_sector: String,
    pub cognition_latency_us: f32,
    pub speech_latency_us: f32,
    pub speech_duration_s: f32,
    pub wav_bytes: Vec<u8>,
}

/// Sovereign Voice AI Agent
pub struct VoiceAgent {
    pub mind: SovereignMind,
    pub sample_rate: u32,
    pub turns_count: usize,
}

impl Default for VoiceAgent {
    fn default() -> Self {
        Self::new(16000)
    }
}

impl VoiceAgent {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            mind: SovereignMind::new(),
            sample_rate,
            turns_count: 0,
        }
    }

    /// Clean text for speech synthesis (strip markdown, code fences, thinking tags)
    pub fn clean_for_speech(text: &str) -> String {
        let mut clean = String::new();
        let mut in_think = false;
        let mut in_code = false;

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("<think>") {
                in_think = true;
                continue;
            }
            if trimmed.contains("</think>") {
                in_think = false;
                continue;
            }
            if trimmed.starts_with("```") {
                in_code = !in_code;
                continue;
            }
            if in_think || in_code {
                continue;
            }
            if !trimmed.is_empty() {
                if !clean.is_empty() {
                    clean.push(' ');
                }
                clean.push_str(trimmed);
            }
        }

        if clean.is_empty() {
            text.replace('<', "").replace('>', "").trim().to_string()
        } else {
            clean
        }
    }

    /// Process a text query: Brain Cognition -> Formant Speech Synthesis
    pub fn interact_text(&mut self, text: &str) -> VoiceTurnResult {
        self.turns_count += 1;
        let start_cog = Instant::now();

        // 1. Query Sovereign Mind (Brain)
        let (raw_response, _, sector) = self.mind.query(text);
        let cog_latency = start_cog.elapsed().as_micros() as f32;

        // 2. Prepare spoken text
        let spoken = Self::clean_for_speech(&raw_response);
        let to_speak = if spoken.is_empty() { "Ойлголоо." } else { &spoken };

        // 3. Synthesize Formant Speech Waveform (TTS)
        let (wav_bytes, report) = synthesize_speech_wav(to_speak, self.sample_rate);

        VoiceTurnResult {
            user_query: text.to_string(),
            input_was_audio: false,
            stt_confidence: None,
            response_text: raw_response,
            spoken_text: to_speak.to_string(),
            brain_sector: sector.to_string(),
            cognition_latency_us: cog_latency,
            speech_latency_us: report.latency_us,
            speech_duration_s: report.duration_seconds,
            wav_bytes,
        }
    }

    /// Process an audio query from WAV file: STT Transcription -> Brain Cognition -> TTS
    pub fn interact_wav_file(&mut self, wav_path: &Path) -> Result<VoiceTurnResult, String> {
        // 1. Transcribe audio to text (STT)
        let stt_report = transcribe_speech_wav(wav_path)?;
        let query_text = if stt_report.text.is_empty() {
            "Сайн байна уу"
        } else {
            &stt_report.text
        };

        let mut res = self.interact_text(query_text);
        res.input_was_audio = true;
        res.stt_confidence = Some(stt_report.confidence);
        Ok(res)
    }

    /// Process raw PCM samples: STT Transcription -> Brain Cognition -> TTS
    pub fn interact_pcm(&mut self, samples: &[f32], sample_rate: usize) -> VoiceTurnResult {
        let stt_report = transcribe_speech_pcm(samples, sample_rate);
        let query_text = if stt_report.text.is_empty() {
            "Сайн байна уу"
        } else {
            &stt_report.text
        };

        let mut res = self.interact_text(query_text);
        res.input_was_audio = true;
        res.stt_confidence = Some(stt_report.confidence);
        res
    }

    /// Helper to play audio through system speaker
    pub fn play_wav_file(wav_path: &Path) {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("powershell")
                .args(["-c", &format!("(New-Object Media.SoundPlayer '{}').PlaySync()", wav_path.display())])
                .status();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("afplay")
                .arg(wav_path)
                .status();
        }
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let _ = std::process::Command::new("aplay")
                .arg(wav_path)
                .status();
        }
    }
}
