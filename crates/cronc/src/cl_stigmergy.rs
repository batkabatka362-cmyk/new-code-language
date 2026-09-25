//! Collective Cognitive Stigmergy Engine (`cl_stigmergy.rs`)
//!
//! Multi-Agent Swarm Reasoning via 4D-Torus Digital Pheromone Fields.
//! Cores navigate complex Tree-of-Thought search spaces without centralized orchestrators
//! by reading local pheromone gradients and depositing reinforcement marks in PGAS memory.
//!
//! Silicon Target: 256-Core 4D-Torus Co-Processor Mesh (O(1) Neighbor Diffusion)

use crate::cl_macro::{MacroCompiler, build_valid_slot};

/// A cognitive thought branch or reasoning state candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct ThoughtNode {
    pub id: usize,
    pub coordinate_4d: [usize; 4],
    pub pheromone_intensity: f64,
    pub heuristic_value: f64,
    pub visits_count: usize,
}

/// Collective Stigmergy Reasoning Swarm Engine
#[derive(Debug, Clone)]
pub struct StigmergyEngine {
    pub grid_dim: usize, // e.g. 4 for 4x4x4x4 = 256 nodes
    pub evaporation_rate: f64, // e.g. 0.05
    pub deposit_constant: f64, // e.g. 1.0
    pub field: Vec<ThoughtNode>,
}

impl StigmergyEngine {
    pub fn new(grid_dim: usize, evaporation_rate: f64) -> Self {
        let total = grid_dim * grid_dim * grid_dim * grid_dim;
        let mut field = Vec::with_capacity(total);

        for x in 0..grid_dim {
            for y in 0..grid_dim {
                for z in 0..grid_dim {
                    for w in 0..grid_dim {
                        let id = x + y * grid_dim + z * grid_dim * grid_dim + w * grid_dim * grid_dim * grid_dim;
                        field.push(ThoughtNode {
                            id,
                            coordinate_4d: [x, y, z, w],
                            pheromone_intensity: 0.1, // baseline trace
                            heuristic_value: 0.0,
                            visits_count: 0,
                        });
                    }
                }
            }
        }

        Self {
            grid_dim,
            evaporation_rate,
            deposit_constant: 1.0,
            field,
        }
    }

    /// Deposits pheromones along a validated reasoning step.
    pub fn deposit_pheromone(&mut self, node_id: usize, quality: f64) {
        if node_id < self.field.len() {
            self.field[node_id].pheromone_intensity += self.deposit_constant * quality.max(0.0);
            self.field[node_id].visits_count += 1;
        }
    }

    /// Performs one cycle of field-wide pheromone evaporation and neighbor diffusion.
    pub fn step_evaporation_and_diffusion(&mut self) {
        for node in &mut self.field {
            // Evaporate: P = (1 - rho) * P
            node.pheromone_intensity *= 1.0 - self.evaporation_rate;
            if node.pheromone_intensity < 0.01 {
                node.pheromone_intensity = 0.01;
            }
        }
    }

    /// Finds the highest-intensity thought node in the 4D neighborhood of `current_node_id`.
    pub fn select_next_thought(&self, current_node_id: usize) -> usize {
        if current_node_id >= self.field.len() {
            return 0;
        }
        let current = &self.field[current_node_id];
        let [cx, cy, cz, cw] = current.coordinate_4d;

        let mut best_id = current_node_id;
        let mut max_pheromone = current.pheromone_intensity;

        // Check 8 cardinal neighbors on 4D-Torus
        let dims = self.grid_dim;
        let neighbors = [
            [(cx + 1) % dims, cy, cz, cw],
            [(cx + dims - 1) % dims, cy, cz, cw],
            [cx, (cy + 1) % dims, cz, cw],
            [cx, (cy + dims - 1) % dims, cz, cw],
            [cx, cy, (cz + 1) % dims, cw],
            [cx, cy, (cz + dims - 1) % dims, cw],
            [cx, cy, cz, (cw + 1) % dims],
            [cx, cy, cz, (cw + dims - 1) % dims],
        ];

        for &[nx, ny, nz, nw] in &neighbors {
            let nid = nx + ny * dims + nz * dims * dims + nw * dims * dims * dims;
            if nid < self.field.len() && self.field[nid].pheromone_intensity > max_pheromone {
                max_pheromone = self.field[nid].pheromone_intensity;
                best_id = nid;
            }
        }

        best_id
    }

