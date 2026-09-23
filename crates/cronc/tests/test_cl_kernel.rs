use cronc::cl_kernel::{list_available_kernels, synthesize_kernel};
use cronc::cl_memcheck::{verify_cl_memory_access, MemcheckOptions};
use cronc::cl_jit::{execute_cl_on_core, ClJitCore};

#[test]
fn test_cl_kernel_list_catalog() {
    let catalog = list_available_kernels();
    assert_eq!(catalog.len(), 6, "Must provide all 6 foundational AI operators");

    let names: Vec<&str> = catalog.iter().map(|k| k.name).collect();
    assert!(names.contains(&"flash-attn"));
    assert!(names.contains(&"bitnet-gemm"));
    assert!(names.contains(&"rmsnorm"));
    assert!(names.contains(&"swiglu"));
    assert!(names.contains(&"rope"));
    assert!(names.contains(&"kv-cache"));
}

#[test]
fn test_cl_kernel_synthesis_and_validation() {
    let kernel_names = ["flash-attn", "bitnet-gemm", "rmsnorm", "swiglu", "rope", "kv-cache"];

    for &name in &kernel_names {
        let kernel_code = synthesize_kernel(name, 64, 16)
            .unwrap_or_else(|e| panic!("Failed to synthesize kernel '{}': {}", name, e));

        assert!(!kernel_code.is_empty(), "Synthesized kernel '{}' cannot be empty", name);
        assert!(kernel_code.contains("B0000:"), "Kernel '{}' must contain bundle B0000", name);
        assert!(kernel_code.contains("_HL00"), "Kernel '{}' must contain halt instruction", name);
    }
}

#[test]
fn test_cl_kernel_memcheck_conflict_freedom() {
    let kernel_names = ["flash-attn", "bitnet-gemm", "rmsnorm", "swiglu", "rope", "kv-cache"];
    let mem_opts = MemcheckOptions {
        enable_swizzling: true,
        strict_mode: true,
        ..Default::default()
    };

    for &name in &kernel_names {
        let kernel_code = synthesize_kernel(name, 64, 16).unwrap();
        let report = verify_cl_memory_access(&kernel_code, &mem_opts)
            .unwrap_or_else(|e| panic!("Memcheck failed for kernel '{}': {}", name, e));

        if !report.is_provably_conflict_free {
            println!("KERNEL CODE:\n{}", kernel_code);
            println!("{}", report.format_ascii_report(name));
        }
        assert!(
            report.is_provably_conflict_free,
            "Synthesized kernel '{}' must be provably 100% bank-conflict-free (had {} conflicted cycles)",
            name,
            report.conflicted_cycles
        );
        assert_eq!(report.total_stall_penalty_cycles, 0);
    }
}

#[test]
fn test_cl_kernel_jit_execution() {
    let kernel_names = ["flash-attn", "bitnet-gemm", "rmsnorm", "swiglu", "rope", "kv-cache"];

    for &name in &kernel_names {
        let kernel_code = synthesize_kernel(name, 64, 16).unwrap();
        let mut core = ClJitCore::new();
        let exec_res = execute_cl_on_core(&kernel_code, &mut core);

        assert!(
            exec_res.is_ok(),
            "Synthesized kernel '{}' must execute cleanly on ClJitCore, got error: {:?}",
            name,
            exec_res.err()
        );
        assert!(core.cycle_count > 0, "Kernel '{}' must execute at least 1 cycle", name);
    }
}
