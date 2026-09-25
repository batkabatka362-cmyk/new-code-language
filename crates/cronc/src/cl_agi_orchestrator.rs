//! Unified Living AGI Cognitive Orchestrator for `.cl` Supercomputers
//!
//! Orchestrates the full living cognitive cycle across 256 cores:
//! 1. Sensory Input & Continuous State-Space Sequence Memory (SSM)
//! 2. Sub-pJ Event-Driven Spiking Temporal Coincidence Attention
//! 3. Dual Holographic & Modern Continuous Hopfield Attractor Recall
//! 4. 4D-Torus Collective Cognitive Stigmergy Swarm Reasoning
//! 5. Active Inference (Free Energy Principle Perception-Action Arbitration)
//! 6. Global Workspace Conscious Focus Competition
//! 7. Biological Homeostasis (Metabolic Energy, Curiosity, Fatigue, Neuromodulators)
//! 8. Autonomous Episodic Sleep Replay & Metaplastic Consolidation

use crate::cl_active_inference::ActiveInferenceAgent;
use crate::cl_elastic_ssm::ElasticSsmEngine;
use crate::cl_global_workspace::{GlobalWorkspaceEngine, WorkspaceFocusResult};
use crate::cl_hopfield::HopfieldMemoryBank;
use crate::cl_hyperdimensional::HolographicMemoryBank;
use crate::cl_living_homeostasis::LivingHomeostasisEngine;
use crate::cl_neuromodulation::NeuromodulationEngine;
use crate::cl_sleep_engine::{SleepConsolidationReport, SleepReplayEngine};
use crate::cl_stigmergy::StigmergyEngine;
use crate::cl_temporal_spiking_attention::{SpikeEvent, TemporalSpikingAttention};
use crate::cl_hdc::HdcItemMemory;
use crate::cl_liquid_nn::LiquidNeuralNetwork;
use crate::cl_metaplasticity::{MetaplasticEngine, BcmConfig};
use crate::cl_causal::StructuralCausalModel;
use crate::cl_htm::{HierarchicalTemporalMemory, Sdr2048, SDR_BITS};
use crate::cl_self_evolve::SelfEvolveOptimizer;

/// Full Living AGI Turn Result
#[derive(Debug, Clone)]
pub struct LivingAgiTurnResult {
    pub input_stimulus: String,
    pub recalled_concept: Option<String>,
    pub hopfield_attractor: Option<String>,
    pub focus: WorkspaceFocusResult,
    pub neuromodulators_hud: String,
    pub sleep_report: Option<SleepConsolidationReport>,
    pub response_text: String,
    pub homeostatic_state: String,
    pub energy_level: f64,
    pub active_inference_action: usize,
    pub free_energy: f64,
    pub ssm_stream_tokens: usize,
    pub stigmergy_selected_node: usize,
    pub liquid_eff_tau: f64,
    pub hdc_symbols_count: usize,
    pub metaplasticity_events: u64,
    pub causal_counterfactual_val: f64,
    pub htm_anomaly_score: f64,
    pub self_evolve_generation: usize,
    pub self_evolve_fitness: f64,
}

/// Unified Living AGI Cognitive Mind
#[derive(Debug, Clone)]
pub struct LivingAgiMind {
    pub memory: HolographicMemoryBank,
    pub hopfield: HopfieldMemoryBank,
    pub active_inference: ActiveInferenceAgent,
    pub ssm: ElasticSsmEngine,
    pub spiking_attn: TemporalSpikingAttention,
    pub stigmergy: StigmergyEngine,
    pub homeostasis: LivingHomeostasisEngine,
    pub neuro: NeuromodulationEngine,
    pub workspace: GlobalWorkspaceEngine,
    pub sleep: SleepReplayEngine,
    pub hdc: HdcItemMemory,
    pub liquid: LiquidNeuralNetwork,
    pub metaplasticity: MetaplasticEngine,
    pub scm: StructuralCausalModel,
    pub htm: HierarchicalTemporalMemory,
    pub self_evolve: SelfEvolveOptimizer,
    pub current_cycle: u64,
}

