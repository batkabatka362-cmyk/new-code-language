// ============================================================================
// Test Suite: CRON AI Kernel Dialect & Sub-Byte Tensor Primitives
// Tests:
//   1. Sub-byte vector primitives (trit16, int4x8, fp16x2) pack/unpack integrity
//   2. High-level .kernel directive AST parsing with tensor type signatures
//   3. Lowering high-level AI operators (RMSNorm, FlashAttention, SwiGLU, BitNet, Rotor4D)
//      into 16-character 3-address VLIW bundles
//   4. End-to-end program lowering (lower_cl_with_kernels) and CRC-8 ATM verification
// ============================================================================

use cronc::cl_lang::{
    pack_trit16, unpack_trit16, pack_int4x8, unpack_int4x8, pack_fp16x2, unpack_fp16x2,
    parse_kernel_header, lower_kernel_to_vliw16, lower_cl_with_kernels,
    verify_cl_program, verify_token_crc8, ClDirective,
};

#[test]
fn test_subbyte_trit16_pack_unpack_roundtrip() {
    let original: [i8; 16] = [1, -1, 0, 1, 1, 0, -1, -1, 0, 1, -1, 0, 1, 1, -1, 0];
    let packed = pack_trit16(&original);
    let unpacked = unpack_trit16(packed);
    assert_eq!(original, unpacked, "trit16 roundtrip packing must be lossless");

    // All +1
    let all_ones = [1i8; 16];
    assert_eq!(pack_trit16(&all_ones), 0x5555_5555);

    // All -1
    let all_negs = [-1i8; 16];
    assert_eq!(pack_trit16(&all_negs), 0xAAAA_AAAA);
}

#[test]
fn test_subbyte_int4x8_pack_unpack_roundtrip() {
    let original: [i8; 8] = [7, -8, 3, -2, 0, 1, -5, 4];
    let packed = pack_int4x8(&original);
    let unpacked = unpack_int4x8(packed);
    assert_eq!(original, unpacked, "int4x8 nibble roundtrip packing must be lossless");
}

#[test]
fn test_subbyte_fp16x2_pack_unpack_roundtrip() {
    let a = 1.5f32;
    let b = -0.75f32;
    let packed = pack_fp16x2(a, b);
    let (unp_a, unp_b) = unpack_fp16x2(packed);
    assert!((unp_a - a).abs() < 1e-4, "FP16 slot 0 must preserve half-precision float");
    assert!((unp_b - b).abs() < 1e-4, "FP16 slot 1 must preserve half-precision float");
}

#[test]
fn test_kernel_directive_header_parsing() {
    let line = ".kernel flash_attention_block(%Q: trit16[32, 128], %K: trit16[8, 128], %V: trit16[8, 128]) -> (%O: fp16x2[32, 128]) {";
    let kernel = parse_kernel_header(line).expect("Kernel header should parse cleanly");

    assert_eq!(kernel.name, "flash_attention_block");
    assert_eq!(kernel.inputs.len(), 3);
    assert_eq!(kernel.inputs[0].name, "%Q");
    assert_eq!(kernel.inputs[0].dtype, "trit16");
    assert_eq!(kernel.inputs[0].dims, vec![32, 128]);

    assert_eq!(kernel.inputs[1].name, "%K");
    assert_eq!(kernel.inputs[1].dims, vec![8, 128]);

    assert_eq!(kernel.outputs.len(), 1);
    assert_eq!(kernel.outputs[0].name, "%O");
    assert_eq!(kernel.outputs[0].dtype, "fp16x2");
    assert_eq!(kernel.outputs[0].dims, vec![32, 128]);
}

#[test]
fn test_kernel_lowering_to_16char_vliw_bundles() {
    let mut kernel = parse_kernel_header(".kernel bitnet_transformer_core() {").unwrap();
    kernel.body_statements.push("%q_norm = rms_norm(%Q)".to_string());
    kernel.body_statements.push("%attn = flash_attn(%q_norm, %K, %V)".to_string());
    kernel.body_statements.push("%ffn = swiglu(%attn)".to_string());
    kernel.body_statements.push("%w = bitnet_linear(%ffn, %W)".to_string());
    kernel.body_statements.push("%rot = rotor4d_rotate(%w, %R)".to_string());
    kernel.body_statements.push("emit %rot -> Core[3, 0, 0, 0]".to_string());

    let bundles = lower_kernel_to_vliw16(&kernel, 0);
    assert_eq!(bundles.len(), 18, "Expected 18 lowered hazard-free VLIW bundles");

    for bundle in &bundles {
        assert!(bundle.starts_with('B'), "Bundle must start with B<cycle>: {}", bundle);
        let (_, slots_part) = bundle.split_once(':').unwrap();
        let slots: Vec<&str> = slots_part.split_whitespace().collect();
        assert_eq!(slots.len(), 4, "Every bundle must have exactly 4 slots");

        for slot in slots {
            assert_eq!(slot.len(), 16, "Lowered slot must be exactly 16 characters: {}", slot);
            assert!(verify_token_crc8(slot), "Lowered slot must satisfy CRC-8 ATM parity: {}", slot);
        }
    }
}

