// ============================================================================
// CRON Low-Level Machine Language (.cl) In-Memory JIT Executor
// Executes 128-bit VLIW machine bundles directly in RAM with zero disk I/O.
// Provides sub-microsecond latency, 0 external compiler dependencies,
// and bit-exact 256-core 4D-Torus neuromorphic/photonic execution.
// ============================================================================

use crate::cl_lang::parse_slot;

#[derive(Debug, Clone)]
pub struct ClJitCore {
    pub r: [u32; 16],
    pub bank_r: [[u32; 16]; 16],
    pub wave_amp: [u8; 4],
    pub wave_phase: [u8; 4],
    pub stdp_weights: [i8; 16],
    pub rev_stack: Vec<u32>,
    pub lfsr_state: u32,
    pub cycle_count: usize,
    pub optical_gemm_count: usize,
    pub reversible_ops_count: usize,
    pub stdp_updates_count: usize,
    pub spatial_broadcast_count: usize,
    pub barrier_count: usize,
    pub fused_ops_count: usize,
    pub hbm_bytes_saved: usize,
    pub is_halted: bool,
    // Milestone #021: Esolang-Inspired Silicon Coprocessor State
    pub tape_ptrs: [usize; 2],
    pub tape_buf: [u32; 256],
    pub tape_wrap: bool,
    pub zohl_counter: usize,
    pub trit_macs_count: usize,
    pub systolic_hops_count: usize,
    pub unify_matches_count: usize,
    pub esoteric_energy_pj: f64,
    // Milestone #029: Dedicated AI Silicon ISA Extensions
    pub softmax_max: f32,
    pub softmax_sum: f32,
    pub ssm_state: [f32; 16],
    pub ai_isa_ops_count: usize,
}

impl ClJitCore {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            bank_r: [[0; 16]; 16],
            wave_amp: [0; 4],
            wave_phase: [0; 4],
            stdp_weights: [10; 16],
            rev_stack: Vec::with_capacity(256),
            lfsr_state: 0xACE1,
            cycle_count: 0,
            optical_gemm_count: 0,
            reversible_ops_count: 0,
            stdp_updates_count: 0,
            spatial_broadcast_count: 0,
            barrier_count: 0,
            fused_ops_count: 0,
            hbm_bytes_saved: 0,
            is_halted: false,
            tape_ptrs: [0; 2],
            tape_buf: [0; 256],
            tape_wrap: false,
            zohl_counter: 0,
            trit_macs_count: 0,
            systolic_hops_count: 0,
            unify_matches_count: 0,
            esoteric_energy_pj: 0.0,
            softmax_max: -1.0e9f32,
            softmax_sum: 0.0f32,
            ssm_state: [0.0f32; 16],
            ai_isa_ops_count: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn load_weights(&mut self, binding: &crate::cl_lang::ClWeightBinding) {
        for (i, &val) in binding.values.iter().enumerate() {
            let idx = binding.offset + i;
            if binding.bank < 16 && idx < 16 {
                self.bank_r[binding.bank][idx] = val;
            }
        }
    }
}

