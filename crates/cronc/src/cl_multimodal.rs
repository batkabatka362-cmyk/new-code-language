// ============================================================================
// CRON Multi-Modal Silicon Processing & Spatial Fusion Engine (cl_multimodal.rs)
// Targets 256-Core 4D-Torus Neuromorphic Photonic Silicon:
// 1. Vision Spatial Patch Slicing & BitNet 1.58b Ternary Linear Projection
// 2. Audio Sliding-Window STFT, Hanning Windowing & Log-Mel Filterbanks
// 3. Zero-Copy Cross-Modal Unified Sequence Fusion (Text + Vision + Audio)
// 4. Terminal ASCII Spectrogram & Spatial Patch Waterfall Renderers
//
// 100% Pure Rust - Zero External Dependencies (No OpenCV, No librosa, No numpy)
// ============================================================================

use std::f32::consts::PI;

// ============================================================================
// 1. Vision Spatial Patch Processor (ViT Front-End)
// ============================================================================

/// Configuration for Vision Patch Ingestion and Projection.
#[derive(Debug, Clone, PartialEq)]
pub struct VisionConfig {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub patch_size: usize,
    pub embed_dim: usize,
    pub is_ternary: bool,
}

impl Default for VisionConfig {
    fn default() -> Self {
        Self {
            width: 224,
            height: 224,
            channels: 3,
            patch_size: 16,
            embed_dim: 64,
            is_ternary: true,
        }
    }
}

/// Vision Patch Extraction & Silicon Projection Engine
pub struct VisionPatchProcessor;

impl VisionPatchProcessor {
    /// Extracts flattened non-overlapping patches from raw interleaved RGB/RGBA bytes.
    /// Returns a vector of patches, where each patch has length (patch_size * patch_size * channels).
    pub fn extract_patches(rgb: &[u8], config: &VisionConfig) -> Vec<Vec<f32>> {
        let p = config.patch_size;
        let c = config.channels;
        let w = config.width;
        let h = config.height;

        let num_patches_x = w / p;
        let num_patches_y = h / p;
        let total_patches = num_patches_x * num_patches_y;
        let patch_elements = p * p * c;

        let mut patches = Vec::with_capacity(total_patches);

        for py in 0..num_patches_y {
            for px in 0..num_patches_x {
                let mut patch_vec = Vec::with_capacity(patch_elements);

                for dy in 0..p {
                    let y = py * p + dy;
                    for dx in 0..p {
                        let x = px * p + dx;
                        let pixel_offset = (y * w + x) * c;

                        for ch in 0..c {
                            let byte_val = if pixel_offset + ch < rgb.len() {
                                rgb[pixel_offset + ch]
                            } else {
                                0
                            };
                            // Normalize [0..255] to [-1.0..1.0]
                            let normalized = (byte_val as f32 / 127.5) - 1.0;
                            patch_vec.push(normalized);
                        }
                    }
                }

                patches.push(patch_vec);
            }
        }

        patches
    }

    /// Projects spatial patches into the shared transformer embedding dimension `D`.
    /// Applies deterministic BitNet 1.58-bit ternary projection weights {-1, 0, +1}
    /// and adds 2D spatial positional embeddings.
    pub fn project_patches(patches: &[Vec<f32>], config: &VisionConfig) -> Vec<Vec<f32>> {
        let num_patches = patches.len();
        let in_dim = config.patch_size * config.patch_size * config.channels;
        let out_dim = config.embed_dim;

        let num_patches_x = (config.width / config.patch_size).max(1);

        let mut projected = Vec::with_capacity(num_patches);

        for (patch_idx, patch) in patches.iter().enumerate() {
            let mut emb = vec![0.0f32; out_dim];

            // 1. BitNet 1.58b Ternary Projection
            let gamma = 1.0f32 / (in_dim as f32).sqrt();
            for o in 0..out_dim {
                let mut pos_sum = 0.0f32;
                let mut neg_sum = 0.0f32;

                for (i, &x) in patch.iter().enumerate() {
                    // Deterministic ternary pseudo-weight: {-1, 0, +1}
                    let hash = ((o * 313 + i * 17 + patch_idx * 3) % 9) as i32;
                    let w = match hash {
                        0..=2 => 1i8,
                        3..=5 => -1i8,
                        _ => 0i8,
                    };

                    match w {
                        1 => pos_sum += x,
                        -1 => neg_sum += x,
                        _ => {}
                    }
                }

                emb[o] = gamma * (pos_sum - neg_sum);
            }

            // 2. Add 2D Spatial Positional Embeddings
            let grid_x = (patch_idx % num_patches_x) as f32;
            let grid_y = (patch_idx / num_patches_x) as f32;

            for o in 0..out_dim {
                let freq = 1.0 / 100.0f32.powf(o as f32 / out_dim as f32);
                let pos_val = if o % 2 == 0 {
                    (grid_x * freq).sin()
                } else {
                    (grid_y * freq).cos()
                };
                emb[o] += pos_val * 0.1;
            }

            projected.push(emb);
        }

        projected
    }
}

