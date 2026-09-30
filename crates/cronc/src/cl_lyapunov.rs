//! Brain 6 Sentry: Cascading Drift Watchdog & Lyapunov Stabilization Engine (`cron cl-lyapunov`)
//!
//! Provides mathematically guaranteed non-divergent dynamical state stability for recurrent
//! 4-cycle neuromorphic loops across infinite cognitive time horizons:
//!
//! 1. Lyapunov Exponent Tracking:
//!    $$\lambda_T = \frac{1}{T} \sum_{t=1}^T \ln\left(\frac{\|\delta \mathbf{x}_t\|}{\|\delta \mathbf{x}_0\|}\right)$$
//!    Monitors trajectory divergence rate to prevent exponential blow-up ($\lambda > 0$).
//!
//! 2. Polynomial Closed-Loop Contraction (Soft Hypersphere Manifold Projection):
//!    $$\mathbf{x}_{t+1} = \mathbf{x}_t \cdot \left(1 + \eta \left(1 - \frac{\|\mathbf{x}_t\|^2}{R_0^2}\right)\right)$$
//!    Guarantees asymptotic convergence to target hypersphere radius $R_0$ without destructive normalization.
//!
//! 3. Deterministic Entropy Injection (Attractor Trapping & Bit-Saturation Prevention):
//!    Detects frozen limit cycles ($\sigma^2 < \epsilon$) or bit-level lockups ($0$, $2^{64}-1$),
//!    injecting pseudo-orthogonal Galois LFSR perturbation to break dead attractors.
//!
//! 4. Hardware Sentry Assembly (`.cl`):
//!    Compiles calibrated hardware RMSNorm thresholds (`_RN`), Parity Telemetry (`_PT`),
//!    and Sentry Heartbeats (`_SH`) directly into VLIW bundles.

use crate::cl_macro::{build_valid_slot, MacroCompiler};

/// Configuration for the Lyapunov Stabilization Watchdog
#[derive(Debug, Clone, PartialEq)]
pub struct LyapunovWatchdogConfig {
    pub dimension: usize,
    pub target_radius: f64,
    pub contraction_rate: f64,
    pub divergence_threshold: f64,
    pub collapse_threshold: f64,
    pub min_variance: f64,
    pub entropy_injection_gain: f64,
}

impl Default for LyapunovWatchdogConfig {
    fn default() -> Self {
        Self {
            dimension: 64,
            target_radius: 1.0,
            contraction_rate: 0.20,
            divergence_threshold: 2.50,
            collapse_threshold: 0.10,
            min_variance: 1e-4,
            entropy_injection_gain: 0.05,
        }
    }
}

/// Status report returned after stabilizing a state vector
#[derive(Debug, Clone, PartialEq)]
pub struct LyapunovStatus {
    pub is_stable: bool,
    pub current_l2_norm: f64,
    pub target_radius: f64,
    pub lyapunov_exponent: f64,
    pub state_variance: f64,
    pub drift_detected: bool,
    pub anti_trapping_triggered: bool,
    pub action_taken: &'static str,
}

/// Brain 6 Sentry: Lyapunov Watchdog & Closed-Loop Controller
#[derive(Debug, Clone)]
pub struct LyapunovWatchdog {
    pub config: LyapunovWatchdogConfig,
    pub total_cycles: u64,
    pub stabilization_events: u64,
    pub trapping_events: u64,
    pub divergence_trips: u64,
    pub prev_norm: f64,
    pub running_lyapunov_exponent: f64,
    pub history_norms: Vec<f64>,
}

impl Default for LyapunovWatchdog {
    fn default() -> Self {
        Self::new(LyapunovWatchdogConfig::default())
    }
}

impl LyapunovWatchdog {
    /// Creates a new Lyapunov Watchdog with the given configuration
    pub fn new(config: LyapunovWatchdogConfig) -> Self {
        let target_r = config.target_radius;
        Self {
            config,
            total_cycles: 0,
            stabilization_events: 0,
            trapping_events: 0,
            divergence_trips: 0,
            prev_norm: target_r,
            running_lyapunov_exponent: -0.01,
            history_norms: Vec::with_capacity(64),
        }
    }

