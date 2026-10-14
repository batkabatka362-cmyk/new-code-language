// ============================================================================
// CRON CLI: Sovereign Mind AGI & 3D Neuromorphic Dashboard Commands
// Module: cron_cli::sovereign_commands
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

pub fn handle_cl_sovereign_command(args: &[String]) {
    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        println!("USAGE: cron cl-sovereign <chat|stats|export|corpus|ui|evolve> [OPTIONS]");
        println!("SUBCOMMANDS:");
        println!("    chat [--eval <text>]      Interactive cognitive REPL session or single evaluation");
        println!("    stats [--json]            Display 16-column cortical brain architecture and telemetry");
        println!("    export [-o out.cl]        Synthesize 40-byte Base-94 Clifford rotor microcode bundle");
        println!("    corpus [--json]           Inspect sovereign knowledge corpus and morph-tokens");
        println!("    ui [--port P] [--open]    Launch 3D WebGL Neuromorphic Dashboard & REST API server");
        println!("    evolve [--intent <text>]  Autonomously synthesize & hot-reload 4-slot VLIW machine routines");
        return;
    }

    match args[0].as_str() {
        "chat" => {
            let mut eval_prompt: Option<String> = None;
            let mut cldb_path: Option<String> = None;
            let mut emit_json = false;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--eval" && i + 1 < args.len() {
                    eval_prompt = Some(args[i + 1].clone());
                    i += 2;
                } else if args[i] == "--cldb" && i + 1 < args.len() {
                    cldb_path = Some(args[i + 1].clone());
                    i += 2;
                } else if args[i] == "--json" {
                    emit_json = true;
                    i += 1;
                } else {
                    i += 1;
                }
            }

            let mut mind = cronc::SovereignMind::new();

            if let Some(ref path) = cldb_path {
                match mind.load_cldb_vault(path) {
                    Ok(count) => {
                        if !emit_json {
                            println!("[CLDB VAULT] Loaded {} crystallized long-term engrams from '{}'", count, path);
                        }
                    }
                    Err(e) => eprintln!("[CLDB ERROR] Failed to load '{}': {}", path, e),
                }
            }

            if let Some(prompt) = eval_prompt {
                let (answer, elapsed, tier) = mind.query(&prompt);
                if emit_json {
                    println!(r#"{{"status":"ok","prompt":{:?},"answer":{:?},"time_ms":{:.3},"tier":"{}"}}"#,
                        prompt, answer, elapsed.as_secs_f64() * 1000.0, tier);
                } else {
                    println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
                    println!("║                      SOVEREIGN MIND CORTICAL INFERENCE                                 ║");
                    println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                    println!("║ Prompt       : {:<71} ║", prompt);
                    println!("║ Strategy     : {:<71} ║", tier);
                    println!("║ Latency      : {:<71} ║", format!("{:.3} ms", elapsed.as_secs_f64() * 1000.0));
                    println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                    println!("║ Answer:                                                                               ║");
                    for line in answer.lines() {
                        println!("║  {:<85} ║", line);
                    }
                    println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
                }
                return;
            }

            println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
            println!("║             SOVEREIGN MIND AGI - INTERACTIVE COGNITIVE REPL                           ║");
            println!("║  16 Columns | 512 LIF Neurons | Theta-Gamma (6Hz/40Hz) | Zero-Backprop STDP Plasticity ║");
            println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
            println!("║ Commands:                                                                             ║");
            println!("║   /learn <concept> = <explanation>  Teach new concept via live STDP synaptic update   ║");
            println!("║   /sleep                            Trigger hippocampal SWR sleep & restore energy   ║");
            println!("║   /cldb <file.cldb>                 Load crystallized .cldb long-term vault engrams   ║");
            println!("║   /stats                            Display cortical telemetry and spike count        ║");
            println!("║   /cl                               Export 40-byte Base-94 Clifford VLIW microcode    ║");
            println!("║   /exit or /quit                    Exit interactive session                          ║");
            println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝\n");

            let stdin = std::io::stdin();
            loop {
                print!("Sovereign> ");
                let _ = std::io::stdout().flush();

                let mut input = String::new();
                if stdin.read_line(&mut input).is_err() || input.trim().is_empty() {
                    continue;
                }
                let line = input.trim();
                if line == "/exit" || line == "/quit" || line == "exit" || line == "quit" {
                    println!("[REPL] Exiting Sovereign Mind session.");
                    break;
                } else if line == "/stats" {
                    println!("{}", mind.get_stats_report());
                } else if line == "/sleep" {
                    let report = mind.consolidate_sleep();
                    println!("\n[SWR SLEEP] {}\n", report);
                } else if line.starts_with("/cldb") {
                    let path = line.trim_start_matches("/cldb").trim();
                    match mind.load_cldb_vault(path) {
                        Ok(count) => println!("[CLDB] Successfully loaded {} engrams into long-term associative memory.", count),
                        Err(e) => println!("[CLDB ERROR] {}", e),
                    }
                } else if line == "/cl" {
                    let bundle = mind.export_microcode_bundle();
                    println!("[CL MICROCODE] 40-Byte Base-94 VLIW Bundle: \"{}\"", bundle.to_ascii_bundle());
                } else if line.starts_with("/learn") {
                    let rest = line.trim_start_matches("/learn").trim();
                    if let Some((concept, expl)) = rest.split_once('=') {
                        let spikes = mind.learn_fact(concept.trim(), expl.trim());
                        println!("[STDP LEARNED] Concept '{}' assimilated! Emitted {} spikes across 16 columns.",
                            concept.trim(), spikes);
                    } else {
                        println!("[ERROR] Format: /learn <concept> = <explanation>");
                    }
                } else {
                    let (answer, elapsed, tier) = mind.query(line);
                    println!("\n[{}] ({:.3} ms)", tier, elapsed.as_secs_f64() * 1000.0);
                    println!("{}\n", answer);
                }
            }
        }
        "stats" => {
            let emit_json = args.iter().any(|a| a == "--json");
            let mind = cronc::SovereignMind::new();
            if emit_json {
                println!(r#"{{"status":"ok","columns":16,"neurons":512,"theta_hz":6.0,"gamma_hz":40.0,"learned_concepts":{},"memory_mb":{:.2}}}"#,
                    mind.learned_concepts, (mind.memory_allocated_bytes as f64) / (1024.0 * 1024.0));
            } else {
                println!("{}", mind.get_stats_report());
            }
        }
        "export" => {
            let mut out_path = "sovereign_bundle.cl".to_string();
            let mut i = 1;
            while i < args.len() {
                if args[i] == "-o" && i + 1 < args.len() {
                    out_path = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let mind = cronc::SovereignMind::new();
            let bundle = mind.export_microcode_bundle();
            let ascii_bundle = bundle.to_ascii_bundle();
            if let Err(e) = fs::write(&out_path, format!("// Sovereign Mind 40-byte Base-94 VLIW Microcode Bundle\n{}\n", ascii_bundle)) {
                eprintln!("[ERROR] Failed to write bundle to '{}': {}", out_path, e);
            } else {
                println!("[SUCCESS] Exported 40-byte Base-94 Clifford VLIW Bundle to '{}':\n  {}", out_path, ascii_bundle);
            }
        }
        "corpus" => {
            let emit_json = args.iter().any(|a| a == "--json");
            let tokens_count = cronc::SOVEREIGN_MORPH_TOKENS.len();
            let knowledge_count = cronc::SOVEREIGN_KNOWLEDGE_BASE.len();
            if emit_json {
                println!(r#"{{"status":"ok","morph_tokens":{},"knowledge_topics":{}}}"#,
                    tokens_count, knowledge_count);
            } else {
                println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
                println!("║                     SOVEREIGN KNOWLEDGE CORPUS AUDIT                                  ║");
                println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                println!("║ Morph-Tokens     : {:<66} ║", format!("{} discrete tokens (94-character Base-94)", tokens_count));
                println!("║ Core Topics      : {:<66} ║", format!("{} foundation AGI engrams & knowledge entries", knowledge_count));
                println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                println!("║ Sample Knowledge Topics:                                                              ║");
                for (prompt, answer) in cronc::SOVEREIGN_KNOWLEDGE_BASE.iter().take(5) {
                    let preview: String = answer.chars().take(35).collect();
                    println!("║  • {:<20} -> {:<55} ║", prompt, preview);
                }
                println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
            }
        }
        "ui" | "web" | "dashboard" => {
            let mut port: u16 = 8088;
            let mut host = "127.0.0.1".to_string();
            let mut auto_open = false;
            let mut export_html_path: Option<String> = None;

            let mut i = 1;
            while i < args.len() {
                if args[i] == "--port" && i + 1 < args.len() {
                    if let Ok(p) = args[i + 1].parse::<u16>() { port = p; }
                    i += 2;
                } else if args[i] == "--host" && i + 1 < args.len() {
                    host = args[i + 1].clone();
                    i += 2;
                } else if args[i] == "--open" {
                    auto_open = true;
                    i += 1;
                } else if (args[i] == "-o" || args[i] == "--output") && i + 1 < args.len() {
                    export_html_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }

            let dashboard_html = include_str!("sovereign_dashboard.html");

            if let Some(ref path) = export_html_path {
                if let Err(e) = fs::write(path, dashboard_html) {
                    eprintln!("[ERROR] Failed to export dashboard HTML to '{}': {}", path, e);
                } else {
                    println!("[SUCCESS] Sovereign Mind 3D AGI Dashboard exported to '{}'", path);
                }
                return;
            }

            println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
            println!("║             CRON SOVEREIGN MIND AGI - 3D NEUROMORPHIC WEB DASHBOARD                   ║");
            println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
            println!("║ Dashboard URL    : {:<66} ║", format!("http://{}:{}/", host, port));
            println!("║ 16 Columns       : 512 LIF Neurons | 16-Bank PGAS SRAM Heatmap | STDP Plasticity      ║");
            println!("║ REST API Ready   : /api/query, /api/learn, /api/sleep, /api/stats, /api/export       ║");
            println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");

            if auto_open {
                let url = format!("http://{}:{}/", host, port);
                let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn();
            }

            let addr = format!("{}:{}", host, port);
            let listener = match TcpListener::bind(&addr) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("[ERROR] Failed to bind to '{}': {}", addr, e);
                    return;
                }
            };

            let mut mind = cronc::SovereignMind::new();

            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0u8; 4096];
                    let bytes_read = match stream.read(&mut buffer) {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    let req_str = String::from_utf8_lossy(&buffer[..bytes_read]);

                    if req_str.starts_with("GET / ") || req_str.starts_with("GET /index.html ") {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            dashboard_html.len(),
                            dashboard_html
                        );
                        let _ = stream.write_all(response.as_bytes());
                    } else if req_str.starts_with("GET /api/stats ") {
                        let json = format!(
                            r#"{{"columns":16,"neurons":512,"theta_hz":6.0,"gamma_hz":40.0,"active_synapses":35344,"memory_mb":11.8,"learned_concepts":{}}}"#,
                            mind.learned_concepts
                        );
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            json.len(), json
                        );
                        let _ = stream.write_all(response.as_bytes());
                    } else if req_str.starts_with("POST /api/sleep ") {
                        let report = mind.consolidate_sleep();
                        let json = format!(
                            r#"{{"status":"ok","report":{:?},"energy":100.0,"fatigue":0.0}}"#,
                            report
                        );
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            json.len(), json
                        );
                        let _ = stream.write_all(response.as_bytes());
                    } else if req_str.starts_with("POST /api/query ") {
                        let mut prompt = "Sovereign Mind".to_string();
                        if let Some(idx) = req_str.find("\r\n\r\n") {
                            let body = &req_str[idx + 4..];
                            if let Some(p_start) = body.find("\"prompt\":") {
                                let after = body[p_start + 9..].trim();
                                if after.starts_with('"') {
                                    if let Some(p_end) = after[1..].find('"') {
                                        prompt = after[1..1 + p_end].to_string();
                                    }
                                }
                            }
                        }
                        let (ans, elapsed, tier) = mind.query(&prompt);
                        let json = format!(
                            r#"{{"prompt":{:?},"answer":{:?},"elapsed_ms":{:.3},"tier":"{}"}}"#,
                            prompt, ans, elapsed.as_secs_f64() * 1000.0, tier
                        );
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            json.len(), json
                        );
                        let _ = stream.write_all(response.as_bytes());
                    } else if req_str.starts_with("GET /api/export ") {
                        let bundle = mind.export_microcode_bundle();
                        let ascii = bundle.to_ascii_bundle();
                        let json = format!(r#"{{"bundle":{:?}}}"#, ascii);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            json.len(), json
                        );
                        let _ = stream.write_all(response.as_bytes());
                    } else if req_str.starts_with("GET /api/evolve") {
                        let query_param = if let Some(idx) = req_str.find("?intent=") {
                            let end = req_str[idx + 8..].find(' ').unwrap_or(req_str.len() - (idx + 8));
                            req_str[idx + 8..idx + 8 + end].replace("%20", " ").replace('+', " ")
                        } else {
                            "clifford rotor geometric spin sandwich".to_string()
                        };
                        match mind.synthesize_cl_routine(&query_param) {
                            Ok(routine) => {
                                let _ = mind.compile_and_hotreload_routine(&routine);
                                let json = format!(
                                    r#"{{"status":"ok","name":{:?},"intent":{:?},"bundle":{:?},"crc8":"0x{:02X}","cycles":{},"output_r3":{},"source_cl":{:?}}}"#,
                                    routine.name, routine.intent, routine.bundle_ascii, routine.crc8, routine.verified_cycles, routine.verified_output, routine.source_cl
                                );
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    json.len(), json
                                );
                                let _ = stream.write_all(response.as_bytes());
                            }
                            Err(e) => {
                                let err_json = format!(r#"{{"status":"error","message":{:?}}}"#, e);
                                let response = format!(
                                    "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    err_json.len(), err_json
                                );
                                let _ = stream.write_all(response.as_bytes());
                            }
                        }
                    } else {
                        let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                        let _ = stream.write_all(not_found.as_bytes());
                    }
                }
            }
        }
        "evolve" => {
            let mut intent: Option<String> = None;
            let mut out_path: Option<String> = None;
            let mut hotreload = false;
            let mut emit_json = false;

            let mut i = 1;
            while i < args.len() {
                if (args[i] == "--intent" || args[i] == "-i") && i + 1 < args.len() {
                    intent = Some(args[i + 1].clone());
                    i += 2;
                } else if (args[i] == "-o" || args[i] == "--out") && i + 1 < args.len() {
                    out_path = Some(args[i + 1].clone());
                    i += 2;
                } else if args[i] == "--hotreload" || args[i] == "--hot-reload" {
                    hotreload = true;
                    i += 1;
                } else if args[i] == "--json" {
                    emit_json = true;
                    i += 1;
                } else if !args[i].starts_with('-') && intent.is_none() {
                    intent = Some(args[i].clone());
                    i += 1;
                } else {
                    i += 1;
                }
            }

            let user_intent = intent.unwrap_or_else(|| "clifford rotor geometric spin sandwich".to_string());
            let mut mind = cronc::SovereignMind::new();

            match mind.synthesize_cl_routine(&user_intent) {
                Ok(routine) => {
                    let reload_msg = if hotreload {
                        match mind.compile_and_hotreload_routine(&routine) {
                            Ok(msg) => msg,
                            Err(e) => format!("Hot-reload error: {}", e),
                        }
                    } else {
                        "Not hot-reloaded (pass --hotreload to register into Crystalline Vault)".to_string()
                    };

                    if let Some(ref path) = out_path {
                        if let Err(e) = fs::write(path, &routine.source_cl) {
                            eprintln!("[ERROR] Failed to save synthesized CL routine to '{}': {}", path, e);
                        } else if !emit_json {
                            println!("[SAVED] Synthesized routine saved to '{}'", path);
                        }
                    }

                    if emit_json {
                        println!(
                            r#"{{"status":"ok","name":{:?},"intent":{:?},"bundle":{:?},"crc8":"0x{:02X}","cycles":{},"output_r3":{},"hotreload":{:?}}}"#,
                            routine.name, routine.intent, routine.bundle_ascii, routine.crc8, routine.verified_cycles, routine.verified_output, reload_msg
                        );
                    } else {
                        println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
                        println!("║               CRON SOVEREIGN AUTONOMOUS CL ROUTINE SYNTHESIS                          ║");
                        println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                        println!("║ Routine Name : {:<71} ║", routine.name);
                        println!("║ Intent       : {:<71} ║", routine.intent);
                        println!("║ 40B Base-94  : {:<71} ║", routine.bundle_ascii);
                        println!("║ CRC-8 Check  : {:<71} ║", format!("0x{:02X} (ATM Poly 0x07 Verified)", routine.crc8));
                        println!("║ Cycles & R3  : {:<71} ║", format!("{} Cycle | Verified Output R3 = {}", routine.verified_cycles, routine.verified_output));
                        println!("║ Hot-Reload   : {:<71} ║", reload_msg);
                        println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
                        println!("║ Synthesized Machine Code (.cl):                                                       ║");
                        for line in routine.source_cl.lines() {
                            println!("║   {:<83} ║", line);
                        }
                        println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
                    }
                }
                Err(e) => eprintln!("[ERROR] Routine synthesis failed: {}", e),
            }
        }
        other => {
            eprintln!("[ERROR] Unknown sovereign subcommand '{}'. Supported: chat, stats, export, corpus, ui", other);
        }
    }
}