#[test]
fn test_end_to_end_cl_with_kernel_blocks() {
    let source = r#"
.stage "cron_bitnet_agent", params="7.544B", precision="1.58b_ternary"
.tensor %Q: [32, 128], %K: [8, 128], %V: [8, 128]
.layout {TP=4, EP=4, CP=4, PP=4, dim="4x4x4x4"}

.kernel neural_attention_step(%Q: trit16[32, 128], %K: trit16[8, 128]) -> (%O: fp16x2[32, 128]) {
    %q_norm = rms_norm(%Q)
    %attn = flash_attn(%q_norm, %K)
    %out = swiglu(%attn)
    emit %out -> Core[1, 0, 0, 0]
}
"#;

    // Verify parser recognizes the .kernel directive in high-level mode
    let report = verify_cl_program(source).expect("Source with .kernel should verify");
    assert_eq!(report.directives_found, 4);
    assert!(report.directives_parsed.iter().any(|d| matches!(d, ClDirective::Kernel(_))));

    // Lower to pure 16-character machine code
    let lowered = lower_cl_with_kernels(source).expect("Lowering must succeed");
    assert!(lowered.contains("@neural_attention_step:"));
    assert!(lowered.contains("B0000:"));
    assert!(lowered.contains("_RM04@01"));
    assert!(lowered.contains("_QK01$01"));
    assert!(lowered.contains("_TX05$05"));

    // Verify lowered code compiles as 100% valid machine code
    let machine_report = verify_cl_program(&lowered).expect("Lowered machine code must verify");
    assert!(machine_report.total_bundles > 0);
    assert!(machine_report.strict_crc_verified > 0);
    if !machine_report.hazards.is_empty() {
        eprintln!("HAZARDS FOUND: {:#?}", machine_report.hazards);
        eprintln!("LOWERED CODE:\n{}", lowered);
    }
    assert!(machine_report.hazards.is_empty(), "Lowered bundles must be hazard-free");
}

#[test]
fn test_kernel_jit_execution() {
    let source = r#"
.stage "demo_jit_stage", params="1M", precision="1.58b_ternary"

.kernel demo_kernel(%Q: trit16[16, 16]) -> (%O: fp16x2[16, 16]) {
    %q_norm = rms_norm(%Q)
    %attn = flash_attn(%q_norm)
    emit %attn -> Core[1, 0, 0, 0]
}
"#;
    let jit_res = cronc::cl_jit::run_cl_jit(source).expect("JIT execution of .kernel code must succeed");
    assert!(jit_res.cycle_count > 0, "JIT must execute cycles for lowered kernel bundles");
    assert!(jit_res.spatial_broadcast_count > 0, "JIT must execute emit broadcast operation");
}

#[test]
fn test_kernel_c23_transpilation() {
    let source = r#"
.stage "demo_c23_stage", params="1M", precision="1.58b_ternary"

.kernel demo_c23_kernel(%Q: trit16[16, 16]) {
    %q_norm = rms_norm(%Q)
    emit %q_norm -> Core[2, 0, 0, 0]
}
"#;
    let c23_code = cronc::cl_c23::compile_cl_to_c23(source, "demo_c23").expect("C23 compilation must succeed");
    assert!(c23_code.contains("cron_execute_bundles"), "Generated C23 must contain execution loop");
    assert!(c23_code.contains("core->cycle_count"), "Generated C23 must track hardware cycles");
}

#[test]
fn test_kernel_demo_file_simulation() {
    let demo_path = if std::path::Path::new("examples/cl/ai_kernel_demo.cl").exists() {
        std::path::PathBuf::from("examples/cl/ai_kernel_demo.cl")
    } else {
        std::path::PathBuf::from("../../examples/cl/ai_kernel_demo.cl")
    };
    assert!(demo_path.exists(), "ai_kernel_demo.cl must exist at {:?}", demo_path);
    let content = std::fs::read_to_string(&demo_path).expect("Read ai_kernel_demo.cl");
    let lowered = cronc::cl_lang::lower_cl_with_kernels(&content).expect("Lowering must succeed");
    let stats = cron_vm::run_cl(&lowered);
    assert_eq!(stats.total_cycles, 11, "ai_kernel_demo.cl must execute in exactly 11 cycles");
    assert_eq!(stats.mesh_packets_routed, 2048, "ai_kernel_demo.cl must route 2048 packets across 4D Torus");
}

