// ============================================================================
// CRON Decompiler — SSS+ Machine-to-Blueprint Reverse Semantic Decompiler (.cl to .cr)
// Translates 128-bit VLIW machine bundles into human-readable, verifiable .cr code.
// Core Philosophy: .cl is the AI-native silicon language; .cr is the human cognitive lens.
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedSlot {
    pub raw: String,
    pub prefix: char,
    pub op: String,
    pub dest: usize,
    pub dest_bank: usize,
    pub mode: char,
    pub src: usize,
    pub parity: char,
    pub imm: usize,
    pub term: char,
    pub imm_val: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedBundle {
    pub cycle: usize,
    pub slots: Vec<ParsedSlot>,
}

pub fn parse_slot_str(slot: &str) -> Option<ParsedSlot> {
    if slot.len() < 3 {
        return None;
    }
    let chars: Vec<char> = slot.chars().collect();
    let prefix = chars[0];

    if slot.starts_with("'=") || slot.starts_with("==") {
        // Immediate load slot: '= <dest_bank> <dest_reg> # <imm16> >
        let op = if slot.len() >= 3 {
            slot[1..3].to_string()
        } else {
            "==".to_string()
        };
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
        let mode = if chars.len() >= 5 { chars[4] } else { '#' };
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
        let term = chars.last().copied().unwrap_or('>');

        return Some(ParsedSlot {
            raw: slot.to_string(),
            prefix,
            op,
            dest: dest_reg,
            dest_bank,
            mode,
            src: 0,
            parity: '0',
            imm: 0,
            term,
            imm_val,
        });
    }

    // Standard slot: [0]=prefix, [1..3]=op, [3]=dest_bank, [4]=dest_reg, [5]=mode, [6]=src, [7]=parity, [8]=imm, [9]=term
    let op = slot[1..3].to_string();
    let dest_bank = if chars.len() >= 4 {
        chars[3].to_digit(16).unwrap_or(0) as usize
    } else {
        0
    };
    let dest_reg = if chars.len() >= 5 {
        chars[4].to_digit(16).unwrap_or(0) as usize
    } else {
        0
    };
    let mode = if chars.len() >= 6 { chars[5] } else { '$' };
    let src = if chars.len() >= 7 { chars[6].to_digit(16).unwrap_or(0) as usize } else { 0 };
    let parity = if chars.len() >= 8 { chars[7] } else { '0' };
    let imm = if chars.len() >= 9 { chars[8].to_digit(16).unwrap_or(0) as usize } else { 0 };
    let term = if chars.len() >= 10 { chars[9] } else { '>' };

    Some(ParsedSlot {
        raw: slot.to_string(),
        prefix,
        op,
        dest: dest_reg,
        dest_bank,
        mode,
        src,
        parity,
        imm,
        term,
        imm_val: None,
    })
}

#[derive(Debug, Clone)]
pub struct DecompileStats {
    pub total_bundles: usize,
    pub total_slots: usize,
    pub optical_ops: usize,
    pub reversible_ops: usize,
    pub stdp_ops: usize,
    pub noc_ops: usize,
    pub pgas_ops: usize,
    pub ssa_variables_generated: usize,
}

pub fn parse_cl_bundles(cl_code: &str) -> Result<Vec<ParsedBundle>, String> {
    let mut bundles: Vec<ParsedBundle> = Vec::new();

    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }

        if let Some((cycle_part, slots_part)) = trimmed.split_once(':') {
            let cycle = cycle_part
                .trim()
                .trim_start_matches('B')
                .parse::<usize>()
                .unwrap_or(bundles.len());

            let mut bundle_slots = Vec::new();
            for slot_str in slots_part.split_whitespace() {
                if let Some(parsed) = parse_slot_str(slot_str) {
                    if parsed.op != "NO" {
                        bundle_slots.push(parsed);
                    }
                }
            }

            if !bundle_slots.is_empty() {
                bundles.push(ParsedBundle {
                    cycle,
                    slots: bundle_slots,
                });
            }
        }
    }

    if bundles.is_empty() {
        return Err("No valid instruction bundles found in .cl machine code".to_string());
    }

    Ok(bundles)
}

