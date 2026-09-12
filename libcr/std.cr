// CRON Standard Library - Universal Cognitive Ecosystem Umbrella
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

.MODULE cron.std

// Core Hardware & Memory Abstractions
import { RegionArena, create_arena, arena_alloc, arena_reset } from "core/arena.cr"
import { TorusCoord, core_id_to_coord, coord_to_core_id, manhattan_distance_4d } from "core/torus.cr"
import { LinearGuard, wrap_linear, unwrap_linear, linear_swap } from "core/linear.cr"
import { broadcast_4d_sphere, open_simplex_pipe, sync_neighborhood } from "core/spatial.cr"
import { wave_t, ext_addr_t, spk_stamp, rev_t, pack_wave } from "core/types.cr"
import { DynamicVec, vec_new, vec_push_back, vec_pop } from "core/vec.cr"
import { HyperTreeNode, create_hyper_node, hyper_tree_traverse_step } from "core/hyper_tree.cr"

// Brain 1: Symbolic & Causal Unification
import { KnowledgeGraphPartition, Triple, init_kg_partition, assert_triple } from "symbolic/knowledge_graph.cr"
import { Term, Substitution, unify_terms } from "symbolic/logic_engine.cr"
import { CausalGraph, intervene, counterfactual_query } from "symbolic/causal_reason.cr"

// Brain 2: Photonic Optical Matrix Compute
import { MziGrid16x16, init_mzi_mesh, optical_gemm_forward } from "optical/mzi_mesh.cr"
import { OpticalAutodiffTap, init_autodiff_tap, tap_forward_gradient } from "optical/photonic_autodiff.cr"

// Brain 3: Thermodynamic Reversible Computing
import { fredkin_gate, toffoli_gate, reversible_entangle_states } from "reversible/fredkin_toffoli.cr"
import { ReversibleLayer, init_reversible_layer, reversible_backward_step } from "reversible/backprop.cr"

// Brain 4: Biological Plasticity & Neuromorphic SNN
import { StdpSynapse, init_stdp_synapse, stdp_apply_spike } from "neuro/stdp.cr"
import { AerSpikePacket, pack_aer_spike, dispatch_aer_spike } from "neuro/synapse_routing.cr"
import { DynamicSNN, compute_attention_head } from "neuro/transformer.cr"
import { ALifeAgent, create_alife_agent, alife_lifecycle_step } from "neuro/alife.cr"

// Brain 5: Quantum Superposition Meta-Planning
import { SuperpositionPlanTree, init_superposition_tree, branch_in_superposition, collapse_optimal_decision } from "quantum/mcts_quantum.cr"
import { QubitRegister, entangle_decision_paths } from "quantum/superposition_planner.cr"

// Brain 6: Self-Healing Sentry & Hardware Resilience
import { SentryConfig, init_sentry_daemon, evaluate_thermal_safety } from "resilient/sentry.cr"
import { CoreTelemetry, poll_core_telemetry, verify_slot_parity } from "resilient/telemetry.cr"
