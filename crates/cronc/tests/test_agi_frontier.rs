use cronc::cl_dendritic::MultiCompartmentNeuron;
use cronc::cl_curiosity::IntrinsicCuriosityEngine;
use cronc::cl_astrocytes::GlialNetworkMesh;
use cronc::cl_structural_morph::{StructuralMorphEngine, CoreSpecialization};

#[test]
fn test_dendritic_multi_compartment_nonlinearity() {
    let mut neuron = MultiCompartmentNeuron::new(0);

    // Case 1: Feedforward input without apical top-down context -> Does NOT fire
    let feedforward = [0.4, 0.4, 0.4, 0.4];
    let lateral_context = [0.1; 8];
    let top_down_attention = [0.1; 4];

    let (fired_no_ctx, _) = neuron.compute_dendritic_step(&feedforward, &lateral_context, &top_down_attention);
    assert!(!fired_no_ctx, "Neuron should not fire without contextual coincidence");

    // Case 2: Feedforward input WITH strong apical top-down attention -> Calcium plateau -> FIRES
    let strong_top_down = [0.8, 0.8, 0.8, 0.8];
    let (fired_with_ctx, _) = neuron.compute_dendritic_step(&feedforward, &lateral_context, &strong_top_down);
    assert!(fired_with_ctx, "Neuron must fire when feedforward and apical feedback coincide");

    let cl_code = neuron.compile_to_cl(0);
    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("_OP"));
    assert!(cl_code.contains("_MD"));
}

#[test]
fn test_intrinsic_curiosity_and_introspection() {
    let mut curiosity = IntrinsicCuriosityEngine::new();
    assert_eq!(curiosity.epistemic_drive_level, 0.85);

    // Probe an uncertain domain
    let goal = curiosity.probe_knowledge_gap("Non-Euclidean 4D Spacetime Rotors", 0.80);
    assert!(goal.is_some());
    assert_eq!(curiosity.active_goals.len(), 1);

    // Introspective resolution
    let insight = curiosity.step_introspective_reasoning();
    assert!(insight.is_some());
    assert!(insight.unwrap().contains("Resolved causal linkage"));
    assert_eq!(curiosity.resolved_insights.len(), 1);
}

#[test]
fn test_astrocytic_glial_calcium_coherence() {
    let mut glial = GlialNetworkMesh::new_256();
    assert_eq!(glial.astrocytes.len(), 256);

    let core_activities = [0.75f32; 256];
    let coherence = glial.step_mesh_dynamics(&core_activities);

    assert!(coherence > 0.80, "Global gamma coherence should remain strong: {}", coherence);
    let hud = glial.render_ascii_hud();
    assert!(hud.contains("SAGI GLIAL ASTROCYTE TRIPARTITE"));
}

#[test]
fn test_structural_morph_and_neurogenesis() {
    let mut morph = StructuralMorphEngine::new_256();
    assert_eq!(morph.core_specializations[0], CoreSpecialization::FrontalExecutive);
    assert_eq!(morph.core_specializations[60], CoreSpecialization::VisualCortex);

    // Sprout an optical highway
    morph.sprout_axon_highway(0, 128, 12.8);
    assert_eq!(morph.dynamic_axon_highways.len(), 1);

    // High cognitive reasoning load triggers live core re-specialization
    let morphed_count = morph.balance_under_reasoning_load(0.95);
    assert!(morphed_count > 0);
    assert_eq!(morph.core_specializations[60], CoreSpecialization::FrontalExecutive);
}
