// ============================================================================
// CRON Debug Adapter Protocol (DAP) Server Engine
// Standard: Debug Adapter Protocol (DAP) over JSON-RPC / BaseProtocol Stdio
// Target: 256-Core 4D-Torus Photonic Neuromorphic Processor Simulator
// Supports: .cr Blueprint and .cl VLIW Machine Code Debugging
// ============================================================================

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use cron_vm::debugger::{disassemble_bundle, Breakpoint, Debugger};
use serde_json::{json, Value};

/// Debug Adapter Protocol Server State
pub struct DapServer {
    pub debugger: Option<Debugger>,
    pub program_path: Option<String>,
    pub program_source_lines: Vec<String>,
    pub is_cr_source: bool,
    pub seq_counter: i64,
    pub inspected_core: usize,
    pub stop_on_entry: bool,
    pub is_running: bool,
    pub is_terminated: bool,
    pub breakpoints_by_line: HashMap<usize, Breakpoint>,
}

impl Default for DapServer {
    fn default() -> Self {
        Self::new()
    }
}

impl DapServer {
    pub fn new() -> Self {
        Self {
            debugger: None,
            program_path: None,
            program_source_lines: Vec::new(),
            is_cr_source: false,
            seq_counter: 1,
            inspected_core: 0,
            stop_on_entry: true,
            is_running: false,
            is_terminated: false,
            breakpoints_by_line: HashMap::new(),
        }
    }

    /// Generate next unique sequence number
    fn next_seq(&mut self) -> i64 {
        let s = self.seq_counter;
        self.seq_counter += 1;
        s
    }

    /// Process a single incoming DAP JSON message and return outgoing messages
    pub fn handle_message(&mut self, json_str: &str) -> Vec<String> {
        let parsed: Value = match serde_json::from_str(json_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[DAP JSON ERROR] {}", e);
                return Vec::new();
            }
        };

        let msg_type = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if msg_type != "request" {
            return Vec::new();
        }

        let req_seq = parsed.get("seq").and_then(|v| v.as_i64()).unwrap_or(0);
        let command = parsed.get("command").and_then(|v| v.as_str()).unwrap_or("");
        let args = parsed.get("arguments").cloned().unwrap_or(json!({}));

        let mut responses = Vec::new();

        match command {
            "initialize" => {
                let resp_seq = self.next_seq();
                let capabilities = json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsFunctionBreakpoints": true,
                    "supportsConditionalBreakpoints": true,
                    "supportsStepBack": true,
                    "supportsSetVariable": true,
                    "supportsRestartFrame": false,
                    "supportsGotoTargetsRequest": false,
                    "supportsStepInTargetsRequest": false,
                    "supportsCompletionsRequest": false,
                    "supportsModulesRequest": false,
                    "supportsExceptionOptions": true,
                    "supportsValueFormattingOptions": true,
                    "supportsExceptionInfoRequest": true,
                    "supportTerminateDebuggee": true,
                    "supportsDelayedStackTraceLoading": false,
                    "supportsLoadedSourcesRequest": false,
                    "supportsLogPoints": false,
                    "supportsTerminateThreadsRequest": false,
                    "supportsDataBreakpoints": false,
                    "supportsReadMemoryRequest": false,
                    "supportsDisassembleRequest": true
                });