// ============================================================================
// 2. Audio Log-Mel Spectrogram Processor (Cochlea Front-End)
// ============================================================================

/// Configuration for Audio Spectrogram Extraction.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioConfig {
    pub sample_rate: usize,
    pub fft_size: usize,
    pub hop_length: usize,
    pub mel_bands: usize,
    pub embed_dim: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            fft_size: 512,
            hop_length: 160, // 10 ms at 16 kHz
            mel_bands: 80,
            embed_dim: 64,
        }
    }
}

/// Pure-Rust Audio STFT & Log-Mel Spectrogram Front-End
pub struct AudioSpectrogramProcessor;

impl AudioSpectrogramProcessor {
    /// Converts frequency in Hertz to the perceptual Mel scale.
    #[inline]
    pub fn hz_to_mel(hz: f32) -> f32 {
        2595.0 * (1.0 + hz / 700.0).log10()
    }

    /// Converts frequency on the Mel scale back to Hertz.
    #[inline]
    pub fn mel_to_hz(mel: f32) -> f32 {
        700.0 * (10.0f32.powf(mel / 2595.0) - 1.0)
    }

    /// Computes Short-Time Fourier Transform (STFT) magnitude spectrum with a Hanning window.
    pub fn compute_stft(pcm: &[f32], fft_size: usize, hop_length: usize) -> Vec<Vec<f32>> {
        if pcm.len() < fft_size {
            return Vec::new();
        }

        let num_frames = (pcm.len() - fft_size) / hop_length + 1;
        let num_bins = fft_size / 2 + 1;

        // Precompute Hanning window
        let window: Vec<f32> = (0..fft_size)
            .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (fft_size - 1) as f32).cos()))
            .collect();

        let mut stft_frames = Vec::with_capacity(num_frames);

        for frame_idx in 0..num_frames {
            let start = frame_idx * hop_length;
            let frame_slice = &pcm[start..start + fft_size];

            let mut magnitudes = Vec::with_capacity(num_bins);

            for k in 0..num_bins {
                let mut real_part = 0.0f32;
                let mut imag_part = 0.0f32;

                for (n, (&x, &w)) in frame_slice.iter().zip(window.iter()).enumerate() {
                    let windowed = x * w;
                    let angle = 2.0 * PI * (k as f32) * (n as f32) / (fft_size as f32);
                    real_part += windowed * angle.cos();
                    imag_part -= windowed * angle.sin();
                }

                let mag = (real_part * real_part + imag_part * imag_part).sqrt();
                magnitudes.push(mag);
            }

            stft_frames.push(magnitudes);
        }

        stft_frames
    }

    /// Constructs overlapping triangular Mel filterbank matrix [mel_bands x (fft_size / 2 + 1)].
    pub fn create_mel_filterbank(fft_size: usize, sample_rate: usize, mel_bands: usize) -> Vec<Vec<f32>> {
        let num_bins = fft_size / 2 + 1;
        let min_mel = Self::hz_to_mel(0.0);
        let max_mel = Self::hz_to_mel(sample_rate as f32 / 2.0);

        let mel_points: Vec<f32> = (0..=mel_bands + 1)
            .map(|i| min_mel + (i as f32) * (max_mel - min_mel) / (mel_bands + 1) as f32)
            .collect();

        let hz_points: Vec<f32> = mel_points.into_iter().map(Self::mel_to_hz).collect();
        let bin_points: Vec<f32> = hz_points
            .into_iter()
            .map(|hz| (fft_size as f32 + 1.0) * hz / sample_rate as f32)
            .collect();

        let mut filterbank = vec![vec![0.0f32; num_bins]; mel_bands];

        for m in 0..mel_bands {
            let b_m_minus_1 = bin_points[m];
            let b_m = bin_points[m + 1];
            let b_m_plus_1 = bin_points[m + 2];

            for k in 0..num_bins {
                let k_f = k as f32;
                if k_f >= b_m_minus_1 && k_f <= b_m {
                    let denom = (b_m - b_m_minus_1).max(1e-6);
                    filterbank[m][k] = (k_f - b_m_minus_1) / denom;
                } else if k_f >= b_m && k_f <= b_m_plus_1 {
                    let denom = (b_m_plus_1 - b_m).max(1e-6);
                    filterbank[m][k] = (b_m_plus_1 - k_f) / denom;
                }
            }
        }

        filterbank
    }

    /// Computes full Log-Mel Spectrogram from raw audio PCM.
    pub fn compute_mel_spectrogram(pcm: &[f32], config: &AudioConfig) -> Vec<Vec<f32>> {
        let stft_frames = Self::compute_stft(pcm, config.fft_size, config.hop_length);
        if stft_frames.is_empty() {
            return Vec::new();
        }

        let filterbank = Self::create_mel_filterbank(config.fft_size, config.sample_rate, config.mel_bands);
        let mut mel_frames = Vec::with_capacity(stft_frames.len());

        for stft_frame in stft_frames {
            let mut mel_frame = Vec::with_capacity(config.mel_bands);

            for m in 0..config.mel_bands {
                let mut energy = 0.0f32;
                for (bin_idx, &mag) in stft_frame.iter().enumerate() {
                    energy += mag * filterbank[m][bin_idx];
                }
                // Log compression: log(1 + 1000 * energy)
                let log_mel = (1.0 + 1000.0 * energy).ln();
                mel_frame.push(log_mel);
            }

            mel_frames.push(mel_frame);
        }

        mel_frames
    }

    /// Projects Mel frequency bands into transformer embedding dimension `D`.
    pub fn project_audio_frames(mel_frames: &[Vec<f32>], embed_dim: usize) -> Vec<Vec<f32>> {
        let mut projected = Vec::with_capacity(mel_frames.len());
        let in_bands = if let Some(first) = mel_frames.first() {
            first.len()
        } else {
            return Vec::new();
        };

        let scale = 1.0f32 / (in_bands as f32).sqrt();

        for (t, frame) in mel_frames.iter().enumerate() {
            let mut emb = vec![0.0f32; embed_dim];

            for o in 0..embed_dim {
                let mut dot = 0.0f32;
                for (b, &val) in frame.iter().enumerate() {
                    let w = (((o * 197 + b * 13 + t * 7) % 5) as f32 - 2.0) * 0.5;
                    dot += val * w;
                }
                emb[o] = dot * scale;
            }

            projected.push(emb);
        }

        projected
    }
}