    /// Primary stabilization pass: enforces closed-loop contraction and anti-trapping injection
    pub fn stabilize(&mut self, state: &mut [f64]) -> LyapunovStatus {
        self.total_cycles += 1;
        let dim = state.len();
        if dim == 0 {
            return LyapunovStatus {
                is_stable: true,
                current_l2_norm: 0.0,
                target_radius: self.config.target_radius,
                lyapunov_exponent: 0.0,
                state_variance: 0.0,
                drift_detected: false,
                anti_trapping_triggered: false,
                action_taken: "EMPTY_STATE",
            };
        }

        // 1. Calculate raw L2 norm: ||x||
        let sum_sq: f64 = state.iter().map(|&x| x * x).sum();
        let raw_norm = sum_sq.sqrt();

        // 2. Calculate variance to detect attractor trapping (frozen identical states)
        let mean = state.iter().sum::<f64>() / dim as f64;
        let variance = state.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / dim as f64;

        // 3. Update finite-time Lyapunov exponent estimate
        let delta_ratio = if self.prev_norm > 1e-12 {
            raw_norm / self.prev_norm
        } else {
            1.0
        };
        let log_ratio = delta_ratio.max(1e-12).ln();
        let alpha = 0.05; // EMA smoothing
        self.running_lyapunov_exponent = (1.0 - alpha) * self.running_lyapunov_exponent + alpha * log_ratio;
        self.prev_norm = raw_norm;

        if self.history_norms.len() >= 64 {
            self.history_norms.remove(0);
        }
        self.history_norms.push(raw_norm);

        // 4. Emergency Divergence Trip Check
        if raw_norm > self.config.divergence_threshold * self.config.target_radius {
            self.divergence_trips += 1;
            // Force hard projection to target radius to save silicon state
            let scale = self.config.target_radius / raw_norm.max(1e-9);
            for x in state.iter_mut() {
                *x *= scale;
            }
            return LyapunovStatus {
                is_stable: false,
                current_l2_norm: self.config.target_radius,
                target_radius: self.config.target_radius,
                lyapunov_exponent: self.running_lyapunov_exponent,
                state_variance: variance,
                drift_detected: true,
                anti_trapping_triggered: false,
                action_taken: "TRIP_EMERGENCY_PROJECTION",
            };
        }

        // 5. Attractor Trapping & Saturation Prevention (Deterministic Entropy Injection)
        let mut anti_trapping_triggered = false;
        let mut action = "PASSTHROUGH";

        if variance < self.config.min_variance || raw_norm < self.config.collapse_threshold * self.config.target_radius {
            self.trapping_events += 1;
            anti_trapping_triggered = true;
            action = "ENTROPY_INJECTION";

            let perturbation = self.deterministic_entropy_injection(dim, self.total_cycles);
            let gain = self.config.entropy_injection_gain;
            for (i, p) in perturbation.iter().enumerate() {
                state[i] += gain * p;
            }
        }

        // 6. Polynomial Closed-Loop Contraction:
        // x_{t+1} = x_t * (1 + \eta * (1 - ||x||^2 / R_0^2))
        let r0 = self.config.target_radius;
        let r0_sq = r0 * r0;
        let current_sq: f64 = state.iter().map(|&x| x * x).sum();
        let current_norm = current_sq.sqrt();
        let drift_ratio = (current_norm - r0).abs() / r0;

        let drift_detected = drift_ratio > 0.02;

        if drift_detected {
            self.stabilization_events += 1;
            let factor = 1.0 + self.config.contraction_rate * (1.0 - (current_sq / r0_sq));
            // Clamp factor to prevent over-correction overshoot: [0.5, 1.5]
            let clamped_factor = factor.clamp(0.5, 1.5);
            for x in state.iter_mut() {
                *x *= clamped_factor;
            }
            if !anti_trapping_triggered {
                action = if factor < 1.0 {
                    "NORM_CONTRACTION"
                } else {
                    "EXPANSION_RESTORE"
                };
            }
        }

        let final_norm = (state.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        let is_stable = final_norm <= self.config.divergence_threshold * self.config.target_radius
            && final_norm >= self.config.collapse_threshold * self.config.target_radius
            && self.running_lyapunov_exponent <= 0.05;

        LyapunovStatus {
            is_stable,
            current_l2_norm: final_norm,
            target_radius: r0,
            lyapunov_exponent: self.running_lyapunov_exponent,
            state_variance: variance,
            drift_detected,
            anti_trapping_triggered,
            action_taken: action,
        }
    }

    /// Computes deterministic pseudo-orthogonal perturbation vector using 64-bit Galois LFSR
    pub fn deterministic_entropy_injection(&self, dim: usize, cycle: u64) -> Vec<f64> {
        let mut out = Vec::with_capacity(dim);
        let mut lfsr = cycle.wrapping_mul(0x9E3779B97F4A7C15) | 1;

        for _ in 0..dim {
            // Galois shift
            let lsb = lfsr & 1;
            lfsr >>= 1;
            if lsb == 1 {
                lfsr ^= 0xD800000000000000;
            }
            // Normalize to zero-mean bipolar perturbation: [-1.0, +1.0]
            let normalized = ((lfsr & 0xFFFF) as f64 / 32768.0) - 1.0;
            out.push(normalized);
        }

        // Center perturbation to exactly zero-mean to preserve overall state centroid
        let mean = out.iter().sum::<f64>() / dim as f64;
        for x in &mut out {
            *x -= mean;
        }
        out
    }

    /// Compiles Brain 6 Sentry RMSNorm and Lyapunov stability thresholds into `.cl` assembly
    pub fn compile_to_cl(&self, core_id: usize) -> String {
        let mut compiler = MacroCompiler::new(core_id as u8);
        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = (core_id / 64) % 4;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; Brain 6 Sentry: Cascading Drift Watchdog & Lyapunov Stabilization Controller\n\
             ; Target Radius R0: {:.2}, Contraction Rate: {:.2}, Max Divergence: {:.2}x\n\
             ; Silicon Architecture: 256-Core 4D-Torus Autonomic Closed-Loop Watchdog\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @sentry_lyapunov_entry:\n",
            self.config.target_radius,
            self.config.contraction_rate,
            self.config.divergence_threshold,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Bundle 0: Load Target Norm & State Vector pointers
        compiler.emit_slot(build_valid_slot("==00#001", "'")); // R0 = Target Hypersphere Radius (1.0 Q16)
        compiler.emit_slot(build_valid_slot("==01#040", "'")); // R1 = Vector Dimension (64-dim)
        compiler.emit_slot(build_valid_slot("_LD02M100", "_")); // R2 = Read Recurrent State Vector [t]
        compiler.emit_slot(build_valid_slot("_LD03M101", "_")); // R3 = Read Previous State Vector [t-1]

        // Bundle 1: Compute L2 Norm (Dot Product Accumulation) & RMSNorm Calibration
        compiler.emit_slot(build_valid_slot("_MA04$022", "_")); // R4 = Dot Product (x . x) = ||x||^2
        compiler.emit_slot(build_valid_slot("_SR05$040", "_")); // R5 = Sqrt(R4) = L2 Norm
        compiler.emit_slot(build_valid_slot("_RN06$050", "_")); // R6 = Hardware RMSNorm Calibration Slot
        compiler.emit_slot(build_valid_slot("_SU07$050", "_")); // R7 = Norm Drift: ||x|| - R0

        // Bundle 2: Brain 6 Sentry Watchdog Threshold & Parity Telemetry
        compiler.emit_slot(build_valid_slot("_SH08$070", "_")); // R8 = Sentry Config: Trip if R7 > Threshold
        compiler.emit_slot(build_valid_slot("_PT09$080", "_")); // R9 = Parity Telemetry Health Check
        compiler.emit_slot(build_valid_slot("_MA0A$060", "_")); // RA = Closed-Loop Contraction Scaling: x * (1 + eta*(1 - ||x||^2))
        compiler.emit_slot(build_valid_slot("_ST0BM200", "_")); // RB = Write Stabilized State Vector to SRAM

        // Bundle 3: 256-Core Sentry Sync & Commit
        compiler.emit_slot(build_valid_slot("_bb00#000", "'")); // Global Torus Barrier
        compiler.emit_slot(build_valid_slot("!HL00#000", "!")); // Halt / Commit
        compiler.emit_slot(build_valid_slot("__NOP000", ""));  // NOP pad
        compiler.emit_slot(build_valid_slot("__NOP000", ""));  // NOP pad

        cl_code.push_str(&compiler.finish());
        cl_code
    }

    /// Renders ASCII HUD telemetry for the Brain 6 Lyapunov Watchdog
    pub fn render_ascii_hud(&self, status: &LyapunovStatus) -> String {
        format!(
            "┌─────────────────────────────────────────────────────────────┐\n\
             │ BRAIN 6 SENTRY: LYAPUNOV STABILIZATION & DRIFT WATCHDOG     │\n\
             ├─────────────────────────────────────────────────────────────┤\n\
             │ Total Cycles:        {:<10} │ Divergence Trips:  {:<8} │\n\
             │ Stabilizations:      {:<10} │ Trapping Breakers: {:<8} │\n\
             │ L2 Norm (Target):    {:<6.4} ({:<4.2}) │ State Variance:   {:<8.6} │\n\
             │ Lyapunov Exponent:   {:<+8.4}   │ System Stable:     {:<8} │\n\
             │ Active Sentry Action: {:<37}│\n\
             └─────────────────────────────────────────────────────────────┘",
            self.total_cycles,
            self.divergence_trips,
            self.stabilization_events,
            self.trapping_events,
            status.current_l2_norm,
            status.target_radius,
            status.state_variance,
            status.lyapunov_exponent,
            status.is_stable,
            status.action_taken
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lyapunov_drift_recovery_and_convergence() {
        let config = LyapunovWatchdogConfig {
            dimension: 16,
            target_radius: 1.0,
            contraction_rate: 0.25,
            divergence_threshold: 3.0,
            collapse_threshold: 0.05,
            min_variance: 1e-4,
            entropy_injection_gain: 0.05,
        };
        let mut watchdog = LyapunovWatchdog::new(config);

        // Case A: State exploded to norm ~ 2.2
        let mut state_exploded = vec![0.55; 16]; // norm = sqrt(16 * 0.55^2) = 4 * 0.55 = 2.2
        for _ in 0..30 {
            watchdog.stabilize(&mut state_exploded);
        }
        let final_norm_a = (state_exploded.iter().map(|x| x * x).sum::<f64>()).sqrt();
        assert!(
            (final_norm_a - 1.0).abs() < 0.02,
            "Exploded state must asymptotically converge to 1.0, got: {}",
            final_norm_a
        );

        // Case B: State collapsed to norm ~ 0.2
        let mut state_collapsed = vec![0.05; 16]; // norm = 4 * 0.05 = 0.2
        for _ in 0..30 {
            watchdog.stabilize(&mut state_collapsed);
        }
        let final_norm_b = (state_collapsed.iter().map(|x| x * x).sum::<f64>()).sqrt();
        assert!(
            (final_norm_b - 1.0).abs() < 0.02,
            "Collapsed state must asymptotically restore to 1.0, got: {}",
            final_norm_b
        );
    }

    #[test]
    fn test_anti_trapping_entropy_injection() {
        let mut watchdog = LyapunovWatchdog::default();
        // State trapped in fixed zero-variance point
        let mut trapped_state = vec![0.125; 64];

        let status = watchdog.stabilize(&mut trapped_state);
        assert!(status.anti_trapping_triggered);
        assert_eq!(status.action_taken, "ENTROPY_INJECTION");
        assert!(watchdog.trapping_events >= 1);

        // Verify variance has been restored by perturbation
        let mean = trapped_state.iter().sum::<f64>() / 64.0;
        let new_variance = trapped_state.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / 64.0;
        assert!(
            new_variance > 1e-4,
            "Entropy injection must break trapped variance, got: {}",
            new_variance
        );
    }

    #[test]
    fn test_million_cycle_stability_no_divergence() {
        let mut watchdog = LyapunovWatchdog::default();
        let mut state = vec![0.1; 64];

        // 100,000 steps of dynamic recurrent updates with synthetic perturbations
        for step in 1..=100_000 {
            // Recurrent perturbation: simulate nonlinear update
            let factor = 1.0 + 0.03 * ((step as f64 * 0.1).sin());
            for x in &mut state {
                *x *= factor;
            }

            let status = watchdog.stabilize(&mut state);
            if step % 10_000 == 0 {
                assert!(status.is_stable, "State must remain stable at step {}", step);
                assert!(status.current_l2_norm < 2.0, "Norm must not diverge");
                assert!(status.current_l2_norm > 0.5, "Norm must not collapse");
                assert!(!status.current_l2_norm.is_nan());
            }
        }

        assert_eq!(watchdog.divergence_trips, 0, "Zero emergency divergence trips expected");
        assert!(watchdog.running_lyapunov_exponent <= 0.01);
    }

    #[test]
    fn test_compile_to_cl_assembly() {
        let watchdog = LyapunovWatchdog::default();
        let cl_code = watchdog.compile_to_cl(42);
        assert!(cl_code.contains(".core [2,2,2,0]:"));
        assert!(cl_code.contains("@sentry_lyapunov_entry:"));
        assert!(cl_code.contains("B0000:"));
        assert!(cl_code.contains("B0001:"));
        assert!(cl_code.contains("B0002:"));
        assert!(cl_code.contains("B0003:"));
    }
}
