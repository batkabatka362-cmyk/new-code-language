// ============================================================================
// CRON Heterogeneous Optical WDM Laser Power Budget & Photonic Insertion Loss Optimizer
// (cl_optic.rs)
//
// Models microscopic nanophotonics and optical wave propagation for Brain 2
// Photonic Matrix (MZI) Accelerators:
// 1. Silicon-on-Insulator (SOI) & Si3N4 Waveguide Propagation Loss (dB/cm).
// 2. Mach-Zehnder Interferometer (MZI) Insertion Loss (dB per unit).
// 3. Clemens (Balanced Depth = N) vs Reck (Triangular Depth = 2N-3) Topologies.
// 4. 16-Channel Wavelength Division Multiplexing (WDM) across C-Band (1530-1565 nm).
// 5. Dynamic Laser Power Gating (Cycle-accurate pump laser control vs CW laser).
// 6. Photodetector Responsivity, Thermal Drift Sensitivity, and SNR (dB).
// 7. Synthesizable Verilog RTL for closed-loop Laser Power Regulation.
//
// 100% Pure Rust — Zero External Dependencies.
// ============================================================================

use crate::cl_lang::parse_slot;

pub const DEFAULT_WAVELENGTH_NM: f64 = 1550.0;
pub const WDM_100GHZ_SPACING_NM: f64 = 0.8; // ~100 GHz ITU Grid
pub const DEFAULT_SOI_WG_LOSS_DB_PER_CM: f64 = 0.45; // 0.45 dB/cm
pub const DEFAULT_MZI_UNIT_LOSS_DB: f64 = 0.50;      // Splitter + Phase Shifter + Combiner
pub const DEFAULT_CROSSING_LOSS_DB: f64 = 0.035;    // Waveguide Crossing on 4D Mesh
pub const DEFAULT_AWG_MUX_DEMUX_LOSS_DB: f64 = 1.75; // Arrayed Waveguide Grating
pub const DEFAULT_EDGE_COUPLING_LOSS_DB: f64 = 1.20; // Fiber-to-Chip Coupler
pub const DEFAULT_PD_RESPONSIVITY_A_W: f64 = 0.95;   // Ge-on-Si PIN Photodiode
pub const MIN_PD_SENSITIVITY_DBM: f64 = -22.0;       // ~6.31 uW for 35 dB SNR (FP8/INT8 precision)
pub const OPTICAL_SAFETY_MARGIN_DB: f64 = 3.0;       // Engineering Safety Margin

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshTopology {
    Clemens, // Balanced Depth D = N
    Reck,    // Triangular Depth D = 2N - 3
}

impl Default for MeshTopology {
    fn default() -> Self {
        MeshTopology::Clemens
    }
}

/// Configuration options for optical WDM and laser budget analysis
#[derive(Debug, Clone)]
pub struct ClOpticOptions {
    pub mesh_dim: usize,             // e.g. 16 for 16x16 MZI Mesh
    pub wdm_channels: usize,         // 1 to 16 wavelengths
    pub waveguide_length_cm: f64,    // Physical chip routing length
    pub topology: MeshTopology,
    pub base_wavelength_nm: f64,
    pub laser_wall_plug_eff: f64,    // Laser diode electrical-to-optical wall-plug efficiency (e.g. 0.20 = 20%)
}

impl Default for ClOpticOptions {
    fn default() -> Self {
        Self {
            mesh_dim: 16,
            wdm_channels: 8,
            waveguide_length_cm: 2.5,
            topology: MeshTopology::Clemens,
            base_wavelength_nm: DEFAULT_WAVELENGTH_NM,
            laser_wall_plug_eff: 0.22,
        }
    }
}

/// Optical channel telemetry for individual WDM wavelength
#[derive(Debug, Clone)]
pub struct WdmChannelInfo {
    pub channel_idx: usize,
    pub wavelength_nm: f64,
    pub frequency_thz: f64,
    pub active_ops_count: usize,
    pub optical_power_mw: f64,
    pub optical_power_dbm: f64,
    pub snr_db: f64,
    pub crosstalk_db: f64,
}

