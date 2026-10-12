// ============================================================================
// CRON Sovereign Machine Code Canonical Intermediate Representation (Micro-Op IR)
// Module: cronc::cl_ir (crates/cronc/src/cl_ir.rs)
//
// Converts raw 10-char ASCII VLIW slots and 128-bit .clb binary words into a
// unified, strongly-typed Abstract Micro-Op stream for execution across
// JIT, C23 Native AOT, LLVM, and Verilog RTL backends.
// ============================================================================

use crate::cl_binary::{assemble_cl_to_clb, disassemble_clb_to_cl};
use crate::cl_lang::{parse_slot, ClDirective, ClSlot, ClWeightBinding};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AluOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    CountOnes,
    Exp,
    Sqrt,
    FixedPoint,
    Bitmask,
    Compare,
    Copy,
    SubbyteTernaryDot,
    TileTranspose,
    CordicRotate,
    PrngStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOp {
    LoadWord,
    StoreWord,
    LoadByte,
    StoreByte,
    BankLoad,
    BankStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchCond {
    Always,
    Zero,
    NonZero,
    Less,
    Greater,
    Call,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NeuroOp {
    LifStep,
    LifSpike,
    StdpUpdate,
    OpticalGemm,
    ReversibleSwap,
    ReversibleToffoli,
    ReversibleInvert,
    SsmStep,
    RmsNorm,
    Softmax,
    Silu,
    Gelu,
    WaveResonance,
    CurvedDeflect,
    Homeostasis,
    LayerTransition,
    PhaseToken,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClMicroOp {
    Nop,
    Alu {
        op: AluOp,
        dest: u8,
        src: u8,
        mode: char,
        imm: u32,
    },
    ImmediateLoad {
        dest: u8,
        val: u32,
    },
    SphereGeodesic {
        head: u8,
        w: u32,
        lat_x: u32,
        lon_z: u32,
        quad: u8,
        step: u32,
        alu_op: char,
        reg: u8,
        val: u32,
        is_err: bool,
        is_rd: bool,
        is_wr: bool,
    },
    Memory {
        op: MemOp,
        dest: u8,
        src: u8,
        offset: u32,
        mode: char,
    },
    Branch {
        cond: BranchCond,
        reg: u8,
        target_bundle: usize,
    },
    Neuromorphic {
        op: NeuroOp,
        dest: u8,
        src: u8,
    },
    Sync {
        is_barrier: bool,
        is_halt: bool,
        is_yield: bool,
    },
    Print {
        reg: u8,
        imm_char: Option<char>,
    },
    Raw {
        raw: String,
    },
}

pub type MicroOp = ClMicroOp;

impl MicroOp {
    pub fn from_slot_str(slot_str: &str) -> Self {
        if slot_str.is_empty() || slot_str.starts_with("__NOP") || slot_str.contains("NO00") {
            return MicroOp::Nop;
        }

        // 4D Spherical Geodesic Slot: prefix '`'
        if slot_str.starts_with('`') && slot_str.len() == 10 {
            return Self::parse_sphere_geodesic_slot(slot_str);
        }

        let slot = match parse_slot(slot_str) {
            Ok(s) => s,
            Err(_) => return MicroOp::Raw { raw: slot_str.to_string() },
        };

        Self::from_parsed_slot(&slot, slot_str)
    }

    fn parse_sphere_geodesic_slot(slot_str: &str) -> Self {
        let chars: Vec<char> = slot_str.chars().collect();
        let decode_b64 = |c: char| -> u8 {
            match c {
                'A'..='Z' => c as u8 - b'A',
                'a'..='z' => c as u8 - b'a' + 26,
                '0'..='9' => c as u8 - b'0' + 52,
                '+' | '-' => 62,
                '/' | '_' => 63,
                _ => 0,
            }
        };
        const BASE90: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~";
        let decode_b90 = |c: char| -> u8 {
            BASE90.iter().position(|&b| b == c as u8).unwrap_or(0) as u8
        };

        let mode_val = decode_b64(chars[1]);
        let s_id = ((mode_val >> 4) & 0x3) as u8;
        let is_err = ((mode_val >> 3) & 1) == 1;
        let is_rd = ((mode_val >> 1) & 1) == 1;
        let is_wr = (mode_val & 1) == 1;

        let w = decode_b90(chars[2]) as u32;
        let lat_x = decode_b90(chars[3]) as u32;
        let lon_z = decode_b90(chars[4]) as u32;
        let quad = match chars[5] {
            '1' | 'N' | 'A' => 1,
            '2' | 'W' | 'B' => 2,
            '3' | 'S' | 'C' => 3,
            '4' | 'E' | 'D' => 4,
            _ => 1,
        };
        let step = decode_b90(chars[6]) as u32;
        let alu_op = chars[7];
        let reg = chars[8].to_digit(16).unwrap_or(s_id as u32) as u8 % 16;
        let val = chars[9].to_digit(16).unwrap_or(step % 16);

        MicroOp::SphereGeodesic {
            head: s_id,
            w,
            lat_x,
            lon_z,
            quad,
            step,
            alu_op,
            reg,
            val,
            is_err,
            is_rd,
            is_wr,
        }
    }

    pub fn from_parsed_slot(slot: &ClSlot, raw_str: &str) -> Self {
        let d = slot.dest_reg.unwrap_or(0) as u8 % 16;
        let s = slot.src_reg.unwrap_or(0) as u8 % 16;
        let imm_nibble = slot.imm_token.to_digit(16).unwrap_or(0);

        let imm_val = crate::cl_lang::parse_slot_immediate(raw_str).unwrap_or(imm_nibble);

        // Immediate Load
        if slot.prefix == '\'' || slot.opcode == "==" || slot.opcode == "=0" || slot.opcode == "=1" {
            return MicroOp::ImmediateLoad { dest: d, val: imm_val };
        }

        match slot.opcode.as_str() {
            "NO" => MicroOp::Nop,
            "AD" => MicroOp::Alu { op: AluOp::Add, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "SU" | "SB" if slot.mode == '-' || slot.mode == 'S' || slot.mode == 'M' => {
                MicroOp::Alu { op: AluOp::Sub, dest: d, src: s, mode: slot.mode, imm: imm_val }
            }
            "ML" => MicroOp::Alu { op: AluOp::Mul, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "DV" => MicroOp::Alu { op: AluOp::Div, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "EX" => MicroOp::Alu { op: AluOp::Exp, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "SQ" => MicroOp::Alu { op: AluOp::Sqrt, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "FX" => MicroOp::Alu { op: AluOp::FixedPoint, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "CP" => MicroOp::Alu { op: AluOp::Copy, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "XO" => MicroOp::Alu { op: AluOp::Xor, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "MA" => MicroOp::Alu { op: AluOp::Bitmask, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "CO" => MicroOp::Alu { op: AluOp::Compare, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "MD" => MicroOp::Alu { op: AluOp::SubbyteTernaryDot, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "TT" => MicroOp::Alu { op: AluOp::TileTranspose, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "CD" => MicroOp::Alu { op: AluOp::CordicRotate, dest: d, src: s, mode: slot.mode, imm: imm_val },
            "RN" | "GF" => MicroOp::Alu { op: AluOp::PrngStep, dest: d, src: s, mode: slot.mode, imm: imm_val },

            // Memory
            "LD" => {
                if slot.mode == '@' || slot.mode == ':' {
                    MicroOp::Memory { op: MemOp::BankLoad, dest: d, src: s, offset: imm_val, mode: slot.mode }
                } else {
                    MicroOp::Memory { op: MemOp::LoadWord, dest: d, src: s, offset: imm_val, mode: slot.mode }
                }
            }
            "SR" => MicroOp::Memory { op: MemOp::StoreWord, dest: d, src: s, offset: imm_val, mode: slot.mode },
            "LB" => MicroOp::Memory { op: MemOp::LoadByte, dest: d, src: s, offset: imm_val, mode: slot.mode },
            "BY" => MicroOp::Memory { op: MemOp::StoreByte, dest: d, src: s, offset: imm_val, mode: slot.mode },

            // Branches
            "JP" => MicroOp::Branch { cond: BranchCond::Always, reg: d, target_bundle: imm_val as usize },
            "BZ" => MicroOp::Branch { cond: BranchCond::Zero, reg: d, target_bundle: imm_val as usize },
            "BN" => MicroOp::Branch { cond: BranchCond::NonZero, reg: d, target_bundle: imm_val as usize },
            "BL" | "JL" | "LT" => MicroOp::Branch { cond: BranchCond::Less, reg: d, target_bundle: imm_val as usize },
            "BG" => MicroOp::Branch { cond: BranchCond::Greater, reg: d, target_bundle: imm_val as usize },
            "JS" => MicroOp::Branch { cond: BranchCond::Call, reg: d, target_bundle: imm_val as usize },
            "RT" => MicroOp::Branch { cond: BranchCond::Return, reg: d, target_bundle: 0 },

            // Neuromorphic & AI ISA
            "OP" | "WD" => MicroOp::Neuromorphic { op: NeuroOp::OpticalGemm, dest: d, src: s },
            "ST" => MicroOp::Neuromorphic { op: NeuroOp::StdpUpdate, dest: d, src: s },
            "LI" => MicroOp::Neuromorphic { op: NeuroOp::LifStep, dest: d, src: s },
            "LF" => MicroOp::Neuromorphic { op: NeuroOp::LifSpike, dest: d, src: s },
            "RF" => MicroOp::Neuromorphic { op: NeuroOp::ReversibleSwap, dest: d, src: s },
            "TO" => MicroOp::Neuromorphic { op: NeuroOp::ReversibleToffoli, dest: d, src: s },
            "BK" => MicroOp::Neuromorphic { op: NeuroOp::ReversibleInvert, dest: d, src: s },
            "RM" => MicroOp::Neuromorphic { op: NeuroOp::RmsNorm, dest: d, src: s },
            "SM" => MicroOp::Neuromorphic { op: NeuroOp::Softmax, dest: d, src: s },
            "SI" => MicroOp::Neuromorphic { op: NeuroOp::Silu, dest: d, src: s },
            "GE" => MicroOp::Neuromorphic { op: NeuroOp::Gelu, dest: d, src: s },
            "SS" => MicroOp::Neuromorphic { op: NeuroOp::SsmStep, dest: d, src: s },
            "TP" => MicroOp::Neuromorphic { op: NeuroOp::PhaseToken, dest: d, src: s },
            "WF" => MicroOp::Neuromorphic { op: NeuroOp::WaveResonance, dest: d, src: s },
            "CN" => MicroOp::Neuromorphic { op: NeuroOp::CurvedDeflect, dest: d, src: s },
            "HP" => MicroOp::Neuromorphic { op: NeuroOp::Homeostasis, dest: d, src: s },
            "LY" => MicroOp::Neuromorphic { op: NeuroOp::LayerTransition, dest: d, src: s },

            // Sync
            "bb" | "BB" => MicroOp::Sync { is_barrier: true, is_halt: false, is_yield: false },
            "HL" => MicroOp::Sync { is_barrier: false, is_halt: true, is_yield: false },
            "YD" => MicroOp::Sync { is_barrier: false, is_halt: false, is_yield: true },

            // Hardware Console / Print
            "PR" => {
                let imm_c = if slot.mode == '#' {
                    let hex: String = raw_str.chars().skip(6).take(2).filter(|c| c.is_ascii_hexdigit()).collect();
                    u8::from_str_radix(&hex, 16).ok().map(|b| b as char)
                } else {
                    None
                };
                MicroOp::Print { reg: s, imm_char: imm_c }
            }

            _ => MicroOp::Raw { raw: raw_str.to_string() },
        }
    }

    pub fn to_slot_string(&self) -> String {
        match self {
            MicroOp::Nop => "_NO00$000>".to_string(),
            MicroOp::ImmediateLoad { dest, val } => format!("'==0{:1X}#{:04X}>", dest, val & 0xFFFF),
            MicroOp::Alu { op, dest, src, mode, imm } => {
                let op_str = match op {
                    AluOp::Add => "AD",
                    AluOp::Sub => "SU",
                    AluOp::Mul => "ML",
                    AluOp::Div => "DV",
                    AluOp::Mod => "PO",
                    AluOp::And => "PO",
                    AluOp::Or => "PO",
                    AluOp::Xor => "XO",
                    AluOp::Shl => "PO",
                    AluOp::Shr => "PO",
                    AluOp::CountOnes => "PO",
                    AluOp::Exp => "EX",
                    AluOp::Sqrt => "SQ",
                    AluOp::FixedPoint => "FX",
                    AluOp::Bitmask => "MA",
                    AluOp::Compare => "CO",
                    AluOp::Copy => "CP",
                    AluOp::SubbyteTernaryDot => "MD",
                    AluOp::TileTranspose => "TT",
                    AluOp::CordicRotate => "CD",
                    AluOp::PrngStep => "RN",
                };
                format!("_{}{:02X}{}{:1X}0{:1X}>", op_str, dest, mode, src, imm & 0xF)
            }
            MicroOp::Branch { cond, reg, target_bundle } => {
                let op_str = match cond {
                    BranchCond::Always => "JP",
                    BranchCond::Zero => "BZ",
                    BranchCond::NonZero => "BN",
                    BranchCond::Less => "JL",
                    BranchCond::Greater => "BG",
                    BranchCond::Call => "JS",
                    BranchCond::Return => "RT",
                };
                format!("!{}{:02X}#{:04X}>", op_str, reg, target_bundle)
            }
            MicroOp::Memory { op, dest, src, offset, mode } => {
                let op_str = match op {
                    MemOp::LoadWord => "LD",
                    MemOp::StoreWord => "SR",
                    MemOp::LoadByte => "LB",
                    MemOp::StoreByte => "BY",
                    MemOp::BankLoad | MemOp::BankStore => "LD",
                };
                format!("_{}{:02X}{}{:1X}0{:1X}>", op_str, dest, mode, src, offset & 0xF)
            }
            MicroOp::Neuromorphic { op, dest, src } => {
                let op_str = match op {
                    NeuroOp::LifStep => "LI",
                    NeuroOp::LifSpike => "LF",
                    NeuroOp::StdpUpdate => "ST",
                    NeuroOp::OpticalGemm => "OP",
                    NeuroOp::ReversibleSwap => "RF",
                    NeuroOp::ReversibleToffoli => "TO",
                    NeuroOp::ReversibleInvert => "BK",
                    NeuroOp::RmsNorm => "RM",
                    NeuroOp::Softmax => "SM",
                    NeuroOp::Silu => "SI",
                    NeuroOp::Gelu => "GE",
                    NeuroOp::SsmStep => "SS",
                    NeuroOp::PhaseToken => "TP",
                    NeuroOp::WaveResonance => "WF",
                    NeuroOp::CurvedDeflect => "CN",
                    NeuroOp::Homeostasis => "HP",
                    NeuroOp::LayerTransition => "LY",
                };
                format!("_{}{:02X}$0{:1X}00>", op_str, dest, src)
            }
            MicroOp::Sync { is_barrier, is_halt, is_yield } => {
                if *is_halt {
                    "!HL00#0000>".to_string()
                } else if *is_barrier {
                    "_bb00#0000>".to_string()
                } else if *is_yield {
                    "_YD00#0000>".to_string()
                } else {
                    "_NO00$000>".to_string()
                }
            }
            MicroOp::Print { reg, imm_char } => {
                if let Some(c) = imm_char {
                    format!("_PR00#{:02X}00>", *c as u8)
                } else {
                    format!("_PR00$0{:1X}00>", reg)
                }
            }
            MicroOp::SphereGeodesic { head, alu_op, reg, val, .. } => {
                format!("`{:1X}00000{}{:1X}{:1X}>", head, alu_op, reg, val)
            }
            MicroOp::Raw { raw } => raw.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClBundleIR {
    pub cycle: usize,
    pub ops: [MicroOp; 4],
}

impl ClBundleIR {
    pub fn new(cycle: usize, ops: [MicroOp; 4]) -> Self {
        Self { cycle, ops }
    }

    pub fn to_cl_line(&self) -> String {
        format!(
            "B{:04}: {} {} {} {}",
            self.cycle,
            self.ops[0].to_slot_string(),
            self.ops[1].to_slot_string(),
            self.ops[2].to_slot_string(),
            self.ops[3].to_slot_string()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClProgramIR {
    pub bundles: Vec<ClBundleIR>,
    pub weights: Vec<ClWeightBinding>,
    pub directives: Vec<ClDirective>,
}

impl ClProgramIR {
    pub fn new() -> Self {
        Self::default()
    }

    /// Lower textual .cl code into strongly-typed canonical ClProgramIR
    pub fn from_cl_text(source: &str) -> Result<Self, String> {
        let mut bundles = Vec::new();
        let mut weights = Vec::new();
        let mut directives = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.starts_with(".weights") {
                if let Ok(w) = crate::cl_lang::parse_weights_directive(trimmed) {
                    weights.push(w.clone());
                    directives.push(ClDirective::Weights(w));
                }
                continue;
            }

            if trimmed.starts_with('.') || trimmed.starts_with('@') {
                directives.push(ClDirective::Custom {
                    name: trimmed.to_string(),
                    content: String::new(),
                });
                continue;
            }

            if let Some((cycle_part, slots_part)) = trimmed.split_once(':') {
                if !cycle_part.trim().starts_with('B') && !cycle_part.trim().starts_with('b') {
                    continue;
                }
                let cycle_str = cycle_part.trim().trim_start_matches(|c| c == 'B' || c == 'b');
                let cycle_num = cycle_str.parse::<usize>().unwrap_or(bundles.len());

                let tokens: Vec<&str> = slots_part.split_whitespace().collect();
                let mut ops = [
                    MicroOp::Nop,
                    MicroOp::Nop,
                    MicroOp::Nop,
                    MicroOp::Nop,
                ];

                for (idx, slot_str) in tokens.iter().take(4).enumerate() {
                    ops[idx] = MicroOp::from_slot_str(slot_str);
                }

                bundles.push(ClBundleIR::new(cycle_num, ops));
            }
        }

        Ok(ClProgramIR {
            bundles,
            weights,
            directives,
        })
    }

    /// Read 128-bit binary bytecode (.clb format) and parse directly into ClProgramIR
    pub fn from_clb(bytes: &[u8]) -> Result<Self, String> {
        let cl_text = disassemble_clb_to_cl(bytes)?;
        Self::from_cl_text(&cl_text)
    }

    /// Render IR back to canonical formatted .cl source text
    pub fn to_cl_text(&self) -> String {
        let mut out = String::new();
        for dir in &self.directives {
            if let ClDirective::Weights(ref w) = dir {
                out.push_str(&format!(".weights bank={}, offset={}: {:?}\n", w.bank, w.offset, w.values));
            }
        }
        for bundle in &self.bundles {
            out.push_str(&bundle.to_cl_line());
            out.push('\n');
        }
        out
    }

    /// Assemble IR directly into 128-bit VLIW binary bytecode (.clb)
    pub fn to_clb(&self) -> Result<Vec<u8>, String> {
        let cl_text = self.to_cl_text();
        assemble_cl_to_clb(&cl_text)
    }

    pub fn total_cycles(&self) -> usize {
        self.bundles.len()
    }

    pub fn total_slots(&self) -> usize {
        self.bundles.len() * 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_micro_op_ir_lowering_and_roundtrip() {
        let code = r#"
            B0000: '==01#0400 '==02#0200 _AD03M1020 _SU04M1020
            B0001: _ML05M3040 _CP06M5000 _MA07M6000 !JL01#0003
            B0002: _LD08M0010 _RM09$0000 _ST0AL1000 !HL00#0000
        "#;

        let ir = ClProgramIR::from_cl_text(code).expect("IR lowering should succeed");
        assert_eq!(ir.bundles.len(), 3);
        assert_eq!(ir.total_slots(), 12);

        // Verify first bundle operations
        match &ir.bundles[0].ops[0] {
            MicroOp::ImmediateLoad { dest, val } => {
                assert_eq!(*dest, 1);
                assert_eq!(*val, 0x40);
            }
            other => panic!("Expected ImmediateLoad, got {:?}", other),
        }

        match &ir.bundles[0].ops[2] {
            MicroOp::Alu { op, dest, src, .. } => {
                assert_eq!(*op, AluOp::Add);
                assert_eq!(*dest, 3);
                assert_eq!(*src, 1);
            }
            other => panic!("Expected Add AluOp, got {:?}", other),
        }

        match &ir.bundles[0].ops[3] {
            MicroOp::Alu { op, dest, .. } => {
                assert_eq!(*op, AluOp::Sub);
                assert_eq!(*dest, 4);
            }
            other => panic!("Expected Sub AluOp, got {:?}", other),
        }

        // Verify binary round-trip from IR
        let clb = ir.to_clb().expect("IR to CLB assembly should succeed");
        let ir2 = ClProgramIR::from_clb(&clb).expect("CLB to IR disassembly should succeed");
        assert_eq!(ir2.bundles.len(), 3);
    }
}