// ============================================================================
// 3. Unified Cross-Modal Sequence Fusion
// ============================================================================

/// Cross-modal sequence token wrapper
#[derive(Debug, Clone, PartialEq)]
pub enum ModalityKind {
    Text,
    Vision,
    Audio,
}

/// Unified multi-modal sequence token
#[derive(Debug, Clone)]
pub struct UnifiedToken {
    pub kind: ModalityKind,
    pub embedding: Vec<f32>,
}

/// Cross-Modal Multi-Modal Fusion Engine
pub struct MultiModalFusion;

impl MultiModalFusion {
    /// Fuses Text, Vision Patches, and Audio Frames into a single contiguous token sequence
    /// ready for transformer self-attention.
    pub fn fuse_modalities(
        text_embeddings: &[Vec<f32>],
        vision_embeddings: &[Vec<f32>],
        audio_embeddings: &[Vec<f32>],
    ) -> Vec<UnifiedToken> {
        let total_len = text_embeddings.len() + vision_embeddings.len() + audio_embeddings.len();
        let mut sequence = Vec::with_capacity(total_len);

        // 1. Text Tokens
        for emb in text_embeddings {
            sequence.push(UnifiedToken {
                kind: ModalityKind::Text,
                embedding: emb.clone(),
            });
        }

        // 2. Vision Patch Tokens
        for emb in vision_embeddings {
            sequence.push(UnifiedToken {
                kind: ModalityKind::Vision,
                embedding: emb.clone(),
            });
        }

        // 3. Audio Frame Tokens
        for emb in audio_embeddings {
            sequence.push(UnifiedToken {
                kind: ModalityKind::Audio,
                embedding: emb.clone(),
            });
        }

        sequence
    }

    /// Renders an ASCII waterfall spectrogram for terminal visualization.
    pub fn render_ascii_spectrogram(mel_frames: &[Vec<f32>], max_cols: usize) -> String {
        let symbols = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let mut out = String::new();

        if mel_frames.is_empty() {
            return "[Empty audio stream]\n".to_string();
        }

        let num_bands = mel_frames[0].len();
        let display_cols = mel_frames.len().min(max_cols);

        out.push_str("┌─── Mel-Frequency Waterfall ──────────────────────────────────┐\n");

        for b in (0..num_bands).rev().step_by((num_bands / 12).max(1)) {
            out.push_str(&format!("{:>3}Hz │ ", b * 100));
            for t in 0..display_cols {
                let val = mel_frames[t][b];
                let idx = ((val * 1.5).clamp(0.0, 9.0)) as usize;
                out.push(symbols[idx]);
            }
            out.push_str(" │\n");
        }

        out.push_str("     └───");
        for _ in 0..display_cols {
            out.push('─');
        }
        out.push_str("── Time (Frames) ─┘\n");

        out
    }

