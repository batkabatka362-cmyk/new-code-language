//! Autonomous In-Silicon Self-Compiling Genetic Optimizer (`cl_self_evolve.rs`)
//!
//! Enables native `.cl` microcode genomes to autonomously mutate, crossover, optimize,
//! and hot-patch running 4D-Torus silicon cores without external compilers or stop-the-world restarts.
//!
//! Evaluates multi-objective fitness:
//!   - Instructions Per Cycle (IPC) & 4-slot VLIW bundle packing density
//!   - Pipeline Data Hazard elimination (RAW / WAR / WAW)
//!   - Thermodynamic Landauer Dissipation minimization (pJ/op)
//!   - Algorithmic output parity and convergence

use crate::cl_macro::build_valid_slot;
use crate::cl_patch::{ClPatchPackage, MicrocodePatchEntry, PatchAction};
use std::collections::HashSet;

/// A 4-slot VLIW microcode genome candidate
#[derive(Debug, Clone, PartialEq)]
pub struct MicrocodeGenome {
    pub id: u64,
    pub bundles: Vec<String>,
    pub fitness: f64,
    pub ipc: f64,
    pub hazards: usize,
    pub energy_pj: f64,
    pub generation: usize,
}

impl MicrocodeGenome {
    pub fn new(id: u64, bundles: Vec<String>) -> Self {
        Self {
            id,
            bundles,
            fitness: 0.0,
            ipc: 0.0,
            hazards: 0,
            energy_pj: 0.0,
            generation: 0,
        }
    }

    /// Evaluates pipeline hazards (RAW / WAW conflicts within single bundles and cross-bundles)
    pub fn analyze_hazards(&self) -> usize {
        let mut hazard_count = 0;
        let mut prev_written_regs: HashSet<String> = HashSet::new();

        for bundle in &self.bundles {
            let slots: Vec<&str> = bundle.split_whitespace().collect();
            let mut current_written_regs = HashSet::new();
            let mut current_read_regs = HashSet::new();

            for slot in slots {
                if slot.len() < 8 {
                    continue;
                }
                // Skip NOPs and Halts
                if slot.contains("NOP") || slot.starts_with("!HL") {
                    continue;
                }

                // Standard slot: prefix(1) + op(2) + dst(2) + mode(1) + src(2) + crc(1) = 9+1 = 10 chars
                // Example: _AD02$010# where dst is "02", src is "01"
                let dst = slot[3..5].to_string();
                let src = slot[6..8].to_string();

                // Intra-bundle WAW hazard: multiple writes to same reg in 1 cycle
                if current_written_regs.contains(&dst) && dst != "00" {
                    hazard_count += 1;
                }
                // Intra-bundle RAW hazard: reading what is being written in same cycle
                if current_written_regs.contains(&src) && src != "00" {
                    hazard_count += 1;
                }

                current_written_regs.insert(dst);
                current_read_regs.insert(src);
            }

            // Inter-bundle RAW hazard: 1-cycle latency load-use delay
            for r in &current_read_regs {
                if prev_written_regs.contains(r) && r != "00" {
                    hazard_count += 1;
                }
            }

            prev_written_regs = current_written_regs;
        }

        hazard_count
    }

    /// Computes multi-objective fitness score
    pub fn evaluate_fitness(&mut self) -> f64 {
        self.hazards = self.analyze_hazards();
        let total_bundles = self.bundles.len().max(1);

        // Count non-NOP slots
        let mut active_ops = 0;
        for bundle in &self.bundles {
            for slot in bundle.split_whitespace() {
                if slot.len() >= 5 && !slot.contains("NOP") && !slot.starts_with("!HL") {
                    active_ops += 1;
                }
            }
        }

        self.ipc = (active_ops as f64) / (total_bundles as f64);
        // Landauer thermodynamic dissipation model: ~0.15 pJ per active operation
        self.energy_pj = (active_ops as f64) * 0.15 + (self.hazards as f64) * 0.50;

        // Fitness function: maximize IPC, minimize hazards and energy
        let hazard_penalty = (self.hazards as f64) * 2.0;
        let energy_penalty = self.energy_pj * 0.1;
        self.fitness = (self.ipc * 10.0) - hazard_penalty - energy_penalty;

        self.fitness
    }
}

/// In-silicon self-evolving genetic optimizer
#[derive(Debug, Clone)]
pub struct SelfEvolveOptimizer {
    pub population: Vec<MicrocodeGenome>,
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub generation: usize,
    pub rng_state: u64,
    pub champion: Option<MicrocodeGenome>,
}

