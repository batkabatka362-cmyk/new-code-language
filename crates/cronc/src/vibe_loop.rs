// ============================================================================
// CRON Autonomous AI Vibe-Coding Loop (vibe_loop.rs)
// Atomic feedback loop for AI agents writing .cl microcode.
// Unifies syntax validation, CRC-8 auto-repair (cl-heal), VLIW slot compaction
// (cl-opt), RAM JIT execution (cl-jit), and LLM-tailored JSON diagnostics.
//
// 100% Pure Rust — Zero External Dependencies.
// ============================================================================

use crate::cl_heal::heal_cl_program;
use crate::cl_opt::optimize_cl_program;
use crate::cl_jit::run_cl_jit;
use crate::cl_lang::{verify_cl_program, KNOWN_OPCODES};

#[derive(Debug, Clone)]
pub struct VibeLoopConfig {
    pub auto_heal: bool,
    pub auto_opt: bool,
    pub run_jit: bool,
    pub fix_in_place: bool,
    pub output_path: Option<String>,
}

impl Default for VibeLoopConfig {
    fn default() -> Self {
        Self {
            auto_heal: true,
            auto_opt: true,
            run_jit: true,
            fix_in_place: false,
            output_path: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VibeStatus {
    Success,
    HealedAndExecuted,
    CompilationError,
    ExecutionTrap,
}

impl VibeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VibeStatus::Success => "SUCCESS",
            VibeStatus::HealedAndExecuted => "HEALED_AND_EXECUTED",
            VibeStatus::CompilationError => "COMPILATION_ERROR",
            VibeStatus::ExecutionTrap => "EXECUTION_TRAP",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VibeDiagnostic {
    pub line: usize,
    pub error_code: String,
    pub message: String,
    pub llm_fix_recommendation: String,
}

#[derive(Debug, Clone)]
pub struct VibeLoopResult {
    pub status: VibeStatus,
    pub healed: bool,
    pub fixed_crc_count: usize,
    pub padded_bundles: usize,
    pub resolved_hazards: usize,
    pub heal_diff_log: Vec<String>,
    pub original_ipc: f64,
    pub optimized_ipc: f64,
    pub speedup_percentage: f64,
    pub execution_cycles: usize,
    pub registers: [u32; 16],
    pub optical_ops: usize,
    pub reversible_ops: usize,
    pub stdp_updates: usize,
    pub spatial_broadcasts: usize,
    pub barriers: usize,
    pub fused_ops: usize,
    pub final_code: String,
    pub diagnostics: Vec<VibeDiagnostic>,
}

impl VibeLoopResult {
    /// Pure Rust JSON Serializer for external AI Coding Agents (Zero Dependencies)
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"status\": \"{}\",\n", self.status.as_str()));
        json.push_str(&format!("  \"healed\": {},\n", self.healed));

        // Healing block
        json.push_str("  \"healing_summary\": {\n");
        json.push_str(&format!("    \"fixed_crc_count\": {},\n", self.fixed_crc_count));
        json.push_str(&format!("    \"padded_bundles\": {},\n", self.padded_bundles));
        json.push_str(&format!("    \"resolved_hazards\": {},\n", self.resolved_hazards));
        if self.heal_diff_log.is_empty() {
            json.push_str("    \"diff_log\": []\n");
        } else {
            json.push_str("    \"diff_log\": [\n");
            for (i, diff) in self.heal_diff_log.iter().enumerate() {
                let escaped = diff.replace('\"', "\\\"").replace('\n', " ");
                json.push_str(&format!("      \"{}\"", escaped));
                if i + 1 < self.heal_diff_log.len() {
                    json.push_str(",\n");
                } else {
                    json.push('\n');
                }
            }
            json.push_str("    ]\n");
        }
        json.push_str("  },\n");

        // Optimization block
        json.push_str("  \"optimization_summary\": {\n");
        json.push_str(&format!("    \"original_ipc\": {:.2},\n", self.original_ipc));
        json.push_str(&format!("    \"optimized_ipc\": {:.2},\n", self.optimized_ipc));
        json.push_str(&format!("    \"speedup_percentage\": {:.1}\n", self.speedup_percentage));
        json.push_str("  },\n");

        // Execution block
        json.push_str("  \"execution\": {\n");
        json.push_str(&format!("    \"cycles\": {},\n", self.execution_cycles));
        json.push_str("    \"registers\": {\n");
        for i in 0..16 {
            json.push_str(&format!("      \"r{}\": \"0x{:08X}\"", i, self.registers[i]));
            if i < 15 {
                json.push_str(",\n");
            } else {
                json.push('\n');
            }
        }
        json.push_str("    },\n");
        json.push_str("    \"telemetry\": {\n");
        json.push_str(&format!("      \"optical_ops\": {},\n", self.optical_ops));
        json.push_str(&format!("      \"reversible_ops\": {},\n", self.reversible_ops));
        json.push_str(&format!("      \"stdp_updates\": {},\n", self.stdp_updates));
        json.push_str(&format!("      \"spatial_broadcasts\": {},\n", self.spatial_broadcasts));
        json.push_str(&format!("      \"barriers\": {},\n", self.barriers));
        json.push_str(&format!("      \"fused_ops\": {}\n", self.fused_ops));
        json.push_str("    }\n");
        json.push_str("  },\n");

        // Diagnostics block
        if self.diagnostics.is_empty() {
            json.push_str("  \"diagnostics\": []\n");
        } else {
            json.push_str("  \"diagnostics\": [\n");
            for (i, diag) in self.diagnostics.iter().enumerate() {
                let msg = diag.message.replace('\"', "\\\"").replace('\n', " ");
                let rec = diag.llm_fix_recommendation.replace('\"', "\\\"").replace('\n', " ");
                json.push_str("    {\n");
                json.push_str(&format!("      \"line\": {},\n", diag.line));
                json.push_str(&format!("      \"error_code\": \"{}\",\n", diag.error_code));
                json.push_str(&format!("      \"message\": \"{}\",\n", msg));
                json.push_str(&format!("      \"llm_fix_recommendation\": \"{}\"\n", rec));
                if i + 1 < self.diagnostics.len() {
                    json.push_str("    },\n");
                } else {
                    json.push_str("    }\n");
                }
            }
            json.push_str("  ]\n");
        }
        json.push_str("}\n");

        json
    }
}

