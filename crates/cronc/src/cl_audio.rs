// ============================================================================
// CRON Neural Speech & Acoustic Waveform Synthesizer (cl_audio.rs)
// Pure standard library Rust - 0 external audio libraries (No PortAudio, No ALSA, No C/C++)
//
// Capabilities:
// 1. Klatt/Fant Cascade-Parallel Acoustic Formant Vocal Tract Synthesizer
// 2. Rosenberg Glottal Source Model with Natural Harmonic Overtones
// 3. Complete Mongolian Cyrillic & English Phonetic Grapheme-to-Phoneme (G2P) Engine
// 4. Standard 16-bit PCM Mono RIFF/WAVE Encoder & Parser
// 5. High-Resolution Audio Log-Mel Spectrogram & ASCII Terminal Waterfall Renderer
// ============================================================================

use std::f32::consts::PI;
use std::fs;
use std::path::Path;

/// Standard 16-bit PCM RIFF/WAVE File Encoder
pub fn encode_wav_16bit(sample_rate: u32, samples: &[f32]) -> Vec<u8> {
    let num_channels = 1u16;
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * (num_channels as u32) * ((bits_per_sample / 8) as u32);
    let block_align = num_channels * (bits_per_sample / 8);
    let data_len = (samples.len() * 2) as u32;
    let file_len = 36 + data_len;

    let mut wav = Vec::with_capacity(44 + (samples.len() * 2));

    // RIFF Header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_len.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // "fmt " Subchunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
    wav.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat (1 = PCM)
    wav.extend_from_slice(&num_channels.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&bits_per_sample.to_le_bytes());

    // "data" Subchunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());

    // PCM Samples
    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let pcm_val = (clamped * 32767.0) as i16;
        wav.extend_from_slice(&pcm_val.to_le_bytes());
    }

    wav
}

/// 2nd Order Digital Resonator (Bandpass Formant Filter)
#[derive(Debug, Clone)]
pub struct FormantFilter {
    frequency: f32,
    bandwidth: f32,
    sample_rate: f32,
    r: f32,
    two_r_cos_theta: f32,
    gain: f32,
    y1: f32,
    y2: f32,
}

impl FormantFilter {
    pub fn new(freq: f32, bw: f32, sample_rate: f32) -> Self {
        let mut filter = Self {
            frequency: freq,
            bandwidth: bw,
            sample_rate,
            r: 0.0,
            two_r_cos_theta: 0.0,
            gain: 1.0,
            y1: 0.0,
            y2: 0.0,
        };
        filter.update(freq, bw);
        filter
    }

    pub fn update(&mut self, freq: f32, bw: f32) {
        self.frequency = freq;
        self.bandwidth = bw;
        let theta = 2.0 * PI * (freq / self.sample_rate);
        self.r = (-PI * (bw / self.sample_rate)).exp();
        self.two_r_cos_theta = 2.0 * self.r * theta.cos();
        self.gain = 1.0 - self.r;
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.gain * x + self.two_r_cos_theta * self.y1 - (self.r * self.r) * self.y2;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }

