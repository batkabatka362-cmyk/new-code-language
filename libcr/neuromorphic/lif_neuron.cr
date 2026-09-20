// ============================================================================
// CRON Neuromorphic Standard Library: Leaky Integrate-and-Fire (LIF) Neuron
// Module: cron.neuromorphic.lif_neuron
// Target: 256-Core 4D-Torus Brain 4 Neuromorphic Array
// ============================================================================

.MODULE cron.neuromorphic.lif_neuron

struct LIFNeuronState {
    v_membrane: f32,
    threshold: f32,
    decay_beta: f32,
    reset_voltage: f32,
    refractory_timer: i32,
    refractory_cycles: i32
}

def create_lif_neuron(decay: f32, thresh: f32) -> LIFNeuronState {
    return LIFNeuronState {
        v_membrane: 0.0,
        threshold: thresh,
        decay_beta: decay,
        reset_voltage: 0.0,
        refractory_timer: 0,
        refractory_cycles: 2
    }
}

def lif_integrate_spike(neuron: LIFNeuronState, current: f32) -> i32 {
    if neuron.refractory_timer > 0 {
        return 0
    }

    let next_v: f32 = (neuron.v_membrane * neuron.decay_beta) + current
    if next_v >= neuron.threshold {
        return 1
    }

    return 0
}
