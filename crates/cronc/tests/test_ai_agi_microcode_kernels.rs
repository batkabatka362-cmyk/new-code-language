// ============================================================================
// CRON AI/AGI Silicon Microcode Expansion Test Suite (60 Kernels)
// Architecture: 256-Core 4D-Torus Heterogeneous Silicon (4.00 Nominal IPC)
//
// New Advanced LLM / 7-Brain SAGI Kernels:
// 1. Grouped Query Attention (GQA-8)
// 2. Fused SwiGLU Mixture-of-Experts (MoE)
// 3. Striatal Actor-Critic Dopaminergic RL
// 4. Hippocampal Episodic Memory Replay
// 5. Thalamic Attention Crossbar Router
// 6. Paged Flash-Decoding Split-KV
// 7. BitNet b1.58 Trit Vector Dequantizer
// ============================================================================

use cronc::cl_macro::build_valid_slot;
use cronc::cl_vectorizer::{pack_slots_into_vliw_bundles, VectorizerConfig};
use cronc::cl_lint::lint_cl_source;
use cronc::cl_lang::verify_cl_program;
use cronc::cl_jit::run_cl_jit;

/// Generates Grouped Query Attention (GQA-8) Silicon Microcode Kernel
pub fn generate_gqa8_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: Query Head vector, R02: Shared KV Head vector, R03: Scale 1/sqrt(d)
    slots.push(build_valid_slot("'", "==01#040"));
    slots.push(build_valid_slot("'", "==02#020"));
    slots.push(build_valid_slot("'", "==03#010"));
    slots.push(build_valid_slot("_", "NO00#000"));

    // 8 Query heads projected against shared KV bank
    for h in 0..8 {
        slots.push(build_valid_slot("_", &format!("MD04M102"))); // Q_h * K_shared
        slots.push(build_valid_slot("_", &format!("ML05M403"))); // Scaled by 1/sqrt(d)
        slots.push(build_valid_slot("_", &format!("SM06M500"))); // Online Softmax
        slots.push(build_valid_slot("_", &format!("PO0{:X}M602", (h % 7) + 7))); // Attn * V_shared
    }

    slots.push(build_valid_slot("!", "HL00#000"));
    pack_slots_into_vliw_bundles("grouped_query_attention", &slots, &cfg).unwrap()
}

/// Generates Fused SwiGLU Mixture-of-Experts (MoE) Microcode Kernel
pub fn generate_fused_swiglu_moe_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: Router logits, R02: Input activation, R03: Gate weights, R04: Up weights
    slots.push(build_valid_slot("'", "==01#080"));
    slots.push(build_valid_slot("'", "==02#030"));
    slots.push(build_valid_slot("'", "==03#055"));
    slots.push(build_valid_slot("'", "==04#0AA"));

    // Top-2 Gating and SwiGLU forward
    slots.push(build_valid_slot("_", "TM05M203")); // Gate proj (Trit-MAC)
    slots.push(build_valid_slot("_", "TM06M204")); // Up proj (Trit-MAC)
    slots.push(build_valid_slot("_", "SI07M500")); // SiLU(Gate)
    slots.push(build_valid_slot("_", "ML08M706")); // SiLU(Gate) * Up

    // Router weighted contribution
    slots.push(build_valid_slot("_", "ML09M801")); // Expert contribution
    slots.push(build_valid_slot("~", "RM0AM900")); // RMSNorm projection output
    slots.push(build_valid_slot("!", "HL00#000"));

    pack_slots_into_vliw_bundles("fused_swiglu_expert", &slots, &cfg).unwrap()
}

