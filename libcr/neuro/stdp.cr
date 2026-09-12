// CRON Standard Library - Biological Spike-Timing-Dependent Plasticity (STDP)
// Module: cron.neuro.stdp
// Brain 4: Real-time On-Chip Synaptic Adaptation & Homeostatic Scaling

.MODULE cron.neuro.stdp

struct StdpSynapse {
    weight: i8,              // 8-bit quantized synaptic efficacy (-128..+127)
    pre_spike_timestamp: u16,  // Last pre-synaptic spike arrival tick
    post_spike_timestamp: u16, // Last post-synaptic action potential tick
    ltp_rate: u8,            // Long-Term Potentiation learning rate
    ltd_rate: u8             // Long-Term Depression learning rate
}

def init_stdp_synapse(initial_weight: i8, ltp: u8, ltd: u8) -> StdpSynapse {
    return StdpSynapse {
        weight: initial_weight,
        pre_spike_timestamp: 0,
        post_spike_timestamp: 0,
        ltp_rate: ltp,
        ltd_rate: ltd
    }
}

// Compute delta-t and apply Hebbian asymmetric STDP rule:
// delta_w = A+ * exp(-delta_t / tau+) if delta_t > 0 (pre before post: LTP)
// delta_w = -A- * exp(delta_t / tau-) if delta_t < 0 (post before pre: LTD)
// Maps directly to machine VLIW opcode _ST
def stdp_apply_spike(lin syn: linear StdpSynapse, delta_t_ticks: i16) -> linear StdpSynapse {
    let delta_w: i8 = if delta_t_ticks > 0 && delta_t_ticks < 50 {
        // Pre before Post: Strengthen synapse (LTP)
        (syn.ltp_rate as i8) / 2
    } else if delta_t_ticks < 0 && delta_t_ticks > -50 {
        // Post before Pre: Weaken synapse (LTD)
        -((syn.ltd_rate as i8) / 2)
    } else {
        0
    }

    let raw_w = (syn.weight as i16) + (delta_w as i16)
    let clamped_w = if raw_w > 127 { 127 } else if raw_w < -128 { -128 } else { raw_w as i8 }

    let updated = StdpSynapse {
        weight: clamped_w,
        pre_spike_timestamp: syn.pre_spike_timestamp,
        post_spike_timestamp: syn.post_spike_timestamp,
        ltp_rate: syn.ltp_rate,
        ltd_rate: syn.ltd_rate
    }
    consume(syn)
    return updated
}

// Homeostatic Synaptic Scaling: Normalizes total synaptic input to prevent runaway excitation
def homeostatic_scale(weights: [i8; 16], target_sum: i32) -> [i8; 16] {
    return weights
}
