// ============================================================================
// CRON Live Microphone Audio Capture & Voice Loop Engine (cl_mic.rs)
// Pure standard library Rust - Zero external crate bloat
//
// Windows: Native winmm.dll (mciSendStringA) hardware audio capture
// Linux: arecord (ALSA standard)
// macOS: sox / rec fallback
// ============================================================================

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[cfg(target_os = "windows")]
#[link(name = "winmm")]
extern "system" {
    fn mciSendStringA(
        lpstrCommand: *const u8,
        lpstrReturnString: *mut u8,
        uReturnLength: u32,
        hwndCallback: usize,
    ) -> u32;
}

/// Send a command to the Windows Media Control Interface (MCI)
#[cfg(target_os = "windows")]
fn send_mci_cmd(cmd: &str) -> Result<(), String> {
    let mut null_terminated = cmd.to_string();
    null_terminated.push('\0');
    let res = unsafe {
        mciSendStringA(
            null_terminated.as_ptr(),
            std::ptr::null_mut(),
            0,
            0,
        )
    };
    if res == 0 {
        Ok(())
    } else {
        Err(format!("MCI command failed with error code: {}", res))
    }
}

/// Capture live audio from the system microphone into a WAV file
pub fn record_mic_to_file(
    out_path: &Path,
    duration_seconds: f32,
    sample_rate: u32,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 1. Close any previous recording device
        let _ = send_mci_cmd("close recsound");

        // 2. Open new waveaudio session
        send_mci_cmd("open new type waveaudio alias recsound")?;

        // 3. Set standard 16-bit Mono PCM at desired sample rate
        let set_cmd = format!(
            "set recsound time format ms bitspersample 16 channels 1 samplespersec {}",
            sample_rate
        );
        let _ = send_mci_cmd(&set_cmd);

        // 4. Start recording
        send_mci_cmd("record recsound")?;

        // 5. Sleep for the recording duration
        let sleep_ms = (duration_seconds * 1000.0) as u64;
        std::thread::sleep(Duration::from_millis(sleep_ms));

        // 6. Stop and save
        send_mci_cmd("stop recsound")?;

        let abs_path = if out_path.is_absolute() {
            out_path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(out_path)
        };

        let save_cmd = format!("save recsound \"{}\"", abs_path.display());
        send_mci_cmd(&save_cmd)?;

        // 7. Close session
        send_mci_cmd("close recsound")?;

        if !abs_path.exists() {
            return Err(format!("Failed to create microphone recording at '{}'", abs_path.display()));
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        let dur_str = format!("{:.1}", duration_seconds);
        let sr_str = format!("{}", sample_rate);
        let status = std::process::Command::new("arecord")
            .args(["-q", "-f", "S16_LE", "-r", &sr_str, "-c", "1", "-d", &dur_str, out_path.to_str().unwrap()])
            .status()
            .map_err(|e| format!("arecord failed to launch: {}", e))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("arecord exited with non-zero status: {:?}", status.code()))
        }
    }

    #[cfg(target_os = "macos")]
    {
        let dur_str = format!("{:.1}", duration_seconds);
        let sr_str = format!("{}", sample_rate);
        let status = std::process::Command::new("rec")
            .args(["-q", "-r", &sr_str, "-c", "1", out_path.to_str().unwrap(), "trim", "0", &dur_str])
            .status()
            .map_err(|e| format!("macOS rec/sox failed to launch: {}", e))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("rec exited with non-zero status: {:?}", status.code()))
        }
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "linux"), not(target_os = "macos")))]
    {
        Err("Live microphone capture is currently unsupported on this operating system".to_string())
    }
}

/// Capture live audio from the microphone and return raw PCM samples
pub fn record_mic_pcm(
    duration_seconds: f32,
    sample_rate: u32,
) -> Result<Vec<f32>, String> {
    let temp_wav = std::env::temp_dir().join(format!("cron_mic_rec_{}.wav", std::process::id()));
    record_mic_to_file(&temp_wav, duration_seconds, sample_rate)?;

    let bytes = fs::read(&temp_wav)
        .map_err(|e| format!("Failed to read captured audio from '{}': {}", temp_wav.display(), e))?;
    let _ = fs::remove_file(&temp_wav);

    let (_, pcm) = crate::cl_multimodal::MultiModalFusion::parse_wav(&bytes)?;
    Ok(pcm)
}

/// Continuous Hands-Free Live Voice Interaction Loop
pub struct LiveVoiceLoop {
    pub agent: crate::VoiceAgent,
    pub sample_rate: u32,
    pub is_running: bool,
}

impl Default for LiveVoiceLoop {
    fn default() -> Self {
        Self::new(16000)
    }
}

impl LiveVoiceLoop {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            agent: crate::VoiceAgent::new(sample_rate),
            sample_rate,
            is_running: false,
        }
    }

    /// Run a single live microphone turn: records, transcribes, thinks, speaks
    pub fn run_single_turn(&mut self, record_seconds: f32) -> Result<crate::VoiceTurnResult, String> {
        let temp_wav = std::env::temp_dir().join(format!("cron_mic_turn_{}.wav", std::process::id()));
        record_mic_to_file(&temp_wav, record_seconds, self.sample_rate)?;

        let result = self.agent.interact_wav_file(&temp_wav)?;
        let _ = fs::remove_file(&temp_wav);

        Ok(result)
    }
}