pub fn decompile_cl(cl_code: &str) -> Result<String, String> {
    decompile_cl_with_name(cl_code, "DecompiledCognitiveCore")
}

pub fn decompile_cl_with_name(cl_code: &str, module_name: &str) -> Result<String, String> {
    let bundles = parse_cl_bundles(cl_code)?;

    let mut all_slots = Vec::new();
    for b in &bundles {
        for s in &b.slots {
            all_slots.push(s.clone());
        }
    }

    // Inspect presence of structural features
    let has_spawn = all_slots.iter().any(|s| s.op == "SP");
    let has_sentry = all_slots.iter().any(|s| s.op == "SH" || s.op == "99");
    let has_region = all_slots.iter().any(|s| s.op == "RS" || s.op == "88");
    let has_fallback = all_slots.iter().any(|s| s.op == "RC" && s.mode != 'C' && s.mode != '!');
    let has_barrier = all_slots.iter().any(|s| s.op == "bb");
    let has_optical = all_slots.iter().any(|s| s.op == "OP" || s.op == "WD");
    let has_fuse = all_slots.iter().any(|s| s.op == "FU" || s.op == "FE");
    let has_clifford = cl_code.contains(".clifford")
        || cl_code.contains("clifford_so4_rot")
        || (all_slots.iter().any(|s| s.op == "TT") && all_slots.iter().filter(|s| s.op == "MD").count() >= 4);

    let mut out = String::new();
    out.push_str("// ============================================================================\n");
    out.push_str("// DECOMPILED FROM CRON SILICON MACHINE-NATIVE (.cl)\n");
    out.push_str("// Universal Reverse Semantic Projection Engine (.cl -> .cr)\n");
    out.push_str("// Target: 256-Core 4D-Torus Neuromorphic Photonic Silicon Architecture\n");
    out.push_str("// ============================================================================\n\n");
    out.push_str(&format!(".MODULE {}\n", module_name));
    out.push_str(".ENTRY _main\n\n");

    if has_clifford {
        out.push_str("// CRON Cl(4,0) Spacetime Clifford Algebra Engine\n");
        out.push_str("use cron.clifford::{Rotor4D, Vector4D, rotor_rotate_vector}\n\n");
    }

    if has_spawn {
        out.push_str("async def background_fiber_task(channel: u32) -> u32 {\n");
        out.push_str("    let recvd = await spatial_gather(axis=channel)\n");
        out.push_str("    return recvd\n");
        out.push_str("}\n\n");
    }

    out.push_str("_main:\n");

    // Standard base configuration
    out.push_str("    let ext_hbm_base: ext_addr_t = 0x000A_0000\n");
    if has_optical {
        out.push_str("    let lin w_seed: wave_t = pack_wave(amp=[64, 32, 16, 8], phase=[32, 32, 64, 64])\n\n");
    } else {
        out.push('\n');
    }

    let mut indent = "    ";

    if has_clifford {
        out.push_str(&format!("{}// Cl(4,0) Rotor-Vector Spacetime Rotation\n", indent));
        out.push_str(&format!("{}let rotor = Rotor4D {{ s: 1.0, e12: 0.0, e13: 0.0, e14: 0.0, e23: 0.0, e24: 0.0, e34: 0.0, p: 0.0 }}\n", indent));
        out.push_str(&format!("{}let v = Vector4D {{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 }}\n", indent));
        out.push_str(&format!("{}let v_rot = rotor_rotate_vector(rotor, v)\n\n", indent));
    }

    if has_sentry {
        out.push_str(&format!("{}resilient_compute [fallback_target=X+, max_thermal_thresh=180] {{\n", indent));
        indent = "        ";
    }

    if has_spawn {
        out.push_str(&format!("{}let task_handle = spawn background_fiber_task(channel=2)\n\n", indent));
    }

    if has_region {
        out.push_str(&format!("{}region TileProcessingArena [target=SELF] {{\n", indent));
        indent = "            ";
    }

    if has_fuse {
        out.push_str(&format!("{}fuse [tile=(4, 4), stream=SRAM] {{\n", indent));
        indent = match indent {
            "            " => "                ",
            "        " => "            ",
            _ => "        ",
        };
    }

    // Register SSA name tracking table
    let mut reg_exprs: [String; 16] = Default::default();
    for (i, reg) in reg_exprs.iter_mut().enumerate() {
        *reg = format!("r{}", i);
    }
    // Seed initial hardware registers
    reg_exprs[0] = "0".to_string();
    reg_exprs[1] = "ext_hbm_base".to_string();
    if has_optical {
        reg_exprs[4] = "w_seed".to_string();
    }

    let mut linear_vars: HashSet<String> = HashSet::new();
    if has_optical {
        linear_vars.insert("w_seed".to_string());
    }

    let mut declared_names: HashSet<String> = HashSet::new();
    declared_names.insert("ext_hbm_base".to_string());
    if has_optical {
        declared_names.insert("w_seed".to_string());
    }

    let mut ssa_counter: HashMap<String, usize> = HashMap::new();
    let mut last_assigned_var = "ext_hbm_base".to_string();

    for bundle in &bundles {
        for slot in &bundle.slots {
            let d = slot.dest;
            let s = slot.src;

            match slot.op.as_str() {
                op if op.starts_with('=') || slot.prefix == '\'' => {
                    // Immediate constant load
                    let val = slot.imm_val.unwrap_or(slot.imm as u32);
                    let var_name = if (has_region || has_optical) && d == 1 && val == 0x000A_0000 {
                        "ext_hbm_base".to_string()
                    } else {
                        let count = ssa_counter.entry(format!("c{}", d)).or_insert(0);
                        *count += 1;
                        if *count == 1 {
                            format!("const_r{}", d)
                        } else {
                            format!("const_r{}_{}", d, count)
                        }
                    };

                    if !declared_names.contains(&var_name) {
                        out.push_str(&format!("{}let {}: u32 = {:#06X}\n", indent, var_name, val));
                        declared_names.insert(var_name.clone());
                    }
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "PK" => {
                    let var_name = "packed_w".to_string();
                    let src_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}let {} = pack_subbyte({} as u32, precision=2)\n",
                        indent, var_name, src_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "TL" => {
                    let var_name = "my_tile".to_string();
                    let src_name = &reg_exprs[s];
                    if !linear_vars.contains(&var_name) {
                        out.push_str(&format!(
                            "{}let lin {} = bind_local_tile({})\n",
                            indent, var_name, src_name
                        ));
                        linear_vars.insert(var_name.clone());
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    }
                }

                "OP" => {
                    let src_name = &reg_exprs[s];
                    let var_name = "opt_latent".to_string();
                    let consume_call = if linear_vars.contains(src_name) {
                        format!("consume({})", src_name)
                    } else {
                        src_name.clone()
                    };
                    out.push_str(&format!(
                        "{}let grad lin {} = optical_gemm(wave={})\n",
                        indent, var_name, consume_call
                    ));
                    linear_vars.remove(src_name);
                    linear_vars.insert(var_name.clone());
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "WD" => {
                    let src_name = &reg_exprs[s];
                    let var_name = "wdm_latent".to_string();
                    out.push_str(&format!(
                        "{}let lin {} = wdm_photonic_gemm(consume({}))\n",
                        indent, var_name, src_name
                    ));
                    linear_vars.remove(src_name);
                    linear_vars.insert(var_name.clone());
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "FA" => {
                    let src_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}// Forward autodiff tap on {}\n",
                        indent, src_name
                    ));
                }

                "TT" => {
                    let src_name = &reg_exprs[s];
                    let var_name = format!("trans_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = tensor_transpose({})\n",
                        indent, var_name, src_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "PO" | "P0" | "P1" => {
                    let (op1_name, op2_name) = if slot.imm > 0 && slot.imm < 16 && s > 0 {
                        (reg_exprs[s].clone(), reg_exprs[slot.imm].clone())
                    } else {
                        (reg_exprs[d].clone(), reg_exprs[s].clone())
                    };

                    let op_symbol = match slot.mode {
                        '+' => Some("+"),
                        '-' => Some("-"),
                        '*' => Some("*"),
                        '/' => Some("/"),
                        '%' => Some("%"),
                        '&' => Some("&"),
                        '|' => Some("|"),
                        '^' => Some("^"),
                        '<' => Some("<"),
                        '>' => Some(">"),
                        '=' => Some("=="),
                        'G' => Some(">="),
                        _ => match slot.imm {
                            1 => Some("+"),
                            2 => Some("-"),
                            3 => Some("*"),
                            4 => Some("/"),
                            5 => Some("%"),
                            6 => Some("&"),
                            7 => Some("|"),
                            8 => Some("^"),
                            0xB => Some("=="),
                            0xD => Some("<"),
                            0xE => Some("<="),
                            0xF => Some(">"),
                            _ => None,
                        },
                    };

                    if let Some(op_str) = op_symbol {
                        let count = ssa_counter.entry(format!("r{}", d)).or_insert(0);
                        *count += 1;
                        let var_name = format!("r{}_{}", d, count);
                        out.push_str(&format!(
                            "{}let {} = {} {} {}\n",
                            indent, var_name, op1_name, op_str, op2_name
                        ));
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    } else if linear_vars.contains(&op1_name) && !linear_vars.contains("masked_act") {
                        let var_name = "masked_act".to_string();
                        out.push_str(&format!(
                            "{}let lin {} = predicated_op({}, {}, mask=0x03, op=MZI_MUL)\n",
                            indent, var_name, op1_name, op2_name
                        ));
                        linear_vars.insert(var_name.clone());
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    } else {
                        let count = ssa_counter.entry(format!("r{}", d)).or_insert(0);
                        *count += 1;
                        let var_name = format!("act_r{}_{}", d, count);
                        out.push_str(&format!(
                            "{}let {} = simd_fma({}, {}, 0)\n",
                            indent, var_name, op1_name, op2_name
                        ));
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    }
                }

                "MD" => {
                    let d_name = reg_exprs[d].clone();
                    let s_name = reg_exprs[s].clone();

                    if slot.mode == '.' || slot.imm == 5 || slot.imm == 6 {
                        let var_name = "ternary_acc".to_string();
                        let s_call = if linear_vars.contains(&s_name) {
                            linear_vars.remove(&s_name);
                            format!("consume({})", s_name)
                        } else {
                            s_name
                        };
                        out.push_str(&format!(
                            "{}let lin {} = subbyte_dot({}, {}, precision=2)\n",
                            indent, var_name, d_name, s_call
                        ));
                        linear_vars.insert(var_name.clone());
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    } else if slot.mode == '*' || slot.imm == 3 {
                        let count = ssa_counter.entry(format!("prod_r{}", d)).or_insert(0);
                        *count += 1;
                        let var_name = format!("prod_r{}_{}", d, count);
                        out.push_str(&format!(
                            "{}let {} = {} * {}\n",
                            indent, var_name, d_name, s_name
                        ));
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    }
                }

                "BK" => {
                    let s_name = &reg_exprs[s];
                    let var_name = "d_weights".to_string();
                    if !linear_vars.contains(&var_name) {
                        out.push_str(&format!(
                            "{}let lin {} = backward({})\n",
                            indent, var_name, s_name
                        ));
                        linear_vars.insert(var_name.clone());
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    }
                }

                "BL" => {
                    let s_name = reg_exprs[s].clone();
                    let var_name = "halo_tile".to_string();
                    let s_call = if linear_vars.contains(&s_name) {
                        linear_vars.remove(&s_name);
                        format!("consume({})", s_name)
                    } else {
                        s_name
                    };
                    out.push_str(&format!(
                        "{}let lin {} = blend_staged_halo({}, buffer=STAGING_BUF_1)\n",
                        indent, var_name, s_call
                    ));
                    linear_vars.insert(var_name.clone());
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "RF" => {
                    let d_name = reg_exprs[d].clone();
                    let s_name = reg_exprs[s].clone();
                    if linear_vars.contains(&d_name) && !linear_vars.contains("rev_state") {
                        let var_name = "rev_state".to_string();
                        linear_vars.remove(&d_name);
                        out.push_str(&format!(
                            "{}let lin {} = reversible_entangle(consume({}), {})\n",
                            indent, var_name, d_name, s_name
                        ));
                        linear_vars.insert(var_name.clone());
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    } else {
                        out.push_str(&format!(
                            "{}reversible_swap(&mut {}, &mut {})\n",
                            indent, d_name, s_name
                        ));
                        let tmp = reg_exprs[d].clone();
                        reg_exprs[d] = reg_exprs[s].clone();
                        reg_exprs[s] = tmp;
                    }
                }

                "TO" => {
                    let d_name = &reg_exprs[d];
                    out.push_str(&format!(
                        "{}toffoli_gate(&mut {}, &r10, &r11)\n",
                        indent, d_name
                    ));
                }

                "GU" => {
                    let d_name = reg_exprs[d].clone();
                    let s_name = reg_exprs[s].clone();
                    let var_name = "updated_w".to_string();
                    let d_call = if linear_vars.contains(&d_name) {
                        linear_vars.remove(&d_name);
                        format!("consume({})", d_name)
                    } else {
                        d_name
                    };
                    let s_call = if linear_vars.contains(&s_name) {
                        linear_vars.remove(&s_name);
                        format!("consume({})", s_name)
                    } else {
                        s_name
                    };
                    out.push_str(&format!(
                        "{}let lin {} = apply_gradient_step({}, {}, lr=0)\n",
                        indent, var_name, d_call, s_call
                    ));
                    linear_vars.insert(var_name.clone());
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "ST" => {
                    if slot.imm == 0 {
                        out.push_str(&format!(
                            "{}stage_prefetch(from=X+, buffer=STAGING_BUF_1)\n",
                            indent
                        ));
                    } else {
                        let var_name = "syn_state".to_string();
                        if !linear_vars.contains(&var_name) {
                            let acc_name = if linear_vars.contains("ternary_acc") {
                                linear_vars.remove("ternary_acc");
                                "consume(ternary_acc)"
                            } else {
                                "0"
                            };
                            out.push_str(&format!(
                                "{}let lin {} = step_synaptic_plasticity({}, rate={})\n",
                                indent, var_name, acc_name, slot.imm * 16 + 4
                            ));
                            linear_vars.insert(var_name.clone());
                            reg_exprs[d] = var_name.clone();
                            last_assigned_var = var_name;
                        }
                    }
                }

                "LI" => {
                    let s_name = &reg_exprs[s];
                    let var_name = format!("spike_r{}", d);
                    let thresh = if slot.imm > 0 { slot.imm * 10 } else { 50 };
                    out.push_str(&format!(
                        "{}let {} = lif_neuron_step({}, thresh={})\n",
                        indent, var_name, s_name, thresh
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "LF" => {
                    let s_name = &reg_exprs[s];
                    let var_name = format!("spike_gen_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = lif_spike({})\n",
                        indent, var_name, s_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "SY" => {
                    let s_name = reg_exprs[s].clone();
                    let s_call = if linear_vars.contains(&s_name) {
                        linear_vars.remove(&s_name);
                        format!("consume({})", s_name)
                    } else {
                        s_name
                    };
                    out.push_str(&format!(
                        "{}let sym_concept = ground_to_symbol({})\n",
                        indent, s_call
                    ));
                    last_assigned_var = "sym_concept".to_string();
                }

                "KG" => {
                    let s_name = reg_exprs[s].clone();
                    let s_call = if linear_vars.contains(&s_name) {
                        linear_vars.remove(&s_name);
                        format!("consume({})", s_name)
                    } else {
                        s_name
                    };
                    out.push_str(&format!(
                        "{}let kg_fact = deduce_causal_chain({})\n",
                        indent, s_call
                    ));
                    last_assigned_var = "kg_fact".to_string();
                }

                "SW" => {
                    let var_name = "branch_state".to_string();
                    out.push_str(&format!(
                        "{}let lin {} = branch_in_superposition(branch_id=0)\n",
                        indent, var_name
                    ));
                    linear_vars.insert(var_name);
                }

                "TX" => {
                    let s_idx = if s > 0 { s } else { d };
                    let s_name = &reg_exprs[s_idx];
                    out.push_str(&format!(
                        "{}channel_send(axis=X+, data={})\n",
                        indent, s_name
                    ));
                }

                "RX" => {
                    let var_name = format!("recvd_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = channel_recv(axis=X-)\n",
                        indent, var_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "WH" => {
                    let s_name = &reg_exprs[s];
                    let var_name = format!("wormhole_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = wormhole_tunnel({}, target=0x55)\n",
                        indent, var_name, s_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "SB" => {
                    let s_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}spatial_broadcast({})\n",
                        indent, s_name
                    ));
                }

                "DF" => {
                    let s_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}adaptive_deflect_route({})\n",
                        indent, s_name
                    ));
                }

                "bb" => {
                    out.push_str(&format!(
                        "{}pgas_barrier() // 256-Core Chip-Wide Hardware Barrier\n",
                        indent
                    ));
                }

                "CC" => {
                    out.push_str(&format!(
                        "{}cache_invalidate_all()\n",
                        indent
                    ));
                }

                "DD" => {
                    let s_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}direct_dma_burst({})\n",
                        indent, s_name
                    ));
                }

                "EE" => {
                    out.push_str(&format!(
                        "{}dvfs_power_state(mode=ECO, energy_budget=450)\n",
                        indent
                    ));
                }

                "11" => {
                    out.push_str(&format!(
                        "{}photonic_laser_pump()\n",
                        indent
                    ));
                }

                "88" | "RS" => {
                    out.push_str(&format!(
                        "{}region_arena_reset()\n",
                        indent
                    ));
                }

                "99" => {
                    out.push_str(&format!(
                        "{}sentry_watchdog_trip()\n",
                        indent
                    ));
                }

                "ee" => {
                    out.push_str(&format!(
                        "{}neuromorphic_spike_broadcast()\n",
                        indent
                    ));
                }

                "CS" => {
                    let d_name = &reg_exprs[d];
                    let s_name = &reg_exprs[s];
                    out.push_str(&format!(
                        "{}compare_and_swap(&mut {}, expected={}, new_val={})\n",
                        indent, d_name, s_name, slot.imm
                    ));
                }

                "PS" => {
                    let s_name = &reg_exprs[s];
                    let var_name = format!("prefix_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = simd_prefix_sum({})\n",
                        indent, var_name, s_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "CD" => {
                    let s_name = &reg_exprs[s];
                    let var_name = format!("cordic_r{}", d);
                    out.push_str(&format!(
                        "{}let {} = cordic_sincos({})\n",
                        indent, var_name, s_name
                    ));
                    reg_exprs[d] = var_name.clone();
                    last_assigned_var = var_name;
                }

                "RC" => {
                    if slot.imm == 0 && slot.mode == '!' {
                        out.push_str(&format!(
                            "{}resilient_checkpoint_save()\n",
                            indent
                        ));
                    } else if slot.imm == 1 && slot.mode == '!' {
                        out.push_str(&format!(
                            "{}resilient_checkpoint_restore()\n",
                            indent
                        ));
                    } else if slot.mode == 'C' {
                        let var_name = format!("csr_stat_r{}", d);
                        out.push_str(&format!(
                            "{}let {} = read_hardware_csr({})\n",
                            indent, var_name, s
                        ));
                        reg_exprs[d] = var_name.clone();
                        last_assigned_var = var_name;
                    }
                }

                _ => {}
            }
        }
    }

    // Sound linear affine cleanup: consume remaining unconsumed linear resources
    if linear_vars.contains("syn_state") {
        out.push_str(&format!("{}consume(syn_state)\n", indent));
        linear_vars.remove("syn_state");
    }

    if linear_vars.contains("branch_state") {
        out.push_str(&format!("{}consume(branch_state)\n", indent));
        linear_vars.remove("branch_state");
    }

    if linear_vars.contains("halo_tile") {
        out.push_str(&format!("{}consume(halo_tile)\n", indent));
        linear_vars.remove("halo_tile");
    }

    if linear_vars.contains("my_tile") {
        out.push_str(&format!("{}consume(my_tile)\n", indent));
        linear_vars.remove("my_tile");
    }

    if linear_vars.contains("updated_w") {
        out.push_str(&format!("{}export updated_w as final_core_state\n", indent));
        linear_vars.remove("updated_w");
    } else if linear_vars.contains("w_seed") {
        out.push_str(&format!("{}export w_seed as final_core_state\n", indent));
        linear_vars.remove("w_seed");
    } else if linear_vars.contains("opt_latent") {
        out.push_str(&format!("{}export opt_latent as final_core_state\n", indent));
        linear_vars.remove("opt_latent");
    }

    // Consume any remaining linear variables
    let remaining_linear: Vec<String> = linear_vars.into_iter().collect();
    for lin_var in remaining_linear {
        out.push_str(&format!("{}consume({})\n", indent, lin_var));
    }

    if has_region && last_assigned_var != "ext_hbm_base" && last_assigned_var != "w_seed" && !last_assigned_var.is_empty() {
        let exp_name = format!("exported_{}", last_assigned_var);
        out.push_str(&format!("{}export {} as {}\n", indent, last_assigned_var, exp_name));
        last_assigned_var = exp_name;
    }

    if has_fuse {
        indent = match indent {
            "                " => "            ",
            "            " => "        ",
            _ => "    ",
        };
        out.push_str(&format!("{}}}\n\n", indent));
    }

    if has_region {
        indent = "        ";
        out.push_str(&format!("{}}}\n\n", indent));
    }

    if has_spawn {
        out.push_str(&format!("{}let bg_result = await task_handle\n", indent));
    }

    out.push_str(&format!("{}await_dma_channel(channel=1)\n", indent));
    if last_assigned_var != "ext_hbm_base" && last_assigned_var != "w_seed" {
        out.push_str(&format!("{}spatial_broadcast({})\n", indent, last_assigned_var));
    } else {
        out.push_str(&format!("{}spatial_broadcast(final_core_state)\n", indent));
    }

    if has_sentry {
        indent = "    ";
        out.push_str(&format!("{}}}\n", indent));

        if has_fallback {
            out.push_str(&format!("{}fallback {{\n", indent));
            out.push_str("        let lin fallback_data = recover_from_neighbor(axis=X-)\n");
            out.push_str("        spatial_broadcast(consume(fallback_data))\n");
            out.push_str(&format!("{}}}\n\n", indent));
        }
    }

    if has_barrier {
        out.push_str(&format!("{}pgas_barrier()\n", indent));
    }

    out.push_str(".END\n");

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_slot_str_variants() {
        // Optical GEMM slot (dest=4, mode=$, src=1, parity=0, imm=0, term=>) -> 10 chars
        let slot1 = parse_slot_str("_OP04$100>").expect("Valid slot");
        assert_eq!(slot1.prefix, '_');
        assert_eq!(slot1.op, "OP");
        assert_eq!(slot1.dest, 4);
        assert_eq!(slot1.src, 1);
        assert_eq!(slot1.term, '>');

        // Immediate load slot with # hex delimiter
        let slot2 = parse_slot_str("'=01#000A>").expect("Valid immediate slot");
        assert_eq!(slot2.prefix, '\'');
        assert_eq!(slot2.op, "=0");
        assert_eq!(slot2.dest, 1);
        assert_eq!(slot2.imm_val, Some(0x000A));

        // ALU Predicated Addition slot (10 chars: prefix '_', op "PO", bank 0, dest 6, mode '+', src 5, parity '0', imm 3, term '>')
        let slot3 = parse_slot_str("_PO06+503>").expect("Valid ALU slot");
        assert_eq!(slot3.op, "PO");
        assert_eq!(slot3.dest, 6);
        assert_eq!(slot3.mode, '+');
        assert_eq!(slot3.src, 5);
        assert_eq!(slot3.imm, 3);

        // Reversible Fredkin Swap slot (10 chars: prefix '_', op "RF", bank 0, dest 10, mode '$', src 11, parity '0', imm 0, term '>')
        let slot4 = parse_slot_str("_RF0A$B00>").expect("Valid RF slot");
        assert_eq!(slot4.op, "RF");
        assert_eq!(slot4.dest, 10);
        assert_eq!(slot4.src, 11);

        // Chip barrier slot (10 chars)
        let slot5 = parse_slot_str("_bb00$000>").expect("Valid barrier slot");
        assert_eq!(slot5.op, "bb");
    }

    #[test]
    fn test_decompile_cl_arithmetic_pipeline() {
        let cl_code = r#"
B0000: '=01#000A> '=02#0014> _NO00$000> _NO00$000>
B0001: _PO03+102> _TX00$300> _bb00$000> _HL00$000>
"#;
        let decompiled = decompile_cl(cl_code).expect("Should decompile arithmetic cl");
        assert!(decompiled.contains(".MODULE DecompiledCognitiveCore"));
        assert!(decompiled.contains(".ENTRY _main"));
        assert!(decompiled.contains("const_r1"));
        assert!(decompiled.contains("const_r2"));
        assert!(decompiled.contains("const_r1 + const_r2"));
        assert!(decompiled.contains("channel_send(axis=X+, data="));
        assert!(decompiled.contains("pgas_barrier()"));
        assert!(decompiled.contains(".END"));
    }

    #[test]
    fn test_decompile_photonic_and_reversible() {
        let cl_code = r#"
B0000: _RS00$0000> _SH00$0000> '=01#000A> _NO00$0000>
B0001: _OP04$0100> _TT03$0200> _RF04$0300> _NO00$0000>
B0002: _ST0C$0084> _bb00$0000> _HL00$0000> _NO00$0000>
"#;
        let decompiled = decompile_cl(cl_code).expect("Should decompile photonic kernel");
        assert!(decompiled.contains("resilient_compute"));
        assert!(decompiled.contains("region TileProcessingArena"));
        assert!(decompiled.contains("optical_gemm"));
        assert!(decompiled.contains("tensor_transpose"));
        assert!(decompiled.contains("step_synaptic_plasticity"));
        assert!(decompiled.contains("pgas_barrier"));
        // Check linear type sound cleanup
        assert!(decompiled.contains("consume") || decompiled.contains("export"));
    }

    #[test]
    fn test_decompile_kernel_fusion() {
        let cl_code = r#"
B0000: '=01#000A> '=02#0014> _FU00$000> _NO00$000>
B0001: _PO03+102> _FE00$000> _bb00$000> _HL00$000>
"#;
        let decompiled = decompile_cl(cl_code).expect("Should decompile fused kernel cl");
        assert!(decompiled.contains("fuse [tile=(4, 4), stream=SRAM] {"));
        assert!(decompiled.contains("const_r1 + const_r2"));
        assert!(decompiled.contains("}"));
    }
}