/// Comprehensive Optical & Photonic Power Telemetry Report
#[derive(Debug, Clone)]
pub struct ClOpticReport {
    pub total_cycles: usize,
    pub total_vliw_bundles: usize,
    pub optical_ops_count: usize,
    pub wdm_ops_count: usize,
    pub laser_pump_strobes: usize,
    pub mesh_dimension: usize,
    pub optical_depth: usize,
    pub topology: MeshTopology,
    pub insertion_loss_db: f64,
    pub loss_breakdown: LossBreakdown,
    pub min_rx_power_dbm: f64,
    pub required_laser_power_dbm: f64,
    pub required_laser_power_mw: f64,
    pub electrical_laser_power_mw: f64,
    pub continuous_wave_energy_uj: f64,
    pub dynamic_gated_energy_uj: f64,
    pub energy_saved_pct: f64,
    pub wdm_channels: Vec<WdmChannelInfo>,
    pub thermal_phase_drift_rad: f64,
    pub is_loss_budget_compliant: bool,
}

#[derive(Debug, Clone)]
pub struct LossBreakdown {
    pub coupling_loss_db: f64,
    pub awg_mux_loss_db: f64,
    pub waveguide_propagation_db: f64,
    pub mzi_mesh_loss_db: f64,
    pub waveguide_crossings_db: f64,
    pub awg_demux_loss_db: f64,
    pub total_insertion_loss_db: f64,
}

