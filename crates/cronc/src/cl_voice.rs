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

    /// Stream speech response in incremental clause chunks with a live callback
    pub fn stream_interact_text<F>(&mut self, text: &str, mut chunk_callback: F) -> VoiceTurnResult
    where
        F: FnMut(AudioStreamChunk),
    {
        self.turns_count += 1;
        let start_cog = Instant::now();

        let (raw_response, _, sector) = self.mind.query(text);
        let cog_latency = start_cog.elapsed().as_micros() as f32;

        let spoken = Self::clean_for_speech(&raw_response);
        let to_speak = if spoken.is_empty() { "Ойлголоо." } else { &spoken };

        let clauses = Self::split_into_clauses(to_speak);
        let mut total_samples = Vec::new();
        let mut total_speech_latency = 0.0f32;
        let mut total_duration = 0.0f32;

        for (idx, clause) in clauses.iter().enumerate() {
            let is_final = idx + 1 == clauses.len();
            let (pcm, report) = crate::cl_audio::synthesize_speech_waveform(clause, self.sample_rate);
            let wav_chunk = crate::cl_audio::encode_wav_16bit(self.sample_rate, &pcm);

            total_speech_latency += report.latency_us;
            total_duration += report.duration_seconds;
            total_samples.extend_from_slice(&pcm);

            chunk_callback(AudioStreamChunk {
                chunk_index: idx,
                clause_text: clause.clone(),
                duration_seconds: report.duration_seconds,
                pcm_samples: pcm,
                wav_bytes: wav_chunk,
                latency_us: report.latency_us,
                is_final,
            });
        }

        let full_wav = crate::cl_audio::encode_wav_16bit(self.sample_rate, &total_samples);

        VoiceTurnResult {
            user_query: text.to_string(),
            input_was_audio: false,
            stt_confidence: None,
            response_text: raw_response,
            spoken_text: to_speak.to_string(),
            brain_sector: sector.to_string(),
            cognition_latency_us: cog_latency,
            speech_latency_us: total_speech_latency,
            speech_duration_s: total_duration,
            wav_bytes: full_wav,
        }
    }

    /// Split text into phrasal clauses by punctuation or phrase boundary
    pub fn split_into_clauses(text: &str) -> Vec<String> {
        let mut clauses = Vec::new();
        let mut current = String::new();

        for ch in text.chars() {
            current.push(ch);
            if ch == '.' || ch == ',' || ch == '!' || ch == '?' || ch == ';' || ch == '\n' {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    clauses.push(trimmed);
                }
                current.clear();
            }
        }
        let remainder = current.trim().to_string();
        if !remainder.is_empty() {
            clauses.push(remainder);
        }
        if clauses.is_empty() {
            vec![text.to_string()]
        } else {
            clauses
        }
    }
}

/// An incremental real-time streaming audio chunk
#[derive(Debug, Clone)]
pub struct AudioStreamChunk {
    pub chunk_index: usize,
    pub clause_text: String,
    pub duration_seconds: f32,
    pub pcm_samples: Vec<f32>,
    pub wav_bytes: Vec<u8>,
    pub latency_us: f32,
    pub is_final: bool,
}

/// Real-Time Voice Activity Detection (VAD) & Frame Segmenter (20ms frames)
pub struct StreamingVad {
    pub energy_threshold: f32,
    pub frame_size: usize,      // 320 samples = 20ms at 16kHz
    pub silence_frames_max: usize,
    pub active_frames: usize,
    pub silent_frames: usize,
    pub speech_buffer: Vec<f32>,
}

impl StreamingVad {
    pub fn new(sample_rate: u32, threshold: f32) -> Self {
        let frame_size = (sample_rate as usize * 20) / 1000;
        Self {
            energy_threshold: threshold,
            frame_size,
            silence_frames_max: 15, // 300ms silence ends utterance
            active_frames: 0,
            silent_frames: 0,
            speech_buffer: Vec::new(),
        }
    }

    /// Feed incoming 20ms audio frame; returns Some(utterance_pcm) when speech ends
    pub fn feed_frame(&mut self, frame: &[f32]) -> Option<Vec<f32>> {
        let energy: f32 = frame.iter().map(|&x| x * x).sum::<f32>() / frame.len().max(1) as f32;
        let is_speech = energy > self.energy_threshold;

        if is_speech {
            self.speech_buffer.extend_from_slice(frame);
            self.active_frames += 1;
            self.silent_frames = 0;
            None
        } else if self.active_frames > 0 {
            self.speech_buffer.extend_from_slice(frame);
            self.silent_frames += 1;
            if self.silent_frames >= self.silence_frames_max {
                let utterance = std::mem::take(&mut self.speech_buffer);
                self.active_frames = 0;
                self.silent_frames = 0;
                Some(utterance)
            } else {
                None
            }
        } else {
            None
        }
    }
}

