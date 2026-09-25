//! Morphological Structural Plasticity & Core Re-Specialization for SAGI AGI
//!
//! Enables live dynamic neurogenesis (growing new 4D-Torus routing highways),
//! synaptic axon sprouting/pruning, and live core re-specialization (e.g. morphing idle Visual
//! cores into Frontal Executive units under high cognitive load).


/// Functional Specialization Mode of a Cron Core
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreSpecialization {
    FrontalExecutive,
    VisualCortex,
    AuditoryLanguage,
    HippocampalMemory,
    MotorPremotor,
    ThalamicWorkspace,
}

impl CoreSpecialization {
    pub fn as_str(self) -> &'static str {
        match self {
            CoreSpecialization::FrontalExecutive => "Frontal Executive",
            CoreSpecialization::VisualCortex => "Visual Cortex",
            CoreSpecialization::AuditoryLanguage => "Auditory Language",
            CoreSpecialization::HippocampalMemory => "Hippocampal Memory",
            CoreSpecialization::MotorPremotor => "Motor Premotor",
            CoreSpecialization::ThalamicWorkspace => "Thalamic Workspace",
        }
    }
}

/// Dynamic Structural Morph Controller for 256-Core Mesh
#[derive(Debug, Clone)]
pub struct StructuralMorphEngine {
    pub core_specializations: [CoreSpecialization; 256],
    pub dynamic_axon_highways: Vec<(usize, usize, f32)>, // (src_core, dst_core, bandwidth_gbps)
    pub total_morph_events: u64,
}

impl StructuralMorphEngine {
    pub fn new_256() -> Self {
        let mut specs = [CoreSpecialization::FrontalExecutive; 256];
        for i in 0..256 {
            specs[i] = if i < 48 {
                CoreSpecialization::FrontalExecutive
            } else if i < 96 {
                CoreSpecialization::VisualCortex
            } else if i < 144 {
                CoreSpecialization::AuditoryLanguage
            } else if i < 192 {
                CoreSpecialization::HippocampalMemory
            } else if i < 240 {
                CoreSpecialization::MotorPremotor
            } else {
                CoreSpecialization::ThalamicWorkspace
            };
        }

        Self {
            core_specializations: specs,
            dynamic_axon_highways: Vec::new(),
            total_morph_events: 0,
        }
    }

    /// Dynamically re-specialize an idle core into a high-demand role
    pub fn morph_core(&mut self, core_id: usize, new_role: CoreSpecialization) {
        if core_id < 256 {
            self.core_specializations[core_id] = new_role;
            self.total_morph_events += 1;
        }
    }

    /// Sprout a dynamic high-speed axon highway between two cores on the 4D-Torus
    pub fn sprout_axon_highway(&mut self, src_core: usize, dst_core: usize, bandwidth: f32) {
        self.dynamic_axon_highways.push((src_core, dst_core, bandwidth));
    }

    /// Rebalance the mesh under high Frontal Reasoning load
    pub fn balance_under_reasoning_load(&mut self, reasoning_intensity: f32) -> usize {
        let mut morphed_count = 0;
        if reasoning_intensity > 0.70 {
            // Morph up to 32 idle Visual/Motor cores into Frontal Executive cores
            for i in 60..92 {
                if self.core_specializations[i] == CoreSpecialization::VisualCortex {
                    self.morph_core(i, CoreSpecialization::FrontalExecutive);
                    morphed_count += 1;
                }
            }
        }
        morphed_count
    }

    /// Render Structural Morph HUD
    pub fn render_ascii_hud(&self) -> String {
        let mut frontal = 0;
        let mut visual = 0;
        let mut auditory = 0;
        let mut memory = 0;
        let mut motor = 0;
        let mut thalamus = 0;

        for spec in &self.core_specializations {
            match spec {
                CoreSpecialization::FrontalExecutive => frontal += 1,
                CoreSpecialization::VisualCortex => visual += 1,
                CoreSpecialization::AuditoryLanguage => auditory += 1,
                CoreSpecialization::HippocampalMemory => memory += 1,
                CoreSpecialization::MotorPremotor => motor += 1,
                CoreSpecialization::ThalamicWorkspace => thalamus += 1,
            }
        }

        format!(
            "+-------------------------------------------------------------------------+\n\
             | SAGI DYNAMIC STRUCTURAL PLASTICITY & CORE RE-SPECIALIZATION             |\n\
             +-------------------------------------------------------------------------+\n\
             | • Frontal Executive Units:    {:<3} Cores (High-Priority Reasoning)       |\n\
             | • Visual Sensory Units:       {:<3} Cores                                 |\n\
             | • Auditory & Language Units:  {:<3} Cores                                 |\n\
             | • Hippocampal Memory Units:   {:<3} Cores                                 |\n\
             | • Motor Premotor Units:       {:<3} Cores                                 |\n\
             | • Thalamic Workspace Units:   {:<3} Cores                                 |\n\
             | Dynamic Axon Highways Active: {:<3} Photonic Direct Routes                |\n\
             | Total Morphological Events:   {:<3} Live Re-specializations               |\n\
             +-------------------------------------------------------------------------+\n",
            frontal, visual, auditory, memory, motor, thalamus,
            self.dynamic_axon_highways.len(),
            self.total_morph_events,
        )
    }
}
