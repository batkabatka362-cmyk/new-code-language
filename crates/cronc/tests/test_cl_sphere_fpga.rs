// ============================================================================
// CRON 4D Sphere LLM FPGA RTL & Timing/Power Integration Tests
// Test Suite: test_cl_sphere_fpga (crates/cronc/tests/test_cl_sphere_fpga.rs)
// ============================================================================

use std::path::Path;
use cronc::cl_sphere_fpga::{
    emulate_sphere_transformer_fpga, export_sphere_transformer_fpga, SphereFpgaConfig,
    SphereFpgaTarget,
};
use cronc::cl_sphere_infer::{SphereTransformerConfig, SphereTransformerModel};

#[test]
fn test_sphere_fpga_package_generation_from_safetensors() {
    let fixture_path = Path::new("tests/fixtures/pytorch_transformer_layer.safetensors");
    let actual_path = if fixture_path.exists() {
        fixture_path
    } else {
        Path::new("../../tests/fixtures/pytorch_transformer_layer.safetensors")
    };

    let model = if actual_path.exists() {
        SphereTransformerModel::from_safetensors_file(actual_path).expect("Load safetensors fixture")
    } else {
        let config = SphereTransformerConfig {
            vocab_size: 256,
            hidden_dim: 32,
            num_layers: 1,
            num_heads: 4,
            head_dim: 8,
            intermediate_dim: 64,
            max_seq_len: 64,
        };
        SphereTransformerModel::new_synthetic(config)
    };

    let config = SphereFpgaConfig {
        target: SphereFpgaTarget::XilinxAlveoU280,
        clock_freq_mhz: 500.0,
        voltage_v: 0.85,
        temperature_c: 25.0,
        module_name: "sphere_transformer_llm_top".to_string(),
        enable_vcd_dump: true,
        max_sim_cycles: 20000,
    };

    let pkg = export_sphere_transformer_fpga(&model, "CRON", 4, &config);

    // 1. Validate top Verilog RTL
    assert!(pkg.top_verilog.contains("module sphere_transformer_llm_top"));
    assert!(pkg.top_verilog.contains("assign dsp_multipliers_used = 32'd0;"));
    assert!(pkg.top_verilog.contains("(* use_dsp = \"no\" *)"));
    assert!(pkg.top_verilog.contains("STATE_SWIGLU_GATE"));
    assert!(pkg.top_verilog.contains("candidate_token"));

    // 2. Validate self-checking testbench
    assert!(pkg.testbench_verilog.contains("module tb_sphere_transformer_llm_top"));
    assert!(pkg.testbench_verilog.contains("dsp_multipliers_used != 32'd0"));
    assert!(pkg.testbench_verilog.contains("$finish(0)"));

    // 3. Validate synthesis and constraints scripts
    assert!(pkg.vivado_tcl.contains("maxDSPMults 0"));
    assert!(pkg.vivado_tcl.contains("xcu280-fsvh2892-2L-e"));
    assert!(pkg.timing_xdc.contains("create_clock -period 2.000"));
    assert!(pkg.quartus_tcl.contains("DSP_BLOCK_BALANCING \"OFF\""));
    assert!(pkg.timing_sdc.contains("create_clock -period 2.000"));
    assert!(pkg.yosys_ys.contains("hierarchy -check -top sphere_transformer_llm_top"));

    // 4. Test directory export
    let temp_dir = std::env::temp_dir().join(format!("cron_fpga_test_{}", std::process::id()));
    let write_res = pkg.write_to_directory(&temp_dir);
    assert!(write_res.is_ok());

    assert!(temp_dir.join("sphere_transformer_llm_top.v").exists());
    assert!(temp_dir.join("tb_sphere_transformer_llm_top.v").exists());
    assert!(temp_dir.join("weights.hex").exists());
    assert!(temp_dir.join("synth_vivado.tcl").exists());
    assert!(temp_dir.join("timing.xdc").exists());
    assert!(temp_dir.join("fpga_manifest.json").exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_sphere_fpga_emulation_metrics() {
    let config = SphereTransformerConfig {
        vocab_size: 256,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 4,
        head_dim: 8,
        intermediate_dim: 64,
        max_seq_len: 64,
    };
    let model = SphereTransformerModel::new_synthetic(config);

    let fpga_config = SphereFpgaConfig {
        target: SphereFpgaTarget::XilinxAlveoU280,
        clock_freq_mhz: 500.0,
        voltage_v: 0.85,
        temperature_c: 25.0,
        module_name: "sphere_transformer_llm_top".to_string(),
        enable_vcd_dump: true,
        max_sim_cycles: 20000,
    };

    let report = emulate_sphere_transformer_fpga(&model, "CRON", 8, &fpga_config);

    assert_eq!(report.tokens_generated, 8);
    assert_eq!(report.dsp_multipliers_used, 0);
    assert!(report.total_cycles > 0);
    assert!(report.latency_ms > 0.0);
    assert!(report.tokens_per_sec > 0.0);
    assert!(report.total_geodesic_ops > 0);
    assert!(report.equivalent_gops > 0.0);
    assert!(report.estimated_luts > 0);
    assert!(report.dynamic_power_mw > 0.0);
    assert!(report.total_power_mw > 0.0);
    assert!(report.energy_per_token_nj > 0.0);
    assert!(report.landauer_limit_energy_joules > 0.0);
    // CRON 0-DSP architecture must show significant energy efficiency gain over 700W FP32 GPU
    assert!(report.energy_efficiency_multiplier_vs_gpu > 1.0);

    let json = report.to_json();
    assert!(json.contains("\"target_device\""));
    assert!(json.contains("\"dsp_multipliers_used\": 0"));

    let ascii = report.format_ascii_report();
    assert!(ascii.contains("Zero Multipliers Guaranteed"));
    assert!(ascii.contains("Landauer Thermodynamic E:"));
}

#[test]
fn test_sphere_fpga_different_silicon_targets() {
    let config = SphereTransformerConfig {
        vocab_size: 256,
        hidden_dim: 16,
        num_layers: 1,
        num_heads: 4,
        head_dim: 4,
        intermediate_dim: 32,
        max_seq_len: 32,
    };
    let model = SphereTransformerModel::new_synthetic(config);

    let targets = [
        SphereFpgaTarget::XilinxAlveoU280,
        SphereFpgaTarget::XilinxUltraScalePlus,
        SphereFpgaTarget::IntelAgilex7,
        SphereFpgaTarget::SkyWater130Asic,
    ];

    for target in targets {
        let fpga_config = SphereFpgaConfig {
            target,
            clock_freq_mhz: 250.0,
            voltage_v: 0.80,
            temperature_c: 30.0,
            module_name: "test_target_core".to_string(),
            enable_vcd_dump: false,
            max_sim_cycles: 5000,
        };

        let pkg = export_sphere_transformer_fpga(&model, "HI", 2, &fpga_config);
        assert!(pkg.top_verilog.contains(target.name()) || pkg.top_verilog.contains(target.part_string()));

        let report = emulate_sphere_transformer_fpga(&model, "HI", 2, &fpga_config);
        assert_eq!(report.dsp_multipliers_used, 0);
        assert_eq!(report.clock_freq_mhz, 250.0);
    }
}
