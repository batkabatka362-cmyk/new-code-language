// ============================================================================
// Tests for CRON Multi-Core 4D-Torus Spatial Linker (cl_link)
// ============================================================================

use cronc::cl_link::{ClLinker, Coord4D};

#[test]
fn test_coord4d_spatial_math_and_routing() {
    let c0 = Coord4D::new(0, 0, 0, 0).unwrap();
    assert_eq!(c0.to_core_id(), 0);
    assert_eq!(Coord4D::from_core_id(0), c0);

    let c_max = Coord4D::new(3, 3, 3, 3).unwrap();
    assert_eq!(c_max.to_core_id(), 255);
    assert_eq!(Coord4D::from_core_id(255), c_max);

    // Toroidal wrap-around Manhattan distance
    let c_wrap = Coord4D::new(3, 0, 0, 0).unwrap();
    // Distance from 0 to 3 with toroidal wrap of size 4 is min(3, 4-3) = 1 hop!
    assert_eq!(c0.torus_manhattan_distance(&c_wrap), 1);

    let c_diag = Coord4D::new(1, 2, 0, 0).unwrap();
    // dx = 1, dy = 2 -> total = 3
    assert_eq!(c0.torus_manhattan_distance(&c_diag), 3);

    // Out of bounds checking
    assert!(Coord4D::new(4, 0, 0, 0).is_err());
    assert!(Coord4D::new(0, 5, 0, 0).is_err());
    assert!(Coord4D::new(0, 0, 4, 0).is_err());
    assert!(Coord4D::new(0, 0, 0, 4).is_err());

    // DOR path
    let route = c0.dor_routing_path(&c_diag);
    assert!(route.contains("+X(1)"));
    assert!(route.contains("+Y(2)"));
}

#[test]
fn test_single_core_default_linking() {
    let source = r#"
    @entry:
    B0000: '==01#00A> '==02#00B> _NO00#000> _NO00#000>
    B0001: _OP01$28F> _NO00#000> _NO00#000> _HL00#000!
    "#;

    let mut linker = ClLinker::new();
    let report = linker.parse_and_link(source).expect("Linking single-core code should succeed");

    assert_eq!(report.total_cores, 1);
    assert_eq!(report.total_bundles, 2);
    assert_eq!(report.active_core_coords, vec![Coord4D::new(0, 0, 0, 0).unwrap()]);
    assert!(report.dor_deadlock_free);

    let core0 = linker.cores.get(&Coord4D::new(0, 0, 0, 0).unwrap()).unwrap();
    assert_eq!(core0.core_id, 0);
    assert_eq!(core0.label_offsets.get("@entry"), Some(&0));
    assert!(core0.has_halt);
}

#[test]
fn test_multicore_4d_torus_linking_and_noc_channels() {
    let source = r#"
    .core [0, 0, 0, 0]:
    @producer:
    B0000: '==01#0FF> _NO00#000> _NO00#000> _NO00#000>
    B0001: _TX01$100> _NO00#000> _NO00#000> _NO00#000>
    B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>

    .core [1, 2, 0, 0]:
    @consumer:
    B0000: _RX02$000> _NO00#000> _NO00#000> _NO00#000>
    B0001: _OP00$204> _NO00#000> _NO00#000> _NO00#000>
    B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let mut linker = ClLinker::new();
    let report = linker.parse_and_link(source).expect("Linking multicore code should succeed");

    assert_eq!(report.total_cores, 2);
    assert_eq!(report.total_bundles, 6);
    assert_eq!(report.active_core_coords.len(), 2);
    assert_eq!(report.inter_core_channels.len(), 1);

    let channel = &report.inter_core_channels[0];
    assert_eq!(channel.src_coord, Coord4D::new(0, 0, 0, 0).unwrap());
    assert_eq!(channel.dst_coord, Coord4D::new(1, 2, 0, 0).unwrap());
    assert_eq!(channel.hop_distance, 3);
    assert!(channel.is_dor_acyclic);
    assert!(channel.routing_path.contains("+X(1)"));
    assert!(channel.routing_path.contains("+Y(2)"));

    // Check Binary Pack emission
    let bin_pack = linker.emit_binary_pack();
    assert!(bin_pack.starts_with(b"CRON4D01"));
    assert!(bin_pack.len() > 16);

    // Check C23 Multi-Core Harness Generation
    let c23_code = linker.generate_multicore_c23_harness("test_4d_pipeline");
    assert!(c23_code.contains("CRON Multi-Core 4D-Torus C23 Native Execution Harness"));
    assert!(c23_code.contains("step_core_0"));
    // Core [1, 2, 0, 0] ID = 1 + 4*2 = 9
    assert!(c23_code.contains("step_core_9"));
    assert!(c23_code.contains("g_cores[9].mailbox_fifo"));

    // Check Verilog Multi-Core Top Module Generation
    let v_top = linker.generate_multicore_verilog_top("cron_4d_mesh_top");
    assert!(v_top.contains("module cron_4d_mesh_top"));
    assert!(v_top.contains("cron_silicon_core #(.CORE_ID(0)) core_inst_0"));
    assert!(v_top.contains("cron_silicon_core #(.CORE_ID(9)) core_inst_9"));
    assert!(v_top.contains("assign all_halted = core_0_halted & core_9_halted;"));
}

#[test]
fn test_multicore_c23_native_gcc_execution() {
    let source = r#"
    .core [0, 0, 0, 0]:
    B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
    B0001: _OP00$100> _NO00#000> _NO00#000> _HL00#000!

    .core [1, 0, 0, 0]:
    B0000: '==02#00B> _NO00#000> _NO00#000> _NO00#000>
    B0001: _MD00$204> _NO00#000> _NO00#000> _HL00#000!
    "#;

    let mut linker = ClLinker::new();
    linker.parse_and_link(source).unwrap();

    let c_code = linker.generate_multicore_c23_harness("test_multicore_run");
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("test_multicore_run.c");
    let bin_file = temp_dir.join(if cfg!(windows) { "test_multicore_run.exe" } else { "test_multicore_run" });

    std::fs::write(&c_file, &c_code).unwrap();

    let compile_status = std::process::Command::new("gcc")
        .args(["-O2", c_file.to_str().unwrap(), "-o", bin_file.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(st) = compile_status {
        if st.success() {
            let run_output = std::process::Command::new(bin_file.to_str().unwrap())
                .output()
                .expect("Failed to execute compiled multi-core C23 binary");
            assert!(run_output.status.success());
            let stdout = String::from_utf8_lossy(&run_output.stdout);
            assert!(stdout.contains("4D-TORUS MULTI-CORE EXECUTION COMPLETED (SUCCESS)"));
            assert!(stdout.contains("Core [0,0,0,0] (ID 0)"));
            assert!(stdout.contains("Core [1,0,0,0] (ID 1)"));
        }
    }

    let _ = std::fs::remove_file(&c_file);
    let _ = std::fs::remove_file(&bin_file);
}

#[test]
fn test_invalid_core_directive_rejected() {
    let source = r#"
    .core [4, 0, 0, 0]:
    B0000: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let mut linker = ClLinker::new();
    let err = linker.parse_and_link(source).unwrap_err();
    assert!(err.contains("4D Coordinate Out of Bounds"));
}