                let init_resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "initialize",
                    "success": true,
                    "body": capabilities
                });
                responses.push(init_resp.to_string());

                // DAP standard: follow initialize with initialized event
                let init_event_seq = self.next_seq();
                let init_event = json!({
                    "seq": init_event_seq,
                    "type": "event",
                    "event": "initialized"
                });
                responses.push(init_event.to_string());
            }

            "launch" => {
                let program = args.get("program").and_then(|v| v.as_str()).unwrap_or("").to_string();
                self.stop_on_entry = args.get("stopOnEntry").and_then(|v| v.as_bool()).unwrap_or(true);
                self.inspected_core = args.get("coreId").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

                let mut launch_success = true;
                let mut launch_error = String::new();

                if program.is_empty() {
                    launch_success = false;
                    launch_error = "No program file specified in launch configuration".to_string();
                } else if !Path::new(&program).exists() {
                    launch_success = false;
                    launch_error = format!("Program file does not exist: {}", program);
                } else {
                    let content = match fs::read_to_string(&program) {
                        Ok(c) => c,
                        Err(e) => {
                            launch_success = false;
                            launch_error = format!("Failed to read program '{}': {}", program, e);
                            String::new()
                        }
                    };

                    if launch_success {
                        self.program_path = Some(program.clone());
                        self.is_cr_source = program.ends_with(".cr");
                        self.program_source_lines = content.lines().map(|s| s.to_string()).collect();

                        // Compile .cr to machine-native .cl if needed
                        let cl_code = if self.is_cr_source {
                            match cronc::compile_source(&content) {
                                Ok(compiled) => compiled,
                                Err(e) => {
                                    launch_success = false;
                                    launch_error = format!("CRON compilation error: {}", e);
                                    String::new()
                                }
                            }
                        } else {
                            content
                        };

                        if launch_success {
                            let mut dbg = Debugger::new(&cl_code);
                            dbg.set_inspected_core(self.inspected_core);
                            self.debugger = Some(dbg);
                            self.is_running = true;
                            self.is_terminated = false;
                        }
                    }
                }

                let resp_seq = self.next_seq();
                let launch_resp = if launch_success {
                    json!({
                        "seq": resp_seq,
                        "type": "response",
                        "request_seq": req_seq,
                        "command": "launch",
                        "success": true
                    })
                } else {
                    json!({
                        "seq": resp_seq,
                        "type": "response",
                        "request_seq": req_seq,
                        "command": "launch",
                        "success": false,
                        "message": launch_error
                    })
                };
                responses.push(launch_resp.to_string());

                // If stopOnEntry is set, notify client that execution stopped at entry
                if launch_success && self.stop_on_entry {
                    let stop_event_seq = self.next_seq();
                    let stop_event = json!({
                        "seq": stop_event_seq,
                        "type": "event",
                        "event": "stopped",
                        "body": {
                            "reason": "entry",
                            "threadId": 1,
                            "allThreadsStopped": true,
                            "description": "Stopped at entry (Cycle #1)"
                        }
                    });
                    responses.push(stop_event.to_string());
                }
            }

            "setBreakpoints" => {
                let bp_args = args.get("breakpoints").and_then(|v| v.as_array());
                let mut verified_bps = Vec::new();

                if let Some(dbg) = &mut self.debugger {
                    dbg.clear_breakpoints();
                    self.breakpoints_by_line.clear();

                    if let Some(bps) = bp_args {
                        for bp_val in bps {
                            let line = bp_val.get("line").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                            let condition = bp_val.get("condition").and_then(|v| v.as_str());

                            let parsed_bp = if let Some(cond) = condition {
                                let cond_trimmed = cond.trim();
                                if cond_trimmed.to_lowercase().contains("trap") {
                                    Breakpoint::Trap
                                } else if cond_trimmed.to_lowercase().starts_with("temp") {
                                    let deg: u32 = cond_trimmed
                                        .chars()
                                        .filter(|c| c.is_ascii_digit())
                                        .collect::<String>()
                                        .parse()
                                        .unwrap_or(105);
                                    Breakpoint::Thermal(deg)
                                } else if cond_trimmed.len() == 2 && cond_trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                                    Breakpoint::Opcode(cond_trimmed.to_uppercase())
                                } else if cond_trimmed.starts_with('R') || cond_trimmed.starts_with('r') {
                                    // e.g. "R0 == 42" or "R4 > 100"
                                    let parts: Vec<&str> = cond_trimmed.split_whitespace().collect();
                                    if parts.len() >= 3 {
                                        let reg_idx: usize = parts[0][1..].parse().unwrap_or(0);
                                        let op = parts[1].to_string();
                                        let val = if parts[2].starts_with("0x") || parts[2].starts_with("0X") {
                                            u32::from_str_radix(&parts[2][2..], 16).unwrap_or(0)
                                        } else {
                                            parts[2].parse().unwrap_or(0)
                                        };
                                        Breakpoint::RegisterCond {
                                            core_id: self.inspected_core,
                                            reg: reg_idx,
                                            op,
                                            val,
                                        }
                                    } else {
                                        Breakpoint::Cycle(line)
                                    }
                                } else {
                                    Breakpoint::Cycle(line)
                                }
                            } else {
                                Breakpoint::Cycle(line)
                            };

                            dbg.add_breakpoint(parsed_bp.clone());
                            self.breakpoints_by_line.insert(line, parsed_bp);

                            verified_bps.push(json!({
                                "verified": true,
                                "line": line
                            }));
                        }
                    }
                }

                let resp_seq = self.next_seq();
                let bp_resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "setBreakpoints",
                    "success": true,
                    "body": {
                        "breakpoints": verified_bps
                    }
                });
                responses.push(bp_resp.to_string());
            }

            "setExceptionBreakpoints" => {
                if let Some(dbg) = &mut self.debugger {
                    if let Some(filters) = args.get("filters").and_then(|v| v.as_array()) {
                        for f in filters {
                            if let Some(f_str) = f.as_str() {
                                if f_str.eq_ignore_ascii_case("trap") {
                                    dbg.add_breakpoint(Breakpoint::Trap);
                                } else if f_str.eq_ignore_ascii_case("thermal") {
                                    dbg.add_breakpoint(Breakpoint::Thermal(105));
                                }
                            }
                        }
                    }
                }
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "setExceptionBreakpoints",
                    "success": true
                });
                responses.push(resp.to_string());
            }

            "configurationDone" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "configurationDone",
                    "success": true
                });
                responses.push(resp.to_string());
            }

            "threads" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "threads",
                    "success": true,
                    "body": {
                        "threads": [
                            {
                                "id": 1,
                                "name": format!("Core {} [4D-Torus Active Silicon]", self.inspected_core)
                            }
                        ]
                    }
                });
                responses.push(resp.to_string());
            }

            "stackTrace" => {
                let mut frames = Vec::new();
                if let Some(dbg) = &self.debugger {
                    let current_cycle = dbg.current_cycle.max(1);
                    let bundle_str = if dbg.sim.step_index < dbg.sim.instructions.len() {
                        let inst = &dbg.sim.instructions[dbg.sim.step_index];
                        inst.slots.join(" ")
                    } else {
                        "END OF PROGRAM".to_string()
                    };

                    let line_number = current_cycle;
                    let file_name = self.program_path.as_deref().unwrap_or("program.cl");

                    frames.push(json!({
                        "id": 1,
                        "name": format!("Bundle #{:04}: {}", current_cycle, bundle_str),
                        "line": line_number,
                        "column": 1,
                        "source": {
                            "name": Path::new(file_name).file_name().and_then(|s| s.to_str()).unwrap_or("source"),
                            "path": file_name
                        }
                    }));
                }

                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "stackTrace",
                    "success": true,
                    "body": {
                        "stackFrames": frames,
                        "totalFrames": frames.len()
                    }
                });
                responses.push(resp.to_string());
            }

            "scopes" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "scopes",
                    "success": true,
                    "body": {
                        "scopes": [
                            {
                                "name": "Registers (R0 - R15)",
                                "variablesReference": 1000,
                                "expensive": false
                            },
                            {
                                "name": "Photonic Wave & SNN State",
                                "variablesReference": 2000,
                                "expensive": false
                            },
                            {
                                "name": "Hardware CSR & Thermal Telemetry",
                                "variablesReference": 3000,
                                "expensive": false
                            },
                            {
                                "name": "VLIW Disassembled Micro-Ops",
                                "variablesReference": 4000,
                                "expensive": false
                            }
                        ]
                    }
                });
                responses.push(resp.to_string());
            }

            "variables" => {
                let var_ref = args.get("variablesReference").and_then(|v| v.as_i64()).unwrap_or(0);
                let mut vars = Vec::new();

                if let Some(dbg) = &self.debugger {
                    let core_id = self.inspected_core.min(255);
                    let regs = dbg.sim.core_dump(core_id);
                    let core = &dbg.sim.cores[core_id];

                    match var_ref {
                        1000 => {
                            // Core General Purpose Registers R0..R15
                            for (i, &val) in regs.iter().enumerate() {
                                let was_modified = dbg.last_reg_diffs.iter().any(|d| d.reg == i);
                                let mod_tag = if was_modified { " [MODIFIED]" } else { "" };
                                vars.push(json!({
                                    "name": format!("R{}{}", i, mod_tag),
                                    "value": format!("{:#010X} ({})", val, val),
                                    "type": "u32",
                                    "variablesReference": 0
                                }));
                            }
                        }
                        2000 => {
                            // Photonic Wave Amplitudes & Synaptic State
                            let amps = core.wave_reg.amplitudes;
                            vars.push(json!({
                                "name": "Photonic Wave Amplitudes",
                                "value": format!("[W0={}, W1={}, W2={}, W3={}]", amps[0], amps[1], amps[2], amps[3]),
                                "type": "vec4_u8",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "Photonic Phase Rotors",
                                "value": format!("[P0={}, P1={}, P2={}, P3={}]", core.wave_reg.phases[0], core.wave_reg.phases[1], core.wave_reg.phases[2], core.wave_reg.phases[3]),
                                "type": "vec4_u8",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "STDP Synaptic Weights",
                                "value": format!("{:?}", core.stdp_weights),
                                "type": "[u8; 16]",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "Reversible Stack Depth",
                                "value": format!("{} entries", core.reversible_stack.len()),
                                "type": "usize",
                                "variablesReference": 0
                            }));
                        }
                        3000 => {
                            // Hardware CSR & Telemetry
                            let temp = core.thermal_level;
                            let throttled = dbg.sim.thermal_throttled_cores[core_id];
                            vars.push(json!({
                                "name": "Core Thermal Level",
                                "value": format!("{} °C ({})", temp, if throttled { "THROTTLED (DVFS 1/2 CLK)" } else { "NORMAL (FULL CLK)" }),
                                "type": "temperature",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "Hardware Traps Count",
                                "value": format!("{}", core.trap_count),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "MCAUSE (Trap Reason)",
                                "value": format!("{:#06X}", core.mcause),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "MEPC (Faulting Cycle)",
                                "value": format!("{}", core.mepc),
                                "type": "usize",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "CSR Cycle Count",
                                "value": format!("{}", core.read_csr(0)),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "CSR Stall Cycles",
                                "value": format!("{}", core.read_csr(1)),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "CSR Predicated Exec Ops",
                                "value": format!("{}", core.read_csr(2)),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                            vars.push(json!({
                                "name": "CSR Vector Burst Count",
                                "value": format!("{}", core.read_csr(3)),
                                "type": "u32",
                                "variablesReference": 0
                            }));
                        }
                        4000 => {
                            // Disassembled Micro-Ops in Current Execution Bundle
                            if dbg.sim.step_index < dbg.sim.instructions.len() {
                                let inst = &dbg.sim.instructions[dbg.sim.step_index];
                                let bundle_disasm = disassemble_bundle(inst);
                                for (idx, slot) in bundle_disasm.slots.iter().enumerate() {
                                    vars.push(json!({
                                        "name": format!("Slot {} [{}]", idx, slot.raw),
                                        "value": format!("{} :: {}", slot.mnemonic, slot.description),
                                        "type": "vliw_slot",
                                        "variablesReference": 0
                                    }));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "variables",
                    "success": true,
                    "body": {
                        "variables": vars
                    }
                });
                responses.push(resp.to_string());
            }

            "setVariable" => {
                let var_ref = args.get("variablesReference").and_then(|v| v.as_i64()).unwrap_or(0);
                let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let value_str = args.get("value").and_then(|v| v.as_str()).unwrap_or("");

                let mut updated_val = 0u32;
                let mut success = false;

                if var_ref == 1000 {
                    // Updating a register (e.g. "R0" or "R0 [MODIFIED]")
                    let clean_name = name.split_whitespace().next().unwrap_or("");
                    if clean_name.starts_with('R') || clean_name.starts_with('r') {
                        if let Ok(reg_idx) = clean_name[1..].parse::<usize>() {
                            if reg_idx < 16 {
                                let parsed_num = if value_str.starts_with("0x") || value_str.starts_with("0X") {
                                    u32::from_str_radix(&value_str[2..], 16).ok()
                                } else {
                                    value_str.parse::<u32>().ok()
                                };

                                if let Some(v) = parsed_num {
                                    if let Some(dbg) = &mut self.debugger {
                                        dbg.sim.cores[self.inspected_core].registers[reg_idx] = v;
                                        updated_val = v;
                                        success = true;
                                    }
                                }
                            }
                        }
                    }
                }

                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "setVariable",
                    "success": success,
                    "body": {
                        "value": format!("{:#010X}", updated_val),
                        "type": "u32"
                    }
                });
                responses.push(resp.to_string());
            }

            "next" | "stepIn" => {
                let mut is_halted = false;
                let mut current_cycle = 0;

                if let Some(dbg) = &mut self.debugger {
                    let _step_res = dbg.step();
                    is_halted = dbg.is_halted;
                    current_cycle = dbg.current_cycle;
                }

                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": command,
                    "success": true
                });
                responses.push(resp.to_string());

                if is_halted {
                    let term_seq = self.next_seq();
                    let term_event = json!({
                        "seq": term_seq,
                        "type": "event",
                        "event": "terminated"
                    });
                    responses.push(term_event.to_string());
                } else {
                    let stop_seq = self.next_seq();
                    let stop_event = json!({
                        "seq": stop_seq,
                        "type": "event",
                        "event": "stopped",
                        "body": {
                            "reason": "step",
                            "threadId": 1,
                            "allThreadsStopped": true,
                            "description": format!("Stepped to Cycle #{}", current_cycle)
                        }
                    });
                    responses.push(stop_event.to_string());
                }
            }

            "stepBack" => {
                let mut success = false;
                let mut current_cycle = 0;

                if let Some(dbg) = &mut self.debugger {
                    if dbg.step_backward() {
                        success = true;
                        current_cycle = dbg.current_cycle;
                    }
                }

                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "stepBack",
                    "success": success,
                    "message": if success { Value::Null } else { Value::String("No previous step history available".to_string()) }
                });
                responses.push(resp.to_string());

                if success {
                    let stop_seq = self.next_seq();
                    let stop_event = json!({
                        "seq": stop_seq,
                        "type": "event",
                        "event": "stopped",
                        "body": {
                            "reason": "step",
                            "threadId": 1,
                            "allThreadsStopped": true,
                            "description": format!("Stepped back to Cycle #{}", current_cycle)
                        }
                    });
                    responses.push(stop_event.to_string());
                }
            }

            "continue" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "continue",
                    "success": true
                });
                responses.push(resp.to_string());

                let mut is_halted = false;
                let mut hit_bp_desc = None;
                let mut current_cycle = 0;

                if let Some(dbg) = &mut self.debugger {
                    let _step_res = dbg.continue_exec();
                    is_halted = dbg.is_halted;
                    current_cycle = dbg.current_cycle;
                    if let Some(bp) = &dbg.hit_breakpoint {
                        hit_bp_desc = Some(bp.description());
                    }
                }

                if is_halted {
                    let term_seq = self.next_seq();
                    let term_event = json!({
                        "seq": term_seq,
                        "type": "event",
                        "event": "terminated"
                    });
                    responses.push(term_event.to_string());
                } else {
                    let reason = if hit_bp_desc.is_some() { "breakpoint" } else { "pause" };
                    let desc = hit_bp_desc.unwrap_or_else(|| format!("Paused at Cycle #{}", current_cycle));

                    let stop_seq = self.next_seq();
                    let stop_event = json!({
                        "seq": stop_seq,
                        "type": "event",
                        "event": "stopped",
                        "body": {
                            "reason": reason,
                            "threadId": 1,
                            "allThreadsStopped": true,
                            "description": desc
                        }
                    });
                    responses.push(stop_event.to_string());
                }
            }

            "pause" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": "pause",
                    "success": true
                });
                responses.push(resp.to_string());

                let stop_seq = self.next_seq();
                let stop_event = json!({
                    "seq": stop_seq,
                    "type": "event",
                    "event": "stopped",
                    "body": {
                        "reason": "pause",
                        "threadId": 1,
                        "allThreadsStopped": true
                    }
                });
                responses.push(stop_event.to_string());
            }

            "disconnect" | "terminate" => {
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": command,
                    "success": true
                });
                responses.push(resp.to_string());

                let term_seq = self.next_seq();
                let term_event = json!({
                    "seq": term_seq,
                    "type": "event",
                    "event": "terminated"
                });
                responses.push(term_event.to_string());
                self.is_terminated = true;
            }

            _ => {
                // Unknown command: return graceful acknowledgement
                let resp_seq = self.next_seq();
                let resp = json!({
                    "seq": resp_seq,
                    "type": "response",
                    "request_seq": req_seq,
                    "command": command,
                    "success": true
                });
                responses.push(resp.to_string());
            }
        }

        responses
    }
}

/// Run DAP Server Loop over standard BaseProtocol stdio
pub fn run_dap_server<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut server = DapServer::new();

    loop {
        let mut content_length: usize = 0;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Ok(()); // EOF / client closed connection
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break; // End of HTTP headers
            }
            if let Some((k, v)) = trimmed.split_once(':') {
                if k.trim().eq_ignore_ascii_case("content-length") {
                    content_length = v.trim().parse().unwrap_or(0);
                }
            }
        }

        if content_length == 0 {
            continue;
        }

        let mut body_bytes = vec![0u8; content_length];
        reader.read_exact(&mut body_bytes)?;
        let body_str = String::from_utf8_lossy(&body_bytes);

        let responses = server.handle_message(&body_str);
        for resp in responses {
            let header = format!("Content-Length: {}\r\n\r\n", resp.len());
            writer.write_all(header.as_bytes())?;
            writer.write_all(resp.as_bytes())?;
            writer.flush()?;
        }

        if server.is_terminated {
            break;
        }
    }

    Ok(())
}