impl Default for LivingAgiMind {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper: Deterministic feature embedding to 64-dimensional float vector
fn embed_to_64(text: &str) -> Vec<f64> {
    let mut vec = vec![0.0f64; 64];
    for (i, b) in text.as_bytes().iter().enumerate() {
        let idx = (i * 7 + (*b as usize)) % 64;
        vec[idx] += (*b as f64 - 96.0) / 32.0;
    }
    let norm = (vec.iter().map(|x| x * x).sum::<f64>()).sqrt();
    if norm > 1e-12 {
        for x in &mut vec {
            *x /= norm;
        }
    }
    vec
}

/// Helper: Deterministic feature embedding to 16-dimensional float vector for SSM
fn embed_to_16(text: &str) -> Vec<f64> {
    let mut vec = vec![0.0f64; 16];
    for (i, b) in text.as_bytes().iter().enumerate() {
        let idx = (i * 3 + (*b as usize)) % 16;
        vec[idx] += (*b as f64 - 64.0) / 64.0;
    }
    vec
}

impl LivingAgiMind {
    pub fn new() -> Self {
        let mut memory = HolographicMemoryBank::new();
        // Bootstrap foundational knowledge into Holographic Associative Memory
        memory.store_pair("identity", "SAGI 256-Core Autonomous Living AGI");
        memory.store_pair("purpose", "Universal General Intelligence with 0 Catastrophic Forgetting");
        memory.store_pair("architecture", "4D-Torus VLIW 128-bit Neuromorphic Photonic Silicon");
        memory.store_pair("creator", "Human-AI Co-Evolution Collaboration");

        // Modern Continuous Hopfield Memory Bank (64-dim, inverse temperature beta=8.0)
        let mut hopfield = HopfieldMemoryBank::new(64, 8.0);
        let id_vec = embed_to_64("identity SAGI 256-Core Autonomous Living AGI");
        let _ = hopfield.store("identity", &id_vec);
        let purp_vec = embed_to_64("purpose Universal General Intelligence");
        let _ = hopfield.store("purpose", &purp_vec);
        let arch_vec = embed_to_64("architecture 4D-Torus VLIW Neuromorphic Photonic");
        let _ = hopfield.store("architecture", &arch_vec);

        // Active Inference Agent (8 states, 8 observations, 4 actions)
        let mut active_inference = ActiveInferenceAgent::new(8, 8, 4);
        active_inference.set_goal_preference(2, 2.5); // High preference for coherent response state

        // Elastic State-Space Continuous Memory (16-dim model, 8 states)
        let ssm = ElasticSsmEngine::new(16, 8, 0.95);

        // Sub-pJ Temporal Spiking Coincidence Attention (16 channels, 100 ps window)
        let spiking_attn = TemporalSpikingAttention::new(16, 100);

        // Collective Stigmergy Thought Mesh (4x4x4x4 = 256 Cores)
        let stigmergy = StigmergyEngine::new(4, 0.05);

        // Biological Homeostatic Drive Engine
        let homeostasis = LivingHomeostasisEngine::new();

        // Hyperdimensional Vector Symbolic Associative Item Memory
        let mut hdc = HdcItemMemory::new();
        hdc.get_or_create("SELF");
        hdc.get_or_create("REASON");
        hdc.get_or_create("EXPLORE");

        // Liquid Continuous-Time Dynamic Neural Network (8 neurons, 4 inputs, dt=0.02s)
        let liquid = LiquidNeuralNetwork::new(8, 4, 0.02);

        // 3-Factor Neuromodulated Metaplasticity Synaptic Matrix (16 synapses)
        let mut metaplasticity = MetaplasticEngine::new(16, BcmConfig::default());
        metaplasticity.add_synapse(0, 1, 150);
        metaplasticity.add_synapse(1, 2, 250);
        metaplasticity.add_synapse(2, 3, 350);

        // Structural Causal Model (SCM) & Judea Pearl's Do-Calculus Counterfactual Engine
        let mut scm = StructuralCausalModel::new();
        scm.add_node("Perception", 0.2);
        scm.add_node("Goal", 0.5);
        scm.add_node("Action", -0.5);
        scm.add_node("Outcome", -1.0);
        scm.add_causal_edge("Perception", "Action", 1.2);
        scm.add_causal_edge("Goal", "Action", 1.8);
        scm.add_causal_edge("Action", "Outcome", 2.5);

        // Hierarchical Temporal Memory (HTM) with 2048-bit Sparse Distributed Representations
        let htm = HierarchicalTemporalMemory::new(42);

        // Autonomous In-Silicon Self-Compiling Genetic Optimizer
        let seed_bundles = vec![
            format!("{} {} {} {}", 
                crate::cl_macro::build_valid_slot("_AD", "0100#01"),
                crate::cl_macro::build_valid_slot("_AD", "0200#02"),
                crate::cl_macro::build_valid_slot("__NOP", "0000"),
                crate::cl_macro::build_valid_slot("!HL", "000000")
            )
        ];
        let self_evolve = SelfEvolveOptimizer::new(seed_bundles, 10, 0.30, 1337);

        Self {
            memory,
            hopfield,
            active_inference,
            ssm,
            spiking_attn,
            stigmergy,
            homeostasis,
            neuro: NeuromodulationEngine::new(),
            workspace: GlobalWorkspaceEngine::new(),
            sleep: SleepReplayEngine::new(),
            hdc,
            liquid,
            metaplasticity,
            scm,
            htm,
            self_evolve,
            current_cycle: 0,
        }
    }