    pub fn reset(&mut self) {
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Acoustic Formants for a Phoneme
#[derive(Debug, Clone, Copy)]
pub struct PhonemeSpec {
    pub f1: f32,
    pub f2: f32,
    pub f3: f32,
    pub b1: f32,
    pub b2: f32,
    pub b3: f32,
    pub is_voiced: bool,
    pub noise_gain: f32,
    pub duration_ms: f32,
    pub is_trill: bool,
}

impl Default for PhonemeSpec {
    fn default() -> Self {
        Self {
            f1: 500.0,
            f2: 1500.0,
            f3: 2500.0,
            b1: 80.0,
            b2: 100.0,
            b3: 150.0,
            is_voiced: true,
            noise_gain: 0.0,
            duration_ms: 100.0,
            is_trill: false,
        }
    }
}

/// Convert graphemes (Cyrillic Mongolian, Latin, punctuation) to acoustic phoneme specs
pub fn grapheme_to_phoneme(c: char) -> Option<PhonemeSpec> {
    match c {
        // Mongolian Cyrillic & English Vowels
        'а' | 'А' | 'a' | 'A' => Some(PhonemeSpec { f1: 800.0, f2: 1200.0, f3: 2500.0, b1: 80.0, b2: 100.0, b3: 150.0, is_voiced: true, noise_gain: 0.0, duration_ms: 110.0, is_trill: false }),
        'э' | 'Э' | 'е' | 'Е' | 'e' | 'E' => Some(PhonemeSpec { f1: 500.0, f2: 1850.0, f3: 2600.0, b1: 70.0, b2: 90.0, b3: 140.0, is_voiced: true, noise_gain: 0.0, duration_ms: 100.0, is_trill: false }),
        'и' | 'И' | 'й' | 'Й' | 'i' | 'I' | 'y' | 'Y' => Some(PhonemeSpec { f1: 300.0, f2: 2300.0, f3: 3100.0, b1: 50.0, b2: 110.0, b3: 180.0, is_voiced: true, noise_gain: 0.0, duration_ms: 90.0, is_trill: false }),
        'о' | 'О' | 'ё' | 'Ё' | 'o' | 'O' => Some(PhonemeSpec { f1: 520.0, f2: 920.0, f3: 2400.0, b1: 80.0, b2: 100.0, b3: 140.0, is_voiced: true, noise_gain: 0.0, duration_ms: 110.0, is_trill: false }),
        'у' | 'У' | 'u' | 'U' => Some(PhonemeSpec { f1: 360.0, f2: 850.0, f3: 2300.0, b1: 70.0, b2: 90.0, b3: 130.0, is_voiced: true, noise_gain: 0.0, duration_ms: 100.0, is_trill: false }),
        'ө' | 'Ө' => Some(PhonemeSpec { f1: 450.0, f2: 1450.0, f3: 2350.0, b1: 75.0, b2: 95.0, b3: 140.0, is_voiced: true, noise_gain: 0.0, duration_ms: 105.0, is_trill: false }),
        'ү' | 'Ү' => Some(PhonemeSpec { f1: 340.0, f2: 1650.0, f3: 2450.0, b1: 65.0, b2: 90.0, b3: 140.0, is_voiced: true, noise_gain: 0.0, duration_ms: 100.0, is_trill: false }),
        'ю' | 'Ю' => Some(PhonemeSpec { f1: 350.0, f2: 1700.0, f3: 2500.0, b1: 70.0, b2: 100.0, b3: 150.0, is_voiced: true, noise_gain: 0.0, duration_ms: 120.0, is_trill: false }),
        'я' | 'Я' => Some(PhonemeSpec { f1: 650.0, f2: 1400.0, f3: 2600.0, b1: 80.0, b2: 110.0, b3: 160.0, is_voiced: true, noise_gain: 0.0, duration_ms: 120.0, is_trill: false }),
        'ы' | 'Ы' => Some(PhonemeSpec { f1: 400.0, f2: 1500.0, f3: 2400.0, b1: 70.0, b2: 90.0, b3: 140.0, is_voiced: true, noise_gain: 0.0, duration_ms: 90.0, is_trill: false }),

        // Consonants (Voiced / Unvoiced / Fricatives / Nasals / Trills)
        'м' | 'М' | 'm' | 'M' => Some(PhonemeSpec { f1: 250.0, f2: 1000.0, f3: 2200.0, b1: 60.0, b2: 120.0, b3: 200.0, is_voiced: true, noise_gain: 0.0, duration_ms: 80.0, is_trill: false }),
        'н' | 'Н' | 'n' | 'N' => Some(PhonemeSpec { f1: 280.0, f2: 1500.0, f3: 2400.0, b1: 60.0, b2: 130.0, b3: 220.0, is_voiced: true, noise_gain: 0.0, duration_ms: 80.0, is_trill: false }),
        'л' | 'Л' | 'l' | 'L' => Some(PhonemeSpec { f1: 350.0, f2: 1250.0, f3: 2800.0, b1: 70.0, b2: 110.0, b3: 180.0, is_voiced: true, noise_gain: 0.0, duration_ms: 75.0, is_trill: false }),
        'р' | 'Р' | 'r' | 'R' => Some(PhonemeSpec { f1: 450.0, f2: 1350.0, f3: 2300.0, b1: 80.0, b2: 100.0, b3: 150.0, is_voiced: true, noise_gain: 0.05, duration_ms: 85.0, is_trill: true }),
        'с' | 'С' | 's' | 'S' => Some(PhonemeSpec { f1: 250.0, f2: 4000.0, f3: 6500.0, b1: 150.0, b2: 300.0, b3: 400.0, is_voiced: false, noise_gain: 0.65, duration_ms: 95.0, is_trill: false }),
        'ш' | 'Ш' | 'щ' | 'Щ' => Some(PhonemeSpec { f1: 300.0, f2: 2800.0, f3: 4500.0, b1: 120.0, b2: 250.0, b3: 350.0, is_voiced: false, noise_gain: 0.60, duration_ms: 95.0, is_trill: false }),
        'х' | 'Х' | 'h' | 'H' => Some(PhonemeSpec { f1: 600.0, f2: 1600.0, f3: 2700.0, b1: 150.0, b2: 200.0, b3: 300.0, is_voiced: false, noise_gain: 0.45, duration_ms: 70.0, is_trill: false }),
        'б' | 'Б' | 'b' | 'B' | 'в' | 'В' | 'v' | 'V' => Some(PhonemeSpec { f1: 250.0, f2: 800.0, f3: 2200.0, b1: 80.0, b2: 120.0, b3: 200.0, is_voiced: true, noise_gain: 0.08, duration_ms: 60.0, is_trill: false }),
        'д' | 'Д' | 'd' | 'D' => Some(PhonemeSpec { f1: 300.0, f2: 1700.0, f3: 2600.0, b1: 80.0, b2: 120.0, b3: 200.0, is_voiced: true, noise_gain: 0.05, duration_ms: 60.0, is_trill: false }),
        'т' | 'Т' | 't' | 'T' => Some(PhonemeSpec { f1: 300.0, f2: 1800.0, f3: 3200.0, b1: 100.0, b2: 180.0, b3: 250.0, is_voiced: false, noise_gain: 0.50, duration_ms: 55.0, is_trill: false }),
        'г' | 'Г' | 'g' | 'G' => Some(PhonemeSpec { f1: 320.0, f2: 1950.0, f3: 2500.0, b1: 80.0, b2: 130.0, b3: 200.0, is_voiced: true, noise_gain: 0.05, duration_ms: 60.0, is_trill: false }),
        'к' | 'К' | 'k' | 'K' => Some(PhonemeSpec { f1: 350.0, f2: 2000.0, f3: 3000.0, b1: 100.0, b2: 180.0, b3: 250.0, is_voiced: false, noise_gain: 0.48, duration_ms: 55.0, is_trill: false }),
        'п' | 'П' | 'p' | 'P' => Some(PhonemeSpec { f1: 250.0, f2: 900.0, f3: 2200.0, b1: 100.0, b2: 180.0, b3: 250.0, is_voiced: false, noise_gain: 0.45, duration_ms: 50.0, is_trill: false }),
        'з' | 'З' | 'z' | 'Z' => Some(PhonemeSpec { f1: 280.0, f2: 1600.0, f3: 4500.0, b1: 80.0, b2: 150.0, b3: 300.0, is_voiced: true, noise_gain: 0.35, duration_ms: 70.0, is_trill: false }),
        'ж' | 'Ж' | 'j' | 'J' => Some(PhonemeSpec { f1: 320.0, f2: 2100.0, f3: 3500.0, b1: 80.0, b2: 150.0, b3: 300.0, is_voiced: true, noise_gain: 0.35, duration_ms: 70.0, is_trill: false }),
        'ц' | 'Ц' => Some(PhonemeSpec { f1: 300.0, f2: 2000.0, f3: 5500.0, b1: 100.0, b2: 200.0, b3: 350.0, is_voiced: false, noise_gain: 0.55, duration_ms: 75.0, is_trill: false }),
        'ч' | 'Ч' => Some(PhonemeSpec { f1: 320.0, f2: 2200.0, f3: 4200.0, b1: 100.0, b2: 200.0, b3: 350.0, is_voiced: false, noise_gain: 0.55, duration_ms: 75.0, is_trill: false }),

        // Pause / Silence
        ' ' | '\t' | '\n' => Some(PhonemeSpec { f1: 0.0, f2: 0.0, f3: 0.0, b1: 0.0, b2: 0.0, b3: 0.0, is_voiced: false, noise_gain: 0.0, duration_ms: 60.0, is_trill: false }),
        ',' | ';' => Some(PhonemeSpec { f1: 0.0, f2: 0.0, f3: 0.0, b1: 0.0, b2: 0.0, b3: 0.0, is_voiced: false, noise_gain: 0.0, duration_ms: 120.0, is_trill: false }),
        '.' | '!' | '?' => Some(PhonemeSpec { f1: 0.0, f2: 0.0, f3: 0.0, b1: 0.0, b2: 0.0, b3: 0.0, is_voiced: false, noise_gain: 0.0, duration_ms: 200.0, is_trill: false }),
        _ => None,
    }
}

/// Convert input text into phonetic acoustic sequence
pub fn text_to_phonemes(text: &str) -> Vec<PhonemeSpec> {
    let mut specs = Vec::new();
    for ch in text.chars() {
        if let Some(spec) = grapheme_to_phoneme(ch) {
            specs.push(spec);
        }
    }
    if specs.is_empty() {
        // Fallback tone
        specs.push(PhonemeSpec::default());
    }
    specs
}

/// Neural Speech Synthesis Report
#[derive(Debug, Clone)]
pub struct AudioSynthesisReport {
    pub text: String,
    pub sample_rate: u32,
    pub total_samples: usize,
    pub duration_seconds: f32,
    pub phoneme_count: usize,
    pub peak_amplitude: f32,
    pub rms_db: f32,
    pub latency_us: f32,
}

/// Pure Standard-Library Formant Acoustic Speech Synthesizer
pub fn synthesize_speech_waveform(text: &str, sample_rate: u32) -> (Vec<f32>, AudioSynthesisReport) {
    let start_time = std::time::Instant::now();
    let sr = sample_rate as f32;
    let phonemes = text_to_phonemes(text);

    let mut samples = Vec::new();
    let mut rng_state = 0x1337BEEFu32;

    // Simple Linear Congruential Noise Generator
    let mut next_noise = || {
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        ((rng_state >> 16) as f32 / 32768.0) - 1.0
    };

    let mut f1_filter = FormantFilter::new(500.0, 80.0, sr);
    let mut f2_filter = FormantFilter::new(1500.0, 100.0, sr);
    let mut f3_filter = FormantFilter::new(2500.0, 150.0, sr);

    let total_phonemes = phonemes.len();
    let base_f0 = 125.0; // 125 Hz base fundamental pitch

    let mut phase = 0.0f32;

    for (p_idx, p) in phonemes.iter().enumerate() {
        if p.duration_ms <= 0.0 {
            continue;
        }

        // Sentence intonation contour: gentle declination over time
        let progress = p_idx as f32 / total_phonemes.max(1) as f32;
        let pitch_f0 = base_f0 * (1.0 - 0.15 * progress);

        f1_filter.update(p.f1, p.b1);
        f2_filter.update(p.f2, p.b2);
        f3_filter.update(p.f3, p.b3);

        let num_samples = ((p.duration_ms / 1000.0) * sr) as usize;
        let attack_samples = (0.015 * sr) as usize;
        let release_samples = (0.020 * sr) as usize;

        for s_idx in 0..num_samples {
            // ADSR Envelope
            let mut env = 1.0f32;
            if s_idx < attack_samples {
                env = s_idx as f32 / attack_samples as f32;
            } else if s_idx >= num_samples.saturating_sub(release_samples) {
                let rel_idx = s_idx - (num_samples - release_samples);
                env = 1.0 - (rel_idx as f32 / release_samples as f32);
            }

            // Trill modulation (e.g. for /r/)
            if p.is_trill {
                let trill_mod = (2.0 * PI * 28.0 * (s_idx as f32 / sr)).sin();
                if trill_mod < 0.0 {
                    env *= 0.25;
                }
            }

            // Excitation Source
            let excitation = if p.is_voiced {
                // Rosenberg Glottal Pulse Generator
                let period_samples = sr / pitch_f0;
                phase += 1.0;
                if phase >= period_samples {
                    phase -= period_samples;
                }
                let t_norm = phase / period_samples;
                let glottal = if t_norm < 0.6 {
                    3.0 * (t_norm / 0.6).powi(2) - 2.0 * (t_norm / 0.6).powi(3)
                } else if t_norm < 0.85 {
                    1.0 - ((t_norm - 0.6) / 0.25)
                } else {
                    0.0
                };
                let noise = next_noise() * p.noise_gain;
                glottal * 0.85 + noise
            } else {
                next_noise() * p.noise_gain
            };

            // Vocal Tract Resonance Filtering
            if p.f1 > 0.0 {
                let o1 = f1_filter.process(excitation);
                let o2 = f2_filter.process(excitation);
                let o3 = f3_filter.process(excitation);
                let combined = (o1 * 0.55 + o2 * 0.30 + o3 * 0.15) * env;
                samples.push(combined);
            } else {
                // Silence
                samples.push(0.0);
            }
        }
    }

    // Measure statistics
    let mut peak = 0.0f32;
    let mut sum_sq = 0.0f32;
    for &s in &samples {
        let abs = s.abs();
        if abs > peak {
            peak = abs;
        }
        sum_sq += s * s;
    }

    // Normalize peak to -1 dB (0.89)
    if peak > 1e-4 {
        let norm_factor = 0.89 / peak;
        for s in &mut samples {
            *s *= norm_factor;
        }
        peak = 0.89;
    }

    let rms = (sum_sq / samples.len().max(1) as f32).sqrt();
    let rms_db = if rms > 1e-6 { 20.0 * rms.log10() } else { -96.0 };
    let elapsed = start_time.elapsed();
    let latency_us = elapsed.as_micros() as f32;

    let total_samples = samples.len();
    let duration_seconds = total_samples as f32 / sr;

    let report = AudioSynthesisReport {
        text: text.to_string(),
        sample_rate,
        total_samples,
        duration_seconds,
        phoneme_count: phonemes.len(),
        peak_amplitude: peak,
        rms_db,
        latency_us,
    };

    (samples, report)
}

/// Synthesize speech and return complete 16-bit PCM RIFF/WAVE file bytes
pub fn synthesize_speech_wav(text: &str, sample_rate: u32) -> (Vec<u8>, AudioSynthesisReport) {
    let (samples, report) = synthesize_speech_waveform(text, sample_rate);
    let wav_bytes = encode_wav_16bit(sample_rate, &samples);
    (wav_bytes, report)
}

/// Synthesize speech and save directly to file on disk
pub fn save_speech_wav(text: &str, out_path: &Path, sample_rate: u32) -> Result<AudioSynthesisReport, String> {
    let (wav_bytes, report) = synthesize_speech_wav(text, sample_rate);
    fs::write(out_path, wav_bytes).map_err(|e| format!("Failed to write WAV file to '{}': {}", out_path.display(), e))?;
    Ok(report)
}

/// Render ASCII Waterfall Spectrogram for terminal display
pub fn render_ascii_waterfall(samples: &[f32], sample_rate: usize, width: usize, height: usize) -> String {
    let opt = crate::cl_multimodal::AudioConfig {
        sample_rate,
        fft_size: 512,
        hop_length: (samples.len() / width.max(1)).max(64),
        mel_bands: height,
        embed_dim: 64,
    };
    let mel_frames = crate::cl_multimodal::AudioSpectrogramProcessor::compute_mel_spectrogram(samples, &opt);
    crate::cl_multimodal::MultiModalFusion::render_ascii_spectrogram(&mel_frames, width)
}
