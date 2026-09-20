// ============================================================================
// CRON Optical WDM Laser Power Budget & Photonic Insertion Loss Optimizer Tests
// ============================================================================

use cronc::cl_optic::{
    analyze_cl_optic, format_optic_ascii_hud, optic_report_to_json,
    synthesize_optical_power_verilog, ClOpticOptions, MeshTopology,
};

#[test]
fn test_optic_insertion_loss_clemens_vs_reck() {
    let cl_code = "
        B0000: _OP01$28F> _NO00#000> _NO00#000> _NO00#000>
        B0001: _WD00#100> _NO00#000> _NO00#000> _NO00#000>
        B0002: _NO00#000> _NO00#000> _NO00#000> _HL00#000!
    ";

    // 1. Clemens Topology (Balanced Depth D = N)
    let clemens_opt = ClOpticOptions {
        mesh_dim: 16,
        wdm_channels: 8,
        waveguide_length_cm: 2.0,
        topology: MeshTopology::Clemens,
        base_wavelength_nm: 1550.0,
        laser_wall_plug_eff: 0.20,
    };
    let clemens_report = analyze_cl_optic(cl_code, &clemens_opt).expect("Clemens analysis should succeed");
    assert_eq!(clemens_report.optical_depth, 16);
    assert_eq!(clemens_report.topology, MeshTopology::Clemens);
    assert!(clemens_report.insertion_loss_db > 10.0);
    assert!(clemens_report.is_loss_budget_compliant);

    // 2. Reck Topology (Triangular Depth D = 2N - 3 = 32 - 3 = 29)
    let reck_opt = ClOpticOptions {
        mesh_dim: 16,
        wdm_channels: 8,
        waveguide_length_cm: 2.0,
        topology: MeshTopology::Reck,
        base_wavelength_nm: 1550.0,
        laser_wall_plug_eff: 0.20,
    };
    let reck_report = analyze_cl_optic(cl_code, &reck_opt).expect("Reck analysis should succeed");
    assert_eq!(reck_report.optical_depth, 29);
    assert_eq!(reck_report.topology, MeshTopology::Reck);
    // Reck has significantly higher insertion loss than Clemens due to deeper triangular mesh
    assert!(reck_report.insertion_loss_db > clemens_report.insertion_loss_db);
}

#[test]
fn test_optic_wdm_16_channel_allocation() {
    let cl_code = "
        B0000: _WD00#100> _WD01#100> _WD02#100> _WD03#100>
        B0001: _WD04#100> _WD05#100> _WD06#100> _WD07#100>
        B0002: _OP01$28F> _OP02$28F> _NO00#000> _HL00#000!
    ";

    let opt = ClOpticOptions {
        mesh_dim: 16,
        wdm_channels: 16,
        waveguide_length_cm: 3.0,
        topology: MeshTopology::Clemens,
        base_wavelength_nm: 1530.0,
        laser_wall_plug_eff: 0.22,
    };
    let report = analyze_cl_optic(cl_code, &opt).expect("Analysis should succeed");
    assert_eq!(report.wdm_channels.len(), 16);

    // Verify 100 GHz ITU grid channel spacing (0.8 nm steps)
    for (i, ch) in report.wdm_channels.iter().enumerate() {
        let expected_lambda = 1530.0 + (i as f64 * 0.8);
        assert!((ch.wavelength_nm - expected_lambda).abs() < 1e-4);
        assert!(ch.frequency_thz > 190.0 && ch.frequency_thz < 200.0);
        assert!(ch.snr_db > 35.0, "SNR must be above 35 dB for reliable optical matrix multiplication");
    }
}

#[test]
fn test_optic_laser_power_budget_and_dynamic_gating() {
    let mut cl_code = String::new();
    // 20 cycles total, but only 2 cycles have optical operations
    for cycle in 0..20 {
        if cycle == 5 || cycle == 10 {
            cl_code.push_str(&format!("B{:04}: _OP01$28F> _1100#000> _NO00#000> _NO00#000>\n", cycle));
        } else {
            cl_code.push_str(&format!("B{:04}: _NO00#000> _NO00#000> _NO00#000> _NO00#000>\n", cycle));
        }
    }
    cl_code.push_str("B0020: _HL00#000! _NO00#000> _NO00#000> _NO00#000>\n");

    let opt = ClOpticOptions::default();
    let report = analyze_cl_optic(&cl_code, &opt).expect("Analysis should succeed");

    assert!(report.total_cycles >= 20);
    assert!(report.optical_ops_count >= 2);
    assert!(report.laser_pump_strobes >= 2);

    // Dynamic laser gating should deliver massive energy savings over Continuous Wave (CW) laser
    assert!(report.energy_saved_pct > 70.0, "Expected >70% energy savings with sparse laser gating, got {:.2}%", report.energy_saved_pct);
    assert!(report.dynamic_gated_energy_uj < report.continuous_wave_energy_uj);
}

#[test]
fn test_optic_ascii_hud_and_json_serialization() {
    let cl_code = "
        B0000: _OP01$28F> _WD01#100> _NO00#000> _NO00#000>
        B0001: _NO00#000> _NO00#000> _NO00#000> _HL00#000!
    ";
    let opt = ClOpticOptions::default();
    let report = analyze_cl_optic(cl_code, &opt).expect("Analysis should succeed");

    // Check ASCII HUD
    let hud = format_optic_ascii_hud(&report);
    assert!(hud.contains("CRON HETEROGENEOUS OPTICAL WDM LASER POWER"));
    assert!(hud.contains("PHOTONIC COMPONENT INSERTION LOSS WATERFALL"));
    assert!(hud.contains("WDM C-BAND 100 GHz LAMBDA CHANNELS ALLOCATION"));

    // Check JSON
    let json_str = optic_report_to_json(&report);
    assert!(json_str.contains("\"status\": \"COMPLIANT\""));
    assert!(json_str.contains("\"insertion_loss_db\":"));
    assert!(json_str.contains("\"loss_breakdown\":"));
    assert!(json_str.contains("\"wdm_channels\":"));
}

#[test]
fn test_optic_verilog_rtl_synthesis() {
    let cl_code = "
        B0000: _OP01$28F> _NO00#000> _NO00#000> _NO00#000>
        B0001: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
    ";
    let opt = ClOpticOptions::default();
    let report = analyze_cl_optic(cl_code, &opt).expect("Analysis should succeed");

    let verilog = synthesize_optical_power_verilog(&report);
    assert!(verilog.contains("module optical_laser_power_controller"));
    assert!(verilog.contains("laser_diode_enable"));
    assert!(verilog.contains("pd_feedback_current_ua"));
    assert!(verilog.contains("active_lambda_sel"));
}
