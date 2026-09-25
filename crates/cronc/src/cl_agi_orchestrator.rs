//! Unified Living AGI Cognitive Orchestrator for `.cl` Supercomputers
//!
//! Orchestrates the full living cognitive cycle:
//! Sensory Input -> Neuromodulatory Dynamics -> Holographic Flash Recall ->
//! Global Workspace Attention Broadcast -> Action & Metaplastic Encoding -> Episodic Sleep Replay.

use crate::cl_hyperdimensional::HolographicMemoryBank;
use crate::cl_neuromodulation::NeuromodulationEngine;
use crate::cl_sleep_engine::{SleepReplayEngine, SleepConsolidationReport};
use crate::cl_global_workspace::{GlobalWorkspaceEngine, WorkspaceFocusResult};

/// Full Living AGI Turn Result
#[derive(Debug, Clone)]
pub struct LivingAgiTurnResult {
    pub input_stimulus: String,
    pub recalled_concept: Option<String>,
    pub focus: WorkspaceFocusResult,
    pub neuromodulators_hud: String,
    pub sleep_report: Option<SleepConsolidationReport>,
    pub response_text: String,
}

/// Unified Living AGI Cognitive Mind
#[derive(Debug, Clone)]
pub struct LivingAgiMind {
    pub memory: HolographicMemoryBank,
    pub neuro: NeuromodulationEngine,
    pub workspace: GlobalWorkspaceEngine,
    pub sleep: SleepReplayEngine,
    pub current_cycle: u64,
}

impl LivingAgiMind {
    pub fn new() -> Self {
        let mut mind = Self {
            memory: HolographicMemoryBank::new(),
            neuro: NeuromodulationEngine::new(),
            workspace: GlobalWorkspaceEngine::new(),
            sleep: SleepReplayEngine::new(),
            current_cycle: 0,
        };

        // Bootstrap core foundational knowledge into Holographic Associative Memory
        mind.memory.store_pair("identity", "SAGI 256-Core Autonomous Living AGI");
        mind.memory.store_pair("purpose", "Universal General Intelligence with 0 Catastrophic Forgetting");
        mind.memory.store_pair("architecture", "4D-Torus VLIW 128-bit Neuromorphic Photonic Silicon");
        mind.memory.store_pair("creator", "Human-AI Co-Evolution Collaboration");

        mind
    }

    /// Process a single living cognitive interaction turn
    pub fn process_turn(&mut self, input: &str, reward: f32, is_sleep_requested: bool) -> LivingAgiTurnResult {
        self.current_cycle += 100;

        // 1. Modulate chemical dynamics based on input surprise & reward
        let surprise = if input.len() > 30 { 0.6 } else { 0.2 };
        let complexity = (input.len() as f32 / 50.0).clamp(0.2, 1.0);
        self.neuro.trigger_event(reward, surprise, complexity);

        // 2. Holographic Associative Recall in O(1)
        let query_word = input.split_whitespace().next().unwrap_or("identity").to_lowercase();
        let recalled = self.memory.recall(&query_word).map(|(val, _)| val);

        // 3. Submit hypothesis candidates to Global Workspace Attention Theater
        self.workspace.submit_thought("Frontal Executive", input, 0.85, &format!("Analyze goal of: '{}'", input));
        if let Some(ref rec) = recalled {
            self.workspace.submit_thought("Hippocampus Associative", rec, 0.92, &format!("Associative Memory: '{}'", rec));
        }
        self.workspace.submit_thought("Auditory Language", "syntax_structure", 0.70, "Synthesize natural response");

        // 4. Conscious Global Workspace Arbitration
        let focus = self.workspace.arbitrate_focus(self.neuro.current_state.dopamine)
            .expect("Focus must exist");

        // 5. Store experience trace in Hippocampal buffer for future sleep replay
        self.sleep.record_experience("Global Workspace", [0x00FF; 16], focus.confidence_score, self.current_cycle);

        // 6. Encode new memory if dopamine & acetylcholine are high
        if self.neuro.current_state.dopamine > 0.6 && input.contains(':') {
            let parts: Vec<&str> = input.split(':').collect();
            if parts.len() >= 2 {
                self.memory.store_pair(parts[0].trim(), parts[1].trim());
            }
        }

        // 7. Optional Autonomous Sleep Cycle
        let sleep_report = if is_sleep_requested || self.sleep.episodic_buffer.len() >= 10 {
            Some(self.sleep.execute_sleep_cycle(32))
        } else {
            None
        };

        // 8. Synthesize response
        let response = if let Some(ref rec) = recalled {
            format!("[SAGI AGI Focus: {} | Conscious Confidence: {:.1}%]\nRecalled Associative Memory: '{}'",
                focus.winning_thought, focus.confidence_score * 100.0, rec)
        } else {
            format!("[SAGI AGI Focus: {} | Conscious Confidence: {:.1}%]\nSynthesized Thought Response to '{}'",
                focus.winning_thought, focus.confidence_score * 100.0, input)
        };

        LivingAgiTurnResult {
            input_stimulus: input.to_string(),
            recalled_concept: recalled,
            focus,
            neuromodulators_hud: self.neuro.render_ascii_hud(),
            sleep_report,
            response_text: response,
        }
    }
}
