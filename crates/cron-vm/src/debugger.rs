// ============================================================================
// CRON SSS+ Microarchitectural Hardware Debugger Engine
// Cycle-Accurate Stepping, Breakpoint Management & VLIW Slot Disassembler
// Target: 256-Core 4D-Torus Photonic Neuromorphic Processor
// ============================================================================

use crate::core_engine::PhotonicWave;
use crate::simulator::{HardwareStats, Simulator, VliwInstruction};
use crate::torus_mesh::Coord4D;

/// Microarchitectural Breakpoint Conditions
#[derive(Debug, Clone, PartialEq)]
pub enum Breakpoint {
    /// Break when current cycle index equals N
    Cycle(usize),
    /// Break when any slot in the current bundle contains opcode (e.g. "OP", "FU", "ST")
    Opcode(String),
    /// Break on register condition: core_id, reg_index, comparison operator ("==", "!=", ">", "<"), value
    RegisterCond {
        core_id: usize,
        reg: usize,
        op: String,
        val: u32,
    },
    /// Break whenever any core enters a hardware trap (mcause != 0)
    Trap,
    /// Break when any core's thermal level exceeds threshold in Celsius
    Thermal(u32),
}

impl Breakpoint {
    pub fn description(&self) -> String {
        match self {
            Breakpoint::Cycle(c) => format!("Cycle == {}", c),
            Breakpoint::Opcode(op) => format!("Opcode == '{}'", op),
            Breakpoint::RegisterCond { core_id, reg, op, val } => {
                format!("Core {} R{} {} 0x{:08X}", core_id, reg, op, val)
            }
            Breakpoint::Trap => "Hardware Exception Trap".to_string(),
            Breakpoint::Thermal(t) => format!("Core Temperature >= {}°C", t),
        }
    }
}

/// Disassembled representation of a single 10-character VLIW slot
#[derive(Debug, Clone, PartialEq)]
pub struct DisassembledSlot {
    pub raw: String,
    pub prefix: char,
    pub opcode: String,
    pub dest_bank: usize,
    pub dest_reg: usize,
    pub mode: char,
    pub src_reg: usize,
    pub parity: char,
    pub imm: usize,
    pub imm_val: Option<u32>,
    pub mnemonic: String,
    pub description: String,
}

/// Disassembled VLIW Instruction Bundle (4 slots per 128-bit bundle)
#[derive(Debug, Clone, PartialEq)]
pub struct DisassembledBundle {
    pub cycle: usize,
    pub raw: String,
    pub slots: Vec<DisassembledSlot>,
}

