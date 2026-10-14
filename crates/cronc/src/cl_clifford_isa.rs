//! cl_clifford_isa.rs
//! Pure native Rust implementation of the Clifford Rotor 4-Slot VLIW ISA & Assembler.
//! Replaces clifford_compiler.py and clifford_rotor_isa.py without Python runtime.


pub const BASE94_OFFSET: u8 = 33; // ASCII '!'
pub const BASE94_COUNT: u8 = 94; // '!' to '~'
pub const VLIW_SLOT_LEN: usize = 10;
pub const VLIW_BUNDLE_LEN: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroOpCode {
    Nop = 0,
    CliffordAdd = 1,
    CliffordMul = 2,
    RotSandwich = 3,
    TropicMax = 4,
    LayerShift = 5,
    StdpLearn = 6,
    WaveSpread = 7,
    EmitChar = 8,
    BranchZero = 9,
}

impl MicroOpCode {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Nop => "NOP",
            Self::CliffordAdd => "CLIFFORD_ADD",
            Self::CliffordMul => "CLIFFORD_MUL",
            Self::RotSandwich => "ROT_SANDWICH",
            Self::TropicMax => "TROPIC_MAX",
            Self::LayerShift => "LAYER_SHIFT",
            Self::StdpLearn => "STDP_LEARN",
            Self::WaveSpread => "WAVE_SPREAD",
            Self::EmitChar => "EMIT_CHAR",
            Self::BranchZero => "BRANCH_ZERO",
        }
    }

    pub fn from_mnemonic(m: &str) -> Option<Self> {
        match m.to_uppercase().as_str() {
            "NOP" => Some(Self::Nop),
            "CLIFFORD_ADD" | "ADD" => Some(Self::CliffordAdd),
            "CLIFFORD_MUL" | "MUL" => Some(Self::CliffordMul),
            "ROT_SANDWICH" | "ROTOR" => Some(Self::RotSandwich),
            "TROPIC_MAX" | "MAX" => Some(Self::TropicMax),
            "LAYER_SHIFT" | "SHIFT" => Some(Self::LayerShift),
            "STDP_LEARN" | "STDP" => Some(Self::StdpLearn),
            "WAVE_SPREAD" | "WAVE" => Some(Self::WaveSpread),
            "EMIT_CHAR" | "EMIT" => Some(Self::EmitChar),
            "BRANCH_ZERO" | "BRZ" => Some(Self::BranchZero),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MicroOp {
    pub opcode: MicroOpCode,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i16,
}

impl Default for MicroOp {
    fn default() -> Self {
        Self {
            opcode: MicroOpCode::Nop,
            rd: 0,
            rs1: 0,
            rs2: 0,
            imm: 0,
        }
    }
}

impl MicroOp {
    pub fn pack_64(&self) -> u64 {
        let op = (self.opcode as u64) & 0x0F;
        let rd = (self.rd as u64) & 0x3F;
        let rs1 = (self.rs1 as u64) & 0x3F;
        let rs2 = (self.rs2 as u64) & 0x3F;
        let imm = (self.imm as u16 as u64) & 0xFFFF;
        op | (rd << 4) | (rs1 << 10) | (rs2 << 16) | (imm << 22)
    }

    pub fn unpack_64(val: u64) -> Self {
        let op_raw = (val & 0x0F) as u8;
        let rd = ((val >> 4) & 0x3F) as u8;
        let rs1 = ((val >> 10) & 0x3F) as u8;
        let rs2 = ((val >> 16) & 0x3F) as u8;
        let imm = ((val >> 22) & 0xFFFF) as i16;
        let opcode = match op_raw {
            1 => MicroOpCode::CliffordAdd,
            2 => MicroOpCode::CliffordMul,
            3 => MicroOpCode::RotSandwich,
            4 => MicroOpCode::TropicMax,
            5 => MicroOpCode::LayerShift,
            6 => MicroOpCode::StdpLearn,
            7 => MicroOpCode::WaveSpread,
            8 => MicroOpCode::EmitChar,
            9 => MicroOpCode::BranchZero,
            _ => MicroOpCode::Nop,
        };
        Self { opcode, rd, rs1, rs2, imm }
    }

    pub fn to_base94_slot(&self) -> String {
        let val = self.pack_64();
        let mut n = val;
        let mut chars = Vec::with_capacity(VLIW_SLOT_LEN);
        for _ in 0..VLIW_SLOT_LEN {
            let rem = (n % (BASE94_COUNT as u64)) as u8;
            chars.push((rem + BASE94_OFFSET) as char);
            n /= BASE94_COUNT as u64;
        }
        chars.into_iter().collect()
    }

    pub fn from_base94_slot(slot_str: &str) -> Option<Self> {
        if slot_str.len() < VLIW_SLOT_LEN {
            return None;
        }
        let mut val: u64 = 0;
        let mut mult: u64 = 1;
        for (i, c) in slot_str[..VLIW_SLOT_LEN].chars().enumerate() {
            let b = c as u8;
            if !(BASE94_OFFSET..=126).contains(&b) {
                return None;
            }
            let rem = (b - BASE94_OFFSET) as u64;
            val = val.checked_add(rem.checked_mul(mult)?)?;
            if i + 1 < VLIW_SLOT_LEN {
                mult = mult.checked_mul(BASE94_COUNT as u64)?;
            }
        }
        Some(Self::unpack_64(val))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CliffordBundle {
    pub slot0: MicroOp,
    pub slot1: MicroOp,
    pub slot2: MicroOp,
    pub slot3: MicroOp,
}

impl CliffordBundle {
    pub fn new(slot0: MicroOp, slot1: MicroOp, slot2: MicroOp, slot3: MicroOp) -> Self {
        Self { slot0, slot1, slot2, slot3 }
    }

    pub fn to_ascii_bundle(&self) -> String {
        format!(
            "{}{}{}{}",
            self.slot0.to_base94_slot(),
            self.slot1.to_base94_slot(),
            self.slot2.to_base94_slot(),
            self.slot3.to_base94_slot()
        )
    }

    pub fn from_ascii_bundle(bundle_str: &str) -> Option<Self> {
        let trimmed = bundle_str.trim();
        if trimmed.len() < VLIW_BUNDLE_LEN {
            return None;
        }
        let s0 = MicroOp::from_base94_slot(&trimmed[0..10])?;
        let s1 = MicroOp::from_base94_slot(&trimmed[10..20])?;
        let s2 = MicroOp::from_base94_slot(&trimmed[20..30])?;
        let s3 = MicroOp::from_base94_slot(&trimmed[30..40])?;
        Some(Self { slot0: s0, slot1: s1, slot2: s2, slot3: s3 })
    }
}

pub struct CliffordRotorEmulator {
    pub regs: [i64; 64],
    pub emitted_chars: Vec<char>,
    pub cycle_count: u64,
}

impl Default for CliffordRotorEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl CliffordRotorEmulator {
    pub fn new() -> Self {
        Self {
            regs: [0; 64],
            emitted_chars: Vec::new(),
            cycle_count: 0,
        }
    }

    pub fn execute_micro_op(&mut self, op: MicroOp) {
        let rs1_val = self.regs[(op.rs1 as usize) & 0x3F];
        let rs2_val = self.regs[(op.rs2 as usize) & 0x3F];
        let rd_idx = (op.rd as usize) & 0x3F;

        match op.opcode {
            MicroOpCode::Nop => {}
            MicroOpCode::CliffordAdd => {
                let res = rs1_val.wrapping_add(rs2_val).wrapping_add(op.imm as i64);
                if rd_idx != 0 {
                    self.regs[rd_idx] = res;
                }
            }
            MicroOpCode::CliffordMul => {
                let res = rs1_val.wrapping_mul(rs2_val);
                if rd_idx != 0 {
                    self.regs[rd_idx] = res;
                }
            }
            MicroOpCode::RotSandwich => {
                // Approximate 4D Rotor rotation without transcendental trig: (v * rot) >> 13
                let factor = (op.imm as i64).max(1);
                let rotated = (rs1_val * factor) >> 13;
                if rd_idx != 0 {
                    self.regs[rd_idx] = rotated;
                }
            }
            MicroOpCode::TropicMax => {
                let m = rs1_val.max(rs2_val);
                if rd_idx != 0 {
                    self.regs[rd_idx] = m;
                }
            }
            MicroOpCode::LayerShift => {
                let shifted = rs1_val << (op.imm.clamp(0, 16) as u32);
                if rd_idx != 0 {
                    self.regs[rd_idx] = shifted;
                }
            }
            MicroOpCode::StdpLearn => {
                // Hebbian weight update: W += sign(pre * post)
                let dw = if (rs1_val > 0 && rs2_val > 0) || (rs1_val < 0 && rs2_val < 0) { 1 } else { -1 };
                if rd_idx != 0 {
                    self.regs[rd_idx] = self.regs[rd_idx].saturating_add(dw);
                }
            }
            MicroOpCode::WaveSpread => {
                let wave = (rs1_val + rs2_val) / 2;
                if rd_idx != 0 {
                    self.regs[rd_idx] = wave;
                }
            }
            MicroOpCode::EmitChar => {
                let ascii_code = ((op.imm as u8).wrapping_add(BASE94_OFFSET)) as char;
                self.emitted_chars.push(ascii_code);
            }
            MicroOpCode::BranchZero => {
                // Handled in pipeline control
            }
        }
    }

    pub fn execute_bundle(&mut self, bundle: &CliffordBundle) {
        self.cycle_count += 1;
        self.execute_micro_op(bundle.slot0);
        self.execute_micro_op(bundle.slot1);
        self.execute_micro_op(bundle.slot2);
        self.execute_micro_op(bundle.slot3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_micro_op_roundtrip() {
        let op = MicroOp {
            opcode: MicroOpCode::CliffordAdd,
            rd: 5,
            rs1: 2,
            rs2: 3,
            imm: -42,
        };
        let slot_str = op.to_base94_slot();
        assert_eq!(slot_str.len(), 10);
        let parsed = MicroOp::from_base94_slot(&slot_str).expect("Decode slot");
        assert_eq!(op.opcode, parsed.opcode);
        assert_eq!(op.rd, parsed.rd);
        assert_eq!(op.rs1, parsed.rs1);
        assert_eq!(op.rs2, parsed.rs2);
        assert_eq!(op.imm, parsed.imm);
    }

    #[test]
    fn test_bundle_execution_emit_ok() {
        let mut emu = CliffordRotorEmulator::new();
        let b1 = CliffordBundle::new(
            MicroOp { opcode: MicroOpCode::CliffordAdd, rd: 1, rs1: 0, rs2: 0, imm: 100 },
            MicroOp { opcode: MicroOpCode::RotSandwich, rd: 2, rs1: 1, rs2: 0, imm: 8192 },
            MicroOp { opcode: MicroOpCode::TropicMax, rd: 3, rs1: 1, rs2: 2, imm: 0 },
            MicroOp { opcode: MicroOpCode::EmitChar, rd: 0, rs1: 0, rs2: 0, imm: (b'O' - 33) as i16 },
        );
        let b2 = CliffordBundle::new(
            MicroOp::default(),
            MicroOp::default(),
            MicroOp::default(),
            MicroOp { opcode: MicroOpCode::EmitChar, rd: 0, rs1: 0, rs2: 0, imm: (b'K' - 33) as i16 },
        );

        emu.execute_bundle(&b1);
        emu.execute_bundle(&b2);

        let emitted: String = emu.emitted_chars.into_iter().collect();
        assert_eq!(emitted, "OK");
        assert_eq!(emu.cycle_count, 2);
    }
}
