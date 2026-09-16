// ============================================================================
// Integration Tests for CRON Hardware Synthesis Toolchain Scripts & Constraints
// ============================================================================

use std::fs;
use std::path::Path;

#[test]
fn test_synthesis_files_exist_and_valid() {
    let synth_dir = Path::new("../../synth");
    assert!(synth_dir.exists(), "synth directory must exist");

    // Check XDC constraints
    let xdc_path = synth_dir.join("cron_core_constraints.xdc");
    assert!(xdc_path.exists(), "XDC constraints file must exist");
    let xdc_content = fs::read_to_string(&xdc_path).unwrap();
    assert!(xdc_content.contains("create_clock"));
    assert!(xdc_content.contains("0.833")); // 1.2 GHz period
    assert!(xdc_content.contains("IOSTANDARD LVCMOS18"));

    // Check Vivado Tcl script
    let vivado_tcl = synth_dir.join("vivado_synth.tcl");
    assert!(vivado_tcl.exists(), "vivado_synth.tcl must exist");
    let vivado_content = fs::read_to_string(&vivado_tcl).unwrap();
    assert!(vivado_content.contains("synth_design"));
    assert!(vivado_content.contains("opt_design"));
    assert!(vivado_content.contains("place_design"));
    assert!(vivado_content.contains("route_design"));

    // Check Yosys Tcl script
    let yosys_tcl = synth_dir.join("yosys_synth.tcl");
    assert!(yosys_tcl.exists(), "yosys_synth.tcl must exist");
    let yosys_content = fs::read_to_string(&yosys_tcl).unwrap();
    assert!(yosys_content.contains("hierarchy"));
    assert!(yosys_content.contains("synth"));

    // Check Makefile
    let makefile = synth_dir.join("Makefile");
    assert!(makefile.exists(), "Makefile must exist");
    let makefile_content = fs::read_to_string(&makefile).unwrap();
    assert!(makefile_content.contains("vivado:"));
    assert!(makefile_content.contains("yosys:"));
}