/// Run the complete Autonomous AI Vibe-Coding loop
pub fn run_vibe_loop(source: &str, config: &VibeLoopConfig) -> VibeLoopResult {
    let mut current_code = source.to_string();
    let mut healed = false;
    let mut fixed_crc_count = 0;
    let mut padded_bundles = 0;
    let mut resolved_hazards = 0;
    let mut heal_diff_log = Vec::new();
    let mut diagnostics = Vec::new();

    // Step 1: Preliminary Syntax Audit
    let mut initial_has_syntax_error = false;
    if let Err(err_msg) = verify_cl_program(&current_code) {
        initial_has_syntax_error = true;
        // Attempt to extract line number
        let line = extract_line_number(&err_msg);
        let rec = generate_llm_fix_recommendation(&err_msg);
        diagnostics.push(VibeDiagnostic {
            line,
            error_code: "CL_SYNTAX_ERROR".to_string(),
            message: err_msg,
            llm_fix_recommendation: rec,
        });
    }

    // Step 2: Auto-Heal (if configured)
    if config.auto_heal {
        match heal_cl_program(&current_code) {
            Ok(heal_rep) => {
                let total_hazards = heal_rep.waw_hazards_resolved + heal_rep.raw_hazards_resolved;
                if heal_rep.slots_repaired_crc > 0 || heal_rep.bundles_padded_nops > 0 || total_hazards > 0 {
                    healed = true;
                    fixed_crc_count = heal_rep.slots_repaired_crc;
                    padded_bundles = heal_rep.bundles_padded_nops;
                    resolved_hazards = total_hazards;
                    if heal_rep.slots_repaired_crc > 0 {
                        heal_diff_log.push(format!("Repaired {} CRC-8 ATM slot parity tokens", heal_rep.slots_repaired_crc));
                    }
                    if heal_rep.bundles_padded_nops > 0 {
                        heal_diff_log.push(format!("Padded {} bundles with canonical NOP tokens", heal_rep.bundles_padded_nops));
                    }
                    if total_hazards > 0 {
                        heal_diff_log.push(format!("Resolved {} pipeline latency hazards via cycle splitting", total_hazards));
                    }
                    current_code = heal_rep.canonical_code;
                    // Clear preliminary diagnostics if healing successfully resolved them
                    if verify_cl_program(&current_code).is_ok() {
                        diagnostics.clear();
                        initial_has_syntax_error = false;
                    }
                }
            }
            Err(e) => {
                let line = extract_line_number(&e);
                diagnostics.push(VibeDiagnostic {
                    line,
                    error_code: "CL_HEAL_FAILURE".to_string(),
                    message: e.clone(),
                    llm_fix_recommendation: "Ensure each instruction token is exactly 10 ASCII characters with valid prefix ('_', ''', '~', '@') and terminator ('>', '!', '?', ';').".to_string(),
                });
            }
        }
    }

    // If syntax errors remain unresolved after healing, exit early with diagnostics
    if initial_has_syntax_error && diagnostics.iter().any(|d| d.error_code == "CL_SYNTAX_ERROR") {
        return VibeLoopResult {
            status: VibeStatus::CompilationError,
            healed,
            fixed_crc_count,
            padded_bundles,
            resolved_hazards,
            heal_diff_log,
            original_ipc: 0.0,
            optimized_ipc: 0.0,
            speedup_percentage: 0.0,
            execution_cycles: 0,
            registers: [0; 16],
            optical_ops: 0,
            reversible_ops: 0,
            stdp_updates: 0,
            spatial_broadcasts: 0,
            barriers: 0,
            fused_ops: 0,
            final_code: current_code,
            diagnostics,
        };
    }

    // Step 3: VLIW Slot Compaction & Optimization
    let mut original_ipc = 0.0;
    let mut optimized_ipc = 0.0;
    let mut speedup_percentage = 0.0;

    if config.auto_opt {
        match optimize_cl_program(&current_code) {
            Ok(opt_rep) => {
                original_ipc = opt_rep.original_ipc;
                optimized_ipc = opt_rep.optimized_ipc;
                speedup_percentage = opt_rep.speedup_percentage;
                current_code = opt_rep.optimized_code;
            }
            Err(e) => {
                diagnostics.push(VibeDiagnostic {
                    line: 0,
                    error_code: "CL_OPT_ERROR".to_string(),
                    message: e,
                    llm_fix_recommendation: "Optimization failed due to bundle parsing failure.".to_string(),
                });
            }
        }
    }

    // Step 4: Sub-Microsecond RAM JIT Execution
    let mut execution_cycles = 0;
    let mut registers = [0u32; 16];
    let mut optical_ops = 0;
    let mut reversible_ops = 0;
    let mut stdp_updates = 0;
    let mut spatial_broadcasts = 0;
    let mut barriers = 0;
    let mut fused_ops = 0;
    let mut status = if healed { VibeStatus::HealedAndExecuted } else { VibeStatus::Success };

    if config.run_jit {
        match run_cl_jit(&current_code) {
            Ok(core) => {
                execution_cycles = core.cycle_count;
                registers = core.r;
                optical_ops = core.optical_gemm_count;
                reversible_ops = core.reversible_ops_count;
                stdp_updates = core.stdp_updates_count;
                spatial_broadcasts = core.spatial_broadcast_count;
                barriers = core.barrier_count;
                fused_ops = core.fused_ops_count;
            }
            Err(e) => {
                status = VibeStatus::ExecutionTrap;
                let line = extract_line_number(&e);
                diagnostics.push(VibeDiagnostic {
                    line,
                    error_code: "CL_JIT_EXECUTION_TRAP".to_string(),
                    message: e,
                    llm_fix_recommendation: "Review jump offsets and branch targets to ensure they point to valid cycle boundaries or defined @labels.".to_string(),
                });
            }
        }
    }

    VibeLoopResult {
        status,
        healed,
        fixed_crc_count,
        padded_bundles,
        resolved_hazards,
        heal_diff_log,
        original_ipc,
        optimized_ipc,
        speedup_percentage,
        execution_cycles,
        registers,
        optical_ops,
        reversible_ops,
        stdp_updates,
        spatial_broadcasts,
        barriers,
        fused_ops,
        final_code: current_code,
        diagnostics,
    }
}