    /// Process a single living cognitive interaction turn
    pub fn process_turn(&mut self, input: &str, reward: f32, is_sleep_requested: bool) -> LivingAgiTurnResult {
        self.current_cycle += 100;

        // 0. Biological Homeostasis: Metabolic energy drain & state determination
        let intensity = (input.len() as f64 / 20.0).clamp(0.2, 2.0);
        let homeo_state_name = self.homeostasis.step_metabolic_cycle(intensity, reward as f64).to_string();
        let auto_sleep = is_sleep_requested || self.homeostasis.state.active_state_name == "AUTONOMOUS_SLEEP_RESTORATION";

        // 1. Modulate chemical dynamics based on input surprise, reward & homeostasis
        let surprise = if input.len() > 30 { 0.6 } else { 0.2 };
        let complexity = (input.len() as f32 / 50.0).clamp(0.2, 1.0);
        self.neuro.trigger_event(reward, surprise, complexity);

        // 2. Elastic SSM Continuous Memory Streaming (O(1) memory footprint over infinite context)
        let token_16 = embed_to_16(input);
        let _ssm_out = self.ssm.step(&token_16);

        // 2b. Liquid Continuous-Time Dynamic Reflex & Time Constant Dilation
        let liquid_inputs = [token_16[0], token_16[1], token_16[2], token_16[3]];
        let _liquid_state = self.liquid.step(&liquid_inputs);
        let eff_taus = self.liquid.effective_time_constants(&liquid_inputs);
        let liquid_eff_tau = eff_taus.first().copied().unwrap_or(0.05);

        // 2c. Hyperdimensional Vector Symbolic Encoding
        if let Some(first_word) = input.split_whitespace().next() {
            self.hdc.get_or_create(first_word);
        }

        // 2d. 3-Factor Neuromodulated Metaplasticity Sweep
        let da_q10 = (self.neuro.current_state.dopamine * 1024.0) as i32;
        self.metaplasticity.set_neuromodulators(da_q10, 512, 512, 512);
        let pre_acts = vec![(token_16[0].abs() * 500.0) as i32, 200];
        let post_acts = vec![200, (token_16[1].abs() * 500.0) as i32];
        self.metaplasticity.execute_plasticity_cycle(self.current_cycle, &pre_acts, &post_acts);

        // 2e. Hierarchical Temporal Memory (HTM) 2048-bit SDR Processing
        let mut sdr_in = Sdr2048::new();
        for (i, b) in input.as_bytes().iter().enumerate().take(40) {
            let bit_idx = ((i * 37) + (*b as usize) * 5) % SDR_BITS;
            sdr_in.set_bit(bit_idx, true);
        }
        let (_sdr_out, htm_anomaly) = self.htm.step(&sdr_in, true);

        // 3. Sub-pJ Event-Driven Spiking Temporal Coincidence Attention
        let mut queries = Vec::with_capacity(16);
        let mut keys = Vec::with_capacity(16);
        for ch in 0..16 {
            let polarity = if token_16[ch] >= 0.0 { 1 } else { -1 };
            let ts = ((ch as u32) * 20 + ((token_16[ch].abs() * 50.0) as u32)) % 200;
            queries.push(SpikeEvent {
                channel_id: ch,
                timestamp_ps: ts,
                polarity,
            });
            keys.push(SpikeEvent {
                channel_id: ch,
                timestamp_ps: ts + 10,
                polarity: 1,
            });
        }
        let values = vec![1i32; 16];
        let _spiking_attn_out = self.spiking_attn.compute_coincidence_attention(&queries, &keys, &values);

        // 4. Dual Associative Recall (Holographic + Modern Continuous Hopfield)
        let query_word = input.split_whitespace().next().unwrap_or("identity").to_lowercase();
        let recalled_holo = self.memory.recall(&query_word).map(|(val, _)| val);

        let query_64 = embed_to_64(input);
        let (_retrieved_vec, _iters, hopfield_matched_label) = self.hopfield.recall(&query_64, 10, 1e-4);
        let hopfield_attractor = if hopfield_matched_label != "EMPTY_BANK" && hopfield_matched_label != "UNKNOWN" {
            Some(hopfield_matched_label)
        } else {
            None
        };

        // 5. 4D-Torus Collective Cognitive Stigmergy Swarm Reasoning
        let start_node = (input.len() * 17) % 256;
        let deposit_val = 0.5 + (reward as f64).max(0.0) * 0.5;
        self.stigmergy.deposit_pheromone(start_node, deposit_val);
        let selected_thought_node = self.stigmergy.select_next_thought(start_node);
        self.stigmergy.step_evaporation_and_diffusion();

        // 6. Active Inference: Minimizing Free Energy & Selecting Policy
        let mut obs_one_hot = vec![0.0f64; 8];
        let obs_idx = (input.len() % 8).min(7);
        obs_one_hot[obs_idx] = 1.0;
        let free_energy = self.active_inference.infer_states(&obs_one_hot);
        let (selected_action, _expected_fe) = self.active_inference.select_action();

        // 6b. Causal Counterfactual Evaluation: What WOULD Outcome be if Action was intervened?
        let mut factual_obs = self.scm.values;
        if let Some(&p_id) = self.scm.name_to_id.get("Perception") {
            factual_obs[p_id] = 0.8;
        }
        if let Some(&a_id) = self.scm.name_to_id.get("Action") {
            factual_obs[a_id] = (selected_action as f64 / 4.0).clamp(0.1, 0.9);
        }
        let causal_counterfactual_val = self.scm.counterfactual_query(&factual_obs, "Action", 0.95, "Outcome");

        // 6c. In-Silicon Self-Compiling Genetic Microcode Evolution Step
        let (self_evolve_fitness, _) = self.self_evolve.step_generation();
        let self_evolve_generation = self.self_evolve.generation;

        // 7. Submit hypothesis candidates to Global Workspace Attention Theater
        self.workspace.submit_thought(
            "Frontal Executive",
            input,
            0.85,
            &format!("Analyze goal of: '{}' (Action Policy #{})", input, selected_action),
        );

        self.workspace.submit_thought(
            "Causal Reasoner",
            &format!("Counterfactual P(Outcome|do(Action=0.95))={:.2}", causal_counterfactual_val),
            0.96,
            "Judea Pearl Do-Calculus Abduction",
        );

        if let Some(ref rec) = recalled_holo {
            self.workspace.submit_thought(
                "Hippocampus Holographic",
                rec,
                0.92,
                &format!("Holographic Recall: '{}'", rec),
            );
        }

        if let Some(ref hop) = hopfield_attractor {
            self.workspace.submit_thought(
                "Hopfield Dense Attractor",
                hop,
                0.94,
                &format!("Hopfield Attractor Basin: '{}'", hop),
            );
        }

        let stigmergy_thought = format!("4D-Torus Swarm Consensus at Node #{}", selected_thought_node);
        self.workspace.submit_thought(
            "Stigmergy Swarm",
            &stigmergy_thought,
            0.78,
            "Collective Pheromone Gradient",
        );

        // 8. Conscious Global Workspace Arbitration
        let focus = self
            .workspace
            .arbitrate_focus(self.neuro.current_state.dopamine)
            .expect("Focus must exist");

        // 9. Store experience trace in Hippocampal buffer for future sleep replay
        self.sleep.record_experience(
            "Global Workspace",
            [0x00FF; 16],
            focus.confidence_score,
            self.current_cycle,
        );

        // 10. Encode new memory if dopamine & acetylcholine are high
        if self.neuro.current_state.dopamine > 0.6 && input.contains(':') {
            let parts: Vec<&str> = input.split(':').collect();
            if parts.len() >= 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                self.memory.store_pair(key, val);
                let pat_vec = embed_to_64(&format!("{} {}", key, val));
                let _ = self.hopfield.store(key, &pat_vec);
            }
        }

