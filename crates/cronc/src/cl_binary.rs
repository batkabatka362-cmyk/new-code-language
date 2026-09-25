// ============================================================================
// CRON Low-Level Machine Language 128-bit Binary Bytecode (.clb)
//
// 128-bit VLIW Binary Format Specification:
//
// Header (12 bytes):
//   [0..4]   Magic: "CLB1" (0x43, 0x4C, 0x42, 0x31)
//   [4..6]   Version: u16 (big-endian) = 1
//   [6..8]   Flags: u16 (big-endian) = 0
//   [8..12]  Bundle Count: u32 (big-endian)
//
// Per-Bundle Payload (20 bytes):
//   [0..4]   Cycle Index: u32 (big-endian)
//   [4..20]  128-bit VLIW Execution Bundle:
//            4 Slots x 32 bits (4 bytes) each = 16 bytes.
//
// 32-bit Slot Bitfield Layouts:
//   Standard Slot:
//     [31:30] Prefix (2 bits): 0='_', 1='\'', 2='~', 3='@'
//     [29:24] Opcode ID (6 bits, 0..63)
//     [23:20] Dest Bank (4 bits, 0..15)
//     [19:16] Dest Reg (4 bits, 0..15)
//     [15:10] Mode Symbol ID (6 bits, 0..63)
//     [9:6]   Src Reg (4 bits, 0..15)
//     [5:2]   Immediate Nibble (4 bits, 0..15)
//     [1:0]   Terminator (2 bits): 0='>', 1='!', 2='?', 3=';'
//
//   Immediate Load Slot (Prefix=1 with 16-bit Immediate):
//     [31:30] Prefix (2 bits) = 1
//     [29:24] Opcode ID (6 bits) = e.g. "==" or "=0"
//     [23:20] Dest Bank (4 bits)
//     [19:16] Dest Reg (4 bits)
//     [15:0]  Immediate Value: u16 (16 bits)
// ============================================================================

use crate::codegen::compute_parity;

pub const CLB_MAGIC: [u8; 4] = *b"CLB1";
pub const CLB_VERSION: u16 = 1;

pub const OP_TABLE: &[&str] = &[
    "NO", "OP", "FA", "PO", "MD", "BK", "BL", "RF", "GU", "ST", // 0..9
    "SY", "RS", "PK", "TL", "YD", "SP", "FJ", "DW", "SB", "SH", // 10..19
    "RC", "TO", "WD", "KG", "SW", "PT", "HE", "OD", "CS", "TT", // 20..29
    "PS", "CA", "CD", "AW", "WH", "LF", "LI", "TX", "RX", "IR", // 30..39
    "DF", "AC", "SN", "SC", "RN", "PL", "RT", "HL", "=0", "=1", // 40..49
    "==", "bb", "CC", "DD", "EE", "BB", "FF", "11", "88", "99", // 50..59
    "aa", "ee", "00", "P0",                                     // 60..63
];

pub const MODE_TABLE: &[char] = &[
    '$', '#', '@', '+', '-', '*', '/', '%', '&', '|', // 0..9
    '^', '~', '=', '<', '>', '?', '!', 'G', ':', ';', // 10..19
    ',', '.', '_', '\'', '0', '1', '2', '3', '4', '5', // 20..29
    '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', // 30..39
    'X', 'Y', 'Z', 'W', 'H', 'L', 'S', 'U', 'V', 'T', // 40..49
    'M', 'N', 'O', 'P', 'Q', 'R', 'I', 'J', 'K', 'E', // 50..59
    '?', '!', '/', '\\',                              // 60..63
];

pub fn opcode_to_id(op: &str) -> u8 {
    for (idx, &known) in OP_TABLE.iter().enumerate() {
        if known == op {
            return idx as u8;
        }
    }
    0 // Default to NOP
}

pub fn id_to_opcode(id: u8) -> &'static str {
    let idx = (id as usize).min(OP_TABLE.len() - 1);
    OP_TABLE[idx]
}

pub fn mode_to_id(mode: char) -> u8 {
    for (idx, &known) in MODE_TABLE.iter().enumerate() {
        if known == mode {
            return idx as u8;
        }
    }
    0 // Default to '$'
}

pub fn id_to_mode(id: u8) -> char {
    let idx = (id as usize).min(MODE_TABLE.len() - 1);
    MODE_TABLE[idx]
}

pub fn prefix_to_bits(prefix: char) -> u32 {
    match prefix {
        '\'' => 1,
        '~' => 2,
        '@' => 3,
        _ => 0, // '_'
    }
}

pub fn bits_to_prefix(bits: u32) -> char {
    match bits & 0x3 {
        1 => '\'',
        2 => '~',
        3 => '@',
        _ => '_',
    }
}