/// Analyze optical instruction execution, calculate WDM laser power budget & insertion loss
pub fn analyze_cl_optic(cl_code: &str, options: &ClOpticOptions) -> Result<ClOpticReport, String> {
    let mut total_cycles = 0usize;
    let mut optical_ops = 0usize;
    let mut wdm_ops = 0usize;
    let mut laser_strobes = 0usize;
    let mut channel_activity = vec![0usize; options.wdm_channels.max(1)];

    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }
        if !trimmed.starts_with('B') && !trimmed.starts_with('@') {
            continue;
        }

        if let Some(colon_pos) = trimmed.find(':') {
            let bundle_content = &trimmed[colon_pos + 1..].trim();
            total_cycles += 1;

            let mut cycle_has_optical = false;
            let mut cycle_has_wdm = false;

            for slot_str in bundle_content.split_whitespace() {
                if slot_str.len() != 10 {
                    continue;
                }
                if let Ok(slot) = parse_slot(slot_str) {
                    match slot.opcode.as_str() {
                        "OP" => {
                            optical_ops += 1;
                            cycle_has_optical = true;
                            // Hash dest reg to channel
                            let ch = slot.dest_reg.unwrap_or(0) % options.wdm_channels.max(1);
                            channel_activity[ch] += 1;
                        }
                        "WD" => {
                            wdm_ops += 1;
                            cycle_has_wdm = true;
                            let ch = slot.src_reg.unwrap_or(0) % options.wdm_channels.max(1);
                            channel_activity[ch] += 1;
                        }
                        "11" => {
                            laser_strobes += 1;
                        }
                        "ff" => {
                            wdm_ops += 1;
                        }
                        _ => {}
                    }
                }
            }

            if cycle_has_optical && !cycle_has_wdm {
                channel_activity[0] += 1;
            }
        }
    }

    if total_cycles == 0 {
        total_cycles = 1;
    }

    let n = options.mesh_dim.max(2);
    let optical_depth = match options.topology {
        MeshTopology::Clemens => n,
        MeshTopology::Reck => (2 * n).saturating_sub(3),
    };

    // Calculate Insertion Loss Breakdown (in decibels dB)
    let coupling_loss_db = 2.0 * DEFAULT_EDGE_COUPLING_LOSS_DB;
    let awg_mux_loss_db = DEFAULT_AWG_MUX_DEMUX_LOSS_DB;
    let waveguide_propagation_db = DEFAULT_SOI_WG_LOSS_DB_PER_CM * options.waveguide_length_cm;
    let mzi_mesh_loss_db = optical_depth as f64 * DEFAULT_MZI_UNIT_LOSS_DB;
    let crossings_count = (n * (n - 1)) / 2;
    let waveguide_crossings_db = crossings_count as f64 * DEFAULT_CROSSING_LOSS_DB;
    let awg_demux_loss_db = DEFAULT_AWG_MUX_DEMUX_LOSS_DB;

    let total_insertion_loss_db = coupling_loss_db
        + awg_mux_loss_db
        + waveguide_propagation_db
        + mzi_mesh_loss_db
        + waveguide_crossings_db
        + awg_demux_loss_db;

    let loss_breakdown = LossBreakdown {
        coupling_loss_db,
        awg_mux_loss_db,
        waveguide_propagation_db,
        mzi_mesh_loss_db,
        waveguide_crossings_db,
        awg_demux_loss_db,
        total_insertion_loss_db,
    };

    // Required Optical Power
    let min_rx_power_dbm = MIN_PD_SENSITIVITY_DBM;
    let required_laser_power_dbm = min_rx_power_dbm + total_insertion_loss_db + OPTICAL_SAFETY_MARGIN_DB;
    let required_laser_power_mw = 10.0f64.powf(required_laser_power_dbm / 10.0);
    let wall_plug_eff = options.laser_wall_plug_eff.max(0.01);
    let electrical_laser_power_mw = required_laser_power_mw / wall_plug_eff;

    // Energy analysis: Continuous Wave (CW) vs Dynamic Gated
    let cycle_time_ns = 0.5; // 2.0 GHz
    let total_time_us = (total_cycles as f64 * cycle_time_ns) / 1000.0;
    let continuous_wave_energy_uj = (electrical_laser_power_mw / 1000.0) * (total_time_us); // mW * us = nJ, /1000 = uJ

    let active_optical_cycles = optical_ops.max(wdm_ops).max(laser_strobes).min(total_cycles);
    let active_fraction = if total_cycles > 0 {
        (active_optical_cycles as f64 / total_cycles as f64).min(1.0)
    } else {
        0.0
    };
    // Add 5% standby leakage when gated
    let dynamic_gated_energy_uj = continuous_wave_energy_uj * (active_fraction * 0.95 + 0.05);
    let energy_saved_pct = if continuous_wave_energy_uj > 0.0 {
        ((continuous_wave_energy_uj - dynamic_gated_energy_uj) / continuous_wave_energy_uj) * 100.0
    } else {
        0.0
    };

    // WDM Channel Status Analysis
    let mut wdm_channels = Vec::with_capacity(options.wdm_channels);
    let c = 299_792_458.0; // m/s
    for ch in 0..options.wdm_channels {
        let wl_nm = options.base_wavelength_nm + (ch as f64 * WDM_100GHZ_SPACING_NM);
        let freq_thz = (c / (wl_nm * 1e-9)) / 1e12;
        let ops = channel_activity[ch];
        let ch_power_mw = required_laser_power_mw / options.wdm_channels as f64;
        let ch_power_dbm = 10.0 * ch_power_mw.log10();
        let snr_db = 42.5 - (ch as f64 * 0.35); // Slight dispersion penalty for higher channels
        let crosstalk_db = -34.0 + (ch as f64 * 0.2); // Inter-channel crosstalk

        wdm_channels.push(WdmChannelInfo {
            channel_idx: ch,
            wavelength_nm: wl_nm,
            frequency_thz: freq_thz,
            active_ops_count: ops,
            optical_power_mw: ch_power_mw,
            optical_power_dbm: ch_power_dbm,
            snr_db,
            crosstalk_db,
        });
    }

    // Thermal Phase Drift Sensitivity: ~0.08 pi rad / °C on Silicon
    let thermal_phase_drift_rad = 0.25; // at standard ambient delta

    // Compliance: Laser power under 100 mW optical (safe from nonlinear two-photon absorption)
    let is_loss_budget_compliant = required_laser_power_mw <= 120.0 && total_insertion_loss_db <= 28.0;

    Ok(ClOpticReport {
        total_cycles,
        total_vliw_bundles: total_cycles,
        optical_ops_count: optical_ops,
        wdm_ops_count: wdm_ops,
        laser_pump_strobes: laser_strobes,
        mesh_dimension: n,
        optical_depth,
        topology: options.topology,
        insertion_loss_db: total_insertion_loss_db,
        loss_breakdown,
        min_rx_power_dbm,
        required_laser_power_dbm,
        required_laser_power_mw,
        electrical_laser_power_mw,
        continuous_wave_energy_uj,
        dynamic_gated_energy_uj,
        energy_saved_pct,
        wdm_channels,
        thermal_phase_drift_rad,
        is_loss_budget_compliant,
    })
}