/// Disassembles a 10-character VLIW slot into human-readable micro-operations
pub fn disassemble_slot(slot: &str) -> DisassembledSlot {
    let chars: Vec<char> = slot.chars().collect();
    let prefix = chars.first().copied().unwrap_or('_');

    // Immediate constant load slot: '= <bank> <reg> # <hex16> >
    if slot.starts_with("'=") || slot.starts_with("==") {
        let dest_bank = if chars.len() >= 3 {
            chars[2].to_digit(16).unwrap_or(0) as usize
        } else {
            0
        };
        let dest_reg = if chars.len() >= 4 {
            chars[3].to_digit(16).unwrap_or(0) as usize
        } else {
            0
        };
        let imm_val = if let Some(hash_pos) = slot.find('#') {
            let hex_part: String = slot[hash_pos + 1..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();
            u32::from_str_radix(&hex_part, 16).ok()
        } else {
            let imm_digits: String = slot.chars().skip(4).filter(|c| c.is_ascii_hexdigit()).collect();
            u32::from_str_radix(&imm_digits, 16).ok()
        };
        let val = imm_val.unwrap_or(0);
        return DisassembledSlot {
            raw: slot.to_string(),
            prefix,
            opcode: "LOAD_IMM".to_string(),
            dest_bank,
            dest_reg,
            mode: '#',
            src_reg: 0,
            parity: '0',
            imm: val as usize,
            imm_val,
            mnemonic: format!("LI R{}, #0x{:04X}", dest_reg, val),
            description: format!("Load 16-bit immediate constant 0x{:04X} into register R{}", val, dest_reg),
        };
    }

    let op = if slot.len() >= 3 {
        slot[1..3].to_string()
    } else {
        "NO".to_string()
    };

    let dest_bank = if chars.len() >= 4 { chars[3].to_digit(16).unwrap_or(0) as usize } else { 0 };
    let dest_reg = if chars.len() >= 5 { chars[4].to_digit(16).unwrap_or(0) as usize } else { 0 };
    let mode = if chars.len() >= 6 { chars[5] } else { '$' };
    let src_reg = if chars.len() >= 7 { chars[6].to_digit(16).unwrap_or(0) as usize } else { 0 };
    let parity = if chars.len() >= 8 { chars[7] } else { '0' };
    let imm = if chars.len() >= 9 { chars[8].to_digit(16).unwrap_or(0) as usize } else { 0 };

    let (mnemonic, description) = match op.as_str() {
        "NO" => ("NOP".to_string(), "Pipeline issue bubble (no operation)".to_string()),
        "PO" => {
            let op_sym = match mode {
                '+' => "ADD",
                '-' => "SUB",
                '*' => "MUL",
                '/' => "DIV",
                '%' => "MOD",
                '&' => "AND",
                '|' => "OR",
                '^' => "XOR",
                '<' => "SLT",
                '>' => "SGT",
                '=' => "SEQ",
                'G' => "SGE",
                _ => "ADD",
            };
            (
                format!("{} R{}, R{}, R{}", op_sym, dest_reg, dest_reg, src_reg),
                format!("ALU {} operation on R{} and R{} -> R{}", op_sym, dest_reg, src_reg, dest_reg),
            )
        }
        "OP" => (
            format!("OP_GEMM R{}, R{}", dest_reg, src_reg),
            format!("Brain 2 Photonic MZI Optical Matrix Multiply: wave(R{}) -> R{}", src_reg, dest_reg),
        ),
        "WD" => (
            format!("WDM_PUMP R{}, R{}", dest_reg, src_reg),
            "Wave-Division Multiplexing photonic pump across optical channels".to_string(),
        ),
        "TT" => (
            format!("TILE_TRANS R{}, R{}", dest_reg, src_reg),
            format!("Brain 2 4x4 Systolic Tile Transpose: R{} -> R{}", src_reg, dest_reg),
        ),
        "MD" => {
            if mode == '.' || imm == 5 || imm == 6 {
                (
                    format!("TER_DOT R{}, R{}", dest_reg, src_reg),
                    format!("Brain 2 BitNet 1.58b 16-element subbyte ternary dot product: R{} . R{}", dest_reg, src_reg),
                )
            } else {
                (
                    format!("MUL R{}, R{}", dest_reg, src_reg),
                    format!("Integer product R{} * R{} -> R{}", dest_reg, src_reg, dest_reg),
                )
            }
        }
        "RF" => (
            format!("REV_SWAP R{}, R{}", dest_reg, src_reg),
            format!("Brain 3 Reversible Fredkin Swap: exchange R{} <-> R{} with 0 Landauer dissipation", dest_reg, src_reg),
        ),
        "TO" => (
            format!("REV_TOFFOLI R{}", dest_reg),
            "Brain 3 Reversible 3-bit Toffoli Controlled-NOT Gate".to_string(),
        ),
        "BK" => (
            format!("REV_POP R{}", dest_reg),
            "Brain 3 Reversible Thermodynamic Stack Pop (Zero-Memory AutoDiff)".to_string(),
        ),
        "ST" => (
            format!("STDP_UPDATE R{}, rate={}", dest_reg, imm * 16 + 4),
            "Brain 4 Neuromorphic Spike-Timing Dependent Plasticity weight adaptation".to_string(),
        ),
        "LI" => (
            format!("LIF_STEP R{}, thresh=50", dest_reg),
            "Brain 4 Leaky Integrate-and-Fire membrane potential step".to_string(),
        ),
        "LF" => (
            format!("LIF_SPIKE R{}", dest_reg),
            "Brain 4 Neuromorphic Action Potential Spike generation".to_string(),
        ),
        "TX" => (
            format!("NOC_SEND X+, R{}", src_reg),
            format!("4D Torus NoC packet transmit along X+ axis from R{}", src_reg),
        ),
        "RX" => (
            format!("NOC_RECV R{}, X-", dest_reg),
            format!("4D Torus NoC packet receive from X- axis into R{}", dest_reg),
        ),
        "SB" => (
            format!("NOC_BCAST R{}", src_reg),
            "4D Torus Spatial Broadcast: route packet across all 8 spatial neighbors".to_string(),
        ),
        "WH" => (
            format!("WORMHOLE R{}, R{}, dst=0x55", dest_reg, src_reg),
            "4D Torus Wormhole Hyper-Tunnel bypass route".to_string(),
        ),
        "bb" => (
            "CHIP_BARRIER".to_string(),
            "Global 256-Core Chip-Wide Hardware Lockstep Barrier".to_string(),
        ),
        "CC" => (
            "CACHE_INV".to_string(),
            "Chip-wide instruction and data cache invalidation".to_string(),
        ),
        "DD" => (
            format!("DMA_BURST R{}", src_reg),
            "High-speed Direct Memory Access burst transfer".to_string(),
        ),
        "EE" => (
            "DVFS_ECO".to_string(),
            "Energy-Aware Dynamic Voltage/Frequency Scaling: Switch to ECO mode (save 450uW)".to_string(),
        ),
        "11" => (
            "LASER_PUMP".to_string(),
            "Brain 2 Photonic Active Laser Strobe Pulse".to_string(),
        ),
        "88" | "RS" => (
            "ARENA_RESET".to_string(),
            "0-Cycle PGAS Regional Memory Arena Reset".to_string(),
        ),
        "99" | "SH" => (
            "SENTRY_WATCHDOG".to_string(),
            "Hardware Thermal Sentry Watchdog Arm / Trip".to_string(),
        ),
        "FU" => (
            "FUSE_START".to_string(),
            "Milestone #012: Streaming Kernel Fusion Start (Zero DRAM Writes / Lock Local SRAM)".to_string(),
        ),
        "FE" => (
            "FUSE_END".to_string(),
            "Milestone #012: Streaming Kernel Fusion End (Synchronize Local Streaming Pipeline)".to_string(),
        ),
        "SP" => (
            format!("FIBER_SPAWN R{}", src_reg),
            "Brain 6 Superposition Hardware Fiber Spawn (context saved from R0..R15)".to_string(),
        ),
        "FJ" => (
            format!("FIBER_JOIN R{}", dest_reg),
            "Brain 6 Superposition Hardware Fiber Await / Join".to_string(),
        ),
        "CS" => (
            format!("CAS R{}, R{}, imm={}", dest_reg, src_reg, imm),
            "Atomic Compare-And-Swap on memory word".to_string(),
        ),
        "PS" => (
            format!("SIMD_PREFIX R{}, R{}", dest_reg, src_reg),
            "Brain 1 SIMD Parallel Prefix-Sum scan".to_string(),
        ),
        "CD" => (
            format!("CORDIC_SINCOS R{}, R{}", dest_reg, src_reg),
            "Brain 1 CORDIC Trigonometric Sin/Cos vector rotation".to_string(),
        ),
        "RC" => {
            if mode == '!' {
                ("RESILIENT_CKPT".to_string(), "Brain 6 Fault-tolerant checkpoint save/restore".to_string())
            } else {
                (format!("CSR_READ R{}, csr={}", dest_reg, src_reg), "Read hardware performance Control/Status Register".to_string())
            }
        }
        "HL" => (
            "HALT".to_string(),
            "Core Execution Complete: enter quiescent low-power halt state".to_string(),
        ),

        _ => (
            format!("{}{} R{}, R{}", op, mode, dest_reg, src_reg),
            format!("Micro-op opcode {} on R{}, R{}", op, dest_reg, src_reg),
        ),
    };

    DisassembledSlot {
        raw: slot.to_string(),
        prefix,
        opcode: op,
        dest_bank,
        dest_reg,
        mode,
        src_reg,
        parity,
        imm,
        imm_val: None,
        mnemonic,
        description,
    }
}

/// Disassembles a full VliwInstruction bundle
pub fn disassemble_bundle(inst: &VliwInstruction) -> DisassembledBundle {
    let slots = inst.slots.iter().map(|s| disassemble_slot(s)).collect();
    let raw = format!("B{:04}: {}", inst.cycle, inst.slots.join(" "));
    DisassembledBundle {
        cycle: inst.cycle,
        raw,
        slots,
    }
}

/// Records a modified register diff between cycles
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterDiff {
    pub core_id: usize,
    pub reg: usize,
    pub old_val: u32,
    pub new_val: u32,
}

/// Debugger Stepping Result
#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    Stepped { cycle: usize },
    BreakpointHit { cycle: usize, breakpoint: Breakpoint },
    Halted { total_cycles: usize },
    NoMoreInstructions,
}

