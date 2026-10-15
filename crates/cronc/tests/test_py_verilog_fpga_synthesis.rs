// ============================================================================
// CRON Python-to-Verilog FPGA Physical Synthesis Integration Tests
// Verifies:
// 1. Python math code lowers to synthesizable Verilog RTL with 0 DSP multipliers.
// 2. Hardware deployment package generation for Xilinx Artix-7 (Digilent Arty A7).
// 3. Hardware deployment package generation for Lattice iCE40 (1BitSquared iCEBreaker).
// 4. Physical pin constraints (XDC / PCF) and synthesis scripts (Vivado / Yosys).
// ============================================================================

use cronc::cl_py_transpile::PyToCronTranspiler;
use cronc::silicon_flash::{
    emit_hardware_package, generate_hardware_package, FlashConfig, FpgaBoard, HostInterface,
};
use std::fs;

#[test]
fn test_python_to_xilinx_arty_a7_fpga_package() {
    let py_code = r#"
def matrix_cortex_attention(q, k, v):
    attention = q @ k
    return attention @ v

def neural_lora_step(activations, weights):
    delta = activations @ weights
    return delta
"#;

    let mut transpiler = PyToCronTranspiler::new(64, true);
    transpiler.parse_and_lower(py_code).expect("Python lowering failed");
    let verilog_rtl = transpiler.emit_verilog_rtl();

    assert!(verilog_rtl.contains("module cron_core_matrix_cortex_attention"));
    assert!(verilog_rtl.contains("module cron_core_neural_lora_step"));
    assert!(verilog_rtl.contains("module cron_top"));
    assert!(verilog_rtl.contains("input  wire        clk,"));
    assert!(verilog_rtl.contains("0 DSP Multipliers"));

    let config = FlashConfig {
        board: Some(FpgaBoard::DigilentArtyA7),
        target: FpgaBoard::DigilentArtyA7.silicon_target(),
        core_clock_mhz: FpgaBoard::DigilentArtyA7.clock_mhz(),
        interface: HostInterface::PcieGen5x16,
        ..Default::default()
    };

    let pkg = generate_hardware_package(&verilog_rtl, "py_model", &config);

    assert_eq!(pkg.estimated_dsp, 0, "FPGA synthesis must use strictly 0 DSP slices");
    assert!(pkg.pin_constraints.contains("PACKAGE_PIN E3"), "Arty A7 clk pin must be E3");
    assert!(pkg.pin_constraints.contains("PACKAGE_PIN H5"), "Arty A7 led pin must be H5");
    assert_eq!(pkg.pin_constraints_name, "pins.xdc");
    assert!(pkg.vivado_tcl.contains("synth_design -top cron_top -part xc7a35tcsg324-1 -flatten_hierarchy rebuilt -max_dsp 0"));
    assert!(pkg.flash_device_bat.contains("openFPGALoader -b arty_a7_35t"));

    let temp_dir = std::env::temp_dir().join(format!("cron_test_arty_a7_{}", std::process::id()));
    emit_hardware_package(&pkg, &temp_dir).expect("Package emission failed");

    assert!(temp_dir.join("cron_top.v").exists());
    assert!(temp_dir.join("pins.xdc").exists());
    assert!(temp_dir.join("synth.tcl").exists());
    assert!(temp_dir.join("timing.xdc").exists());
    assert!(temp_dir.join("build_bitstream.bat").exists());
    assert!(temp_dir.join("flash_device.bat").exists());
    assert!(temp_dir.join("flash_manifest.json").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_python_to_lattice_ice40_icebreaker_package() {
    let py_code = r#"
def mcts_tree_traversal(current_node, branches):
    best_child = current_node
    return best_child

def atp_metabolic_kinetics(energy_state, atp_adp_ratio):
    free_energy = energy_state
    return free_energy
"#;

    let mut transpiler = PyToCronTranspiler::new(64, true);
    transpiler.parse_and_lower(py_code).expect("Python lowering failed");
    let verilog_rtl = transpiler.emit_verilog_rtl();

    assert!(verilog_rtl.contains("module cron_core_mcts_tree_traversal"));
    assert!(verilog_rtl.contains("module cron_core_atp_metabolic_kinetics"));
    assert!(verilog_rtl.contains("module cron_top"));

    let config = FlashConfig {
        board: Some(FpgaBoard::IceBreaker),
        target: FpgaBoard::IceBreaker.silicon_target(),
        core_clock_mhz: FpgaBoard::IceBreaker.clock_mhz(),
        interface: HostInterface::Uart115200,
        ..Default::default()
    };

    let pkg = generate_hardware_package(&verilog_rtl, "py_ice40", &config);

    assert_eq!(pkg.estimated_dsp, 0, "iCE40 synthesis must use strictly 0 DSP slices");
    assert!(pkg.pin_constraints.contains("set_io clk 35"), "iCEBreaker clk pin must be 35");
    assert!(pkg.pin_constraints.contains("set_io led_heartbeat 11"), "iCEBreaker led pin must be 11");
    assert_eq!(pkg.pin_constraints_name, "pins.pcf");
    assert!(pkg.yosys_script.contains("synth_ice40 -top cron_top -json cron_top.json"));
    assert!(pkg.flash_device_bat.contains("iceprog cron_top.bin"));

    let temp_dir = std::env::temp_dir().join(format!("cron_test_icebreaker_{}", std::process::id()));
    emit_hardware_package(&pkg, &temp_dir).expect("Package emission failed");

    assert!(temp_dir.join("cron_top.v").exists());
    assert!(temp_dir.join("pins.pcf").exists());
    assert!(temp_dir.join("synth_yosys.ys").exists());
    assert!(temp_dir.join("run_nextpnr.sh").exists());
    assert!(temp_dir.join("build_bitstream.bat").exists());
    assert!(temp_dir.join("flash_device.bat").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}
