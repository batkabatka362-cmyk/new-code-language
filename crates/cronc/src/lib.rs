#![allow(
    clippy::result_large_err,
    clippy::type_complexity,
    clippy::ptr_arg,
    clippy::match_like_matches_macro,
    clippy::needless_range_loop,
    clippy::doc_lazy_continuation
)]

pub mod ast;
pub mod c_backend;
pub mod checker;
pub mod cl_lang;
pub mod codegen;
pub mod diagnostic;
pub mod fdo;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod scheduler;
pub mod token;
pub mod verilog_backend;
pub mod llvm_backend;
pub mod jit_backend;
pub mod autodiff;
pub mod cl_binary;
pub mod gpu_backend;
pub mod autotune;
pub mod cl_c23;
pub mod cl_llvm;
pub mod cl_jit;
pub mod cl_heal;
pub mod cl_opt;
pub mod model_importer;
pub mod comptime;
pub mod silicon_flash;
pub mod formal_verify;
pub mod cl_spec;
pub mod cl_link;
pub mod vibe_loop;
pub mod cl_cosim;
pub mod cl_memcheck;
pub mod cl_fuzz;
pub mod cl_bench;
pub mod cl_kernel;
pub mod cl_tile;
pub mod cl_cluster;
pub mod cl_sparse;
pub mod cl_power;
pub mod cl_autotune;
pub mod cl_stream;
pub mod cl_snn;
pub mod cl_infer;
pub mod cl_cordic;
pub mod cl_vcd;
pub mod cl_perf;
pub mod cl_balance;
pub mod cl_trace;
pub mod cl_router;
pub mod cl_esoteric;
pub mod cl_optic;
pub mod cl_patch;
pub mod cl_compare;
pub mod fusion;
pub mod constrained_sampler;
pub mod speculative_decoding;

pub use fusion::{
    emit_flash_attention_2_c, emit_fused_rmsnorm_linear_c, emit_fused_swiglu_c,
    emit_fused_transformer_block_c, verify_zero_heap_allocations, FusionGraph,
    FusionGroup, FusionNode, FusionOpKind,
};

pub use cl_compare::{
    comparison_to_json, evaluate_workload, generate_whitepaper_markdown,
    get_system_specs, render_ascii_comparison_scoreboard, run_comparison_suite,
    BaselineFilter, ComparisonSuiteReport, SystemSpec, SystemWorkloadResult,
    WorkloadComparison, WorkloadKind, BOLTZMANN_CONSTANT, LANDAUER_LIMIT_PER_BIT_JOULES,
    ROOM_TEMP_KELVIN,
};

pub use cl_patch::{
    apply_cl_patch, format_patch_ascii_hud, patch_package_to_json,
    synthesize_patch_controller_verilog, ClPatchPackage, ClPatchReport,
    MicrocodePatchEntry, PatchAction, MAX_PATCH_ENTRIES,
};

pub use cl_optic::{
    analyze_cl_optic, format_optic_ascii_hud, optic_report_to_json,
    synthesize_optical_power_verilog, ClOpticOptions, ClOpticReport,
    LossBreakdown, MeshTopology, WdmChannelInfo,
};