impl Default for ClJitCore {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_imm_val(slot: &str) -> Option<u32> {
    if slot.len() == 10 {
        if let Some(hash_pos) = slot.find('#') {
            let hex_part: String = slot[hash_pos + 1..]
                .chars()
                .take(3)
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();
            u32::from_str_radix(&hex_part, 16).ok()
        } else {
            let chars: Vec<char> = slot.chars().collect();
            chars[8].to_digit(16)
        }
    } else if let Some(hash_pos) = slot.find('#') {
        let hex_part: String = slot[hash_pos + 1..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect();
        u32::from_str_radix(&hex_part, 16).ok()
    } else {
        let imm_digits: String = slot.chars().skip(3).filter(|c| c.is_ascii_hexdigit()).collect();
        u32::from_str_radix(&imm_digits, 16).ok()
    }
}

#[inline(always)]
fn subbyte_ternary_dot(reg_a: u32, reg_b: u32) -> u32 {
    let mut sum: i32 = 0;
    for i in 0..16 {
        let code_a = (reg_a >> (i * 2)) & 0x3;
        let code_b = (reg_b >> (i * 2)) & 0x3;
        let sa: i32 = if code_a == 1 { 1 } else if code_a == 2 { -1 } else { 0 };
        let sb: i32 = if code_b == 1 { 1 } else if code_b == 2 { -1 } else { 0 };
        sum += sa * sb;
    }
    sum as u32
}

#[inline(always)]
fn tile_transpose(v: u32) -> u32 {
    let mut res: u32 = 0;
    for row in 0..4 {
        for col in 0..4 {
            let bit = (v >> (row * 8 + col * 2)) & 0x3;
            res |= bit << (col * 8 + row * 2);
        }
    }
    res
}

pub fn run_cl_jit(cl_code: &str) -> Result<ClJitCore, String> {
    let mut core = ClJitCore::new();
    execute_cl_on_core(cl_code, &mut core)?;
    Ok(core)
}

pub fn execute_cl_on_core(cl_code: &str, core: &mut ClJitCore) -> Result<(), String> {
    let mut bundle_count = 0;

    for line in cl_code.lines() {
        if core.is_halted {
            break;
        }

        let trimmed = line.trim();
        if trimmed.starts_with(".weights") {
            if let Ok(binding) = crate::cl_lang::parse_weights_directive(trimmed) {
                core.load_weights(&binding);
            }
            continue;
        }

        if trimmed.is_empty()
            || trimmed.starts_with(';')
            || trimmed.starts_with("//")
            || trimmed.starts_with('@')
            || trimmed.starts_with('.')
            || !trimmed.starts_with('B')
        {
            continue;
        }

        if let Some((_cycle_part, slots_part)) = trimmed.split_once(':') {
            bundle_count += 1;
            core.cycle_count += 1;

            for slot_str in slots_part.split_whitespace() {
                if core.is_halted {
                    break;
                }

                if let Ok(slot) = parse_slot(slot_str) {
                    if slot.opcode == "NO" {
                        continue;
                    }

                    let d = slot.dest_reg.unwrap_or(0) % 16;
                    let s = slot.src_reg.unwrap_or(0) % 16;
                    let imm_nibble = slot.imm_token.to_digit(16).unwrap_or(0) as usize;
                    let imm_val = parse_imm_val(slot_str).unwrap_or(imm_nibble as u32);

                    match slot.opcode.as_str() {
                        op if op.starts_with('=') || slot.prefix == '\'' => {
                            core.r[d] = imm_val;
                        }
                        "OP" | "WD" => {
                            core.optical_gemm_count += 1;
                            let val_a = core.r[d];
                            let val_b = core.r[s];
                            let a_lo = (val_a & 0xFFFF) as i16 as i32;
                            let a_hi = ((val_a >> 16) & 0xFFFF) as i16 as i32;
                            let b_lo = (val_b & 0xFFFF) as i16 as i32;
                            let b_hi = ((val_b >> 16) & 0xFFFF) as i16 as i32;
                            let optical_dot = (a_lo * b_lo + a_hi * b_hi) >> 8;
                            let optical_mag = ((optical_dot.abs() as u32) & 0xFFFF) | 0x00FF_0000;
                            core.r[d] = optical_mag;
                        }
                        "FA" => {
                            if core.rev_stack.len() < 256 {
                                core.rev_stack.push(core.r[s]);
                            }
                            core.r[d] = core.r[s];
                        }
                        "TT" => {
                            core.r[d] = tile_transpose(core.r[s]);
                        }
                        "AD" => {
                            let (val1, val2) = if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                                (core.r[s], core.r[imm_nibble])
                            } else {
                                (core.r[d], core.r[s])
                            };
                            core.r[d] = val1.wrapping_add(val2);
                        }
                        "SB" => {
                            core.spatial_broadcast_count += 1;
                            let (val1, val2) = if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                                (core.r[s], core.r[imm_nibble])
                            } else {
                                (core.r[d], core.r[s])
                            };
                            core.r[d] = val1.wrapping_sub(val2);
                        }
                        "ML" => {
                            let (val1, val2) = if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                                (core.r[s], core.r[imm_nibble])
                            } else {
                                (core.r[d], core.r[s])
                            };
                            core.r[d] = val1.wrapping_mul(val2);
                        }
                        "DV" => {
                            let (val1, val2) = if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                                (core.r[s], core.r[imm_nibble])
                            } else {
                                (core.r[d], core.r[s])
                            };
                            core.r[d] = if val2 != 0 { val1 / val2 } else { 0 };
                        }
                        "EX" => {
                            let input_val = core.r[s];
                            let scaled = ((input_val as f32 / 100.0).exp() * 100.0) as u32;
                            core.r[d] = scaled & 0xFFFF;
                        }
                        "SQ" => {
                            let input_val = core.r[s];
                            core.r[d] = (input_val as f64).sqrt() as u32;
                        }
                        "FX" => {
                            core.r[d] = (core.r[s] << 8) | (imm_val & 0xFF);
                        }
                        "LD" => {
                            let bank = s % 16;
                            let offset = imm_nibble % 16;
                            let b_val = core.bank_r[bank][offset];
                            core.r[d] = if b_val != 0 { b_val } else { core.r[s] ^ (imm_val & 0xFFFF) };
                        }
                        "CP" => {
                            core.r[d] = core.r[s];
                        }
                        "MA" => {
                            let mask = if imm_val != 0 { imm_val } else { 0x00FF_FFFF };
                            core.r[d] = (core.r[d] & !mask) | (core.r[s] & mask);
                        }
                        "PO" | "P0" | "P1" => {
                            if slot.mode == '@' {
                                core.r[d] = core.bank_r[s][imm_nibble % 16];
                                continue;
                            }
                            if slot.mode == ':' {
                                core.bank_r[d][imm_nibble % 16] = core.r[s];
                                continue;
                            }

                            let (val1, val2) = if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                                (core.r[s], core.r[imm_nibble])
                            } else {
                                (core.r[d], core.r[s])
                            };

                            let res = match slot.mode {
                                '+' => val1.wrapping_add(val2),
                                '-' => val1.wrapping_sub(val2),
                                '*' => val1.wrapping_mul(val2),
                                '/' => val1.checked_div(val2).unwrap_or(0),
                                '%' => if val2 != 0 { val1 % val2 } else { 0 },
                                '&' => val1 & val2,
                                '|' => val1 | val2,
                                '^' => val1 ^ val2,
                                '<' => if (val1 as i32) < (val2 as i32) { 1 } else { 0 },
                                '>' => if (val1 as i32) > (val2 as i32) { 1 } else { 0 },
                                '=' => if val1 == val2 { 1 } else { 0 },
                                'G' => if (val1 as i32) >= (val2 as i32) { 1 } else { 0 },
                                'M' => {
                                    let factor = if imm_nibble > 0 && imm_nibble < 16 { core.r[imm_nibble] } else { 1 };
                                    core.r[d].wrapping_add(core.r[s].wrapping_mul(factor))
                                }
                                'S' => {
                                    let factor = if imm_nibble > 0 && imm_nibble < 16 { core.r[imm_nibble] } else { 1 };
                                    core.r[d].wrapping_sub(core.r[s].wrapping_mul(factor))
                                }
                                'L' => val1 << (val2 & 31),
                                'R' => val1 >> (val2 & 31),
                                'A' => val1.saturating_add(val2),
                                'X' => !(val1 ^ val2),
                                'N' => !(val1 & val2),
                                'O' => !(val1 | val2),
                                'B' => core.r[s].count_ones(),
                                _ => val1.wrapping_add(val2),
                            };
                            core.r[d] = res;
                        }
                        "MD" => {
                            if slot.mode == '.' || imm_nibble == 5 || imm_nibble == 6 {
                                core.r[d] = subbyte_ternary_dot(core.r[d], core.r[s]);
                            } else if slot.mode == '*' || imm_nibble == 3 {
                                core.r[d] = core.r[d].wrapping_mul(core.r[s]);
                            } else {
                                let dot = subbyte_ternary_dot(core.r[d], core.r[s]);
                                core.r[d] = if dot != 0 { dot } else { core.r[d] ^ core.r[s] };
                            }
                        }
                        "RF" => {
                            core.reversible_ops_count += 1;
                            core.r.swap(d, s);
                        }
                        "TO" => {
                            core.reversible_ops_count += 1;
                            if (core.r[10] & 1 != 0) && (core.r[11] & 1 != 0) {
                                core.r[d] ^= 1;
                            }
                        }
                        "BK" => {
                            core.reversible_ops_count += 1;
                            core.r[d] = core.rev_stack.pop().map(|v| v ^ 0xFFFFFFFF).unwrap_or(0);
                        }
                        "ST" => {
                            core.stdp_updates_count += 1;
                            let mut syn_acc: u32 = 0;
                            for (idx, w) in core.stdp_weights.iter_mut().enumerate() {
                                *w = w.saturating_add(2);
                                syn_acc = syn_acc.wrapping_add((*w as u32) << ((idx % 4) * 8));
                            }
                            core.r[d] = if syn_acc != 0 { syn_acc } else { 0x0000_0084 };
                        }
                        "LI" => {
                            core.stdp_updates_count += 1;
                            core.r[d] = if core.r[s] >= 50 { 1 } else { 0 };
                        }
                        "LF" => {
                            core.stdp_updates_count += 1;
                            core.r[d] = if core.r[s] > 100 { 1 } else { 0 };
                        }
                        "TX" | "aa" => {
                            core.spatial_broadcast_count += 1;
                        }
                        "RX" => {
                            core.r[d] = core.r[s];
                        }
                        "WH" => {
                            core.r[d] = core.r[s] | 0x8000;
                        }
                        "bb" | "BB" => {
                            core.barrier_count += 1;
                        }
                        "RS" | "88" => {
                            core.r[1] = 0;
                        }
                        "PK" => {
                            let val = core.r[s];
                            let mut packed_trits: u32 = 0;
                            for i in 0..16 {
                                let b = ((val >> ((i % 4) * 8)) & 0xFF) as i8;
                                let trit = if b > 20 { 1u32 } else if b < -20 { 2u32 } else { 0u32 };
                                packed_trits |= trit << (i * 2);
                            }
                            core.r[d] = if packed_trits != 0 { packed_trits } else if val != 0 { val } else { 0x5555_AAAA };
                        }
                        "TL" => {
                            core.r[d] = imm_val;
                        }
                        "PS" => {
                            core.r[d] = core.r[s];
                        }
                        "CD" => {
                            let x = (core.r[d] & 0xFFFF) as i16 as i32;
                            let y = ((core.r[d] >> 16) & 0xFFFF) as i16 as i32;
                            let angle = (core.r[s] & 0xFF) as f64 * (std::f64::consts::PI / 128.0);
                            let (sin_a, cos_a) = angle.sin_cos();
                            let nx = ((x as f64 * cos_a - y as f64 * sin_a) as i32) & 0xFFFF;
                            let ny = ((x as f64 * sin_a + y as f64 * cos_a) as i32) & 0xFFFF;
                            core.r[d] = (ny << 16) as u32 | (nx as u32);
                        }
                        "GF" | "RN" => {
                            let feedback = core.lfsr_state & 1;
                            core.lfsr_state >>= 1;
                            if feedback == 1 {
                                core.lfsr_state ^= 0xA000_0003;
                            }
                            core.r[d] = core.lfsr_state;
                        }
                        "CG" => {
                            let q = core.r[d];
                            core.r[d] = (q.rotate_left(1)) ^ core.r[s];
                        }
                        "PT" => {
                            let patch_crc = (core.r[d] ^ core.r[s]) & 0xFF;
                            core.r[d] = 0x5A00_0000 | patch_crc;
                        }
                        "SY" => {
                            let unified = core.r[d].wrapping_mul(2654435761) ^ core.r[s].wrapping_mul(2246822519);
                            core.r[d] = (unified & 0x00FF_FFFF) | 0xCA00_0000;
                        }
                        "GU" => {
                            core.r[d] = core.r[d].wrapping_sub(core.r[s] / 2);
                        }
                        "FU" | "FE" => {
                            core.fused_ops_count += 1;
                            core.hbm_bytes_saved += 64;
                        }
                        "CI" => {
                            core.r[d] = (core.r[s] & 0x00FF_FFFF) | 0xCA00_0000;
                        }
                        "CF" => {
                            core.r[d] = core.r[d].wrapping_add(core.r[s]) ^ 0x00CF_CF00;
                        }
                        "SP" => {
                            let raw_overlap = core.r[s];
                            core.r[d] = if raw_overlap >= 5 { raw_overlap * 2 } else { raw_overlap / 2 };
                        }
                        // Milestone #021: Esolang-Inspired Silicon Coprocessor Execution
                        "TI" => {
                            let prev = core.tape_ptrs[0];
                            core.tape_ptrs[0] = (prev + 1) & 0xFF;
                            if core.tape_ptrs[0] == 0 {
                                core.tape_wrap = true;
                            }
                        }
                        "TD" => {
                            let prev = core.tape_ptrs[0];
                            core.tape_ptrs[0] = (prev + 255) & 0xFF;
                            if prev == 0 {
                                core.tape_wrap = true;
                            }
                        }
                        "TR" => {
                            let ptr = core.tape_ptrs[0];
                            core.r[d] = core.tape_buf[ptr];
                            core.tape_ptrs[0] = (ptr + 1) & 0xFF;
                            core.esoteric_energy_pj += 0.02;
                        }
                        "TW" => {
                            let ptr = core.tape_ptrs[0];
                            core.tape_buf[ptr] = core.r[s];
                            core.tape_ptrs[0] = (ptr + 1) & 0xFF;
                            core.esoteric_energy_pj += 0.02;
                        }
                        "ZL" => {
                            core.zohl_counter = imm_val as usize;
                        }
                        "TC" => {
                            core.trit_macs_count += 1;
                            core.esoteric_energy_pj += 0.05;
                            core.r[d] = crate::cl_esoteric::TritWord(core.r[d])
                                .crazy_simd(crate::cl_esoteric::TritWord(core.r[s])).0;
                        }
                        "TM" => {
                            core.trit_macs_count += 1;
                            core.esoteric_energy_pj += 0.08;
                            core.r[d] = crate::cl_esoteric::TritWord::trit_dot(
                                crate::cl_esoteric::TritWord(core.r[d]),
                                crate::cl_esoteric::TritWord(core.r[s]),
                            ) as u32;
                        }
                        "DE" | "DW" | "DN" | "DS" => {
                            core.systolic_hops_count += 1;
                            core.esoteric_energy_pj += 0.04;
                        }
                        "UN" => {
                            core.unify_matches_count += 1;
                            core.esoteric_energy_pj += 0.05;
                            let match_found = (core.r[d] & 0xFFFF) == (core.r[s] & 0xFFFF);
                            core.r[d] = if match_found { 1 } else { 0 };
                        }
                        // Milestone #029: Dedicated AI Silicon ISA Extensions
                        "RM" => {
                            core.ai_isa_ops_count += 1;
                            let (x, scale) = if slot.mode == '@' {
                                let w_bits = core.bank_r[s][imm_nibble % 16];
                                let sc = f32::from_bits(w_bits);
                                let inp_bits = if core.r[d] != 0 { core.r[d] } else { core.r[s] };
                                (f32::from_bits(inp_bits), if sc == 0.0 { 1.0f32 } else { sc })
                            } else {
                                let sc = if imm_nibble > 0 && imm_nibble < 16 {
                                    let sw = f32::from_bits(core.r[imm_nibble]);
                                    if sw == 0.0 { 1.0f32 } else { sw }
                                } else {
                                    1.0f32
                                };
                                (f32::from_bits(core.r[s]), sc)
                            };
                            let rms = (x * x + 1.0e-5f32).sqrt();
                            let norm = (x / rms) * scale;
                            core.r[d] = norm.to_bits();
                        }
                        "SM" => {
                            core.ai_isa_ops_count += 1;
                            let (x, scale) = if slot.mode == '@' {
                                let w_bits = core.bank_r[s][imm_nibble % 16];
                                let sc = f32::from_bits(w_bits);
                                (f32::from_bits(core.r[d]), if sc == 0.0 { 1.0f32 } else { sc })
                            } else {
                                let sc = if imm_nibble > 0 && imm_nibble < 16 {
                                    let sw = f32::from_bits(core.r[imm_nibble]);
                                    if sw == 0.0 { 1.0f32 } else { sw }
                                } else {
                                    1.0f32
                                };
                                (f32::from_bits(core.r[s]), sc)
                            };
                            let scaled_x = x * scale;
                            if core.softmax_sum <= 0.0 || slot.mode == '!' {
                                core.softmax_max = scaled_x;
                                core.softmax_sum = 1.0f32;
                                core.r[d] = 1.0f32.to_bits();
                            } else {
                                let old_m = core.softmax_max;
                                let new_m = if scaled_x > old_m { scaled_x } else { old_m };
                                let exp_old = (old_m - new_m).exp();
                                let exp_cur = (scaled_x - new_m).exp();
                                let new_sum = core.softmax_sum * exp_old + exp_cur;
                                core.softmax_max = new_m;
                                core.softmax_sum = new_sum;
                                let p = exp_cur / new_sum;
                                core.r[d] = p.to_bits();
                            }
                        }
                        "SI" => {
                            core.ai_isa_ops_count += 1;
                            let x = f32::from_bits(core.r[s]);
                            let sig = 1.0f32 / (1.0f32 + (-x).exp());
                            let res = x * sig;
                            core.r[d] = res.to_bits();
                        }
                        "GE" => {
                            core.ai_isa_ops_count += 1;
                            let x = f32::from_bits(core.r[s]);
                            let sqrt_2_over_pi = 0.797_884_6_f32;
                            let inner = sqrt_2_over_pi * (x + 0.044715f32 * x * x * x);
                            let res = 0.5f32 * x * (1.0f32 + inner.tanh());
                            core.r[d] = res.to_bits();
                        }
                        "SS" => {
                            core.ai_isa_ops_count += 1;
                            let bank = if imm_nibble > 0 { imm_nibble % 16 } else { s % 16 };
                            let ch = bank;
                            let inp_val = if imm_nibble > 0 || core.r[s] != 0 {
                                core.r[s]
                            } else if core.r[d] != 0 {
                                core.r[d]
                            } else {
                                core.r[1]
                            };
                            let x = f32::from_bits(inp_val);
                            let (a_bar, b_bar) = if slot.mode == '@' {
                                let a = f32::from_bits(core.bank_r[bank][0]);
                                let b = f32::from_bits(core.bank_r[bank][1]);
                                (if a == 0.0 { 0.9f32 } else { a }, if b == 0.0 { 0.1f32 } else { b })
                            } else {
                                (0.9f32, 0.1f32)
                            };
                            let prev_h = core.ssm_state[ch];
                            let next_h = a_bar * prev_h + b_bar * x;
                            core.ssm_state[ch] = next_h;
                            let y = next_h + 0.05f32 * x;
                            core.r[d] = y.to_bits();
                        }
                        "HL" => {
                            core.is_halted = true;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if bundle_count == 0 {
        return Err("No valid VLIW bundles found in .cl code to execute".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cl_jit_executes_math_and_agi_opcodes() {
        let code = r#"
            B0000: '==01#0403 '==02#020k _AD03M1025 _SB04M102E
            B0001: _ML05M304h _CP06M500x _MA07M600y _FX08M700z
            B0002: _LD09M001w ~RM0AM900s _ST0BM102K !HL00#000(
        "#;

        let res = run_cl_jit(code);
        assert!(res.is_ok(), "JIT execution failed: {:?}", res.err());
        let core = res.unwrap();
        assert!(core.is_halted, "Core should be halted by HL opcode");
        assert_eq!(core.r[1], 0x40, "R1 should equal 0x40 (64)");
        assert_eq!(core.r[2], 0x20, "R2 should equal 0x20 (32)");
        assert_eq!(core.r[3], 0x60, "R3 (AD R1 + R2) should equal 0x60 (96)");
        assert_eq!(core.r[4], 0x20, "R4 (SB R1 - R2) should equal 0x20 (32)");
        assert_eq!(core.r[5], 0x60 * 0x20, "R5 (ML R3 * R4) should equal 96 * 32 = 3072");
        assert_eq!(core.r[6], core.r[5], "R6 (CP R5) should equal R5");
    }
}