fn extract_line_number(msg: &str) -> usize {
    if let Some(pos) = msg.find("Line ") {
        let remainder = &msg[pos + 5..];
        if let Some(end) = remainder.find(':') {
            if let Ok(num) = remainder[..end].trim().parse::<usize>() {
                return num;
            }
        }
    }
    0
}

fn generate_llm_fix_recommendation(msg: &str) -> String {
    if msg.contains("Slot width violation") {
        "Each slot token must be exactly 10 characters long. Format: [Prefix][Opcode:2][Dest:2][Mode][Src][CRC:1][Imm][Term]. Example: '_OP01$28F>'.".to_string()
    } else if msg.contains("must contain exactly 4 slots") {
        "Each VLIW bundle line starting with 'B<cycle>:' must have exactly 4 slots. Pad remaining slots with '_NO00#000>'.".to_string()
    } else if msg.contains("Unknown .cl opcode") {
        let mut known_str = KNOWN_OPCODES.iter().take(12).cloned().collect::<Vec<_>>().join(", ");
        known_str.push_str(", ...");
        format!("Check opcode catalog for valid slot syntax. Valid opcodes include: {}. See 'cron vibe-spec --format prompt' for full list.", known_str)
    } else if msg.contains("Missing bundle cycle header") {
        "Every bundle instruction line must begin with cycle identifier 'B<num>:' (e.g. 'B0000:', 'B0001:').".to_string()
    } else if msg.contains("Structural hazard: Multiple Photonic MZI Optical operations") {
        "A single core has only 1 physical Photonic MZI mesh. Schedule only one '_OP' instruction per bundle cycle.".to_string()
    } else {
        "Ensure syntax strictly conforms to 10-character slot rules and 4-slot bundle lines. Run 'cron cl-heal' for auto-repair.".to_string()
    }
}