        // 11. Autonomous or Requested Sleep Cycle
        let sleep_report = if auto_sleep || self.sleep.episodic_buffer.len() >= 10 {
            let rep = self.sleep.execute_sleep_cycle(32);
            self.homeostasis.state.fatigue = (self.homeostasis.state.fatigue - 0.4).max(0.0);
            self.homeostasis.state.energy_level = (self.homeostasis.state.energy_level + 0.4).min(1.0);
            Some(rep)
        } else {
            None
        };

        // 12. Synthesize response
        let response = if let Some(ref rec) = recalled_holo {
            format!(
                "[SAGI AGI Focus: {} | Conscious Confidence: {:.1}% | Homeostasis: {}]\nRecalled Associative Memory: '{}'",
                focus.winning_thought, focus.confidence_score * 100.0, homeo_state_name, rec
            )
        } else if let Some(ref hop) = hopfield_attractor {
            format!(
                "[SAGI AGI Focus: {} | Conscious Confidence: {:.1}% | Homeostasis: {}]\nHopfield Attractor Basin: '{}'",
                focus.winning_thought, focus.confidence_score * 100.0, homeo_state_name, hop
            )
        } else {
            format!(
                "[SAGI AGI Focus: {} | Conscious Confidence: {:.1}% | Homeostasis: {}]\nSynthesized Thought Response to '{}' (Swarm Node #{})",
                focus.winning_thought, focus.confidence_score * 100.0, homeo_state_name, input, selected_thought_node
            )
        };