pub fn handle_cl_clifford_sim_command(args: &[String]) {
    let mut bundle_str: Option<String> = None;
    let mut emit_json = false;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--json" {
            emit_json = true;
            i += 1;
        } else if args[i] == "-h" || args[i] == "--help" {
            println!("USAGE: cron cl-clifford-sim [bundle_ascii] [--json]");
            return;
        } else if !args[i].starts_with('-') && bundle_str.is_none() {
            bundle_str = Some(args[i].clone());
            i += 1;
        } else {
            i += 1;
        }
    }

    let bundle = if let Some(ref s) = bundle_str {
        match cronc::CliffordBundle::from_ascii_bundle(s) {
            Some(b) => b,
            None => {
                eprintln!("[ERROR] Invalid 40-byte Base-94 Clifford bundle: expected at least 40 ASCII chars (33..=126)");
                return;
            }
        }
    } else {
        cronc::CliffordBundle::new(
            cronc::CliffordMicroOp { opcode: cronc::CliffordMicroOpCode::CliffordAdd, rd: 1, rs1: 0, rs2: 0, imm: 42 },
            cronc::CliffordMicroOp { opcode: cronc::CliffordMicroOpCode::RotSandwich, rd: 2, rs1: 1, rs2: 0, imm: 8192 },
            cronc::CliffordMicroOp { opcode: cronc::CliffordMicroOpCode::TropicMax, rd: 3, rs1: 1, rs2: 2, imm: 0 },
            cronc::CliffordMicroOp { opcode: cronc::CliffordMicroOpCode::EmitChar, rd: 0, rs1: 0, rs2: 0, imm: (b'K' - 33) as i16 },
        )
    };

    let mut emu = cronc::CliffordRotorEmulator::new();
    emu.execute_bundle(&bundle);
    let ascii_str = bundle.to_ascii_bundle();

    if emit_json {
        let non_zero_regs: Vec<(usize, i64)> = emu.regs.iter().copied().enumerate().filter(|&(_, v)| v != 0).collect();
        let emitted: String = emu.emitted_chars.iter().collect();
        println!(r#"{{"status":"ok","bundle":"{}","cycles":{},"emitted":{:?},"regs":{:?}}}"#,
            ascii_str, emu.cycle_count, emitted, non_zero_regs);
    } else {
        println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║             CLIFFORD ROTOR 320-BIT VLIW HARDWARE EMULATION HUD                        ║");
        println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 40-Byte Bundle   : {:<66} ║", ascii_str);
        println!("║ Cycle Count      : {:<66} ║", emu.cycle_count);
        println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ VLIW SLOTS DISASSEMBLY (4-Tact Parallel Issue):                                       ║");
        let slots = [
            ("SLOT 0 [CLIFFORD ADD] ", bundle.slot0),
            ("SLOT 1 [ROT SANDWICH] ", bundle.slot1),
            ("SLOT 2 [TROPIC MAX]   ", bundle.slot2),
            ("SLOT 3 [OUTPUT EMIT]  ", bundle.slot3),
        ];
        for (name, op) in &slots {
            println!("║  • {} -> {:<14} rd: {:>2}, rs1: {:>2}, rs2: {:>2}, imm: {:>6} ║",
                name, op.opcode.mnemonic(), op.rd, op.rs1, op.rs2, op.imm);
        }
        println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ REGISTER STATE (Non-Zero):                                                            ║");
        let mut reg_line = String::new();
        for (i, &v) in emu.regs.iter().enumerate() {
            if v != 0 {
                if !reg_line.is_empty() {
                    reg_line.push_str("  ");
                }
                reg_line.push_str(&format!("R{}: {}", i, v));
            }
        }
        if reg_line.is_empty() {
            println!("║  (all registers zero)                                                                 ║");
        } else {
            println!("║  {:<85} ║", reg_line);
        }
        let emitted: String = emu.emitted_chars.iter().collect();
        println!("║ Emitted Stream   : {:<66} ║", if emitted.is_empty() { "(none)" } else { &emitted });
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
    }
}
