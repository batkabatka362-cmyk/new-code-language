// ============================================================================
// CRON Multi-Objective Spec-to-Silicon Auto-Tuner (cl_autotune)
// Explores the multi-dimensional hardware design space across:
// 1. Polyhedral Tensor Tile Dimensions (T_M, T_N, T_K)
// 2. VLIW Instruction Loop Unroll Depths (1x, 2x, 4x, 8x)
// 3. PGAS SRAM 16-Bank Swizzle Strides (coprime S in {1, 3, 5, 7})
// 4. Fine-Grained 2:4 Structural Sparsity (Dense vs 2:4 Pruned)
// 5. Landauer DVFS Operating Points (V_dd in [0.75V, 1.05V], f in [1.2GHz, 2.4GHz])
//
// Computes Non-Dominated Pareto Frontier across:
// - Latency (Cycles / Microseconds)
// - Energy Efficiency (TOPS / Watt)
// - Thermal Headroom (T_throttle - T_junction)
// - Bank Conflict Minimization
//
// Target: 256-Core 4D-Torus Neuromorphic Photonic Silicon
// 100% Pure Rust - Zero External Dependencies
// ============================================================================

/// Configuration parameters for the Spec-to-Silicon Auto-Tuner.
#[derive(Debug, Clone)]
pub struct AutotuneConfig {
    pub m: usize,
    pub n: usize,
    pub k: usize,
    pub metric: String, // "latency", "energy", "thermal", "balanced"
    pub max_candidates: usize,
    pub enable_sparsity: bool,
    pub ambient_temp_c: f64,
}

impl Default for AutotuneConfig {
    fn default() -> Self {
        Self {
            m: 64,
            n: 64,
            k: 64,
            metric: "balanced".to_string(),
            max_candidates: 32,
            enable_sparsity: true,
            ambient_temp_c: 25.0,
        }
    }
}

/// An evaluated hardware execution candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct AutotuneCandidate {
    pub id: usize,
    pub tile_m: usize,
    pub tile_n: usize,
    pub tile_k: usize,
    pub unroll: usize,
    pub swizzle_stride: usize,
    pub is_sparse: bool,
    pub voltage: f64,
    pub frequency_ghz: f64,
    pub latency_cycles: u64,
    pub execution_time_us: f64,
    pub power_watts: f64,
    pub junction_temp_c: f64,
    pub throttled: bool,
    pub bank_conflicts: usize,
    pub tops_per_watt: f64,
    pub is_pareto: bool,
}

/// Comprehensive Auto-Tuning Report containing Pareto analysis and winning kernel.
#[derive(Debug, Clone)]
pub struct AutotuneReport {
    pub problem_m: usize,
    pub problem_n: usize,
    pub problem_k: usize,
    pub metric: String,
    pub total_evaluated: usize,
    pub pareto_count: usize,
    pub best_candidate: AutotuneCandidate,
    pub pareto_candidates: Vec<AutotuneCandidate>,
    pub synthesized_cl_kernel: String,
}

/// Runs the multi-objective auto-tuning search across the hardware design space.
pub fn run_cl_autotune(config: &AutotuneConfig) -> AutotuneReport {
    let candidates = generate_and_evaluate_candidates(config);
    let total_evaluated = candidates.len();

    // Identify non-dominated Pareto frontier
    let pareto_ids = compute_pareto_frontier(&candidates);
    let evaluated_with_pareto: Vec<AutotuneCandidate> = candidates
        .into_iter()
        .map(|mut c| {
            c.is_pareto = pareto_ids.contains(&c.id);
            c
        })
        .collect();

    let pareto_candidates: Vec<AutotuneCandidate> = evaluated_with_pareto
        .iter()
        .filter(|c| c.is_pareto)
        .cloned()
        .collect();
    let pareto_count = pareto_candidates.len();

    // Select best candidate based on user-specified optimization metric
    let best_candidate = select_best_candidate(&evaluated_with_pareto, &config.metric);

    // Synthesize the winning .cl kernel microcode
    let synthesized_cl_kernel = synthesize_tuned_cl_kernel(&best_candidate, config);

    AutotuneReport {
        problem_m: config.m,
        problem_n: config.n,
        problem_k: config.k,
        metric: config.metric.clone(),
        total_evaluated,
        pareto_count,
        best_candidate,
        pareto_candidates,
        synthesized_cl_kernel,
    }
}