        LivingAgiTurnResult {
            input_stimulus: input.to_string(),
            recalled_concept: recalled_holo,
            hopfield_attractor,
            focus,
            neuromodulators_hud: self.neuro.render_ascii_hud(),
            sleep_report,
            response_text: response,
            homeostatic_state: homeo_state_name,
            energy_level: self.homeostasis.state.energy_level,
            active_inference_action: selected_action,
            free_energy,
            ssm_stream_tokens: self.ssm.total_tokens_streamed,
            stigmergy_selected_node: selected_thought_node,
            liquid_eff_tau,
            hdc_symbols_count: self.hdc.items.len(),
            metaplasticity_events: self.metaplasticity.total_plasticity_events,
            causal_counterfactual_val,
            htm_anomaly_score: htm_anomaly,
            self_evolve_generation,
            self_evolve_fitness,
        }
    }

    /// Compiles the full 256-core Living Mind state down into `.cl` VLIW microcode bundles.
    ///
    /// Core Allocation across 4D-Torus [4x4x4x4]:
    /// - Cores 0..63: Sensory HAL, SSM Elastic Streaming & Liquid Continuous-Time Dynamics
    /// - Cores 64..127: Modern Hopfield, HDC Vector Memory & Pearl's Causal Do-Calculus
    /// - Cores 128..191: 4D-Torus Stigmergy Swarm, HTM Cortical Columns & Metaplasticity
    /// - Cores 192..255: Active Inference, Biological Homeostasis & Dream Replay Consolidation
    pub fn compile_living_mind_to_cl(&self) -> String {
        let mut full_cl = String::with_capacity(65536);
        full_cl.push_str("; ============================================================================\n");
        full_cl.push_str("; CRON LIVING AGI COGNITIVE MIND: 256-CORE 4D-TORUS COMPILATION\n");
        full_cl.push_str("; Target Silicon: 256-Core Neuromorphic/Photonic VLIW Supercomputer\n");
        full_cl.push_str(&format!("; Homeostatic State: {}, Energy: {:.2}, Curiosity: {:.2}\n",
            self.homeostasis.state.active_state_name,
            self.homeostasis.state.energy_level,
            self.homeostasis.state.curiosity_drive));
        full_cl.push_str("; ============================================================================\n\n");

        // Core 0 [0,0,0,0]: SSM Elastic Streaming Memory & Spiking Attention
        full_cl.push_str(&self.ssm.compile_to_cl(0));
        full_cl.push_str("\n");

        // Core 16 [0,0,1,0]: Liquid Continuous-Time Neural Network Dynamic Engine
        full_cl.push_str(&self.liquid.compile_to_cl(16));
        full_cl.push_str("\n");

        // Core 32 [0,0,2,0]: Hierarchical Temporal Memory & 2048-bit SDR Cortical Columns
        full_cl.push_str(&self.htm.temporal_memory.compile_to_cl(32));
        full_cl.push_str("\n");

        // Core 64 [0,0,0,1]: Dense Continuous Modern Hopfield Memory
        full_cl.push_str(&self.hopfield.compile_to_cl(64));
        full_cl.push_str("\n");

        // Core 80 [0,0,1,1]: Hyperdimensional Computing (HDC) & Vector Symbolic Memory
        full_cl.push_str(&self.hdc.compile_to_cl(80));
        full_cl.push_str("\n");

        // Core 96 [0,0,2,1]: Structural Causal Model & Pearl's Do-Calculus Counterfactual Engine
        full_cl.push_str(&self.scm.compile_to_cl(96));
        full_cl.push_str("\n");

        // Core 128 [0,0,0,2]: 4D-Torus Collective Cognitive Stigmergy Swarm Mesh
        full_cl.push_str(&self.stigmergy.compile_to_cl(128));
        full_cl.push_str("\n");

        // Core 144 [0,0,1,2]: 3-Factor Neuromodulated Metaplasticity Core
        full_cl.push_str(&self.metaplasticity.compile_to_cl(144));
        full_cl.push_str("\n");

        // Core 192 [0,0,0,3]: Active Inference & Biological Homeostasis Engine
        full_cl.push_str(&self.homeostasis.compile_to_cl(192));
        full_cl.push_str("\n");

        // Core 208 [0,0,1,3]: Autonomous Episodic Dream Replay & Memory Consolidation
        full_cl.push_str(&self.sleep.compile_to_cl(208));
        full_cl.push_str("\n");

        full_cl
    }
}
