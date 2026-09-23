// ============================================================================
// CRON Hardware Roofline Model & Operational Intensity Benchmark
// Pure Rust Implementation (Zero External Dependencies)
//
// Models kernel performance on the 256-Core 4D-Torus Photonic/Neuromorphic
// Silicon Architecture using the Williams-Patterson Roofline Formulation.
// Determines whether an AI-generated .cl kernel is Memory-Bound or Compute-Bound
// and calculates exact operational intensity (FLOP/Byte) and attainable throughput.
// ============================================================================

use crate::cl_lang::parse_slot;

/// Peak hardware silicon parameters for CRON 256-Core 4D-Torus Processor
pub const PEAK_CORE_COMPUTE_GFLOPS: f64 = 80.0;       // Optical MZI + SIMD ALU @ 2.5 GHz
pub const PEAK_CORE_SRAM_BW_GBPS: f64 = 160.0;        // 16 Banks * 4B @ 2.5 GHz
pub const ROOFLINE_KNEE_OI: f64 = PEAK_CORE_COMPUTE_GFLOPS / PEAK_CORE_SRAM_BW_GBPS; // 0.5 FLOP/Byte
pub const TOTAL_CHIP_CORES: usize = 256;
pub const CHIP_PEAK_TFLOPS: f64 = (PEAK_CORE_COMPUTE_GFLOPS * TOTAL_CHIP_CORES as f64) / 1000.0; // 20.48 TFLOPs
pub const CHIP_PEAK_TBPS: f64 = (PEAK_CORE_SRAM_BW_GBPS * TOTAL_CHIP_CORES as f64) / 1000.0;     // 40.96 TB/s

/// Operating regime on the Roofline curve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RooflineRegime {
    ComputeBound,
    MemoryBound,
}

impl RooflineRegime {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ComputeBound => "COMPUTE-BOUND (Silicon Optical/ALU Saturated)",
            Self::MemoryBound => "MEMORY-BOUND (SRAM Crossbar/NoC Latency Limited)",
        }
    }
}

/// Comprehensive Roofline performance and operational intensity report
#[derive(Debug, Clone)]
pub struct ClBenchReport {
    pub total_cycles: usize,
    pub total_flops: f64,
    pub optical_gemm_ops: usize,
    pub subbyte_mac_ops: usize,
    pub simd_alu_ops: usize,
    pub total_memory_traffic_bytes: usize,
    pub sram_access_count: usize,
    pub noc_packet_transfers: usize,
    pub operational_intensity: f64,
    pub regime: RooflineRegime,
    pub attainable_core_gflops: f64,
    pub attainable_chip_tflops: f64,
    pub silicon_efficiency_pct: f64,
    pub theoretical_ipc: f64,
    pub ai_optimization_guidance: String,
}

impl ClBenchReport {
    pub fn to_json(&self) -> String {
        let mut json = String::with_capacity(2048);
        json.push_str("{\n");
        json.push_str(&format!("  \"total_cycles\": {},\n", self.total_cycles));
        json.push_str(&format!("  \"total_flops\": {:.1},\n", self.total_flops));
        json.push_str(&format!("  \"optical_gemm_ops\": {},\n", self.optical_gemm_ops));
        json.push_str(&format!("  \"subbyte_mac_ops\": {},\n", self.subbyte_mac_ops));
        json.push_str(&format!("  \"simd_alu_ops\": {},\n", self.simd_alu_ops));
        json.push_str(&format!("  \"total_memory_traffic_bytes\": {},\n", self.total_memory_traffic_bytes));
        json.push_str(&format!("  \"sram_access_count\": {},\n", self.sram_access_count));
        json.push_str(&format!("  \"noc_packet_transfers\": {},\n", self.noc_packet_transfers));
        json.push_str(&format!("  \"operational_intensity_flop_per_byte\": {:.4},\n", self.operational_intensity));
        json.push_str(&format!("  \"roofline_knee_oi\": {:.2},\n", ROOFLINE_KNEE_OI));
        json.push_str(&format!("  \"regime\": \"{}\",\n", match self.regime {
            RooflineRegime::ComputeBound => "COMPUTE_BOUND",
            RooflineRegime::MemoryBound => "MEMORY_BOUND",
        }));
        json.push_str(&format!("  \"attainable_core_gflops\": {:.2},\n", self.attainable_core_gflops));
        json.push_str(&format!("  \"attainable_chip_tflops\": {:.2},\n", self.attainable_chip_tflops));
        json.push_str(&format!("  \"silicon_efficiency_pct\": {:.2},\n", self.silicon_efficiency_pct));
        json.push_str(&format!("  \"theoretical_ipc\": {:.2},\n", self.theoretical_ipc));
        let escaped_guidance = self.ai_optimization_guidance.replace('\"', "\\\"");
        json.push_str(&format!("  \"ai_optimization_guidance\": \"{}\"\n", escaped_guidance));
        json.push_str("}\n");
        json
    }

