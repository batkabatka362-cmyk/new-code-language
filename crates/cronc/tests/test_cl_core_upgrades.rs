use cronc::cl_lang::{parse_directive, verify_cl_program, ClDirective};
use cronc::cl_binary::{assemble_cl_to_clb, disassemble_clb_to_cl};
use cronc::cl_opt::optimize_cl_program;

#[test]
fn test_cl_loop_and_barrier_directives() {
    let loop_directive = ".loop count=1024, stride=1 {";
    let parsed_loop = parse_directive(loop_directive).expect("Must parse .loop directive");
    match parsed_loop {
        ClDirective::Loop { count, stride } => {
            assert_eq!(count, 1024);
            assert_eq!(stride, 1);
        }
        _ => panic!("Expected ClDirective::Loop"),
    }

    let barrier_directive = ".barrier name=\"attn_layer_barrier\", cores=\"all\"";
    let parsed_barrier = parse_directive(barrier_directive).expect("Must parse .barrier directive");
    match parsed_barrier {
        ClDirective::Barrier { name, cores } => {
            assert_eq!(name, "attn_layer_barrier");
            assert_eq!(cores, "all");
        }
        _ => panic!("Expected ClDirective::Barrier"),
    }
}

#[test]
fn test_cl_memory_hierarchy_directives() {
    let sram_directive = ".sram size=64KB, bank=0";
    let parsed_sram = parse_directive(sram_directive).expect("Must parse .sram directive");
    match parsed_sram {
        ClDirective::MemorySpace { space, size_kb, bank } => {
            assert_eq!(space, "sram");
            assert_eq!(size_kb, 64);
            assert_eq!(bank, 0);
        }
        _ => panic!("Expected ClDirective::MemorySpace for sram"),
    }

    let hbm_directive = ".hbm size=16GB, bank=1";
    let parsed_hbm = parse_directive(hbm_directive).expect("Must parse .hbm directive");
    match parsed_hbm {
        ClDirective::MemorySpace { space, size_kb, bank } => {
            assert_eq!(space, "hbm");
            assert_eq!(size_kb, 16 * 1024 * 1024);
            assert_eq!(bank, 1);
        }
        _ => panic!("Expected ClDirective::MemorySpace for hbm"),
    }
}

#[test]
fn test_cl_program_verification_with_upgrades() {
    let cl_code = r#"
.stage "bitnet_core_loop", precision="1.58b_ternary", d_model=256
.sram size=64KB, bank=0
.hbm size=16GB, bank=1
.barrier name="sync_all_cores", cores="all"
.core [0, 0, 0, 0]:
.loop count=256, stride=1 {
    B0000: '==01#010> _SZ05$081> ~PF03@210> _NO00#000>
    B0001: _TX05$1B0> _TX06$110> _TX07$170> _PK08`270>
    B0002: _SB00#060> _NO00#000> _NO00#000> _NO00#000>
}
"#;

    let report = verify_cl_program(cl_code).expect("Must verify cleanly");
    assert!(report.directives_found >= 4, "Directives count must be >= 4, got {}", report.directives_found);
    assert_eq!(report.hazards.len(), 0, "Hazard count must be 0: {:?}", report.hazards);

    // Test assembling to .clb binary
    let binary = assemble_cl_to_clb(cl_code).expect("Must assemble to .clb");
    assert!(binary.len() > 12);
    assert_eq!(&binary[0..4], b"CLB1");

    // Test disassembling back
    let disassembled = disassemble_clb_to_cl(&binary).expect("Must disassemble back");
    assert!(disassembled.contains("B0000:"));
    assert!(disassembled.contains("B0001:"));
}

#[test]
fn test_cl_super_optimizer_compaction() {
    let unoptimized = r#"
    B0000: '==01#010> _NO00#000> _NO00#000> _NO00#000>
    B0001: _NO00#000> '==02#020> _NO00#000> _NO00#000>
    B0002: _NO00#000> _NO00#000> '==03#030> _NO00#000>
    B0003: _NO00#000> _NO00#000> _NO00#000> '==04#040>
    "#;

    let report = optimize_cl_program(unoptimized).expect("Optimizer must succeed");
    assert!(report.optimized_bundles < report.original_bundles, 
        "Optimizer must compact bundles from {} to fewer, got {}", report.original_bundles, report.optimized_bundles);
    assert!(report.optimized_ipc > report.original_ipc,
        "Optimized IPC must be higher than original IPC");
}