#[test]
fn test_safetensors_weights_binding_and_sovereign_1b_pipeline() {
    use cronc::model_importer::safetensors::{create_synthetic_safetensors, parse_safetensors, safetensors_to_cl_weights_binding, SafeTensorDType};

    let embed_vals: Vec<f32> = (0..64).map(|i| if i % 2 == 0 { 0.5 } else { -0.5 }).collect();
    let norm_vals: Vec<f32> = vec![1.0; 16];
    let q_vals: Vec<f32> = (0..64).map(|i| if i % 3 == 0 { 0.8 } else if i % 3 == 1 { -0.8 } else { 0.0 }).collect();
    let gate_vals: Vec<f32> = (0..64).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();

    let tensors = [
        ("model.embed_tokens.weight", SafeTensorDType::F32, &[32, 2][..], &embed_vals[..]),
        ("model.layers.0.input_layernorm.weight", SafeTensorDType::F32, &[16][..], &norm_vals[..]),
        ("model.layers.0.self_attn.q_proj.weight", SafeTensorDType::F32, &[8, 8][..], &q_vals[..]),
        ("model.layers.0.mlp.gate_proj.weight", SafeTensorDType::F32, &[8, 8][..], &gate_vals[..]),
    ];

    let bytes = create_synthetic_safetensors(&tensors);
    assert!(!bytes.is_empty(), "SafeTensors binary must not be empty");

    let model = parse_safetensors(&bytes).expect("Must parse synthetic SafeTensors");
    assert_eq!(model.tensors.len(), 4, "Must have exactly 4 ingested tensors");

    let cl_weights = safetensors_to_cl_weights_binding(&model).expect("Must convert to .weights directives");
    assert!(cl_weights.contains(".weights bank=0"), "Must have Bank 0 binding");
    assert!(cl_weights.contains(".weights bank=1"), "Must have Bank 1 binding");
    assert!(cl_weights.contains(".weights bank=2"), "Must have Bank 2 binding");
    assert!(cl_weights.contains(".weights bank=3"), "Must have Bank 3 binding");

    // Verify main sovereign_1b file loads and runs cleanly
    let main_path = if std::path::Path::new("models/sovereign_1b/main.cl").exists() {
        std::path::PathBuf::from("models/sovereign_1b/main.cl")
    } else {
        std::path::PathBuf::from("../../models/sovereign_1b/main.cl")
    };
    assert!(main_path.exists(), "sovereign_1b/main.cl must exist");
    let content = std::fs::read_to_string(&main_path).expect("Read main.cl");
    let lowered = cronc::cl_lang::lower_cl_with_kernels(&content).expect("Lowering must succeed");
    let report = cronc::cl_lang::verify_cl_program(&lowered).expect("Lowered main.cl must verify");
    assert_eq!(report.total_bundles, 63, "main.cl must have 63 bundles");
    assert!(report.hazards.is_empty(), "Lowered bundles must be hazard-free");

    // Execute in 4D-Torus VM
    let stats = cron_vm::run_cl(&lowered);
    assert_eq!(stats.total_cycles, 5, "main.cl executes across 6 cores in 5 cycles");
    assert_eq!(stats.mesh_packets_routed, 2048, "main.cl routes 2048 packets across 4D Torus NoC");
}

#[test]
fn test_dynamic_symbolic_register_allocation_and_adapter_pipeline() {
    let source = r#"
.stage "symbolic_test", params="1B", precision="1.58b"
.kernel custom_dynamic_layer(%in_act: trit16[16], %in_weight: trit16[16]) -> (%out_act: trit16[16]) {
    %norm = rms_norm(%in_act)
    %attn = flash_attn(%norm, %in_weight)
    %mid = residual_add(%in_act, %attn)
    %mlp = swiglu(%mid)
    %out_act = residual_add(%mid, %mlp)
    emit %out_act -> Core[2, 0, 0, 0]
}
"#;

    let lowered = cronc::cl_lang::lower_cl_with_kernels(source).expect("Symbolic kernel lowering must succeed");
    assert!(lowered.contains("@custom_dynamic_layer:"));
    assert!(lowered.contains("B0000:"));

    // Verify machine code integrity and CRC-8 ATM
    let report = cronc::cl_lang::verify_cl_program(&lowered).expect("Lowered code must be valid .cl");
    assert!(report.total_bundles > 0);
    assert_eq!(report.hazards.len(), 0, "Lowered bundles must be hazard-free");

    // Execute on silicon JIT core
    let mut core = cronc::ClJitCore::new();
    core.r[1] = 10;
    core.r[2] = 20;
    for line in lowered.lines() {
        let tl = line.trim();
        if tl.starts_with('B') && tl.contains(':') {
            cronc::cl_jit::execute_cl_on_core(tl, &mut core).expect("JIT execution must succeed");
        }
    }
    assert!(core.cycle_count > 0, "Core must have advanced cycles");
}