    pub fn format_ascii_report(&self, source_label: &str) -> String {
        let mut out = String::with_capacity(3072);
        out.push_str("========================================================================================\n");
        out.push_str("       CRON 256-CORE 4D-TORUS HARDWARE ROOFLINE & PERFORMANCE BENCHMARK                 \n");
        out.push_str("========================================================================================\n");
        out.push_str(&format!(" Target Source:            {}\n", source_label));
        out.push_str(&format!(" Kernel Cycles Analyzed:   {} cycles\n", self.total_cycles));
        let regime_colored = match self.regime {
            RooflineRegime::ComputeBound => "\x1b[1;32mCOMPUTE-BOUND (Peak Silicon Utilization)\x1b[0m",
            RooflineRegime::MemoryBound => "\x1b[1;33mMEMORY-BOUND (Bandwidth / Latency Limited)\x1b[0m",
        };
        out.push_str(&format!(" Operating Regime:         {}\n", regime_colored));
        out.push_str(&format!(" Operational Intensity:    {:.3} FLOPs/Byte (Silicon Knee: 0.50 FLOP/Byte)\n", self.operational_intensity));
        out.push_str(&format!(" Attainable Throughput:    {:.2} GFLOPs/Core | {:.2} TFLOPs Chip Aggregate\n", self.attainable_core_gflops, self.attainable_chip_tflops));
        out.push_str(&format!(" Hardware Efficiency:      {:.1}% of Peak Silicon (20.48 TFLOPs max)\n", self.silicon_efficiency_pct));
        out.push_str(&format!(" Achieved VLIW IPC:        {:.2} ops/cycle (Max Theoretical: 4.0)\n", self.theoretical_ipc));
        out.push_str("----------------------------------------------------------------------------------------\n");
        out.push_str(" Workload Operational Characterization:\n");
        out.push_str(&format!("   - Optical Photonic GEMMs:     {:<5} ops   ({:.1} FLOPs, 0ns Latency)\n",
            self.optical_gemm_ops, self.optical_gemm_ops as f64 * 32.0));
        out.push_str(&format!("   - Ternary Sub-Byte MACs:      {:<5} ops   ({:.1} INT-ops)\n",
            self.subbyte_mac_ops, self.subbyte_mac_ops as f64 * 16.0));
        out.push_str(&format!("   - SIMD Arithmetic / Softmax:  {:<5} ops   ({:.1} FLOPs)\n",
            self.simd_alu_ops, self.simd_alu_ops as f64 * 4.0));
        out.push_str(&format!("   - Total Effective Compute:    {:.1} FLOPs\n", self.total_flops));
        out.push_str(&format!("   - SRAM Memory Traffic:        {:<5} bytes ({:<4} 32-bit accesses)\n",
            self.total_memory_traffic_bytes, self.sram_access_count));
        out.push_str(&format!("   - 4D Torus NoC Transfers:     {:<5} packets\n", self.noc_packet_transfers));
        out.push_str("----------------------------------------------------------------------------------------\n");
        out.push_str(" Silicon Roofline Model Curve:\n");
        out.push_str(&render_ascii_roofline(self.operational_intensity, self.attainable_core_gflops));
        out.push_str("----------------------------------------------------------------------------------------\n");
        out.push_str(" AI Vibe-Coding Performance Recommendation:\n");
        out.push_str(&format!("   -> {}\n", self.ai_optimization_guidance));
        out.push_str("========================================================================================\n");
        out
    }
}

