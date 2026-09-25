//! `cl_epigenetic_morph.rs` - DNA-like Self-Morphing Epigenetic Micro-Code Engine
//!
//! Provides biological axon myelination and in-place micro-code plasticity:
//! 1. Epigenetic Myelin Tokens: Frequently executed VLIW bundles accumulate "myelin" insulation,
//!    reducing propagation latency from 4 cycles down to 1 cycle.
//! 2. Autonomous Slot Fusing: Over-activated bundles physically mutate their 4 slots into fused
//!    zero-bubble single-cycle instructions without stopping the core.
//! 3. Synaptic Fatigue & Quiescence: Unused bundles slowly un-myelinate (demethylation) to free up
//!    thermal energy and SRAM bandwidth.

use std::collections::HashMap;

/// State of Axonal Myelination for a specific VLIW instruction address
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxonMyelinationState {
    pub bundle_address: u32,
    pub execution_count: u64,
    pub myelin_thickness_pct: f32, // 0.0% (raw unmyelinated) to 100.0% (fully insulated)
    pub effective_latency_cycles: u8, // 4 -> 3 -> 2 -> 1 cycle
    pub is_fused_macro: bool,
}

impl AxonMyelinationState {
    pub fn new(address: u32) -> Self {
        Self {
            bundle_address: address,
            execution_count: 0,
            myelin_thickness_pct: 0.0,
            effective_latency_cycles: 4, // Unmyelinated axon baseline
            is_fused_macro: false,
        }
    }

    /// Record a single execution of this instruction bundle and thicken myelin sheath
    pub fn record_activation(&mut self) -> bool {
        self.execution_count += 1;

        // Progressive biological myelination curve
        if self.execution_count >= 10 && self.myelin_thickness_pct < 25.0 {
            self.myelin_thickness_pct = 25.0;
            self.effective_latency_cycles = 3;
        } else if self.execution_count >= 50 && self.myelin_thickness_pct < 50.0 {
            self.myelin_thickness_pct = 50.0;
            self.effective_latency_cycles = 2;
        } else if self.execution_count >= 100 && self.myelin_thickness_pct < 100.0 {
            self.myelin_thickness_pct = 100.0;
            self.effective_latency_cycles = 1; // Pure 1-cycle superconductor speed
            self.is_fused_macro = true;
            return true; // Triggered physical in-place micro-mutation
        }

        false
    }

    /// Cool down unused axons (Synaptic Demethylation / Pruning)
    pub fn decay_myelin(&mut self, decay_pct: f32) {
        self.myelin_thickness_pct = (self.myelin_thickness_pct - decay_pct).max(0.0);
        if self.myelin_thickness_pct < 50.0 {
            self.effective_latency_cycles = if self.myelin_thickness_pct < 25.0 { 4 } else { 3 };
            self.is_fused_macro = false;
        }
    }
}

/// The Epigenetic Micro-Code Self-Morphing Engine
#[derive(Debug, Clone)]
pub struct EpigeneticMorphEngine {
    pub axon_registry: HashMap<u32, AxonMyelinationState>,
    pub total_mutated_macros: usize,
    pub cumulative_cycles_saved: u64,
}

impl Default for EpigeneticMorphEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EpigeneticMorphEngine {
    pub fn new() -> Self {
        Self {
            axon_registry: HashMap::new(),
            total_mutated_macros: 0,
            cumulative_cycles_saved: 0,
        }
    }

    /// Step an instruction bundle execution and calculate latency speedup
    pub fn step_bundle(&mut self, address: u32) -> u8 {
        let axon = self
            .axon_registry
            .entry(address)
            .or_insert_with(|| AxonMyelinationState::new(address));

        let mutated = axon.record_activation();
        if mutated {
            self.total_mutated_macros += 1;
        }

        let saved = (4 - axon.effective_latency_cycles) as u64;
        self.cumulative_cycles_saved += saved;
        axon.effective_latency_cycles
    }

    /// Get current myelination percentage for a bundle
    pub fn get_myelin_pct(&self, address: u32) -> f32 {
        self.axon_registry
            .get(&address)
            .map(|a| a.myelin_thickness_pct)
            .unwrap_or(0.0)
    }
}
