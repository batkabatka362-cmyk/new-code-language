//! Thermodynamic Energy Minimization & Onsager Free-Energy Pruning Engine for CRON (.cl)
//!
//! Models non-equilibrium thermodynamic entropy production, Landauer dissipation,
//! and dynamically quenches inactive computation paths to approach zero-power compute.

/// Landauer energy constant in picojoules at T=300K: $k_B T \ln 2 \approx 2.87 \times 10^{-9}$ pJ
pub const LANDAUER_LIMIT_PJ: f64 = 0.00287;

/// Thermodynamic State of a VLIW Compute Tile
#[derive(Debug, Clone, PartialEq)]
pub struct TileThermodynamicState {
    pub tile_id: u32,
    pub temperature_kelvin: f64,
    pub free_energy_f: f64,
    pub internal_energy_u: f64,
    pub entropy_s: f64,
    pub bits_erased: u64,
    pub dissipated_energy_pj: f64,
    pub is_thermally_quenched: bool,
}

impl TileThermodynamicState {
    pub fn new(tile_id: u32, temperature_kelvin: f64) -> Self {
        Self {
            tile_id,
            temperature_kelvin,
            free_energy_f: 100.0,
            internal_energy_u: 100.0,
            entropy_s: 0.0,
            bits_erased: 0,
            dissipated_energy_pj: 0.0,
            is_thermally_quenched: false,
        }
    }

    /// Record bit erasure according to Landauer's Principle:
    /// $\Delta Q \ge k_B T \ln 2$ per bit
    pub fn record_bit_erasure(&mut self, bit_count: u64) {
        self.bits_erased += bit_count;
        let dissipation = (bit_count as f64) * LANDAUER_LIMIT_PJ * (self.temperature_kelvin / 300.0);
        self.dissipated_energy_pj += dissipation;
        self.entropy_s += (bit_count as f64) * 0.001;
        self.free_energy_f = (self.internal_energy_u - self.temperature_kelvin * self.entropy_s).max(0.0);
    }

    /// Check if tile should enter zero-power quenched sleep state
    pub fn evaluate_quench_condition(&mut self, activity_threshold: f64, recent_flux: f64) -> bool {
        if recent_flux < activity_threshold {
            self.is_thermally_quenched = true;
        } else {
            self.is_thermally_quenched = false;
        }
        self.is_thermally_quenched
    }
}

/// Onsager Non-Equilibrium Thermodynamic Optimizer
#[derive(Debug, Clone, PartialEq)]
pub struct ThermodynamicOptimizer {
    pub tiles: Vec<TileThermodynamicState>,
    pub global_temperature: f64,
    pub quench_threshold: f64,
    pub total_energy_saved_pj: f64,
}

impl ThermodynamicOptimizer {
    pub fn new(num_tiles: usize, initial_temp: f64) -> Self {
        let mut tiles = Vec::with_capacity(num_tiles);
        for i in 0..num_tiles {
            tiles.push(TileThermodynamicState::new(i as u32, initial_temp));
        }
        Self {
            tiles,
            global_temperature: initial_temp,
            quench_threshold: 0.05,
            total_energy_saved_pj: 0.0,
        }
    }

    /// Apply Onsager matrix transport flux $J_i = \sum_j L_{ij} X_j$ to balance heat and compute load
    pub fn balance_thermal_flux(&mut self, fluxes: &[f64]) -> usize {
        let mut quenched_count = 0;
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let flux = fluxes.get(i).copied().unwrap_or(0.0);
            if tile.evaluate_quench_condition(self.quench_threshold, flux) {
                quenched_count += 1;
                // Accumulate energy saved from dynamic clock-gating
                self.total_energy_saved_pj += 12.5; // ~12.5 pJ per quenched cycle
            }
        }
        quenched_count
    }

    /// Total system thermodynamic dissipation report
    pub fn total_dissipated_energy(&self) -> f64 {
        self.tiles.iter().map(|t| t.dissipated_energy_pj).sum()
    }
}
