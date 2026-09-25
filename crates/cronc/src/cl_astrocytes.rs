//! Glial Tripartite Astrocytic Calcium Waves & Metabolic Coherence for SAGI AGI
//!
//! Models non-neuronal astrocyte glial cells that envelop synapses, propagate slow intercellular
//! calcium waves (0.1 - 2 Hz), regulate local metabolic ATP/glycogen budgets, and synchronize
//! phase-locked gamma/theta neural oscillations across the 4D-Torus.

/// Astrocytic Glial Cell State
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstrocyteCell {
    pub id: usize,
    /// Cytosolic Calcium concentration [Ca2+] (0.0 to 1.0)
    pub calcium_level: f32,
    /// Inositol trisphosphate (IP3) messenger concentration
    pub ip3_level: f32,
    /// Available metabolic energy budget (virtual ATP) (0.0 to 100.0)
    pub metabolic_atp: f32,
}

impl AstrocyteCell {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            calcium_level: 0.1,
            ip3_level: 0.05,
            metabolic_atp: 100.0,
        }
    }

    /// Step calcium dynamics based on neighboring synaptic glutamate release
    pub fn step_calcium_wave(&mut self, synaptic_glutamate_activity: f32, neighbor_ca: f32) -> f32 {
        // 1. Synaptic spillover triggers IP3 production
        self.ip3_level = (self.ip3_level + synaptic_glutamate_activity * 0.2).clamp(0.0, 1.0);

        // 2. IP3 triggers Calcium release from internal stores + neighbor gap junctions
        let ca_influx = self.ip3_level * 0.3 + neighbor_ca * 0.15;
        self.calcium_level = (self.calcium_level + ca_influx - 0.05).clamp(0.05, 1.0);

        // 3. Metabolic energy consumption
        self.metabolic_atp = (self.metabolic_atp - self.calcium_level * 2.0).clamp(10.0, 100.0);

        // 4. Slow exponential decay of IP3
        self.ip3_level *= 0.90;

        self.calcium_level
    }
}

/// 256-Astrocyte Glial Network covering the 4D-Torus Supercomputer
#[derive(Debug, Clone)]
pub struct GlialNetworkMesh {
    pub astrocytes: [AstrocyteCell; 256],
    pub global_gamma_coherence: f32,
    pub step_cycle: u64,
}

impl GlialNetworkMesh {
    pub fn new_256() -> Self {
        let mut astrocytes = [AstrocyteCell::new(0); 256];
        for i in 0..256 {
            astrocytes[i] = AstrocyteCell::new(i);
        }
        Self {
            astrocytes,
            global_gamma_coherence: 0.95,
            step_cycle: 0,
        }
    }

    /// Propagate calcium wave across 4D-Torus spatial neighbors
    pub fn step_mesh_dynamics(&mut self, core_activities: &[f32; 256]) -> f32 {
        self.step_cycle += 1;
        let mut total_ca = 0.0f32;

        let prev_ca: Vec<f32> = self.astrocytes.iter().map(|a| a.calcium_level).collect();

        for i in 0..256 {
            // Neighbor on X-axis in 4D torus
            let neighbor_idx = (i + 1) % 256;
            let neighbor_ca = prev_ca[neighbor_idx];
            let ca = self.astrocytes[i].step_calcium_wave(core_activities[i], neighbor_ca);
            total_ca += ca;
        }

        // Global Gamma phase coherence is modulated by mean calcium wave
        let avg_ca = total_ca / 256.0;
        self.global_gamma_coherence = (0.80 + avg_ca * 0.20).clamp(0.0, 1.0);
        self.global_gamma_coherence
    }

    /// Render Glial HUD
    pub fn render_ascii_hud(&self) -> String {
        let avg_atp: f32 = self.astrocytes.iter().map(|a| a.metabolic_atp).sum::<f32>() / 256.0;
        let avg_ca: f32 = self.astrocytes.iter().map(|a| a.calcium_level).sum::<f32>() / 256.0;
        format!(
            "+-------------------------------------------------------------------------+\n\
             | SAGI GLIAL ASTROCYTE TRIPARTITE & METABOLIC COHERENCE MESH              |\n\
             +-------------------------------------------------------------------------+\n\
             | Active Astrocyte Glial Cells:          256 / 256 Cores Enveloped        |\n\
             | Mean Intracellular Calcium [Ca2+]:     [{:<20}] {:.2} µM      |\n\
             | Mean Metabolic Glycogen/ATP Budget:    [{:<20}] {:.1} %       |\n\
             | Global Gamma Phase-Lock Coherence:     [{:<20}] {:.1} %       |\n\
             +-------------------------------------------------------------------------+\n",
            make_bar(avg_ca),
            avg_ca * 10.0,
            make_bar(avg_atp / 100.0),
            avg_atp,
            make_bar(self.global_gamma_coherence),
            self.global_gamma_coherence * 100.0,
        )
    }
}

fn make_bar(val: f32) -> String {
    let filled = ((val.clamp(0.0, 1.0) * 20.0).round() as usize).min(20);
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('#');
    }
    while bar.len() < 20 {
        bar.push('-');
    }
    bar
}
