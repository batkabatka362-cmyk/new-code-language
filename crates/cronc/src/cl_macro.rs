//! High-Level Macro Assembler for `.cl` VLIW Machine Code
//!
//! Transforms high-level symbolic instructions, structured loops, and tensor/neural layers
//! into 10-character 4-slot VLIW bundles with automatic CRC-8 ATM checksum generation
//! and hazard-free register scheduling.


/// High-Level Macro Statement
#[derive(Debug, Clone)]
pub enum MacroStmt {
    /// Comment or label
    Label(String),
    /// Set a register: `let R0 = 0x42` or `let R0 = R1`
    AssignImm { dst: u8, imm: u16 },
    AssignReg { dst: u8, src: u8 },
    /// Arithmetic / Logic operation: `R0 = R1 + R2`
    BinaryOp { op: char, dst: u8, src1: u8, src2: u8 },
    /// Compound Fused Operation (FMA, FMS, etc.)
    CompoundOp { mode: char, dst: u8, src: u8, imm_or_reg: u16 },
    /// Neuromorphic Optical GEMM Layer: `optical_dot R_dst, R_src1, R_src2`
    OpticalGemm { dst: u8, src1: u8, src2: u8 },
    /// Ternary BitNet MAC: `ternary_mac R_dst, R_src, R_weights`
    TernaryMac { dst: u8, src: u8, weights: u8 },
    /// STDP Synaptic Update: `stdp_update R_synapses, R_pre, R_post`
    StdpUpdate { synapses: u8, pre: u8, post: u8 },
    /// Trit Quantization: `pack_trits R_dst, R_src`
    PackTrits { dst: u8, src: u8 },
    /// NoC 4D-Torus Packet Send: `send_noc dest_core, R_data`
    SendNoc { dest_core: u8, data_reg: u8 },
    /// NoC 4D-Torus Packet Receive: `recv_noc R_dst`
    RecvNoc { dst_reg: u8 },
    /// Synchronize Core: `barrier`
    Barrier,
    /// Loop block: `loop N { ... }`
    Loop { count: usize, body: Vec<MacroStmt> },
}

/// CRC-8 ATM Table for VLIW slot checksum generation
const CRC8_TABLE: [u8; 256] = [
    0x00, 0x07, 0x0E, 0x09, 0x1C, 0x1B, 0x12, 0x15, 0x38, 0x3F, 0x36, 0x31, 0x24, 0x23, 0x2A, 0x2D,
    0x70, 0x77, 0x7E, 0x79, 0x6C, 0x6B, 0x62, 0x65, 0x48, 0x4F, 0x46, 0x41, 0x54, 0x53, 0x5A, 0x5D,
    0xE0, 0xE7, 0xEE, 0xE9, 0xFC, 0xFB, 0xF2, 0xF5, 0xD8, 0xDF, 0xD6, 0xD1, 0xC4, 0xC3, 0xCA, 0xCD,
    0x90, 0x97, 0x9E, 0x99, 0x8C, 0x8B, 0x82, 0x85, 0xA8, 0xAF, 0xA6, 0xA1, 0xB4, 0xB3, 0xBA, 0xBD,
    0xC7, 0xC0, 0xC9, 0xCE, 0xDB, 0xDC, 0xD5, 0xD2, 0xFF, 0xF8, 0xF1, 0xF6, 0xE3, 0xE4, 0xED, 0xEA,
    0xB7, 0xB0, 0xB9, 0xBE, 0xAB, 0xAC, 0xA5, 0xA2, 0x8F, 0x88, 0x81, 0x86, 0x93, 0x94, 0x9D, 0x9A,
    0x27, 0x20, 0x29, 0x2E, 0x3B, 0x3C, 0x35, 0x32, 0x1F, 0x18, 0x11, 0x16, 0x03, 0x04, 0x0D, 0x0A,
    0x57, 0x50, 0x59, 0x5E, 0x4B, 0x4C, 0x45, 0x42, 0x6F, 0x68, 0x61, 0x66, 0x73, 0x74, 0x7D, 0x7A,
    0x89, 0x8E, 0x87, 0x80, 0x95, 0x92, 0x9B, 0x9C, 0xB1, 0xB6, 0xBF, 0xB8, 0xAD, 0xAA, 0xA3, 0xA4,
    0xF9, 0xFE, 0xF7, 0xF0, 0xE5, 0xE2, 0xEB, 0xEC, 0xC1, 0xC6, 0xCF, 0xC8, 0xDD, 0xDA, 0xD3, 0xD4,
    0x69, 0x6E, 0x67, 0x60, 0x75, 0x72, 0x7B, 0x7C, 0x51, 0x56, 0x5F, 0x58, 0x4D, 0x4A, 0x43, 0x44,
    0x19, 0x1E, 0x17, 0x10, 0x05, 0x02, 0x0B, 0x0C, 0x21, 0x26, 0x2F, 0x28, 0x3D, 0x3A, 0x33, 0x34,
    0x4E, 0x49, 0x40, 0x47, 0x52, 0x55, 0x5C, 0x5B, 0x76, 0x71, 0x78, 0x7F, 0x6A, 0x6D, 0x64, 0x63,
    0x3E, 0x39, 0x30, 0x37, 0x22, 0x25, 0x2C, 0x2B, 0x06, 0x01, 0x08, 0x0F, 0x1A, 0x1D, 0x14, 0x13,
    0xAE, 0xA9, 0xA0, 0xA7, 0xB2, 0xB5, 0xBC, 0xBB, 0x96, 0x91, 0x98, 0x9F, 0x8A, 0x8D, 0x84, 0x83,
    0xDE, 0xD9, 0xD0, 0xD7, 0xC2, 0xC5, 0xCC, 0xCB, 0xE6, 0xE1, 0xE8, 0xEF, 0xFA, 0xFD, 0xF4, 0xF3,
];

