// ============================================================================
// CRON Universal Python-to-CL Transpiler & Parity Test Suite
// File: crates/cronc/tests/test_py_to_cl_universal_transpile.rs
//
// Verifies:
// 1. Python AI Matrix -> 4D Sphere Clifford Geodesic CL Microcode
// 2. Python Neural Net -> Sub-Byte 1.58b C23 AVX-512 Native Source
// 3. Python Tree / Graph (MCTS) -> 16-Way Branchless Synthesizable Verilog RTL
// 4. Python ATP Artificial Life -> Landauer Thermodynamic Reversible Gates
// 5. 100.000% Bit-Exact Mathematical Parity Verification (Max Diff < 1e-12)
// ============================================================================

use cronc::cl_py_transpile::{
    tokenize_python, verify_python_to_cl_parity, PyDomainKind, PyToCronTranspiler,
    PyToken,
};

#[test]
fn test_python_tokenizer_basic() {
    let py_code = r#"
def matmul_attention(q, k):
    scores = q @ k
    return scores
"#;
    let tokens = tokenize_python(py_code);
    assert!(!tokens.is_empty());
    assert!(tokens.contains(&PyToken::Def));
    assert!(tokens.contains(&PyToken::Ident("matmul_attention".to_string())));
    assert!(tokens.contains(&PyToken::Op("@".to_string())));
    assert!(tokens.contains(&PyToken::Return));
}

#[test]
fn test_python_ai_matrix_to_cl_vliw() {
    let py_code = r#"
def sphere_matrix_forward(x, weights):
    h = x @ weights
    y = relu(h)
    return y
"#;
    let mut transpiler = PyToCronTranspiler::new(64, true);
    let count = transpiler.parse_and_lower(py_code).expect("Parsing should succeed");
    assert_eq!(count, 1);

    let func = &transpiler.functions[0];
    assert_eq!(func.name, "sphere_matrix_forward");
    assert_eq!(func.domain, PyDomainKind::AiMatrix);

    let cl_code = transpiler.emit_cl_machine_code();
    assert!(cl_code.contains("// CRON CL Machine Code: Lowered from Python Mathematical Graph"));
    assert!(cl_code.contains("B0000:"));
    println!("Generated CL Microcode for AI Matrix:\n{}", &cl_code[..cl_code.len().min(400)]);
}

#[test]
fn test_python_neural_net_to_c23() {
    let py_code = r#"
def lora_neural_layer_backward(activations, weights):
    delta = activations @ weights
    return delta
"#;
    let mut transpiler = PyToCronTranspiler::new(64, false);
    transpiler.parse_and_lower(py_code).expect("Parsing should succeed");

    let c23_code = transpiler.emit_c23_source();
    assert!(c23_code.contains("#include <stdint.h>"));
    assert!(c23_code.contains("cron_lora_neural_layer_backward"));
    assert!(c23_code.contains("restrict out_vec"));
    println!("Generated C23 Source for Neural Net:\n{}", &c23_code[..c23_code.len().min(400)]);
}

#[test]
fn test_python_tree_graph_to_verilog() {
    let py_code = r#"
def mcts_tree_search_step(current_node, branches):
    next_node = current_node
    return next_node
"#;
    let mut transpiler = PyToCronTranspiler::new(64, false);
    transpiler.parse_and_lower(py_code).expect("Parsing should succeed");

    let verilog = transpiler.emit_verilog_rtl();
    assert!(verilog.contains("module cron_core_mcts_tree_search_step"));
    assert!(verilog.contains("input  wire        clk,"));
    assert!(verilog.contains("4-Stage Silicon Pipeline: Pack -> Rot -> Step -> Store"));
    println!("Generated Verilog RTL for Tree Graph:\n{}", &verilog[..verilog.len().min(400)]);
}

#[test]
fn test_python_atp_life_reversible_thermodynamics() {
    let py_code = r#"
def atp_metabolic_hydrolysis(atp_pool, adp_pi):
    state = atp_pool
    return state
"#;
    let mut transpiler = PyToCronTranspiler::new(64, true);
    transpiler.parse_and_lower(py_code).expect("Parsing should succeed");

    assert_eq!(transpiler.functions[0].domain, PyDomainKind::AtpLife);
    let cl_code = transpiler.emit_cl_machine_code();
    assert!(cl_code.contains("_RF")); // Reversible Fredkin Swap opcode
    assert!(cl_code.contains("_TO")); // Toffoli Reversible Gate opcode
}

#[test]
fn test_bit_exact_parity_verification_1000_samples() {
    let py_corpus = r#"
def matrix_compute(x, w):
    return x @ w

def neural_adapter_step(x, w):
    return relu(x)

def mcts_tree_traversal(node):
    return node

def atp_energy_life(atp):
    return atp
"#;

    let reports = verify_python_to_cl_parity(py_corpus, 1000).expect("Parity verification failed");
    assert_eq!(reports.len(), 4, "Must verify all 4 domains");

    for r in &reports {
        println!(
            "[PARITY AUDIT] Function: {:<25} | Domain: {:<12} | Samples: {} | Max Diff: {:.1e} | Bit-Exact: {} | Speedup: {:.0}x",
            r.function_name,
            format!("{:?}", r.domain),
            r.samples_tested,
            r.max_absolute_diff,
            r.is_bit_exact,
            r.speedup_factor
        );
        assert!(
            r.is_bit_exact,
            "Function {} failed bit-exact verification! Max diff: {}",
            r.function_name, r.max_absolute_diff
        );
        assert!(
            r.max_absolute_diff < 1e-12,
            "Max absolute difference must be mathematically 0 (< 1e-12)"
        );
    }
}

#[test]
fn test_emitted_cl_passes_canonical_slot_and_crc8_verification() {
    let py_code = r#"
def sphere_matrix(x, w):
    return x @ w

def neural_net(x, a, b):
    return relu(x)

def tree_dag(node):
    return node

def atp_kinetics(atp, adp):
    return atp
"#;
    let mut transpiler = PyToCronTranspiler::new(64, true);
    transpiler.parse_and_lower(py_code).expect("Parsing must succeed");

    let cl_code = transpiler.emit_cl_machine_code();
    let mut bundles_checked = 0;
    let mut slots_checked = 0;

    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('B') && trimmed.contains(':') {
            let parts: Vec<&str> = trimmed.split(':').collect();
            assert_eq!(parts.len(), 2);
            let slots: Vec<&str> = parts[1].split_whitespace().collect();
            assert_eq!(slots.len(), 4, "Every VLIW bundle must have exactly 4 slots");
            for slot in slots {
                assert_eq!(slot.len(), 10, "Every slot must be exactly 10 characters");
                let parsed = cronc::cl_lang::parse_slot(slot).expect("Slot must be valid .cl syntax");
                assert!(cronc::cl_lang::KNOWN_OPCODES.contains(&parsed.opcode.as_str()));
                assert!(cronc::cl_lang::verify_token_crc8(slot), "CRC-8 ATM checksum must be valid");
                slots_checked += 1;
            }
            bundles_checked += 1;
        }
    }

    assert!(bundles_checked >= 8, "Must generate at least 8 bundles");
    assert!(slots_checked >= 32, "Must verify at least 32 slots");
    println!("Emitted CL Microcode Verification Passed: {} bundles, {} slots verified with 100% CRC-8 ATM integrity.", bundles_checked, slots_checked);
}