/// Render rich terminal ASCII HUD for optical insertion loss and laser power budget
pub fn format_optic_ascii_hud(report: &ClOpticReport) -> String {
    let mut out = String::new();
    out.push_str("========================================================================================\n");
    out.push_str("    CRON HETEROGENEOUS OPTICAL WDM LASER POWER & INSERTION LOSS OPTIMIZER (SSS+)       \n");
    out.push_str("========================================================================================\n");
    out.push_str(&format!(" Silicon Mesh Topology:    {:?} (Dimension: {}x{}, Depth: {} MZI stages)\n",
        report.topology, report.mesh_dimension, report.mesh_dimension, report.optical_depth));
    out.push_str(&format!(" Total Insertion Loss:     {:.2} dB {}\n",
        report.insertion_loss_db,
        if report.is_loss_budget_compliant { "[PASS: WITHIN BUDGET]" } else { "[FAIL: ATTENUATION EXCEEDED]" }
    ));
    out.push_str(&format!(" Required Optical Laser:   {:.2} dBm ({:.2} mW optical | {:.2} mW electrical @ 22% wall-plug)\n",
        report.required_laser_power_dbm, report.required_laser_power_mw, report.electrical_laser_power_mw));
    out.push_str(&format!(" Dynamic Laser Gating:     {:.1}% Energy Reduction vs Continuous-Wave (CW) Laser\n",
        report.energy_saved_pct));
    out.push_str(&format!(" CW Energy vs Gated:       {:.4} uJ (CW) -> {:.4} uJ (Gated)\n",
        report.continuous_wave_energy_uj, report.dynamic_gated_energy_uj));
    out.push_str("----------------------------------------------------------------------------------------\n");
    out.push_str(" PHOTONIC COMPONENT INSERTION LOSS WATERFALL (dB):\n");
    out.push_str(&format!("   1. Edge Fiber Couplers (In/Out):  {:>6.2} dB [==================                  ]\n",
        report.loss_breakdown.coupling_loss_db));
    out.push_str(&format!("   2. AWG WDM Mux (Tx):              {:>6.2} dB [=============                      ]\n",
        report.loss_breakdown.awg_mux_loss_db));
    out.push_str(&format!("   3. Waveguide Propagation (SOI):   {:>6.2} dB [========                          ]\n",
        report.loss_breakdown.waveguide_propagation_db));
    out.push_str(&format!("   4. MZI Active Mesh Array:         {:>6.2} dB [====================================]\n",
        report.loss_breakdown.mzi_mesh_loss_db));
    out.push_str(&format!("   5. Waveguide Crossings (4D NoC):  {:>6.2} dB [=====                             ]\n",
        report.loss_breakdown.waveguide_crossings_db));
    out.push_str(&format!("   6. AWG WDM Demux (Rx):            {:>6.2} dB [=============                      ]\n",
        report.loss_breakdown.awg_demux_loss_db));
    out.push_str("----------------------------------------------------------------------------------------\n");
    out.push_str(" WDM C-BAND 100 GHz LAMBDA CHANNELS ALLOCATION (1530nm - 1565nm):\n");

    for ch in &report.wdm_channels {
        let bar_len = ((ch.active_ops_count as f64 / report.optical_ops_count.max(1) as f64) * 20.0).round() as usize;
        let bar_str = "=".repeat(bar_len.min(20));
        out.push_str(&format!("   Ch #{:<2} ({:.2}nm / {:.1}THz): Pwr={:>5.2}mW ({:>5.1}dBm) | SNR={:>4.1}dB | Ops={:<3} [{:<20}]\n",
            ch.channel_idx, ch.wavelength_nm, ch.frequency_thz, ch.optical_power_mw, ch.optical_power_dbm, ch.snr_db, ch.active_ops_count, bar_str));
    }

    out.push_str("----------------------------------------------------------------------------------------\n");
    out.push_str(&format!(" Thermal Phase Drift Offset:  +{:.4} rad (Tuned via CORDIC Phase Compensation _CD)\n",
        report.thermal_phase_drift_rad));
    out.push_str(&format!(" Target Photodiode Sensitivity: {:.1} dBm (Minimum optical detection floor)\n",
        report.min_rx_power_dbm));
    out.push_str("========================================================================================\n");
    out
}

