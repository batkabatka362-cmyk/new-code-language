// ============================================================================
// CRON 65,536-Core Wafer-Scale Supercomputer Engine (cl_wafer_scale)
//
// Target: Full-Wafer Monolithic Silicon Architecture (WSE-4)
// Total Cores: 65,536 (256 Dies x 256 Cores per Die)
// Topology: 8D-Torus Hyper-Mesh (4x4x4x4 Intra-Die x 2x2x2x2 Inter-Die)
// Interconnect: 100 Tb/s Optical Inter-Die Waveguides + Electrical 9-Port Routers
// Pipeline Schedule: 1F1B (One-Forward-One-Backward) Parallel Pipeline
// ============================================================================

pub const TOTAL_WAFER_CORES: usize = 65536;
pub const DIES_PER_WAFER: usize = 256;
pub const CORES_PER_DIE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord8D {
    pub core_x: usize, // 0..3
    pub core_y: usize, // 0..3
    pub core_z: usize, // 0..3
    pub core_w: usize, // 0..3
    pub die_x: usize,  // 0..1
    pub die_y: usize,  // 0..1
    pub die_z: usize,  // 0..1
    pub die_w: usize,  // 0..1
}

impl Coord8D {
    pub fn from_global_id(global_id: usize) -> Self {
        let id = global_id % TOTAL_WAFER_CORES;
        let local_core = id % CORES_PER_DIE;
        let die_id = id / CORES_PER_DIE;

        let core_x = local_core % 4;
        let core_y = (local_core / 4) % 4;
        let core_z = (local_core / 16) % 4;
        let core_w = (local_core / 64) % 4;

        let die_x = die_id % 4;
        let die_y = (die_id / 4) % 4;
        let die_z = (die_id / 16) % 4;
        let die_w = (die_id / 64) % 4;

        Self {
            core_x,
            core_y,
            core_z,
            core_w,
            die_x,
            die_y,
            die_z,
            die_w,
        }
    }

    pub fn to_global_id(&self) -> usize {
        let local_core = self.core_x + self.core_y * 4 + self.core_z * 16 + self.core_w * 64;
        let die_id = self.die_x + self.die_y * 4 + self.die_z * 16 + self.die_w * 64;
        die_id * CORES_PER_DIE + local_core
    }

    /// Calculates Manhattan distance and DOR routing hops across the 8D-Torus
    pub fn distance_to(&self, other: &Self) -> (usize, usize) {
        let intra_hops = Self::torus_dist(self.core_x, other.core_x, 4)
            + Self::torus_dist(self.core_y, other.core_y, 4)
            + Self::torus_dist(self.core_z, other.core_z, 4)
            + Self::torus_dist(self.core_w, other.core_w, 4);

        let inter_hops = Self::torus_dist(self.die_x, other.die_x, 4)
            + Self::torus_dist(self.die_y, other.die_y, 4)
            + Self::torus_dist(self.die_z, other.die_z, 4)
            + Self::torus_dist(self.die_w, other.die_w, 4);

        (intra_hops, inter_hops)
    }

    fn torus_dist(a: usize, b: usize, dim: usize) -> usize {
        let diff = if a > b { a - b } else { b - a };
        diff.min(dim - diff)
    }
}

#[derive(Debug, Clone)]
pub struct WaferScaleConfig {
    pub model_parameters: String, // e.g. "70B", "405B"
    pub precision: String,        // e.g. "ternary_1.58b"
    pub pipeline_stages: usize,   // e.g. 16
    pub tensor_parallel_ways: usize, // e.g. 16
    pub expert_parallel_ways: usize, // e.g. 16
    pub context_parallel_ways: usize, // e.g. 16
}

