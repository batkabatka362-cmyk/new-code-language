// ============================================================================
// CRON Standard Library - Biological Neuromorphic STDP Plasticity Layer
// Module: cron.nn.stdp
// Target: Brain 4 (Neuromorphic SNN & Spike-Timing Plasticity)
// ============================================================================

.MODULE cron.nn.stdp

struct STDPLayer {
    num_pre: i32,
    num_post: i32,
    a_plus: f64,
    a_minus: f64,
    tau_plus: f64,
    tau_minus: f64,
    learning_rate: f64
}

struct SynapseTrace {
    pre_trace: f64,
    post_trace: f64,
    current_weight: f64
}

// Create and initialize an STDP neuromorphic learning layer
def create_stdp_layer(num_pre: i32, num_post: i32, lr: f64) -> STDPLayer {
    return STDPLayer {
        num_pre: num_pre,
        num_post: num_post,
        a_plus: 0.005,
        a_minus: 0.00525,
        tau_plus: 20.0,
        tau_minus: 20.0,
        learning_rate: lr
    }
}

// Compute biological weight delta based on relative spike timing (Delta t in ms)
// If delta_t > 0: Pre-before-post (LTP: Long-Term Potentiation)
// If delta_t < 0: Post-before-pre (LTD: Long-Term Depression)
def stdp_compute_weight_delta(layer: STDPLayer, delta_t_ms: f64) -> f64 {
    if delta_t_ms > 0.0 {
        // LTP: exponential decay based on tau_plus
        let decay: f64 = 1.0 - (delta_t_ms / layer.tau_plus)
        let effective_decay: f64 = if decay > 0.0 { decay } else { 0.0 }
        return layer.learning_rate * layer.a_plus * effective_decay
    } else {
        // LTD: depression based on tau_minus
        let abs_dt: f64 = 0.0 - delta_t_ms
        let decay: f64 = 1.0 - (abs_dt / layer.tau_minus)
        let effective_decay: f64 = if decay > 0.0 { decay } else { 0.0 }
        return 0.0 - (layer.learning_rate * layer.a_minus * effective_decay)
    }
}

// Update synaptic trace and weight in one biological clock cycle
def stdp_step(layer: STDPLayer, synapse: SynapseTrace, delta_t_ms: f64) -> SynapseTrace {
    let dw: f64 = stdp_compute_weight_delta(layer, delta_t_ms)
    let new_weight: f64 = synapse.current_weight + dw
    // Bound weights in [0.0, 1.0] for synaptic stability
    let clamped_weight: f64 = if new_weight > 1.0 { 1.0 } else if new_weight < 0.0 { 0.0 } else { new_weight }

    return SynapseTrace {
        pre_trace: synapse.pre_trace * 0.95,
        post_trace: synapse.post_trace * 0.95,
        current_weight: clamped_weight
    }
}
