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
import { Option, Result, is_some, is_none, unwrap_or, is_ok, is_err, unwrap_result_or } from "core/option_result.cr"
import { BootConfig, CoreStatus, create_default_boot_config, resolve_core_pgas_base, initialize_genesis_core, bootstrap_256_torus_mesh } from "core/boot.cr"

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
import { EntangledPair, EpigeneticAxon, create_entangled_pair, step_forward_reversible, step_backward_time_inversion, create_epigenetic_axon, step_axon_myelination } from "reversible/entangled.cr"

// Brain 4: Biological Plasticity & Neuromorphic SNN
import { StdpSynapse, init_stdp_synapse, stdp_apply_spike } from "neuro/stdp.cr"
import { AerSpikePacket, pack_aer_spike, dispatch_aer_spike } from "neuro/synapse_routing.cr"
import { DynamicSNN, compute_attention_head } from "neuro/transformer.cr"
import { ALifeAgent, create_alife_agent, alife_lifecycle_step } from "neuro/alife.cr"
import { CorticalColumn, NeuromodulatorCocktail, initialize_cortical_column, inject_sensory_spike_to_layer4, modulate_dopamine_level } from "neuro/micro_circuit.cr"

// Brain 5: Quantum Superposition Meta-Planning
import { SuperpositionPlanTree, init_superposition_tree, branch_in_superposition, collapse_optimal_decision } from "quantum/mcts_quantum.cr"
import { QubitRegister, entangle_decision_paths } from "quantum/superposition_planner.cr"

// Brain 6: Self-Healing Sentry & Hardware Resilience
import { SentryConfig, init_sentry_daemon, evaluate_thermal_safety } from "resilient/sentry.cr"
import { CoreTelemetry, poll_core_telemetry, verify_slot_parity } from "resilient/telemetry.cr"

// Neural Network & Tensor Engine (libcr/nn)
import { Tensor4D, create_tensor4d, tensor_to_torus_coord, tensor4d_total_elements } from "nn/tensor4d.cr"
import { FlashAttentionConfig, AttentionOutput, init_flash_attention_config, photonic_flash_attention_forward } from "nn/attention.cr"
import { FlashAttn3Config, FlashAttn3Output, init_flash_attn3_config, flash_attn3_forward } from "nn/flash_attention_3.cr"
import { MoeConfig, MoeDispatchResult, init_moe_config, moe_route_top2 } from "nn/moe.cr"
import { MlaConfig, MlaOutput, init_mla_config, mla_forward } from "nn/mla.cr"
import { SwiGluConfig, SwiGluOutput, init_swiglu_config, swiglu_forward } from "nn/swiglu.cr"
import { STDPLayer, SynapseTrace, create_stdp_layer, stdp_step } from "nn/stdp.cr"

// Concurrency & 4D-Torus CSP Mesh (libcr/concurrency)
import { Channel, channel_create, channel_shutdown } from "concurrency/channel.cr"
import { TorusRouter, TorusMessage, torus_hop_distance, torus_next_hop } from "concurrency/torus.cr"

import { Tensor, tensor_create, tensor_reshape, tensor_fma_simd, tensor_dot_simd } from "tensor/tensor.cr"
import { TransformerBlock, MultiHeadAttention, FeedForward, RMSNorm, transformer_block_forward } from "nn/transformer.cr"
import { LIFNeuron, SNNLayer, SNNTapeEntry, create_lif_neuron, snn_layer_forward, snn_layer_backward } from "neuro/snn_autograd.cr"

// Advanced Sensory & Distributed Mesh (Phase 15 Extensions)
import { DvsPixelEvent, create_dvs_event, accumulate_event_surface, dispatch_dvs_spike_to_cortex } from "vision/dvs_stream.cr"
import { CochleaChannel, init_cochlea_channel, step_cochlea_filter } from "audio/cochlea.cr"
import { pgas_ring_all_reduce, optical_cross_chip_broadcast } from "distributed/collectives.cr"

// AI Ecosystem: Native BPE Tokenizer & Autonomous Agent Tool Runtime
import { BpeVocabConfig, init_bpe_config, is_raw_byte_token, is_special_token, bpe_merge_pair, bpe_encode_4bytes, bpe_decode_to_lead_byte } from "tokenizer/bpe.cr"
import { HardToolCatalog, init_tool_catalog, AgentFrame, init_agent_frame, dispatch_hard_tool, agent_execute_step } from "agent/runtime.cr"

// SAGI Sovereign Superintelligence Subsystem (libcr/sagi)
import { SagiMailboxFrame, create_sagi_mailbox, dispatch_sagi_directive } from "sagi/bridge.cr"
import { NeuromodulatorPool, create_neuromodulator_pool, step_bcm_weight } from "sagi/metaplasticity.cr"
import { ThermalState, create_thermal_state, evaluate_quench } from "sagi/thermodynamics.cr"
import { OpticalRotor2D, create_optical_rotor } from "sagi/mzi_rotor.cr"
import { Torus4DCoordinate, create_torus_coord, compute_manhattan_distance_4d } from "sagi/torus_noc.cr"
import { PackedTrit128, create_packed_trits, trit_get_polarity, trit_zero_mul_accumulate } from "sagi/ternary.cr"
import { DendriticUnit, create_dendritic_unit, evaluate_local_error, step_predictive_synapse } from "sagi/predictive.cr"
import { LivePatchDescriptor, create_patch_descriptor, hot_patch_slot } from "sagi/self_rewriter.cr"
import { EpisodicBuffer, create_episodic_buffer, record_episodic_trace, execute_sleep_cycle } from "sagi/sleep_consolidation.cr"
import { DvsEventPacket, create_dvs_event, ingest_dvs_event, CochleaAudioSpike, create_cochlea_spike, ingest_cochlea_spike } from "sagi/sensory_hal.cr"