/// Generates Striatal Actor-Critic Dopaminergic Reinforcement Learning Kernel
pub fn generate_striatal_actor_critic_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: State feature vector, R02: Reward signal, R03: Value estimate V(s), R04: Discount gamma
    slots.push(build_valid_slot("'", "==01#050"));
    slots.push(build_valid_slot("'", "==02#014")); // Reward = +20
    slots.push(build_valid_slot("'", "==03#010")); // V(s) = 16
    slots.push(build_valid_slot("'", "==04#07E")); // Gamma = 0.98 (~126/128)

    // Compute TD-Error: delta = R + gamma * V(s') - V(s)
    slots.push(build_valid_slot("_", "ML05M304")); // gamma * V(s')
    slots.push(build_valid_slot("_", "AD06M205")); // R + gamma * V(s')
    slots.push(build_valid_slot("_", "SB07M603")); // delta = TD error

    // Dopamine burst modulation tap ('DA') and policy weight update
    slots.push(build_valid_slot("_", "DA08M700")); // Dopamine phasic spike tap
    slots.push(build_valid_slot("_", "ML09M801")); // Delta * Eligibility trace
    slots.push(build_valid_slot("_", "AD0AM900")); // Update Critic weights
    slots.push(build_valid_slot("!", "HL00#000"));

    pack_slots_into_vliw_bundles("striatal_actor_critic", &slots, &cfg).unwrap()
}

/// Generates Hippocampal Episodic Memory Replay Kernel
pub fn generate_hippocampal_replay_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: Dentate Gyrus sparse index, R02: CA3 recurrence matrix base, R03: Quiescent sharp-wave threshold
    slots.push(build_valid_slot("'", "==01#00F"));
    slots.push(build_valid_slot("'", "==02#040"));
    slots.push(build_valid_slot("'", "==03#07F"));
    slots.push(build_valid_slot("_", "NO00#000"));

    // 4-step Autoassociative Hopfield/HDC retrieval & sharp wave ripple broadcast
    for step in 0..4 {
        slots.push(build_valid_slot("_", &format!("OP04M102"))); // Optical crossbar projection
        slots.push(build_valid_slot("_", &format!("XO05M400"))); // XOR Hamming threshold clean
        slots.push(build_valid_slot("_", &format!("CO06M503"))); // Compare with sharp-wave threshold
        slots.push(build_valid_slot("~", &format!("TX07M6{:02}", step))); // Torus broadcast to Neocortex
    }

    slots.push(build_valid_slot("!", "HL00#000"));
    pack_slots_into_vliw_bundles("hippocampal_episodic_replay", &slots, &cfg).unwrap()
}

/// Generates Thalamic Attention Crossbar Router Kernel
pub fn generate_thalamic_router_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: Sensory stream buffer, R02: Attentional salience mask, R03: Cortical core target
    slots.push(build_valid_slot("'", "==01#0FF"));
    slots.push(build_valid_slot("'", "==02#0C0"));
    slots.push(build_valid_slot("'", "==03#012")); // Cortical target core (1, 2)
    slots.push(build_valid_slot("_", "NO00#000"));

    // Salience gate filtering & 4D-Torus priority packet inject
    slots.push(build_valid_slot("_", "MA04M102")); // Bitmask filter salient tokens
    slots.push(build_valid_slot("_", "WH05M403")); // High-priority wormhole route
    slots.push(build_valid_slot("_", "CP06M500")); // Checkpoint routed payload
    slots.push(build_valid_slot("~", "BB07M600")); // Brain-Bridge synchronizer tap
    slots.push(build_valid_slot("!", "HL00#000"));

    pack_slots_into_vliw_bundles("thalamic_crossbar_router", &slots, &cfg).unwrap()
}

