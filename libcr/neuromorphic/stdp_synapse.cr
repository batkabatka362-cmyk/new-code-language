// ============================================================================
// CRON Neuromorphic Standard Library: STDP Synapse Crossbar
// Module: cron.neuromorphic.stdp_synapse
// Target: 256-Core 4D-Torus Brain 4 Neuromorphic Array
// ============================================================================

.MODULE cron.neuromorphic.stdp_synapse

struct STDPSynapseState {
    weight: f32,
    a_plus: f32,
    a_minus: f32,
    w_min: f32,
    w_max: f32
}

def create_stdp_synapse(init_weight: f32, ltp: f32, ltd: f32) -> STDPSynapseState {
    return STDPSynapseState {
        weight: init_weight,
        a_plus: ltp,
        a_minus: ltd,
        w_min: -2.0,
        w_max: 2.0
    }
}

def stdp_compute_dw(syn: STDPSynapseState, delta_t: i32) -> f32 {
    if delta_t > 0 {
        return syn.a_plus
    } else if delta_t < 0 {
        return 0.0 - syn.a_minus
    }
    return 0.0
}
