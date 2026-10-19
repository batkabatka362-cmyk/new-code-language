// ============================================================================
// CRON & CL TEST SUITE: Frontier Models & Formal Self-Healing Contract
// Tests compilation, 4-Snake lowering, and self-healing diagnostics
// ============================================================================

use cronc::cl_high_parser::parse_high_cl;
use cronc::cl_high_lowering::compile_high_cl_to_cl;
use cronc::cl_high_heal::verify_and_heal_high_cl;
use cronc::cl_jit::run_cl_jit;
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

    let module = parse_high_cl(&source).expect("Should parse DeepSeek-V4");
    assert_eq!(module.name, "deepseek_v4_frontier_moe");

    let cl_vliw = compile_high_cl_to_cl(&source).expect("Should lower DeepSeek-V4 to VLIW");
    assert!(cl_vliw.contains("B0000:"));

    let core = run_cl_jit(&cl_vliw).expect("Should execute DeepSeek-V4 in JIT");
    assert!(core.cycle_count > 0);
}

#[test]
fn test_sora_video_dit_flow_compilation_and_lowering() {
    let source = read_example("sora_video_dit_flow.cl");

    let module = parse_high_cl(&source).expect("Should parse Sora DiT");
    assert_eq!(module.name, "sora_video_dit_flow");

    let cl_vliw = compile_high_cl_to_cl(&source).expect("Should lower Sora DiT to VLIW");
    assert!(cl_vliw.contains("B0000:"));

    let core = run_cl_jit(&cl_vliw).expect("Should execute Sora DiT in JIT");
    assert!(core.cycle_count > 0);
}

#[test]
fn test_nequip_allegro_equivariant_sciml_compilation_and_lowering() {
    let source = read_example("nequip_allegro_equivariant_sciml.cl");

    let module = parse_high_cl(&source).expect("Should parse NequIP Allegro");
    assert_eq!(module.name, "nequip_allegro_equivariant_sciml");

    let cl_vliw = compile_high_cl_to_cl(&source).expect("Should lower NequIP to VLIW");
    assert!(cl_vliw.contains("B0000:"));

    let core = run_cl_jit(&cl_vliw).expect("Should execute NequIP in JIT");
    assert!(core.cycle_count > 0);
}

#[test]
fn test_pi0_vla_physical_intelligence_compilation_and_lowering() {
    let source = read_example("pi0_vla_physical_intelligence.cl");

    let module = parse_high_cl(&source).expect("Should parse Pi0 VLA");
    assert_eq!(module.name, "pi0_vla_physical_intelligence");

    let cl_vliw = compile_high_cl_to_cl(&source).expect("Should lower Pi0 VLA to VLIW");
    assert!(cl_vliw.contains("B0000:"));

    let core = run_cl_jit(&cl_vliw).expect("Should execute Pi0 VLA in JIT");
    assert!(core.cycle_count > 0);
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
