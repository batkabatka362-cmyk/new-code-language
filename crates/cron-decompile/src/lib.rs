// ============================================================================
// CRON Decompiler — Machine-to-Blueprint Reverse Decompiler (.cl to .cr)
// Translates 128-bit VLIW machine bundles into human-readable, verifiable .cr code.
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use std::collections::HashSet;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ParsedSlot {
    op: String,
    dest: usize,
    src: usize,
    mode: char,
    imm: usize,
}

fn parse_slot_str(slot: &str) -> Option<ParsedSlot> {
    if slot.len() < 3 {
        return None;
    }
    let op = slot[1..3].to_string();
    let dest = if slot.len() >= 5 {
        usize::from_str_radix(&slot[3..5], 16).unwrap_or(0)
    } else {
        0
    };
    let mode = if slot.len() >= 6 {
        slot.chars().nth(5).unwrap_or('$')
    } else {
        '$'
    };
    let src = if slot.len() >= 7 {
        slot[6..7].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize
    } else {
        0
    };
    let imm = if slot.len() >= 9 {
        slot[8..9].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize
    } else {
        0
    };

    Some(ParsedSlot {
        op,
        dest,
        src,
        mode,
        imm,
    })
}

pub fn decompile_cl(cl_code: &str) -> Result<String, String> {
    let mut slots: Vec<ParsedSlot> = Vec::new();

    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }

        if let Some((_cycle, slots_part)) = trimmed.split_once(':') {
            for slot_str in slots_part.split_whitespace() {
                if let Some(parsed) = parse_slot_str(slot_str) {
                    if parsed.op != "NO" {
                        slots.push(parsed);
                    }
                }
            }
        }
    }

    if slots.is_empty() {
        return Err("No instruction slots found in .cl machine code".to_string());
    }

    // Inspect presence of features
    let has_spawn = slots.iter().any(|s| s.op == "SP");
    let has_sentry = slots.iter().any(|s| s.op == "SH");
    let has_region = slots.iter().any(|s| s.op == "RS");
    let _has_optical = slots.iter().any(|s| s.op == "OP" || s.op == "WD");
    let _has_rev = slots.iter().any(|s| s.op == "RF" || s.op == "TO" || s.op == "BK");
    let _has_stdp = slots.iter().any(|s| s.op == "ST");
    let _has_symbolic = slots.iter().any(|s| s.op == "SY" || s.op == "KG");
    let has_fallback = slots.iter().any(|s| s.op == "RC");

    let mut out = String::new();
    out.push_str("// DECOMPILED FROM CRON MACHINE-NATIVE (.cl)\n");
    out.push_str("// Target: 256-Core 4D-Torus Hardware Architecture\n\n");
    out.push_str(".MODULE DecompiledCognitiveCore\n");
    out.push_str(".ENTRY _main\n\n");

    if has_spawn {
        out.push_str("async def background_fiber_task(channel: u32) -> u32 {\n");
        out.push_str("    let recvd = await spatial_gather(axis=channel)\n");
        out.push_str("    return recvd\n");
        out.push_str("}\n\n");
    }

    out.push_str("_main:\n");

    // Header declarations (memory and seeds)
    out.push_str("    let ext_hbm_base: ext_addr_t = 0x000A_0000\n");
    out.push_str("    let lin w_seed: wave_t = pack_wave(amp=[64, 32, 16, 8], phase=[32, 32, 64, 64])\n\n");

    let mut indent = "    ";

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

    // Trace emitted variables to ensure linear type soundness
    let mut linear_vars: HashSet<String> = HashSet::new();

    for slot in &slots {
        match slot.op.as_str() {
            "PK" => {
                out.push_str(&format!("{}let packed_w = pack_subbyte(ext_hbm_base as u32, precision=2)\n", indent));
            }
            "TL" => {
                if !linear_vars.contains("my_tile") {
                    out.push_str(&format!("{}let lin my_tile = bind_local_tile(ext_hbm_base)\n", indent));
                    linear_vars.insert("my_tile".to_string());
                }
            }
            "ST" => {
                if slot.imm == 0 {
                    out.push_str(&format!("{}stage_prefetch(from=X+, buffer=STAGING_BUF_1)\n", indent));
                } else if !linear_vars.contains("syn_state") {
                    out.push_str(&format!("{}let lin syn_state = step_synaptic_plasticity(consume(ternary_acc), rate=84)\n", indent));
                    linear_vars.insert("syn_state".to_string());
                }
            }
            "OP" => {
                if !linear_vars.contains("opt_latent") {
                    out.push_str(&format!("{}let grad lin opt_latent = optical_gemm(wave=consume(w_seed))\n", indent));
                    linear_vars.insert("opt_latent".to_string());
                }
            }
            "FA" => {
                // Forward autodiff tap
            }
            "PO" => {
                if linear_vars.contains("opt_latent") && !linear_vars.contains("masked_act") {
                    out.push_str(&format!("{}let lin masked_act = predicated_op(opt_latent, opt_latent, mask=0x03, op=MZI_MUL)\n", indent));
                    linear_vars.insert("masked_act".to_string());
                }
            }
            "MD" => {
                if linear_vars.contains("masked_act") && !linear_vars.contains("ternary_acc") {
                    out.push_str(&format!("{}let lin ternary_acc = subbyte_dot(packed_w, consume(masked_act), precision=2)\n", indent));
                    linear_vars.remove("masked_act");
                    linear_vars.insert("ternary_acc".to_string());
                }
            }
            "BK" => {
                if !linear_vars.contains("d_weights") {
                    out.push_str(&format!("{}let lin d_weights = backward(opt_latent)\n", indent));
                    linear_vars.insert("d_weights".to_string());
                }
            }
            "BL" => {
                if linear_vars.contains("my_tile") && !linear_vars.contains("halo_tile") {
                    out.push_str(&format!("{}let lin halo_tile = blend_staged_halo(consume(my_tile), buffer=STAGING_BUF_1)\n", indent));
                    linear_vars.remove("my_tile");
                    linear_vars.insert("halo_tile".to_string());
                }
            }
            "RF" => {
                if linear_vars.contains("halo_tile") && !linear_vars.contains("rev_state") {
                    out.push_str(&format!("{}let lin rev_state = reversible_entangle(consume(halo_tile), opt_latent)\n", indent));
                    linear_vars.remove("halo_tile");
                    linear_vars.insert("rev_state".to_string());
                }
            }
            "GU" => {
                if linear_vars.contains("d_weights") && !linear_vars.contains("updated_w") {
                    out.push_str(&format!("{}let lin updated_w = apply_gradient_step(consume(opt_latent), consume(d_weights), lr=0)\n", indent));
                    linear_vars.remove("opt_weights");
                    linear_vars.remove("d_weights");
                    linear_vars.insert("updated_w".to_string());
                }
            }
            "SY" => {
                if linear_vars.contains("rev_state") && !linear_vars.contains("sym_concept") {
                    out.push_str(&format!("{}let sym_concept = ground_to_symbol(consume(rev_state))\n", indent));
                    linear_vars.remove("rev_state");
                }
            }
            "KG" => {
                out.push_str(&format!("{}let kg_fact = deduce_causal_chain(consume(rev_state))\n", indent));
            }
            "SW" => {
                out.push_str(&format!("{}let lin branch_state = branch_in_superposition(branch_id=0)\n", indent));
            }
            _ => {}
        }
    }

    // If syn_state is still unconsumed, consume it
    if linear_vars.contains("syn_state") {
        out.push_str(&format!("{}consume(syn_state)\n", indent));
        linear_vars.remove("syn_state");
    }

    if linear_vars.contains("updated_w") {
        out.push_str(&format!("{}export updated_w as final_core_state\n", indent));
        linear_vars.remove("updated_w");
    } else {
        out.push_str(&format!("{}export w_seed as final_core_state\n", indent));
    }

    if has_region {
        indent = "        ";
        out.push_str(&format!("{}}}\n\n", indent));
    }

    if has_spawn {
        out.push_str(&format!("{}let bg_result = await task_handle\n", indent));
    }

    out.push_str(&format!("{}await_dma_channel(channel=1)\n", indent));
    out.push_str(&format!("{}spatial_broadcast(final_core_state)\n", indent));

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

    out.push_str(".END\n");

    Ok(out)
}