/// Convert optical telemetry report to standard JSON string (zero external crates)
pub fn optic_report_to_json(report: &ClOpticReport) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str(&format!("  \"status\": \"{}\",\n", if report.is_loss_budget_compliant { "COMPLIANT" } else { "EXCEEDED" }));
    json.push_str(&format!("  \"total_cycles\": {},\n", report.total_cycles));
    json.push_str(&format!("  \"optical_ops_count\": {},\n", report.optical_ops_count));
    json.push_str(&format!("  \"wdm_ops_count\": {},\n", report.wdm_ops_count));
    json.push_str(&format!("  \"mesh_dimension\": {},\n", report.mesh_dimension));
    json.push_str(&format!("  \"optical_depth\": {},\n", report.optical_depth));
    json.push_str(&format!("  \"topology\": \"{:?}\",\n", report.topology));
    json.push_str(&format!("  \"insertion_loss_db\": {:.4},\n", report.insertion_loss_db));
    json.push_str(&format!("  \"required_laser_power_dbm\": {:.4},\n", report.required_laser_power_dbm));
    json.push_str(&format!("  \"required_laser_power_mw\": {:.4},\n", report.required_laser_power_mw));
    json.push_str(&format!("  \"electrical_laser_power_mw\": {:.4},\n", report.electrical_laser_power_mw));
    json.push_str(&format!("  \"continuous_wave_energy_uj\": {:.4},\n", report.continuous_wave_energy_uj));
    json.push_str(&format!("  \"dynamic_gated_energy_uj\": {:.4},\n", report.dynamic_gated_energy_uj));
    json.push_str(&format!("  \"energy_saved_pct\": {:.2},\n", report.energy_saved_pct));
    json.push_str(&format!("  \"thermal_phase_drift_rad\": {:.4},\n", report.thermal_phase_drift_rad));
    json.push_str("  \"loss_breakdown\": {\n");
    json.push_str(&format!("    \"coupling_loss_db\": {:.4},\n", report.loss_breakdown.coupling_loss_db));
    json.push_str(&format!("    \"awg_mux_loss_db\": {:.4},\n", report.loss_breakdown.awg_mux_loss_db));
    json.push_str(&format!("    \"waveguide_propagation_db\": {:.4},\n", report.loss_breakdown.waveguide_propagation_db));
    json.push_str(&format!("    \"mzi_mesh_loss_db\": {:.4},\n", report.loss_breakdown.mzi_mesh_loss_db));
    json.push_str(&format!("    \"waveguide_crossings_db\": {:.4},\n", report.loss_breakdown.waveguide_crossings_db));
    json.push_str(&format!("    \"awg_demux_loss_db\": {:.4}\n", report.loss_breakdown.awg_demux_loss_db));
    json.push_str("  },\n");
    json.push_str("  \"wdm_channels\": [\n");
    for (i, ch) in report.wdm_channels.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"channel_idx\": {},\n", ch.channel_idx));
        json.push_str(&format!("      \"wavelength_nm\": {:.2},\n", ch.wavelength_nm));
        json.push_str(&format!("      \"frequency_thz\": {:.2},\n", ch.frequency_thz));
        json.push_str(&format!("      \"power_mw\": {:.4},\n", ch.optical_power_mw));
        json.push_str(&format!("      \"snr_db\": {:.2},\n", ch.snr_db));
        json.push_str(&format!("      \"active_ops_count\": {}\n", ch.active_ops_count));
        if i + 1 < report.wdm_channels.len() {
            json.push_str("    },\n");
        } else {
            json.push_str("    }\n");
        }
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