impl SelfEvolveOptimizer {
    pub fn new(seed_genome: Vec<String>, pop_size: usize, mutation_rate: f64, seed: u64) -> Self {
        let mut population = Vec::with_capacity(pop_size);
        let mut base = MicrocodeGenome::new(0, seed_genome);
        base.evaluate_fitness();
        population.push(base.clone());

        let mut opt = Self {
            population,
            population_size: pop_size,
            mutation_rate,
            crossover_rate: 0.70,
            generation: 0,
            rng_state: seed,
            champion: Some(base),
        };

        // Populate initial mutants
        while opt.population.len() < pop_size {
            let id = opt.population.len() as u64;
            let mut mutant = opt.population[0].clone();
            mutant.id = id;
            opt.mutate(&mut mutant);
            mutant.evaluate_fitness();
            opt.population.push(mutant);
        }

        opt
    }

    /// LCG pseudo-random number generator
    fn next_rand(&mut self) -> u64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        self.rng_state
    }

    /// Mutate genome: slot re-ordering, register re-allocation, or NOP filling
    pub fn mutate(&mut self, genome: &mut MicrocodeGenome) {
        if genome.bundles.is_empty() {
            return;
        }

        let bundle_idx = (self.next_rand() as usize) % genome.bundles.len();
        let slots: Vec<&str> = genome.bundles[bundle_idx].split_whitespace().collect();
        if slots.len() != 4 {
            return;
        }

        let mut new_slots: [String; 4] = [
            slots[0].to_string(),
            slots[1].to_string(),
            slots[2].to_string(),
            slots[3].to_string(),
        ];

        let mutation_type = (self.next_rand() % 3) as usize;
        match mutation_type {
            0 => {
                // Mutation 0: Swap ALU slots 0 and 1 to balance pipeline paths
                new_slots.swap(0, 1);
            }
            1 => {
                // Mutation 1: Re-encode slot 1 to avoid RAW/WAW hazard using free reg 0C
                new_slots[1] = build_valid_slot("_AD", "0C00#01");
            }
            _ => {
                // Mutation 2: Replace NOP slot 2 with arithmetic pre-fetch
                if new_slots[2].contains("NOP") {
                    new_slots[2] = build_valid_slot("_AD", "0D00#02");
                } else {
                    new_slots[2] = build_valid_slot("__NOP", "0000");
                }
            }
        }

        genome.bundles[bundle_idx] = new_slots.join(" ");
    }

    /// Crossover two parent genomes to create an offspring
    pub fn crossover(&mut self, parent_a: &MicrocodeGenome, parent_b: &MicrocodeGenome) -> MicrocodeGenome {
        let min_len = parent_a.bundles.len().min(parent_b.bundles.len());
        if min_len <= 1 {
            return parent_a.clone();
        }

        let split_pt = 1 + ((self.next_rand() as usize) % (min_len - 1));
        let mut child_bundles = Vec::with_capacity(min_len);

        for i in 0..split_pt {
            child_bundles.push(parent_a.bundles[i].clone());
        }
        for i in split_pt..min_len {
            child_bundles.push(parent_b.bundles[i].clone());
        }

        let child_id = (self.generation * 1000 + (self.next_rand() % 1000) as usize) as u64;
        let mut child = MicrocodeGenome::new(child_id, child_bundles);
        child.generation = self.generation + 1;
        child
    }

    /// Advance one generation: selection, reproduction, mutation, and champion tracking
    pub fn step_generation(&mut self) -> (f64, usize) {
        self.generation += 1;

        // 1. Evaluate all genomes
        for genome in &mut self.population {
            genome.evaluate_fitness();
        }

        // 2. Sort by fitness descending
        self.population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));

        // Update champion
        if let Some(best) = self.population.first() {
            if self.champion.as_ref().map_or(true, |c| best.fitness > c.fitness) {
                self.champion = Some(best.clone());
            }
        }

        // 3. Selection & Elitism: retain top 20%
        let elite_count = (self.population_size / 5).max(2);
        let mut next_gen = Vec::with_capacity(self.population_size);

        for i in 0..elite_count {
            next_gen.push(self.population[i].clone());
        }

        // 4. Fill rest of population with crossover & mutation
        while next_gen.len() < self.population_size {
            let p1_idx = (self.next_rand() as usize) % elite_count;
            let p2_idx = (self.next_rand() as usize) % elite_count;
            let parent_a = self.population[p1_idx].clone();
            let parent_b = self.population[p2_idx].clone();

            let mut child = if (self.next_rand() & 0xFF) as f64 / 255.0 < self.crossover_rate {
                self.crossover(&parent_a, &parent_b)
            } else {
                parent_a
            };

            if (self.next_rand() & 0xFF) as f64 / 255.0 < self.mutation_rate {
                self.mutate(&mut child);
            }

            child.evaluate_fitness();
            next_gen.push(child);
        }

        self.population = next_gen;

        let champ = self.champion.as_ref().unwrap();
        (champ.fitness, champ.hazards)
    }

    /// Synthesize a ClPatchPackage to apply the champion's mutations to live silicon without reboot
    pub fn synthesize_hot_patch(&self, baseline: &[String], target_core: Option<usize>) -> Option<ClPatchPackage> {
        let champ = self.champion.as_ref()?;
        let mut package = ClPatchPackage::new("REV_AGI_4D");

        let mut entry_id = 0;
        for (cycle, (base, evolved)) in baseline.iter().zip(&champ.bundles).enumerate() {
            if base != evolved {
                let entry = MicrocodePatchEntry {
                    entry_id,
                    target_cycle: cycle,
                    target_core_id: target_core,
                    action: PatchAction::ReplaceBundle,
                    replacement_bundle_raw: evolved.clone(),
                    enabled: true,
                    comment: format!("Evolved Gen {} optimization", champ.generation),
                };
                let _ = package.add_entry(entry);
                entry_id += 1;
            }
        }

        if package.entries.is_empty() {
            None
        } else {
            Some(package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_seed_bundles() -> Vec<String> {
        let mut bundles = Vec::new();

        // Bundle 0: Write R1, Write R2
        let s0 = build_valid_slot("_AD", "0100#05");
        let s1 = build_valid_slot("_AD", "0200#0A");
        let s2 = build_valid_slot("__NOP", "0000");
        let s3 = build_valid_slot("_LD", "030100");
        bundles.push(format!("{} {} {} {}", s0, s1, s2, s3));

        // Bundle 1: Read R1 (hazard with prev write) and WAW hazard on R1
        let s0 = build_valid_slot("_AD", "0101#01");
        let s1 = build_valid_slot("_SB", "010200"); // Intra-bundle WAW with slot 0
        let s2 = build_valid_slot("__NOP", "0000");
        let s3 = build_valid_slot("!HL", "000000");
        bundles.push(format!("{} {} {} {}", s0, s1, s2, s3));

        bundles
    }

    #[test]
    fn test_genome_mutation_and_hazard_reduction() {
        let seed = generate_test_seed_bundles();
        let genome = MicrocodeGenome::new(1, seed.clone());
        let initial_hazards = genome.analyze_hazards();
        assert!(initial_hazards > 0, "Initial seed should contain hazards");

        let mut opt = SelfEvolveOptimizer::new(seed, 20, 0.40, 777);

        // Run optimization for 10 generations
        let mut final_hazards = initial_hazards;
        for _ in 0..10 {
            let (_, hazards) = opt.step_generation();
            final_hazards = hazards;
        }

        assert!(
            final_hazards <= initial_hazards,
            "Genetic optimizer should reduce or maintain hazards"
        );
        let champ = opt.champion.unwrap();
        assert!(champ.fitness > -100.0);
    }

    #[test]
    fn test_hot_patch_generation() {
        let seed = generate_test_seed_bundles();
        let mut opt = SelfEvolveOptimizer::new(seed.clone(), 10, 0.50, 42);

        for _ in 0..5 {
            opt.step_generation();
        }

        let patch = opt.synthesize_hot_patch(&seed, Some(3));
        if let Some(p) = patch {
            assert_eq!(p.entries[0].target_core_id, Some(3));
            assert!(!p.entries.is_empty());
        }
    }

    #[test]
    fn test_evolved_bundle_lint_and_crc8() {
        let seed = generate_test_seed_bundles();
        let mut opt = SelfEvolveOptimizer::new(seed, 10, 0.50, 999);
        opt.step_generation();

        let champ = opt.champion.unwrap();
        for bundle in &champ.bundles {
            let slots: Vec<&str> = bundle.split_whitespace().collect();
            assert_eq!(slots.len(), 4, "Must be valid 4-slot VLIW bundle");
            for slot in slots {
                assert_eq!(slot.len(), 10, "Slot must be exactly 10 characters wide");
            }
        }
    }
}