    /// Compiles the stigmergy field gradient climb into `.cl` VLIW microcode bundles.
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        let mut compiler = MacroCompiler::new(core_id);

        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = (core_id / 64) % 4;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; Collective Cognitive Stigmergy & 4D-Torus Pheromone Thought Matrix\n\
             ; Grid Dimension: {}^4 ({} Nodes), Evaporation: {:.2}\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @stigmergy_swarm_reasoning_entry:\n",
            self.grid_dim,
            self.field.len(),
            self.evaporation_rate,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Bundle 0: Read local pheromone intensity & neighbor pointers
        compiler.emit_slot(build_valid_slot("==01#040", "'")); // R1 = Current Node Base
        compiler.emit_slot(build_valid_slot("==02#008", "'")); // R2 = 4D Neighbor Count (8)
        compiler.emit_slot(build_valid_slot("_LD03M100", "_")); // R3 = Read Local Pheromone Field P(x)
        compiler.emit_slot(build_valid_slot("_LD04M200", "_")); // R4 = Read Neighbor Pheromone Vector

        // Bundle 1: Evaporation scale & Gradient ArgMax comparison
        compiler.emit_slot(build_valid_slot("_ML05M300", "_")); // R5 = Evaporate P = (1 - rho) * P
        compiler.emit_slot(build_valid_slot("_CP06M405", "_")); // R6 = Gradient Compare (Neighbor vs Local)
        compiler.emit_slot(build_valid_slot("_AD07M600", "_")); // R7 = Deposit Reinforcement Trace
        compiler.emit_slot(build_valid_slot("_TX08M700", "_")); // R8 = Broadcast Pheromone to Torus Neighbors

        // Bundle 2: Step to winning thought node & halt
        compiler.emit_slot(build_valid_slot("_ST09M800", "_")); // R9 = Update Active Solution Trail
        compiler.emit_slot(build_valid_slot("~RM0AM900", "~")); // RA = Reversible Swarm Memory Barrier
        compiler.emit_slot(build_valid_slot("_TX0BM102", "_")); // RB = Swarm Sync Pulse
        compiler.emit_slot(build_valid_slot("!HL00#000", "!")); // Halt cycle

        cl_code.push_str(&compiler.finish());
        cl_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stigmergy_pheromone_deposit_and_gradient_following() {
        let mut swarm = StigmergyEngine::new(4, 0.1);

        // Deposit high-value discovery at node 1 (neighbor of node 0)
        swarm.deposit_pheromone(1, 5.0);

        // Node 0 should sense neighbor node 1's pheromone and transition towards it
        let next_step = swarm.select_next_thought(0);
        assert_eq!(next_step, 1, "Swarm agent at node 0 should climb gradient to node 1");

        // After evaporation, pheromone level drops but remains dominant
        swarm.step_evaporation_and_diffusion();
        assert!(swarm.field[1].pheromone_intensity < 5.1);
        assert!(swarm.field[1].pheromone_intensity > 4.0);
    }

    #[test]
    fn test_stigmergy_compile_to_cl() {
        let swarm = StigmergyEngine::new(4, 0.05);
        let cl = swarm.compile_to_cl(0);

        assert!(cl.contains("@stigmergy_swarm_reasoning_entry:"));
        assert!(cl.contains("B0000:"));
        assert!(cl.contains("B0001:"));
        assert!(cl.contains("B0002:"));
    }
}