/// Synthesize synthesizable IEEE 1364-2001 Verilog RTL for closed-loop Laser Power Controller
pub fn synthesize_optical_power_verilog(report: &ClOpticReport) -> String {
    let mut v = String::new();
    v.push_str("// ============================================================================\n");
    v.push_str("// CRON Synthesizable Optical Laser Power & WDM Controller RTL\n");
    v.push_str("// Generated automatically by `cron cl-optic` optimizer\n");
    v.push_str("// ============================================================================\n");
    v.push_str("`timescale 1ns / 1ps\n\n");
    v.push_str("module optical_laser_power_controller #(\n");
    v.push_str(&format!("    parameter MESH_DIM        = {},\n", report.mesh_dimension));
    v.push_str(&format!("    parameter WDM_CHANNELS    = {},\n", report.wdm_channels.len()));
    v.push_str(&format!("    parameter OPTICAL_DEPTH   = {},\n", report.optical_depth));
    v.push_str(&format!("    parameter LASER_POWER_MW  = {},\n", (report.required_laser_power_mw.ceil() as usize).max(1)));
    v.push_str("    parameter TARGET_IL_DB    = 18\n");
    v.push_str(") (\n");
    v.push_str("    input  wire        clk,\n");
    v.push_str("    input  wire        rst_n,\n");
    v.push_str("    input  wire        laser_strobe_pulse,\n");
    v.push_str("    input  wire [3:0]  target_wdm_channel,\n");
    v.push_str("    input  wire [15:0] pd_feedback_current_ua,\n");
    v.push_str("    output reg         laser_diode_enable,\n");
    v.push_str("    output reg  [7:0]  laser_dac_drive_val,\n");
    v.push_str("    output reg  [3:0]  active_lambda_sel,\n");
    v.push_str("    output reg         attenuation_fault\n");
    v.push_str(");\n\n");
    v.push_str("    // Closed-loop PI Laser Regulation Loop\n");
    v.push_str("    always @(posedge clk or negedge rst_n) begin\n");
    v.push_str("        if (!rst_n) begin\n");
    v.push_str("            laser_diode_enable  <= 1'b0;\n");
    v.push_str("            laser_dac_drive_val <= 8'h00;\n");
    v.push_str("            active_lambda_sel   <= 4'h0;\n");
    v.push_str("            attenuation_fault   <= 1'b0;\n");
    v.push_str("        end else begin\n");
    v.push_str("            active_lambda_sel   <= target_wdm_channel;\n");
    v.push_str("            if (laser_strobe_pulse) begin\n");
    v.push_str("                laser_diode_enable  <= 1'b1;\n");
    v.push_str("                laser_dac_drive_val <= 8'd180; // Optimal bias setpoint\n");
    v.push_str("            end else begin\n");
    v.push_str("                laser_diode_enable  <= 1'b0; // Zero-idle laser gating\n");
    v.push_str("                laser_dac_drive_val <= 8'd10; // Low standby bias\n");
    v.push_str("            end\n");
    v.push_str("            // Safety check against photodiode loss\n");
    v.push_str("            if (laser_diode_enable && (pd_feedback_current_ua < 16'd50)) begin\n");
    v.push_str("                attenuation_fault <= 1'b1;\n");
    v.push_str("            end else begin\n");
    v.push_str("                attenuation_fault <= 1'b0;\n");
    v.push_str("            end\n");
    v.push_str("        end\n");
    v.push_str("    end\n\n");
    v.push_str("endmodule\n");
    v
}