pub use cl_esoteric::{
    render_ascii_esoteric_hud, synthesize_verilog_esoteric_coprocessor,
    BefungeSystolicGrid, EsotericCoprocessor, HardwareStatusFlags,
    HardwareTapePointers, PrologUnificationEngine, SystolicDirection,
    Trit, TritWord, UnificationMatchResult, ZohlState,
};
pub use cl_router::{
    render_ascii_router_hud, synthesize_verilog_router, Flit, FlitType,
    NoCRouterConfig, NoCRouterSimulation, RouterPort, VirtualChannel,
};
pub use cl_trace::{
    optimize_cl_trace, render_ascii_trace_schedule, parse_cl_bundles, is_slot_nop,
    ModuloSchedule, TraceCacheConfig, TraceCacheReport, TraceOptimizationResult,
};
pub use cl_balance::{
    balance_workload, derive_optimal_strategy, render_ascii_mesh_heatmap,
    synthesize_multicore_cl_bundle, BalanceConfig, CoreWorkloadAssignment,
    ParallelismStrategy, WorkloadBalancePlan,
};
pub use cl_perf::{
    profile_cl_kernel, run_cl_perf_suite, render_ascii_ppa_scoreboard,
    ClPerfConfig, ClPerfSuiteReport, PpaMetrics,
};
pub use cl_vcd::{
    dump_vcd, generate_vcd_trace, render_ascii_waveform,
    PipelineCycleSnapshot, VcdConfig, VcdSignalDef, VcdSignalType, VcdTraceReport,
};
pub use cl_cordic::{
    compute_magnitude_angle, compute_rope_frequencies, compute_sin_cos,
    compute_sinh_cosh, render_ascii_bloch_sphere, render_ascii_phase_orbit,
    run_cordic, synthesize_cordic_cl, CordicConfig, CordicMode, CordicResult,
};
pub use cl_infer::{
    generate_tokens, synthesize_transformer_cl, transformer_forward_step,
    GenerationResult, InferenceTelemetry, KVCache, TransformerConfig,
};
pub use cl_snn::{
    simulate_snn, synthesize_snn_kernel,
    LifNeuronConfig, StdpConfig, SpikeEvent, SnnSimulationResult,
};
pub use cl_autotune::{
    run_cl_autotune, synthesize_tuned_cl_kernel, compute_pareto_frontier,
    AutotuneCandidate, AutotuneConfig, AutotuneReport,
};
pub use cl_stream::{
    synthesize_streaming_pipeline, StreamModality, StreamPipelineConfig,
    StreamReport, StreamStage,
};
pub use cl_sparse::{
    compress_2_4_matrix, decompress_2_4_matrix, prune_to_2_4,
    analyze_matrix_sparsity, synthesize_sparse_2_4_gemm,
    Compressed2_4, SparsityReport,
};
pub use cl_power::{
    analyze_cl_power, ClPowerOptions, ClPowerReport,
    MAX_CHIP_TDP_WATTS, THROTTLE_TEMPERATURE_CELSIUS,
};
pub use cl_tile::{tile_gemm, tile_conv2d, derive_optimal_gemm_tiles, TileOptions, TileResult};
pub use cl_cluster::{ClusterCoord, CollectiveSchedule, CollectiveType, synthesize_collective_schedule, render_cluster_topology_ascii, generate_distributed_c23_harness};
pub use cl_lang::{verify_cl_program, audit_alphabet_coverage, parse_slot, parse_weights_directive, ClReport, ClSlot, ClWeightBinding};
pub use cl_spec::{generate_cl_spec, generate_cl_ebnf_grammar, generate_cl_json_schema, generate_cl_ai_system_prompt, SpecFormat};
pub use cl_link::{ClLinker, ClLinkReport, Coord4D, CoreProgram, InterCoreChannel};
pub use vibe_loop::{run_vibe_loop, VibeLoopConfig, VibeLoopResult, VibeStatus, VibeDiagnostic};
pub use cl_cosim::{run_cl_cosim, ClCosimReport, CosimCycleRecord, CosimOptions, VerilogRtlCoreSimulator};
pub use cl_memcheck::{verify_cl_memory_access, ClMemcheckReport, MemcheckOptions, CycleBankConflict, compute_linear_bank, compute_swizzled_bank, prove_strided_conflict_freedom};
pub use cl_fuzz::{run_cl_fuzz, ClFuzzReport, FuzzOptions, MutationStrategy};
pub use cl_bench::{analyze_cl_roofline, ClBenchReport, RooflineRegime, PEAK_CORE_COMPUTE_GFLOPS, PEAK_CORE_SRAM_BW_GBPS, ROOFLINE_KNEE_OI};
pub use cl_kernel::{
    generate_flash_attention, generate_bitnet_gemm, generate_rmsnorm,
    generate_swiglu, generate_rope, generate_kv_cache_stream,
    list_available_kernels, synthesize_kernel, KernelDescriptor,
};
pub use cl_llvm::{compile_cl_to_llvm, ClLlvmCompiler};
pub use cl_jit::{run_cl_jit, execute_cl_on_core, ClJitCore};
pub use cl_heal::{heal_cl_program, heal_slot_crc, ClHealReport, CANONICAL_NOP};
pub use cl_opt::{optimize_cl_program, optimize_cl_program_advanced, ClOptConfig, ClOptLevel, ClOptReport};
pub use comptime::{evaluate_and_fold_program, ComptimeEvaluator, ComptimeValue, ComptimeEnv};
pub use silicon_flash::{generate_hardware_package, emit_hardware_package, FlashConfig, HardwarePackage, HostInterface, SiliconTarget};
pub use formal_verify::{FormalVerifier, FormalVerificationReport};
pub use scheduler::{AOTHazardScheduler, IRInstruction};
pub use jit_backend::run_source_jit;
pub use cl_binary::{assemble_cl_to_clb, disassemble_clb_to_cl};
pub use optimizer::Optimizer;
pub use fdo::{parse_fdo_profile, analyze_profile, compact_nop_bundles, fdo_recompile};
pub use autotune::{SiliconAutotuner, AutotuneDecision};
pub use cl_c23::{compile_cl_to_c23, compile_cl_to_native_binary};
pub use model_importer::{
    import_model_file, lower_onnx_graph, lower_safetensors_model,
    ImportOptions, ImportedModel, LoweredCode, LoweringOptions,
    QuantizationMode, QuantizationReport, QuantizedTensor,
    SafeTensorDType, SafeTensorData, SafeTensorsModel,
    OnnxGraph, OnnxModel, OnnxNode, OnnxTensor,
};