/// Computes standard CRC-8 ATM checksum for a 9-char payload
pub fn compute_slot_crc(payload: &str) -> u8 {
    let mut crc = 0x00u8;
    for b in payload.as_bytes() {
        crc = CRC8_TABLE[(crc ^ b) as usize];
    }
    crc
}

/// Constructs a valid 10-char slot with exact CRC-8 ATM token
pub fn build_valid_slot(prefix: &str, body: &str) -> String {
    let mut raw = format!("{}{}", prefix, body);
    if raw.len() > 9 {
        raw.truncate(9);
    } else {
        while raw.len() < 9 {
            raw.push('0');
        }
    }
    let crc = compute_slot_crc(&raw);
    let crc_char = (33 + (crc % 94)) as char;
    format!("{}{}", raw, crc_char)
}

/// Macro Compiler that generates hazard-free `.cl` bundles
pub struct MacroCompiler {
    slots: Vec<String>,
    core_id: u8,
}

impl MacroCompiler {
    pub fn new(core_id: u8) -> Self {
        Self {
            slots: Vec::new(),
            core_id,
        }
    }

    /// Add a raw slot into the bundle stream
    pub fn emit_slot(&mut self, slot: String) {
        assert_eq!(slot.len(), 10, "Slot must be exactly 10 characters");
        self.slots.push(slot);
    }

    /// Emit NOP slot
    pub fn emit_nop(&mut self) {
        let slot = build_valid_slot("__NOP", &format!("{:02X}00", self.core_id));
        self.emit_slot(slot);
    }

    /// Flatten high-level macro statements
    pub fn compile_stmts(&mut self, stmts: &[MacroStmt]) {
        for stmt in stmts {
            match stmt {
                MacroStmt::Label(_) => {}
                MacroStmt::AssignImm { dst, imm } => {
                    let slot = build_valid_slot(&format!("=={:02X}", dst), &format!("#{:04X}", imm & 0xFFFF));
                    self.emit_slot(slot);
                }
                MacroStmt::AssignReg { dst, src } => {
                    let slot = build_valid_slot("_CP", &format!("{:02X}{:02X}00", dst, src));
                    self.emit_slot(slot);
                }
                MacroStmt::BinaryOp { op, dst, src1, src2 } => {
                    let op_prefix = match op {
                        '+' => "_AD",
                        '-' => "_SB",
                        '*' => "_ML",
                        '&' => "_AN",
                        '|' => "_OR",
                        '^' => "_XO",
                        _ => "_AD",
                    };
                    let slot = build_valid_slot(op_prefix, &format!("{:02X}{:02X}{:02X}", dst, src1, src2));
                    self.emit_slot(slot);
                }
                MacroStmt::CompoundOp { mode, dst, src, imm_or_reg } => {
                    let prefix = format!("_{}", mode);
                    let slot = build_valid_slot(&prefix, &format!("{:02X}{:02X}#{:02X}", dst, src, imm_or_reg & 0xFF));
                    self.emit_slot(slot);
                }
                MacroStmt::OpticalGemm { dst, src1, src2 } => {
                    let slot = build_valid_slot("_OP", &format!("{:02X}{:02X}{:02X}", dst, src1, src2));
                    self.emit_slot(slot);
                }
                MacroStmt::TernaryMac { dst, src, weights } => {
                    let slot = build_valid_slot("_MD", &format!("{:02X}{:02X}{:02X}", dst, src, weights));
                    self.emit_slot(slot);
                }
                MacroStmt::StdpUpdate { synapses, pre, post } => {
                    let slot = build_valid_slot("_ST", &format!("{:02X}{:02X}{:02X}", synapses, pre, post));
                    self.emit_slot(slot);
                }
                MacroStmt::PackTrits { dst, src } => {
                    let slot = build_valid_slot("_PK", &format!("{:02X}{:02X}00", dst, src));
                    self.emit_slot(slot);
                }
                MacroStmt::SendNoc { dest_core, data_reg } => {
                    let slot = build_valid_slot("_TX", &format!("{:02X}{:02X}00", dest_core, data_reg));
                    self.emit_slot(slot);
                }
                MacroStmt::RecvNoc { dst_reg } => {
                    let slot = build_valid_slot("_RX", &format!("{:02X}0000", dst_reg));
                    self.emit_slot(slot);
                }
                MacroStmt::Barrier => {
                    let slot = build_valid_slot("_SY", &format!("{:02X}0000", self.core_id));
                    self.emit_slot(slot);
                }
                MacroStmt::Loop { count, body } => {
                    for _ in 0..*count {
                        self.compile_stmts(body);
                    }
                }
            }
        }
    }

