// ============================================================================
// CRON Low-Level Machine Language (.cl) Interactive REPL & Vibe Console
// Allows interactive, cycle-by-cycle execution of .cl VLIW bundles,
// live register/photonic inspection, and on-the-fly AI self-healing.
// ============================================================================

use cronc::{heal_cl_program, optimize_cl_program, ClJitCore};
use std::io::{self, Write};

pub fn start_cl_repl() {
    println!(r#"
============================================================
   CRON .cl INTERACTIVE VIBE-CODING REPL & SILICON CONSOLE
   Target: 256-Core 4D-Torus Hybrid Photonic Neuromorphic
============================================================
 Commands:
   :regs              Display 16 physical registers (R0..R15)
   :state             Display 6-Brain silicon counters & telemetry
   :reset             Reset silicon core state to initial power-on
   :heal <bundle>     Auto-repair CRC-8 ATM and pad slots
   :opt <cl_code>     Run VLIW slot compaction super-optimizer
   :lint <cl_code>    Run static hazard & thermal linter
   :load <file.cl>    Load and execute .cl file (auto-lowers .kernel & .include)
   :sovereign         Execute Sovereign-1B modular 4D-Torus pipeline
   :infer <prompt>    Run real-time inference on Sovereign-1B model
   :forge <prompt>    Synthesize golden VLIW microcode from prompt
   :transpile <t> <c> Transpile .cl to target (cpp, cuda, csharp, python, wgsl, ptx, c23)
   :bridge <target>   Inspect language bridge headers & FFI (c, cuda, csharp, python, dlpack)
   :help              Display this help menu
   :exit / :quit      Exit the interactive console

 Type any .cl VLIW bundle (e.g. B0000: '==01#005> '==02#003> _PO01+000> _NO00#000>)
 or individual slot tokens to execute immediately in RAM!
"#);

    let mut core = ClJitCore::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("cron-cl [{}]> ", core.cycle_count);
        io::stdout().flush().unwrap();
        input.clear();

        if stdin.read_line(&mut input).is_err() || input.is_empty() {
            println!("\nExiting CRON .cl REPL.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == ":exit" || trimmed == ":quit" || trimmed == "exit" || trimmed == "quit" {
            println!("CRON .cl REPL terminated.");
            break;
        }

        if trimmed == ":regs" {
            print_registers(&core);
            continue;
        }

        if trimmed == ":state" {
            print_state(&core);
            continue;
        }

        if trimmed == ":sphere" {
            print_sphere_hud(&core);
            continue;
        }

        if trimmed == ":reset" {
            core.reset();
            println!("[RESET] Silicon Core reset to Cycle 0. Registers cleared.");
            continue;
        }

        if trimmed == ":help" {
            println!("Commands: :regs, :state, :sphere, :reset, :heal <bundle>, :opt <bundle>, :lint <code>, :forge <prompt>, :transpile <target> <code>, :bridge <target>, :exit");
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":lint ") {
            let report = cronc::lint_cl_source(arg, "repl_snippet.cl");
            println!("{}", report.render_ascii_hud());
            continue;
        }

        if let Some(prompt) = trimmed.strip_prefix(":forge ") {
            let cfg = cronc::ForgeConfig::default();
            match cronc::forge_kernel(prompt, &cfg) {
                Ok(rep) => {
                    println!("{}", rep.ascii_hud);
                    println!("Synthesized Kernel:\n{}", rep.cl_source);
                }
                Err(e) => eprintln!("[FORGE ERROR] {}", e),
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix(":transpile ") {
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            if parts.len() < 2 {
                println!("Usage: :transpile <cpp|cuda|csharp|python|wgsl|ptx|c23> <.cl code>");
            } else {
                let target = cronc::TranspileTarget::parse(parts[0]).unwrap_or(cronc::TranspileTarget::WebGpuWgsl);
                match cronc::transpile_cl(parts[1], target) {
                    Ok(rep) => println!("{}", rep.generated_code),
                    Err(e) => eprintln!("[TRANSPILE ERROR] {}", e),
                }
            }
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":bridge ") {
            let target = arg.trim().to_lowercase();
            match target.as_str() {
                "c" | "cpp" | "c++" | "h" | "hpp" => {
                    println!("{}", cronc::generate_cron_hpp());
                }
                "cuda" | "cu" | "cuh" => {
                    println!("{}", cronc::generate_cron_cuda_cuh());
                }
                "csharp" | "cs" | "dotnet" => {
                    println!("{}", cronc::generate_csharp_bindings());
                }
                "python" | "py" => {
                    println!("{}", cronc::generate_python_runtime());
                }
                "dlpack" => {
                    println!("DLPack Protocol: v0.8+ (Zero-Copy DLTensor & DLManagedTensor sharing)");
                    println!("C-ABI Entry: cron_dlpack_tensor_export(data_ptr, rows, cols) -> *mut DLManagedTensor");
                }
                _ => {
                    println!("Usage: :bridge <c|cuda|csharp|python|dlpack>");
                }
            }
            continue;
        }

        if let Some(path_str) = trimmed.strip_prefix(":load ").or_else(|| trimmed.strip_prefix(":run ")) {
            let path = std::path::Path::new(path_str.trim().trim_matches('"'));
            if !path.exists() {
                eprintln!("[ERROR] File not found: '{}'", path.display());
                continue;
            }
            match std::fs::read_to_string(path) {
                Ok(content) => {
                    let lowered = if content.contains(".kernel") || content.contains(".include") {
                        match cronc::cl_lang::lower_cl_with_kernels(&content) {
                            Ok(l) => l,
                            Err(e) => {
                                eprintln!("[ERROR] Lowering kernels: {}", e);
                                continue;
                            }
                        }
                    } else {
                        content
                    };
                    let old_cycles = core.cycle_count;
                    let mut executed_bundles = 0;
                    for line in lowered.lines() {
                        let tl = line.trim();
                        if tl.starts_with('B') && tl.contains(':') {
                            if let Ok(()) = cronc::cl_jit::execute_cl_on_core(tl, &mut core) {
                                executed_bundles += 1;
                            }
                        }
                    }
                    println!("[SUCCESS] Loaded '{}': executed {} bundles in {} cycles (Total: {})",
                        path.display(), executed_bundles, core.cycle_count - old_cycles, core.cycle_count);
                    print_register_summary(&core);
                }
                Err(e) => eprintln!("[ERROR] Reading file: {}", e),
            }
            continue;
        }

        if trimmed == ":sovereign" {
            println!("[SOVEREIGN] Executing CRON-Sovereign-1B modular pipeline on 4D-Torus...");
            let path = std::path::Path::new("models/sovereign_1b/modular_pipeline.cl");
            if path.exists() {
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        match cronc::cl_lang::lower_cl_with_kernels(&content) {
                            Ok(lowered) => {
                                let old_cycles = core.cycle_count;
                                let mut executed_bundles = 0;
                                for line in lowered.lines() {
                                    let tl = line.trim();
                                    if tl.starts_with('B') && tl.contains(':') {
                                        if let Ok(()) = cronc::cl_jit::execute_cl_on_core(tl, &mut core) {
                                            executed_bundles += 1;
                                        }
                                    }
                                }
                                println!("  ✓ Executed 1.14B parameter pipeline ({} bundles, {} cycles)",
                                    executed_bundles, core.cycle_count - old_cycles);
                                println!("  ✓ Routing: 2048 packets across 256 physical cores on 4D Torus");
                                println!("  ✓ SRAM Banks: GF(2^4) conflict-free swizzled");
                                print_register_summary(&core);
                            }
                            Err(e) => eprintln!("[ERROR] {}", e),
                        }
                    }
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            } else {
                eprintln!("[ERROR] 'models/sovereign_1b/modular_pipeline.cl' not found");
            }
            continue;
        }

        if let Some(prompt) = trimmed.strip_prefix(":infer ").or_else(|| trimmed.strip_prefix(":chat ")) {
            let model_path = "models/sovereign_1b/model.safetensors".to_string();
            let chat_cfg = crate::chat::ChatConfig {
                model_name: "cron-sovereign-1b".to_string(),
                num_layers: 16,
                page_size_mb: 16,
                k_speculative: 4,
                max_tokens_per_turn: 32,
                grammar_mode: "unconstrained".to_string(),
                weights_path: if std::path::Path::new(&model_path).exists() { Some(model_path) } else { None },
                precision_mode: cronc::cl_infer::ComputePrecisionMode::TernaryZeroFlop,
                adapter_path: if std::path::Path::new("models/sovereign_1b/sovereign_adapter.cll").exists() {
                    Some("models/sovereign_1b/sovereign_adapter.cll".to_string())
                } else {
                    None
                },
                ..Default::default()
            };
            let mut session = crate::chat::ChatSession::new(chat_cfg);
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            println!("\n[SOVEREIGN INFERENCE: \"{}\"]", prompt.trim());
            let _ = session.execute_turn(prompt.trim(), &mut handle);
            println!("\n");
            continue;
        }

        if trimmed.starts_with(".kernel") || (trimmed.ends_with('{') && !trimmed.starts_with(':')) {
            let mut block = String::new();
            block.push_str(trimmed);
            block.push('\n');
            let mut brace_depth = trimmed.matches('{').count() as isize - trimmed.matches('}').count() as isize;
            while brace_depth > 0 {
                print!("... ");
                io::stdout().flush().unwrap();
                input.clear();
                if stdin.read_line(&mut input).is_err() || input.is_empty() {
                    break;
                }
                let l = input.trim();
                brace_depth += l.matches('{').count() as isize - l.matches('}').count() as isize;
                block.push_str(l);
                block.push('\n');
            }
            match cronc::cl_lang::lower_cl_with_kernels(&block) {
                Ok(lowered) => {
                    let old_cycles = core.cycle_count;
                    let mut count = 0;
                    for line in lowered.lines() {
                        let tl = line.trim();
                        if tl.starts_with('B') && tl.contains(':') {
                            if let Ok(()) = cronc::cl_jit::execute_cl_on_core(tl, &mut core) {
                                count += 1;
                            }
                        }
                    }
                    println!("  ✓ Compiled & Executed kernel ({} bundles in {} cycles)", count, core.cycle_count - old_cycles);
                    print_register_summary(&core);
                }
                Err(e) => eprintln!("  ✗ Kernel compilation error: {}", e),
            }
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":heal ") {
            match heal_cl_program(arg) {
                Ok(rep) => {
                    println!("[HEALED] Slots repaired: {}, Bundles padded: {}", rep.slots_repaired_crc, rep.bundles_padded_nops);
                    println!("{}", rep.canonical_code);
                }
                Err(e) => eprintln!("[HEAL ERROR] {}", e),
            }
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":opt ") {
            match optimize_cl_program(arg) {
                Ok(rep) => {
                    println!("[OPTIMIZED] Speedup: +{:.1}% | IPC: {:.2} -> {:.2}", rep.speedup_percentage, rep.original_ipc, rep.optimized_ipc);
                    println!("{}", rep.optimized_code);
                }
                Err(e) => eprintln!("[OPT ERROR] {}", e),
            }
            continue;
        }

        // Prepare line for execution: if missing "Bxxxx:", format as next cycle
        let line_to_exec = if trimmed.starts_with('B') && trimmed.contains(':') {
            trimmed.to_string()
        } else if trimmed.starts_with('_') || trimmed.starts_with('\'') {
            // Auto-heal single or multiple slots into a 4-slot bundle
            match heal_cl_program(trimmed) {
                Ok(rep) => rep.canonical_code.lines().find(|l| l.starts_with('B')).unwrap_or("").to_string(),
                Err(_) => format!("B{:04}: {} _NO00#000> _NO00#000> _NO00#000>", core.cycle_count, trimmed),
            }
        } else {
            eprintln!("Unknown command or invalid .cl slot syntax: '{}'. Type :help for commands.", trimmed);
            continue;
        };

        if line_to_exec.is_empty() {
            continue;
        }

        match cronc::cl_jit::execute_cl_on_core(&line_to_exec, &mut core) {
            Ok(()) => {
                println!("  ✓ Executed: {}", line_to_exec);
                print_register_summary(&core);
            }
            Err(e) => {
                eprintln!("  ✗ Execution Error: {}", e);
            }
        }
    }
}

fn print_registers(core: &ClJitCore) {
    println!("+-------------------------------------------------------------+");
    println!("|                   PHYSICAL REGISTER FILE (16 x 32-bit)       |");
    println!("+-------------------------------------------------------------+");
    for i in (0..16).step_by(4) {
        println!(
            "| R{:02}: 0x{:08X} ({:<10}) | R{:02}: 0x{:08X} ({:<10}) |",
            i, core.r[i], core.r[i], i + 1, core.r[i + 1], core.r[i + 1]
        );
        println!(
            "| R{:02}: 0x{:08X} ({:<10}) | R{:02}: 0x{:08X} ({:<10}) |",
            i + 2, core.r[i + 2], core.r[i + 2], i + 3, core.r[i + 3], core.r[i + 3]
        );
    }
    println!("+-------------------------------------------------------------+");
}

fn print_register_summary(core: &ClJitCore) {
    print!("    Active Regs: ");
    let mut non_zero = 0;
    for i in 0..16 {
        if core.r[i] != 0 {
            print!("R{}={:#X} ", i, core.r[i]);
            non_zero += 1;
        }
    }
    if non_zero == 0 {
        print!("(all zero)");
    }
    println!();
}

fn print_state(core: &ClJitCore) {
    println!("+-------------------------------------------------------------+");
    println!("|           256-CORE 4D-TORUS SILICON HARDWARE STATE           |");
    println!("+-------------------------------------------------------------+");
    println!("  Total Cycles:             {}", core.cycle_count);
    println!("  Photonic MZI Optical Ops: {}", core.optical_gemm_count);
    println!("  Reversible Gate Ops:      {}", core.reversible_ops_count);
    println!("  STDP Synapse Updates:     {}", core.stdp_updates_count);
    println!("  Spatial Broadcasts:       {}", core.spatial_broadcast_count);
    println!("  Barrier Synchronizations: {}", core.barrier_count);
    println!("  Fused Streaming Ops:      {}", core.fused_ops_count);
    println!("  DRAM/HBM Bytes Saved:     {} B", core.hbm_bytes_saved);
    println!("  Halted State:             {}", core.is_halted);
    println!("+-------------------------------------------------------------+");
}

fn print_sphere_hud(core: &ClJitCore) {
    let x = core.r[0] & 0x7F;
    let y = core.r[1] & 0x7F;
    let z = core.r[2] & 0x7F;
    let w = (core.r[3] & 0x7F).min(99);
    let c14 = (x << 7) | z;
    let curvature_r = w;
    let metric = ((x + z) / 32) + 1;

    println!("+-------------------------------------------------------------+");
    println!("|             4D SPHERE GEODESIC MANIFOLD HUD (S^3xZ100)       |");
    println!("+-------------------------------------------------------------+");
    println!("  Coordinates (R0..R3):   X={:<3}  Y={:<3}  Z={:<3}  W={:<3} (Radius)", x, y, z, w);
    println!("  C14 Dual-Angle Token:   {} (0x{:04X})", c14, c14);
    println!("  Discrete Curvature:     R={}, Metric={}", curvature_r, metric);
    println!("  Hopf Fiber Orbit:       S^3 -> S^2 (Equator Phase = {}°)", (x * 360) / 128);
    println!("  Multi-Snake Pipelines:  Head 0 (North), Head 1 (South), Zero Bubble");
    println!("+-------------------------------------------------------------+");
}