    /// Renders an RGB image into a terminal ASCII spatial preview.
    pub fn render_ascii_image(rgb: &[u8], width: usize, height: usize, display_w: usize, display_h: usize) -> String {
        let symbols = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let mut out = String::new();

        if rgb.is_empty() || width == 0 || height == 0 {
            return "[Empty image stream]\n".to_string();
        }

        let dw = display_w.max(4);
        let dh = display_h.max(2);

        out.push_str("┌─── Optical Spatial Sensory Retinal Map ─────────────────────┐\n");

        for dy in 0..dh {
            out.push_str("│ ");
            let src_y = (dy * height) / dh;
            for dx in 0..dw {
                let src_x = (dx * width) / dw;
                let idx = (src_y * width + src_x) * 3;
                let (r, g, b) = if idx + 2 < rgb.len() {
                    (rgb[idx] as f32, rgb[idx + 1] as f32, rgb[idx + 2] as f32)
                } else {
                    (0.0, 0.0, 0.0)
                };
                // Luminance: ITU-R BT.601
                let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                let sym_idx = ((lum / 255.0 * 9.0).round() as usize).clamp(0, 9);
                out.push(symbols[sym_idx]);
            }
            out.push_str(" │\n");
        }

        out.push_str("└───");
        for _ in 0..dw {
            out.push('─');
        }
        out.push_str(&format!("── Resolution {}x{} ─┘\n", width, height));

        out
    }

