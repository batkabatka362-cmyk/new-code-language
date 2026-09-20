use cronc::{
    compile_cl_to_c23, compile_cl_to_llvm, heal_cl_program, optimize_cl_program,
    run_cl_jit, verify_cl_program, FormalVerifier,
};

const SAMPLE_ATTENTION_CL: &str = r#"
@attn_init:
B0000: '==01#004> '==02#008> '==03#00C> _NO00#000>
B0001: '==04#010> '==05#020> '==06#001> _NO00#000>

@q_k_projection:
B0002: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>
B0003: _FA03$142> _TT05$200> _PO06&100> _bb00#000>

@attention_weights_softmax:
B0004: _MD07.280> _PO08*300> _PO09+600> _NO00#000>
B0005: _PO01/400> _PO02/400> _PO03/400> _bb00#000>

@context_aggregation:
B0006: _OP0A$48F> _MD0B*2A2> _PO0C+100> _SB00#000>
B0007: _FU00#000> _PO00+600> _FE00#000> _HL00!000>
"#;

#[test]
fn test_cl_program_verification_with_labels() {
    let report = verify_cl_program(SAMPLE_ATTENTION_CL)
        .expect("Sample attention .cl must pass verification");
    assert_eq!(report.total_bundles, 8);
    assert_eq!(report.total_slots, 32);
    assert_eq!(report.labels_found, 4);
    assert!(report.hazards.is_empty(), "Hazards should be zero: {:?}", report.hazards);
}

#[test]
fn test_cl_in_memory_jit_execution() {
    let core = run_cl_jit(SAMPLE_ATTENTION_CL)
        .expect(".cl in-memory JIT execution must succeed");
    assert_eq!(core.cycle_count, 8);
    assert!(core.optical_gemm_count >= 2, "Optical GEMM ops should be at least 2");
    assert_eq!(core.barrier_count, 2, "Barrier count should match");
    assert!(core.is_halted, "Core should reach halted state");
}

#[test]
fn test_cl_llvm_ir_generation() {
    let llvm_ir = compile_cl_to_llvm(SAMPLE_ATTENTION_CL, "attention_kernel")
        .expect(".cl to LLVM IR compilation must succeed");
    assert!(llvm_ir.contains("; ModuleID = 'cron_cl_attention_kernel'"));
    assert!(llvm_ir.contains("define void @cron_execute_bundles"));
    assert!(llvm_ir.contains("define i32 @main"));
    assert!(llvm_ir.contains("call i32 (i8*, ...) @printf"));
}

#[test]
fn test_cl_ai_self_healing_and_crc_repair() {
    // LLM generated broken .cl with:
    // 1. Corrupted CRC token (slot 0 has '?')
    // 2. Missing slot padding (only 2 slots in B0001)
    // 3. Raw hazard (R1 written in slot 0, read in slot 1 in same cycle)
    let malformed_ai_cl = r#"
B0000: _OP01$2?F> '==02#004> _NO00#000> _NO00#000>
B0001: '==01#005> _PO02+100>
"#;

    let heal_report = heal_cl_program(malformed_ai_cl)
        .expect("Healer must repair malformed AI .cl code");
    assert!(heal_report.slots_repaired_crc > 0, "CRC should be repaired");
    assert!(heal_report.bundles_padded_nops > 0, "Bundle should be padded with NOPs");
    assert!(heal_report.raw_hazards_resolved > 0, "RAW hazard should be resolved via cycle splitting");

    // The resulting canonical code must pass standard verification cleanly!
    let verify_res = verify_cl_program(&heal_report.canonical_code);
    assert!(verify_res.is_ok(), "Healed canonical code must be valid: {:?}", verify_res);
}

#[test]
fn test_cl_vliw_slot_compaction_super_optimizer() {
    // Sparse code with lots of NOPs that can be compacted
    let sparse_cl = r#"
B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
B0001: '==02#00B> _NO00#000> _NO00#000> _NO00#000>
B0002: '==03#00C> _NO00#000> _NO00#000> _NO00#000>
B0003: '==04#00D> _NO00#000> _NO00#000> _NO00#000>
"#;

    let opt_report = optimize_cl_program(sparse_cl)
        .expect("Optimizer must compact sparse .cl code");
    assert_eq!(opt_report.original_bundles, 4);
    assert_eq!(opt_report.optimized_bundles, 1, "4 independent loads must be packed into a single 4-slot bundle!");
    assert_eq!(opt_report.compacted_slots, 4);
    assert!(opt_report.optimized_ipc > opt_report.original_ipc);
    assert!(opt_report.speedup_percentage >= 75.0);
}

#[test]
fn test_cl_direct_c23_translation() {
    let c23_code = compile_cl_to_c23(SAMPLE_ATTENTION_CL, "attention_c23")
        .expect(".cl to C23 compilation must succeed");
    assert!(c23_code.contains("CronSiliconCore"));
    assert!(c23_code.contains("cron_execute_bundles"));
    assert!(c23_code.contains("int main(int argc, char** argv)"));
}

#[test]
fn test_cl_direct_formal_verification() {
    let verifier = FormalVerifier::new(300.0, 1.0e9);
    let report = verifier.verify_cl(SAMPLE_ATTENTION_CL);
    assert!(report.is_provably_safe);
    assert!(report.dor_deadlock_free);
    assert!(report.total_operations_analyzed > 0);
    assert!(report.landauer_power_microwatts >= 0.0);
}

#[test]
fn test_cl_direct_verilog_synthesis() {
    let verilog = cronc::verilog_backend::generate_verilog_hdl(SAMPLE_ATTENTION_CL, "cl_attention_core")
        .expect(".cl to Verilog HDL synthesis must succeed");
    assert!(verilog.contains("module cl_attention_core"));
    assert!(verilog.contains("reg [31:0] inst_data;"));
    assert!(verilog.contains("always @(*) begin"));
}