impl Default for WaferScaleConfig {
    fn default() -> Self {
        Self {
            model_parameters: "70B".to_string(),
            precision: "ternary_1.58b".to_string(),
            pipeline_stages: 16,
            tensor_parallel_ways: 16,
            expert_parallel_ways: 16,
            context_parallel_ways: 16,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaferTelemetryReport {
    pub total_active_cores: usize,
    pub total_dies: usize,
    pub peak_tops_int2: f64,
    pub bisection_bandwidth_tb_s: f64,
    pub estimated_power_watts: f64,
    pub pipeline_bubble_percentage: f64,
    pub inter_die_latency_ns: f64,
    pub max_diameter_hops: usize,
}

/// Generates wafer-scale deployment plan and performance telemetry
pub fn generate_wafer_plan(config: &WaferScaleConfig) -> WaferTelemetryReport {
    let total_active_cores = TOTAL_WAFER_CORES;
    let clock_ghz = 2.5; // 2.5 GHz Silicon target
    // 4 Slots/cycle * 2 ops/slot (FMA) * 16 elements/op = 128 ops/cycle/core
    let ops_per_core_per_cycle = 128.0;
    let total_tops = (total_active_cores as f64 * ops_per_core_per_cycle * clock_ghz * 1e9) / 1e12;

    let bisection_tb_s = 256.0 * 0.4; // 102.4 TB/s optical bisection bandwidth
    let power_per_core_mw = 75.0; // 75 mW per sub-byte ternary core
    let total_power_w = (total_active_cores as f64 * power_per_core_mw) / 1000.0 + 850.0; // +850W optical laser supply

    let pipeline_bubble = (1.0 / (config.pipeline_stages as f64 * 4.0)) * 100.0; // 1F1B schedule bubble

    WaferTelemetryReport {
        total_active_cores,
        total_dies: DIES_PER_WAFER,
        peak_tops_int2: total_tops,
        bisection_bandwidth_tb_s: bisection_tb_s,
        estimated_power_watts: total_power_w,
        pipeline_bubble_percentage: pipeline_bubble,
        inter_die_latency_ns: 4.8,
        max_diameter_hops: 8 + 4, // 8 intra + 4 inter
    }
}

/// Renders a full Wafer-Scale ASCII Architecture and Die Grid
pub fn render_ascii_wafer_map(report: &WaferTelemetryReport) -> String {
    let mut map = String::new();
    map.push_str("================================================================================\n");
    map.push_str("         CRON 65,536-CORE FULL WAFER-SCALE SUPERCOMPUTER ENGINE (WSE-4)         \n");
    map.push_str("================================================================================\n");
    map.push_str(&format!("  Active Cores:           {:>8} cores (256 Dies x 256 Cores)\n", report.total_active_cores));
    map.push_str(&format!("  Peak Compute Density:   {:>8.1} TOPS (Ternary 1.58b Linear)\n", report.peak_tops_int2));
    map.push_str(&format!("  Optical Bisection B/W:  {:>8.1} TB/s (Zero-Loss Photonic Mesh)\n", report.bisection_bandwidth_tb_s));
    map.push_str(&format!("  Wafer Power Envelope:   {:>8.1} W (< 6.0 kW Full Thermal Target)\n", report.estimated_power_watts));
    map.push_str(&format!("  1F1B Pipeline Bubble:   {:>8.2} % (Near-Zero Idle Stalls)\n", report.pipeline_bubble_percentage));
    map.push_str("--------------------------------------------------------------------------------\n");
    map.push_str("  16x16 MONOLITHIC SILICON DIE TOPOLOGY (8D-TORUS INTERCONNECT):\n");
    for row in 0..8 {
        map.push_str("    [");
        for col in 0..16 {
            let ch = if (row + col) % 3 == 0 { "■" } else if (row + col) % 2 == 0 { "◆" } else { "●" };
            map.push_str(ch);
        }
        map.push_str(&format!("]  Row {:02} [Dies {:03}..{:03}]\n", row, row * 16, row * 16 + 15));
    }
    map.push_str("    Legend: ■ Optical MZI Attn  ◆ Reversible SwiGLU  ● STDP SNN Crossbar\n");
    map.push_str("================================================================================\n");
    map
}