use checker::SemanticChecker;
use codegen::Codegen;
use diagnostic::Diagnostic;
use lexer::Lexer;
use parser::Parser;

pub fn compile_source(source: &str) -> Result<String, String> {
    compile_source_with_name(source, None)
}

pub fn compile_source_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    // SSS+ Optimization Pipeline (runs after validation)
    let mut opt = optimizer::Optimizer::new();
    opt.optimize_program(&mut program);

    let mut codegen = Codegen::new();
    let machine_code = codegen.generate(&program);

    Ok(machine_code)
}

/// Compile source code, then apply FDO-guided recompilation using the given profile.
/// Returns the FDO-optimized VLIW machine code string.
pub fn compile_source_with_fdo(source: &str, fdo_profile: &str) -> Result<String, String> {
    let cl_code = compile_source(source)?;
    let (optimized, _analysis) = fdo_recompile(&cl_code, fdo_profile)?;
    Ok(optimized)
}

pub fn check_source_diagnostics(source: &str) -> Vec<Diagnostic> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            return vec![Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md")];
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            return vec![Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses")];
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return vec![diag];
    }

    Vec::new()
}

pub fn compile_to_verilog(source: &str, module_name: &str) -> Result<String, String> {
    let cl_code = compile_source(source)?;
    verilog_backend::generate_verilog_hdl(&cl_code, module_name)
}

pub fn compile_to_c23(source: &str) -> Result<String, String> {
    compile_to_c23_with_name(source, None)
}

pub fn compile_to_c23_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    // SSS+ Optimization Pipeline (runs after validation)
    let mut opt = optimizer::Optimizer::new();
    opt.optimize_program(&mut program);

    let mut c_backend = c_backend::CBackend::new();
    let c_code = c_backend.generate(&program);

    Ok(c_code)
}

pub fn compile_to_llvm(source: &str) -> Result<String, String> {
    compile_to_llvm_with_name(source, None)
}

pub fn compile_to_llvm_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    // SSS+ Optimization Pipeline (runs after validation)
    let mut opt = optimizer::Optimizer::new();
    opt.optimize_program(&mut program);

    let mut llvm_backend = llvm_backend::LlvmBackend::new();
    let llvm_ir = llvm_backend.generate(&program);

    Ok(llvm_ir)
}

pub fn execute_jit(source: &str) -> Result<i64, String> {
    jit_backend::run_source_jit(source)
}

pub fn compile_to_ptx(source: &str) -> Result<String, String> {
    compile_to_ptx_with_name(source, None)
}

pub fn compile_to_ptx_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    // SSS+ Optimization Pipeline (runs after validation)
    let mut opt = optimizer::Optimizer::new();
    opt.optimize_program(&mut program);

    let mut gpu = gpu_backend::GpuBackend::new();
    let ptx_code = gpu.generate_ptx(&program);

    Ok(ptx_code)
}

pub fn compile_to_metal(source: &str) -> Result<String, String> {
    compile_to_metal_with_name(source, None)
}

pub fn compile_to_metal_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let _ = comptime::evaluate_and_fold_program(&mut program);
    let _ = autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    // SSS+ Optimization Pipeline (runs after validation)
    let mut opt = optimizer::Optimizer::new();
    opt.optimize_program(&mut program);

    let mut gpu = gpu_backend::GpuBackend::new();
    let metal_code = gpu.generate_metal(&program);

    Ok(metal_code)
}