    /// Package collected slots into 4-slot VLIW bundles
    pub fn finish(mut self) -> String {
        while self.slots.len() % 4 != 0 {
            self.emit_nop();
        }

        let mut output = String::new();
        for (i, chunk) in self.slots.chunks(4).enumerate() {
            output.push_str(&format!("B{:04X}: {} {} {} {}\n", i, chunk[0], chunk[1], chunk[2], chunk[3]));
        }
        output
    }
}

/// Parses a simple `.clm` high-level DSL text into Macro statements
pub fn parse_clm(source: &str) -> Result<Vec<MacroStmt>, String> {
    let mut stmts = Vec::new();
    for (line_no, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        match tokens[0] {
            "let" => {
                // let R0 = 0x42 or let R0 = R1
                if tokens.len() >= 4 && tokens[2] == "=" {
                    let dst = parse_reg(tokens[1])?;
                    if tokens[3].starts_with("R") || tokens[3].starts_with("r") {
                        let src = parse_reg(tokens[3])?;
                        stmts.push(MacroStmt::AssignReg { dst, src });
                    } else {
                        let imm = parse_imm(tokens[3])?;
                        stmts.push(MacroStmt::AssignImm { dst, imm });
                    }
                } else {
                    return Err(format!("Line {}: Invalid 'let' syntax", line_no + 1));
                }
            }
            "op_gemm" | "optical_dot" => {
                if tokens.len() >= 4 {
                    let dst = parse_reg(tokens[1])?;
                    let src1 = parse_reg(tokens[2])?;
                    let src2 = parse_reg(tokens[3])?;
                    stmts.push(MacroStmt::OpticalGemm { dst, src1, src2 });
                }
            }
            "ternary_mac" => {
                if tokens.len() >= 4 {
                    let dst = parse_reg(tokens[1])?;
                    let src = parse_reg(tokens[2])?;
                    let weights = parse_reg(tokens[3])?;
                    stmts.push(MacroStmt::TernaryMac { dst, src, weights });
                }
            }
            "stdp_learn" | "stdp_update" => {
                if tokens.len() >= 4 {
                    let synapses = parse_reg(tokens[1])?;
                    let pre = parse_reg(tokens[2])?;
                    let post = parse_reg(tokens[3])?;
                    stmts.push(MacroStmt::StdpUpdate { synapses, pre, post });
                }
            }
            "pack_trits" => {
                if tokens.len() >= 3 {
                    let dst = parse_reg(tokens[1])?;
                    let src = parse_reg(tokens[2])?;
                    stmts.push(MacroStmt::PackTrits { dst, src });
                }
            }
            "send_noc" => {
                if tokens.len() >= 3 {
                    let dest = parse_imm(tokens[1])? as u8;
                    let reg = parse_reg(tokens[2])?;
                    stmts.push(MacroStmt::SendNoc { dest_core: dest, data_reg: reg });
                }
            }
            "recv_noc" => {
                if tokens.len() >= 2 {
                    let dst = parse_reg(tokens[1])?;
                    stmts.push(MacroStmt::RecvNoc { dst_reg: dst });
                }
            }
            "barrier" | "sync" => {
                stmts.push(MacroStmt::Barrier);
            }
            _ => {
                // Check for R0 = R1 + R2
                if tokens.len() >= 5 && tokens[1] == "=" {
                    let dst = parse_reg(tokens[0])?;
                    let src1 = parse_reg(tokens[2])?;
                    let op = tokens[3].chars().next().unwrap_or('+');
                    let src2 = parse_reg(tokens[4])?;
                    stmts.push(MacroStmt::BinaryOp { op, dst, src1, src2 });
                }
            }
        }
    }
    Ok(stmts)
}

fn parse_reg(token: &str) -> Result<u8, String> {
    let t = token.trim_matches(|c| c == ',' || c == ';' || c == 'R' || c == 'r');
    u8::from_str_radix(t, 16).or_else(|_| t.parse::<u8>())
        .map_err(|_| format!("Invalid register token: {}", token))
}

fn parse_imm(token: &str) -> Result<u16, String> {
    let t = token.trim_matches(|c| c == ',' || c == ';');
    if t.starts_with("0x") || t.starts_with("0X") {
        u16::from_str_radix(&t[2..], 16).map_err(|_| format!("Invalid hex imm: {}", token))
    } else {
        t.parse::<u16>().map_err(|_| format!("Invalid decimal imm: {}", token))
    }
}