pub fn terminator_to_bits(term: char) -> u32 {
    match term {
        '!' => 1,
        '?' => 2,
        ';' => 3,
        _ => 0, // '>'
    }
}

pub fn bits_to_terminator(bits: u32) -> char {
    match bits & 0x3 {
        1 => '!',
        2 => '?',
        3 => ';',
        _ => '>',
    }
}

/// Encode a single 10-character .cl slot into a 32-bit binary representation
pub fn encode_slot_to_u32(slot: &str) -> u32 {
    if slot.len() < 3 {
        return 0; // NOP
    }

    let chars: Vec<char> = slot.chars().collect();
    if chars.len() < 3 {
        return 0; // NOP
    }

    let prefix = chars.first().copied().unwrap_or('_');
    let prefix_bits = prefix_to_bits(prefix);

    // Check for 16-bit Immediate Load (e.g. '=00#0A04>, '=06#0064>, '==04#000A>)
    if prefix == '\'' && slot.contains('#') {
        let is_double_eq = chars.len() >= 3 && chars.get(1) == Some(&'=') && chars.get(2) == Some(&'=');
        let op_id = if is_double_eq { opcode_to_id("==") } else { opcode_to_id("=0") };

        // Bank and Reg extraction
        let (dest_bank, dest_reg) = if chars.len() >= 5 && chars.get(4) == Some(&'#') {
            let h = chars.get(2).and_then(|c| c.to_digit(16)).unwrap_or(0);
            let l = chars.get(3).and_then(|c| c.to_digit(16)).unwrap_or(0);
            (h, l)
        } else if chars.len() >= 5 {
            let h = chars.get(3).and_then(|c| c.to_digit(16)).unwrap_or(0);
            let l = chars.get(4).and_then(|c| c.to_digit(16)).unwrap_or(0);
            (h, l)
        } else {
            (0, 0)
        };

        // Immediate hex value
        let imm16 = if let Some(pos) = slot.find('#') {
            let hex_str: String = slot[pos + 1..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();
            u16::from_str_radix(&hex_str, 16).unwrap_or(0)
        } else {
            0
        };

        return (prefix_bits << 30)
            | ((op_id as u32 & 0x3F) << 24)
            | ((dest_bank & 0xF) << 20)
            | ((dest_reg & 0xF) << 16)
            | (imm16 as u32);
    }

    // Standard Slot Encoding
    let op_str: String = if chars.len() >= 3 {
        chars[1..3].iter().collect()
    } else {
        "NO".to_string()
    };
    let op_id = opcode_to_id(&op_str);

    let (dest_bank, dest_reg) = if chars.len() >= 5 {
        let h = chars.get(3).and_then(|c| c.to_digit(16)).unwrap_or(0);
        let l = chars.get(4).and_then(|c| c.to_digit(16)).unwrap_or(0);
        (h, l)
    } else {
        (0, 0)
    };

    let mode_char = chars.get(5).copied().unwrap_or('$');
    let mode_id = mode_to_id(mode_char);

    let src_reg = if chars.len() >= 7 {
        chars.get(6).and_then(|c| c.to_digit(16)).unwrap_or(0)
    } else {
        0
    };

    let imm_nibble = if chars.len() >= 9 {
        chars.get(8).and_then(|c| c.to_digit(16)).unwrap_or(0)
    } else {
        0
    };

    let terminator = chars.get(9).copied().unwrap_or('>');
    let term_bits = terminator_to_bits(terminator);

    (prefix_bits << 30)
        | ((op_id as u32 & 0x3F) << 24)
        | ((dest_bank & 0xF) << 20)
        | ((dest_reg & 0xF) << 16)
        | ((mode_id as u32 & 0x3F) << 10)
        | ((src_reg & 0xF) << 6)
        | ((imm_nibble & 0xF) << 2)
        | (term_bits & 0x3)
}

/// Decode a 32-bit binary slot back into a valid 10-character .cl slot
pub fn decode_u32_to_slot(encoded: u32) -> String {
    let prefix_bits = (encoded >> 30) & 0x3;
    let prefix = bits_to_prefix(prefix_bits);
    let op_id = ((encoded >> 24) & 0x3F) as u8;
    let op_str = id_to_opcode(op_id);
    let dest_bank = (encoded >> 20) & 0xF;
    let dest_reg = (encoded >> 16) & 0xF;

    // Check for Immediate Load Slot
    if prefix == '\'' && (op_str == "==" || op_str == "=0" || op_str == "=1") {
        let imm16 = (encoded & 0xFFFF) as u16;
        return format!("'={:1X}{:1X}#{:04X}>", dest_bank, dest_reg, imm16);
    }

    // Standard Slot Decoding
    let mode_id = ((encoded >> 10) & 0x3F) as u8;
    let mode_char = id_to_mode(mode_id);
    let src_reg = (encoded >> 6) & 0xF;
    let imm_nibble = (encoded >> 2) & 0xF;
    let term_bits = encoded & 0x3;
    let terminator = bits_to_terminator(term_bits);

    let dest_hex = format!("{:1X}{:1X}", dest_bank, dest_reg);
    let src_char = format!("{:1X}", src_reg).chars().next().unwrap();
    let imm_char = format!("{:1X}", imm_nibble).chars().next().unwrap();
    let parity = compute_parity(prefix, op_str, &dest_hex, mode_char, src_char, imm_char);

    format!(
        "{}{}{}{}{}{}{}{}",
        prefix, op_str, dest_hex, mode_char, src_char, parity, imm_char, terminator
    )
}

/// Assemble textual .cl code into 128-bit binary bytecode (.clb format)
pub fn assemble_cl_to_clb(cl_content: &str) -> Result<Vec<u8>, String> {
    let mut bundles: Vec<(u32, [u32; 4])> = Vec::new();

    for line in cl_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with(';')
            || trimmed.starts_with("//")
            || trimmed.starts_with('.')
            || trimmed.starts_with('@')
            || trimmed.starts_with('#')
        {
            continue;
        }

        if let Some((cycle_part, slots_part)) = trimmed.split_once(':') {
            let cycle_str = cycle_part.trim().trim_start_matches('B');
            let cycle = cycle_str
                .parse::<u32>()
                .map_err(|e| format!("Invalid cycle header '{}': {}", cycle_part, e))?;

            let slot_tokens: Vec<&str> = slots_part.split_whitespace().collect();
            let mut encoded_slots = [0u32; 4];

            for i in 0..4 {
                if let Some(&token) = slot_tokens.get(i) {
                    encoded_slots[i] = encode_slot_to_u32(token);
                } else {
                    // Default NOP slot
                    encoded_slots[i] = encode_slot_to_u32("_NO00$000>");
                }
            }

            bundles.push((cycle, encoded_slots));
        }
    }

    // Allocate binary buffer: 12-byte header + 20 bytes per bundle
    let total_size = 12 + bundles.len() * 20;
    let mut out = Vec::with_capacity(total_size);

    // Write Header
    out.extend_from_slice(&CLB_MAGIC);
    out.extend_from_slice(&CLB_VERSION.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes()); // Reserved
    out.extend_from_slice(&(bundles.len() as u32).to_be_bytes());

    // Write Bundles
    for (cycle, slots) in bundles {
        out.extend_from_slice(&cycle.to_be_bytes());
        for slot in slots {
            out.extend_from_slice(&slot.to_be_bytes());
        }
    }

    Ok(out)
}

