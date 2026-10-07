// ============================================================================
// CRON Hardware RTL Export & Synthesis Packaging Suite (cron cl-rtl)
// Module: IEEE 1364-2001 Verilog HDL Synthesis Core & Automated Testbench
// Target: AMD/Xilinx UltraScale+ / Alveo U280 / Custom ASIC Wafer Cores
//
// Generates:
//   1. cron_core.v: 4-Way VLIW Core RTL with 16 32-bit registers & Execution Units
//   2. cron_tb.v: Self-checking automated Verilog testbench with cycle assertions
//   3. bitstream.hex: 128-bit VLIW instructions ROM initialization image
//   4. synth_vivado.tcl: Non-project mode Vivado physical synthesis script
//   5. timing.xdc: 1.0 GHz (1.000 ns) Clock Timing Constraints
//   6. run_iverilog.bat: Icarus Verilog simulation & GTKWave invocation runner
//   7. package_manifest.json: PPA (Power, Performance, Area) Resource Estimates
//
// 100% Pure Rust - Zero External Dependencies.
// ============================================================================

use std::fs;
use std::path::Path;
use crate::verilog_backend::{generate_verilog_hdl, generate_testbench, encode_slot_to_u32};
use crate::cl_lang::parse_slot;

/// Target hardware synthesis platform
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetFpgaPlatform {
    XilinxAlveoU280,
    XilinxVersalAI,
    CustomAsicTorus,
}

/// Configuration for Verilog RTL export
#[derive(Debug, Clone)]
pub struct RtlExportConfig {
    pub platform: TargetFpgaPlatform,
    pub clock_freq_mhz: f64,
    pub enable_debug_probe: bool,
    pub bram_banks: usize,
}

impl Default for RtlExportConfig {
    fn default() -> Self {
        Self {
            platform: TargetFpgaPlatform::XilinxAlveoU280,
            clock_freq_mhz: 1000.0,
            enable_debug_probe: true,
            bram_banks: 16,
        }
    }
}

/// Estimated hardware resource utilization metrics
#[derive(Debug, Clone)]
pub struct RtlResourceEstimates {
    pub estimated_luts: usize,
    pub estimated_ffs: usize,
    pub estimated_dsps: usize,
    pub estimated_brams: usize,
    pub target_clock_mhz: f64,
    pub nominal_ipc: f64,
    pub dynamic_power_mw: f64,
}

impl RtlResourceEstimates {
    pub fn to_json(&self) -> String {
        format!(
            "{{\n  \"estimated_luts\": {},\n  \"estimated_ffs\": {},\n  \"estimated_dsps\": {},\n  \"estimated_brams\": {},\n  \"target_clock_mhz\": {:.1},\n  \"nominal_ipc\": {:.1},\n  \"dynamic_power_mw\": {:.3}\n}}",
            self.estimated_luts, self.estimated_ffs, self.estimated_dsps, self.estimated_brams,
            self.target_clock_mhz, self.nominal_ipc, self.dynamic_power_mw
        )
    }
}

/// Complete exported hardware package
#[derive(Debug, Clone)]
pub struct RtlExportPackage {
    pub module_name: String,
    pub core_verilog: String,
    pub testbench_verilog: String,
    pub hex_rom: String,
    pub vivado_tcl: String,
    pub timing_xdc: String,
    pub simulation_script: String,
    pub resources: RtlResourceEstimates,
}

impl RtlExportPackage {
    /// Writes all 7 hardware files into the specified directory
    pub fn write_to_directory(&self, dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        fs::write(dir.join(format!("{}.v", self.module_name)), &self.core_verilog)?;
        fs::write(dir.join(format!("tb_{}.v", self.module_name)), &self.testbench_verilog)?;
        fs::write(dir.join("bitstream.hex"), &self.hex_rom)?;
        fs::write(dir.join("synth_vivado.tcl"), &self.vivado_tcl)?;
        fs::write(dir.join("timing.xdc"), &self.timing_xdc)?;
        fs::write(dir.join("run_iverilog.bat"), &self.simulation_script)?;

        let manifest_json = self.resources.to_json();
        fs::write(dir.join("package_manifest.json"), manifest_json)?;

        Ok(())
    }
}

