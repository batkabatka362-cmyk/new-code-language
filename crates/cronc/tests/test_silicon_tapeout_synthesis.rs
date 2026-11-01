// ============================================================================
// Integration Test: 1-Click FPGA & OpenLane ASIC Silicon Synthesis Suite
// Tests: crates/cronc/src/cl_silicon_tapeout.rs
//
// 100% Pure Rust - Zero Python, Zero External EDA Dependency for Generation
// ============================================================================

use cronc::cl_silicon_tapeout::{
    generate_openlane_config, generate_sdc_constraints, generate_silicon_synth_package,
    generate_verilog_testbench, generate_verilog_top, generate_xdc_constraints,
    SiliconSynthConfig, SiliconTarget,
};
use std::fs;
use std::path::Path;

#[test]
fn test_synthesizable_verilog_generation() {
    let mut cfg = SiliconSynthConfig::default();
    cfg.design_name = "test_core".to_string();
    cfg.in_dim = 8;
    cfg.out_dim = 8;
    cfg.clock_freq_mhz = 250.0;

    let verilog = generate_verilog_top(&cfg);
    assert!(verilog.contains("module test_core"));
    assert!(verilog.contains("s_axis_tdata"));
    assert!(verilog.contains("s_axis_tvalid"));
    assert!(verilog.contains("s_axis_tready"));
    assert!(verilog.contains("m_axis_tdata"));
    assert!(verilog.contains("m_axis_tvalid"));
    assert!(verilog.contains("m_axis_tready"));
    assert!(verilog.contains("FSM_COMPUTE"));
}

#[test]
fn test_self_checking_testbench_generation() {
    let mut cfg = SiliconSynthConfig::default();
    cfg.design_name = "test_core".to_string();
    cfg.clock_freq_mhz = 100.0;

    let tb = generate_verilog_testbench(&cfg);
    assert!(tb.contains("module tb_test_core;"));
    assert!(tb.contains("test_core uut"));
    assert!(tb.contains("SILICON HARDWARE VERILOG TESTBENCH PASSED"));
    assert!(tb.contains("$finish;"));
}

#[test]
fn test_timing_and_openlane_constraints() {
    let mut cfg = SiliconSynthConfig::default();
    cfg.design_name = "sky130_core".to_string();
    cfg.target = SiliconTarget::AsicSky130;
    cfg.clock_freq_mhz = 500.0; // 2.0 ns period

    let sdc = generate_sdc_constraints(&cfg);
    assert!(sdc.contains("create_clock -name clk -period 2.000"));
    assert!(sdc.contains("set_input_delay"));
    assert!(sdc.contains("set_output_delay"));

    let xdc = generate_xdc_constraints(&cfg);
    assert!(xdc.contains("create_clock -period 2.000"));

    let openlane = generate_openlane_config(&cfg);
    assert!(openlane.contains("\"DESIGN_NAME\": \"sky130_core\""));
    assert!(openlane.contains("\"CLOCK_PERIOD\": 2.000"));
    assert!(openlane.contains("\"PDK\": \"sky130A\""));
}

#[test]
fn test_full_silicon_synth_package_export() {
    let mut cfg = SiliconSynthConfig::default();
    cfg.design_name = "cron_soc_accelerator".to_string();
    cfg.target = SiliconTarget::AsicSky130;
    cfg.clock_freq_mhz = 500.0;
    cfg.in_dim = 16;
    cfg.out_dim = 16;
    let out_dir = "target/test_silicon_tapeout_pkg";
    cfg.out_dir = out_dir.to_string();

    let manifest = generate_silicon_synth_package(&cfg).expect("Silicon package export must succeed");

    assert_eq!(manifest.design_name, "cron_soc_accelerator");
    assert_eq!(manifest.target_clock_mhz, 500.0);
    assert!(manifest.estimated_luts > 0);
    assert!(manifest.estimated_ffs > 0);
    assert!(manifest.estimated_die_area_um2 > 0.0);

    // Verify all generated files physically exist on disk
    assert!(Path::new(out_dir).join("cl_silicon_top.v").exists());
    assert!(Path::new(out_dir).join("tb_silicon_top.v").exists());
    assert!(Path::new(out_dir).join("timing_constraints.sdc").exists());
    assert!(Path::new(out_dir).join("timing.xdc").exists());
    assert!(Path::new(out_dir).join("config.json").exists());
    assert!(Path::new(out_dir).join("synth_vivado.tcl").exists());
    assert!(Path::new(out_dir).join("run_synth_yosys.sh").exists());
    assert!(Path::new(out_dir).join("manifest.json").exists());

    let manifest_bytes = fs::read_to_string(Path::new(out_dir).join("manifest.json"))
        .expect("Read manifest.json");
    assert!(manifest_bytes.contains("cron_soc_accelerator"));

    let _ = fs::remove_dir_all(out_dir);
}