pub fn compile_native_binary(source: &str, output_path: &std::path::Path, extra_flags: &[&str]) -> Result<(), String> {
    let c23_code = if source.trim_start().starts_with("B0") || source.contains("B0000:") {
        compile_cl_to_c23(source, "CronSiliconNative")?
    } else {
        compile_to_c23(source)?
    };
    let temp_c_path = output_path.with_extension("c");
    std::fs::write(&temp_c_path, c23_code).map_err(|e| format!("Failed to write temporary C source: {}", e))?;

    let compilers = ["clang", "gcc"];
    let mut success = false;
    let mut last_err = String::new();

    for comp in &compilers {
        let mut cmd = std::process::Command::new(comp);
        cmd.arg("-std=c2x")
           .arg("-O3")
           .arg(&temp_c_path)
           .arg("-o")
           .arg(output_path);

        for flag in extra_flags {
            cmd.arg(flag);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    success = true;
                    break;
                } else {
                    last_err = String::from_utf8_lossy(&output.stderr).to_string();
                }
            }
            Err(e) => {
                last_err = format!("Failed to execute '{}': {}", comp, e);
            }
        }
    }

    let _ = std::fs::remove_file(temp_c_path);

    if success {
        Ok(())
    } else {
        Err(format!("Native linking failed: {}", last_err))
    }
}