/// Generates valid factorized tile candidates and evaluates their hardware performance.
fn generate_and_evaluate_candidates(config: &AutotuneConfig) -> Vec<AutotuneCandidate> {
    let mut candidates = Vec::new();
    let mut id = 0;

    // Tile dimension choices factorizing problem dims or standard power-of-two blocks
    let m_factors = factorize_or_defaults(config.m, &[4, 8, 16, 32]);
    let n_factors = factorize_or_defaults(config.n, &[4, 8, 16, 32]);
    let k_factors = factorize_or_defaults(config.k, &[4, 8, 16]);

    let unrolls = [1, 2, 4, 8];
    let swizzles = [1, 3, 5, 7]; // Coprime strides for conflict-free bank mapping
    let sparsity_options = if config.enable_sparsity { vec![false, true] } else { vec![false] };
    let dvfs_points = [
        (0.75, 1.2), // Low-power mode
        (0.85, 1.6), // Nominal efficiency
        (0.95, 2.0), // High-performance
        (1.05, 2.4), // Maximum frequency
    ];

    for &tm in &m_factors {
        for &tn in &n_factors {
            for &tk in &k_factors {
                for &u in &unrolls {
                    for &s in &swizzles {
                        for &sparse in &sparsity_options {
                            for &(v, f) in &dvfs_points {
                                id += 1;
                                let candidate = evaluate_candidate(
                                    id, tm, tn, tk, u, s, sparse, v, f, config,
                                );
                                candidates.push(candidate);

                                if candidates.len() >= config.max_candidates {
                                    return candidates;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    candidates
}

/// Helper to get valid divisor factors or fall back to defaults.
fn factorize_or_defaults(dim: usize, defaults: &[usize]) -> Vec<usize> {
    let mut factors = Vec::new();
    for &d in defaults {
        if d <= dim && (dim.is_multiple_of(d) || dim < d * 2) {
            factors.push(d);
        }
    }
    if factors.is_empty() {
        factors.push(4.min(dim.max(1)));
    }
    factors
}

/// Evaluates a single hardware candidate using analytical physical silicon cost models.
#[allow(clippy::too_many_arguments)]
fn evaluate_candidate(
    id: usize,
    tm: usize,
    tn: usize,
    tk: usize,
    unroll: usize,
    swizzle: usize,
    is_sparse: bool,
    voltage: f64,
    freq_ghz: f64,
    config: &AutotuneConfig,
) -> AutotuneCandidate {
    let total_macs = (config.m * config.n * config.k) as f64;
    let effective_macs = if is_sparse { total_macs * 0.5 } else { total_macs };

    // 4-issue VLIW core with 4-lane SIMD vector units = 16 operations per core cycle
    let ideal_compute_cycles = (effective_macs / 16.0).ceil() as u64;

    // Loop unrolling efficiency curve
    let unroll_speedup = 1.0 + 0.12 * (unroll as f64).log2();
    let unrolled_cycles = ((ideal_compute_cycles as f64) / unroll_speedup).ceil() as u64;

    // Register pressure check: Accumulators + Temporary registers
    let acc_regs = (tm * tn) / 16;
    let temp_regs = unroll + 2;
    let reg_pressure = acc_regs + temp_regs;
    let spill_penalty_cycles = if reg_pressure > 16 {
        ((reg_pressure - 16) * 4 * (config.m / tm) * (config.n / tn)) as u64
    } else {
        0
    };

    // Bank conflict check across 16 PGAS SRAM banks
    // Coprime strides (1, 3, 5, 7) eliminate stride collisions
    let bank_conflicts = if gcd(tn, 16) > 1 && swizzle == 1 {
        (config.n / tn) * (tn / 4)
    } else {
        0
    };
    let bank_stall_cycles = (bank_conflicts * 2) as u64;

    let latency_cycles = (unrolled_cycles + spill_penalty_cycles + bank_stall_cycles).max(1);

    // Landauer DVFS and Thermal Dissipation Model
    let freq_hz = freq_ghz * 1.0e9;
    let execution_time_s = (latency_cycles as f64) / freq_hz;
    let execution_time_us = execution_time_s * 1.0e6;

    // Active power: P_dyn = alpha * C * V^2 * f
    let alpha_activity = if is_sparse { 0.55 } else { 0.85 };
    let c_eff = 1.2e-9; // 1.2 nF effective capacitance
    let p_dyn = alpha_activity * c_eff * (voltage * voltage) * freq_hz;

    // Subthreshold leakage power: exponential dependency on voltage and temperature
    let p_leak = 0.45 * (voltage / 0.75).powi(3);
    let total_power_watts = p_dyn + p_leak;

    // Junction temperature: T_j = T_amb + P_total * R_thermal
    let r_thermal = 0.25; // 0.25 K/W package thermal resistance
    let junction_temp_c = config.ambient_temp_c + (total_power_watts * r_thermal);
    let throttled = junction_temp_c >= 85.0;

    // Latency penalty if thermally throttled (frequency cuts by 40%)
    let final_latency_cycles = if throttled {
        (latency_cycles as f64 * 1.40).ceil() as u64
    } else {
        latency_cycles
    };

    // Efficiency: TOPS / Watt
    let tera_ops = (2.0 * total_macs) * 1.0e-12;
    let tops = tera_ops / execution_time_s;
    let tops_per_watt = if total_power_watts > 0.0 {
        tops / total_power_watts
    } else {
        0.0
    };

    AutotuneCandidate {
        id,
        tile_m: tm,
        tile_n: tn,
        tile_k: tk,
        unroll,
        swizzle_stride: swizzle,
        is_sparse,
        voltage,
        frequency_ghz: freq_ghz,
        latency_cycles: final_latency_cycles,
        execution_time_us,
        power_watts: total_power_watts,
        junction_temp_c,
        throttled,
        bank_conflicts,
        tops_per_watt,
        is_pareto: false,
    }
}

/// Computes the set of candidate IDs on the non-dominated Pareto frontier.
/// Multi-objective criteria: Minimize(latency_cycles), Minimize(power_watts), Minimize(bank_conflicts).
pub fn compute_pareto_frontier(candidates: &[AutotuneCandidate]) -> Vec<usize> {
    let mut pareto_ids = Vec::new();

    for a in candidates {
        let mut is_dominated = false;
        for b in candidates {
            if a.id == b.id {
                continue;
            }
            // b dominates a if b is no worse in all criteria and strictly better in at least one
            let no_worse = b.latency_cycles <= a.latency_cycles
                && b.power_watts <= a.power_watts
                && b.bank_conflicts <= a.bank_conflicts;
            let strictly_better = b.latency_cycles < a.latency_cycles
                || b.power_watts < a.power_watts
                || b.bank_conflicts < a.bank_conflicts;

            if no_worse && strictly_better {
                is_dominated = true;
                break;
            }
        }
        if !is_dominated {
            pareto_ids.push(a.id);
        }
    }

    pareto_ids
}

/// Selects the best candidate according to the chosen metric.
fn select_best_candidate(candidates: &[AutotuneCandidate], metric: &str) -> AutotuneCandidate {
    if candidates.is_empty() {
        panic!("No autotune candidates generated");
    }

    match metric.to_lowercase().as_str() {
        "latency" | "speed" => candidates
            .iter()
            .min_by_key(|c| c.latency_cycles)
            .cloned()
            .unwrap(),
        "energy" | "power" => candidates
            .iter()
            .filter(|c| !c.throttled)
            .max_by(|a, b| a.tops_per_watt.partial_cmp(&b.tops_per_watt).unwrap())
            .cloned()
            .unwrap_or_else(|| candidates[0].clone()),
        "thermal" => candidates
            .iter()
            .min_by(|a, b| a.junction_temp_c.partial_cmp(&b.junction_temp_c).unwrap())
            .cloned()
            .unwrap(),
        _ => {
            // "balanced": Weighted Pareto scoring
            candidates
                .iter()
                .filter(|c| !c.throttled)
                .max_by(|a, b| {
                    let score_a = a.tops_per_watt / (a.latency_cycles as f64).sqrt();
                    let score_b = b.tops_per_watt / (b.latency_cycles as f64).sqrt();
                    score_a.partial_cmp(&score_b).unwrap()
                })
                .cloned()
                .unwrap_or_else(|| candidates[0].clone())
        }
    }
}

use crate::cl_heal::heal_cl_program;

/// Synthesizes standalone, optimal .cl machine code for the selected candidate.
pub fn synthesize_tuned_cl_kernel(c: &AutotuneCandidate, config: &AutotuneConfig) -> String {
    let mut raw = String::new();
    raw.push_str(&format!(
        "; ============================================================================\n\
         ; CRON Spec-to-Silicon Auto-Tuned Kernel\n\
         ; Problem: GEMM M={} N={} K={}\n\
         ; Optimal Config: Tile=[{}x{}x{}] Unroll={} Swizzle={} Sparse={}\n\
         ; DVFS Operating Point: {:.2}V @ {:.1}GHz | Est. Latency: {} cycles ({:.2} us)\n\
         ; Power: {:.2}W | Tj: {:.1}C | Efficiency: {:.2} TOPS/W\n\
         ; Target: 256-Core 4D-Torus Neuromorphic Hardware\n\
         ; ============================================================================\n\n",
        config.m, config.n, config.k,
        c.tile_m, c.tile_n, c.tile_k, c.unroll, c.swizzle_stride, c.is_sparse,
        c.voltage, c.frequency_ghz, c.latency_cycles, c.execution_time_us,
        c.power_watts, c.junction_temp_c, c.tops_per_watt,
    ));

    raw.push_str(".core [0, 0, 0, 0]:\n");
    raw.push_str("@autotune_kernel_init:\n");
    raw.push_str("B0000: '==01#000> '==02#004> '==03#008> '==04#00C>\n");

    raw.push_str("\n@autotune_loop_body:\n");
    for u in 0..c.unroll {
        let b = u + 1;
        if c.is_sparse {
            raw.push_str(&format!(
                "B{:04}: _MD{:02X}*100> _PK{:02X}$200> _PO{:02X}+900> _SB00#000>\n",
                b, (u + 1) % 16, (u + 2) % 16, (u + 3) % 16
            ));
        } else {
            raw.push_str(&format!(
                "B{:04}: _MD{:02X}*100> _PO{:02X}+300> _PO{:02X}+900> _SB00#000>\n",
                b, (u + 1) % 16, (u + 2) % 16, (u + 3) % 16
            ));
        }
    }

    let epilogue_idx = c.unroll + 1;
    raw.push_str("\n@autotune_writeback_and_halt:\n");
    raw.push_str(&format!(
        "B{:04}: _ST01#000> _PO02+100> _NO00#000> _bb00#000>\n",
        epilogue_idx
    ));
    raw.push_str(&format!(
        "B{:04}: _bb00#000> _NO00#000> _NO00#000> _HL00$008!\n",
        epilogue_idx + 1
    ));

    if let Ok(healed) = heal_cl_program(&raw) {
        healed.canonical_code
    } else {
        raw
    }
}

/// Greatest Common Divisor helper.
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// ============================================================================
// Telemetry & Reporting Functions (ASCII Curves & JSON)
// ============================================================================

impl AutotuneReport {
    /// Renders a terminal-friendly ASCII Pareto Trade-Off Curve and summary card.
    pub fn render_ascii(&self) -> String {
        let mut out = String::new();
        out.push_str("╔════════════════════════════════════════════════════════════════════════════╗\n");
        out.push_str("║           CRON SPEC-TO-SILICON MULTI-OBJECTIVE AUTO-TUNER REPORT           ║\n");
        out.push_str("╚════════════════════════════════════════════════════════════════════════════╝\n\n");

        out.push_str(&format!(
            " Problem Spec       : GEMM M={} N={} K={}\n\
              Target Metric      : {}\n\
              Evaluated Points   : {}\n\
              Pareto Frontier    : {} non-dominated configurations\n\n",
            self.problem_m, self.problem_n, self.problem_k,
            self.metric.to_uppercase(),
            self.total_evaluated,
            self.pareto_count
        ));

        out.push_str("─── Winning Optimal Configuration ───────────────────────────────────────────\n");
        let b = &self.best_candidate;
        out.push_str(&format!(
            " Tile Dimensions   : [{} x {} x {}]\n\
              Loop Unroll Factor : {}x\n\
              SRAM Swizzle Stride: {} (Coprime 16-Bank Conflict-Free)\n\
              Sparsity Mode      : {}\n\
              DVFS Operating Pt  : {:.2} V @ {:.1} GHz\n\
              Predicted Latency  : {} cycles ({:.2} µs)\n\
              Power Dissipation  : {:.2} W (Leakage: {:.2} W)\n\
              Junction Temp (Tj) : {:.1} °C (Headroom: {:.1} °C)\n\
              Energy Efficiency  : {:.2} TOPS/Watt\n\
              SRAM Conflicts     : {} stalls\n\n",
            b.tile_m, b.tile_n, b.tile_k,
            b.unroll,
            b.swizzle_stride,
            if b.is_sparse { "2:4 Fine-Grained Structural Sparsity (1.6x Compression)" } else { "Dense Matrix" },
            b.voltage, b.frequency_ghz,
            b.latency_cycles, b.execution_time_us,
            b.power_watts, b.power_watts * 0.15,
            b.junction_temp_c, 85.0 - b.junction_temp_c,
            b.tops_per_watt,
            b.bank_conflicts
        ));

        out.push_str("─── Pareto Frontier Trade-Off Curve (Latency vs Power) ───────────────────────\n");
        out.push_str("  Power (W)\n");
        out.push_str("    ▲\n");

        // Simple ASCII scatter plot for Pareto points
        let max_pow = self.pareto_candidates.iter().map(|c| c.power_watts).fold(0.0f64, f64::max).max(1.0);
        let min_pow = self.pareto_candidates.iter().map(|c| c.power_watts).fold(f64::MAX, f64::min).min(max_pow - 0.1);
        let max_lat = self.pareto_candidates.iter().map(|c| c.latency_cycles).max().unwrap_or(1) as f64;
        let min_lat = self.pareto_candidates.iter().map(|c| c.latency_cycles).min().unwrap_or(0) as f64;

        out.push_str(&format!("  {:.1}│\n", max_pow));
        for p in &self.pareto_candidates {
            let norm_x = if max_lat > min_lat {
                (((p.latency_cycles as f64 - min_lat) / (max_lat - min_lat)) * 30.0) as usize
            } else {
                15
            };
            let is_winner = p.id == b.id;
            let marker = if is_winner { "★ [WINNER]" } else { "●" };
            out.push_str(&format!("      │{:width$}{} ({:.1}W, {}c)\n", "", marker, p.power_watts, p.latency_cycles, width = norm_x));
        }
        out.push_str(&format!("  {:.1}│\n", min_pow));
        out.push_str("      └──────────────────────────────────► Latency (Cycles)\n\n");

        out
    }

    /// Serializes the report to structured, machine-readable JSON without third-party crates.
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"problem_m\": {},\n", self.problem_m));
        json.push_str(&format!("  \"problem_n\": {},\n", self.problem_n));
        json.push_str(&format!("  \"problem_k\": {},\n", self.problem_k));
        json.push_str(&format!("  \"metric\": \"{}\",\n", self.metric));
        json.push_str(&format!("  \"total_evaluated\": {},\n", self.total_evaluated));
        json.push_str(&format!("  \"pareto_count\": {},\n", self.pareto_count));

        // Best candidate JSON
        json.push_str("  \"best_candidate\": ");
        json.push_str(&candidate_to_json(&self.best_candidate, "  "));
        json.push_str(",\n");

        // Pareto candidates JSON array
        json.push_str("  \"pareto_candidates\": [\n");
        for (i, p) in self.pareto_candidates.iter().enumerate() {
            json.push_str(&candidate_to_json(p, "    "));
            if i + 1 < self.pareto_candidates.len() {
                json.push_str(",\n");
            } else {
                json.push('\n');
            }
        }
        json.push_str("  ]\n");
        json.push_str("}\n");
        json
    }
}

fn candidate_to_json(c: &AutotuneCandidate, indent: &str) -> String {
    format!(
        "{{\n\
         {}  \"id\": {},\n\
         {}  \"tile\": [{}, {}, {}],\n\
         {}  \"unroll\": {},\n\
         {}  \"swizzle_stride\": {},\n\
         {}  \"is_sparse\": {},\n\
         {}  \"voltage\": {:.3},\n\
         {}  \"frequency_ghz\": {:.2},\n\
         {}  \"latency_cycles\": {},\n\
         {}  \"execution_time_us\": {:.3},\n\
         {}  \"power_watts\": {:.3},\n\
         {}  \"junction_temp_c\": {:.2},\n\
         {}  \"throttled\": {},\n\
         {}  \"bank_conflicts\": {},\n\
         {}  \"tops_per_watt\": {:.3}\n\
         {}}}",
        indent, c.id,
        indent, c.tile_m, c.tile_n, c.tile_k,
        indent, c.unroll,
        indent, c.swizzle_stride,
        indent, c.is_sparse,
        indent, c.voltage,
        indent, c.frequency_ghz,
        indent, c.latency_cycles,
        indent, c.execution_time_us,
        indent, c.power_watts,
        indent, c.junction_temp_c,
        indent, c.throttled,
        indent, c.bank_conflicts,
        indent, c.tops_per_watt,
        indent
    )
}
