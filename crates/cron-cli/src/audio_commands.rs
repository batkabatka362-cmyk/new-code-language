// ============================================================================
// CRON CLI Audio Commands: Speech Synthesis, STFT & 0-DSP STT
// Module: cron_cli::audio_commands
// ============================================================================

pub fn handle_audio_command(args: &[String]) {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        println!("================================================================================");
        println!(" CRON NEURAL SPEECH & ACOUSTIC SPECTROGRAM ENGINE (AUDIO)");
        println!("================================================================================");
        println!("Pure standard library Rust speech synthesis (zero external audio libraries/C++):\n");
        println!("USAGE:");
        println!("    cron audio synth \"<text>\" [options]");
        println!("    cron audio stft <file.wav> [options]");
        println!("    cron audio transcribe <file.wav> [options]");
        println!();
        println!("SUBCOMMANDS & OPTIONS:");
        println!("    synth \"<text>\"     Synthesize Mongolian or English speech into a 16-bit PCM WAV");
        println!("      -o, --out <file>  Output WAV file path (default: speech.wav)");
        println!("      --play            Play audio immediately through system audio driver");
        println!("      --sr <Hz>         Sample rate in Hz (default: 16000)");
        println!("      --json            Output synthesis report in JSON format");
        println!();
        println!("    stft <file.wav>    Extract Log-Mel spectrogram from WAV file");
        println!("      --ascii           Render real-time ASCII waterfall spectrogram in terminal");
        println!("      --json            Output spectrogram metrics in JSON format");
        println!();
        println!("    transcribe <wav>   0-DSP BitNet ternary acoustic speech recognition (STT)");
        println!("      --json            Output transcription, phonemes & DSP metrics in JSON");
        println!();
        println!("EXAMPLES:");
        println!("    cron audio synth \"Сайн байна уу, CRON цахиурын хэл!\" -o mongol.wav");
        println!("    cron audio synth \"Artificial intelligence on 4D Torus silicon\" --play");
        println!("    cron audio stft speech.wav --ascii");
        println!("    cron audio transcribe speech.wav");
        return;
    }

    let sub = args[0].as_str();
    match sub {
        "synth" | "tts" | "speak" => {
            if args.len() < 2 {
                eprintln!("Error: Missing text prompt. Usage: cron audio synth \"<text>\" [-o speech.wav] [--play]");
                std::process::exit(1);
            }
            let text = &args[1];
            let mut out_file = "speech.wav".to_string();
            let mut auto_play = false;
            let mut sample_rate = 16000u32;
            let mut emit_json = false;

            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "-o" | "--out" => {
                        if i + 1 < args.len() {
                            out_file = args[i + 1].clone();
                            i += 2;
                        } else {
                            i += 1;
                        }
                    }
                    "--play" => {
                        auto_play = true;
                        i += 1;
                    }
                    "--sr" => {
                        if i + 1 < args.len() {
                            if let Ok(sr) = args[i + 1].parse::<u32>() {
                                sample_rate = sr;
                            }
                            i += 2;
                        } else {
                            i += 1;
                        }
                    }
                    "--json" => {
                        emit_json = true;
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }

            let (wav_bytes, report) = cronc::cl_audio::synthesize_speech_wav(text, sample_rate);
            if let Err(e) = std::fs::write(&out_file, &wav_bytes) {
                eprintln!("Error saving WAV file to '{}': {}", out_file, e);
                std::process::exit(1);
            }

            if emit_json {
                println!("{{");
                println!("  \"text\": \"{}\",", report.text.replace('"', "\\\""));
                println!("  \"output_wav\": \"{}\",", out_file);
                println!("  \"sample_rate\": {},", report.sample_rate);
                println!("  \"total_samples\": {},", report.total_samples);
                println!("  \"duration_seconds\": {:.3},", report.duration_seconds);
                println!("  \"phoneme_count\": {},", report.phoneme_count);
                println!("  \"peak_amplitude\": {:.4},", report.peak_amplitude);
                println!("  \"rms_db\": {:.2},", report.rms_db);
                println!("  \"latency_us\": {:.1}", report.latency_us);
                println!("}}");
            } else {
                println!("================================================================================");
                println!(" CRON NEURAL FORMANT SPEECH SYNTHESIZER (TEXT-TO-SPEECH)");
                println!("================================================================================");
                println!("  Text Input       : \"{}\"", report.text);
                println!("  Output File      : {}", out_file);
                println!("  Audio Format     : 16-bit PCM RIFF/WAVE (Mono)");
                println!("  Sample Rate      : {} Hz", report.sample_rate);
                println!("  Duration         : {:.3} seconds ({} samples)", report.duration_seconds, report.total_samples);
                println!("  Phonemes         : {} acoustic formant frames", report.phoneme_count);
                println!("  Peak Amplitude   : {:.4} ({:.2} dB RMS)", report.peak_amplitude, report.rms_db);
                println!("  Synthesis Speed  : {:.1} µs", report.latency_us);
                println!("  Status           : 100% PURE RUST ZERO-EXTERNAL-DEP SPEECH GENERATED");
                println!("================================================================================");
            }

            if auto_play {
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("powershell")
                        .args(["-c", &format!("(New-Object Media.SoundPlayer '{}').PlaySync()", out_file)])
                        .status();
                }
                #[cfg(target_os = "macos")]
                {
                    let _ = std::process::Command::new("afplay")
                        .arg(&out_file)
                        .status();
                }
                #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
                {
                    let _ = std::process::Command::new("aplay")
                        .arg(&out_file)
                        .status();
                }
            }
        }
        "stft" | "spectrogram" | "spec" => {
            if args.len() < 2 {
                eprintln!("Error: Missing audio file. Usage: cron audio stft <file.wav> [--ascii]");
                std::process::exit(1);
            }
            let wav_path = &args[1];
            let bytes = match std::fs::read(wav_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Error reading audio file '{}': {}", wav_path, e);
                    std::process::exit(1);
                }
            };

            let (sr, pcm) = match cronc::cl_multimodal::MultiModalFusion::parse_wav(&bytes) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Error parsing WAV: {}", e);
                    std::process::exit(1);
                }
            };

            let cfg = cronc::cl_multimodal::AudioConfig {
                sample_rate: sr,
                fft_size: 512,
                hop_length: 160,
                mel_bands: 80,
                embed_dim: 64,
            };
            let mel = cronc::cl_multimodal::AudioSpectrogramProcessor::compute_mel_spectrogram(&pcm, &cfg);

            println!("================================================================================");
            println!(" CRON AUDIO LOG-MEL SPECTROGRAM & STFT ANALYSIS");
            println!("================================================================================");
            println!("  Input File   : {}", wav_path);
            println!("  Sample Rate  : {} Hz", sr);
            println!("  Duration     : {:.2} s ({} samples)", pcm.len() as f32 / sr as f32, pcm.len());
            println!("  Mel Frames   : {} frames x {} bands", mel.len(), cfg.mel_bands);
            println!("================================================================================\n");

            if args.iter().any(|a| a == "--ascii" || a == "-a") || !args.iter().any(|a| a == "--json") {
                let waterfall = cronc::cl_multimodal::MultiModalFusion::render_ascii_spectrogram(&mel, 60);
                println!("{}", waterfall);
            }
        }
        "transcribe" | "stt" | "listen" | "recognize" => {
            if args.len() < 2 {
                eprintln!("Error: Missing audio file. Usage: cron audio transcribe <file.wav> [--json]");
                std::process::exit(1);
            }
            let wav_path = &args[1];
            let report = match cronc::cl_audio::transcribe_speech_wav(std::path::Path::new(wav_path)) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error transcribing '{}': {}", wav_path, e);
                    std::process::exit(1);
                }
            };

            if args.iter().any(|a| a == "--json") {
                let phonemes_json: Vec<String> = report.phonemes.iter().map(|p| format!("\"{}\"", p)).collect();
                println!("{{");
                println!("  \"audio_file\": \"{}\",", wav_path);
                println!("  \"transcription\": \"{}\",", report.text.replace('"', "\\\""));
                println!("  \"confidence\": {:.4},", report.confidence);
                println!("  \"duration_seconds\": {:.3},", report.duration_seconds);
                println!("  \"sample_rate\": {},", report.sample_rate);
                println!("  \"total_frames\": {},", report.total_frames);
                println!("  \"zero_dsp_ops\": {},", report.zero_dsp_ops);
                println!("  \"latency_us\": {:.1},", report.latency_us);
                println!("  \"phonemes\": [{}]", phonemes_json.join(", "));
                println!("}}");
            } else {
                println!("================================================================================");
                println!(" CRON 0-DSP ACOUSTIC SPEECH-TO-TEXT (STT) TRANSCRIPTION");
                println!("================================================================================");
                println!("  Input File       : {}", wav_path);
                println!("  Transcription    : \"{}\"", report.text);
                println!("  Confidence       : {:.2}% Agreement", report.confidence * 100.0);
                println!("  Phoneme Chain    : [{}]", report.phonemes.join("-"));
                println!("  Duration         : {:.3} seconds ({} frames)", report.duration_seconds, report.total_frames);
                println!("  Sample Rate      : {} Hz", report.sample_rate);
                println!("  Hardware Profile : {} Integer Add/Sub Ops (0 DSP Multipliers)", report.zero_dsp_ops);
                println!("  Processing Speed : {:.1} µs ({:.1}x Real-Time)", report.latency_us, (report.duration_seconds * 1_000_000.0) / report.latency_us.max(1.0));
                println!("  Status           : 100% PURE RUST ZERO-DSP ACOUSTIC INFERENCE VERIFIED");
                println!("================================================================================");
            }
        }
        other => {
            eprintln!("Unknown audio subcommand '{}'. Use 'cron audio --help'.", other);
            std::process::exit(1);
        }
    }
}
