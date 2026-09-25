use cronc::cl_kernel::{list_available_kernels, synthesize_kernel};
use cron_vm::Simulator;

#[test]
fn test_kernel_catalog_contains_advanced_kernels() {
    let catalog = list_available_kernels();
    let names: Vec<&str> = catalog.iter().map(|k| k.name).collect();
    assert!(names.contains(&"flash-attn-3"), "Must contain FlashAttention-3");
    assert!(names.contains(&"moe-router"), "Must contain MoE dynamic router");
    assert!(names.contains(&"mla-latent-attn"), "Must contain Multi-Head Latent Attention");
    assert!(names.contains(&"bitnet-swiglu-expert"), "Must contain BitNet SwiGLU expert");
}

#[test]
fn test_flash_attention_3_synthesis_and_simulation() {
    let cl_code = synthesize_kernel("flash-attn-3", 64, 128).expect("FlashAttention-3 synthesis");
    assert!(cl_code.contains(".stage \"flash_attention_3_tile\""));
    assert!(cl_code.contains("@flash3_photonic_gemm:"));
    assert!(cl_code.contains("@flash3_online_logsumexp:"));

    let mut sim = Simulator::new();
    sim.load_machine_code(&cl_code);
    let stats = sim.run();

    assert!(stats.total_cycles >= 7, "Expected at least 7 execution cycles");
    assert!(stats.optical_gemm_ops > 0, "FlashAttention-3 must execute optical MZI GEMM ops");
    assert!(stats.reversible_gate_ops > 0, "FlashAttention-3 must execute reversible gate ops");
}

#[test]
fn test_moe_router_synthesis_and_simulation() {
    let cl_code = synthesize_kernel("moe-router", 64, 8).expect("MoE Router synthesis");
    assert!(cl_code.contains(".stage \"moe_router_top2\""));
    assert!(cl_code.contains("@moe_wormhole_dispatch:"));

    let mut sim = Simulator::new();
    sim.load_machine_code(&cl_code);
    let stats = sim.run();

    assert!(stats.total_cycles >= 7, "Expected at least 7 execution cycles");
    assert!(stats.mesh_packets_routed > 0, "MoE router must route wormhole NoC packets");
}

#[test]
fn test_mla_latent_attention_synthesis_and_simulation() {
    let cl_code = synthesize_kernel("mla-latent-attn", 64, 128).expect("MLA synthesis");
    assert!(cl_code.contains(".stage \"mla_latent_attention\""));
    assert!(cl_code.contains("@mla_decoupled_rope_keys:"));

    let mut sim = Simulator::new();
    sim.load_machine_code(&cl_code);
    let stats = sim.run();

    assert!(stats.total_cycles >= 6, "Expected at least 6 execution cycles");
    assert!(stats.optical_gemm_ops > 0, "MLA must execute optical MZI matrix multiply");
}

#[test]
fn test_bitnet_swiglu_expert_synthesis_and_simulation() {
    let cl_code = synthesize_kernel("bitnet-swiglu-expert", 64, 128).expect("BitNet SwiGLU synthesis");
    assert!(cl_code.contains(".stage \"bitnet_swiglu_expert\""));
    assert!(cl_code.contains("@expert_fused_silu_gating:"));

    let mut sim = Simulator::new();
    sim.load_machine_code(&cl_code);
    let stats = sim.run();

    assert!(stats.total_cycles >= 6, "Expected at least 6 execution cycles");
}
