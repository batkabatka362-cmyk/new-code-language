// ============================================================================
// CRON Landauer Thermodynamic DVFS & Thermal Simulator Test Suite
// ============================================================================

use cronc::cl_power::{analyze_cl_power, ClPowerOptions};

#[test]
fn test_landauer_zero_dissipation_on_reversible_kernels() {
    // Program containing only Reversible Fredkin gates (_FA) and Photonic MZIs (_OP)
    let rev_cl = r#"
    B0000: _OP01$28F> _OP02$38F> _NO00#000> _NO00#000>
    B0001: _FA03$142> _FA04$242> _NO00#000> _NO00#000>
    B0002: _8800$000> _NO00#000> _NO00#000> _HL00$008!
    "#;

    let opts = ClPowerOptions::default();
    let rep = analyze_cl_power(rev_cl, &opts).expect("Power analysis must succeed");

    assert_eq!(rep.total_bits_erased, 0, "Reversible + Photonic microcode must erase exactly 0 bits");
    assert_eq!(rep.landauer_energy_joules, 0.0, "Delta S must be exactly 0 (0 Joules dissipated)");
    assert!(rep.is_landauer_optimal, "Must be classified as Landauer optimal");
    assert!(rep.reversible_zero_entropy_ops >= 2);
    assert!(rep.photonic_mzi_ops >= 2);
}

#[test]
fn test_irreversible_cmos_landauer_entropy() {
    let cmos_cl = r#"
    B0000: '==01#000> '==02#004> _PO03+100> _PO04+200>
    B0001: _ST01#000> _ST02#004> _PO05+300> _HL00$008!
    "#;

    let opts = ClPowerOptions::default();
    let rep = analyze_cl_power(cmos_cl, &opts).expect("Power analysis must succeed");

    assert!(rep.total_bits_erased > 0, "Irreversible writes must erase bits");
    assert!(rep.landauer_energy_joules > 0.0, "Landauer energy must be > 0");
    assert!(rep.landauer_power_microwatts > 0.0);
    assert!(rep.dynamic_power_watts > 0.0);
    assert!(rep.junction_temperature_c >= opts.ambient_temp_c);
}

#[test]
fn test_dvfs_voltage_and_frequency_scaling() {
    let cl_code = r#"
    B0000: '==01#000> _MD02*100> _PO03+100> _NO00#000>
    B0001: _TT04$200> _PO05+300> _NO00#000> _HL00$008!
    "#;

    // 1. Nominal: 2.0 GHz, 0.85 V
    let opts_nominal = ClPowerOptions {
        frequency_ghz: 2.0,
        voltage_v: 0.85,
        ..Default::default()
    };
    let rep_nominal = analyze_cl_power(cl_code, &opts_nominal).unwrap();

    // 2. Low-power DVFS state: 1.0 GHz, 0.70 V
    let opts_low = ClPowerOptions {
        frequency_ghz: 1.0,
        voltage_v: 0.70,
        ..Default::default()
    };
    let rep_low = analyze_cl_power(cl_code, &opts_low).unwrap();

    // Dynamic power scales with f * V^2:
    // Ratio should be ~ (1.0 / 2.0) * (0.70 / 0.85)^2 ≈ 0.5 * 0.678 ≈ 0.339
    assert!(rep_low.dynamic_power_watts < rep_nominal.dynamic_power_watts);
    assert!(rep_low.total_power_watts < rep_nominal.total_power_watts);
    assert!(rep_low.junction_temperature_c < rep_nominal.junction_temperature_c);
}

#[test]
fn test_thermal_throttling_detection() {
    let cl_code = r#"
    B0000: '==01#000> _PO02+100> _PO03+200> _PO04+300>
    B0001: _ST01#000> _ST02#004> _ST03#008> _HL00$008!
    "#;

    // Set ambient temperature to 85°C so Tj exceeds 85°C throttle threshold
    let opts = ClPowerOptions {
        ambient_temp_c: 85.0,
        frequency_ghz: 2.5,
        voltage_v: 0.95,
        ..Default::default()
    };
    let rep = analyze_cl_power(cl_code, &opts).unwrap();

    assert!(rep.junction_temperature_c >= 85.0);
    assert!(rep.is_thermal_throttling, "High thermal regime must trigger thermal throttling");
}

#[test]
fn test_power_ascii_and_json_reports() {
    let cl_code = r#"
    B0000: _OP01$28F> _FA02$142> _PO03+100> _HL00$008!
    "#;

    let opts = ClPowerOptions::default();
    let rep = analyze_cl_power(cl_code, &opts).unwrap();

    let ascii = rep.render_ascii_report("test_kernel.cl");
    assert!(ascii.contains("CRON LANDAUER THERMODYNAMIC DVFS"));
    assert!(ascii.contains("Reversible Ops (Fredkin/Toffoli)"));
    assert!(ascii.contains("Landauer Energy Dissipation"));

    let json = rep.to_json();
    assert!(json.contains("\"total_cycles\": 1"));
    assert!(json.contains("\"reversible_zero_entropy_ops\": 1"));
    assert!(json.contains("\"photonic_mzi_ops\": 1"));
}