/// Generates Paged Flash-Decoding Split-KV Kernel
pub fn generate_flash_decoding_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: Query vector, R02: Page table entry address, R03: Split-KV reduction accumulator
    slots.push(build_valid_slot("'", "==01#040"));
    slots.push(build_valid_slot("'", "==02#080"));
    slots.push(build_valid_slot("'", "==03#000"));
    slots.push(build_valid_slot("_", "NO00#000"));

    // Split-KV parallel block execution across Torus dim-W
    for blk in 0..4 {
        slots.push(build_valid_slot("_", &format!("LD04M20{:X}", blk))); // PGAS bank load paged KV block
        slots.push(build_valid_slot("_", "MD05M104")); // Q * K_block dot product
        slots.push(build_valid_slot("_", "SM06M500")); // Block online Softmax
        slots.push(build_valid_slot("_", "AD03M306")); // Accumulate into reduction pool
    }

    slots.push(build_valid_slot("~", "RM07M300")); // Final RMSNorm reduction
    slots.push(build_valid_slot("!", "HL00#000"));

    pack_slots_into_vliw_bundles("flash_decoding_paged", &slots, &cfg).unwrap()
}

/// Generates BitNet b1.58 Packed Trit Vector Dequantizer Kernel
pub fn generate_bitnet_dequant_kernel() -> String {
    let mut slots = Vec::new();
    let cfg = VectorizerConfig::default();

    // R01: 32-bit Packed Trit Word (16 trits, 2 bits each), R02: Quant scale factor (FP16/fixed)
    slots.push(build_valid_slot("'", "==01#0AA"));
    slots.push(build_valid_slot("'", "==02#018"));
    slots.push(build_valid_slot("_", "NO00#000"));
    slots.push(build_valid_slot("_", "NO00#000"));

    // Unpack 4 parallel trit lanes and apply scale
    for lane in 0..4 {
        slots.push(build_valid_slot("_", &format!("RO03M10{:X}", lane))); // Rotate bits
        slots.push(build_valid_slot("_", &format!("MA04M303"))); // Mask 2-bit trit
        slots.push(build_valid_slot("_", &format!("FX05M400"))); // Trit convert {-1, 0, +1}
        slots.push(build_valid_slot("_", &format!("ML0{:X}M502", lane + 6))); // Scaled FP16 dequant
    }

    slots.push(build_valid_slot("!", "HL00#000"));
    pack_slots_into_vliw_bundles("quant_trit_dequant", &slots, &cfg).unwrap()
}

#[test]
fn test_new_ai_agi_microcode_kernels_integrity_and_jit() {
    let kernels = vec![
        ("grouped_query_attention", generate_gqa8_kernel()),
        ("fused_swiglu_expert", generate_fused_swiglu_moe_kernel()),
        ("striatal_actor_critic", generate_striatal_actor_critic_kernel()),
        ("hippocampal_episodic_replay", generate_hippocampal_replay_kernel()),
        ("thalamic_crossbar_router", generate_thalamic_router_kernel()),
        ("flash_decoding_paged", generate_flash_decoding_kernel()),
        ("quant_trit_dequant", generate_bitnet_dequant_kernel()),
    ];

    for (name, code) in &kernels {
        // 1. Static linter: 0 errors, 0 warnings
        let lint = lint_cl_source(code, name);
        assert_eq!(lint.error_count, 0, "Kernel '{}' has lint errors: {:?}", name, lint.issues);
        assert_eq!(lint.warning_count, 0, "Kernel '{}' has hazard warnings: {:?}", name, lint.issues);

        // 2. Structural verification
        let rep = verify_cl_program(code).expect(&format!("Structural verify failed for '{}'", name));
        assert!(rep.total_bundles > 0);
        assert!(rep.hazards.is_empty(), "Kernel '{}' has hazards: {:?}", name, rep.hazards);

        // 3. In-memory Silicon JIT execution
        let core = run_cl_jit(code).expect(&format!("JIT execution failed for '{}'", name));
        assert!(core.cycle_count > 0);
        assert!(core.is_halted, "Kernel '{}' did not halt cleanly", name);

        // 4. Save to libcl directory if exists
        let libcl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libcl");
        if libcl_dir.exists() {
            let fpath = libcl_dir.join(format!("{}.cl", name));
            let _ = std::fs::write(&fpath, code);
        }
    }
}
