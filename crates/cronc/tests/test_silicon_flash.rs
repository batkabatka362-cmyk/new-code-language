use cronc::*;
use std::fs;
use std::process::Command;

#[test]
fn test_generate_hardware_package_alveo_u280() {
    let source = r#"
    .MODULE SiliconDeployment
    .ENTRY main

    def main() -> i64 {
        let x: i64 = 42;
        return x;
    }
    .END
    "#;

    let verilog_rtl = compile_to_verilog(source, "cron_top").expect("Failed to synthesize Verilog RTL");
    let config = FlashConfig {
        target: SiliconTarget::XilinxAlveoU280,
        interface: HostInterface::PcieGen5x16,
        ..Default::default()
    };

    let pkg = generate_hardware_package(&verilog_rtl, "SiliconDeployment", &config);

    // Verify Vivado Tcl script
    assert!(pkg.vivado_tcl.contains("xcu280-fsvh2892-2L-e"), "Expected U280 part in Vivado Tcl");
    assert!(pkg.vivado_tcl.contains("write_bitstream"), "Expected write_bitstream in Vivado Tcl");

    // Verify Timing Constraints XDC
    assert!(pkg.constraints_xdc.contains("clk_core"), "Expected clk_core clock definition");
    assert!(pkg.constraints_xdc.contains("clk_noc"), "Expected clk_noc clock definition");

    // Verify PCIe Gen5 DMA Host Driver C Code
    assert!(pkg.pcie_dma_header.contains("cron_dma_desc_t"), "Expected DMA descriptor in header");
    assert!(pkg.pcie_dma_source.contains("cron_pcie_open"), "Expected device open in source");

    // Test compiling the generated PCIe C driver with gcc
    let test_dir = std::env::temp_dir().join("cron_test_flash_pkg");
    emit_hardware_package(&pkg, &test_dir).expect("Failed to emit hardware package");

    assert!(test_dir.join("cron_top.v").exists());
    assert!(test_dir.join("synth.tcl").exists());
    assert!(test_dir.join("timing.xdc").exists());
    assert!(test_dir.join("cron_pcie_dma.h").exists());
    assert!(test_dir.join("cron_pcie_dma.c").exists());
    assert!(test_dir.join("flash_manifest.json").exists());

    let c_path = test_dir.join("cron_pcie_dma.c");
    let o_path = test_dir.join(if cfg!(windows) { "cron_pcie_dma.obj" } else { "cron_pcie_dma.o" });

    let comp_status = Command::new("gcc")
        .args(["-c", "-O3", c_path.to_str().unwrap(), "-o", o_path.to_str().unwrap()])
        .status();

    if let Ok(st) = comp_status {
        assert!(st.success(), "Generated PCIe DMA driver C code must compile cleanly with gcc");
        let _ = fs::remove_file(o_path);
    }

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_generate_hardware_package_custom_asic() {
    let source = r#"
    .MODULE AsicTapeout
    .ENTRY compute

    def compute() -> i64 {
        return 100;
    }
    .END
    "#;

    let verilog_rtl = compile_to_verilog(source, "cron_top").expect("Failed to synthesize Verilog RTL");
    let config = FlashConfig {
        target: SiliconTarget::CustomAsicN5,
        core_clock_mhz: 1500, // 1.5 GHz ASIC core
        noc_clock_mhz: 800,   // 800 MHz ASIC NoC
        ..Default::default()
    };

    let pkg = generate_hardware_package(&verilog_rtl, "AsicTapeout", &config);

    assert!(pkg.manifest_json.contains("TSMC N5"), "Expected TSMC N5 in ASIC manifest");
    assert!(pkg.constraints_xdc.contains("1500"), "Expected 1500 MHz core clock in XDC constraints");
}