pub fn compile_native_shared_library(source: &str, output_path: &std::path::Path, extra_flags: &[&str]) -> Result<(), String> {
    let mut flags = vec!["-shared"];
    flags.extend_from_slice(extra_flags);
    compile_native_binary(source, output_path, &flags)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_and_parser() {
        let code = r#"
        .MODULE TestMod
        .ENTRY _main
        _main:
            let ext_hbm_base: u32 = 0x1000
            let lin w_seed: wave_t = pack_wave(amp=[1, 2], phase=[3, 4])
            export w_seed as exported_w
        .END
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().expect("Tokenize failed");
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("Parse failed");
        assert_eq!(prog.module_name, "TestMod");
        assert_eq!(prog.entry_name, Some("_main".to_string()));
    }

    #[test]
    fn test_linear_type_leak_detected() {
        let code = r#"
        .MODULE LeakMod
        _main:
            let lin unconsumed_var = 123
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Linear variable 'unconsumed_var' was allocated but never consumed"));
        assert!(err.contains("error[E0002]"));
        assert!(err.contains("--> <input>:4:21"));
    }

    #[test]
    fn test_linear_type_double_consume_detected() {
        let code = r#"
        .MODULE DoubleConsumeMod
        _main:
            let lin x = 123
            let a = consume(x)
            let b = consume(x)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("was already consumed"));
    }

    #[test]
    fn test_operator_precedence_and_expressions() {
        let code = r#"
        .MODULE ExprMod
        _main:
            let a = 2 + 3 * 4
            let b = (2 + 3) * 4
            let c = a > 10 and b <= 20
            let d = not c or false
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile expressions: {:?}", result.err());
    }

    #[test]
    fn test_control_flow_if_while_for() {
        let code = r#"
        .MODULE ControlFlowMod
        _main:
            let mut x = 10
            if x > 5 {
                let y = x + 1
            } else {
                let y = 0
            }

            while x > 0 {
                x = x - 1
            }

            for item in [1, 2, 3] {
                let z = item * 2
            }
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile control flow: {:?}", result.err());
    }

    #[test]
    fn test_reassign_immutable_fails() {
        let code = r#"
        .MODULE ImmMod
        _main:
            let x = 10
            x = 20
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("error[E0005]"));
        assert!(err.contains("Cannot assign twice to immutable variable 'x'"));
        assert!(err.contains("--> <input>:5:13"));
    }

    #[test]
    fn test_assign_to_consumed_linear_rejected() {
        let code = r#"
        .MODULE InvalidAssignMod
        _main:
            let mut lin res = 50
            let x = consume(res)
            res = 100
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err(), "Expected assigning to consumed linear variable to fail");
        assert!(result.unwrap_err().contains("was already consumed"));
    }

    #[test]
    fn test_struct_declaration_and_init() {
        let code = r#"
        .MODULE StructTest
        struct Waveform {
            amp: u32,
            phase: u32,
        }
        _main:
            let w = Waveform { amp: 10, phase: 20 }
            let a = w.amp
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile struct init: {:?}", result.err());
    }

    #[test]
    fn test_import_and_type_alias() {
        let code = r#"
        .MODULE ImportTest
        import { SpatialTensor, RoutingMatrix } from "core/spatial.cr"
        type Angle = f64
        type Matrix4x4 = [[f64; 4]; 4]
        _main:
            let theta: Angle = 3.14159
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile import and type alias: {:?}", result.err());
    }

    #[test]
    fn test_nested_if_else_chains() {
        let code = r#"
        .MODULE NestedIfMod
        _main:
            let x = 15
            let mut result = 0
            if x > 20 {
                result = 1
            } else if x > 10 {
                result = 2
            } else {
                result = 3
            }
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile nested if: {:?}", result.err());
    }

    #[test]
    fn test_function_multi_params() {
        let code = r#"
        .MODULE FuncMod
        def compute_energy(freq: f64, lin photon: u32, h_bar: f64) -> f64 {
            let p = consume(photon)
            return freq * h_bar
        }
        _main:
            let lin p = 42
            let e = compute_energy(1.5e14, consume(p), 6.626e-34)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile multi-param function: {:?}", result.err());
    }

    #[test]
    fn test_inline_function_expansion() {
        let code = r#"
        .MODULE InlineMod
        inline def fast_dot(a: u32, b: u32) -> u32 {
            let res = a * b
            return res
        }
        _main:
            let x = fast_dot(3, 4)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile inline function: {:?}", result.err());
    }

    #[test]
    fn test_bitwise_operators() {
        let code = r#"
        .MODULE BitwiseMod
        _main:
            let a = 0x0F & 0x33
            let b = 0x0F | 0x30
            let c = 0xAA ^ 0x55
            let d = a << 2
            let e = b >> 1
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile bitwise ops: {:?}", result.err());
    }

    #[test]
    fn test_comparison_all_operators() {
        let code = r#"
        .MODULE CmpMod
        _main:
            let a = 10 < 20
            let b = 10 <= 10
            let c = 20 > 10
            let d = 20 >= 20
            let e = 10 == 10
            let f = 10 != 20
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile comparisons: {:?}", result.err());
    }

    #[test]
    fn test_type_cast_expression() {
        let code = r#"
        .MODULE CastMod
        _main:
            let raw: u64 = 0x1234
            let truncated = raw as u32
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile cast: {:?}", result.err());
    }

    #[test]
    fn test_unary_negation_and_inversion() {
        let code = r#"
        .MODULE UnaryMod
        _main:
            let a = -42
            let b = -3.14
            let c = not true
            let d = !false
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile unary: {:?}", result.err());
    }

    #[test]
    fn test_array_repeat_and_empty_tuple() {
        let code = r#"
        .MODULE ArrayTupleMod
        _main:
            let empty_unit = ()
            let zeros = [0; 256]
            let pair = (1, 2)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile array repeat and tuple: {:?}", result.err());
    }

    #[test]
    fn test_scientific_notation_literals() {
        let code = r#"
        .MODULE SciMod
        _main:
            let a = 1e5
            let b = 1.0e-5
            let c = 2.5e+10
            let d = 3E4
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile scientific notation: {:?}", result.err());
    }

    #[test]
    fn test_comments_and_blank_lines() {
        let code = r#"
        // Line comment 1
        # Python-style comment

        /* Multi-line
           block comment */

        .MODULE CommentMod

        // Comment before entry
        _main:
            ; Semicolon start comment
            let x = 100 // trailing comment
            /* inline block */ let y = 200

        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile with various comments: {:?}", result.err());
    }

    #[test]
    fn test_hex_literals_with_underscores() {
        let code = r#"
        .MODULE HexMod
        _main:
            let addr = 0x000A_0000
            let mask = 0xFFFF_0000
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile hex with underscores: {:?}", result.err());
    }

    #[test]
    fn test_await_and_spawn_expressions() {
        let code = r#"
        .MODULE AsyncMod
        _main:
            let handle = spawn (1 + 2)
            let result = await handle
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile async expressions: {:?}", result.err());
    }

    #[test]
    fn test_crc8_token_integrity() {
        use crate::cl_lang::{compute_crc8_atm, pack_slot_with_crc8, verify_token_crc8};
        let crc = compute_crc8_atm(b"_OP04$1");
        assert_ne!(crc, 0);

        let packed_slot = pack_slot_with_crc8('_', "OP", "04", '$', '1', '2', '>');
        assert_eq!(packed_slot.len(), 10);
        assert!(verify_token_crc8(&packed_slot));

        // Corrupt a byte in the payload and verify detection
        let corrupted = "_OP05$1".to_string() + &packed_slot[7..];
        assert!(!verify_token_crc8(&corrupted));
    }
}