#[test]
fn test_kernel_structured_control_flow_loop_and_branching() {
    let source = r#"
.stage "loop_test", params="1B", precision="1.58b"
.kernel iterative_transformer_layer(%x: trit16[16]) -> (%out: trit16[16]) {
    repeat 4 {
        %norm = rms_norm(%x)
        %x = residual_add(%x, %norm)
    }
    if %x {
        %out = swiglu(%x)
    } else {
        %out = %x
    }
    emit %out -> Core[1, 0, 0, 0]
}
"#;

    let lowered = cronc::cl_lang::lower_cl_with_kernels(source).expect("Structured loop kernel lowering must succeed");
    assert!(lowered.contains("@iterative_transformer_layer:"));
    assert!(lowered.contains("_BN"));
    assert!(lowered.contains("_BZ"));

    let report = cronc::cl_lang::verify_cl_program(&lowered).expect("Lowered code must be valid .cl");
    assert!(report.total_bundles > 0);
    assert_eq!(report.hazards.len(), 0, "Lowered bundles must be hazard-free");

    // Execute in JIT
    let mut core = cronc::ClJitCore::new();
    core.r[1] = 5;
    cronc::cl_jit::execute_cl_on_core(&lowered, &mut core).expect("JIT execution of loop must succeed");
    assert!(core.cycle_count >= 10, "Loop must execute multiple cycles");
}

#[test]
fn test_cl_module_system_and_namespace_export() {
    let source = r#"
.module sovereign.attention
.export { mla_project, mla_compress }

.kernel mla_project(%q: trit16[32, 128]) -> (%out: fp16x2[32, 128]) {
    %norm = rms_norm(%q)
    %out = %norm
}

.kernel mla_compress(%kv: trit16[32, 512]) -> (%c: trit16[32, 128]) {
    %c = %kv
}

.kernel internal_cache_swizzle(%x: trit16[16]) -> (%y: trit16[16]) {
    %y = %x
}
"#;

    let lowered = cronc::cl_lang::lower_cl_with_kernels(source).expect("Module lowering must succeed");
    assert!(lowered.contains("; [Directive: .module sovereign.attention]"));
    assert!(lowered.contains("; === Lowered AI Kernel (Public): mla_project ==="));
    assert!(lowered.contains("; === Lowered AI Kernel (Public): mla_compress ==="));
    assert!(lowered.contains("; === Lowered AI Kernel (Internal): internal_cache_swizzle ==="));
    assert!(lowered.contains("@mla_project:"));
    assert!(lowered.contains("@mla_compress:"));
    assert!(lowered.contains("@internal_cache_swizzle:"));

    let report = cronc::cl_lang::verify_cl_program(&lowered).expect("Module lowered code must pass verification");
    assert_eq!(report.hazards.len(), 0);
}

#[test]
fn test_cl_use_import_and_aliasing_and_cycle_prevention() {
    // Test importing an existing file (examples/cl/multi_step_reasoning.cl)
    let source = r#"
.module test.consumer
.use "examples/cl/multi_step_reasoning.cl" as reasoning
.use "examples/cl/multi_step_reasoning.cl" as reasoning_dup

.kernel consumer_entry(%in: trit16[16]) -> (%out: trit16[16]) {
    %out = %in
}
"#;

    let lowered = cronc::cl_lang::lower_cl_with_kernels(source).expect("Modular import must succeed");
    // Verify aliased label is emitted
    assert!(lowered.contains("@reasoning_multi_step_deduction_kernel:"));
    assert!(lowered.contains("@reasoning::multi_step_deduction_kernel:"));
    // Verify duplicate inclusion was prevented
    assert!(lowered.contains("; --- Already included module 'examples/cl/multi_step_reasoning.cl' (skipped duplicate) ---"));
    assert!(lowered.contains("@consumer_entry:"));

    let report = cronc::cl_lang::verify_cl_program(&lowered).expect("Lowered modular program must verify");
    assert_eq!(report.hazards.len(), 0);
}