/// Disassemble 128-bit binary bytecode (.clb) back into human-readable .cl assembly
pub fn disassemble_clb_to_cl(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 12 {
        return Err(format!(
            "Corrupt CLB binary: file size ({} bytes) is smaller than the 12-byte header",
            bytes.len()
        ));
    }

    if bytes[0..4] != CLB_MAGIC {
        return Err(format!(
            "Invalid CLB magic bytes: expected {:?}, got {:?}",
            CLB_MAGIC,
            &bytes[0..4]
        ));
    }

    let version = u16::from_be_bytes([bytes[4], bytes[5]]);
    if version != CLB_VERSION {
        return Err(format!(
            "Unsupported CLB version: {} (current supported: {})",
            version, CLB_VERSION
        ));
    }

    let bundle_count = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
    let expected_len = 12 + bundle_count * 20;
    if bytes.len() < expected_len {
        return Err(format!(
            "Truncated CLB binary: expected at least {} bytes for {} bundles, found {}",
            expected_len,
            bundle_count,
            bytes.len()
        ));
    }

    let mut out = String::new();
    out.push_str("; ============================================================================\n");
    out.push_str("; Disassembled from CRON 128-bit Binary Bytecode (.clb)\n");
    out.push_str(&format!("; CLB Version: {} | Total Bundles: {}\n", version, bundle_count));
    out.push_str("; ============================================================================\n\n");

    let mut offset = 12;
    for _ in 0..bundle_count {
        let cycle = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let mut slot_strings = Vec::with_capacity(4);
        for _ in 0..4 {
            let slot_val = u32::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            offset += 4;
            slot_strings.push(decode_u32_to_slot(slot_val));
        }

        out.push_str(&format!(
            "B{:04}: {} {} {} {}\n",
            cycle, slot_strings[0], slot_strings[1], slot_strings[2], slot_strings[3]
        ));
    }

    Ok(out)
}
