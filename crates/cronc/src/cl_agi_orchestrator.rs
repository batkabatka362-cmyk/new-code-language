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

        // 7. Submit hypothesis candidates to Global Workspace Attention Theater
        self.workspace.submit_thought(
            "Frontal Executive",
            input,
            0.85,
            &format!("Analyze goal of: '{}' (Action Policy #{})", input, selected_action),
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
        }
    }

    /// Compiles the full 256-core Living Mind state down into `.cl` VLIW microcode bundles.
    ///
    /// Core Allocation across 4D-Torus [4x4x4x4]:
    /// - Cores 0..63: Sensory HAL, SSM Elastic Streaming & Spiking Temporal Attention
    /// - Cores 64..127: Modern Hopfield & Holographic Dense Associative Memory Banks
    /// - Cores 128..191: 4D-Torus Collective Stigmergy Pheromone Swarm Reasoning Mesh
    /// - Cores 192..255: Active Inference Free Energy Minimizer & Biological Homeostasis Engine
    pub fn compile_living_mind_to_cl(&self) -> String {
        let mut full_cl = String::with_capacity(32768);
        full_cl.push_str("; ============================================================================\n");
        full_cl.push_str("; CRON LIVING AGI COGNITIVE MIND: 256-CORE 4D-TORUS COMPILATION\n");
        full_cl.push_str("; Target Silicon: 256-Core Neuromorphic/Photonic VLIW Supercomputer\n");
        full_cl.push_str(&format!("; Homeostatic State: {}, Energy: {:.2}, Curiosity: {:.2}\n",
            self.homeostasis.state.active_state_name,
            self.homeostasis.state.energy_level,
            self.homeostasis.state.curiosity_drive));
        full_cl.push_str("; ============================================================================\n\n");

        // Core 0: SSM & Spiking Attention
        full_cl.push_str(&self.ssm.compile_to_cl(0));
        full_cl.push_str("\n");

        // Core 64: Hopfield Dense Associative Memory
        full_cl.push_str(&self.hopfield.compile_to_cl(64));
        full_cl.push_str("\n");

        // Core 128: 4D-Torus Collective Stigmergy Mesh
        full_cl.push_str(&self.stigmergy.compile_to_cl(128));
        full_cl.push_str("\n");

        // Core 192: Active Inference & Biological Homeostasis
        full_cl.push_str(&self.homeostasis.compile_to_cl(192));
        full_cl.push_str("\n");

        full_cl
    }
}