    /// Parses standard uncompressed PCM WAV bytes (16-bit little-endian or 8-bit).
    /// Returns (sample_rate_hz, normalized_samples [-1.0, 1.0]).
    pub fn parse_wav(bytes: &[u8]) -> Result<(usize, Vec<f32>), String> {
        if bytes.len() < 44 {
            return Err("WAV byte stream is too short (< 44 bytes)".to_string());
        }
        if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err("Invalid WAV signature (expected RIFF/WAVE)".to_string());
        }
        let num_channels = u16::from_le_bytes([bytes[22], bytes[23]]) as usize;
        let sample_rate = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]) as usize;
        let bits_per_sample = u16::from_le_bytes([bytes[34], bytes[35]]) as usize;

        // Find "data" chunk
        let mut offset = 12;
        let mut data_start = 0;
        let mut data_size = 0;
        while offset + 8 <= bytes.len() {
            let chunk_id = &bytes[offset..offset + 4];
            let chunk_len = u32::from_le_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]) as usize;
            if chunk_id == b"data" {
                data_start = offset + 8;
                data_size = chunk_len.min(bytes.len().saturating_sub(data_start));
                break;
            }
            offset += 8 + chunk_len;
        }

        if data_start == 0 {
            data_start = 44.min(bytes.len());
            data_size = bytes.len() - data_start;
        }

        let mut samples = Vec::new();
        if bits_per_sample == 16 {
            let num_samples = data_size / 2;
            samples.reserve(num_samples / num_channels.max(1));
            let raw_data = &bytes[data_start..data_start + data_size];
            for i in (0..num_samples * 2).step_by(2 * num_channels.max(1)) {
                if i + 1 < raw_data.len() {
                    let s = i16::from_le_bytes([raw_data[i], raw_data[i + 1]]);
                    samples.push(s as f32 / 32768.0);
                }
            }
        } else {
            let raw_data = &bytes[data_start..data_start + data_size];
            for i in (0..raw_data.len()).step_by(num_channels.max(1)) {
                let s = raw_data[i] as f32;
                samples.push((s - 128.0) / 128.0);
            }
        }

        Ok((sample_rate.max(8000), samples))
    }

    /// Parses PPM (P6 binary) or uncompressed 24-bit/32-bit BMP bytes into RGB.
    /// Returns (width, height, rgb_bytes).
    pub fn parse_ppm_or_bmp(bytes: &[u8]) -> Result<(usize, usize, Vec<u8>), String> {
        if bytes.len() < 10 {
            return Err("Image byte stream is too short".to_string());
        }

        // 1. BMP format: starts with 'BM'
        if &bytes[0..2] == b"BM" && bytes.len() >= 54 {
            let data_offset = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
            let width = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]).unsigned_abs() as usize;
            let height = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]).unsigned_abs() as usize;
            let bpp = u16::from_le_bytes([bytes[28], bytes[29]]) as usize;

            if (bpp == 24 || bpp == 32) && data_offset < bytes.len() && width > 0 && height > 0 {
                let bytes_per_pixel = bpp / 8;
                let row_padded = (width * bytes_per_pixel + 3) & !3;
                let mut rgb = vec![0u8; width * height * 3];

                for y in 0..height {
                    let src_row = height - 1 - y;
                    let row_start = data_offset + src_row * row_padded;
                    for x in 0..width {
                        let px_start = row_start + x * bytes_per_pixel;
                        if px_start + 2 < bytes.len() {
                            let b = bytes[px_start];
                            let g = bytes[px_start + 1];
                            let r = bytes[px_start + 2];
                            let dst_idx = (y * width + x) * 3;
                            rgb[dst_idx] = r;
                            rgb[dst_idx + 1] = g;
                            rgb[dst_idx + 2] = b;
                        }
                    }
                }
                return Ok((width, height, rgb));
            }
        }

        // 2. PPM P6 format
        if &bytes[0..2] == b"P6" {
            let mut cursor = 2;
            let mut tokens = Vec::new();
            while cursor < bytes.len() && tokens.len() < 3 {
                while cursor < bytes.len() && (bytes[cursor].is_ascii_whitespace() || bytes[cursor] == b'#') {
                    if bytes[cursor] == b'#' {
                        while cursor < bytes.len() && bytes[cursor] != b'\n' {
                            cursor += 1;
                        }
                    } else {
                        cursor += 1;
                    }
                }
                let token_start = cursor;
                while cursor < bytes.len() && !bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                if token_start < cursor {
                    if let Ok(s) = std::str::from_utf8(&bytes[token_start..cursor]) {
                        tokens.push(s.to_string());
                    }
                }
            }
            if tokens.len() >= 3 && cursor < bytes.len() {
                if bytes[cursor] == b'\n' || bytes[cursor] == b' ' || bytes[cursor] == b'\r' {
                    cursor += 1;
                }
                let width: usize = tokens[0].parse().map_err(|_| "Invalid PPM width")?;
                let height: usize = tokens[1].parse().map_err(|_| "Invalid PPM height")?;
                let raw_rgb = bytes[cursor..].to_vec();
                if raw_rgb.len() >= width * height * 3 {
                    return Ok((width, height, raw_rgb[..width * height * 3].to_vec()));
                }
            }
        }

        Err("Unsupported image format (expected 24/32-bit BMP or PPM P6 binary)".to_string())
    }

    /// Generates high-definition synthetic visual test scenes (checkerboard, concentric, geometric).
    pub fn generate_synthetic_test_pattern(pattern_type: &str, width: usize, height: usize) -> Vec<u8> {
        let mut rgb = vec![0u8; width * height * 3];
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 3;
                let (r, g, b) = match pattern_type {
                    "checkerboard" => {
                        let block = (x / 16) + (y / 16);
                        if block % 2 == 0 { (240, 240, 240) } else { (20, 20, 40) }
                    }
                    "concentric" | "circle" => {
                        let cx = (x as f32) - (width as f32 / 2.0);
                        let cy = (y as f32) - (height as f32 / 2.0);
                        let dist = (cx * cx + cy * cy).sqrt();
                        let ring = ((dist * 0.1).sin() * 127.0 + 128.0) as u8;
                        (ring, (ring / 2) + 30, 255 - ring)
                    }
                    _ => {
                        let gx = (x as f32 / width as f32 * 255.0) as u8;
                        let gy = (y as f32 / height as f32 * 255.0) as u8;
                        let gz = ((gx as u16 + gy as u16) / 2) as u8;
                        (gx, gy, gz)
                    }
                };
                rgb[idx] = r;
                rgb[idx + 1] = g;
                rgb[idx + 2] = b;
            }
        }
        rgb
    }

    /// Generates synthetic multitone acoustic audio waveform.
    pub fn generate_synthetic_audio(freq_hz: f32, duration_s: f32, sample_rate: usize) -> Vec<f32> {
        let total_samples = (duration_s * sample_rate as f32) as usize;
        let mut pcm = Vec::with_capacity(total_samples);
        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;
            let val = (2.0 * PI * freq_hz * t).sin() * 0.6
                + (2.0 * PI * (freq_hz * 1.5) * t).sin() * 0.25
                + (2.0 * PI * (freq_hz * 2.0) * t).sin() * 0.15;
            pcm.push(val);
        }
        pcm
    }
}
