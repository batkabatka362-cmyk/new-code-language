// ============================================================================
// Integration Test: Hardware RTL Co-Simulation & VCD Waveform Trace
// Verifies:
// 1. Synthesizable Verilog top and testbench generation
// 2. Cycle-accurate RTL simulation of AXI4-Stream FSM
// 3. Input streaming & output word reception parity
// 4. GTKWave-compliant IEEE 1364-2001 VCD trace emission
// ============================================================================

use cronc::cl_silicon_tapeout::{
    generate_silicon_synth_package, simulate_silicon_and_dump_vcd,
    SiliconSynthConfig, SiliconTarget,
};
use std::fs;
use std::path::Path;

#[test]
fn test_silicon_hardware_cosim_and_vcd_trace() {
    let tmp_vcd = "target/test_waveform_trace.vcd";
    if Path::new(tmp_vcd).exists() {
        let _ = fs::remove_file(tmp_vcd);
    }

    let mut cfg = SiliconSynthConfig::default();
    cfg.design_name = "test_fsm_core".to_string();
    cfg.target = SiliconTarget::AsicSky130;
    cfg.clock_freq_mhz = 250.0;
    cfg.in_dim = 8;
    cfg.out_dim = 8;
    cfg.out_dir = "target/test_silicon_cosim_out".to_string();

    // 1. Verify package synthesis
    let manifest = generate_silicon_synth_package(&cfg).expect("Synthesis package generation must succeed");
    assert_eq!(manifest.design_name, "test_fsm_core");
    assert!(manifest.generated_files.iter().any(|f| f.ends_with("tb_silicon_top.v")));

    // 2. Execute Cycle-Accurate Co-Simulation
    let report = simulate_silicon_and_dump_vcd(&cfg, Some(tmp_vcd))
        .expect("Silicon co-simulation must succeed");

    assert!(report.total_cycles > 20, "Must simulate multiple clock cycles");
    assert_eq!(report.input_words_fed, 8, "Must feed all 8 input words");
    assert_eq!(report.output_words_received, 8, "Must receive all 8 output words");
    assert!(report.latency_cycles > 0, "Latency must be positive");

    // 3. Verify VCD File Structure
    assert!(Path::new(tmp_vcd).exists(), "VCD file must be written to disk");
    let vcd_str = fs::read_to_string(tmp_vcd).expect("VCD trace must be readable");
    assert!(vcd_str.contains("$version"), "VCD must have $version header");
    assert!(vcd_str.contains("$timescale"), "VCD must have $timescale");
    assert!(vcd_str.contains("$scope module tb_test_fsm_core"), "VCD must scope testbench");
    assert!(vcd_str.contains("$var wire 1 ! clk"), "VCD must define clk wire");
    assert!(vcd_str.contains("$dumpvars"), "VCD must have $dumpvars section");
    assert!(vcd_str.contains("$enddefinitions"), "VCD must close definitions");

    // Clean up
    let _ = fs::remove_file(tmp_vcd);
    let _ = fs::remove_dir_all("target/test_silicon_cosim_out");
}

