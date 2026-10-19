// ============================================================================
// CRON & CL TEST SUITE: Frontier Models & Formal Self-Healing Contract
// Tests compilation, 4-Snake lowering, and self-healing diagnostics
// ============================================================================

use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_v2_lowering::V2LoweringEngine;
use cronc::cl_high_heal::verify_and_heal_high_cl;
use cronc::cl_jit::{execute_cl_on_core, ClJitCore};
use std::path::Path;

fn read_example(filename: &str) -> String {
    let p1 = Path::new("examples").join("high_level").join(filename);
    if p1.exists() {
        return std::fs::read_to_string(&p1).unwrap_or_else(|_| panic!("Failed to read {:?}", p1));
    }
    let p2 = Path::new("../..").join("examples").join("high_level").join(filename);
    if p2.exists() {
        return std::fs::read_to_string(&p2).unwrap_or_else(|_| panic!("Failed to read {:?}", p2));
    }
    panic!("Could not find example file {} in {:?} or {:?}", filename, p1, p2);
}

#[test]
fn test_deepseek_v4_frontier_moe_compilation_and_lowering() {
    let source = read_example("deepseek_v4_frontier_moe.cl");

    let doc = parse_cl_v2(&source).expect("Should parse DeepSeek-V4");
    assert_eq!(doc.target_contract.as_ref().unwrap().kernel_name, "deepseek_v4_frontier_moe");

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Should lower DeepSeek-V4 to VLIW");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let cl_vliw = engine.emit_canonical(&doc_with_vliw);
    assert!(cl_vliw.contains("B0000:"));

    let mut core = ClJitCore::new();
    execute_cl_on_core(&cl_vliw, &mut core).expect("Should execute DeepSeek-V4 in JIT");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_sora_video_dit_flow_compilation_and_lowering() {
    let source = read_example("sora_video_dit_flow.cl");

    let doc = parse_cl_v2(&source).expect("Should parse Sora DiT");
    assert_eq!(doc.target_contract.as_ref().unwrap().kernel_name, "sora_video_dit_flow");

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Should lower Sora DiT to VLIW");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let cl_vliw = engine.emit_canonical(&doc_with_vliw);
    assert!(cl_vliw.contains("B0000:"));

    let mut core = ClJitCore::new();
    execute_cl_on_core(&cl_vliw, &mut core).expect("Should execute Sora DiT in JIT");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_nequip_allegro_equivariant_sciml_compilation_and_lowering() {
    let source = read_example("nequip_allegro_equivariant_sciml.cl");

    let doc = parse_cl_v2(&source).expect("Should parse NequIP Allegro");
    assert_eq!(doc.target_contract.as_ref().unwrap().kernel_name, "nequip_allegro_equivariant_sciml");

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Should lower NequIP to VLIW");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let cl_vliw = engine.emit_canonical(&doc_with_vliw);
    assert!(cl_vliw.contains("B0000:"));

    let mut core = ClJitCore::new();
    execute_cl_on_core(&cl_vliw, &mut core).expect("Should execute NequIP in JIT");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_pi0_vla_physical_intelligence_compilation_and_lowering() {
    let source = read_example("pi0_vla_physical_intelligence.cl");

    let doc = parse_cl_v2(&source).expect("Should parse Pi0 VLA");
    assert_eq!(doc.target_contract.as_ref().unwrap().kernel_name, "pi0_vla_physical_intelligence");

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Should lower Pi0 VLA to VLIW");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let cl_vliw = engine.emit_canonical(&doc_with_vliw);
    assert!(cl_vliw.contains("B0000:"));

    let mut core = ClJitCore::new();
    execute_cl_on_core(&cl_vliw, &mut core).expect("Should execute Pi0 VLA in JIT");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_formal_self_healing_contract_verified() {
    let source = r#"
        module valid_contract;
        arch SimpleArch {
            fn test_op(x: Tensor[FP32, 16], w: Tensor[FP32, 16]) -> Tensor[FP32, 16] {
                let coupled = x <~> w;
                let normed = .op.rmsnorm(coupled);
                return normed;
            }
        }
    "#;

    let report = verify_and_heal_high_cl(source, Some("test.cl"));
    assert_eq!(report.status, "FORMAL_VERIFIED");
    assert!(report.vliw_bundle_count > 0);

    let json = report.to_json();
    assert!(json.contains("\"diagnostic_status\": \"FORMAL_VERIFIED\""));
}

#[test]
fn test_formal_self_healing_contract_rejected_and_patched() {
    // Deliberate syntax contract violation: unclosed braces and invalid tokens
    let faulty_source = "arch BrokenArch { let x = ; ; ;";

    let report = verify_and_heal_high_cl(faulty_source, Some("faulty.cl"));
    assert_eq!(report.status, "FORMAL_REJECTED");
    assert_eq!(report.error_domain, Some("SYNTAX_GRAMMAR_CONTRACT_VIOLATION".to_string()));
    assert!(report.self_heal_patch.is_some());

    let json = report.to_json();
    assert!(json.contains("\"diagnostic_status\": \"FORMAL_REJECTED\""));
    assert!(json.contains("\"self_heal_patch\""));
}