/// Analyze `.cl` source code against the CRON 4D-Torus Roofline model
pub fn analyze_cl_roofline(cl_source: &str) -> Result<ClBenchReport, String> {
    let mut total_cycles = 0;
    let mut optical_gemm_ops = 0;
    let mut subbyte_mac_ops = 0;
    let mut simd_alu_ops = 0;
    let mut sram_access_count = 0;
    let mut noc_packet_transfers = 0;
    let mut total_active_slots = 0;

    for line in cl_source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('@') || trimmed.starts_with('.') || !trimmed.starts_with('B') {
            continue;
        }

        if let Some((_b_part, slots_part)) = trimmed.split_once(':') {
            total_cycles += 1;
            let slot_strs: Vec<&str> = slots_part.split_whitespace().collect();

            for &s_str in slot_strs.iter().take(4) {
                if let Ok(slot) = parse_slot(s_str) {
                    if slot.opcode == "NO" || slot.opcode == "HL" {
                        continue;
                    }
                    total_active_slots += 1;

                    match slot.opcode.as_str() {
                        // Photonic Brain 2: 4x4 complex MZI GEMM = 16 dot ops = 32 FLOPs
                        "OP" => optical_gemm_ops += 1,
                        // Brain 4 Sub-byte Ternary MAC = 16 sub-byte ops
                        "MD" => {
                            subbyte_mac_ops += 1;
                            sram_access_count += 1; // Stream weight from SRAM
                        }
                        // Brain 2 In-register 4x4 Transpose / Tile load
                        "TT" => {
                            sram_access_count += 1;
                        }
                        // SIMD ALU
                        "PO" | "P0" | "P1" | "FA" => {
                            simd_alu_ops += 1;
                        }
                        // Memory Store
                        "ST" | "PK" => {
                            sram_access_count += 1;
                        }
                        // NoC Communication
                        "TX" | "RX" | "SB" | "DW" | "DD" => {
                            noc_packet_transfers += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if total_cycles == 0 {
        return Err("No valid VLIW instruction cycles found to benchmark".to_string());
    }

    let total_flops = (optical_gemm_ops as f64 * 32.0)
        + (subbyte_mac_ops as f64 * 16.0)
        + (simd_alu_ops as f64 * 4.0);

    // Each memory access transfers 4 bytes (32-bit word) across SRAM or NoC router
    let total_memory_traffic_bytes = (sram_access_count * 4) + (noc_packet_transfers * 16);
    let operational_intensity = total_flops / total_memory_traffic_bytes.max(4) as f64;

    let regime = if operational_intensity >= ROOFLINE_KNEE_OI {
        RooflineRegime::ComputeBound
    } else {
        RooflineRegime::MemoryBound
    };

    // Attainable throughput according to Roofline formula
    let attainable_core_gflops = (operational_intensity * PEAK_CORE_SRAM_BW_GBPS).min(PEAK_CORE_COMPUTE_GFLOPS);
    let attainable_chip_tflops = (attainable_core_gflops * TOTAL_CHIP_CORES as f64) / 1000.0;
    let silicon_efficiency_pct = (attainable_core_gflops / PEAK_CORE_COMPUTE_GFLOPS) * 100.0;
    let theoretical_ipc = total_active_slots as f64 / total_cycles as f64;

    let ai_optimization_guidance = match regime {
        RooflineRegime::ComputeBound => {
            format!(
                "Kernel is COMPUTE-BOUND with high operational intensity ({:.2} FLOP/Byte). Photonic MZI mesh is saturated at {:.1}% efficiency. Recommended next step: deploy via 'cron cl-link' across 256 cores.",
                operational_intensity, silicon_efficiency_pct
            )
        }
        RooflineRegime::MemoryBound => {
            format!(
                "Kernel is MEMORY-BOUND with operational intensity ({:.2} FLOP/Byte < {:.2}). Bottlenecked by SRAM bandwidth. Recommended action: Increase 4D-tile reuse, enable GF(2^4) XOR swizzling ('cron cl-memcheck'), and pack sub-bytes via '_PK' opcode.",
                operational_intensity, ROOFLINE_KNEE_OI
            )
        }
    };

    Ok(ClBenchReport {
        total_cycles,
        total_flops,
        optical_gemm_ops,
        subbyte_mac_ops,
        simd_alu_ops,
        total_memory_traffic_bytes,
        sram_access_count,
        noc_packet_transfers,
        operational_intensity,
        regime,
        attainable_core_gflops,
        attainable_chip_tflops,
        silicon_efficiency_pct,
        theoretical_ipc,
        ai_optimization_guidance,
    })
}

/// Generates a high-density ASCII Roofline chart marking the kernel operating point
fn render_ascii_roofline(kernel_oi: f64, kernel_gflops: f64) -> String {
    let mut chart = String::with_capacity(1024);

    let is_compute = kernel_oi >= ROOFLINE_KNEE_OI;
    let marker_line = if is_compute {
        format!("   80 GFLOPs |---------------------------------[*]-------- Peak Silicon Ceiling (80.0 GFLOPs) <- Operating Point ({:.2} GFLOPs)\n", kernel_gflops)
    } else {
        "   80 GFLOPs |------------------------------------------- Peak Silicon Ceiling (80.0 GFLOPs)\n".to_string()
    };

    chart.push_str(&marker_line);
    chart.push_str("             |                                /          \n");
    chart.push_str("   60 GFLOPs |                               /           \n");
    if !is_compute && kernel_gflops >= 30.0 {
        chart.push_str(&format!("             |                     [*]      /            <- Operating Point ({:.2} GFLOPs)\n", kernel_gflops));
    } else {
        chart.push_str("             |                             /             \n");
    }
    chart.push_str("   40 GFLOPs |                            /              \n");
    if !is_compute && (10.0..30.0).contains(&kernel_gflops) {
        chart.push_str(&format!("             |               [*]         /               <- Operating Point ({:.2} GFLOPs)\n", kernel_gflops));
    } else {
        chart.push_str("             |                          /                \n");
    }
    chart.push_str("   20 GFLOPs |                         /  SRAM Slope     \n");
    if !is_compute && kernel_gflops < 10.0 {
        chart.push_str(&format!("             |         [*]            /   (160 GB/s)     <- Operating Point ({:.2} GFLOPs)\n", kernel_gflops));
    } else {
        chart.push_str("             |                       /    (160 GB/s)     \n");
    }
    chart.push_str("    0 GFLOPs +---------+-------------+-------------+-----\n");
    chart.push_str("              0.05    0.1           0.50          2.0    Operational Intensity (FLOP/Byte)\n");
    chart.push_str("             <--- MEMORY-BOUND ---> | <--- COMPUTE-BOUND --->\n");

    chart
}