/// Interactive Microarchitectural Debugger
pub struct Debugger {
    pub sim: Simulator,
    pub disassembly: Vec<DisassembledBundle>,
    pub breakpoints: Vec<Breakpoint>,
    pub inspected_core: usize,
    pub current_cycle: usize,
    pub is_halted: bool,
    pub hit_breakpoint: Option<Breakpoint>,
    pub last_reg_diffs: Vec<RegisterDiff>,
    pub previous_registers: [u32; 16],
    pub step_history: Vec<(usize, [u32; 16], HardwareStats)>,
}

impl Debugger {
    pub fn new(cl_code: &str) -> Self {
        let mut sim = Simulator::new();
        sim.load_machine_code(cl_code);

        let disassembly: Vec<DisassembledBundle> = sim
            .instructions
            .iter()
            .map(disassemble_bundle)
            .collect();

        let initial_regs = sim.core_dump(0);

        Self {
            sim,
            disassembly,
            breakpoints: Vec::new(),
            inspected_core: 0,
            current_cycle: 0,
            is_halted: false,
            hit_breakpoint: None,
            last_reg_diffs: Vec::new(),
            previous_registers: initial_regs,
            step_history: Vec::new(),
        }
    }

    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, bp: Breakpoint) {
        if !self.breakpoints.contains(&bp) {
            self.breakpoints.push(bp);
        }
    }

    /// Remove a breakpoint by index
    pub fn remove_breakpoint(&mut self, idx: usize) -> Option<Breakpoint> {
        if idx < self.breakpoints.len() {
            Some(self.breakpoints.remove(idx))
        } else {
            None
        }
    }

    /// Clear all breakpoints
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    /// Switch inspected core ID (0..255)
    pub fn set_inspected_core(&mut self, core_id: usize) {
        if core_id < 256 {
            self.inspected_core = core_id;
            self.previous_registers = self.sim.core_dump(core_id);
            self.last_reg_diffs.clear();
        }
    }

    /// Check if any breakpoint is triggered on the upcoming bundle
    fn check_breakpoints_before_step(&self) -> Option<Breakpoint> {
        if self.sim.step_index >= self.sim.instructions.len() {
            return None;
        }

        let next_inst = &self.sim.instructions[self.sim.step_index];
        let current_cycle = next_inst.cycle;

        for bp in &self.breakpoints {
            match bp {
                Breakpoint::Cycle(c) => {
                    if *c == current_cycle {
                        return Some(bp.clone());
                    }
                }
                Breakpoint::Opcode(op) => {
                    for slot in &next_inst.slots {
                        if slot.len() >= 3 && (&slot[1..3] == op || slot.starts_with(op)) {
                            return Some(bp.clone());
                        }
                    }
                }
                Breakpoint::RegisterCond { core_id, reg, op, val } => {
                    let cur_val = self.sim.cores.get(*core_id)
                        .map(|c| c.registers[*reg])
                        .unwrap_or(0);
                    let matched = match op.as_str() {
                        "==" => cur_val == *val,
                        "!=" => cur_val != *val,
                        ">" => cur_val > *val,
                        "<" => cur_val < *val,
                        ">=" => cur_val >= *val,
                        "<=" => cur_val <= *val,
                        _ => false,
                    };
                    if matched {
                        return Some(bp.clone());
                    }
                }
                Breakpoint::Trap => {
                    for core in &self.sim.cores {
                        if core.in_trap || core.mcause != 0 {
                            return Some(bp.clone());
                        }
                    }
                }
                Breakpoint::Thermal(thresh) => {
                    for core in &self.sim.cores {
                        if core.thermal_level >= *thresh {
                            return Some(bp.clone());
                        }
                    }
                }
            }
        }
        None
    }

    /// Single-step one VLIW instruction bundle
    pub fn step(&mut self) -> StepResult {
        if self.is_halted || self.sim.step_index >= self.sim.instructions.len() {
            self.is_halted = true;
            return StepResult::NoMoreInstructions;
        }

        // Check if breakpoint matches
        if let Some(bp) = self.check_breakpoints_before_step() {
            // If we just hit this breakpoint and didn't move yet, skip to avoid getting stuck
            if self.hit_breakpoint.as_ref() != Some(&bp) {
                self.hit_breakpoint = Some(bp.clone());
                return StepResult::BreakpointHit {
                    cycle: self.current_cycle,
                    breakpoint: bp,
                };
            }
        }
        self.hit_breakpoint = None;

        // Snapshot previous register state for diffing
        let pre_regs = self.sim.core_dump(self.inspected_core);
        self.step_history.push((self.current_cycle, pre_regs, self.sim.stats.clone()));

        // Perform microarchitectural step in simulator
        let did_step = self.sim.step();
        if !did_step {
            self.is_halted = true;
            return StepResult::Halted {
                total_cycles: self.current_cycle,
            };
        }

        self.current_cycle += 1;

        // Compute register diffs for the inspected core
        let post_regs = self.sim.core_dump(self.inspected_core);
        self.last_reg_diffs.clear();
        for i in 0..16 {
            if pre_regs[i] != post_regs[i] {
                self.last_reg_diffs.push(RegisterDiff {
                    core_id: self.inspected_core,
                    reg: i,
                    old_val: pre_regs[i],
                    new_val: post_regs[i],
                });
            }
        }
        self.previous_registers = post_regs;

        // Check if halt occurred
        if self.sim.cores[self.inspected_core].is_halted || self.sim.step_index >= self.sim.instructions.len() {
            self.is_halted = true;
        }

        StepResult::Stepped {
            cycle: self.current_cycle,
        }
    }

    /// Continue execution until a breakpoint is hit or simulation halts
    pub fn continue_exec(&mut self) -> StepResult {
        if self.is_halted {
            return StepResult::Halted {
                total_cycles: self.current_cycle,
            };
        }

        // Step forward once past current position
        let first_res = self.step();
        match first_res {
            StepResult::BreakpointHit { .. } => return first_res,
            StepResult::Halted { .. } | StepResult::NoMoreInstructions => return first_res,
            StepResult::Stepped { .. } => {}
        }

        while !self.is_halted {
            if let Some(bp) = self.check_breakpoints_before_step() {
                self.hit_breakpoint = Some(bp.clone());
                return StepResult::BreakpointHit {
                    cycle: self.current_cycle,
                    breakpoint: bp,
                };
            }

            let res = self.step();
            match res {
                StepResult::BreakpointHit { .. } => return res,
                StepResult::Halted { .. } | StepResult::NoMoreInstructions => return res,
                StepResult::Stepped { .. } => {}
            }
        }

        StepResult::Halted {
            total_cycles: self.current_cycle,
        }
    }

    /// Restart simulation from Cycle 0 with current code and breakpoints preserved
    pub fn restart(&mut self, cl_code: &str) {
        let mut sim = Simulator::new();
        sim.load_machine_code(cl_code);
        self.current_cycle = 0;
        self.is_halted = false;
        self.hit_breakpoint = None;
        self.last_reg_diffs.clear();
        self.step_history.clear();
        self.previous_registers = sim.core_dump(self.inspected_core);
        self.sim = sim;
    }

    /// Read inspected core's 4D Torus coordinates
    pub fn get_core_coord(&self) -> Coord4D {
        Coord4D::from_core_id(self.inspected_core)
    }

    /// Read inspected core's photonic wave state
    pub fn get_core_wave(&self) -> PhotonicWave {
        if self.inspected_core < self.sim.cores.len() {
            self.sim.cores[self.inspected_core].wave_reg.clone()
        } else {
            PhotonicWave {
                amplitudes: [0; 4],
                phases: [0; 4],
            }
        }
    }

    /// Read inspected core's STDP synaptic weights
    pub fn get_core_stdp_weights(&self) -> [i8; 16] {
        if self.inspected_core < self.sim.cores.len() {
            self.sim.cores[self.inspected_core].stdp_weights
        } else {
            [0; 16]
        }
    }

    /// Read inspected core's thermal level in °C
    pub fn get_core_temp(&self) -> u32 {
        if self.inspected_core < self.sim.cores.len() {
            self.sim.cores[self.inspected_core].thermal_level
        } else {
            25
        }
    }

    /// Read inspected core's reversible stack depth
    pub fn get_core_rev_depth(&self) -> usize {
        if self.inspected_core < self.sim.cores.len() {
            self.sim.cores[self.inspected_core].reversible_stack.len()
        } else {
            0
        }
    }
}