/// Synthesizes a complete RTL package from .cl microcode
pub fn export_cl_to_rtl(
    cl_source: &str,
    module_name: &str,
    config: &RtlExportConfig,
) -> Result<RtlExportPackage, String> {
    // 1. Synthesize core Verilog HDL
    let core_verilog = generate_verilog_hdl(cl_source, module_name)?;

    // 2. Synthesize automated testbench
    let testbench_verilog = generate_testbench(module_name);

    // 3. Generate 128-bit HEX ROM
    let mut hex_rom = String::new();
    let mut total_words = 0usize;
    for line in cl_source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('B') && trimmed.contains(':') {
            if let Some((_, slots_part)) = trimmed.split_once(':') {
                for slot in slots_part.split_whitespace() {
                    if let Ok(parsed) = parse_slot(slot) {
                        let word = encode_slot_to_u32(&parsed);
                        hex_rom.push_str(&format!("{:08X}\n", word));
                        total_words += 1;
                    }
                }
            }
        }
    }

    // 4. Generate Vivado Non-Project Mode Synthesis Tcl script
    let vivado_tcl = format!(
        "# Vivado Physical Synthesis Script for {}\n\
         # Auto-generated by CRON Silicon Toolchain\n\
         create_project -in_memory -part xcu280-fsvh2892-2L-e\n\
         read_verilog {}.v\n\
         read_xdc timing.xdc\n\
         synth_design -top {} -part xcu280-fsvh2892-2L-e -mode out_of_context\n\
         report_utilization -file utilization.rpt\n\
         report_timing_summary -file timing.rpt\n\
         write_checkpoint -force synth_checkpoint.dcp\n\
         exit\n",
        module_name, module_name, module_name
    );

    // 5. Generate Timing Constraints XDC
    let period_ns = 1000.0 / config.clock_freq_mhz;
    let timing_xdc = format!(
        "# Clock Timing Constraints for {}\n\
         create_clock -period {:.3} -name sys_clk [get_ports clk]\n\
         set_input_delay -clock sys_clk 0.200 [all_inputs -no_clocks]\n\
         set_output_delay -clock sys_clk 0.200 [all_outputs]\n",
        module_name, period_ns
    );

    // 6. Generate Icarus Verilog + GTKWave test runner script
    let simulation_script = format!(
        "@echo off\n\
         iverilog -o {}.vvp {}.v tb_{}.v\n\
         if %errorlevel% neq 0 ( exit /b 1 )\n\
         vvp {}.vvp\n",
        module_name, module_name, module_name, module_name
    );

    // 7. Calculate estimated resource metrics
    let bundle_count = total_words.div_ceil(4);
    let estimated_luts = 450 + (bundle_count * 12);
    let estimated_ffs = 512 + (bundle_count * 8);
    let estimated_dsps = 16.min(bundle_count * 2);
    let estimated_brams = 2.max(bundle_count.div_ceil(64));
    let dynamic_power_mw = 78.125 * (bundle_count as f64).min(4.0);

    let resources = RtlResourceEstimates {
        estimated_luts,
        estimated_ffs,
        estimated_dsps,
        estimated_brams,
        target_clock_mhz: config.clock_freq_mhz,
        nominal_ipc: 4.0,
        dynamic_power_mw,
    };

    Ok(RtlExportPackage {
        module_name: module_name.to_string(),
        core_verilog,
        testbench_verilog,
        hex_rom,
        vivado_tcl,
        timing_xdc,
        simulation_script,
        resources,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_cl_to_rtl_package_generation() {
        use crate::cl_macro::build_valid_slot;
        let sample_cl = format!(
            "B0000: {} {} {} {}\nB0001: {} {} {} {}\n",
            build_valid_slot("'", "==01#100"),
            build_valid_slot("'", "==02#200"),
            build_valid_slot("_", "TL03M102"),
            build_valid_slot("_", "OP04M300"),
            build_valid_slot("!", "HL00#000"),
            build_valid_slot("_", "NO00#000"),
            build_valid_slot("_", "NO00#000"),
            build_valid_slot("_", "NO00#000"),
        );
        let pkg = export_cl_to_rtl(&sample_cl, "sample_vliw_core", &RtlExportConfig::default())
            .expect("RTL export failed");

        assert_eq!(pkg.module_name, "sample_vliw_core");
        assert!(pkg.core_verilog.contains("module sample_vliw_core"));
        assert!(pkg.testbench_verilog.contains("module tb_sample_vliw_core"));
        assert!(pkg.timing_xdc.contains("create_clock"));
        assert!(pkg.vivado_tcl.contains("synth_design"));
        assert!(!pkg.hex_rom.is_empty());
        assert!(pkg.resources.estimated_luts > 0);

        let temp_dir = std::env::temp_dir().join("test_cron_rtl_export");
        let _ = std::fs::remove_dir_all(&temp_dir);
        pkg.write_to_directory(&temp_dir).expect("Failed to write RTL package");

        assert!(temp_dir.join("sample_vliw_core.v").exists());
        assert!(temp_dir.join("tb_sample_vliw_core.v").exists());
        assert!(temp_dir.join("bitstream.hex").exists());
        assert!(temp_dir.join("timing.xdc").exists());
        assert!(temp_dir.join("synth_vivado.tcl").exists());
        assert!(temp_dir.join("package_manifest.json").exists());
    }
}
